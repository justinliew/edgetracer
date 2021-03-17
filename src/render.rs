const INFINITY : f64 = std::f64::INFINITY;

use std::time::{Instant};
use std::sync::Arc;

use crate::camera::{Camera};
use crate::hittable::Hittable;
use crate::hittable_list::{HittableList,HittableListWithTile};
use crate::material::Material;
use crate::scenes;
use crate::ray::Ray;
use crate::vec3::{Colour, Point3, Vec3};
use crate::utils::{clamp};
use image::codecs::jpeg;
use image::{RgbImage, ImageBuffer};

use rayon::prelude::*;

#[cfg(not(feature = "ecp"))]
use std::thread;

#[cfg(not(feature = "ecp"))]
use serde_json;

#[cfg(not(feature = "ecp"))]
use futures::{future};

fn ray_colour(r : &Ray, world: &HittableList, depth: usize) -> Colour {

	if depth  <= 0 {
		return Colour::new(0.0,0.0,0.0);
	}
	match world.hit(r, 0.001, INFINITY) {
		Some(hr) => {
			match hr.material.scatter(r, &hr) {
				Some((scattered,attenuation)) => {
					ray_colour(&scattered, world, depth-1) * attenuation
				},
				None => Colour::new(0.0,0.0,0.0)
			}
		},
		None => {
			let unit_direction = Vec3::unit_vector(r.dir);
			let t = 0.5 * (unit_direction.y + 1.0);
			Colour::new(1.0,1.0,1.0) * (1.0-t) + Colour::new(0.5,0.7,1.0) * t
		}
	}
}

#[derive(Clone,Copy,Serialize,Deserialize)]
pub struct ScreenPixel {
	r: u8,
	g: u8,
	b: u8,
	x: usize,
	y: usize,
}

const SAMPLES_PER_PIXEL : usize = 10;
const MAX_DEPTH : usize = 10;

const ASPECT_RATIO : f64 = 16.0/9.0;
const WIDTH : usize = 1024;
const HEIGHT : usize = ((WIDTH as f64) / ASPECT_RATIO) as usize;

pub fn render_tile(thread_world: &HittableList, ti: usize, tj: usize, tile_width: usize, tile_height: usize, width: usize, height: usize) -> Vec<ScreenPixel> {
	let start = Instant::now();
	let mut ret : Vec<ScreenPixel> = Vec::new();
	ret.reserve(width*height);

	let camera = Camera::new(ASPECT_RATIO, 30.0, Point3::new(13.0,2.0,3.0), Point3::new(-2.0,-2.0,0.0), Point3::new(0.0,1.0,0.0));
	let scale = 1.0 / (SAMPLES_PER_PIXEL as f64);

	let mut raytimings = vec![];

	for j in 0..tile_height {
		if tj+j >= height {
			continue;
		}
		for i in 0..tile_width {
			if ti+i >= width {
				continue;
			}
			let rs = Instant::now();
			let mut pixel_colour = Colour::new(0.0,0.0,0.0);
			for _ in 0..SAMPLES_PER_PIXEL {
				let s = (((ti + i) as f64) + rand::random::<f64>()) / ((WIDTH-1) as f64);
				let t = (((tj + j) as f64) + rand::random::<f64>()) / ((HEIGHT-1) as f64);
				let r = &camera.get_ray(s,t);
				let c = ray_colour(&r, thread_world, MAX_DEPTH);
				pixel_colour = pixel_colour + c;
			}
			ret.push(ScreenPixel{r: (256.0 * clamp(f64::sqrt(pixel_colour.x * scale), 0.0, 0.999)) as u8,
						g: (256.0 * clamp(f64::sqrt(pixel_colour.y * scale), 0.0, 0.999)) as u8,
						b: (256.0 * clamp(f64::sqrt(pixel_colour.z * scale), 0.0, 0.999)) as u8,
						x:ti+i,
						y:tj+j});
						raytimings.push(rs.elapsed().as_nanos());
		}
	}
//	println!("render_tile done {} {} {}", ti,tj, start.elapsed().as_millis());
//	println!("render_tile done {} {} Avg: {}ns, {} iterations", ti,tj, raytimings.iter().sum::<u128>() as f32 / raytimings.len() as f32, raytimings.len());
ret
}

// TODO - I need to send the dimensions along
#[cfg(feature = "edge")]
async fn send_tile_render(world: &HittableList, i: usize, j: usize, dimi: usize, dimj: usize) -> Result<String, Box<dyn std::error::Error>> {

//	println!("send_tile_render {} {}", i, j);
	let body = HittableListWithTile{
		h: world.clone(),
		i: i,
		j: j,
		dimi: dimi,
		dimj: dimj,
		height: HEIGHT,
		width: WIDTH,
	};
	let client = reqwest::Client::new();
	let resp = client.post("https://singularly-integral-cobra.edgecompute.app/rendertile")
		.json(&body)
		.send()
        .await?;

	Ok(resp.text().await?)
}

pub async fn do_render(complexity: u32, tile_dim: usize) -> (u128, Vec<u8>) {

	let start = Instant::now();
	let world = scenes::scene_of_complexity(complexity);

	// Render
	let mut img: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

	#[cfg(feature = "local")]
	{
//		println!("LOCAL");
		let mut handles : Vec<thread::JoinHandle<Vec<ScreenPixel>>> = Vec::new();
		handles.reserve((WIDTH*HEIGHT) / (tile_dim*tile_dim));
		// we are missing the top scanline
		if tile_dim >= HEIGHT || tile_dim >= WIDTH {
			println!("Single threaded");
			let p = render_tile(&world, 0,0, WIDTH, HEIGHT, WIDTH, HEIGHT);
			for t in &p {
				let pixel = img.get_pixel_mut(t.x as u32, (HEIGHT-t.y-1) as u32);
				*pixel = image::Rgb([t.r, t.g, t.b]);
			}
		} else {
			let mut iter = vec![];
			for tj in (0..HEIGHT).step_by(tile_dim).rev() {
				for ti in (0..WIDTH).step_by(tile_dim) {
					iter.push((ti,tj));
				}
			}
			let res : Vec<ScreenPixel> = iter.into_par_iter().map(|i| {
				let thread_world = Arc::clone(&world);
				render_tile(&thread_world, i.0,i.1, tile_dim, tile_dim, WIDTH, HEIGHT)
			}).flatten().collect();
			for p in res {
				let pixel = img.get_pixel_mut(p.x as u32, (HEIGHT-p.y-1) as u32);
				*pixel = image::Rgb([p.r, p.g, p.b]);
			}
		}
	}
	#[cfg(feature = "edge")]
	{
//		println!("EDGE");
		let mut futures = Vec::new();
		for tj in (0..HEIGHT-tile_dim).step_by(tile_dim).rev() {
			for ti in (0..WIDTH).step_by(tile_dim) {

				// async http request
				let ret = send_tile_render(&world,ti,tj,tile_dim,tile_dim);
				futures.push(ret);
			}
		}
		let unpin_futs: Vec<_> = futures.into_iter().map(Box::pin).collect();
		let mut futs = unpin_futs;

		while !futs.is_empty() {
			match future::select_all(futs).await {
				(Ok(val), _, remaining) => {
					futs = remaining;
					match serde_json::from_str(&val) {
						Ok(res) => {
							let v : Vec<ScreenPixel> = res;
							for t in v {
								let pixel = img.get_pixel_mut(t.x as u32, (HEIGHT-t.y-1) as u32);
								*pixel = image::Rgb([t.r, t.g, t.b]);
							}
						},
						Err(e) => { eprintln!("Error extracting pixels: {}. Got back <{}>", e, val)}
					}
				}
				(Err(_e), _, remaining) => {
					// Ignoring all errors
					futs = remaining;
					println!("error; {} left", futs.len());
				}
			}
		}
	}

	print!("{} {}\n", tile_dim, start.elapsed().as_millis());

	let mut data = Vec::new();
	let mut encoder = jpeg::JpegEncoder::new(&mut data);
	encoder.encode(
        img.as_raw(),
        img.width(),
        img.height(),
		image::ColorType::Rgb8,
    )
    .unwrap();

	let res = img.save("image.jpg");
	match res {
		Ok(_) => {
//			println!("Saved");
		},
		Err(e) => {
			println!("Error saving image {}", e);
		}
	}

	(start.elapsed().as_millis(), data)
}

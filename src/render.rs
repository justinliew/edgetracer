const INFINITY : f64 = std::f64::INFINITY;

use std::time::{Instant};
use std::sync::Arc;

use crate::camera::{Camera};
use crate::hittable::Hittable;
use crate::hittable_list::{HittableList,HittableListWithTile};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Colour, Point3, Vec3};
use crate::utils::{clamp};
use image::codecs::jpeg;
use image::{RgbImage, ImageBuffer};

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

fn random_scene(seed: Option<u128>) -> HittableList {

	// TODO so we can reproduce scenes for perf testing
	match seed {
		Some(_) => {

		},
		_ => (),
	}

	let mut world : HittableList = HittableList::new();

	let ground_material = Arc::new(Material::Lambertian{albedo: Colour::new(0.5,0.5,0.5)});
	world.add(Hittable::Sphere{centre: Point3::new(0.0,-100.5,-1.0), radius: 100., material: ground_material});

	// for a in -11..11 {
	// 	for b in -11..11 {
	// 		let choose_mat = rand::random::<f64>();
	// 		let centre = Point3::new(a as f64 + 0.9*rand::random::<f64>(), 0.2, b as f64 + 0.9*rand::random::<f64>());
	// 		if (centre - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
	// 			if choose_mat < 0.8 {
	// 				// diffuse
	// 				let albedo = Colour::random() * Colour::random();
	// 				let mat = Arc::new(Material::Lambertian{albedo: albedo});
	// 				world.add(Hittable::Sphere{centre: centre, radius: 0.2, material: mat});
	// 			} else if choose_mat < 0.95 {
	// 				// metal
	// 				let albedo = Colour::random_range(0.5, 1.0);
	// 				let mat = Arc::new(Material::Metal{albedo: albedo});
	// 				world.add(Hittable::Sphere{centre: centre, radius: 0.2, material: mat});
	// 			} else {
	// 				// glass
	// 				let mat = Arc::new(Material::Dielectric{ir: 1.5});
	// 				world.add(Hittable::Sphere{centre: centre, radius: 0.2, material: mat});
	// 			}
	// 		}
	// 	}
	// }

	let mat1 = Arc::new(Material::Dielectric{ir: 1.5});
	world.add(Hittable::Sphere{centre: Point3::new(0.0,0.1,0.0), radius: 1.0, material: mat1});
	let mat2 = Arc::new(Material::Lambertian{albedo: Colour::new(0.4,0.2,0.1)});
	world.add(Hittable::Sphere{centre: Point3::new(-4.0,1.0,0.0), radius: 1.0, material: mat2});
	let mat3 = Arc::new(Material::Metal{albedo: Colour::new(0.7,0.6,0.5)});
	world.add(Hittable::Sphere{centre: Point3::new(4.0,1.0,0.0), radius: 1.0, material: mat3});

	world
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
const WIDTH : usize = 400;
const HEIGHT : usize = ((WIDTH as f64) / ASPECT_RATIO) as usize;

const TILE_DIM : usize = 32;

pub fn render_tile(thread_world: &HittableList, ti: usize, tj: usize, width: usize, height: usize) -> Vec<ScreenPixel> {
	let mut ret : Vec<ScreenPixel> = Vec::new();
	ret.reserve(TILE_DIM*TILE_DIM);

	let camera = Camera::new(ASPECT_RATIO, 20.0, Point3::new(13.0,2.0,3.0), Point3::new(0.0,0.0,0.0), Point3::new(0.0,1.0,0.0));
	let scale = 1.0 / (SAMPLES_PER_PIXEL as f64);

	for j in 0..TILE_DIM {
		if tj+j >= height {
			continue;
		}
		for i in 0..TILE_DIM {
			if ti+i >= width {
				continue;
			}
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
		}
	}
	ret
}

// TODO - I need to send the dimensions along
#[cfg(feature = "edge")]
async fn send_tile_render(world: &HittableList, i: usize, j: usize, dimi: usize, dimj: usize) -> Result<String, Box<dyn std::error::Error>> {

	println!("send_tile_render {} {}", i, j);
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

pub async fn do_render() -> (u128, Vec<u8>) {

	let start = Instant::now();
	let world = random_scene(None);

	// Render
	let mut img: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

	#[cfg(feature = "local")]
	{
		println!("LOCAL");
		let mut handles : Vec<thread::JoinHandle<Vec<ScreenPixel>>> = Vec::new();
		handles.reserve((WIDTH*HEIGHT) / (TILE_DIM*TILE_DIM));
		// we are missing the top scanline
		for tj in (0..HEIGHT-TILE_DIM).step_by(TILE_DIM).rev() {
			for ti in (0..WIDTH).step_by(TILE_DIM) {
					let thread_world = world.clone();
					handles.push(thread::spawn(move || {
						render_tile(&thread_world, ti,tj, WIDTH, HEIGHT)
					}));
				}
		}

		for h in handles {
			let p = h.join().unwrap();
			for _ in 0..TILE_DIM*TILE_DIM {
				for t in &p {
					let pixel = img.get_pixel_mut(t.x as u32, (HEIGHT-t.y-1) as u32);
					*pixel = image::Rgb([t.r, t.g, t.b]);
				}
			}
		}
	}
	#[cfg(feature = "edge")]
	{
		println!("EDGE");
		let mut futures = Vec::new();
		for tj in (0..HEIGHT-TILE_DIM).step_by(TILE_DIM).rev() {
			for ti in (0..WIDTH).step_by(TILE_DIM) {

				// async http request
				let ret = send_tile_render(&world,ti,tj,TILE_DIM,TILE_DIM);
				futures.push(ret);

				// let ret = render_tile(&world, ti, tj);
				// for _ in 0..TILE_DIM*TILE_DIM {
				// 	for t in &ret {
				// 		let pixel = img.get_pixel_mut(t.x as u32, (HEIGHT-t.y-1) as u32);
				// 		*pixel = image::Rgb([t.r, t.g, t.b]);
				// 	}
				// }
			}
		}
		let unpin_futs: Vec<_> = futures.into_iter().map(Box::pin).collect();
		let mut futs = unpin_futs;

		println!("starting to wait for futures");
		while !futs.is_empty() {
			match future::select_all(futs).await {
				(Ok(val), _, remaining) => {
					futs = remaining;
					println!("found one: {} left", futs.len());
					match serde_json::from_str(&val) {
						Ok(res) => {
							let v : Vec<ScreenPixel> = res;
							for t in v {
								println!("Found {} {}", t.x, t.y);
								let pixel = img.get_pixel_mut(t.x as u32, (HEIGHT-t.y-1) as u32);
								*pixel = image::Rgb([t.r, t.g, t.b]);
							}
						},
						Err(e) => { eprintln!("Error extracting pixels: {}", e)}
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

	print!("{} took {}ms\n", TILE_DIM, start.elapsed().as_millis());

	let mut data = Vec::new();
	let mut encoder = jpeg::JpegEncoder::new(&mut data);
	encoder.encode(
        img.as_raw(),
        img.width(),
        img.height(),
		image::ColorType::Rgb8,
    )
    .unwrap();

	img.save("image.jpg").unwrap();

	(start.elapsed().as_millis(), data)
}

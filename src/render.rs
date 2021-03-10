const INFINITY : f64 = std::f64::INFINITY;

use std::time::{Instant};
use std::sync::Arc;

use crate::camera::{Camera};
use crate::hittable::{Hittable};
use crate::hittable_list::{HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Colour, Point3, Vec3};
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::dielectric::Dielectric;
use crate::utils::{clamp};
use image::codecs::jpeg;
use image::{RgbImage, ImageBuffer};
use std::thread;

fn ray_colour(r : &Ray, world: &dyn Hittable, depth: usize) -> Colour {

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

	let ground_material = Arc::new(Lambertian::new(&Colour::new(0.5,0.5,0.5)));
	world.add(Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100., ground_material)));

	for a in -11..11 {
		for b in -11..11 {
			let choose_mat = rand::random::<f64>();
			let centre = Point3::new(a as f64 + 0.9*rand::random::<f64>(), 0.2, b as f64 + 0.9*rand::random::<f64>());
			if (centre - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
				if choose_mat < 0.8 {
					// diffuse
					let albedo = Colour::random() * Colour::random();
					let mat = Arc::new(Lambertian::new(&albedo));
					world.add(Box::new(Sphere::new(centre, 0.2, mat)));
				} else if choose_mat < 0.95 {
					// metal
					let albedo = Colour::random_range(0.5, 1.0);
					let mat = Arc::new(Metal::new(&albedo));
					world.add(Box::new(Sphere::new(centre, 0.2, mat)));
				} else {
					// glass
					let mat = Arc::new(Dielectric::new(1.5));
					world.add(Box::new(Sphere::new(centre, 0.2, mat)));
				}
			}
		}
	}

	let mat1 = Arc::new(Dielectric::new(1.5));
	world.add(Box::new(Sphere::new(Point3::new(0.0,0.1,0.0), 1.0, mat1)));
	let mat2 = Arc::new(Lambertian::new(&Colour::new(0.4,0.2,0.1)));
	world.add(Box::new(Sphere::new(Point3::new(-4.0,1.0,0.0), 1.0, mat2)));
	let mat3 = Arc::new(Metal::new(&Colour::new(0.7,0.6,0.5)));
	world.add(Box::new(Sphere::new(Point3::new(4.0,1.0,0.0), 1.0, mat3)));

	world
}

#[derive(Clone,Copy)]
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

const TILE_DIM : usize = 16;

pub fn render_tile(thread_world: &HittableList, ti: usize, tj: usize) -> Vec<ScreenPixel> {
	let mut ret : Vec<ScreenPixel> = Vec::new();
	ret.reserve(TILE_DIM*TILE_DIM);

	let camera = Camera::new(ASPECT_RATIO, 20.0, Point3::new(13.0,2.0,3.0), Point3::new(0.0,0.0,0.0), Point3::new(0.0,1.0,0.0));
	let scale = 1.0 / (SAMPLES_PER_PIXEL as f64);

	for j in 0..TILE_DIM {
		for i in 0..TILE_DIM {
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

pub fn do_render() -> (u128, Vec<u8>) {

	let start = Instant::now();
	let world = random_scene(None);

	// Render
	let mut img: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

	#[cfg(feature = "local")]
	{
		let mut handles : Vec<thread::JoinHandle<Vec<ScreenPixel>>> = Vec::new();
		handles.reserve((WIDTH*HEIGHT) / (TILE_DIM*TILE_DIM));
		// we are missing the top scanline
		for tj in (0..HEIGHT-TILE_DIM).step_by(TILE_DIM).rev() {
			for ti in (0..WIDTH).step_by(TILE_DIM) {
					let thread_world = world.clone();
					handles.push(thread::spawn(move || {
						render_tile(&thread_world, ti,tj)
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
		for tj in (0..HEIGHT-TILE_DIM).step_by(TILE_DIM).rev() {
			for ti in (0..WIDTH).step_by(TILE_DIM) {
				let ret = render_tile(&world, ti, tj);
				for _ in 0..TILE_DIM*TILE_DIM {
					for t in &ret {
						let pixel = img.get_pixel_mut(t.x as u32, (HEIGHT-t.y-1) as u32);
						*pixel = image::Rgb([t.r, t.g, t.b]);
					}
				}
			}
		}
	}

	print!("\ndone {}ms\n", start.elapsed().as_millis());

	let mut data = Vec::new();
	let mut encoder = jpeg::JpegEncoder::new(&mut data);
	encoder.encode(
        img.as_raw(),
        img.width(),
        img.height(),
		image::ColorType::Rgb8,
//        <image::Rgb<u8> as image::Pixel>::color_type(),
    )
    .unwrap();

	img.save("image.jpg").unwrap();

	(start.elapsed().as_millis(), data)
}

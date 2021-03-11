mod vec3;
mod ray;
mod camera;
mod dielectric;
mod hittable;
mod sphere;
mod hittable_list;
mod material;
mod lambertian;
mod metal;
mod utils;
mod render;

#[macro_use]
extern crate serde;

fn main() {
	render::do_render();
}


mod vec3;
mod ray;
mod camera;
mod hittable;
mod hitrecord;
mod hittable_list;
mod material;
mod utils;
mod render;

#[macro_use]
extern crate serde;

#[tokio::main]
async fn main() {
	render::do_render().await;
}


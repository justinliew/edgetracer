mod vec3;
mod ray;
mod camera;
mod hittable;
mod hitrecord;
mod hittable_list;
mod material;
mod utils;
mod render;
mod scenes;

#[macro_use]
extern crate serde;

#[tokio::main]
async fn main() {
//	for i in (75..150).step_by(1) {
		render::do_render(100,100).await;
//	}
}


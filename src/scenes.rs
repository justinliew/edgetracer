use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::vec3::{Colour,Point3, Vec3};
use crate::hittable::Hittable;

use std::sync::Arc;

/*
Here is a basic heuristic for scene complexity:
- depth
- number of objects
- material type?
	- dielectric is ~20% more expensive
- resolution

- we need to run a bunch of simulations and try to get where the graph crosses over.
- parameters are complexity, tile size, edge vs local.
*/

pub fn random_scene(seed: Option<u128>) -> Arc<HittableList> {

	// TODO so we can reproduce scenes for perf testing
	match seed {
		Some(_) => {

		},
		_ => (),
	}

	let mut world : HittableList = HittableList::new();

	let ground_material = Arc::new(Material::Lambertian{albedo: Colour::new(0.5,0.5,0.5)});
	world.add(Hittable::Sphere{centre: Point3::new(0.0,-100.5,-1.0), radius: 100., material: ground_material});

	for a in -11..11 {
		for b in -11..11 {
			let choose_mat = rand::random::<f64>();
			let centre = Point3::new(a as f64 + 0.9*rand::random::<f64>(), 0.2, b as f64 + 0.9*rand::random::<f64>());
			if (centre - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
				if choose_mat < 0.8 {
					// diffuse
					let albedo = Colour::random() * Colour::random();
					let mat = Arc::new(Material::Lambertian{albedo: albedo});
					world.add(Hittable::Sphere{centre: centre, radius: 0.2, material: mat});
				} else if choose_mat < 0.95 {
					// metal
					let albedo = Colour::random_range(0.5, 1.0);
					let mat = Arc::new(Material::Metal{albedo: albedo});
					world.add(Hittable::Sphere{centre: centre, radius: 0.2, material: mat});
				} else {
					// glass
					let mat = Arc::new(Material::Dielectric{ir: 1.5});
					world.add(Hittable::Sphere{centre: centre, radius: 0.2, material: mat});
				}
			}
		}
	}

	let mat1 = Arc::new(Material::Dielectric{ir: 1.5});
	world.add(Hittable::Sphere{centre: Point3::new(0.0,0.1,0.0), radius: 1.0, material: mat1});
	let mat2 = Arc::new(Material::Lambertian{albedo: Colour::new(0.4,0.2,0.1)});
	world.add(Hittable::Sphere{centre: Point3::new(-4.0,1.0,0.0), radius: 1.0, material: mat2});
	let mat3 = Arc::new(Material::Metal{albedo: Colour::new(0.7,0.6,0.5)});
	world.add(Hittable::Sphere{centre: Point3::new(4.0,1.0,0.0), radius: 1.0, material: mat3});

	Arc::new(world)
}

pub fn scene_of_complexity(complexity: u32) -> Arc<HittableList> {

	let mut world : HittableList = HittableList::new();

	let ground_material = Arc::new(Material::Lambertian{albedo: Colour::new(0.5,0.5,0.5)});
	world.add(Hittable::Sphere{centre: Point3::new(0.0,-100.5,-1.0), radius: 100., material: ground_material});

	for c in 0..complexity {
		let a = c % 20;
		let b = c / 20;
		let centre = Point3::new(a as f64 + 0.9*rand::random::<f64>(), 0.2, b as f64 + 0.9*rand::random::<f64>());
		if (centre - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
			// diffuse
			// let albedo = Colour::random() * Colour::random();
			// let mat = Arc::new(Material::Lambertian{albedo: albedo});
			// world.add(Hittable::Sphere{centre: centre, radius: 0.2, material: mat});

			// // metal
			let albedo = Colour::random_range(0.5, 1.0);
			let mat = Arc::new(Material::Metal{albedo: albedo});
			world.add(Hittable::Sphere{centre: centre, radius: 0.2, material: mat});

			// // glass
			// let mat = Arc::new(Material::Dielectric{ir: 1.5});
			// world.add(Hittable::Sphere{centre: centre, radius: 0.2, material: mat});
		}
	}

	Arc::new(world)
}
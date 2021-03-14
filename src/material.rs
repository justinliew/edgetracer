use crate::ray::Ray;
use crate::hitrecord::HitRecord;
use crate::vec3::{Colour,Vec3};
use crate::utils::{reflect, refract, reflectance, rand_unit_vector};

#[derive(Clone,Serialize,Deserialize)]
pub enum Material {
	Lambertian{
		albedo: Colour,
	},
	Metal{
		albedo: Colour,
	},
	Dielectric{
		ir: f64,
	},
}

impl Material {

	pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
		match self {
			Material::Lambertian{albedo} => {
				let mut dir = rec.normal + rand_unit_vector();

				// catches degenerate scatter direction
				if dir.near_zero() {
					dir = rec.normal;
				}
				Some((
						Ray::new(&rec.p, &dir),
						*albedo,
					))
			},
			Material::Metal{albedo} => {
				let reflected = reflect(&Vec3::unit_vector(r_in.dir), &rec.normal);
				let scattered = Ray::new(&rec.p, &reflected);
				match Vec3::dot(&scattered.dir, &rec.normal) {
					d if d > 0.0 => {
						Some(
							(
								scattered,
								*albedo,
							)
						)
					},
					_ => None,
				}
			},
			Material::Dielectric{ir} => {
				let refraction_ratio = match rec.front {
					true => 1.0/ir,
					false => *ir
				};
				let unit_direction = Vec3::unit_vector(r_in.dir);
				let cos_theta = f64::min(Vec3::dot(&-unit_direction, &rec.normal), 1.0);
				let sin_theta = f64::sqrt(1.0 - cos_theta*cos_theta);

				let direction = match refraction_ratio * sin_theta > 1.0 || reflectance(cos_theta, refraction_ratio) > rand::random::<f64>() {
					true => reflect(&unit_direction, &rec.normal),
					false => refract(&unit_direction, &rec.normal, refraction_ratio),
				};

				Some(
					(
						Ray::new(&rec.p, &direction),
						Colour::new(1.0,1.0,1.0),
					)
				)
			}
		}
	}
}

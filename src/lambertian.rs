use crate::vec3::Colour;
use crate::material::Material;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::utils::{rand_unit_vector};

#[derive(Serialize,Deserialize)]
pub struct Lambertian {
	albedo: Colour,
}

impl Lambertian {
	pub fn new(a: &Colour) -> Self {
		Lambertian{
			albedo: *a,
		}
	}
}

impl Material for Lambertian {
	fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {

		let mut dir = rec.normal + rand_unit_vector();

		// catches degenerate scatter direction
		if dir.near_zero() {
			dir = rec.normal;
		}

		Some(
			(
				Ray::new(&rec.p, &dir),
				self.albedo,
			)
		)
	}
}
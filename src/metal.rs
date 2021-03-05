use crate::vec3::{Colour,Vec3};
use crate::material::Material;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::utils::{reflect};

pub struct Metal {
	albedo: Colour,
}

impl Metal {
	pub fn new(a: &Colour) -> Self {
		Metal{
			albedo: *a,
		}
	}

}
impl Material for Metal {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
		let reflected = reflect(&Vec3::unit_vector(r_in.dir), &rec.normal);
		let scattered = Ray::new(&rec.p, &reflected);
		match Vec3::dot(&scattered.dir, &rec.normal) {
			d if d > 0.0 => {
				Some(
					(
						scattered,
						self.albedo,
					)
				)
			},
			_ => None,
		}
	}
}
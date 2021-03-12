use crate::vec3::{Colour,Vec3};
use crate::material::Material;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::utils::{reflect, refract};

#[derive(Serialize,Deserialize)]
pub struct Dielectric {
	ir: f64,
}

impl Dielectric {
	pub fn new(ir: f64) -> Self {
		Dielectric{
			ir: ir,
		}
	}

	// Schlick's approximation
	fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
		let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
		r0 = r0*r0;
		r0 + (1.0-r0) * f64::powf(1.0 - cosine, 5.0)
	}
}

impl Material for Dielectric {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
		let refraction_ratio = match rec.front {
			true => 1.0/self.ir,
			false => self.ir
		};
		let unit_direction = Vec3::unit_vector(r_in.dir);
		let cos_theta = f64::min(Vec3::dot(&-unit_direction, &rec.normal), 1.0);
		let sin_theta = f64::sqrt(1.0 - cos_theta*cos_theta);

		let direction = match refraction_ratio * sin_theta > 1.0 || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random::<f64>() {
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
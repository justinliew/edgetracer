use crate::vec3::{Point3,Vec3};
use crate::ray::{Ray};
use crate::material::Material;

use std::sync::Arc;
use serde::{Serialize,Deserialize};

use crate::hitrecord::HitRecord;

#[derive(Clone,Serialize,Deserialize)]
pub enum Hittable {
	Sphere {
		centre: Point3,
		radius: f64,
		material: Arc<Material>,
	}
}


impl Hittable {

	pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
		match self {
			Hittable::Sphere{centre, radius, material} => {
				let oc = r.origin - *centre;
				let a = r.dir.len_sq();
				let half_b = Vec3::dot(&oc, &r.dir);
				let c = oc.len_sq() - radius * radius;
				let disc = half_b * half_b - a*c;
				if disc < 0.0 {
					return None;
				}

				let root = (-half_b - disc.sqrt()) / a;
				if root < tmin || root > tmax {
					return None;
				}

				let p = r.at(root);
				let outward_normal = (p - *centre) / *radius;
				Some(HitRecord::new(r, root, &outward_normal, Arc::clone(&material)))
			}
		}
	}
}

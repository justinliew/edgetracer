use crate::vec3::{Point3,Vec3};
use crate::material::Material;
use crate::ray::Ray;

use std::sync::Arc;

pub struct HitRecord {
	pub p: Point3,
	pub normal: Vec3,
	pub material: Arc<Material>,
	pub t: f64,
	pub front: bool,
}

impl HitRecord {
	pub fn new(r: &Ray, root: f64, outward_normal: &Vec3, m: Arc<Material>) -> Self {
		let p = r.at(root);
		let front = Vec3::dot(&r.dir, &outward_normal) < 0.0;
		let normal = match front {
			true => *outward_normal,
			false => -*outward_normal,
		};
		HitRecord{
			p: p,
			normal: normal,
			t: root,
			front: front,
			material: m,
		}
	}
}

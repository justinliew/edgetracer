use crate::hittable::{Hittable};
use crate::ray::{Ray};
use crate::hittable::{HitRecord};

#[derive(Clone)]
pub struct HittableList {
	list: Vec<Box<dyn Hittable + Sync + Send>>,
}

impl HittableList {
	pub fn new() -> Self {
		HittableList{
			list: Vec::new()
		}
	}
	pub fn add(&mut self, o: Box<dyn Hittable + Sync + Send>) {
		self.list.push(o);
	}
}

impl Hittable for HittableList {

	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
		let mut rec : Option<HitRecord> = None;
		let mut closest = tmax;

		for o in &self.list {
			match o.hit(r, tmin, closest) {
				None => {},
				Some(hr) => {
					closest = hr.t;
					rec = Some(hr);
				}
			}
		}

		rec
	}

	fn clone_hittable(&self) -> Box<dyn Hittable + Sync + Send> {
		Box::new(self.clone())
	}
}
use crate::hittable::{Hittable};
use crate::ray::{Ray};
use crate::hittable::{HitRecord};

use serde::{Serialize,Serializer};

#[derive(Clone)]
pub struct HittableList {
	list: Vec<Box<dyn Hittable>>,
}

impl Serialize for HittableList {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
		println!("serialize hittable_list");
		for h in &self.list {
			h.serialize_hittable();
		}
		serializer.serialize_i32(0)
	}
}

impl HittableList {
	pub fn new() -> Self {
		HittableList{
			list: Vec::new()
		}
	}
	pub fn add(&mut self, o: Box<dyn Hittable>) {
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

	fn clone_hittable(&self) -> Box<dyn Hittable> {
		Box::new(self.clone())
	}

	fn serialize_hittable(&self) {
		println!("hittable_list serialize");
	}
}
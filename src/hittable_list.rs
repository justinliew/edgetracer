use crate::hittable::{Hittable};
use crate::ray::{Ray};
use crate::hitrecord::HitRecord;

#[derive(Clone,Serialize,Deserialize)]
pub struct HittableList {
	list: Vec<Hittable>,
}

#[derive(Clone,Serialize,Deserialize)]
pub struct HittableListWithTile {
	pub h: HittableList,
	pub i: usize,
	pub j: usize,
	pub dimi: usize,
	pub dimj: usize,
	pub height: usize,
	pub width: usize,
}

impl HittableList {
	pub fn new() -> Self {
		HittableList{
			list: Vec::new()
		}
	}
	pub fn add(&mut self, o: Hittable) {
		self.list.push(o);
	}

	pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
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
}

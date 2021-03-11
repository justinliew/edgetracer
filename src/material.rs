use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Colour;

#[typetag::serde]
pub trait Material {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)>;
}

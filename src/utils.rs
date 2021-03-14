use crate::vec3::{Vec3};

pub fn rand_in_unit_sphere() -> Vec3 {
	loop {
		let p = Vec3::random_range(-1.0,1.0);
		if p.len_sq() < 1.0 {
			return p;
		}
	}
}

pub fn rand_unit_vector() -> Vec3 {
	Vec3::unit_vector(rand_in_unit_sphere())
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
	if x < min {
		min
	} else if x > max {
		max
	} else {
		x
	}
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
	*v -  *n * Vec3::dot(v,n) * 2.
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
	let cos_theta = f64::min(Vec3::dot(&-(*uv), n), 1.0);
	let r_out_perp = (*uv + *n*cos_theta) * etai_over_etat;
	let r_out_parallel = *n * -f64::sqrt(f64::abs(1.0 - r_out_perp.len_sq()));
	r_out_perp + r_out_parallel
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
	let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
	r0 = r0*r0;
	r0 + (1.0-r0) * f64::powf(1.0 - cosine, 5.0)
}

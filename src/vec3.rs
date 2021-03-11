use std::ops;
use rand;

#[derive(Clone, Copy, Debug,Serialize,Deserialize)]
pub struct Vec3 {
	pub x : f64,
	pub y : f64,
	pub z : f64,
}

impl ops::Add for Vec3 {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl ops::Sub for Vec3 {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

impl ops::Neg for Vec3 {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Vec3::new(-self.x,-self.y,-self.z)
	}
}

impl ops::Mul<f64> for Vec3 {
	type Output = Vec3;
	fn mul(self, multiplier: f64) -> Self  {
		Self {
			x: self.x * multiplier,
			y: self.y * multiplier,
			z: self.z * multiplier,
		}
	}
}

impl ops::Mul for Vec3 {
	type Output = Vec3;
	fn mul(self, other: Vec3) -> Self {
		Self {
			x: self.x * other.x,
			y: self.y * other.y,
			z: self.z * other.z,
		}
	}
}

impl ops::Div<f64> for Vec3 {
	type Output = Vec3;
	fn div(self, divisor: f64) -> Self  {
		Self {
			x: self.x / divisor,
			y: self.y / divisor,
			z: self.z / divisor,
		}
	}
}

impl Vec3 {
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Self{x: x, y: y, z: z}
	}

	pub fn random() -> Self {
		Self{
			x: rand::random::<f64>(),
			y: rand::random::<f64>(),
			z: rand::random::<f64>(),
		}
	}

	pub fn random_range(min: f64, max: f64) -> Self {
		Self{
			x: (rand::random::<f64>() * (max-min)) + min,
			y: (rand::random::<f64>() * (max-min)) + min,
			z: (rand::random::<f64>() * (max-min)) + min,
		}
	}

	pub fn len_sq(&self) -> f64 {
		self.x * self.x + self.y * self.y + self.z * self.z
	}

	pub fn len(&self) -> f64 {
		self.len_sq().sqrt()
	}

	// TODO - reference to self? operator overload needs fixing
	pub fn unit_vector(self) -> Vec3 {
		self / self.len()
	}

	pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
		a.x * b.x + a.y * b.y + a.z * b.z
	}

	pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
		Vec3::new(
			a.y * b.z - a.z * b.y,
			a.z * b.x - a.x * b.z,
			a.x * b.y - a.y * b.x,
		)
	}

	pub fn near_zero(&self) -> bool {
		const SMALL : f64 = 1e-8;
		f64::abs(self.x) < SMALL && f64::abs(self.y) < SMALL && f64::abs(self.z) < SMALL
	}
}

pub type Colour = Vec3;
pub type Point3 = Vec3;
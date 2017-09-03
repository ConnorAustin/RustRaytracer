use std::ops::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct v3 {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

impl v3 {
	pub fn new(x: f64, y: f64, z: f64) -> v3 {
		return v3 {
			x: x,
			y: y,
			z: z
		};
	}
	
	pub fn len(&self) -> f64 {
		return 
			(self.x * self.x + 
			self.y * self.y + 
			self.z * self.z).sqrt();
	}
	
	pub fn norm(&self) -> v3 {
		let l = self.len();
		return v3 {
			x: self.x / l,
			y: self.y / l,
			z: self.z / l
		};
	}
	
	pub fn dot(&self, other: v3) -> f64 {
		return 
			self.x * other.x + 
			self.y * other.y + 
			self.z * other.z;
	}
	
	pub fn cross(&self, other: v3) -> v3 {
		return v3 {
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x
		};
	}
}

impl Add for v3 {
	type Output = v3;
	
	fn add(self, other: v3) -> v3 {
		return v3 {
			x: self.x + other.x, 
			y: self.y + other.y,
			z: self.z + other.z
		};
	}
}

impl Sub for v3 {
	type Output = v3;

    fn sub(self, other: v3) -> v3 {
        return v3 {
			x: self.x - other.x, 
			y: self.y - other.y, 
			z: self.z - other.z
		};
    }
}

impl Mul<f64> for v3 {
	type Output = v3;
	
	fn mul(self, s: f64) -> v3 {
		return v3 {
			x: self.x * s,
			y: self.y * s,
			z: self.z * s
		};
	}
}

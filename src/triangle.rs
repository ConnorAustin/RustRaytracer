use vector::*;

#[derive(Copy, Clone)]
pub struct Triangle {
	pub p1: v3,
	pub p2: v3,
	pub p3: v3,
	pub col: (f64, f64, f64)
}

impl Triangle {
	pub fn new(col: (f64, f64, f64), p1: v3, p2: v3, p3: v3) -> Triangle {
		return Triangle {
			col: col,
			p1: p1,
			p2: p2,
			p3: p3
		};
	}
}

use triangle::*;
use vector::*;

pub struct Model {
	mesh: Vec<Triangle>
}

impl Model {
	pub fn new(angle: f64) -> Model {
		let mut model = Model {
			mesh: Vec::new()
		};

		// Model height and width
		let mh = 29.0;
		let mw = 12.0;
		
		model.add_triangle((196.0, 119.0, 249.0),  (0.0, mh, 0.0), (-mw, -0.0001, -mw), (mw, -0.0001, -mw));
		model.add_triangle((113.0, 252.0, 114.0),  (0.0, mh, 0.0), (-mw, -0.0001, mw), (mw, -0.0001, mw));
		model.add_triangle((237.0, 108.0, 59.0),  (0.0, mh, 0.0), (-mw, -0.0001, -mw), (-mw, -0.0001, mw));
		model.add_triangle((59.0, 173.0, 237.0),  (0.0, mh, 0.0), (mw, -0.0001, -mw), (mw, -0.0001, mw));

		model.add_triangle((196.0, 119.0, 249.0),  (0.0, -mh, 0.0), (-mw, 0.0001, -mw), (mw, 0.0001, -mw));
		model.add_triangle((113.0, 252.0, 114.0),  (0.0, -mh, 0.0), (-mw, 0.0001, mw), (mw, 0.0001, mw));
		model.add_triangle((237.0, 108.0, 59.0),  (0.0, -mh, 0.0), (-mw, 0.0001, -mw), (-mw, 0.0001, mw));
		model.add_triangle((59.0, 173.0, 237.0),  (0.0, -mh, 0.0), (mw, 0.0001, -mw), (mw, 0.0001, mw));
		model.rotate(angle);
		
		return model;
	}
	
	pub fn rotate(&mut self, theta: f64) {
		let cos = theta.cos();
		let sin = theta.sin();
		
		for tri in self.mesh.iter_mut() {
			tri.p1 = v3::new(tri.p1.x * cos - tri.p1.z * sin, tri.p1.y, tri.p1.x * sin + tri.p1.z * cos);
			tri.p2 = v3::new(tri.p2.x * cos - tri.p2.z * sin, tri.p2.y, tri.p2.x * sin + tri.p2.z * cos);
			tri.p3 = v3::new(tri.p3.x * cos - tri.p3.z * sin, tri.p3.y, tri.p3.x * sin + tri.p3.z * cos);
		}
		
		for tri in self.mesh.iter_mut() {
			tri.p1 = v3::new(tri.p1.x, tri.p1.y * cos - tri.p1.z * sin, tri.p1.y * sin + tri.p1.z * cos);
			tri.p2 = v3::new(tri.p2.x, tri.p2.y * cos - tri.p2.z * sin, tri.p2.y * sin + tri.p2.z * cos);
			tri.p3 = v3::new(tri.p3.x, tri.p3.y * cos - tri.p3.z * sin, tri.p3.y * sin + tri.p3.z * cos);
		}
	}

	// Returns the normal and distance and color if the ray hit the model and none otherwise
	pub fn raycast(&self, origin: v3, dir: v3) -> Option<(v3, f64, (f64, f64, f64))> {
		let mut best_dist: f64 = 999999999.0;
		let mut best_normal: v3 = v3::new(1.0, 0.0, 0.0);
		let mut best_color = (0.0, 0.0, 0.0);
		
		let mut hit = false;
		
		for tri in self.mesh.iter() {
			let plane_norm = (tri.p2 - tri.p1).cross(tri.p3 - tri.p1).norm();
			let b = plane_norm.dot(dir);
			if b < 0.00000001 && b > 0.00000001 {
				continue;
			}
			let a = (tri.p1 - origin).dot(plane_norm);
			let dist = a / b;
			
			if dist <= 0.0 {
				continue;
			}
			
			// Point-triangle intersection using barycentric coordinates
			let p = origin + (dir * dist);
			
			let v0 = tri.p2 - tri.p1;
			let v1 = tri.p3 - tri.p1;
			let v2 = p - tri.p1;
			
			let u = ((v1.dot(v1)) * (v2.dot(v0))-(v1.dot(v0)) * (v2.dot(v1))) / ((v0.dot(v0)) * (v1.dot(v1)) - (v0.dot(v1)) * (v1.dot(v0)));
			let v = ((v0.dot(v0)) * (v2.dot(v1))-(v0.dot(v1)) * (v2.dot(v0))) / ((v0.dot(v0)) * (v1.dot(v1)) - (v0.dot(v1)) * (v1.dot(v0)));
			
			if u <= 0.0 || v <= 0.0 || u + v > 1.0 {
				continue;
			}
			
			hit = true;
			
			if dist < best_dist {
				best_dist = dist;
				best_normal = plane_norm;
				best_color = tri.col;
			}
		}
		
		if !hit {
			return None;
		}
		
		return Some((best_normal, best_dist, best_color));
	}
	
	pub fn add_triangle(&mut self, col: (f64, f64, f64), p1: (f64, f64, f64), p2: (f64, f64, f64), p3: (f64, f64, f64)) {
		let p1 = v3::new(p1.0, p1.1, p1.2);
		let p2 = v3::new(p2.0, p2.1, p2.2);
		let p3 = v3::new(p3.0, p3.1, p3.2);
		self.mesh.push(Triangle::new(col, p1, p2, p3));
	}
}

mod event_handler;
mod vector;
mod model;
mod triangle;

extern crate sdl2;

use sdl2::rect::*;
use sdl2::render::*;
use event_handler::*;
use sdl2::pixels::Color;

use vector::*;
use model::*;

const WIDTH:u32 = 200;
const HEIGHT:u32 = 200;
const FOV:f64 = 0.9;

struct App<'a> {
	renderer: Renderer<'a>,
	event_handler: EventHandler,
	time: f64
}

impl<'a> App<'a> {
	fn new() -> App<'a> {
	    let context = match sdl2::init() {
			Ok(c) => c,
			Err(e) => panic!("Error creating SDL2 context: {}", e)
		};
		
	    let video = match context.video() {
			Ok(v) => v,
			Err(e) => panic!("Error creating video context: {}", e)
		};
		
	    let window_result = 
			video.window("Tracer", WIDTH, HEIGHT).position_centered().opengl().build();
			
		let window = match window_result {
			Ok(w) => w,
			Err(e) => panic!("Error on creating window: {}", e)
		};
		
		let renderer_result = 
			window.renderer().accelerated().build();
	    
		let renderer = match renderer_result {
			Ok(r) => r,
			Err(e) => panic!("Error on creating renderer: {}", e)
		};
		
		let event_handler = EventHandler::new(&context);
		
		return App {
			renderer: renderer,
			event_handler: event_handler,
			time: 0.0
		};
	}
	
	fn render(&mut self) {
		self.time += 0.2;
		
		let clear_r = ((self.time * 0.01).cos() * 50.0 + 200.0) as u8;
		let clear_g = ((self.time * 0.01).sin() * 50.0 + 200.0) as u8;
		let clear_b = 200;
		
		let model = Model::new(self.time);
		
		let cam_pos = v3::new(0.0, 0.0, -100.0);
		let cam_dir = v3::new(0.0, 0.0, 1.0);
		let cam_up = v3::new(0.0, 1.0, 0.0);
		let cam_right = cam_up.cross(cam_dir);
		
		let width = WIDTH as f64;
		let height = HEIGHT as f64;
		let half_width = width / 2.0;
		let half_height = height / 2.0;
		
		let interp = |a: f64, b: f64, t: f64| -> f64 {
			return a + (b - a) * t.max(0.0).min(1.0);
		};
		
		for y in 0..HEIGHT {
			for x in 0..WIDTH {
				let mut output_color = (clear_r, clear_g, clear_b);
				let xx = x as f64;
				let yy = y as f64;
				
				// Calculate normalized coordiantes (from -0.5 to 0.5)
				let nx = (xx - half_width) / width;
				let ny = (yy - half_height) / height;
				
				let ray_dir = (cam_dir + (cam_right * nx * FOV) + (cam_up * ny * FOV)).norm();
				
				let hit = model.raycast(cam_pos, ray_dir);
				if hit.is_some() {
					let d = (hit.unwrap().1 * 0.01).min(1.0);
					let dist = (0.3 + d * cam_dir.dot(hit.unwrap().0).abs()).min(1.0).max(0.0); 
					let shade = (yy - half_height).abs();
					let col = hit.unwrap().2;
					
					output_color = ((col.0 * dist + shade).min(255.0) as u8, (col.1 * dist + shade).min(255.0) as u8, (col.2 * dist + shade).min(255.0) as u8);
				} else {
					if yy > half_height {
						let plane_norm = v3::new(0.0, 1.0, 0.0);
						
						let b = plane_norm.dot(ray_dir);
						if b < 0.0001 && b > 0.0001 {
							continue;
						}
						
						let a = (v3::new(1.0, 20.0, 1.0) - cam_pos).dot(plane_norm);
						let dist = a / b;
						if dist > 0.0 {
							let p = cam_pos + (ray_dir * dist);
							let dist_factor = dist / 400.0;
							
							if (((p.x + 500.0).abs() as i32) % 20 < 10 && ((p.z + 10.0 * self.time).abs()  as i32) % 20 > 10) || (((p.x + 500.0).abs() as i32) % 20 > 10 && ((p.z + 10.0 * self.time).abs() as i32) % 20 < 10){
								output_color = (interp(120.0, clear_r as f64, dist_factor) as u8, interp(200.0, clear_g as f64, dist_factor) as u8, interp(255.0, clear_b as f64, dist_factor) as u8);
							} else {
								output_color = (interp(200.0, clear_r as f64, dist_factor) as u8, interp(120.0, clear_g as f64, dist_factor) as u8, interp(255.0, clear_b as f64, dist_factor) as u8);
							}
						}
					}
				}
				self.renderer.set_draw_color(Color::RGB(output_color.0, output_color.1, output_color.2));
				self.renderer.draw_point(Point::new(x as i32, y as i32)).ok().unwrap();
			}
		}
		
		self.renderer.present();
	}
	
	fn step(&mut self) -> bool {
		self.event_handler.process();
		if self.event_handler.quit {
			return false;
		}
    	
		self.render();
		return true;
	}
}

fn main() {
    let mut app = App::new();
	while app.step() {};
}
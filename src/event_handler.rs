use sdl2::*;
use sdl2::event::Event::*;
use sdl2::keyboard::Keycode::*;

pub struct EventHandler {
	pump: EventPump,
	pub quit: bool
}

impl EventHandler {
	pub fn new(context: &Sdl) -> EventHandler {
		let pump_result = context.event_pump();
		return match pump_result {
			Ok(p) => EventHandler {pump: p, quit: false},
			Err(e) => panic!("Couldn't create event pump for SDL events: {}", e)
		};
	}
	
	pub fn process(&mut self) {
		for event in self.pump.poll_iter() {
			match event {
				Quit { .. } => self.quit = true,
				KeyDown { keycode, .. } => 
					match keycode {
                    	Some(Escape) => self.quit = true,
                    	_ => {}
                	},
				_ => { }
			}
		}
	}
}
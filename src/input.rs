use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Input {
    event_pump: sdl2::EventPump,
}

impl Input {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let event_pump = sdl_context.event_pump().unwrap();
        Input { event_pump }
    }

    pub fn process_input(&mut self) -> bool {
        let mut quit = false;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => quit = true,
                _ => {}
            }
        }

        quit
    }
}

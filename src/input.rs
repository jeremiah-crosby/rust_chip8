use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Input {
    event_pump: sdl2::EventPump,
    keys: [bool; 16],
}

impl Input {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let event_pump = sdl_context.event_pump().unwrap();
        Input {
            event_pump,
            keys: [false; 16],
        }
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

        self.keys = [false; 16];

        let sdl_keys: Vec<Keycode> = self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        for key in sdl_keys {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };

            if let Some(i) = index {
                self.keys[i] = true;
            }
        }

        quit
    }

    pub fn is_pressed(&self, key: usize) -> bool {
        self.keys[key]
    }

    pub fn get_first_pressed_key(&self) -> Option<usize> {
        for i in 0..self.keys.len() {
            if self.keys[i] {
                return Some(i);
            }
        }

        None
    }
}

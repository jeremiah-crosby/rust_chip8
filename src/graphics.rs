use bitvec::prelude::*;
use sdl2::pixels::Color;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Graphics {
    vram: [[u8; WIDTH]; HEIGHT],
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Graphics {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video = sdl_context.video().unwrap();
        let window = video
            .window("rust-chip-8", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas
            .set_logical_size(WIDTH as u32, HEIGHT as u32)
            .expect("Could not set logical size");

        Graphics {
            vram: [[0; WIDTH]; HEIGHT],
            canvas,
        }
    }

    pub fn clear(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.vram[y][x] = 0;
            }
        }
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite_bytes: &[u8]) -> bool {
        let mut collision = false;
        for row in 0..sprite_bytes.len() {
            let y = (y as usize + row) % HEIGHT;
            let byte = sprite_bytes[row];
            let bits = BitVec::<Msb0, u8>::from_element(byte);
            for col in 0..bits.len() {
                let x = (x as usize + col) % WIDTH;
                let cur_pixel = self.vram[y].get_mut(x).unwrap();
                if bits[col] && (*cur_pixel > 0) {
                    collision = true;
                }
                *cur_pixel ^= if bits[col] { 1 } else { 0 };
            }
        }

        collision
    }

    pub fn render(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pixel = self.vram[y][x];
                let color = if pixel > 0 {
                    Color::RGB(255, 255, 255)
                } else {
                    Color::RGB(0, 0, 0)
                };
                self.canvas.set_draw_color(color);
                self.canvas
                    .draw_point(sdl2::rect::Point::new(x as i32, y as i32));
            }
        }
        self.canvas.present();
    }
}

use sdl2::pixels::Color;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Graphics {
    video: sdl2::VideoSubsystem,
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
            video,
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

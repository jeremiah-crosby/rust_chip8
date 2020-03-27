use sdl2::pixels::Color;

pub struct Graphics {
    video: sdl2::VideoSubsystem,
    //window: sdl2::video::Window,
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
        let canvas = window.into_canvas().build().unwrap();

        Graphics {
            video,
            //window,
            canvas,
        }
    }

    pub fn render(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }
}

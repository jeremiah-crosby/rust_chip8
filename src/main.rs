extern crate sdl2;

mod graphics;
mod instruction;
mod types;
mod util;
mod vm;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let sdl_context = sdl2::init().unwrap();
    let mut vm = vm::VirtualMachine::new(&sdl_context);
    let rom_path = &args[1];
    vm.run(rom_path.to_string());
    // let video_subsystem = sdl_context.video().unwrap();

    // let window = video_subsystem
    //     .window("rust-sdl2 demo", 800, 600)
    //     .position_centered()
    //     .build()
    //     .unwrap();
    // let mut canvas = window.into_canvas().build().unwrap();
    // canvas.set_draw_color(Color::RGB(0, 0, 0));
    // canvas.clear();
    // canvas.present();
    // let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;
    // 'running: loop {
    //     i = (i + 1) % 255;
    //     canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    //     canvas.clear();
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Escape),
    //                 ..
    //             } => break 'running,
    //             _ => {}
    //         }
    //     }
    //     // The rest of the game loop goes here...

    //     canvas.present();
    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    //}
}

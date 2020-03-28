extern crate sdl2;

mod audio;
mod graphics;
mod input;
mod instruction;
mod timer;
mod types;
mod util;
mod vm;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || args[1] == "-h" {
        print_usage();
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let mut vm = vm::VirtualMachine::new(&sdl_context);
    let rom_path = &args[1];
    vm.run(rom_path.to_string());
}

fn print_usage() {
    println!("Usage: rust_chip8 rom_file")
}

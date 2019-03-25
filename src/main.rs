use chip8::Chip8;
use chip8::Emulation;
use std::fs::File;

mod chip8;

fn main() {
    let file = File::open("demo/Maze_David_Winter_199x.ch8").unwrap();
    let mut chip8 = Chip8::new("Game", &file);

    for _ in 0..10 { chip8.emulate_cycle(); }

    println!("Hello")
}

mod chip8;

use chip8::Chip8;
use chip8::ActionInterface;
use std::fs::File;

fn main() {
    let file = File::open("demo/Maze_David_Winter_199x.ch8").unwrap();
    let mut chip8 = Chip8::new("Game", &file);

    for _ in 0..10 { chip8.emulate_cycle(); }

    println!("Hello")
}

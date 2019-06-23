mod chip8;

use chip8::Chip8;
use chip8::ActionInterface;
use std::fs::File;

fn main() {
    let file = File::open("demo/Maze_David_Winter_199x.ch8").unwrap();
    let mut chip8 = Chip8::new("Game", &file);

    for iteration in 0..15 {
        println!("start: {}", iteration);
        for element in chip8.gfx.iter_mut() {
            println!("{}\n", element.into_iter().map(|i| i.to_string()).collect::<String>());
        }
        println!("end: {}", iteration);
        chip8.emulate_cycle();
    }
}

use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Chip8<'a> {
    name: &'a str,
    pub opcode: u16,
    // 2 bytes
    pub memory: [u8; 4096],
    // CPU reg processor
    pub v: [u8; 16],
    // index register
    pub i: u16,
    // program counter
    pub pc: usize,
    pub gfx: [[u8; 64]; 32],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16],
    pub sp: usize,
    // HEX based keypad (0x0-0xF)
    pub key: [u16; 16],
}

impl<'a> Chip8<'a> {
    pub fn new(game_name: &'a str, rom_file: &File) -> Chip8<'a> {
        let fontset: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];


        // Game memory init. From 0 to 80 bytes - fonts, from 512 to 4096 bytes - game data
        let mut initialized_memory = [0; 4096];
        for (place, data) in initialized_memory.iter_mut().zip(fontset.iter()) {
            *place = *data
        }

        let game_data_pointer = 0x200 as usize; // 512
        let mut buf_reader = BufReader::new(rom_file);
        let rom_bytes = buf_reader.fill_buf().unwrap();
        for (place, data) in initialized_memory[game_data_pointer..4096]
            .iter_mut().zip(rom_bytes.iter()) {
            *place = *data
        }

        Chip8 {
            name: game_name,
            //512
            opcode: 0,
            memory: initialized_memory,
            v: [0; 16],
            i: 0,
            pc: game_data_pointer,
            gfx: [[0; 64]; 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
        }
    }
}

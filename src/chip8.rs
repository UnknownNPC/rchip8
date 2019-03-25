use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Chip8<'a> {
    name: &'a str,
    opcode: u16,
    // 2 bytes
    memory: [u8; 4096],
    // CPU reg processor
    v: [u8; 16],
    // index register
    i: u16,
    // program counter
    pc: usize,
    gfx: [[u8; 64]; 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: usize,
    // HEX based keypad (0x0-0xF)
    key: [u16; 16],
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

impl<'a> Emulation for Chip8<'a> {
    fn emulate_cycle(&mut self) {
        // TODO: Investigate `usize` vs `u16` for `pc` index:
        // https://www.reddit.com/r/rust/comments/8k4vwc/rust_noob_using_a_value_from_an_array_as_an_index/
        let memory_unit_1_opt = self.memory.get(self.pc);
        let memory_unit_2_opt = self.memory.get(self.pc + 1);

        if let (Some(memory_unit_1), Some(memory_unit_2)) = (memory_unit_1_opt, memory_unit_2_opt) {
            let new_opcode = (*memory_unit_1 as u16) << 8 | *memory_unit_2 as u16;
            self.opcode = new_opcode;

            match self.opcode & 0xF000 {
                0x0000 => {
                    println!("0x0000")
                    // Calls RCA 1802 program at address NNN.
                }
                0x1000 => {
                    println!("0x1000");
                    // Jumps to address NNN.
                    self.pc = (self.opcode & 0x0FFF) as usize;
                }
                0x2000 => {
                    println!("0x2000");
                    // Calls subroutine at NNN.
                    // TODO: test
                    self.stack[self.sp] = self.pc as u16;
                    self.sp = self.sp + 1;
                    self.pc = (self.opcode & 0x0FFF) as usize;
                }
                0xA000 => {
                    println!("0xA000");
                    self.i = self.opcode & 0x0FFF;
                    self.pc = self.pc + 2;
                }
                0xB000 => {
                    println!("0xB000");
                    self.pc = ((self.opcode & 0x0FFF) + self.v[0] as u16) as usize
                }
                _ => {
                    println!("Jump +2");
                    self.pc = self.pc + 2;
                }
            }
        } else {
            println!("Unable to read op_code. Skipping circle");
        }
    }

    fn set_keys() {
        unimplemented!()
    }
}

pub trait Emulation {
    fn emulate_cycle(&mut self);
    fn set_keys();
}
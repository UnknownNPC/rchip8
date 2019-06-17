use std::fs::File;
use std::io::{BufReader, BufRead};
use rand::Rng;

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

            // []XXX
            match self.opcode & 0xF000 {
                0x0000 => {
                    match self.opcode & 0x000F {
                        // [0]0E[0] - Clear screen
                        0x0000 =>

                        // [0]0E[E] - Return from subroutine
                            0x000E =>

                        _ => {
                            println!("Unknown command: {}", self.opcode);
                            break;
                        }
                    }
                    println!("0x0000");
                    println!("Calls RCA 1802 program at address NNN")
                }
                // [1]NNN - Jumps to address NNN
                0x1000 => {
                    self.pc = (self.opcode & 0x0FFF) as usize;
                    break;
                }
                // [2]NNN - Calls subroutine at NNN
                0x2000 => {
                    self.stack[self.sp] = self.pc as u16;
                    self.sp = self.sp + 1;
                    self.pc = (self.opcode & 0x0FFF) as usize;
                    break;
                }
                // [3]XNN - Skips the next instruction if VX equals NN.
                0x3000 => {
                    if self.v[self.opcode & 0x0F00 >> 8] == self.opcode & 0x00FF {
                        self.pc = self.pc + 4;
                        break;
                    } else {
                        self.pc = self.pc + 2;
                        brea;
                    }
                }
                // [4]XNN - Skips the next instruction if VX does not equal NN.
                0x4000 => {
                    if self.v[self.opcode & 0x0F00 >> 8] != self.opcode & 0x00FF {
                        self.pc = self.pc + 4;
                        break;
                    } else {
                        self.pc = self.pc + 2;
                        break;
                    }
                }
                // [5]XY0 - Skips the next instruction if VX equals VY.
                0x5000 => {
                    if self.v[self.opcode & 0x0F00 >> 8] == self.v[self.opcode & 0x00F0 >> 4] {
                        self.pc = self.pc + 4;
                        break;
                    } else {
                        self.pc = self.pc + 2;
                        break;
                    }
                }
                // [6]XNN - Sets VX to NN.
                0x6000 => {
                    self.v[self.opcode & 0x0F00 >> 8] = self.opcode & 0x00FF;
                    self.pc = self.pc + 2;
                    break;
                }
                // [7]XNN - Adds NN to VX.
                0x7000 => {
                    let vx_index = (self.opcode & 0x0F00 >> 8) as usize;
                    let vx_value = self.v[vx_index];
                    self.v[vx_index] = vx_value + self.opcode & 0x00FF;
                    self.pc = self.pc + 2;
                    break;
                }

                // 8XY[]
                0x8000 => {
                    match self.opcode & 0x00F {
                        // [8]XY[0] - Sets VX to the value of VY.
                        0x0000 => {
                            self.v[self.opcode & 0x0F00 >> 8] = self.v[self.opcode & 0x00F0 >> 4];
                            self.pc = self.pc + 2;
                            break;
                        }
                        // [8]XY[1] - Sets VX to (VX OR VY).
                        0x0001 => {
                            let vx_index = self.opcode & 0x0F00 >> 8;
                            let vx_value = self.v[vx_index];
                            self.v[vx_index] = vx_value | self.v[self.opcode & 0x00F0 >> 4];
                            self.pc = self.pc + 2;
                            break;
                        }
                        // [8]XY[2] - Sets VX to (VX AND VY).
                        0x0002 => {
                            let vx_index = self.opcode & 0x0F00 >> 8;
                            let vx_value = self.v[vx_index];
                            self.v[vx_index] = vx_value & self.v[self.opcode & 0x00F0 >> 4];
                            self.pc = self.pc + 2;
                            break;
                        }
                        // [8]XY[3] - Sets VX to (VX XOR VY).
                        0x0003 => {
                            let vx_index = self.opcode & 0x0F00 >> 8;
                            let vx_value = self.v[vx_index];
                            self.v[vx_index] = vx_value ^ self.v[self.opcode & 0x00F0 >> 4];
                            self.pc = self.pc + 2;
                            break;
                        }
                        // [8]XY[4] - Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
                        0x0004 => {
                            self.v[self.opcode & 0x0F00 >> 8] = self.v[self.opcode & 0x00F0 >> 4];
                            if self.v[self.opcode & 0x00F0 >> 4] > (0xFF - self.v[self.opcode & 0x0F00 >> 8]) {
                                self.v[0xF] = 1; //carry
                            } else {
                                self.v[0xF] = 0;
                            }
                            self.pc = self.pc + 2;
                            break;
                        }
                        // [8]XY[5] - VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                        0x0005 => {
                            if self.v[self.opcode & 0x00F0 >> 4] > self.v[self.opcode & 0x0F00 >> 8] {
                                self.v[0xF] = 0; //borrow
                            } else {
                                self.v[0xF] = 1;
                            }
                            let vx_index = self.opcode & 0x0F00 >> 8;
                            self.v[vx_index] = self.v[vx_index] - self.v[self.opcode & 0x00F0 >> 4];
                            self.pc = self.pc + 2;
                            break;
                        }
                        // [8]XY[6] - Shifts VX right by one. VF is set to the value of the least significant bit of VX before the shift.
                        0x0006 => {
                            self.v[0xF] = self.v[(self.opcode & 0x0F00) >> 8] & 0x1;
                            let vx_index = (self.opcode & 0x0F00) >> 8;
                            self.v[vx_index] = self.v[vx_index] >> 1;
                            self.pc = self.pc + 2;
                            break;
                        }
                        // [8]XY[7] -Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
                        0x0007 => {
                            if self.v[self.opcode & 0x00F0 >> 4] > self.v[self.opcode & 0x0F00 >> 8] {
                                self.v[0xF] = 0; //borrow
                            } else {
                                self.v[0xF] = 1;
                            }
                            let vx_index = (self.opcode & 0x0F00) >> 8;
                            self.v(vx_index) = self.v(self.opcode & 0x0F0 >> 4) - self.v(vx_index);
                            self.pc = self.pc + 2;
                            break;
                        }
                        // [8]XY[E] - Shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift.
                        0x000E => {
                            self.v[0xF] = self.v[(self.opcode & 0x0F00) >> 8] >> 7;
                            let vx_index = self.opcode & 0x0F00;
                            self.v[vx_index] = self.v[vx_index] << 1;
                            self.pc = self.pc + 2;
                            break;
                        }
                        _ => {
                            println!("Unknown command: {}", self.opcode);
                            break;
                        }
                    }
                }

                // [9]XY0 - Skips the next instruction if VX doesn't equal VY.
                0x9000 => {
                    if self.v[self.opcode & 0x0F00] == self.v[self.opcode & 0x00F0] {
                        self.pc = self.pc + 2;
                    } else {
                        self.pc = self.pc + 4;
                    }
                    break;
                }

                0xA000 => {
                    self.i = self.opcode & 0x0FFF;
                    self.pc = self.pc + 2;
                    break;
                }
                0xB000 => {
                    self.pc = ((self.opcode & 0x0FFF) + self.v[0] as u16) as usize
                    break;
                }
                // [C]XNN - Sets VX to a random number, masked by NN.
                0xC000 => {
                    self.v[self.opcode & 0x0F00 >> 8] = (rand::random() % (0xFF + 1)) & (self.opcode & 0x00FF);
                    self.pc = self.pc + 2;
                    break;
                }
                _ => {
                    panic!("Unknown command")
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
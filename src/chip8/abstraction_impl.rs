use rand::Rng;

use chip8::abstraction::Action;
use chip8::abstraction::ActionInterface;

use chip8::structure::Chip8;

impl<'a> Action for Chip8<'a> {
    fn current_op(&self) -> u16 {
        // TODO: Investigate `usize` vs `u16` for `pc` index:
        // https://www.reddit.com/r/rust/comments/8k4vwc/rust_noob_using_a_value_from_an_array_as_an_index/
        let memory_unit_1_opt = self.memory.get(self.pc);
        let memory_unit_2_opt = self.memory.get(self.pc + 1);

        if let (Some(memory_unit_1), Some(memory_unit_2)) = (memory_unit_1_opt, memory_unit_2_opt) {
            return (*memory_unit_1 as u16) << 8 | *memory_unit_2 as u16;
        } else {
            return panic!("Unable to read next op_code");
        }
    }

    fn op_0NNN(&mut self) {
        unimplemented!()
    }

    fn op_00E0(&mut self) {
        for (i, row) in self.gfx.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                self.gfx[i][j] = 0;
            }
        }
        self.pc = self.pc + 2;
    }

    fn op_00EE(&mut self) {
        self.sp = self.sp - 1;
        self.pc = self.stack[self.sp] as usize;
        self.pc = self.pc + 2;
    }

    fn op_1NNN(&mut self) {
        self.pc = (self.opcode & 0x0FFF) as usize;
    }

    fn op_2NNN(&mut self) {
        self.stack[self.sp] = self.pc as u16;
        self.sp = self.sp + 1;
        self.pc = (self.opcode & 0x0FFF) as usize;
    }

    fn op_3XNN(&mut self) {
        if self.v[self.opcode & 0x0F00 >> 8] == self.opcode & 0x00FF {
            self.pc = self.pc + 4;
        } else {
            self.pc = self.pc + 2;
        }
    }

    fn op_4XNN(&mut self) {
        if self.v[self.opcode & 0x0F00 >> 8] != self.opcode & 0x00FF {
            self.pc = self.pc + 4;
        } else {
            self.pc = self.pc + 2;
        }
    }

    fn op_5XY0(&mut self) {
        if self.v[self.opcode & 0x0F00 >> 8] == self.v[self.opcode & 0x00F0 >> 4] {
            self.pc = self.pc + 4;
        } else {
            self.pc = self.pc + 2;
        }
    }

    fn op_6XNN(&mut self) {
        self.v[self.opcode & 0x0F00 >> 8] = self.opcode & 0x00FF;
        self.pc = self.pc + 2;
    }

    fn op_7XNN(&mut self) {
        let vx_index = (self.opcode & 0x0F00 >> 8) as usize;
        let vx_value = self.v[vx_index];
        self.v[vx_index] = vx_value + self.opcode & 0x00FF as u8;
        self.pc = self.pc + 2;
    }

    fn op_8XY0(&mut self) {
        self.v[self.opcode & 0x0F00 >> 8] = self.v[self.opcode & 0x00F0 >> 4];
        self.pc = self.pc + 2;
    }

    fn op_8XY1(&mut self) {
        let vx_index = self.opcode & 0x0F00 >> 8;
        let vx_value = self.v[vx_index];
        self.v[vx_index] = vx_value | self.v[self.opcode & 0x00F0 >> 4];
        self.pc = self.pc + 2;
    }

    fn op_8XY2(&mut self) {
        let vx_index = self.opcode & 0x0F00 >> 8;
        let vx_value = self.v[vx_index];
        self.v[vx_index] = vx_value & self.v[self.opcode & 0x00F0 >> 4];
        self.pc = self.pc + 2;
    }

    fn op_8XY3(&mut self) {
        let vx_index = self.opcode & 0x0F00 >> 8;
        let vx_value = self.v[vx_index];
        self.v[vx_index] = vx_value ^ self.v[self.opcode & 0x00F0 >> 4];
        self.pc = self.pc + 2;
    }

    fn op_8XY4(&mut self) {
        self.v[self.opcode & 0x0F00 >> 8] = self.v[self.opcode & 0x00F0 >> 4];
        if self.v[self.opcode & 0x00F0 >> 4] > (0xFF - self.v[self.opcode & 0x0F00 >> 8]) {
            self.v[0xF] = 1; //carry
        } else {
            self.v[0xF] = 0;
        }
        self.pc = self.pc + 2;
    }

    fn op_8XY5(&mut self) {
        if self.v[self.opcode & 0x00F0 >> 4] > self.v[self.opcode & 0x0F00 >> 8] {
            self.v[0xF] = 0; //borrow
        } else {
            self.v[0xF] = 1;
        }
        let vx_index = self.opcode & 0x0F00 >> 8;
        self.v[vx_index] = self.v[vx_index] - self.v[self.opcode & 0x00F0 >> 4];
        self.pc = self.pc + 2;
    }

    fn op_8XY6(&mut self) {
        self.v[0xF] = self.v[(self.opcode & 0x0F00) >> 8] & 0x1;
        let vx_index = (self.opcode & 0x0F00) >> 8;
        self.v[vx_index] = self.v[vx_index] >> 1;
        self.pc = self.pc + 2;
    }

    fn op_8XY7(&mut self) {
        if self.v[self.opcode & 0x00F0 >> 4] > self.v[self.opcode & 0x0F00 >> 8] {
            self.v[0xF] = 0; //borrow
        } else {
            self.v[0xF] = 1;
        }
        let vx_index = (self.opcode & 0x0F00) >> 8;
        self.v[vx_index] = self.v[self.opcode & 0x0F0 >> 4] - self.v[vx_index];
        self.pc = self.pc + 2;
    }

    fn op_8XYE(&mut self) {
        self.v[0xF] = self.v[(self.opcode & 0x0F00) >> 8] >> 7;
        let vx_index = self.opcode & 0x0F00;
        self.v[vx_index] = self.v[vx_index] << 1;
        self.pc = self.pc + 2;
    }

    fn op_9XY0(&mut self) {
        if self.v[self.opcode & 0x0F00] == self.v[self.opcode & 0x00F0] {
            self.pc = self.pc + 2;
        } else {
            self.pc = self.pc + 4;
        }
    }

    fn op_ANNN(&mut self) {
        self.i = self.opcode & 0x0FFF;
        self.pc = self.pc + 2;
    }

    fn op_BNNN(&mut self) {
        self.pc = ((self.opcode & 0x0FFF) + self.v[0] as u16) as usize;
    }

    fn op_CXNN(&mut self) {
        self.v[self.opcode & 0x0F00 >> 8] = (rand::random() % (0xFF + 1)) & (self.opcode & 0x00FF);
        self.pc = self.pc + 2;
    }

    fn op_DXYN(&mut self) {
        unimplemented!()
    }

    fn op_EX9E(&mut self) {
        unimplemented!()
    }

    fn op_EXA1(&mut self) {
        unimplemented!()
    }

    fn op_FX07(&mut self) {
        unimplemented!()
    }

    fn op_FX0A(&mut self) {
        unimplemented!()
    }

    fn op_FX15(&mut self) {
        unimplemented!()
    }

    fn op_FX18(&mut self) {
        unimplemented!()
    }

    fn op_FX1E(&mut self) {
        unimplemented!()
    }

    fn op_FX29(&mut self) {
        unimplemented!()
    }

    fn op_FX33(&mut self) {
        unimplemented!()
    }

    fn op_FX55(&mut self) {
        unimplemented!()
    }

    fn op_FX65(&mut self) {
        unimplemented!()
    }
}

impl<T: Action> ActionInterface for T {
    fn emulate_cycle(&self) {
        unimplemented!()
//        // []XXX
//        match self. & 0xF000 {
//            0x0000 => {
//                match self.opcode & 0x000F {
//                    0x0000 => {
//                        self.op_00E0();
//                        break;
//                    }
//                    0x000E => {
//                        self.op_00EE();
//                        break;
//                    }
//                    _ => {
//                        println!("Unknown command: {}", self.opcode);
//                        break;
//                    }
//                }
//                println!("0x0000");
//                println!("Calls RCA 1802 program at address NNN")
//            }
//            0x1000 => {
//                self.op_1NNN();
//                break;
//            }
//            0x2000 => {
//                self.op_2NNN();
//                break;
//            }
//            0x3000 => {
//                self.op_3NNN();
//                break;
//            }
//            //4XNN
//            0x4000 => {
//                self.op_4NNN();
//                break;
//            }
//            //5XY0
//            0x5000 => {
//                self.op_5XY0();
//                break;
//            }
//            //6XNN
//            0x6000 => {
//                self.op_6XNN();
//                break;
//            }
//            //7XNN
//            0x7000 => {
//                self.op_7XNN();
//                break;
//            }
//            //8XY[]
//            0x8000 => {
//                match self.opcode & 0x00F {
//                    //8XY0
//                    0x0000 => {
//                        self.8XY0();
//                        break;
//                    }
//                    //8XY1
//                    0x0001 => {
//                        self.op_8XY1();
//                        break;
//                    }
//                    //8XY2
//                    0x0002 => {
//                        self.op_8XY2();
//                        break;
//                    }
//                    //8XY3
//                    0x0003 => {
//                        self.op_8XY3();
//                        break;
//                    }
//                    //8XY4
//                    0x0004 => {
//                        self.op_8XY4();
//                        break;
//                    }
//                    //8XY5
//                    0x0005 => {
//                        self.op_8XY5();
//                        break;
//                    }
//                    //8XY6
//                    0x0006 => {
//                        self.op_8XY6();
//                        break;
//                    }
//                    //8XY7
//                    0x0007 => {
//                        self.op_8XY7();
//                        break;
//                    }
//                    //8XYE
//                    0x000E => {
//                        self.op_8XYE();
//                        break;
//                    }
//                    _ => {
//                        println!("Unknown command: {}", self.opcode);
//                        break;
//                    }
//                }
//            }
//
//            //9XY0
//            0x9000 => {
//                self.op_9XY0();
//                break;
//            }
//            //ANNN
//            0xA000 => {
//                self.op_ANNN();
//                break;
//            }
//            //BNNN
//            0xB000 => {
//                self.op_BNNN();
//                break;
//            }
//            //CXNN
//            0xC000 => {
//                self.op_CXNN();
//                break;
//            }
//
//            //DXYN
//            0xD000 => {
//                self.op_DXYN();
//            }
//            _ => {
//                panic!("Unknown command")
//            }
//        }
    }

    fn set_keys() {
        unimplemented!()
    }
}

use std::{env::args, fs};

struct Stack {
    pointer: usize,
    data: [u16; 16],
}

impl Stack {
    fn new() -> Stack {
        Stack {
            pointer: 0,
            data: [0; 16],
        }
    }
    fn push(&mut self, n: u16) {
        self.pointer += 1;
        if self.pointer > 15 {
            panic!("The stack is filled up");
        }
        self.data[self.pointer] = n;
    }
    fn pop(&mut self) -> u16 {
        let result = self.data[self.pointer];
        self.pointer -= 1;
        result
    }
}
struct Chip8 {
    registers: [u8; 16],
    i: u16,
    dt: u8,
    st: u8,
    pc: u16,
    stack: Stack,
    key: u8,
    memory: [u8; 4096],
    display: [[bool; 64]; 32],
    redraw: bool,
}

impl Chip8 {
    fn new() -> Chip8 {
        Chip8 {
            registers: [0; 16],
            i: 0x200,
            dt: 0,
            st: 0,
            pc: 0x200,
            stack: Stack::new(),
            key: 0,
            memory: [0; 4096],
            display: [[false; 64]; 32],
            redraw: false,
        }
    }
    fn ld_font(&mut self) {
        let fonts: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            [0x20, 0x60, 0x20, 0x20, 0x70], // 1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
            [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
            [0xF0, 0x80, 0xF0, 0x40, 0x90], // 6
            [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
            [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
        ];
        let mut position = 0;
        for character in fonts {
            for byte in character {
                self.memory[position] = byte;
                position += 1;
            }
        }
    }
    fn cls(&mut self) {
        self.display = [[false; 64]; 32];
        self.redraw = true;
    }
    fn ret(&mut self) {
        self.pc = self.stack.pop();
        self.stack.pointer -= 1;
    }

    fn jump(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn call(&mut self, addr: u16) {
        self.stack.push(self.pc);
        self.pc = addr;
    }

    fn se(&mut self, x: usize, data: u8) {
        if self.registers[x] == data {
            self.pc += 2;
        }
    }

    fn sne(&mut self, x: usize, data: u8) {
        if self.registers[x] != data {
            self.pc += 2;
        }
    }

    fn se_registers(&mut self, x: usize, y: usize) {
        if self.registers[x] == self.registers[y] {
            self.pc += 2;
        }
    }

    fn ld(&mut self, x: usize, data: u8) {
        self.registers[x] = data;
    }

    fn add(&mut self, x: usize, data: u8) {
        self.registers[x] += data;
    }

    fn ld_registers(&mut self, x: usize, y: usize) {
        self.registers[x] = self.registers[y];
    }

    fn or(&mut self, x: usize, y: usize) {
        self.registers[x] |= self.registers[y];
    }
    fn and(&mut self, x: usize, y: usize) {
        self.registers[x] &= self.registers[y];
    }
    fn xor(&mut self, x: usize, y: usize) {
        self.registers[x] ^= self.registers[y];
    }
    fn add_registers(&mut self, x: usize, y: usize) {
        let lhs = self.registers[x] as u16;
        let rhs = self.registers[y] as u16;
        let result = lhs + rhs;
        if result > 255 {
            self.registers[x] = 255;
            self.registers[15] = 1;
        } else {
            self.registers[x] = result as u8;
            self.registers[15] = 0;
        }
    }

    fn sub(&mut self, x: usize, y: usize) {
        if self.registers[x] > self.registers[y] {
            self.registers[15] = 1;
            self.registers[x] -= self.registers[y];
        } else {
            self.registers[15] = 0;
        }
    }

    fn shr(&mut self, x: usize) {
        // if self.registers[x] ^ 1 == self.registers[x] - 1 {
        //     self.registers[15] = 1;
        //     self.registers[x] /= 2;
        // }
        self.registers[x] >>= 1;
        self.registers[x] /= 2;
    }

    fn subn(&mut self, x: usize, y: usize) {
        if self.registers[y] > self.registers[x] {
            self.registers[15] = 1;
            self.registers[x] = self.registers[y] - self.registers[x];
        } else {
            self.registers[15] = 0;
        }
    }

    fn shl(&mut self, x: usize) {
        self.registers[x] <<= 1;
        self.registers[15] = self.registers[x] >> 7;
        self.registers[x] *= 2;
    }

    fn sne_registers(&mut self, x: usize, y: usize) {
        if self.registers[x] != self.registers[y] {
            self.pc += 2;
        }
    }

    fn ld_i(&mut self, addr: u16) {
        self.i = addr;
    }

    fn jp(&mut self, addr: u16) {
        self.pc = addr + self.registers[0] as u16;
    }

    fn rand(&mut self, x: usize, addr: u16) {}

    fn drw(&mut self, x: usize, y: usize, n: u8) {
        let c_x = self.registers[x] as usize;
        let c_y = self.registers[y] as usize;
        for i in 0..n {
            let pixel = self.memory[self.i as usize + i as usize];
            for j in 0..4 {
                let row = (c_y + i as usize) % 32;
                let col = (c_x + j) % 64;
                if self.display[row][col] {
                    self.registers[15] = 1;
                } else {
                    self.registers[15] = 0;
                }
                let temp = pixel >> (j + 4);
                if temp & 1 == 1 {
                    self.display[row][col] ^= true;
                }
            }
        }
        self.redraw = true;
    }

    fn skp(&mut self, x: usize) {
        if self.key == self.registers[x] {
            self.pc += 2;
        }
    }
    fn sknp(&mut self, x: usize) {
        if self.key != self.registers[x] {
            self.pc += 2;
        }
    }

    fn ld_delay(&mut self, x: usize) {
        self.registers[x] = self.dt;
    }

    fn ld_key(&mut self, x: usize) {
        self.registers[x] = self.key;
    }

    fn set_dt(&mut self, x: usize) {
        self.dt = self.registers[x];
    }

    fn set_st(&mut self, x: usize) {
        self.st = self.registers[x];
    }

    fn add_i(&mut self, x: usize) {
        self.i += self.registers[x] as u16;
    }

    fn set_i(&mut self, x: usize) {
        self.i = self.registers[x] as u16;
    }

    fn ld_bcd(&mut self, x: usize) {
        let data = self.registers[x];
        let hundredth = (data / 100) % 10;
        let tenth = data / 10;
        let digit = data % 10;
        self.memory[self.i as usize] = hundredth;
        self.memory[self.i as usize + 1] = tenth;
        self.memory[self.i as usize + 2] = digit
    }

    fn ld_v(&mut self, x: usize) {
        for i in 0..x {
            self.memory[self.i as usize + i] = self.registers[i];
        }
    }
    fn ld_into_v(&mut self, x: usize) {
        for i in 0..x {
            self.registers[i] = self.memory[self.i as usize + i];
        }
    }
    fn load_rom(&mut self, rom: &String) {
        let contents = fs::read(rom).expect("Rom doesn't exist");

        // println!("------- Instructions from the rom --------");
        // let mut counter = 0;
        for i in 0..(contents.len() / 2) {
            self.memory[0x200 + (i * 2)] = contents[(i * 2)];
            self.memory[0x200 + 1 + (i * 2)] = contents[(i * 2) + 1];
            // counter += 1;
        }
        // println!("The counter is {}", counter);
    }

    fn run(&mut self, pc: &usize) {
        let instruction: u16 = ((self.memory[*pc] as u16) << 8) | (self.memory[pc + 1] as u16);
        // println!("{:04x}", instruction);

        match instruction {
            0x00E0 => {
                println!("Cleared screen");
                self.cls();
                return;
            }
            0x00EE => {
                self.ret();
                return;
            }
            _ => (),
        }
        let opcode = instruction & 0xF000;
        let param = instruction & 0x0FFF;
        match opcode {
            0x1000 => {
                let addr = instruction & 0x0FFF;
                self.jump(addr);
            }
            0x2000 => {
                let addr = instruction & 0x0FFF;
                self.call(addr);
            }
            0x3000 => {
                let x = (param & 0x0F00) >> 8;
                let byte = param & 0x00FF;
                self.se(x as usize, byte as u8);
            }
            0x4000 => {
                let x = (param & 0x0F00) >> 8;
                let byte = param & 0x00FF;
                self.sne(x as usize, byte as u8);
            }
            0x5000 => {
                let x = (param & 0x0F00) >> 8;
                let y = (param & 0x00F0) >> 4;
                self.se_registers(x as usize, y as usize);
            }
            0x6000 => {
                let x = (param & 0x0F00) >> 8;
                let byte = param & 0x00FF;
                self.ld(x as usize, byte as u8);
            }
            0x7000 => {
                let x = (param & 0x0F00) >> 8;
                let byte = param & 0x00FF;
                self.add(x as usize, byte as u8);
            }
            0x8000 => {
                let last = param & 0x000F;
                let x = ((param & 0x0F00) >> 8) as usize;
                let y = ((param & 0x00F0) >> 4) as usize;
                match last {
                    0x0000 => self.ld_registers(x, y),
                    0x0001 => self.or(x, y),
                    0x0002 => self.and(x, y),
                    0x0003 => self.xor(x, y),
                    0x0004 => self.add_registers(x, y),
                    0x0005 => self.sub(x, y),
                    0x0006 => {
                        self.shr(x);
                        println!("Activated Function shr")
                    }
                    0x0007 => self.subn(x, y),
                    0x000E => {
                        self.shl(x);
                        println!("Activated Function shl")
                    }
                    _ => (),
                }
            }
            0x9000 => {
                let x = ((param & 0x0F00) >> 8) as usize;
                let y = ((param & 0x00F0) >> 4) as usize;
                self.sne_registers(x, y);
            }
            0xA000 => {
                let byte = param & 0x00FF;
                self.ld_i(byte);
            }
            0xB000 => {
                let byte = param & 0x00FF;
                self.jp(byte);
            }
            0xC000 => {
                println!("Activated Function");
                let x = (param & 0x0F00) >> 8;
                let byte = param & 0x00FF;
                self.rand(x as usize, byte);
            }
            0xD000 => {
                let x = ((param & 0x0F00) >> 8) as usize;
                let y = ((param & 0x00F0) >> 4) as usize;
                let n = param & 0x000F;
                self.drw(x, y, n as u8);
            }

            0xE000 => {
                let x = ((param & 0x0F00) >> 8) as usize;
                let byte = param & 0x00FF;
                match byte {
                    0x009E => self.skp(x),
                    0x00A1 => self.sknp(x),
                    _ => (),
                }
            }
            0xF000 => {
                let x = ((param & 0x0F00) >> 8) as usize;
                let byte = param & 0x00FF;
                match byte {
                    0x0007 => self.ld_delay(x),
                    0x000A => self.ld_key(x),
                    0x0015 => self.set_dt(x),
                    0x0018 => self.set_st(x),
                    0x001E => self.add_i(x),
                    0x0029 => self.set_i(x),
                    0x0033 => self.ld_bcd(x),
                    0x0055 => self.ld_v(x),
                    0x0065 => self.ld_into_v(x),
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let mut emulator = Chip8::new();
    emulator.ld_font();
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    emulator.load_rom(filename);
    while emulator.pc <= 644 {
        emulator.redraw = false;
        emulator.run(&(emulator.pc as usize));
        if emulator.redraw {
            for row in emulator.display {
                for col in row {
                    if col {
                        print!("=");
                    } else {
                        print!(" ");
                    }
                }
                println!("");
            }
            // emulator.cls();
        }
        emulator.pc += 2;
    }
}

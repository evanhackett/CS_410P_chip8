// For each opcode I pasted documentation that I got from the infamous
// Cowgod's Chip-8 Technical Reference, found here: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

// I also used my own javascript chip8 interpreter as a reference: https://github.com/evanhackett/chip8/blob/master/src/chip8.js

// I also found a Rust chip8 interpreter on github that came with a tutorial video as well. I tried to mostly implement things myself, but to save time
// I borrowed a few things from it. I try to explicitly reference that where appropriate. That is found here: https://github.com/AlexEne/rust-chip8

use display::Display; // Using AlexEne's Display module. Found here: https://github.com/AlexEne/rust-chip8/blob/master/src/display.rs
use rand;
use rand::distributions::{IndependentSample, Range};

pub struct Chip8 {
    memory: [u8; 4096], // the chip8 has 4096 bytes of ram.
    v: [u8; 16], // 16 8-bit data registers named V0 to VF. The VF register doubles as a carry flag.
    i: u16,      // The adress register, named 'I'
    pc: u16,     // the program counter, stores the currently executing address.
    stack: Vec<u16>, // the stack. stores return addresses.
    display: Display, // Display module I found on github to save myself some work: https://github.com/AlexEne/rust-chip8/blob/master/src/display.rs
    rng: rand::ThreadRng, // needed for the rand instruction (0xCxkk)
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200, // Chip8 programs typically start at address 0x200.
            stack: vec![0; 16],
            display: Display::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        for i in 0..rom.len() {
            // starting at address 0x200 (again, chip8 programs are expected to start there),
            // load each byte from the ROM into the chip8's memory.
            self.memory[0x200 + i] = rom[i];
        }
    }

    // moves the program counter to point to the address of the next instruction.
    fn increment_pc(&mut self) {
        // We increment by 2 instead of 1 since each instruction is 2 bytes, and each index in the memory array is 1 byte.
        self.pc += 2;
    }

    // returns the internal screen buffer from the Display module.
    // This is essentially an array of bits where each bit corresponds to a pixel and whether it should be colored white or black.
    pub fn screen_buffer(&self) -> &[u8] {
        self.display.get_display_buffer()
    }

    // This function draws a sprite to the screen buffer.
    // It also sets the register Vf based on whether or not there was a collision.
    // From Cowgod's Reference:
    // The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on
    // screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased,
    // VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the
    // display, it wraps around to the opposite side of the screen.
    fn draw(&mut self, x: u8, y: u8, height: u8) {
        // I also used AlexEne's Chip8 interpreter as a reference here to help me implement this to see how he was using his display module.
        // Once again, linke to AlexEne's interpreter: https://github.com/AlexEne/rust-chip8
        let mut collision = false;
        for sprite_y in 0..height {
            let b = self.memory[(self.i + sprite_y as u16) as usize];
            if self.display.debug_draw_byte(b, x, y + sprite_y) {
                collision = true;
            }
        }
        if collision {
            self.v[0xF as usize] = 1;
        } else {
            self.v[0xF as usize] = 0;
        }
    }

    pub fn run(&mut self) {
        // Fetch opcode.
        // Each opcode is 2 bytes. Here we grab 2 bytes from memory and merge
        // them together with a left shift and a bitwise 'OR'.
        let byte1 = self.memory[self.pc as usize] as u16;
        let byte2 = self.memory[(self.pc + 1) as usize] as u16;

        let opcode: u16 = (byte1 << 8) | byte2;

        println!("Opcode: {:#X}",opcode);

        // Straight from Cowgod's Reference:
        // n or nibble - A 4-bit value, the lowest 4 bits of the instruction
        // x - A 4-bit value, the lower 4 bits of the high byte of the instruction
        // y - A 4-bit value, the upper 4 bits of the low byte of the instruction
        // kk or byte - An 8-bit value, the lowest 8 bits of the instruction

        let n = (opcode & 0x00F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let kk = (opcode & 0x0FF) as u8;

        // The highest 4 bits determine which opcode to run, so we match on those bits.
        match (opcode & 0xF000) >> 12 {
            // A 0 could be one of two opcodes
            0x0 => {
                match n {
                    // 00E0 - CLS
                    // Clear the display.
                    0xE0 => {
                        self.display.clear();
                        self.increment_pc();
                    }

                    // 00EE - RET
                    // Return from a subroutine.
                    // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
                    0xEE => {
                        self.pc = self.stack.pop().unwrap();
                    }
                    _ => panic!("Error: {:#X} is not a supported opcode.", opcode),
                }
            }

            // Annn - LD I, addr
            // Set I = nnn.
            // The value of register I is set to nnn.
            0xA => {
                self.i = opcode & 0x0FFF;
                self.increment_pc();
            }

            // Cxkk - RND Vx, byte
            // Set Vx = random byte AND kk.
            // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
            // The results are stored in Vx.
            0xC => {
                let interval = Range::new(0, 255);
                let number = interval.ind_sample(&mut self.rng);
                self.v[x as usize] = number & kk;
                self.increment_pc();
            }

            // Dxyn - DRW Vx, Vy, nibble
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            0xD => {
                let vx = self.v[x as usize];
                let vy = self.v[y as usize];
                self.draw(vx, vy, n);
                self.increment_pc();
            }

            // 1nnn - JP addr
            // Jump to location nnn.
            // The interpreter sets the program counter to nnn.
            0x1 => {
                self.pc = opcode & 0x0FFF;
            }

            // 3xkk - SE Vx, byte
            // Skip next instruction if Vx = kk.
            // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
            0x3 => {
                if self.v[x as usize] == kk {
                    self.increment_pc();
                }
                self.increment_pc();
            }

            // 6xkk - LD Vx, byte
            // Set Vx = kk.
            // The interpreter puts the value kk into register Vx.
            0x6 => {
                self.v[x as usize] = kk;
                self.increment_pc();
            }

            // 7xkk - ADD Vx, byte
            // Set Vx = Vx + kk.
            // Adds the value kk to the value of register Vx, then stores the result in Vx.
            0x7 => {
                self.v[x as usize] += kk;
                self.increment_pc();
            }

            _ => panic!("Error: {:#X} is not a supported opcode.", opcode),
        }
    }
}

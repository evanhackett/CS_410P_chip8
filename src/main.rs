// Evan Hackett
// CS 410P - Rust
// Final project
// Chip-8 Interpreter
// Sources used:
// Cowgod's Technical Reference: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
// My own previous chip8 interpreter in javascript: https://github.com/evanhackett/chip8
// AlexEne's Chip8 interpreter in Rust: https://github.com/AlexEne/rust-chip8

extern crate minifb; // minifb is the lib we use for rendering graphics and handling keyboard input.
extern crate rand;

use chip8::Chip8; // The chip8 system itself. This is where the interesting stuff is.
use display::Display; // Using AlexEne's Display module. Found here: https://github.com/AlexEne/rust-chip8/blob/master/src/display.rs
use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

mod chip8;
mod display;

fn main() {
    // the first thing we want to do is open up the ROM file.
    // For now we hard code in the MAZE file path.
    // Maze is the easiest ROM to get working since it requires no controller implementation or sound,
    // and generally seems to have the fewest instructions.
    let mut file = File::open("ROMs/MAZE").unwrap();

    let mut chip = Chip8::new();

    // load the ROM file into the chip8 memory.
    let mut rom_data = Vec::<u8>::new();
    file.read_to_end(&mut rom_data);
    chip.load_rom(&rom_data);

    // The chip8 draws to a screen that is 640 x 320.
    let width = 640;
    let height = 320;

    // This buffer will take values from the chip8's internal screen buffer,
    // and then will be passed to minifb's update function (which will render the screen).
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(
        "Chip8 - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // In javascript I used requestAnimationFrame to handle the animation and my emulator was slow.
    // Outside the browser we can be more explicit regarding how fast to run this thing.
    // I used AlexEne's chip8 emulator as a reference here to see how he handled the timings.
    // This code was found here: https://github.com/AlexEne/rust-chip8

    // We will need to keep track of when instructions last ran so we know if it is time for another processor tick from the chip8.
    let mut last_instruction_run_time = Instant::now();
    let mut last_display_time = Instant::now();

    // press escape to exit!
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // run one instruction every 2 milliseconds.
        if Instant::now() - last_instruction_run_time > Duration::from_millis(2) {
            chip.run();
            last_instruction_run_time = Instant::now();
        }

        // redraw the screen every 10 milliseconds
        if Instant::now() - last_display_time > Duration::from_millis(10) {
            let chip8_buffer = chip.screen_buffer();

            // loop through all the pixels and set buffer[i] to the color corresponding to chip8's screenbuffer[i]
            for y in 0..height {
                let y_coord = y / 10;
                let offset = y * width;
                for x in 0..width {
                    let index = Display::get_index_from_coords(x / 10, y_coord);
                    let pixel = chip8_buffer[index];
                    let color_pixel = match pixel {
                        0 => 0x0,
                        1 => 0xffffff,
                        _ => unreachable!(),
                    };
                    buffer[offset + x] = color_pixel;
                }
            }

            // window is the minifb object we use to render graphics.
            // We pass the buffer object directly in, and it draws the corresponding RGB hex color at each pixel.
            window.update_with_buffer(&buffer);

            // update display time.
            last_display_time = Instant::now();
        }
    }
}

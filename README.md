# CS_410P_chip8
Final project for Rust (CS410P) - Chip 8 Interpreter

I created a [chip8](https://en.wikipedia.org/wiki/CHIP-8) interpreter a few years ago in javascript. It was really slow and had a blinky sprite bug, so I always wanted to redo it. Rust seems like a great language for this sort of thing.

This project is for an elective course I am taking at PSU (CS410P - Rust).

To see it in action:
```
$ cargo run
```

In the root dir.

At the time of this writing I have not implemented all opcodes, only enough to get the ROM called MAZE working, which is about 1/3 of the total opcodes. This is enough for a proof of concept, the other opcodes at this point wouldn't be hard to implement. The hardest part is loading the ROM into memory, starting the decode process, drawing to screen, etc. The rest of the opcodes are basic instructions such as logical operators and load instructions. The main feature missing would be getting keyboard input for the chip-8, but the [minifb](https://github.com/emoon/rust_minifb) library seems like it makes that pretty easy. I would highly recommend [minifb](https://github.com/emoon/rust_minifb) to anyone looking for a super simple cross platform library for directly drawing a buffer to a window. Just hand it a buffer with color values for each pixel and it will draw it to a window. This works really well for an emulator like the chip-8 since the chip-8 internally stores a screen-buffer of its own.

I used the following as references while implementing this:

[Cowgod's Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

[My own previous chip8 interpreter in javascript](https://github.com/evanhackett/chip8)

[AlexEne's Chip8 interpreter in Rust](https://github.com/AlexEne/rust-chip8)

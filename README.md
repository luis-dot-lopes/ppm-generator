# ppm image generation using rust

This is a simple project with various utilities to generate ppm images. [Learn more about the PPM format](https://en.wikipedia.org/wiki/Netpbm)  
There's not much structure to this project yet, but hopefully it can become an usable library in the future.

To run:

    cargo build
    cargo run

This will generate foo.ppm and bar.ppm. To view them, use:

    python render.py [filename]

The ppm viewer was written by me and has its own [repository](github.com/luis-dot-lopes/ppm-viewer)

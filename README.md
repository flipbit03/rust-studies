# Cadu's Rust Studies.

Repo I use to dump random things and experiments I'm doing to learn and get better at Rust. 

Each study is implemented as a single binary or tool that can be run separately.

## Studies:


### st01: Multithread Test
-----------------------

Simple CLI that receives inputs (X+Y) and offloads the "computations" to a worker thread, leaving the main thread free to keep receiving inputs. I've used this to learn about threads and `mpsc` channels.

    cargo run --bin multithread_cli

### st02: PDF Generation
-----------------------

Generates a PDF with skia-safe (Rust bindings for the C++ Skia Lib), drawing text etc.

Dependencies:

On Ubuntu, this is what worked for me:

    sudo apt install build-essential clang libfontconfig-dev

Running: 

    cargo run --bin pdfgen

This will generate a `test.pdf` file in the root of the project with the contents of a simple shopping list.


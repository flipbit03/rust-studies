# Cadu's Rust Studies.

Implemented as a huge project with several mini binaries.

## Studies:


### Multithread Test
-----------------------

Simple CLI which offloads computations to a worker thread, so that you can keep typing inputs while the worker thread is calculating the desired computation result

    cargo run --bin multithread_cli

#### PDF Generation

Generates a PDF with skia-safe (Rust bindings for the C++ Skia Lib), drawing text etc.

    cargo run --bin pdfgen

Follow [Rust-Skia](https://github.com/rust-skia/rust-skia)'s instructions on what do you need installed in order for skia-safe to build Skia (C++) in the background.
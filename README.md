# Advent of code 2022

This is my attempt at the [Advent of Code 2022](https://adventofcode.com/2022) challenges by using Rust.

## Run with Cargo

Cargo will look for a `src/main.rs` file to build and/or run and if it finds a `src/lib.rs` file, it will build a library which then can be required by a different crate.  

You run your Rust project with Cargo like this:
```bash
$ cargo run
   Compiling advent-of-code-2022 v0.1.0 (/Users/ivicabatinic/Documents/projects/DEV_PLAYGROUND/advent-of-code-2022)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/advent-of-code-2022`
Hello world!
```
To ignore the Cargo output use -q (for --quiet):

```bash
$ cargo -q run
Hello world!
```

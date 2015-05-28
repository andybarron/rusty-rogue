# Rusty Rogue
_(Name subject to change)_

## Overview

Rusty Rogue is a (prototype) procedurally generated, real-time action roguelike written in the [Rust programming language](http://www.rust-lang.org). It was originally built as a course project for the Spring 2014 section of UVA's [CS 4414 Operating Systems](http://www.rust-class.org) class. The game is a proof-of-concept for Rust's concurrency and game development capabilities. The main features are:

* __Deterministic procedural generation__
* __Two-dimensional graphics__
* __Concurrent graph search using A*__

## Requirements

* [Rust](http://www.rust-lang.org/), a new totally awesome systems programming language. This project is currently using Rust 1.0 stable.
* [Cargo](http://doc.crates.io/), Rust's wonderful package manager. May or may not be included with your Rust install.
* [SFML](http://www.sfml-dev.org/) version 2.2, a cross-platform multimedia library. Make sure to install the right version!
* [CSFML](http://www.sfml-dev.org/download/csfml/) version 2.2. C bindings for SFML.
* The Rust library will be automatically downloaded and compiled by Cargo. (See `Cargo.toml` for the list.)

## Instructions

__This project is currently undergoing major refactoring. At present, only the dungeon generation test will be compiled/run.__

To compile: `cargo build`

To run: `cargo run`


## Resources

All the assets used are from the amazing [OpenGameArt.org](http://opengameart.org/)! Links to the individual asset pack(s) will be added here soon.


## License
_The Unlicense_

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>
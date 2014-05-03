# Rusty Rogue


## Overview

Rusty Rogue is a (prototype) procedurally generated, real-time action roguelike written in the [Rust programming language](http://www.rust-lang.org) for UVA's [CS 4414 Operating Systems](http://www.rust-class.org) course. The game is a proof-of-concept for Rust's concurrency and game development capabilities. The main features are:

* __Deterministic procedural generation__
* __Two-dimensional graphics__
* __Concurrent graph search using A*__


## Instructions

A Makefile is provided, so use `make` to compile, or `make run` to compile and launch.

While running the game, use the arrow keys to navigate the dungeon. The G and L keys toggle debug visualizations for the navigation graph and line of sight, respectively; the D key toggles all debug visualizations.

To try an interactive command-line test of the dungeon generator, run `make run MAINFILE=gen_test.rs`.

To try a randomized command-line test of A* search, run `make run MAINFILE=search_test.rs`.


## Requirements

* [Rust](http://www.rust-lang.org/), a new and lightweight systems programming language. This project is currently using Rust 0.11-pre, but might soon be ported to the master branch of [Rust's Git repository](https://github.com/mozilla/rust), or to a stable distribution.
* [Rust SFML](http://rust-sfml.org/), a fantastic Rust binding of the popular [SFML](http://www.sfml-dev.org/). This should be compiled to a .rlib file, which should be placed in the lib/rsfml folder. A Mac OS 10.9 version is included, but any other platforms will require you to compile Rust SFML yourself. Sorry!


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
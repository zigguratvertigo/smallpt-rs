[![license](https://img.shields.io/github/license/mashape/apistatus.svg)]()
[![Build Status](https://travis-ci.org/zigguratvertigo/smallpt-rs.svg?branch=master)](https://travis-ci.org/zigguratvertigo/smallpt-rs)

A [Rust](https://www.rust-lang.org/) implementation of a small ray/pathtracer.

Inspired by [Kevin Beason's educational 99-line raytracer/pathtracer](http://www.kevinbeason.com/smallpt/).

![alt text](https://github.com/zigguratvertigo/smallpt-rs/blob/master/smallpt.png)

## External Dependencies
smallpt-rs relies on the following [crates](https://crates.io):
- [rand](https://crates.io/crates/rand): library for random number generation
- [cgmath](https://crates.io/crates/cgmath): linear algebra and mathematics library for computer graphics
- [num_cpus](https://crates.io/crates/num_cpus): count the number of CPUs on the current machine
- [minifb](https://crates.io/crates/minifb): Cross-platform window setup with optional bitmap rendering
- [rayon](https://crates.io/crates/rayon): data-parallelism library for Rust
- [structopt](https://crates.io/crates/structopt): parse command line argument by defining a struct.

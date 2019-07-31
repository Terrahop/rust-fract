# **rust-fract**

[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

### **NOTE, if you are viewing this on github, you are looking at a mirror, original is on gitlab [here](https://gitlab.com/Terrahop/rust-fract) for issues and PR's.**

>A simple fractal viewing game in rust using [ggez](https://github.com/ggez/ggez).

## Table of Contents

- [Features](#features)
- [Compatibility](#compatibility)
- [Requirements](#requirements)
- [Usage](#usage)
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

## Features

### Implemented

* Mandelbrot set rendering
* Zoom
* Panning
* Saving and loading coordinates

### Planned

* Julia set rendering
* Menu for switching fractal's and general settings(quality, resolution etc)
* Multithreading
* Saving and loading multiple different coordinates

## Compatibility

* Linux: Supported
* Windows: Unsupported
* MacOS: Unsupported

## Requirements

* Hardware supporting OpenGL 3.2 or later
* Rust `stable-x86_64-unknown-linux-gnu` toolchain

## Usage

### Setup

```
git clone https://github.com/Terrahop/rust-fract.git
cd rust-fract
cargo run --release
```

### In-game

* `x` and `z` to zoom in and out respectively.
* Arrow keys to pan view.
* `s` to save coordinates and `l` to load last saved coordinates.
* `=/+` to increase quality(iterations).
* `0` to decrease quality(iterations).
* `Escape` to quit.

## Maintainers

Alex @terrahop

## Contributing

PRs accepted.

Please conform to the [karma](https://karma-runner.github.io/3.0/dev/git-commit-msg.html) commit convention.

## License

GNU Affero General Public License v3.0 only @ terrahop

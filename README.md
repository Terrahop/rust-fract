# **rust-fract**

### **NOTE, if you are viewing this on github, you are looking at a mirror, original is on gitlab [here](https://gitlab.com/Terrahop/rust-fract)**

A simple fractal viewing game in rust using [ggez](https://github.com/ggez/ggez)

# Overview

## Features

### Implemented
* Mandelbrot set rendering
* Zoom
* Panning
* Saving and loading coordinates

### To-Do
* Julia set rendering
* Menu for switching fractal's and general settings(quality, resolution etc)
* Multithreading
* Saving and loading multiple different coordinates

## Supported Platforms

* Linux: Supported
* Windows: Unsupported
* MacOS: Unsupported

## Requirements
* OpenGL 3.2 or later
* Decent hardware or you'll be waiting a while

# Usage

## Setup
```
git clone https://github.com/Terrahop/rust-fract.git
cd rust-fract
cargo run --release
```

## In-game

* `x` and `z` to zoom in and out respectively
* Arrow keys to pan view
* `s` to save coordinates and `l` to load coordinates last save coordinates
* `=/+` to increase quality(iterations)
* `-/_` to decrease quality(iterations)
* `Escape` to quit

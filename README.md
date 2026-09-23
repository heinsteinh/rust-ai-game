# Rust AI Game

A small game/sandbox engine written in Rust, built on SDL2 and OpenGL.

Right now it opens a 1280×720 window with an OpenGL 4.6 core debug context and clears it to dark green every frame. At startup it prints the OpenGL version, vendor and renderer, and any OpenGL errors or warnings are printed to stderr. More will be built on top of this.

## Requirements

- Rust (stable) with Cargo. Install it from [rustup.rs](https://rustup.rs).
- SDL2 development libraries
- A GPU and driver that support **OpenGL 4.6**

Install SDL2:

```sh
# Arch Linux
sudo pacman -S sdl2-compat

# Debian / Ubuntu
sudo apt install libsdl2-dev

# Fedora
sudo dnf install SDL2-devel

# macOS (Homebrew)
brew install sdl2
```

## Build and run

```sh
git clone https://github.com/heinsteinh/rust-ai-game.git
cd rust-ai-game
cargo run            # debug build
cargo run --release  # optimized build
```

## Controls

| Key / action     | Effect |
|------------------|--------|
| `Esc`            | Quit   |
| Close the window | Quit   |

## Project structure

```
src/
├── main.rs            # Entry point and main event loop
└── engine/
    ├── mod.rs
    ├── window.rs      # GameWindow: SDL2 setup, window, OpenGL context
    ├── renderer.rs    # Renderer: loads OpenGL functions, clears the screen
    ├── opengl.rs      # OpenGL info and the debug message callback
    └── error.rs       # EngineError type
```

## Troubleshooting

- **Panic: `trying to construct an enum from an invalid value`**
  The `sdl2` crate is older than the SDL2 library installed on your system. This project uses `sdl2 = "0.38"`, which works with current SDL2 releases. Don't downgrade it.
- **The window or OpenGL context fails to open**
  Your GPU or driver may not support OpenGL 4.6. To use an older version, lower the version in `src/engine/window.rs` (`gl_attr.set_context_version(4, 6)`). 3.3 works on most hardware.

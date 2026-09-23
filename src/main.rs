mod engine;

use engine::renderer::Renderer;
use engine::window::GameWindow;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let game_window = GameWindow::new("Rust AI Sandbox", 1280, 720).unwrap();

    let renderer =
        Renderer::new(&game_window.window, game_window.width, game_window.height).unwrap();

    let mut event_pump = game_window.sdl.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        renderer.clear();

        game_window.window.gl_swap_window();
    }
}

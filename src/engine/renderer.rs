use crate::engine::error::EngineError;
use crate::engine::opengl::{self, OpenGLInfo};

pub struct Renderer;

impl Renderer {
    pub fn new(
        window: &sdl2::video::Window,
        width: u32,
        height: u32,
    ) -> Result<Renderer, EngineError> {
        gl::load_with(|name| window.subsystem().gl_get_proc_address(name) as *const _);

        println!("{}", OpenGLInfo::get_info());

        if !opengl::enable_debug_output() {
            eprintln!("OpenGL debug output unavailable (not a debug context or GL < 4.3)");
        }

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        Ok(Self {})
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(0.1, 0.2, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

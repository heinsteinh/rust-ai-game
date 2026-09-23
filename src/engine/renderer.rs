use crate::engine::error::EngineError;

pub struct Renderer;

impl Renderer {
    pub fn new(window: &sdl2::video::Window) -> Result<Renderer, EngineError> {
        gl::load_with(|name| window.subsystem().gl_get_proc_address(name) as *const _);

        unsafe {
            gl::Viewport(0, 0, 800, 600);
        }

        Ok(Self {})
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

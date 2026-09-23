use sdl2::{
    video::{GLContext, Window},
    Sdl,
};

use crate::engine::error::EngineError;

pub struct GameWindow {
    pub sdl: Sdl,

    pub window: Window,
    pub width: u32,
    pub height: u32,

    #[allow(dead_code)]
    pub gl_context: GLContext,
}

impl GameWindow {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, EngineError> {
        let sdl = sdl2::init().map_err(EngineError::SDLError)?;

        let video = sdl.video().map_err(EngineError::SDLError)?;

        let gl_attr = video.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

        gl_attr.set_context_flags().debug().set();

        gl_attr.set_context_version(4, 6);

        let window = video
            .window(title, width, height)
            .opengl()
            .resizable()
            .build()
            .map_err(|e| EngineError::WindowError(e.to_string()))?;

        let gl_context = window
            .gl_create_context()
            .map_err(EngineError::WindowError)?;

        Ok(Self {
            sdl,

            window,
            width,
            height,

            gl_context,
        })
    }
}

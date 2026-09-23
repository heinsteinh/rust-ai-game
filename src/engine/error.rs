use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("SDL error: {0}")]
    SDLError(String),

    #[error("Window error: {0}")]
    WindowError(String),

    #[allow(dead_code)]
    #[error("OpenGL error")]
    OpenGLError,
}

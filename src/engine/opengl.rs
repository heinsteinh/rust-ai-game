pub struct OpenGLInfo;

impl OpenGLInfo {
    pub fn get_info() -> String {
        let version = unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8) }
            .to_str()
            .unwrap_or("Unknown");
        let vendor = unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VENDOR) as *const i8) }
            .to_str()
            .unwrap_or("Unknown");
        let renderer =
            unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::RENDERER) as *const i8) }
                .to_str()
                .unwrap_or("Unknown");

        format!(
            "OpenGL Version: {}\nVendor: {}\nRenderer: {}",
            version, vendor, renderer
        )
    }
}

/// Routes OpenGL debug messages to stderr.
///
/// Only has an effect when the context was created with the debug flag
/// (`gl_attr.set_context_flags().debug().set()`) and supports OpenGL 4.3+.
/// Returns `true` if the callback was installed.
pub fn enable_debug_output() -> bool {
    if !gl::DebugMessageCallback::is_loaded() {
        return false;
    }

    let mut flags = 0;
    unsafe { gl::GetIntegerv(gl::CONTEXT_FLAGS, &mut flags) };
    if flags as u32 & gl::CONTEXT_FLAG_DEBUG_BIT == 0 {
        return false;
    }

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        // Report messages from the GL call that caused them, so backtraces are useful
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageCallback(Some(debug_callback), std::ptr::null());
    }
    true
}

extern "system" fn debug_callback(
    source: gl::types::GLenum,
    gltype: gl::types::GLenum,
    id: gl::types::GLuint,
    severity: gl::types::GLenum,
    length: gl::types::GLsizei,
    message: *const gl::types::GLchar,
    _user_param: *mut std::ffi::c_void,
) {
    let message = if length >= 0 {
        let bytes = unsafe { std::slice::from_raw_parts(message as *const u8, length as usize) };
        String::from_utf8_lossy(bytes)
    } else {
        unsafe { std::ffi::CStr::from_ptr(message) }.to_string_lossy()
    };

    let source = match source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "Window System",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "Shader Compiler",
        gl::DEBUG_SOURCE_THIRD_PARTY => "Third Party",
        gl::DEBUG_SOURCE_APPLICATION => "Application",
        _ => "Other",
    };
    let gltype = match gltype {
        gl::DEBUG_TYPE_ERROR => "Error",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined Behavior",
        gl::DEBUG_TYPE_PORTABILITY => "Portability",
        gl::DEBUG_TYPE_PERFORMANCE => "Performance",
        gl::DEBUG_TYPE_MARKER => "Marker",
        _ => "Other",
    };
    let severity = match severity {
        gl::DEBUG_SEVERITY_HIGH => "HIGH",
        gl::DEBUG_SEVERITY_MEDIUM => "MEDIUM",
        gl::DEBUG_SEVERITY_LOW => "LOW",
        _ => "NOTIFICATION",
    };

    eprintln!("[GL {severity}] {source} {gltype} ({id}): {message}");
}

use gl::types::*;

/// Buffer Name Object
#[derive(Debug, Clone, Copy)]
pub struct Buffer {
    pub(crate) id: GLuint,
}

/// Framebuffer Name Object
#[derive(Debug, Clone, Copy)]
pub struct Framebuffer {
    pub(crate) id: GLuint,
}

/// Shader Object
#[derive(Debug, Clone, Copy)]
pub struct Shader {
    pub(crate) id: GLuint,
}

/// Program Object
#[derive(Debug, Clone, Copy)]
pub struct Program {
    pub(crate) id: GLuint,
}

/// Vertex Array Name Object
#[derive(Debug, Clone, Copy)]
pub struct VertexArray {
    pub(crate) id: GLuint,
}

/// Location of a specific uniform variable within a program
#[derive(Debug, Clone, Copy)]
pub struct UniformLocation {
    pub(crate) location: GLint,
}

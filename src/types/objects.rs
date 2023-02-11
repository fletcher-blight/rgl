use gl::types::*;

/// Buffer Name Object
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Buffer(pub(crate) GLuint);

/// Framebuffer Name Object
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Framebuffer(pub(crate) GLuint);

/// Shader Object
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Shader(pub(crate) GLuint);

/// Texture Object
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Texture(pub(crate) GLuint);

/// Program Object
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Program(pub(crate) GLuint);

/// Vertex Array Name Object
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct VertexArray(pub(crate) GLuint);

/// Location of a specific uniform variable within a program
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct UniformLocation(pub(crate) GLint);

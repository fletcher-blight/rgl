use crate::*;

/// Actual API Errors
#[derive(Debug, Clone)]
pub enum Error {
    OpenGL(ErrorOpenGL),
    Unreachable(ErrorOpenGL),
    ConversionFailure(String),

    NonOpenGLBuffer(Buffer),
    NonOpenGLFramebuffer(Framebuffer),
    NonOpenGLProgram(Program),
    NonOpenGLShader(Shader),
    NonOpenGLVertexArray(VertexArray),

    NotABuffer(Buffer),
    NotAProgram(Program),
    NotAShader(Shader),

    ShaderAlreadyAttachedToProgram(Program, Shader),
    TransportFeedbackModeActive(Program),
    ProgramCannotBeUsed(Program),

    MissingComputeShader(Program),
    MissingGeometryShader(Program),
    UnknownUniformName(std::ffi::CString),
    UnlinkedProgram(Program),

    BufferTargetNull(BufferBindingTarget),
}

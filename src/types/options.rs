use crate::*;
use gl::types::*;

/// Buffer Name Target Type
#[derive(Debug, Clone, Copy)]
pub enum BufferBindingTarget {
    /// Vertex attributes
    Array,

    /// Atomic counter storage
    AtomicCounter,

    /// Buffer copy source
    CopyRead,

    /// Buffer copy destination
    CopyWrite,

    /// Indirect compute dispatch commands
    DispatchIndirect,

    /// Indirect command arguments
    DrawIndirect,

    /// Vertex array indices
    ElementArray,

    /// Pixel read target
    PixelPack,

    /// Texture data source
    PixelUnpack,

    /// Query result buffer
    Query,

    /// Read-write storage for shaders
    ShaderStorage,

    /// Texture data buffer
    Texture,

    /// Transform feedback buffer
    TransformFeedbackBuffer,

    /// Uniform block storage
    Uniform,
}

impl From<BufferBindingTarget> for GLenum {
    fn from(value: BufferBindingTarget) -> Self {
        match value {
            BufferBindingTarget::Array => gl::ARRAY_BUFFER,
            BufferBindingTarget::AtomicCounter => gl::ATOMIC_COUNTER_BUFFER,
            BufferBindingTarget::CopyRead => gl::COPY_READ_BUFFER,
            BufferBindingTarget::CopyWrite => gl::COPY_WRITE_BUFFER,
            BufferBindingTarget::DispatchIndirect => gl::DISPATCH_INDIRECT_BUFFER,
            BufferBindingTarget::DrawIndirect => gl::DRAW_INDIRECT_BUFFER,
            BufferBindingTarget::ElementArray => gl::ELEMENT_ARRAY_BUFFER,
            BufferBindingTarget::PixelPack => gl::PIXEL_PACK_BUFFER,
            BufferBindingTarget::PixelUnpack => gl::PIXEL_UNPACK_BUFFER,
            BufferBindingTarget::Query => gl::QUERY_BUFFER,
            BufferBindingTarget::ShaderStorage => gl::SHADER_STORAGE_BUFFER,
            BufferBindingTarget::Texture => gl::TEXTURE_BUFFER,
            BufferBindingTarget::TransformFeedbackBuffer => gl::TRANSFORM_FEEDBACK_BUFFER,
            BufferBindingTarget::Uniform => gl::UNIFORM_BUFFER,
        }
    }
}

bitflags::bitflags! {
    /// Bitmask for [clear] to specify the desired buffer(s) to clear
    pub struct ClearMask: GLenum {
        /// Indicates the buffers currently enabled for color writing
        const COLOUR = gl::COLOR_BUFFER_BIT;
        /// Indicates the depth buffer
        const DEPTH = gl::DEPTH_BUFFER_BIT;
        /// Indicates the stencil buffer
        const STENCIL = gl::STENCIL_BUFFER_BIT;
    }
}

/// Currently defined OpenGL errors
#[derive(Debug, Clone, Copy)]
pub enum ErrorOpenGL {
    /// No error has been recorded
    NoError,

    /// An unacceptable value is specified for an enumerated argument. The offending command is
    /// ignored and has no other side effect than to set the error flag
    InvalidEnum,

    /// A numeric argument is out of range. The offending command is ignored and has no other
    /// side effect than to set the error flag
    InvalidValue,

    /// The specified operation is not allowed in the current state. The offending command is
    /// ignored and has no other side effect than to set the error flag
    InvalidOperation,

    /// The framebuffer object is not complete. The offending command is ignored and has no other
    /// side effect than to set the error flag
    InvalidFrameBufferOperation,

    /// There is not enough memory left to execute the command. The state of the GL is undefined,
    /// except for the state of the error flags, after this error is recorded
    OutOfMemory,

    /// An attempt has been made to perform an operation that would cause an internal stack to underflow
    StackUnderflow,

    /// An attempt has been made to perform an operation that would cause an internal stack to overflow
    StackOverflow,

    /// GL returned a non-standard error code
    Unknown(u32),
}

/// Framebuffer Name Target Type
#[derive(Debug, Clone, Copy)]
pub enum FramebufferBindingTarget {
    Draw,
    Read,
    ReadDraw,
}

impl From<FramebufferBindingTarget> for GLenum {
    fn from(value: FramebufferBindingTarget) -> Self {
        match value {
            FramebufferBindingTarget::Draw => gl::DRAW_FRAMEBUFFER,
            FramebufferBindingTarget::Read => gl::READ_FRAMEBUFFER,
            FramebufferBindingTarget::ReadDraw => gl::FRAMEBUFFER,
        }
    }
}

/// Index Buffer Integer Type
#[derive(Debug, Clone, Copy)]
pub enum IndicesType {
    U8,
    U16,
    U32,
}

impl From<IndicesType> for GLenum {
    fn from(value: IndicesType) -> Self {
        match value {
            IndicesType::U8 => gl::UNSIGNED_BYTE,
            IndicesType::U16 => gl::UNSIGNED_SHORT,
            IndicesType::U32 => gl::UNSIGNED_INT,
        }
    }
}

/// Primitive Render Draw Shape Type
#[derive(Debug, Clone, Copy)]
pub enum RenderPrimitive {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdjacency,
    LinesAdjacency,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdjacency,
    TrianglesAdjacency,
    Patches,
}

impl From<RenderPrimitive> for GLenum {
    fn from(value: RenderPrimitive) -> Self {
        match value {
            RenderPrimitive::Points => gl::POINTS,
            RenderPrimitive::LineStrip => gl::LINE_STRIP,
            RenderPrimitive::LineLoop => gl::LINE_LOOP,
            RenderPrimitive::Lines => gl::LINES,
            RenderPrimitive::LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
            RenderPrimitive::LinesAdjacency => gl::LINES_ADJACENCY,
            RenderPrimitive::TriangleStrip => gl::TRIANGLE_STRIP,
            RenderPrimitive::TriangleFan => gl::TRIANGLE_FAN,
            RenderPrimitive::Triangles => gl::TRIANGLES,
            RenderPrimitive::TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
            RenderPrimitive::TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
            RenderPrimitive::Patches => gl::PATCHES,
        }
    }
}

impl TryFrom<GLenum> for RenderPrimitive {
    type Error = Error;
    fn try_from(value: GLenum) -> Result<Self, Self::Error> {
        match value {
            gl::POINTS => Ok(RenderPrimitive::Points),
            gl::LINE_STRIP => Ok(RenderPrimitive::LineStrip),
            gl::LINE_LOOP => Ok(RenderPrimitive::LineLoop),
            gl::LINES => Ok(RenderPrimitive::Lines),
            gl::LINE_STRIP_ADJACENCY => Ok(RenderPrimitive::LineStripAdjacency),
            gl::LINES_ADJACENCY => Ok(RenderPrimitive::LinesAdjacency),
            gl::TRIANGLE_STRIP => Ok(RenderPrimitive::TriangleStrip),
            gl::TRIANGLE_FAN => Ok(RenderPrimitive::TriangleFan),
            gl::TRIANGLES => Ok(RenderPrimitive::Triangles),
            gl::TRIANGLE_STRIP_ADJACENCY => Ok(RenderPrimitive::TriangleStripAdjacency),
            gl::TRIANGLES_ADJACENCY => Ok(RenderPrimitive::TrianglesAdjacency),
            gl::PATCHES => Ok(RenderPrimitive::Patches),
            _ => Err(Error::ConversionFailure(format!(
                "Invalid Render Primitive: {value}"
            ))),
        }
    }
}

/// Shader Object Types
#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    /// intended to run on the programmable compute processor
    Compute,
    /// intended to run on the programmable vertex processor
    Vertex,
    /// intended to run on the programmable tessellation processor in the control stage
    TessControl,
    /// intended to run on the programmable tessellation processor in the evaluation stage
    TessEvaluation,
    /// intended to run on the programmable geometry processor
    Geometry,
    /// intended to run on the programmable fragment processor
    Fragment,
}

impl From<ShaderType> for GLenum {
    fn from(value: ShaderType) -> Self {
        match value {
            ShaderType::Compute => gl::COMPUTE_SHADER,
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::TessControl => gl::TESS_CONTROL_SHADER,
            ShaderType::TessEvaluation => gl::TESS_EVALUATION_SHADER,
            ShaderType::Geometry => gl::GEOMETRY_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

impl TryFrom<GLenum> for ShaderType {
    type Error = Error;
    fn try_from(value: GLenum) -> Result<Self, Self::Error> {
        match value {
            gl::COMPUTE_SHADER => Ok(ShaderType::Compute),
            gl::VERTEX_SHADER => Ok(ShaderType::Vertex),
            gl::TESS_CONTROL_SHADER => Ok(ShaderType::TessControl),
            gl::TESS_EVALUATION_SHADER => Ok(ShaderType::TessEvaluation),
            gl::GEOMETRY_SHADER => Ok(ShaderType::Geometry),
            gl::FRAGMENT_SHADER => Ok(ShaderType::Fragment),
            _ => Err(Error::ConversionFailure(format!(
                "Invalid Shader Type: {value}"
            ))),
        }
    }
}

/// Transform Feedback Buffer Capturing Mode
#[derive(Debug, Clone, Copy)]
pub enum TransformFeedbackBufferMode {
    Interleaved,
    Separate,
}

impl From<TransformFeedbackBufferMode> for GLenum {
    fn from(value: TransformFeedbackBufferMode) -> Self {
        match value {
            TransformFeedbackBufferMode::Interleaved => gl::INTERLEAVED_ATTRIBS,
            TransformFeedbackBufferMode::Separate => gl::SEPARATE_ATTRIBS,
        }
    }
}

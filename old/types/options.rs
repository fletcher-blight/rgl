use crate::*;
use gl::types::*;

/// Buffer Name Target Type
///
/// See [bind_buffer]
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

/// Buffer Usage Frequency
///
/// the frequency of access (modification and usage)
/// See [mod@buffer_data]
#[derive(Debug, Clone, Copy)]
pub enum BufferUsageFrequency {
    /// The data store contents will be modified once and used at most a few times.
    Stream,
    /// The data store contents will be modified once and used many times.
    Static,
    /// The data store contents will be modified repeatedly and used many times.
    Dynamic,
}

/// Buffer Usage Nature of Access
///
/// See [mod@buffer_data]
#[derive(Debug, Clone, Copy)]
pub enum BufferUsageNature {
    /// The data store contents are modified by the application, and used as the source for GL drawing
    /// and image specification commands.
    Draw,
    /// The data store contents are modified by reading data from the GL, and used to return that data
    /// when queried by the application.
    Read,
    /// The data store contents are modified by reading data from the GL, and used as the source for
    /// GL drawing and image specification commands.
    Copy,
}

/// Buffer Usage
///
/// is a hint to the GL implementation as to how a buffer object's data store will be accessed.
/// This enables the GL implementation to make more intelligent decisions that may significantly
/// impact buffer object performance. It does not, however, constrain the actual usage of the data store.
///
/// See [mod@buffer_data]
pub struct BufferUsage(pub BufferUsageFrequency, pub BufferUsageNature);

impl From<BufferUsage> for GLenum {
    fn from(BufferUsage(frequency, nature): BufferUsage) -> Self {
        match frequency {
            BufferUsageFrequency::Stream => match nature {
                BufferUsageNature::Draw => gl::STREAM_DRAW,
                BufferUsageNature::Read => gl::STREAM_READ,
                BufferUsageNature::Copy => gl::STREAM_COPY,
            },
            BufferUsageFrequency::Static => match nature {
                BufferUsageNature::Draw => gl::STATIC_DRAW,
                BufferUsageNature::Read => gl::STATIC_READ,
                BufferUsageNature::Copy => gl::STATIC_COPY,
            },
            BufferUsageFrequency::Dynamic => match nature {
                BufferUsageNature::Draw => gl::DYNAMIC_DRAW,
                BufferUsageNature::Read => gl::DYNAMIC_READ,
                BufferUsageNature::Copy => gl::DYNAMIC_COPY,
            },
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

/// Server-side GL capabilities
///
/// Use [is_enabled] or [get] to determine the current setting of any capability. The initial value
/// for each capability with the exception of [Dither] and [Multisample] is false.
#[derive(Debug, Clone, Copy)]
pub enum Capability {
    /// If enabled, blend the computed fragment color values with the values in the color buffers.
    /// See [blend_func].
    Blend,

    /// If enabled, apply the currently selected logical operation to the computed fragment color
    /// and color buffer values. See [logic_op].
    ColourLogicOp,

    /// If enabled, cull polygons based on their winding in window coordinates. See [cull_face].
    CullFace,

    /// If enabled, debug messages are produced by a debug context. When disabled, the debug message
    /// log is silenced. Note that in a non-debug context, very few, if any messages might be produced,
    /// even when [DebugOutput] is enabled.
    ///
    /// requires 4.3 or greater
    DebugOutput,

    /// If enabled, debug messages are produced synchronously by a debug context. If disabled, debug
    /// messages may be produced asynchronously. In particular, they may be delayed relative to the
    /// execution of GL commands, and the debug callback function may be called from a thread other
    /// than that in which the commands are executed. See [debug_message_callback].
    ///
    /// requires 4.3 or greater
    DebugOutputSynchronous,

    /// If enabled, the −w ≤ z ≤ w plane equation is ignored by view volume clipping (effectively,
    /// there is no near or far plane clipping). See [depth_range].
    DepthClamp,

    /// If enabled, do depth comparisons and update the depth buffer. Note that even if the depth
    /// buffer exists and the depth mask is non-zero, the depth buffer is not updated if the depth
    /// test is disabled. See [depth_func] and [depth_range].
    DepthTest,

    /// If enabled, dither color components or indices before they are written to the color buffer.
    Dither,

    /// If enabled and the value of [FramebufferAttachmentColourEncoding] for the framebuffer
    /// attachment corresponding to the destination buffer is [sRGB], the R, G, and B destination
    /// color values (after conversion from fixed-point to floating-point) are considered to be
    /// encoded for the sRGB color space and hence are linearized prior to their use in blending.
    FramebufferSRGB,

    /// If enabled, draw lines with correct filtering. Otherwise, draw aliased lines. See [line_width].
    LineSmooth,

    /// If enabled, use multiple fragment samples in computing the final color of a pixel.
    /// See [sample_coverage].
    Multisample,

    /// If enabled, and if the polygon is rendered in [Fill] mode, an offset is added to depth values
    /// of a polygon's fragments before the depth comparison is performed. See [polygon_offset].
    PolygonOffsetFill,

    /// If enabled, and if the polygon is rendered in [Line] mode, an offset is added to depth values
    /// of a polygon's fragments before the depth comparison is performed. See [polygon_offset].
    PolygonOffsetLine,

    /// If enabled, an offset is added to depth values of a polygon's fragments before the depth
    /// comparison is performed, if the polygon is rendered in [Point] mode. See [polygon_offset].
    PolygonOffsetPoint,

    /// If enabled, draw polygons with proper filtering. Otherwise, draw aliased polygons. For correct
    /// antialiased polygons, an alpha buffer is needed and the polygons must be sorted front to back.
    PolygonSmooth,

    /// Enables primitive restarting. If enabled, any one of the draw commands which transfers a set
    /// of generic attribute array elements to the GL will restart the primitive when the index of
    /// the vertex is equal to the primitive restart index. See [primitive_restart_index].
    ///
    /// request 3.1 or greater
    PrimitiveRestart,

    /// Enables primitive restarting with a fixed index. If enabled, any one of the draw commands
    /// which transfers a set of generic attribute array elements to the GL will restart the
    /// primitive when the index of the vertex is equal to the fixed primitive index for the specified
    /// index type. The fixed index is equal to 2n−1 where n is equal to the number of bytes.
    ///
    /// requires 4.3 or greater
    PrimitiveRestartFixedIndex,

    /// If enabled, primitives are discarded after the optional transform feedback stage, but before
    /// rasterization. Furthermore, when enabled, [clear], [clear_buffer_data], [clear_buffer_sub_data],
    /// [clear_texture_image], and [clear_texture_sub_image] are ignored.
    RasterizerDiscard,

    /// If enabled, compute a temporary coverage value where each bit is determined by the alpha
    /// value at the corresponding sample location. The temporary coverage value is then ANDed with
    /// the fragment coverage value.
    SampleAlphaToCoverage,

    /// If enabled, each sample alpha value is replaced by the maximum representable alpha value.
    SampleAlphaToOne,

    /// If enabled, the fragment's coverage is ANDed with the temporary coverage value. If
    /// [SampleCoverageInvert] is set, invert the coverage value. See [sample_coverage].
    SampleCoverage,

    /// If enabled, the active fragment shader is run once for each covered sample, or at fraction
    /// of this rate as determined by the current value of [MinSampleShadingValue].
    /// See [min_sample_shading].
    SampleShading,

    /// If enabled, the sample coverage mask generated for a fragment during rasterization will be
    /// ANDed with the value of [SampleMaskValue] before shading occurs. See [sample_mask].
    SampleMask,

    /// If enabled, discard fragments that are outside the scissor rectangle. See [scissor].
    ScissorTest,

    /// If enabled, do stencil testing and update the stencil buffer.
    /// See [stencil_func] and [stencil_op].
    StencilTest,

    /// If enabled, cubemap textures are sampled such that when linearly sampling from the border
    /// between two adjacent faces, texels from both faces are used to generate the final sample value.
    /// When disabled, texels from only a single face are used to construct the final sample value.
    ///
    /// requires 3.2 or greater
    TextureCubeMapSeamless,

    /// If enabled and a vertex or geometry shader is active, then the derived point size is taken
    /// from the (potentially clipped) shader builtin `gl_PointSize` and clamped to the
    /// implementation-dependent point size range.
    ProgramPointSize,
}

impl From<Capability> for GLenum {
    fn from(value: Capability) -> Self {
        match value {
            Capability::Blend => gl::BLEND,
            Capability::ColourLogicOp => gl::COLOR_LOGIC_OP,
            Capability::CullFace => gl::CULL_FACE,
            Capability::DebugOutput => gl::DEBUG_OUTPUT,
            Capability::DebugOutputSynchronous => gl::DEBUG_OUTPUT_SYNCHRONOUS,
            Capability::DepthClamp => gl::DEPTH_CLAMP,
            Capability::DepthTest => gl::DEPTH_TEST,
            Capability::Dither => gl::DITHER,
            Capability::FramebufferSRGB => gl::FRAMEBUFFER_SRGB,
            Capability::LineSmooth => gl::LINE_SMOOTH,
            Capability::Multisample => gl::MULTISAMPLE,
            Capability::PolygonOffsetFill => gl::POLYGON_OFFSET_FILL,
            Capability::PolygonOffsetLine => gl::POLYGON_OFFSET_LINE,
            Capability::PolygonOffsetPoint => gl::POLYGON_OFFSET_POINT,
            Capability::PolygonSmooth => gl::POLYGON_SMOOTH,
            Capability::PrimitiveRestart => gl::PRIMITIVE_RESTART,
            Capability::PrimitiveRestartFixedIndex => gl::PRIMITIVE_RESTART_FIXED_INDEX,
            Capability::RasterizerDiscard => gl::RASTERIZER_DISCARD,
            Capability::SampleAlphaToCoverage => gl::SAMPLE_ALPHA_TO_COVERAGE,
            Capability::SampleAlphaToOne => gl::SAMPLE_ALPHA_TO_ONE,
            Capability::SampleCoverage => gl::SAMPLE_COVERAGE,
            Capability::SampleShading => gl::SAMPLE_SHADING,
            Capability::SampleMask => gl::SAMPLE_MASK,
            Capability::ScissorTest => gl::SCISSOR_TEST,
            Capability::StencilTest => gl::STENCIL_TEST,
            Capability::TextureCubeMapSeamless => gl::TEXTURE_CUBE_MAP_SEAMLESS,
            Capability::ProgramPointSize => gl::PROGRAM_POINT_SIZE,
        }
    }
}

/// Depth comparison function
///
/// See [depth_func] for more details
#[derive(Debug, Clone, Copy)]
pub enum DepthFunc {
    /// Never passes.
    Never,

    /// Passes if the incoming depth value is less than the stored depth value.
    Less,

    /// Passes if the incoming depth value is equal to the stored depth value.
    Equal,

    /// Passes if the incoming depth value is less than or equal to the stored depth value.
    LessOrEqual,

    /// Passes if the incoming depth value is greater than the stored depth value.
    Greater,

    /// Passes if the incoming depth value is not equal to the stored depth value.
    NotEqual,

    /// Passes if the incoming depth value is greater than or equal to the stored depth value.
    GreaterOrEqual,

    /// Always passes.
    Always,
}

impl From<DepthFunc> for GLenum {
    fn from(value: DepthFunc) -> Self {
        match value {
            DepthFunc::Never => gl::NEVER,
            DepthFunc::Less => gl::LESS,
            DepthFunc::Equal => gl::EQUAL,
            DepthFunc::LessOrEqual => gl::LEQUAL,
            DepthFunc::Greater => gl::GREATER,
            DepthFunc::NotEqual => gl::NOTEQUAL,
            DepthFunc::GreaterOrEqual => gl::GEQUAL,
            DepthFunc::Always => gl::ALWAYS,
        }
    }
}

/// Currently defined OpenGL errors
///
/// Retrieve from [get_error]
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
///
/// See [bind_framebuffer]
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
///
/// See [create_shader]
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

/// Texture Bind Targets
///
/// See [bind_texture]
#[derive(Debug, Clone, Copy)]
pub enum TextureBindingTarget {
    Image1D,
    Image2D,
    Image3D,
    Array1D,
    Array2D,
    Rectangle,
    CubeMap,
    CubeMapArray,
    Buffer,
    Image2DMultisample,
    Image2DMultisampleArray,
}

impl From<TextureBindingTarget> for GLenum {
    fn from(value: TextureBindingTarget) -> Self {
        match value {
            TextureBindingTarget::Image1D => gl::TEXTURE_1D,
            TextureBindingTarget::Image2D => gl::TEXTURE_2D,
            TextureBindingTarget::Image3D => gl::TEXTURE_3D,
            TextureBindingTarget::Array1D => gl::TEXTURE_1D_ARRAY,
            TextureBindingTarget::Array2D => gl::TEXTURE_2D_ARRAY,
            TextureBindingTarget::Rectangle => gl::TEXTURE_RECTANGLE,
            TextureBindingTarget::CubeMap => gl::TEXTURE_CUBE_MAP,
            TextureBindingTarget::CubeMapArray => gl::TEXTURE_CUBE_MAP_ARRAY,
            TextureBindingTarget::Buffer => gl::TEXTURE_BUFFER,
            TextureBindingTarget::Image2DMultisample => gl::TEXTURE_2D_MULTISAMPLE,
            TextureBindingTarget::Image2DMultisampleArray => gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
        }
    }
}

/// Depth or Stencil mode for Textures
///
/// See [Depth Stencil Mode](texture_parameter#depth-stencil-mode)
#[derive(Debug, Clone, Copy)]
pub enum TextureDepthStencilMode {
    Depth,
    Stencil,
}

impl From<TextureDepthStencilMode> for GLenum {
    fn from(value: TextureDepthStencilMode) -> Self {
        match value {
            TextureDepthStencilMode::Depth => gl::DEPTH_COMPONENT,
            TextureDepthStencilMode::Stencil => gl::STENCIL_INDEX,
        }
    }
}

/// Texture Magnification Filter Options
///
/// See [Mag Filter](texture_parameter#mag-filter)
#[derive(Debug, Clone, Copy)]
pub enum TextureMagFilter {
    /// Returns the value of the texture element that is nearest (in Manhattan distance) to the
    /// specified texture coordinates.
    Nearest,

    /// Returns the weighted average of the texture elements that are closest to the specified
    /// texture coordinates. These can include items wrapped or repeated from other parts of a texture,
    /// depending on the values of [WrapS](TextureWrapTarget::S) and [WrapT](TextureWrapTarget::T),
    /// and on the exact mapping.
    Linear,
}

impl From<TextureMagFilter> for GLenum {
    fn from(value: TextureMagFilter) -> Self {
        match value {
            TextureMagFilter::Nearest => gl::NEAREST,
            TextureMagFilter::Linear => gl::LINEAR,
        }
    }
}

/// Texture Minifying Filter Options
///
/// See [Min Filter](texture_parameter#min-filter)
#[derive(Debug, Clone, Copy)]
pub enum TextureMinFilter {
    /// Returns the value of the texture element that is nearest (in Manhattan distance) to the
    /// specified texture coordinates.
    Nearest,

    /// Returns the weighted average of the four texture elements that are closest to the specified
    /// texture coordinates. These can include items wrapped or repeated from other parts of a texture,
    /// depending on the values of [WrapS](TextureWrapTarget::S) and [WrapT](TextureWrapTarget::T),
    /// and on the exact mapping.
    Linear,

    /// Chooses the mipmap that most closely matches the size of the pixel being textured and uses
    /// the [Nearest](TextureMinFilter::Nearest) criterion (the texture element closest to the
    /// specified texture coordinates) to produce a texture value.
    NearestMipmapNearest,

    /// Chooses the mipmap that most closely matches the size of the pixel being textured and uses
    /// the [Linear](TextureMinFilter::Linear) criterion (a weighted average of the four texture
    /// elements that are closest to the specified texture coordinates) to produce a texture value.
    LinearMipmapNearest,

    /// Chooses the two mipmaps that most closely match the size of the pixel being textured and
    /// uses the [Nearest](TextureMinFilter::Nearest) criterion (the texture element closest to the
    /// specified texture coordinates) to produce a texture value from each mipmap. The final texture
    /// value is a weighted average of those two values.
    NearestMipmapLinear,

    /// Chooses the two mipmaps that most closely match the size of the pixel being textured and uses
    /// the [Linear](TextureMinFilter::Linear) criterion (a weighted average of the texture elements
    /// that are closest to the specified texture coordinates) to produce a texture value from each
    /// mipmap. The final texture value is a weighted average of those two values.
    LinearMipmapLinear,
}

impl From<TextureMinFilter> for GLint {
    fn from(value: TextureMinFilter) -> Self {
        (match value {
            TextureMinFilter::Nearest => gl::NEAREST,
            TextureMinFilter::Linear => gl::LINEAR,
            TextureMinFilter::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
            TextureMinFilter::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
            TextureMinFilter::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
            TextureMinFilter::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
        }) as GLint
    }
}

/// Texture Wrap Targets
///
/// See [wrap](texture_parameter#wrap)
#[derive(Debug, Clone, Copy)]
pub enum TextureWrapTarget {
    S,
    T,
    R,
}

impl From<TextureWrapTarget> for GLenum {
    fn from(value: TextureWrapTarget) -> Self {
        match value {
            TextureWrapTarget::S => gl::TEXTURE_WRAP_S,
            TextureWrapTarget::T => gl::TEXTURE_WRAP_T,
            TextureWrapTarget::R => gl::TEXTURE_WRAP_R,
        }
    }
}

/// Modes for each [Texture Wrap Target](TextureWrapTarget)
pub enum TextureWrapMode {
    /// causes a [TextureWrapTarget] coordinates to be clamped to the range \[12N,1−12N\], where N
    /// is the size of the texture in the direction of clamping.
    ClampToEdge,

    /// evaluates a [TextureWrapTarget] coordinates in a similar manner to
    /// [ClampToEdge](TextureWrapMode::ClampToEdge).  However, in cases where clamping would have
    /// occurred in [ClampToEdge](TextureWrapMode::ClampToEdge) mode, the fetched texel data is
    /// substituted with the values specified by [texture_target_border_colour].
    ClampToBorder,

    /// causes the integer part of the a [TextureWrapTarget] coordinate to be ignored; the GL uses
    /// only the fractional part, thereby creating a repeating pattern.
    Repeat,

    /// causes the a [TextureWrapTarget] coordinate to be set to the fractional part of the texture
    /// coordinate if the integer part is even; if the integer part is odd, then the texture
    /// coordinate is set to 1−frac(s), where frac(s) represents the fractional part of the coordinate.
    MirroredRepeat,

    /// causes the a [TextureWrapTarget] coordinate to be repeated as for
    /// [MirroredRepeat](TextureWrapMode::MirroredRepeat) for one repetition of the texture, at which
    /// point the coordinate to be clamped as in [ClampToEdge](TextureWrapMode::ClampToEdge).
    ///
    /// # Compatibility
    /// 4.4 or greater is required
    MirrorClampToEdge,
}

impl From<TextureWrapMode> for GLenum {
    fn from(value: TextureWrapMode) -> Self {
        match value {
            TextureWrapMode::ClampToEdge => gl::CLAMP_TO_EDGE,
            TextureWrapMode::ClampToBorder => gl::CLAMP_TO_BORDER,
            TextureWrapMode::MirroredRepeat => gl::MIRRORED_REPEAT,
            TextureWrapMode::Repeat => gl::REPEAT,
            TextureWrapMode::MirrorClampToEdge => gl::MIRROR_CLAMP_TO_EDGE,
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

/// Floating Point Data Types for a Vertex Attribute
///
/// See [vertex_attribute_pointer]
#[derive(Debug, Clone, Copy)]
pub enum VertexAttributeFloatType {
    Integer(VertexAttributeIntegerType),
    F16,
    F32,
    Fixed,
}

impl From<VertexAttributeFloatType> for GLenum {
    fn from(value: VertexAttributeFloatType) -> Self {
        match value {
            VertexAttributeFloatType::Integer(t) => t.into(),
            VertexAttributeFloatType::F16 => gl::HALF_FLOAT,
            VertexAttributeFloatType::F32 => gl::FLOAT,
            VertexAttributeFloatType::Fixed => gl::FIXED,
        }
    }
}

/// Integer Data Types for a Vertex Attribute
///
/// See [vertex_attribute_pointer]
#[derive(Debug, Clone, Copy)]
pub enum VertexAttributeIntegerType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
}

impl From<VertexAttributeIntegerType> for GLenum {
    fn from(value: VertexAttributeIntegerType) -> Self {
        match value {
            VertexAttributeIntegerType::U8 => gl::UNSIGNED_BYTE,
            VertexAttributeIntegerType::I8 => gl::BYTE,
            VertexAttributeIntegerType::U16 => gl::UNSIGNED_SHORT,
            VertexAttributeIntegerType::I16 => gl::SHORT,
            VertexAttributeIntegerType::U32 => gl::UNSIGNED_INT,
            VertexAttributeIntegerType::I32 => gl::INT,
        }
    }
}

/// Vertex Attribute Size Options
///
/// See [vertex_attribute_pointer]
#[derive(Debug, Clone, Copy)]
pub enum VertexAttributeSize {
    Single,
    Duple,
    Triple,
    Quad,
}

impl From<VertexAttributeSize> for GLint {
    fn from(value: VertexAttributeSize) -> Self {
        match value {
            VertexAttributeSize::Single => 1,
            VertexAttributeSize::Duple => 2,
            VertexAttributeSize::Triple => 3,
            VertexAttributeSize::Quad => 4,
        }
    }
}

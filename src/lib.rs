//! RGL - Minimal Safe Rust OpenGL Bindings
//!
//! # Overview
//! RGL uses the API reference documentation from <https://www.khronos.org/opengl/wiki/Category:Core_API_Reference>
//!

/// # Template Function Documentation
/// <http://kronos/docs>
///
/// # Arguments
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// ```
///
/// # Description
///
/// # Compatability
///
/// # Errors
///
/// # Associated Gets
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [my_func] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
#[allow(unused)]
struct Stub;

pub mod buffer;
pub mod get;
pub mod masks;
pub mod prelude;
pub mod shader_creation;
pub mod shader_query;
pub mod shader_state;
pub mod texture;
pub mod vertex_array;
pub mod vertex_render;

use gl::types::*;

pub use gl::load_with;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    NoError,
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory,
    StackUnderflow,
    StackOverflow,
    ImplementationSpecific(u32),
}

impl From<GLenum> for Error {
    fn from(value: GLenum) -> Self {
        match value {
            gl::NO_ERROR => Error::NoError,
            gl::INVALID_ENUM => Error::InvalidEnum,
            other => Error::ImplementationSpecific(other),
        }
    }
}

/// # Return error information
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGetError.xhtml>
pub fn get_error() -> Error {
    // SAFE: just an integer copy
    let error = unsafe { gl::GetError() };
    Error::from(error)
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

/// # Enable server-side GL capabilities
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glEnable.xhtml>
///
/// # Arguments
/// * `capability` - Specifies a GL capability.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// enable(Capability::DepthTest);
/// ```
///
/// # Description
/// [enable] and [disable] enable and disable various capabilities. Use [is_enabled] or [get] to
/// determine the current setting of any capability. The initial value for each capability with the
/// exception of [Capability::Dither] and [Capability::Multisample] is `false`. The initial value
/// for [Capability::Dither] and [Capability::Multisample] is `true`.
///
/// # Compatability
///
/// # Errors
///
/// # Associated Gets
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [enable] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [active_texture]
/// * [blend_func]
/// * [cull_face]
/// * [depth_func]
/// * [depth_range]
/// * [get]
/// * [is_enabled]
/// * [line_width]
/// * [logic_op]
/// * [point_size]
/// * [polygon_mode]
/// * [polygon_offset]
/// * [sample_coverage]
/// * [scissor]
/// * [stencil_func]
/// * [stencil_op]
/// * [tex_image_1d], [tex_image_2d], [tex_image_3d]
pub fn enable(capability: Capability) {
    let cap = GLenum::from(capability);
    unsafe { gl::Enable(cap) }
}

/// # disable server-side GL capabilities
/// see [enable]
pub fn disable(capability: Capability) {
    let cap = GLenum::from(capability);
    unsafe { gl::Disable(cap) }
}

//! # Whole Framebuffer
//! <https://www.khronos.org/opengl/wiki/Category:Core_API_Ref_Whole_Framebuffer>
//!
//! # Description
//! The core OpenGL API reference for functions that affect the
//! [entire framebuffer](https://www.khronos.org/opengl/wiki/Framebuffer) in some way. This includes
//! [fragment write masking](https://www.khronos.org/opengl/wiki/Write_Mask) and buffer clearing
//! operations.

use crate::prelude::*;
use gl::types::*;

bitflags::bitflags! {
    pub struct ClearMask : u32 {
        /// Indicates the buffers currently enabled for color writing.
        const COLOUR = gl::COLOR_BUFFER_BIT;

        /// Indicates the depth buffer.
        const DEPTH = gl::DEPTH_BUFFER_BIT;

        /// Indicates the stencil buffer.
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

/// # Stencil Target Face
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StencilMaskFace {
    Front,
    Back,
    FrontAndBack,
}

impl From<StencilMaskFace> for GLenum {
    fn from(value: StencilMaskFace) -> Self {
        match value {
            StencilMaskFace::Front => gl::FRONT,
            StencilMaskFace::Back => gl::BACK,
            StencilMaskFace::FrontAndBack => gl::FRONT_AND_BACK,
        }
    }
}

/// # Comparison Functions
pub enum CompareFunc {
    Never,
    Always,
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

impl From<CompareFunc> for GLenum {
    fn from(value: CompareFunc) -> Self {
        match value {
            CompareFunc::Never => gl::NEVER,
            CompareFunc::Less => gl::LESS,
            CompareFunc::Equal => gl::EQUAL,
            CompareFunc::LessOrEqual => gl::LEQUAL,
            CompareFunc::Greater => gl::GREATER,
            CompareFunc::NotEqual => gl::NOTEQUAL,
            CompareFunc::GreaterOrEqual => gl::GEQUAL,
            CompareFunc::Always => gl::ALWAYS,
        }
    }
}

/// # Clear buffers to preset values
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glClear.xhtml>
///
/// # Arguments
/// * `mask` - Bitwise OR of masks that indicate the buffers to be cleared.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// clear(ClearMask::COLOUR | ClearMask::DEPTH);
/// ```
///
/// # Description
/// [clear] sets the bitplane area of the window to values previously selected by [clear_colour],
/// [clear_depth], and [clear_stencil]. Multiple color buffers can be cleared simultaneously by
/// selecting more than one buffer at a time using [draw_buffer] glDrawBuffer.
///
/// The pixel ownership test, the scissor test, dithering, and the buffer writemasks affect the
/// operation of [clear]. The scissor box bounds the cleared region. Alpha function, blend function,
/// logical operation, stenciling, texture mapping, and depth-buffering are ignored by [clear].
///
/// [clear] takes a single argument that is the bitwise OR of several values indicating which buffer
/// is to be cleared.
///
/// The value to which each buffer is cleared depends on the setting of the clear value for that
/// buffer.
///
/// If a buffer is not present, then a [clear] directed at that buffer has no effect.
///
/// # Associated Gets
/// * [get_depth_clear_value]
/// * [get_colour_clear_value]
/// * [get_stencil_clear_value]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [clear] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [clear_colour]
/// * [clear_depth]
/// * [clear_stencil]
/// * [colour_mask]
/// * [depth_mask]
/// * [draw_buffer]
/// * [scissor]
/// * [stencil_mask]
pub fn clear(mask: ClearMask) {
    let mask = mask.bits;
    unsafe { gl::Clear(mask) }
}

/// # Specify clear values for the color buffers
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glClearColor.xhtml>
///
/// # Arguments
/// * `red`, `green`, `blue`, `alpha` - Specify the red, green, blue, and alpha values used when the
/// color buffers are cleared. The initial values are all 0.0.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// clear_colour(0.1, 0.1, 0.1, 0.1);
/// ```
///
/// # Description
/// [clear_colour] specifies the red, green, blue, and alpha values used by [clear] to clear the
/// colour buffers. Values specified by [clear_colour] are clamped to the range [0,1].
///
/// # Associated Gets
/// * [get_colour_clear]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [clear_colour] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [clear]
pub fn clear_colour(red: f32, green: f32, blue: f32, alpha: f32) {
    // SAFE: synchronous integer copy
    unsafe { gl::ClearColor(red, green, blue, alpha) }
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

/// # Disable server-side GL capabilities
/// see [enable]
pub fn disable(capability: Capability) {
    let cap = GLenum::from(capability);
    unsafe { gl::Disable(cap) }
}

/// # Enable or disable writing into the depth buffer
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glDepthMask.xhtml>
///
/// # Arguments
/// * `enabled` - Specifies whether the depth buffer is enabled for writing. If `enabled` is false,
/// depth buffer writing is disabled. Otherwise, it is enabled. Initially, depth buffer writing is
/// enabled.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// depth_mask(true);
/// ```
///
/// # Description
/// [depth_mask] specifies whether the depth buffer is enabled for writing. If flag is false, depth
/// buffer writing is disabled. Otherwise, it is enabled. Initially, depth buffer writing is enabled.
///
/// Even if the depth buffer exists and the depth mask is non-zero, the depth buffer is not updated
/// if the depth test is disabled. In order to unconditionally write to the depth buffer, the depth
/// test should be enabled and set to [depth_func]([DepthFunc::Always]).
///
/// # Associated Gets
/// * [get_depth_write_mask]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [depth_mask] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [colour_mask]
/// * [depth_func]
/// * [depth_range]
/// * [stencil_mask]
pub fn depth_mask(enabled: bool) {
    let flag = GLboolean::from(enabled);
    unsafe { gl::DepthMask(flag) }
}

/// # Specify the value used for depth buffer comparisons
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glDepthFunc.xhtml>
///
/// # Arguments
/// * `func` - Specifies the depth comparison function.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// depth_func(CompareFunc::Greater);
/// ```
///
/// # Description
/// [depth_func] specifies the function used to compare each incoming pixel depth value with the
/// depth value present in the depth buffer. The comparison is performed only if depth testing is
/// enabled. (See [enable] and [disable] of [Capability::DepthTest].)
///
/// `func` specifies the conditions under which the pixel will be drawn.
///
/// | [CompareFunc] | Usage |
/// |------------------|-------|
/// | Never | Never passes |
/// | Always | Always passes |
/// | Equal | Passes if the incoming depth value is equal to the stored depth value |
/// | NotEqual | Passes if the incoming depth value is not equal to the stored depth value |
/// | Less | Passes if the incoming depth value is less than the stored depth value |
/// | LessOrEqual | Passes if the incoming depth value is less than or equal to the stored depth value |
/// | Greater | Passes if the incoming depth value is greater than the stored depth value |
/// | GreaterOrEqual | Passes if the incoming depth value is greater than or equal to the stored depth value |
///
/// The initial value of func is [CompareFunc::Less]. Initially, depth testing is disabled. If depth
/// testing is disabled or if no depth buffer exists, it is as if the depth test always passes.
///
/// Even if the depth buffer exists and the depth mask is non-zero, the depth buffer is not updated
/// if the depth test is disabled. In order to unconditionally write to the depth buffer, the depth
/// test should be enabled and set to [CompareFunc::Always].
///
/// # Associated Gets
/// * [get_depth_func]
/// * [is_enabled]([Capability::DepthTest])
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [depth_func] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [depth_range]
/// * [enable]
/// * [polygon_offset]
pub fn depth_func(func: CompareFunc) {
    let func = GLenum::from(func);
    unsafe { gl::DepthFunc(func) }
}

/// # Control the front and back writing of individual bits in the stencil planes
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glStencilMask.xhtml>
///
/// # Arguments
/// * `mask` - Specifies a bit mask to enable and disable writing of individual bits in the stencil
/// planes. Initially, the mask is all 1's.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// stencil_mask(0xff00aa99);
/// ```
///
/// # Description
/// [stencil_mask] controls the writing of individual bits in the stencil planes. The least
/// significant `n` bits of `mask`, where `n` is the number of bits in the stencil buffer, specify a
/// mask. Where a `1` appears in the mask, it's possible to write to the corresponding bit in the
/// stencil buffer. Where a `0` appears, the corresponding bit is write-protected. Initially, all
/// bits are enabled for writing.
///
/// There can be two separate `mask` writemasks; one affects back-facing polygons, and the other
/// affects front-facing polygons as well as other non-polygon primitives. [stencil_mask] sets both
/// front and back stencil writemasks to the same values. Use [stencil_mask_separate] to set front
/// and back stencil writemasks to different values.
///
/// [stencil_mask] is the same as calling [stencil_mask_separate] with `face` set to
/// [StencilMaskFace::FrontAndBack], like so:
/// ```no_run
/// # use rgl::prelude::*;
/// fn equivilent_stencil_mask(mask: u32) {
///     stencil_mask_separate(StencilMaskFace::FrontAndBack, mask);
/// }
/// ```
///
/// # Associated Gets
/// * [get_stencil_writemask]
/// * [get_stencil_back_writemask]
/// * [get_stencil_bits]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [stencil_mask] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [colour_mask]
/// * [depth_mask]
/// * [stencil_mask]
/// * [stencil_func]
/// * [stencil_func_separate]
/// * [stencil_mask_separate]
/// * [stencil_op]
/// * [stencil_op_separate]
pub fn stencil_mask(mask: u32) {
    unsafe { gl::StencilMask(mask) }
}

/// # Control the front and/or back writing of individual bits in the stencil planes
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glStencilMaskSeparate.xhtml>
///
/// # Arguments
/// * `face` - Specifies whether the front and/or back stencil writemask is updated.
/// * `mask` - Specifies a bit mask to enable and disable writing of individual bits in the stencil
/// planes. Initially, the mask is all 1's.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// stencil_mask_separate(StencilMaskFace::Front, 0xFFFFFFFF);
/// stencil_mask_separate(StencilMaskFace::Back, 0x00000000);
/// ```
///
/// # Description
/// [stencil_mask_separate] controls the writing of individual bits in the stencil planes. The least
/// significant `n` bits of `mask`, where `n` is the number of bits in the stencil buffer, specify a
/// mask. Where a `1` appears in the mask, it's possible to write to the corresponding bit in the
/// stencil buffer. Where a `0` appears, the corresponding bit is write-protected. Initially, all
/// bits are enabled for writing.
///
/// There can be two separate mask writemasks; one affects back-facing polygons, and the other
/// affects front-facing polygons as well as other non-polygon primitives. See [stencil_mask] to
/// simply set both faces at once.
///
/// # Associated Gets
/// * [get_stencil_writemask]
/// * [get_stencil_back_writemask]
/// * [get_stencil_bits]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [stencil_mask_separate] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [colour_mask]
/// * [depth_mask]
/// * [stencil_mask]
/// * [stencil_func]
/// * [stencil_func_separate]
/// * [stencil_mask]
/// * [stencil_op]
/// * [stencil_op_separate]
pub fn stencil_mask_separate(face: StencilMaskFace, mask: u32) {
    let face = GLenum::from(face);
    unsafe { gl::StencilMaskSeparate(face, mask) }
}
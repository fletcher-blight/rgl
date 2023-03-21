//! # Post Fragment Shader Operations
//! <https://www.khronos.org/opengl/wiki/Category:Core_API_Ref_Post_Fragment_Shader_Operations>
//!
//! # Description
//! The core OpenGL API reference for functions that deal with state for various post-fragment
//! shader operations. These operations include:
//! * [Blending](https://www.khronos.org/opengl/wiki/Blending)
//! * [Stencil Test](https://www.khronos.org/opengl/wiki/Stencil_Test)
//! * [Depth Test](https://www.khronos.org/opengl/wiki/Depth_Test)
//! * [Scissor Test](https://www.khronos.org/opengl/wiki/Scissor_Test)
//! * [Logical Operation](https://www.khronos.org/opengl/wiki/Logical_Operation)

use crate::prelude::*;
use gl::types::*;

/// # Stencil Op Actions
/// see [stencil_op] or [stencil_op_separate]
pub enum StencilOp {
    /// Keeps the current value.
    Keep,

    /// Sets the stencil buffer value to 0.
    Zero,

    /// Sets the stencil buffer value to `reference`, as specified by [stencil_func].
    Replace,

    /// Increments the current stencil buffer value. Clamps to the maximum representable unsigned
    /// value.
    IncrementClamp,

    /// Increments the current stencil buffer value. Wraps stencil buffer value to zero when
    /// incrementing the maximum representable unsigned value.
    IncrementWrap,

    /// Decrements the current stencil buffer value. Clamps to 0.
    DecrementClamp,

    /// Decrements the current stencil buffer value. Wraps stencil buffer value to the maximum
    /// representable unsigned value when decrementing a stencil buffer value of zero.
    DecrementWrap,

    /// Bitwise inverts the current stencil buffer value.
    Invert,
}

impl From<StencilOp> for GLenum {
    fn from(value: StencilOp) -> Self {
        match value {
            StencilOp::Keep => gl::KEEP,
            StencilOp::Zero => gl::ZERO,
            StencilOp::Replace => gl::REPLACE,
            StencilOp::IncrementClamp => gl::INCR,
            StencilOp::IncrementWrap => gl::INCR_WRAP,
            StencilOp::DecrementClamp => gl::DECR,
            StencilOp::DecrementWrap => gl::DECR_WRAP,
            StencilOp::Invert => gl::INVERT,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BlendFactor {
    Zero,
    One,
    SourceColour,
    OneMinusSourceColour,
    DestColour,
    OneMinusDestColour,
    SourceAlpha,
    OneMinusSourceAlpha,
    DestAlpha,
    OneMinusDestAlpha,
    ConstantColour,
    OneMinusConstantColour,
    ConstantAlpha,
    OneMinusConstantAlpha,
    SourceAlphaSaturate,
    Source1Colour,
    OneMinusSource1Colour,
    Source1Alpha,
    OneMinusSource1Alpha,
}

impl From<BlendFactor> for GLenum {
    fn from(value: BlendFactor) -> Self {
        match value {
            BlendFactor::Zero => gl::ZERO,
            BlendFactor::One => gl::ONE,
            BlendFactor::SourceColour => gl::SRC_COLOR,
            BlendFactor::OneMinusSourceColour => gl::ONE_MINUS_SRC_COLOR,
            BlendFactor::DestColour => gl::DST_COLOR,
            BlendFactor::OneMinusDestColour => gl::ONE_MINUS_DST_COLOR,
            BlendFactor::SourceAlpha => gl::SRC_ALPHA,
            BlendFactor::OneMinusSourceAlpha => gl::ONE_MINUS_SRC_ALPHA,
            BlendFactor::DestAlpha => gl::DST_ALPHA,
            BlendFactor::OneMinusDestAlpha => gl::ONE_MINUS_DST_ALPHA,
            BlendFactor::ConstantColour => gl::CONSTANT_COLOR,
            BlendFactor::OneMinusConstantColour => gl::ONE_MINUS_CONSTANT_COLOR,
            BlendFactor::ConstantAlpha => gl::CONSTANT_ALPHA,
            BlendFactor::OneMinusConstantAlpha => gl::ONE_MINUS_CONSTANT_ALPHA,
            BlendFactor::SourceAlphaSaturate => gl::SRC_ALPHA_SATURATE,
            BlendFactor::Source1Colour => gl::SRC1_COLOR,
            BlendFactor::OneMinusSource1Colour => gl::ONE_MINUS_SRC1_COLOR,
            BlendFactor::Source1Alpha => gl::SRC1_ALPHA,
            BlendFactor::OneMinusSource1Alpha => gl::ONE_MINUS_SRC1_ALPHA,
        }
    }
}

/// # Specify pixel arithmetic
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBlendFunc.xhtml>
///
/// # Arguments
/// * `source_factor` - Specifies how the red, green, blue, and alpha source blending factors are
/// computed. The initial value is [BlendFactor::One].
/// * `dest_factor` - Specifies how the red, green, blue, and alpha destination blending factors are
/// computed. The initial value is [BlendFactor::One].
///
/// # Example
/// Transparency is best implemented using blend function ([BlendFactor::SourceAlpha],
/// [BlendFactor::OneMinusSourceAlpha]) with primitives sorted from farthest to nearest. Note that
/// this transparency calculation does not require the presence of alpha bitplanes in the frame
/// buffer. This is also useful for rendering antialiased points and lines in arbitrary order.
/// ```no_run
/// # use rgl::prelude::*;
/// blend_func(BlendFactor::SourceAlpha, BlendFactor::OneMinusSourceAlpha);
/// ```
///
/// Polygon antialiasing is optimized using blend function ([BlendFactor::SourceAlphaSaturate],
/// [BlendFactor::One]) with polygons sorted from nearest to farthest. (See [enable], [disable] with
/// [Capability::PolygonSmooth] argument for information on polygon antialiasing.) Destination alpha
/// bitplanes, which must be present for this blend function to operate correctly, store the
/// accumulated coverage.
/// ```no_run
/// # use rgl::prelude::*;
/// blend_func(BlendFactor::SourceAlphaSaturate, BlendFactor::One);
/// ```
///
/// # Description
/// Pixels can be drawn using a function that blends the incoming (source) RGBA values with the RGBA
/// values that are already in the frame buffer (the destination values). Blending is initially
/// disabled. Use [enable] and [disable] with argument [Capability::Blend] to enable and disable
/// blending.
///
/// [blend_func] defines the operation of blending for all draw buffers when it is enabled.
/// [blend_func_buffer] defines the operation of blending for a single draw buffer specified by
/// `buf` when enabled for that draw buffer. `source_factor` specifies which method is used to scale the
/// source color components. `dest_factor` specifies which method is used to scale the destination
/// colour components. The possible methods are described in the following table. Each method
/// defines four scale factors, one each for red, green, blue, and alpha. In the table and in
/// subsequent equations, first source, second source and destination color components are referred
/// to as (R<sub>s0</sub>, G<sub>s0</sub>, B<sub>s0</sub>, A<sub>s0</sub>), (R<sub>s1</sub>,
/// G<sub>s1</sub>, B<sub>s1</sub>, A<sub>s1</sub>) and (R<sub>d</sub>, G<sub>d</sub>,
/// B<sub>d</sub>, A<sub>d</sub>), respectively. The colour specified by [blend_colour] is referred
/// to as (R<sub>c</sub>, G<sub>c</sub>, B<sub>c</sub>, A<sub>c</sub>). They are understood to have
/// integer values between 0 and (k<sub>R</sub>, k<sub>G</sub>, k<sub>B</sub>, k<sub>A</sub>), where
///
/// k<sub>c</sub> = 2<sup>m<sub>c</sub></sup> - 1
///
/// and (m<sub>R</sub>, m<sub>G</sub>, m<sub>B</sub>, m<sub>A</sub>) is the number of red, green,
/// blue, and alpha bitplanes.
///
/// Source and destination scale factors are referred to as (s<sub>R</sub>, s<sub>G</sub>,
/// s<sub>B</sub>, s<sub>A</sub>) and (d<sub>R</sub>, d<sub>G</sub>, d<sub>B</sub>, d<sub>A</sub>).
/// The scale factors described in the table, denoted (f<sub>R</sub>, f<sub>G</sub>, f<sub>B</sub>,
/// f<sub>A</sub>), represent either source or destination factors. All scale factors have range
/// \[0,1\].
///
/// | [BlendFactor] | (f<sub>R</sub>, f<sub>G</sub>, f<sub>B</sub>, f<sub>A</sub>) |
/// |---------------|--------------------------------------------------------------|
/// | [BlendFactor::Zero] | (0, 0, 0, 0) |
/// | [BlendFactor::One] | (1, 1, 1, 1) |
/// | [BlendFactor::SourceColour] | (R<sub>s0</sub>, G<sub>s0</sub>, B<sub>s0</sub>, A<sub>s0</sub>) / (k<sub>R</sub>, k<sub>G</sub>, k<sub>B</sub>, k<sub>A</sub>) |
/// | [BlendFactor::OneMinusSourceColour] | (1, 1, 1, 1) - (R<sub>s0</sub>, G<sub>s0</sub>, B<sub>s0</sub>, A<sub>s0</sub>) / (k<sub>R</sub>, k<sub>G</sub>, k<sub>B</sub>, k<sub>A</sub>) |
/// | [BlendFactor::DestColour] | (R<sub>d</sub>, G<sub>d</sub>, B<sub>d</sub>, A<sub>d</sub>) / (k<sub>R</sub>, k<sub>G</sub>, k<sub>B</sub>, k<sub>A</sub>) |
/// | [BlendFactor::OneMinusDestColour] | (1, 1, 1, 1) - (R<sub>d</sub>, G<sub>d</sub>, B<sub>d</sub>, A<sub>d</sub>) / (k<sub>R</sub>, k<sub>G</sub>, k<sub>B</sub>, k<sub>A</sub>) |
/// | [BlendFactor::SourceAlpha] | (A<sub>s0</sub>, A<sub>s0</sub>, A<sub>s0</sub>, A<sub>s0</sub>) / (k<sub>A</sub>, k<sub>A</sub>, k<sub>A</sub>, k<sub>A</sub>) |
/// | [BlendFactor::OneMinusSource1Alpha] | (1, 1, 1, 1) - (A<sub>s0</sub>, A<sub>s0</sub>, A<sub>s0</sub>, A<sub>s0</sub>) / (k<sub>A</sub>, k<sub>A</sub>, k<sub>A</sub>, k<sub>A</sub>) |
/// | [BlendFactor::DestAlpha] | (A<sub>d</sub>, A<sub>d</sub>, A<sub>d</sub>, A<sub>d</sub>) / (k<sub>A</sub>, k<sub>A</sub>, k<sub>A</sub>, k<sub>A</sub>) |
/// | [BlendFactor::OneMinusDestAlpha] | (1, 1, 1, 1) - (A<sub>d</sub>, A<sub>d</sub>, A<sub>d</sub>, A<sub>d</sub>) / (k<sub>A</sub>, k<sub>A</sub>, k<sub>A</sub>, k<sub>A</sub>) |
/// | [BlendFactor::ConstantColour] | (R<sub>c</sub>, G<sub>c</sub>, B<sub>c</sub>, A<sub>c</sub>) |
/// | [BlendFactor::OneMinusConstantColour] | (1, 1, 1, 1) - (R<sub>c</sub>, G<sub>c</sub>, B<sub>c</sub>, A<sub>c</sub>) |
/// | [BlendFactor::ConstantAlpha] | (A<sub>c</sub>, A<sub>c</sub>, A<sub>c</sub>, A<sub>c</sub>) |
/// | [BlendFactor::OneMinusConstantAlpha] | (1, 1, 1, 1) - (A<sub>c</sub>, A<sub>c</sub>, A<sub>c</sub>, A<sub>c</sub>) |
/// | [BlendFactor::SourceAlphaSaturate] | (`i`, `i`, `i`, 1) |
/// | [BlendFactor::Source1Colour] | (R<sub>s1</sub>, G<sub>s1</sub>, B<sub>s1</sub>, A<sub>s1</sub>) / (k<sub>R</sub>, k<sub>G</sub>, k<sub>B</sub>, k<sub>A</sub>) |
/// | [BlendFactor::OneMinusSource1Colour] | (1, 1, 1, 1) - (R<sub>s1</sub>, G<sub>s1</sub>, B<sub>s1</sub>, A<sub>s1</sub>) / (k<sub>R</sub>, k<sub>G</sub>, k<sub>B</sub>, k<sub>A</sub>) |
/// | [BlendFactor::Source1Alpha] | (A<sub>s1</sub>, A<sub>s1</sub>, A<sub>s1</sub>, A<sub>s1</sub>) / (k<sub>A</sub>, k<sub>A</sub>, k<sub>A</sub>, k<sub>A</sub>) |
/// | [BlendFactor::OneMinusSource1Alpha] | (1, 1, 1, 1) - (A<sub>s1</sub>, A<sub>s1</sub>, A<sub>s1</sub>, A<sub>s1</sub>) / (k<sub>A</sub>, k<sub>A</sub>, k<sub>A</sub>, k<sub>A</sub>) |
///
/// In the table,
///
/// `i = min(A`<sub>`s`</sub>`, k`<sub>`A`</sub>` - A`<sub>`d`</sub>`) / k`<sub>`A`</sub>
///
/// To determine the blended RGBA values of a pixel, the system uses the following equations:
/// * `R`<sub>`d`</sub>` = min(k`<sub>`R`</sub>`, R`<sub>`s`</sub>`*s`<sub>R</sub>` + R`<sub>`d`</sub>`*d`<sub>`R`</sub>`)`
/// * `G`<sub>`d`</sub>` = min(k`<sub>`G`</sub>`, G`<sub>`s`</sub>`*s`<sub>G</sub>` + G`<sub>`d`</sub>`*d`<sub>`G`</sub>`)`
/// * `B`<sub>`d`</sub>` = min(k`<sub>`B`</sub>`, B`<sub>`s`</sub>`*s`<sub>B</sub>` + B`<sub>`d`</sub>`*d`<sub>`B`</sub>`)`
/// * `A`<sub>`d`</sub>` = min(k`<sub>`A`</sub>`, A`<sub>`s`</sub>`*s`<sub>A</sub>` + A`<sub>`d`</sub>`*d`<sub>`A`</sub>`)`
///
/// Despite the apparent precision of the above equations, blending arithmetic is not exactly
/// specified, because blending operates with imprecise integer color values. However, a blend
/// factor that should be equal to `1` is guaranteed not to modify its multiplicand, and a blend
/// factor equal to `0` reduces its multiplicand to `0`. For example, when `source_factor` is
/// [BlendFactor::SourceAlpha], `dest_factor` is [BlendFactor::OneMinusSourceAlpha], and A<sub>s</sub>
/// is equal to k<sub>A</sub<, the equations reduce to simple replacement:
/// * `R`<sub>`d`</sub>` = `R`<sub>`s`</sub>`
/// * `G`<sub>`d`</sub>` = `G`<sub>`s`</sub>`
/// * `B`<sub>`d`</sub>` = `B`<sub>`s`</sub>`
/// * `A`<sub>`d`</sub>` = `A`<sub>`s`</sub>`
///
/// Incoming (source) alpha would typically be used as a material opacity, ranging from 1.0
/// (K<sub>A</sub>), representing complete opacity, to 0.0 (0), representing complete transparency.
///
/// When more than one color buffer is enabled for drawing, the GL performs blending separately for
/// each enabled buffer, using the contents of that buffer for destination color. (See
/// [draw_buffer])
///
/// When dual source blending is enabled (i.e., one of the blend factors requiring the second color
/// input is used), the maximum number of enabled draw buffers is given by
/// [get_max_dual_source_draw_buffers], which may be lower than [get_max_draw_buffers].
///
/// # Associated Gets
/// * [get_blend_source_rgb]
/// * [get_blend_source_alpha]
/// * [get_blend_dest_rgb]
/// * [get_blend_dest_alpha]
/// * [is_enabled]([Capability::Blend])
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [blend_func] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [blend_func_buffer] | N | N | N | N | N | N | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [blend_colour]
/// * [blend_equation]
/// * [blend_func_separate]
/// * [clear]
/// * [draw_buffer]
/// * [enable]
/// * [logic_op]
/// * [stencil_func]
pub fn blend_func(source_factor: BlendFactor, dest_factor: BlendFactor) {
    let sfactor = GLenum::from(source_factor);
    let dfactor = GLenum::from(dest_factor);
    unsafe { gl::BlendFunc(sfactor, dfactor) }
}

/// # Specify pixel arithmetic
/// see [blend_func]
///
/// # Arguments
/// * `blend_index` - specifies the index of the draw buffer for which to set the blend function.
pub fn blend_func_buffer(buffer_index: u32, source_factor: BlendFactor, dest_factor: BlendFactor) {
    let sfactor = GLenum::from(source_factor);
    let dfactor = GLenum::from(dest_factor);
    unsafe { gl::BlendFunci(buffer_index, sfactor, dfactor) }
}

/// # Set front and back function and reference value for stencil testing
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glStencilFunc.xhtml>
///
/// # Arguments
/// * `func` - Specifies the test function.
/// * `reference` - Specifies the reference value for the stencil test. `reference` is clamped to
/// the range \[0, 2<sup>n</sup> − 1\], where `n` is the number of bitplanes in the stencil buffer. The
/// initial value is `0`.
/// * `mask` - Specifies a mask that is ANDed with both the `reference` value and the stored stencil
/// value when the test is done. The initial value is all 1's.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// stencil_func(CompareFunc::Always, 1, 0xFF);
/// ```
///
/// # Description
/// Stenciling, like depth-buffering, enables and disables drawing on a per-pixel basis. Stencil
/// planes are first drawn into using GL drawing primitives, then geometry and images are rendered
/// using the stencil planes to mask out portions of the screen. Stenciling is typically used in
/// multipass rendering algorithms to achieve special effects, such as decals, outlining, and
/// constructive solid geometry rendering.
///
/// The stencil test conditionally eliminates a pixel based on the outcome of a comparison between
/// the reference value and the value in the stencil buffer. To enable and disable the test, call
/// [enable] and [disable] with argument [Capability::StencilTest]. To specify actions based on the
/// outcome of the stencil test, call [stencil_op] or [stencil_op_separate].
///
/// There can be two separate sets of `func`, `reference`, and `mask` parameters; one affects
/// back-facing polygons, and the other affects front-facing polygons as well as other non-polygon
/// primitives. [stencil_func] sets both front and back stencil state to the same values. Use
/// [stencil_func_separate] to set front and back stencil state to different values.
///
/// `func` is a symbolic constant that determines the stencil comparison function. It accepts one of
/// eight values, shown in the following list. `reference` is an integer reference value that is
/// used in the stencil comparison. It is clamped to the range \[0, 2<sup>n</sup> − 1\], where `n`
/// is the number of bitplanes in the stencil buffer. `mask` is bitwise ANDed with both the
/// `reference` value and the stored stencil value, with the ANDed values participating in the
/// comparison.
///
/// If `stencil` represents the value stored in the corresponding stencil buffer location, the
/// following list shows the effect of each comparison function that can be specified by `func`.
/// Only if the comparison succeeds is the pixel passed through to the next stage in the
/// rasterization process (see glStencilOp). All tests treat stencil values as unsigned integers in
/// the range \[0, 2<sup>n</sup> − 1\], where `n` is the number of bitplanes in the stencil buffer.
///
/// | [CompareFunc] | Usage |
/// |---------------|-------|
/// | Never | Always fails |
/// | Always | Always passes |
/// | Equal | Passes if ( `ref` & `mask` ) = ( `stencil` & `mask` ) |
/// | NotEqual | Passes if ( `ref` & `mask` ) != ( `stencil` & `mask` ) |
/// | Less | Passes if ( `ref` & `mask` ) < ( `stencil` & `mask` ) |
/// | LessOrEqual | Passes if ( `ref` & `mask` ) <= ( `stencil` & `mask` ) |
/// | Greater | Passes if ( `ref` & `mask` ) > ( `stencil` & `mask` ) |
/// | GreaterOrEqual | Passes if ( `ref` & `mask` ) >= ( `stencil` & `mask` ) |
///
/// Initially, the stencil test is disabled. If there is no stencil buffer, no stencil modification
/// can occur and it is as if the stencil test always passes.
///
/// [stencil_func] is the same as calling [stencil_func_separate] with `face` set to
/// [StencilFace::FrontAndBack].
/// ```no_run
/// # use rgl::prelude::*;
/// fn equivalent_stencil_func(func: CompareFunc, reference: i32, mask: u32) {
///     stencil_func_separate(StencilFace::FrontAndBack, func, reference, mask)
/// }
/// ```
///
/// # Associated Gets
/// * [get_stencil_func]
/// * [get_stencil_value_mask]
/// * [get_stencil_ref]
/// * [get_stencil_back_func]
/// * [get_stencil_back_value_mask]
/// * [get_stencil_back_ref]
/// * [get_stencil_bits]
/// * [is_enabled]([Capability::StencilTest])
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [stencil_func] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [blend_func]
/// * [depth_func]
/// * [enable]
/// * [logic_op]
/// * [stencil_func_separate]
/// * [stencil_mask]
/// * [stencil_mask_separate]
/// * [stencil_op]
/// * [stencil_op_separate]
pub fn stencil_func(func: CompareFunc, reference: i32, mask: u32) {
    let func = GLenum::from(func);
    unsafe { gl::StencilFunc(func, reference, mask) }
}

/// # Set front and/or back function and reference value for stencil testing
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glStencilFuncSeparate.xhtml>
///
/// # Arguments
/// * `face` - Specifies whether front and/or back stencil state is updated.
/// * `func` - Specifies the test function.
/// * `reference` - Specifies the reference value for the stencil test. `reference` is clamped to
/// the range \[0, 2<sup>n</sup> − 1\], where `n` is the number of bitplanes in the stencil buffer. The
/// initial value is `0`.
/// * `mask` - Specifies a mask that is ANDed with both the `reference` value and the stored stencil
/// value when the test is done. The initial value is all 1's.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// stencil_func_separate(StencilFace::Back, CompareFunc::Greater, 1, 0x32);
/// ```
///
/// # Description
/// Stenciling, like depth-buffering, enables and disables drawing on a per-pixel basis. You draw
/// into the stencil planes using GL drawing primitives, then render geometry and images, using the
/// stencil planes to mask out portions of the screen. Stenciling is typically used in multipass
/// rendering algorithms to achieve special effects, such as decals, outlining, and constructive
/// solid geometry rendering.
///
/// The stencil test conditionally eliminates a pixel based on the outcome of a comparison between
/// the reference value and the value in the stencil buffer. To enable and disable the test, call
/// [enable] and [disable] with argument [Capability::StencilTest]. To specify actions based on the
/// outcome of the stencil test, call [stencil_op] or [stencil_op_separate].
///
/// There can be two separate sets of `func`, `reference`, and `mask` parameters; one affects
/// back-facing polygons, and the other affects front-facing polygons as well as other non-polygon
/// primitives. [stencil_func] sets both front and back stencil state to the same values, as if
/// [stencil_func_separate] were called with face set to [StencilFace::FrontAndBack].
///
/// `func` is a symbolic constant that determines the stencil comparison function. It accepts one of
/// eight values, shown in the following list. `reference` is an integer reference value that is
/// used in the stencil comparison. It is clamped to the range \[0, 2<sup>n</sup> − 1\], where `n`
/// is the number of bitplanes in the stencil buffer. `mask` is bitwise ANDed with both the
/// `reference` value and the stored stencil value, with the ANDed values participating in the
/// comparison.
///
/// If `stencil` represents the value stored in the corresponding stencil buffer location, the
/// following list shows the effect of each comparison function that can be specified by `func`.
/// Only if the comparison succeeds is the pixel passed through to the next stage in the
/// rasterization process (see [stencil_op]). All tests treat stencil values as unsigned integers in
/// the range \[0, 2<sup>n</sup> − 1\], where `n` is the number of bitplanes in the stencil buffer.
///
/// | [CompareFunc] | Usage |
/// |---------------|-------|
/// | Never | Always fails |
/// | Always | Always passes |
/// | Equal | Passes if ( `ref` & `mask` ) = ( `stencil` & `mask` ) |
/// | NotEqual | Passes if ( `ref` & `mask` ) != ( `stencil` & `mask` ) |
/// | Less | Passes if ( `ref` & `mask` ) < ( `stencil` & `mask` ) |
/// | LessOrEqual | Passes if ( `ref` & `mask` ) <= ( `stencil` & `mask` ) |
/// | Greater | Passes if ( `ref` & `mask` ) > ( `stencil` & `mask` ) |
/// | GreaterOrEqual | Passes if ( `ref` & `mask` ) >= ( `stencil` & `mask` ) |
///
/// Initially, the stencil test is disabled. If there is no stencil buffer, no stencil modification
/// can occur and it is as if the stencil test always passes.
///
/// # Associated Gets
/// * [get_stencil_func]
/// * [get_stencil_value_mask]
/// * [get_stencil_ref]
/// * [get_stencil_back_func]
/// * [get_stencil_back_value_mask]
/// * [get_stencil_back_ref]
/// * [get_stencil_bits]
/// * [is_enabled]([Capability::StencilTest])
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [stencil_func_separate] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [blend_func]
/// * [depth_func]
/// * [enable]
/// * [logic_op]
/// * [stencil_func]
/// * [stencil_mask]
/// * [stencil_mask_separate]
/// * [stencil_op]
/// * [stencil_op_separate]
pub fn stencil_func_separate(face: StencilFace, func: CompareFunc, reference: i32, mask: u32) {
    let face = GLenum::from(face);
    let func = GLenum::from(func);
    unsafe { gl::StencilFuncSeparate(face, func, reference, mask) }
}

/// # Set front and back stencil test actions
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glStencilOp.xhtml>
///
/// # Arguments
/// * `stencil_fail_op` - Specifies the action to take when the stencil test fails. The initial
/// value is [StencilOp::Keep].
/// * `depth_fail_op` - Specifies the stencil action when the stencil test passes, but the depth
/// test fails. The initial value is [StencilOp::Keep].
/// * `depth_pass_op` - Specifies the stencil action when both the stencil test and the depth test
/// pass, or when the stencil test passes and either there is no depth buffer or depth testing is
/// not enabled. The initial value is [StencilOp::Keep].
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// stencil_op(StencilOp::Keep, StencilOp::Keep, StencilOp::Replace);
/// ```
///
/// # Description
/// Stenciling, like depth-buffering, enables and disables drawing on a per-pixel basis. You draw
/// into the stencil planes using GL drawing primitives, then render geometry and images, using the
/// stencil planes to mask out portions of the screen. Stenciling is typically used in multipass
/// rendering algorithms to achieve special effects, such as decals, outlining, and constructive
/// solid geometry rendering.
///
/// The stencil test conditionally eliminates a pixel based on the outcome of a comparison between
/// the value in the stencil buffer and a reference value. To enable and disable the test, call
/// [enable] and [disable] with argument [Capability::StencilTest]; to control it, call
/// [stencil_func] or [stencil_func_separate].
///
/// There can be two separate sets of `stencil_fail_op`, `depth_fail_op`, and `depth_pass_op`
/// parameters; one affects back-facing polygons, and the other affects front-facing polygons as
/// well as other non-polygon primitives. [stencil_op] sets both front and back stencil state to the
/// same values. Use [stencil_op_separate] to set front and back stencil state to different values.
///
/// [stencil_op] takes three arguments that indicate what happens to the stored stencil value while
/// stenciling is enabled. If the stencil test fails, no change is made to the pixel's color or
/// depth buffers, and `stencil_fail_op` specifies what happens to the stencil buffer contents.
///
/// Stencil buffer values are treated as unsigned integers. When incremented and decremented, values
/// are clamped to `0` and `2`<sup>`n`</sup>` − 1`, where `n` is the value returned by querying
/// [get_stencil_bits].
///
/// The other two arguments to [stencil_op] specify stencil buffer actions that depend on whether
/// subsequent depth buffer tests succeed (`depth_pass_op`) or fail (`depth_fail_op`) (see
/// [depth_func]). Note that `depth_fail_op` is ignored when there is no depth buffer, or when the
/// depth buffer is not enabled. In these cases, `stencil_fail_op` and `depth_pass_op` specify
/// stencil action when the stencil test fails and passes, respectively.
///
/// Initially the stencil test is disabled. If there is no stencil buffer, no stencil modification
/// can occur and it is as if the stencil tests always pass, regardless of any call to [stencil_op].
///
/// [stencil_op] is the same as calling [stencil_op_separate] with `face` set to
/// [StencilFace::FrontAndBack], like so:
/// ```no_run
/// # use rgl::prelude::*;
/// fn equivalent_stencil_op(stencil_fail_op: StencilOp, depth_fail_op: StencilOp, depth_pass_op: StencilOp) {
///     stencil_op_separate(StencilFace::FrontAndBack, StencilOp::Keep, StencilOp::Keep, StencilOp::Replace);
/// }
/// ```
///
/// # Associated Gets
/// * [get_stencil_fail]
/// * [get_stencil_pass_depth_pass]
/// * [get_stencil_pass_depth_fail]
/// * [get_stencil_back_fail]
/// * [get_stencil_back_pass_depth_pass]
/// * [get_stencil_back_pass_depth_fail]
/// * [get_stencil_bits]
/// * [is_enabled]([Capability::StencilTest])
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [stencil_op] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [blend_func]
/// * [depth_func]
/// * [enable]
/// * [logic_op]
/// * [stencil_func]
/// * [stencil_func_separate]
/// * [stencil_mask]
/// * [stencil_mask_separate]
/// * [stencil_op_separate]
pub fn stencil_op(stencil_fail_op: StencilOp, depth_fail_op: StencilOp, depth_pass_op: StencilOp) {
    let sfail = GLenum::from(stencil_fail_op);
    let zfail = GLenum::from(depth_fail_op);
    let zpass = GLenum::from(depth_pass_op);
    unsafe { gl::StencilOp(sfail, zfail, zpass) }
}

/// # Set front and/or back stencil test actions
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glStencilOpSeparate.xhtml>
///
/// # Arguments
/// * `face` - Specifies whether front and/or back stencil state is updated.
/// * `stencil_fail_op` - Specifies the action to take when the stencil test fails. The initial
/// value is [StencilOp::Keep].
/// * `depth_fail_op` - Specifies the stencil action when the stencil test passes, but the depth
/// test fails. The initial value is [StencilOp::Keep].
/// * `depth_pass_op` - Specifies the stencil action when both the stencil test and the depth test
/// pass, or when the stencil test passes and either there is no depth buffer or depth testing is
/// not enabled. The initial value is [StencilOp::Keep].
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// stencil_op_separate(StencilFace::Front, StencilOp::Zero, StencilOp::Zero, StencilOp::Zero);
/// stencil_op_separate(StencilFace::Back, StencilOp::Keep, StencilOp::Keep, StencilOp::Replace);
/// ```
///
/// # Description
/// Stenciling, like depth-buffering, enables and disables drawing on a per-pixel basis. You draw
/// into the stencil planes using GL drawing primitives, then render geometry and images, using the
/// stencil planes to mask out portions of the screen. Stenciling is typically used in multipass
/// rendering algorithms to achieve special effects, such as decals, outlining, and constructive
/// solid geometry rendering.
///
/// The stencil test conditionally eliminates a pixel based on the outcome of a comparison between
/// the value in the stencil buffer and a reference value. To enable and disable the test, call
/// [enable] and [disable] with argument [Capability::StencilTest]; to control it, call
/// [stencil_func] or [stencil_func_separate].
///
/// There can be two separate sets of `stencil_fail_op`, `depth_fail_op`, and `depth_pass_op`
/// parameters; one affects back-facing polygons, and the other affects front-facing polygons as
/// well as other non-polygon primitives. [stencil_op] sets both front and back stencil state to the
/// same values. Use [stencil_op_separate] to set front and back stencil state to different values.
///
/// [stencil_op] takes three arguments that indicate what happens to the stored stencil value while
/// stenciling is enabled. If the stencil test fails, no change is made to the pixel's color or
/// depth buffers, and `stencil_fail_op` specifies what happens to the stencil buffer contents.
///
/// Stencil buffer values are treated as unsigned integers. When incremented and decremented, values
/// are clamped to `0` and `2`<sup>`n`</sup>` − 1`, where `n` is the value returned by querying
/// [get_stencil_bits].
///
/// The other two arguments to [stencil_op_separate] specify stencil buffer actions that depend on
/// whether subsequent depth buffer tests succeed (`depth_pass_op`) or fail (`depth_fail_op`) (see
/// [depth_func]). Note that `depth_fail_op` is ignored when there is no depth buffer, or when the
/// depth buffer is not enabled. In these cases, `stencil_fail_op` and `depth_pass_op` specify
/// stencil action when the stencil test fails and passes, respectively.
///
/// Initially the stencil test is disabled. If there is no stencil buffer, no stencil modification
/// can occur and it is as if the stencil tests always pass, regardless of any call to [stencil_op].
///
/// # Associated Gets
/// * [get_stencil_fail]
/// * [get_stencil_pass_depth_pass]
/// * [get_stencil_pass_depth_fail]
/// * [get_stencil_back_fail]
/// * [get_stencil_back_pass_depth_pass]
/// * [get_stencil_back_pass_depth_fail]
/// * [get_stencil_bits]
/// * [is_enabled]([Capability::StencilTest])
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [stencil_op] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [blend_func]
/// * [depth_func]
/// * [enable]
/// * [logic_op]
/// * [stencil_func]
/// * [stencil_func_separate]
/// * [stencil_mask]
/// * [stencil_mask_separate]
/// * [stencil_op]
pub fn stencil_op_separate(
    face: StencilFace,
    stencil_fail_op: StencilOp,
    depth_fail_op: StencilOp,
    depth_pass_op: StencilOp,
) {
    let face = GLenum::from(face);
    let sfail = GLenum::from(stencil_fail_op);
    let dpfail = GLenum::from(depth_fail_op);
    let dppass = GLenum::from(depth_pass_op);
    unsafe { gl::StencilOpSeparate(face, sfail, dpfail, dppass) }
}

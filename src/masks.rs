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

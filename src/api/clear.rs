use crate::*;

/// clear buffers to preset values
///
/// # Description
/// [clear] sets the bitplane area of the window to values previously selected by
/// [Colour](ClearMask::COLOUR), [Depth](ClearMask::DEPTH), and [Stencil](ClearMask::STENCIL).
/// Multiple colour buffers can be cleared simultaneously by selecting more than one buffer at a time
/// using [draw_buffer].
///
/// The pixel ownership test, the scissor test, dithering, and the buffer writemasks affect the
/// operation of [clear]. The scissor box bounds the cleared region. Alpha function, blend function,
/// logical operation, stenciling, texture mapping, and depth-buffering are ignored by [clear].
///
/// [clear] takes a single argument that is the bitwise OR of several values indicating which
/// buffer is to be cleared.
///
/// The value to which each buffer is cleared depends on the setting of the clear value for that buffer.
pub fn clear(mask: ClearMask) -> () {
    unsafe { gl::Clear(mask.bits()) }
}

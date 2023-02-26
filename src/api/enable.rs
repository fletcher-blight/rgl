use crate::*;
use gl::types::*;

// TODO: gl::Enablei (Blend and ScissorTest)

/// enable server-side GL capabilities
pub fn enable(cap: Capability) -> () {
    let cap = GLenum::from(cap);
    unsafe { gl::Enable(cap) }
}

/// disable server-side GL capabilities
pub fn disable(cap: Capability) -> () {
    let cap = GLenum::from(cap);
    unsafe { gl::Disable(cap) }
}

/// enable server-side clip distance
///
/// # Description
/// If enabled, clip geometry against user-defined half space `i`
pub fn enable_clip_distance(i: u32) -> Result<(), Error> {
    let cap: GLenum = gl::CLIP_DISTANCE0 + i;
    unsafe { gl::Enable(cap) };
    handle_clip_distance_error(i)
}

/// disable server-side clip distance
pub fn disable_clip_distance(i: u32) -> Result<(), Error> {
    let cap: GLenum = gl::CLIP_DISTANCE0 + i;
    unsafe { gl::Enable(cap) };
    handle_clip_distance_error(i)
}

fn handle_clip_distance_error(i: u32) -> Result<(), Error> {
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(Error::OutOfBoundsClipDistance(i)),
        error => Err(Error::Unreachable(error)),
    }
}

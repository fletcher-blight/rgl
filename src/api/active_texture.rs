use crate::*;

/// select active texture unit
///
/// # Description
/// Selects which texture unit subsequent texture state calls will affect. The number of texture
/// units an implementation supports is implementation dependent, but must be at least 80.
pub fn active_texture(texture_index: u32) -> Result<(), Error> {
    let texture = gl::TEXTURE0 + texture_index;
    unsafe { gl::ActiveTexture(texture) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidEnum => Err(Error::OutOfBoundsTextureIndex(texture_index)),
        error => Err(Error::Unreachable(error)),
    }
}

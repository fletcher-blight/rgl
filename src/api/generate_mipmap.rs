use crate::*;
use gl::types::*;

/// generate mipmaps for a specified texture object
///
/// # Description
/// TODO
pub fn generate_mipmap(target: TextureBindingTarget) -> Result<(), Error> {
    let target = GLenum::from(target);
    unsafe { gl::GenerateMipmap(target) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        error => Err(Error::Unreachable(error)),
    }
}

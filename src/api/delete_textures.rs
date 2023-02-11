use crate::*;
use gl::types::*;

/// delete named textures
///
/// # Description
/// Deletes all textures named by the elements of the array `textures`. After a texture is deleted,
/// it has no contents or dimensionality, and its name is free for reuse (for example by [gen_textures]).
/// If a texture that is currently bound is deleted, the binding reverts to 0 (the default texture).
///
/// [delete_textures] silently ignores 0's and names that do not correspond to existing textures.
pub fn delete_textures(textures: &[Texture]) -> () {
    let n = textures.len() as GLsizei;
    let textures = textures.as_ptr() as *const GLuint;
    unsafe { gl::DeleteTextures(n, textures) }
}

use crate::*;
use gl::types::*;

/// delete named textures
///
/// # Description
/// Deletes all textures named by the elements of the array `textures`. After a texture is deleted,
/// it has no contents or dimensionality, and its name is free for reuse (for example by [gen_texture]).
/// If a texture that is currently bound is deleted, the binding reverts to 0 (the default texture).
///
/// [delete_textures] silently ignores 0's and names that do not correspond to existing textures.
///
/// # Examples
/// ```no_run
/// # fn cleanup(texture: rgl::Texture, textures: &[rgl::Texture]) {
/// rgl::delete_textures(&[texture]);
/// rgl::delete_textures(textures);
/// # }
/// ```
pub fn delete_textures(textures: &[Texture]) -> () {
    let n = textures.len() as GLsizei;
    let textures = textures.as_ptr() as *const GLuint;
    unsafe { gl::DeleteTextures(n, textures) }
}

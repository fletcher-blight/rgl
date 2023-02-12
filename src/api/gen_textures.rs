use crate::*;
use gl::types::*;

/// generate texture names
///
/// # Description
/// Fills texture names into `textures`. There is no guarantee that the names form a contiguous set
/// of integers; however, it is guaranteed that none of the returned names was in use immediately
/// before the call to [gen_textures].
///
/// The generated textures have no dimensionality; they assume the dimensionality of the texture
/// target to which they are first bound (see [bind_texture]).
///
/// Texture names returned by a call to [gen_textures] are not returned by subsequent calls, unless
/// they are first deleted with [delete_textures].
///
/// # Examples
/// Creating a single [Texture]
/// ```no_run
/// # use rgl::*;
/// let mut texture = Default::default();
/// gen_textures(std::slice::from_mut(&mut texture));
/// ```
pub fn gen_textures(textures: &mut [Texture]) -> () {
    let n = textures.len() as GLsizei;
    let textures = textures.as_mut_ptr() as *mut GLuint;
    unsafe { gl::GenTextures(n, textures) }
}

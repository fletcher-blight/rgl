use crate::*;
use gl::types::*;

/// generate a texture name
///
/// # Description
/// [gen_texture] generates a texture name. There is no guarantee that the names form a contiguous
/// set of integers; however, it is guaranteed that the returned name was not in use immediately
/// before the call to [gen_texture].
///
/// The generated texture has no dimensionality; they assume the dimensionality of the texture
/// target to which they are first bound (see [bind_texture]).
///
/// The texture name returned by a call to [gen_texture] is not returned by subsequent calls, unless
/// they are first deleted with [delete_textures].
///
/// # Examples
/// ```no_run
/// let texture: rgl::Texture = rgl::gen_texture();
/// ```
///
/// For safety, texture must be generated from [gen_texture]
/// ```compile_fail
/// let texture = rgl::Texture(42);
/// ```
pub fn gen_texture() -> Texture {
    let mut texture = Texture(0);
    let textures = &mut texture.0 as *mut GLuint;
    unsafe { gl::GenTextures(1, textures) };
    texture
}

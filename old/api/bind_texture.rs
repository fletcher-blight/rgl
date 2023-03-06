use crate::*;
use gl::types::*;

/// bind a named texture to a texturing target
///
/// # Description
/// Lets you create or use a named texture. Binds the `texture` name to the `target`. When a texture
/// is bound to a target, the previous binding for that target is automatically broken.
///
/// Texture names are optional. The value `None` is reserved to represent the default texture for
/// each texture target. Texture names and the corresponding texture contents are local to the shared
/// object space of the current GL rendering context; two rendering contexts share texture names only
/// if they explicitly enable sharing between contexts through the appropriate GL windows interfaces
/// functions.
///
/// You must use [gen_texture] to generate a set of new texture names.
///
/// While a texture is bound, GL operations on the target to which it is bound affect the bound texture,
/// and queries of the target to which it is bound return state from the bound texture. In effect,
/// the texture targets become aliases for the textures currently bound to them, and the texture name
/// `None` refers to the default textures that were bound to them at initialization.
///
/// A texture binding created with [bind_texture] remains active until a different texture is bound
/// to the same target, or until the bound texture is deleted with [delete_textures].
///
/// Once created, a named texture may be re-bound to its same original target as often as needed.
/// It is usually much faster to use [bind_texture] to bind an existing named texture to one of the
/// texture targets than it is to reload the texture image using [texture_image_1d],
/// [texture_image_2d], [texture_image_3d] or another similar function.
///
/// # Compatability
/// 3.2 or greater is required for: [2D Multisample](TextureBindingTarget::Image2DMultisample) and
/// [2D Multisample Array](TextureBindingTarget::Image2DMultisampleArray)
///
/// # Examples
/// ```no_run
/// # fn setup_texture(texture: rgl::Texture) -> Result<(), rgl::Error> {
/// rgl::bind_texture(rgl::TextureBindingTarget::Image2D, Some(texture))?;
/// // ... setup logic ...
/// rgl::bind_texture(rgl::TextureBindingTarget::Image2D, None)?;
/// # Ok(())
/// # }
/// ```
pub fn bind_texture(target: TextureBindingTarget, texture: Option<Texture>) -> Result<(), Error> {
    let target_val: GLenum = target.into();
    let texture = texture.unwrap_or(Texture(0));
    unsafe { gl::BindTexture(target_val, texture.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(Error::NotATexture(texture)),
        ErrorOpenGL::InvalidOperation => Err(Error::TextureAttemptedTargetChange(texture, target)),
        error => Err(Error::Unreachable(error)),
    }
}

//! # Texturing
//! <https://www.khronos.org/opengl/wiki/Category:Core_API_Ref_Texturing>
//!
//! # Description
//! The core OpenGL API functions for creating, managing, binding, and deleting
//! [textures](https://www.khronos.org/opengl/wiki/Texture) and
//! [sampler](https://www.khronos.org/opengl/wiki/Sampler_Object) objects.

use crate::prelude::*;
use gl::types::*;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Texture(pub u32);

#[derive(Debug, Copy, Clone)]
pub enum TextureBindingTarget {
    Image1D,
    Image2D,
    Image3D,
    Array1D,
    Array2D,
    Rectangle,
    CubeMap,
    CubeMapArray,
    Buffer,
    Multisample2D,
    Multisample2DArray,
}

impl From<TextureBindingTarget> for u32 {
    fn from(value: TextureBindingTarget) -> Self {
        match value {
            TextureBindingTarget::Image1D => gl::TEXTURE_1D,
            TextureBindingTarget::Image2D => gl::TEXTURE_2D,
            TextureBindingTarget::Image3D => gl::TEXTURE_3D,
            TextureBindingTarget::Array1D => gl::TEXTURE_1D_ARRAY,
            TextureBindingTarget::Array2D => gl::TEXTURE_2D_ARRAY,
            TextureBindingTarget::Rectangle => gl::TEXTURE_RECTANGLE,
            TextureBindingTarget::CubeMap => gl::TEXTURE_CUBE_MAP,
            TextureBindingTarget::CubeMapArray => gl::TEXTURE_CUBE_MAP_ARRAY,
            TextureBindingTarget::Buffer => gl::TEXTURE_BUFFER,
            TextureBindingTarget::Multisample2D => gl::TEXTURE_2D_MULTISAMPLE,
            TextureBindingTarget::Multisample2DArray => gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
        }
    }
}

/// # Select active texture unit
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glActiveTexture.xhtml>
///
/// # Arguments
/// * `texture_index` - Specifies which texture unit to make active. The number of texture units is
/// implementation dependent, but must be at least 80. ranges from zero to the value of
/// [get_max_combined_texture_image_units] minus one.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// active_texture(0);
/// ```
///
/// # Description
/// [active_texture] selects which texture unit subsequent texture state calls will affect. The
/// number of texture units an implementation supports is implementation dependent, but must be at
/// least 80.
///
/// # Errors
/// * [Error::InvalidEnum] - if `texture_index` is outside the allowed range
///
/// # Associated Gets
/// * [get_active_texture]
/// * [get_max_combined_texture_image_units]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [active_texture] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [gen_textures]
/// * [bind_texture]
/// * [compressed_tex_image_1d], [compressed_tex_image_2d], [compressed_tex_image_3d]
/// * [compressed_tex_sub_image_1d], [compressed_tex_sub_image_2d], [compressed_tex_sub_image_3d]
/// * [copy_tex_image_1d], [copy_tex_image_2d], [copy_tex_image_3d]
/// * [copy_tex_sub_image_1d], [copy_tex_sub_image_2d], [copy_tex_sub_image_3d]
/// * [delete_textures]
/// * [is_texture]
/// * [tex_image_1d], [tex_image_2d], [tex_image_2d_multisample], [tex_image_3d],
/// [tex_image_3d_multisample]
/// * [tex_sub_image_1d], [tex_sub_image_2d], [tex_sub_image_3d],
/// * [tex_parameter]
pub fn active_texture(texture_index: u32) {
    let texture = gl::TEXTURE0 + texture_index;

    // SAFE: synchronous integer copy
    unsafe { gl::ActiveTexture(texture) }
}

/// # Bind a named texture to a texturing target
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBindTexture.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the texture is bound.
/// * `texture` - Specifies the name of a texture.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// bind_texture(TextureBindingTarget::Image2D, Texture(42));
/// ```
///
/// # Description
/// [bind_texture] lets you create or use a named texture. Calling [bind_texture] with `texture` set
/// to the name of the new texture binds the texture name to the target. When a texture is bound to
/// a target, the previous binding for that target is automatically broken.
///
/// Texture names are `u32`. The value zero is reserved to represent the default texture for each
/// texture target. Texture names and the corresponding texture contents are local to the shared
/// object space of the current GL rendering context; two rendering contexts share texture names
/// only if they explicitly enable sharing between contexts through the appropriate GL windows
/// interfaces functions.
///
/// You must use [gen_textures] to generate a set of new texture names.
///
/// While a texture is bound, GL operations on the target to which it is bound affect the bound
/// texture, and queries of the target to which it is bound return state from the bound texture. In
/// effect, the texture targets become aliases for the textures currently bound to them, and the
/// texture name zero refers to the default textures that were bound to them at initialization.
///
/// A texture binding created with [bind_texture] remains active until a different texture is bound
/// to the same target, or until the bound texture is deleted with [delete_textures].
///
/// Once created, a named texture may be re-bound to its same original target as often as needed. It
/// is usually much faster to use [bind_texture] to bind an existing named texture to one of the
/// texture targets than it is to reload the texture image using [tex_image_1d], [tex_image_2d],
/// [tex_image_3d] or another similar function.
///
/// # Compatability
/// * 3.2 - [TextureBindingTarget::Multisample2D] and [TextureBindingTarget::Multisample2DArray]
///
/// # Errors
/// * [Error::InvalidValue] - if `texture` is not a name returned from a previous call to
/// [gen_textures].
/// * [Error::InvalidOperation] - if `texture` was previously created with a target that doesn't
/// match that of `target`.
///
/// # Associated Gets
/// * [get_texture_binding_1d], [get_texture_binding_2d], [get_texture_binding_3d]
/// * [get_texture_binding_1d_array], [get_texture_binding_2d_array]
/// * [get_texture_binding_rectangle]
/// * [get_texture_binding_buffer]
/// * [get_texture_binding_cube_map]
/// * [get_texture_binding_cub_map_array]
/// * [get_texture_binding_multisample_2d], [get_texture_binding_multisample_2d_array]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [bind_texture] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [delete_textures]
/// * [gen_textures]
/// * [get_tex_parameter]
/// * [is_texture]
/// * [tex_image_1d], [tex_image_2d], [tex_image_3d]
/// * [tex_image_2d_multisample], [tex_image_3d_multisample]
/// * [tex_buffer]
/// * [tex_parameter]
pub fn bind_texture(target: TextureBindingTarget, texture: Texture) {
    let target = GLenum::from(target);
    let texture = texture.0;

    // SAFE: synchronous integer copy
    unsafe { gl::BindTexture(target, texture) }
}

/// # Delete named textures
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glDeleteTextures.xhtml>
///
/// # Arguments
/// * `textures` - Specifies a slice of textures to be deleted.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// delete_textures(&[Texture(1), Texture(2)]);
/// ```
///
/// # Description
/// [delete_textures] deletes all textures named by the elements of the slice `textures`. After a
/// texture is deleted, it has no contents or dimensionality, and its name is free for reuse (for
/// example by [gen_textures]). If a texture that is currently bound is deleted, the binding reverts
/// to 0 (the default texture).
///
/// [delete_textures] silently ignores 0's and names that do not correspond to existing textures.
///
/// # Associated Gets
/// * [is_texture]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [delete_textures] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_texture]
/// * [copy_tex_image_1d], [copy_tex_image_2d]
/// * [gen_textures]
/// * [get_tex_parameter]
/// * [tex_image_1d], [tex_image_2d]
/// * [tex_parameter]
pub fn delete_textures(textures: &[Texture]) {
    let n = textures.len() as GLsizei;
    let textures = textures.as_ptr() as *const GLuint;

    // SAFE: synchronous read of `textures`, no memory retained
    unsafe { gl::DeleteTextures(n, textures) }
}

/// # Generate mipmaps for a specified texture object
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGenerateMipmap.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the texture object is bound
/// * `texture` - Specifies the texture object name for glGenerateTextureMipmap
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// generate_mipmap(TextureBindingTarget::Image2D);
/// generate_texture_mipmap(Texture(42));
/// ```
///
/// # Description
/// [generate_mipmap] and [generate_texture_mipmap] generates mipmaps for the specified texture
/// object. For [generate_mipmap], the texture object that is bound to `target`. For
/// [generate_texture_mipmap], `texture` is the name of the texture object.
///
/// For cube map and cube map array textures, the texture object must be cube complete or cube array
/// complete respectively.
///
/// Mipmap generation replaces texel image levels `level<sub>base</sub> + 1` through `q` with images
/// derived from the `level<sub>base</sub>` image, regardless of their previous contents. All other
/// mimap images, including the `level<sub>base</sub> + 1` image, are left unchanged by this
/// computation.
///
/// The internal formats of the derived mipmap images all match those of the `level<sub>base</sub>`
/// image. The contents of the derived images are computed by repeated, filtered reduction of the
/// `level<sub>base</sub> + 1` image. For one- and two-dimensional array and cube map array
/// textures, each layer is filtered independently.
///
/// # Compatability
/// * 4.0 - Cube map array textures
///
/// # Errors
/// * [Error::InvalidOperation] - if `target` is [TextureBindingTarget::CubeMap] or
/// [TextureBindingTarget::CubeMapArray], and the specified texture object is not cube complete or
/// cube array complete, respectively.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [generate_mipmap] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [generate_texture_mipmap] | N | N | N | N | N | N | N | N | N | N | N | Y |
///
/// # See Also
/// * [tex_image_2d]
/// * [bind_texture]
/// * [gen_textures]
pub fn generate_mipmap(target: TextureBindingTarget) {
    let target = GLenum::from(target);

    // SAFE: synchronous integer copy
    unsafe { gl::GenerateMipmap(target) }
}

/// # Generate mipmaps for a specified texture object
/// see [generate_mipmap]
///
/// # Arguments
/// * `texture` - Specifies the texture object name
///
/// # Errors
/// * [Error::InvalidOperation] - if `texture` is not the name of an existing texture object.
pub fn generate_texture_mipmap(texture: Texture) {
    let texture = texture.0;

    // SAFE: synchronous integer copy
    unsafe { gl::GenerateTextureMipmap(texture) }
}

/// # Generate texture names
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGenTextures.xhtml>
///
/// # Arguments
/// * `textures` - Specifies an array in which the generated texture names are stored.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// let mut texture = Texture::default();
/// gen_textures(std::slice::from_mut(&mut texture));
/// assert_ne!(texture, Texture(0));
/// ```
///
/// # Description
/// [gen_textures] returns all texture names in `textures`. There is no guarantee that the names
/// form a contiguous set of integers; however, it is guaranteed that none of the returned names was
/// in use immediately before the call to [gen_textures].
///
/// The generated textures have no dimensionality; they assume the dimensionality of the texture
/// target to which they are first bound (see [bind_texture]).
///
/// Texture names returned by a call to [gen_textures] are not returned by subsequent calls, unless
/// they are first deleted with [delete_textures].
///
/// # Associated Gets
/// * [is_texture]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [gen_textures] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_texture]
/// * [copy_tex_image_1d], [copy_tex_image_2d]
/// * [delete_textures]
/// * [get_tex_parameter]
/// * [tex_image_1d], [tex_image_2d], [tex_image_3d]
/// * [tex_parameter]
pub fn gen_textures(textures: &mut [Texture]) {
    let n = textures.len() as GLsizei;
    let textures = textures.as_mut_ptr() as *mut u32;

    // SAFE: `textures` is an out param, and `n` dictates the pointer length given by std::slice
    unsafe { gl::GenTextures(n, textures) }
}

pub enum TextureTarget2D {
    Image2D,
    Proxy2D,
    Image1DArray,
    Proxy1DArray,
    Rectangle,
    ProxyRectangle,
    CubeMapPositiveX,
    CubeMapPositiveY,
    CubeMapPositiveZ,
    CubeMapNegativeX,
    CubeMapNegativeY,
    CubeMapNegativeZ,
    ProxyCubeMap,
}

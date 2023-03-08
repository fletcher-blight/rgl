//! # Texturing
//! <https://www.khronos.org/opengl/wiki/Category:Core_API_Ref_Texturing>
//!
//! # Description
//! The core OpenGL API functions for creating, managing, binding, and deleting
//! [textures](https://www.khronos.org/opengl/wiki/Texture) and
//! [sampler](https://www.khronos.org/opengl/wiki/Sampler_Object) objects.

use crate::prelude::*;
use gl::types::*;

#[derive(Default, Debug, Copy, Clone)]
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
pub fn bind_texture(target: TextureBindingTarget, texture: Texture) {
    let target = GLenum::from(target);
    let texture = texture.0;

    // SAFE: synchronous integer copy
    unsafe { gl::BindTexture(target, texture) }
}

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

//! # Texturing
//! <https://www.khronos.org/opengl/wiki/Category:Core_API_Ref_Texturing>
//!
//! # Description
//! The core OpenGL API functions for creating, managing, binding, and deleting
//! [textures](https://www.khronos.org/opengl/wiki/Texture) and
//! [sampler](https://www.khronos.org/opengl/wiki/Sampler_Object) objects.

use gl::types::*;

#[derive(Debug, Copy, Clone)]
pub enum TextureError {}

#[derive(Default, Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Texture(pub u32);

#[derive(Debug, Copy, Clone)]
pub enum TextureTarget {
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

/// # Generate texture names
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGenTextures.xhtml>
///
/// # Description
///
/// # Arguments
///
/// # Compatability
///
/// # Errors
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// let mut texture;
/// gen_textures(std::slice::from_mut(&mut texture));
/// ```
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

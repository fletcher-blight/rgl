//! set texture parameters
//!
//! # Description
//! Each of the following is duplicated for both a [texture name](Texture) and a
//! [texture binding target](TextureBindingTarget).
//!
//! ### Depth Stencil Mode
//! If the depth stencil mode is [Depth](TextureDepthStencilMode::Depth), then reads from
//! depth-stencil format textures will return the depth component of the texel in Rt and the
//! stencil component will be discarded. If the depth stencil mode is
//! [Stencil](TextureDepthStencilMode::Stencil) then the stencil component is returned in Rt and
//! the depth component is discarded. The initial value is [Depth](TextureDepthStencilMode::Depth).
//!
//!
//! ### Min Filter
//! The texture minifying function is used whenever the level-of-detail function used when
//! sampling from the texture determines that the texture should be minified. There are six
//! defined minifying functions. Two of them use either the nearest texture elements or a
//! weighted average of multiple texture elements to compute the texture value.
//! The other four use mipmaps.
//!
//! A mipmap is an ordered set of arrays representing the same image at progressively lower
//! resolutions. If the texture has dimensions 2n×2m, there are max(n,m)+1 mipmaps. The first
//! mipmap is the original texture, with dimensions 2n×2m.
//!
//! Each subsequent mipmap has dimensions 2k−1×2l−1, where 2k×2l are the dimensions of the
//! previous mipmap, until either k=0 or l=0. At that point, subsequent mipmaps have dimension
//! 1×2l−1 or 2k−1×1 until the final mipmap, which has dimension 1×1. To define the mipmaps,
//! call [texture_image_1d], [texture_image_2d], [texture_image_3d], [copy_texture_image_1d],
//! or [copy_texture_image_2d] with the level argument indicating the order of the mipmaps.
//! Level 0 is the original texture; level max(n,m) is the final 1×1 mipmap.
//!
//! As more texture elements are sampled in the minification process, fewer aliasing artifacts
//! will be apparent. While the [Nearest](TextureMinFilter::Nearest) and
//! [Linear](TextureMinFilter::Linear) minification functions can be faster than the other four,
//! they sample only one or multiple texture elements to determine the texture value of the pixel
//! being rendered and can produce moire patterns or ragged transitions.
//!
//! The initial value is [NearestMipmapLinear](TextureMinFilter::NearestMipmapLinear).
//!
//! ### Mag Filter
//! The texture magnification function is used whenever the level-of-detail function used when
//! sampling from the texture determines that the texture should be magnified. It sets the texture
//! magnification function to either [Nearest](TextureMinFilter::Nearest) or
//! [Linear](TextureMinFilter::Linear). [Nearest](TextureMinFilter::Nearest) is generally faster
//! than [Linear](TextureMinFilter::Linear), but it can produce textured images with sharper edges
//! because the transition between texture elements is not as smooth.
//!
//! The initial value is [Linear](TextureMinFilter::Linear).
//!
//! ### Wrap
//! Initially is set to [Repeat](TextureWrapMode::Repeat).
//! See [Wrap options](TextureWrapMode) for effects of each available option.
//!
//! # Notes
//! Suppose that a program attempts to sample from a texture and has set one of the `min_filter`
//! functions to one of the functions that requires a mipmap. If either the dimensions of the texture
//! images currently defined (with previous calls to [texture_image_1d], [texture_image_2d],
//! [texture_image_3d], [copy_texture_image_1d], or [copy_texture_image_2d]) do not follow the proper
//! sequence for mipmaps or there are fewer texture images defined than are needed, or the set of
//! texture images have differing numbers of texture components, then the texture is considered incomplete.
//!
//! Linear filtering accesses the four nearest texture elements only in 2D textures. In 1D textures,
//! linear filtering accesses the two nearest texture elements. In 3D textures, linear filtering
//! accesses the eight nearest texture elements.
//!
//! Using a [TextureBindingTarget] specifies the texture parameters for the active texture unit,
//! specified by calling [active_texture]. Whereas using a [Texture] specifies the texture parameters
//! for the texture object with ID from a given [Texture].
//!
//! # Compatability
//! 4.3 or greater is required for: all [texture_depth_stencil_mode]
//! 4.5 or greater is required for: configurating textures using a [Texture] type instead of
//! [TextureBindingTarget]
//!
//! # Examples
//! ```no_run
//! # fn main() -> Result<(), rgl::Error> {
//! // Given `texture` is bound to `rgl::TextureBindingTarget::Image2D`
//! let texture = rgl::gen_texture();
//! rgl::bind_texture(rgl::TextureBindingTarget::Image2D, Some(texture))?;
//!
//! // setting the target
//! rgl::texture_target_min_filter(
//!     rgl::TextureBindingTarget::Image2D,
//!     rgl::TextureMinFilter::Nearest)?;
//! // does the same as setting the texture itself (but only available in 4.5)
//! rgl::texture_min_filter(
//!     texture,
//!     rgl::TextureMinFilter::Nearest)?;
//!
//! # Ok(())
//! # }
//! ```

use crate::*;
use gl::types::*;

/// Specifies the mode used to read from depth-stencil format textures.
///
/// See [Depth Stencil Mode](texture_parameter#depth-stencil-mode) for more details
pub fn texture_depth_stencil_mode(
    texture: Texture,
    mode: TextureDepthStencilMode,
) -> Result<(), Error> {
    let mode = GLenum::from(mode) as GLint;
    texture_set_i32(texture, gl::DEPTH_STENCIL_TEXTURE_MODE, mode);
    handle_error()
}

/// Specifies the mode used to read from depth-stencil format textures.
///
/// See [Depth Stencil Mode](texture_parameter#depth-stencil-mode) for more details
pub fn texture_target_depth_stencil_mode(
    target: TextureBindingTarget,
    mode: TextureDepthStencilMode,
) -> Result<(), Error> {
    let mode = GLenum::from(mode) as GLint;
    texture_target_set_i32(target, gl::DEPTH_STENCIL_TEXTURE_MODE, mode);
    handle_error()
}

/// Specifies the minifying function used when sampling textures
///
/// See [Min Filter](texture_parameter#min-filter) for more details
pub fn texture_min_filter(texture: Texture, filter: TextureMinFilter) -> Result<(), Error> {
    texture_set_i32(texture, gl::TEXTURE_MIN_FILTER, filter.into());
    handle_error()
}

/// Specifies the minifying function used when sampling textures
///
/// See [Min Filter](texture_parameter#min-filter) for more details
pub fn texture_target_min_filter(
    target: TextureBindingTarget,
    filter: TextureMinFilter,
) -> Result<(), Error> {
    texture_target_set_i32(target, gl::TEXTURE_MIN_FILTER, filter.into());
    handle_error()
}

/// Specifies the magnification function used when sampling textures
///
/// See [Mag Filter](texture_parameter#mag-filter) for more details
pub fn texture_mag_filter(texture: Texture, filter: TextureMagFilter) -> Result<(), Error> {
    let filter = GLenum::from(filter) as GLint;
    texture_set_i32(texture, gl::TEXTURE_MIN_FILTER, filter);
    handle_error()
}

/// Specifies the magnification function used when sampling textures
///
/// See [Mag Filter](texture_parameter#mag-filter) for more details
pub fn texture_target_mag_filter(
    target: TextureBindingTarget,
    filter: TextureMagFilter,
) -> Result<(), Error> {
    let filter = GLenum::from(filter) as GLint;
    texture_target_set_i32(target, gl::TEXTURE_MIN_FILTER, filter);
    handle_error()
}

/// Sets the wrap parameter for texture coordinate corresponding to `wrap_target`.
///
/// See [Wrap](texture_parameter#wrap) for more details
pub fn texture_wrap(
    texture: Texture,
    wrap_target: TextureWrapTarget,
    mode: TextureWrapMode,
) -> Result<(), Error> {
    let wrap_target: GLenum = wrap_target.into();
    let value = GLenum::from(mode) as GLint;
    texture_set_i32(texture, wrap_target, value);
    handle_error()
}

/// Sets the wrap parameter for texture coordinate corresponding to `wrap_target`.
///
/// See [Wrap](texture_parameter#wrap) for more details
pub fn texture_target_wrap(
    target: TextureBindingTarget,
    wrap_target: TextureWrapTarget,
    mode: TextureWrapMode,
) -> Result<(), Error> {
    let wrap_target: GLenum = wrap_target.into();
    let value = GLenum::from(mode) as GLint;
    texture_target_set_i32(target, wrap_target, value);
    handle_error()
}

// Implementation details

fn texture_set_i32(texture: Texture, pname: GLenum, param: GLint) {
    unsafe { gl::TextureParameteri(texture.0, pname, param) }
}
//
// fn texture_set_f32(texture: Texture, pname: GLenum, param: GLfloat) {
//     unsafe { gl::TextureParameterf(texture.0, pname, param) }
// }
//
// fn texture_set_i32_values(texture: Texture, pname: GLenum, params: &[GLint]) {
//     unsafe { gl::TextureParameteriv(texture.0, pname, params.as_ptr()) }
// }
//
// fn texture_set_f32_values(texture: Texture, pname: GLenum, params: &[GLfloat]) {
//     unsafe { gl::TextureParameterfv(texture.0, pname, params.as_ptr()) }
// }
//
// fn texture_set_i32_integer_values(texture: Texture, pname: GLenum, params: &[GLint]) {
//     unsafe { gl::TextureParameterIiv(texture.0, pname, params.as_ptr()) }
// }
//
// fn texture_set_u32_integer_values(texture: Texture, pname: GLenum, params: &[GLuint]) {
//     unsafe { gl::TextureParameterIuiv(texture.0, pname, params.as_ptr()) }
// }
//
fn texture_target_set_i32(target: TextureBindingTarget, pname: GLenum, param: GLint) {
    let target: GLenum = target.into();
    unsafe { gl::TexParameteri(target, pname, param) }
}
//
// fn texture_target_set_f32(target: TextureBindingTarget, pname: GLenum, param: GLfloat) {
//     let target: GLenum = target.into();
//     unsafe { gl::TexParameterf(target, pname, param) }
// }
//
// fn texture_target_set_i32_values(target: TextureBindingTarget, pname: GLenum, params: &[GLint]) {
//     let target: GLenum = target.into();
//     unsafe { gl::TexParameteriv(target, pname, params.as_ptr()) }
// }
//
// fn texture_target_set_f32_values(target: TextureBindingTarget, pname: GLenum, params: &[GLfloat]) {
//     let target: GLenum = target.into();
//     unsafe { gl::TexParameterfv(target, pname, params.as_ptr()) }
// }
//
// fn texture_target_set_i32_integer_values(
//     target: TextureBindingTarget,
//     pname: GLenum,
//     params: &[GLint],
// ) {
//     let target: GLenum = target.into();
//     unsafe { gl::TexParameterIiv(target, pname, params.as_ptr()) }
// }
//
// fn texture_target_set_u32_integer_values(
//     target: TextureBindingTarget,
//     pname: GLenum,
//     params: &[GLuint],
// ) {
//     let target: GLenum = target.into();
//     unsafe { gl::TexParameterIuiv(target, pname, params.as_ptr()) }
// }

fn handle_error() -> Result<(), Error> {
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidEnum => {
            // multisample used for any of the sampler states
            // rectangle target and invalid wrap values
            // rectangle target and invalid min_filter values
            todo!()
        }
        ErrorOpenGL::InvalidOperation => {
            // multisample target with base level non-zero
            // not a texture
            // rectangle target and base level non-zero
            todo!()
        }
        error => Err(Error::Unreachable(error)),
    }
}

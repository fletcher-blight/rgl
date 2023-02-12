use crate::*;
use gl::types::*;

/// Setter functions for different texture parameters
///
/// # Description
/// Abstraction to use both a [TextureBindingTarget] or a [Texture] to configure texture parameters.
/// This trait is only implemented for these types, and is not intended for user usage. One can
/// implement this trait, but won't find a purpose for such a custom implementation in this library.
///
/// # Notes
/// Suppose that a program attempts to sample from a texture and has set
/// [min_filter](TextureParameter::min_filter) to one of the functions that requires a mipmap. If
/// either the dimensions of the texture images currently defined (with previous calls to
/// [texture_image_1d], [texture_image_2d], [texture_image_3d], [copy_texture_image_1d], or
/// [copy_texture_image_2d]) do not follow the proper sequence for mipmaps (described in
/// [min_filter](TextureParameter::min_filter), or there are fewer texture images defined than are
/// needed, or the set of texture images have differing numbers of texture components, then the
/// texture is considered incomplete.
///
/// Linear filtering accesses the four nearest texture elements only in 2D textures. In 1D textures,
/// linear filtering accesses the two nearest texture elements. In 3D textures, linear filtering
/// accesses the eight nearest texture elements.
///
/// Using a [TextureBindingTarget] specifies the texture parameters for the active texture unit,
/// specified by calling [active_texture]. Whereas using a [Texture] specifies the texture parameters
/// for the texture object with ID from a given [Texture].
///
/// # Compatability
/// 4.3 or greater is required for: [depth_stencil_mode](TextureParameter::depth_stencil_mode)
/// 4.5 or greater is required to configuration textures using the [Texture] type
///
/// # Examples
///
/// Using a [TextureBindingTarget] to configure parameters
/// ```no_run
/// # use rgl::*;
/// TextureBindingTarget::Image2D.wrap(TextureWrapTarget::T, TextureWrapMode::ClampToEdge).unwrap();
/// TextureBindingTarget::Image2D.min_filter(TextureMinFilter::NearestMipmapLinear).unwrap();
/// ```
///
/// Using a [Texture] to configure parameters
/// ```no_run
/// # use rgl::*;
/// # fn configure(texture: Texture) {
/// texture.wrap(TextureWrapTarget::T, TextureWrapMode::ClampToEdge).unwrap();
/// texture.min_filter(TextureMinFilter::Nearest).unwrap();
/// # }
/// ```
pub trait TextureParameter {
    /// Specifies the mode used to read from depth-stencil format textures.
    ///
    /// # Description
    /// If the depth stencil mode is [Depth](TextureDepthStencilMode::Depth), then reads from
    /// depth-stencil format textures will return the depth component of the texel in Rt and the
    /// stencil component will be discarded. If the depth stencil mode is
    /// [Stencil](TextureDepthStencilMode::Stencil) then the stencil component is returned in Rt and
    /// the depth component is discarded. The initial value is [Depth](TextureDepthStencilMode::Depth).
    fn depth_stencil_mode(self, mode: TextureDepthStencilMode) -> Result<(), Error>;

    /// Specifies the minifying function used when sampling textures
    ///
    /// # Description
    /// The texture minifying function is used whenever the level-of-detail function used when
    /// sampling from the texture determines that the texture should be minified. There are six
    /// defined minifying functions. Two of them use either the nearest texture elements or a
    /// weighted average of multiple texture elements to compute the texture value.
    /// The other four use mipmaps.
    ///
    /// A mipmap is an ordered set of arrays representing the same image at progressively lower
    /// resolutions. If the texture has dimensions 2n×2m, there are max(n,m)+1 mipmaps. The first
    /// mipmap is the original texture, with dimensions 2n×2m.
    ///
    /// Each subsequent mipmap has dimensions 2k−1×2l−1, where 2k×2l are the dimensions of the
    /// previous mipmap, until either k=0 or l=0. At that point, subsequent mipmaps have dimension
    /// 1×2l−1 or 2k−1×1 until the final mipmap, which has dimension 1×1. To define the mipmaps,
    /// call [texture_image_1d], [texture_image_2d], [texture_image_3d], [copy_texture_image_1d],
    /// or [copy_texture_image_2d] with the level argument indicating the order of the mipmaps.
    /// Level 0 is the original texture; level max(n,m) is the final 1×1 mipmap.
    ///
    /// As more texture elements are sampled in the minification process, fewer aliasing artifacts
    /// will be apparent. While the [Nearest](TextureMinFilter::Nearest) and
    /// [Linear](TextureMinFilter::Linear) minification functions can be faster than the other four,
    /// they sample only one or multiple texture elements to determine the texture value of the pixel
    /// being rendered and can produce moire patterns or ragged transitions.
    ///
    /// The initial value is [NearestMipmapLinear](TextureMinFilter::NearestMipmapLinear).
    fn min_filter(self, filter: TextureMinFilter) -> Result<(), Error>;

    /// Specifies the magnification function used when sampling textures
    ///
    /// # Description
    /// The texture magnification function is used whenever the level-of-detail function used when
    /// sampling from the texture determines that the texture should be magnified. It sets the texture
    /// magnification function to either [Nearest](TextureMinFilter::Nearest) or
    /// [Linear](TextureMinFilter::Linear). [Nearest](TextureMinFilter::Nearest) is generally faster
    /// than [Linear](TextureMinFilter::Linear), but it can produce textured images with sharper edges
    /// because the transition between texture elements is not as smooth.
    ///
    /// The initial value is [Linear](TextureMinFilter::Linear).
    fn mag_filter(self, filter: TextureMagFilter) -> Result<(), Error>;

    /// Sets the wrap parameter for texture coordinate corresponding to `wrap_target`.
    ///
    /// Initially is set to [Repeat](TextureWrapMode::Repeat).
    fn wrap(self, wrap_target: TextureWrapTarget, mode: TextureWrapMode) -> Result<(), Error>;
}

impl<T: TextureOrTarget + Copy> TextureParameter for T {
    fn depth_stencil_mode(self, mode: TextureDepthStencilMode) -> Result<(), Error> {
        let mode = GLenum::from(mode) as GLint;
        self.set_i32(gl::DEPTH_STENCIL_TEXTURE_MODE, mode);
        handle_error()
    }

    fn min_filter(self, filter: TextureMinFilter) -> Result<(), Error> {
        self.set_i32(gl::TEXTURE_MIN_FILTER, filter.into());
        handle_error()
    }

    fn mag_filter(self, filter: TextureMagFilter) -> Result<(), Error> {
        let filter = GLenum::from(filter) as GLint;
        self.set_i32(gl::TEXTURE_MIN_FILTER, filter);
        handle_error()
    }

    fn wrap(self, wrap_target: TextureWrapTarget, mode: TextureWrapMode) -> Result<(), Error> {
        let target: GLenum = wrap_target.into();
        let value = GLenum::from(mode) as GLint;
        self.set_i32(target, value);
        handle_error()
    }
}

trait TextureOrTarget {
    fn set_i32(self, pname: GLenum, param: GLint);
    fn set_f32(self, pname: GLenum, param: GLfloat);

    fn set_i32_values(self, pname: GLenum, params: &[GLint]);
    fn set_f32_values(self, pname: GLenum, params: &[GLfloat]);

    fn set_i32_integer_values(self, pname: GLenum, params: &[GLint]);
    fn set_u32_integer_values(self, pname: GLenum, params: &[GLuint]);
}

impl TextureOrTarget for Texture {
    fn set_i32(self, pname: GLenum, param: GLint) {
        unsafe { gl::TextureParameteri(self.0, pname, param) }
    }

    fn set_f32(self, pname: GLenum, param: GLfloat) {
        unsafe { gl::TextureParameterf(self.0, pname, param) }
    }

    fn set_i32_values(self, pname: GLenum, params: &[GLint]) {
        unsafe { gl::TextureParameteriv(self.0, pname, params.as_ptr()) }
    }

    fn set_f32_values(self, pname: GLenum, params: &[GLfloat]) {
        unsafe { gl::TextureParameterfv(self.0, pname, params.as_ptr()) }
    }

    fn set_i32_integer_values(self, pname: GLenum, params: &[GLint]) {
        unsafe { gl::TextureParameterIiv(self.0, pname, params.as_ptr()) }
    }

    fn set_u32_integer_values(self, pname: GLenum, params: &[GLuint]) {
        unsafe { gl::TextureParameterIuiv(self.0, pname, params.as_ptr()) }
    }
}

impl TextureOrTarget for TextureBindingTarget {
    fn set_i32(self, pname: GLenum, param: GLint) {
        let target: GLenum = self.into();
        unsafe { gl::TexParameteri(target, pname, param) }
    }

    fn set_f32(self, pname: GLenum, param: GLfloat) {
        let target: GLenum = self.into();
        unsafe { gl::TexParameterf(target, pname, param) }
    }

    fn set_i32_values(self, pname: GLenum, params: &[GLint]) {
        let target: GLenum = self.into();
        unsafe { gl::TexParameteriv(target, pname, params.as_ptr()) }
    }

    fn set_f32_values(self, pname: GLenum, params: &[GLfloat]) {
        let target: GLenum = self.into();
        unsafe { gl::TexParameterfv(target, pname, params.as_ptr()) }
    }

    fn set_i32_integer_values(self, pname: GLenum, params: &[GLint]) {
        let target: GLenum = self.into();
        unsafe { gl::TexParameterIiv(target, pname, params.as_ptr()) }
    }

    fn set_u32_integer_values(self, pname: GLenum, params: &[GLuint]) {
        let target: GLenum = self.into();
        unsafe { gl::TexParameterIuiv(target, pname, params.as_ptr()) }
    }
}

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

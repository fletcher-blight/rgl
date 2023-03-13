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

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextureBinding2DTarget {
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

impl From<TextureBinding2DTarget> for u32 {
    fn from(value: TextureBinding2DTarget) -> Self {
        match value {
            TextureBinding2DTarget::Image2D => gl::TEXTURE_2D,
            TextureBinding2DTarget::Proxy2D => gl::PROXY_TEXTURE_2D,
            TextureBinding2DTarget::Image1DArray => gl::TEXTURE_1D_ARRAY,
            TextureBinding2DTarget::Proxy1DArray => gl::PROXY_TEXTURE_1D_ARRAY,
            TextureBinding2DTarget::Rectangle => gl::TEXTURE_RECTANGLE,
            TextureBinding2DTarget::ProxyRectangle => gl::PROXY_TEXTURE_RECTANGLE,
            TextureBinding2DTarget::CubeMapPositiveX => gl::TEXTURE_CUBE_MAP_POSITIVE_X,
            TextureBinding2DTarget::CubeMapPositiveY => gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
            TextureBinding2DTarget::CubeMapPositiveZ => gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
            TextureBinding2DTarget::CubeMapNegativeX => gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
            TextureBinding2DTarget::CubeMapNegativeY => gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
            TextureBinding2DTarget::CubeMapNegativeZ => gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
            TextureBinding2DTarget::ProxyCubeMap => gl::PROXY_TEXTURE_CUBE_MAP,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextureInternalFormat {
    DepthComponent,
    DepthStencil,
    R,
    RG,
    RGB,
    RGBA,
    // TODO: Table2 + Table3
}

impl From<TextureInternalFormat> for GLenum {
    fn from(value: TextureInternalFormat) -> Self {
        match value {
            TextureInternalFormat::DepthComponent => gl::DEPTH_COMPONENT,
            TextureInternalFormat::DepthStencil => gl::DEPTH_STENCIL,
            TextureInternalFormat::R => gl::RED,
            TextureInternalFormat::RG => gl::RG,
            TextureInternalFormat::RGB => gl::RGB,
            TextureInternalFormat::RGBA => gl::RGBA,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextureFormat {
    R,
    RG,
    RGB,
    BGR,
    RGBA,
    BGRA,
    I32R,
    I32RG,
    I32RGB,
    I32BGR,
    I32RGBA,
    I32BGRA,
    StencilIndex,
    DepthComponent,
    DepthStencil,
}

impl From<TextureFormat> for GLenum {
    fn from(value: TextureFormat) -> Self {
        match value {
            TextureFormat::R => gl::RED,
            TextureFormat::RG => gl::RG,
            TextureFormat::RGB => gl::RGB,
            TextureFormat::BGR => gl::BGR,
            TextureFormat::RGBA => gl::RGBA,
            TextureFormat::BGRA => gl::BGRA,
            TextureFormat::I32R => gl::RED_INTEGER,
            TextureFormat::I32RG => gl::RG_INTEGER,
            TextureFormat::I32RGB => gl::RGB_INTEGER,
            TextureFormat::I32BGR => gl::BGR_INTEGER,
            TextureFormat::I32RGBA => gl::RGBA_INTEGER,
            TextureFormat::I32BGRA => gl::BGRA_INTEGER,
            TextureFormat::StencilIndex => gl::STENCIL_INDEX,
            TextureFormat::DepthComponent => gl::DEPTH_COMPONENT,
            TextureFormat::DepthStencil => gl::DEPTH_STENCIL,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TexturePixelType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F16,
    F32,
    U8r3g3b2,
    U8b2g3r3,
    U16r5g6b5,
    U16b5g6r5,
    U16r4g4b4a4,
    U16a4b4g4r4,
    U16r5g5b5a1,
    U16a1b5g5r5,
    U32r8g8b8a8,
    U32a8b8g8r8,
    U32r10g10b10a2,
    U32a2b10g10r10,
    U32a5b9g9r9,
    U32b10fg11fr11f,
}

impl From<TexturePixelType> for GLenum {
    fn from(value: TexturePixelType) -> Self {
        match value {
            TexturePixelType::U8 => gl::UNSIGNED_BYTE,
            TexturePixelType::I8 => gl::BYTE,
            TexturePixelType::U16 => gl::UNSIGNED_SHORT,
            TexturePixelType::I16 => gl::SHORT,
            TexturePixelType::U32 => gl::UNSIGNED_INT,
            TexturePixelType::I32 => gl::INT,
            TexturePixelType::F16 => gl::HALF_FLOAT,
            TexturePixelType::F32 => gl::FLOAT,
            TexturePixelType::U8r3g3b2 => gl::UNSIGNED_BYTE_3_3_2,
            TexturePixelType::U8b2g3r3 => gl::UNSIGNED_BYTE_2_3_3_REV,
            TexturePixelType::U16r5g6b5 => gl::UNSIGNED_SHORT_5_6_5,
            TexturePixelType::U16b5g6r5 => gl::UNSIGNED_SHORT_5_6_5_REV,
            TexturePixelType::U16r4g4b4a4 => gl::UNSIGNED_SHORT_4_4_4_4,
            TexturePixelType::U16a4b4g4r4 => gl::UNSIGNED_SHORT_4_4_4_4_REV,
            TexturePixelType::U16r5g5b5a1 => gl::UNSIGNED_SHORT_5_5_5_1,
            TexturePixelType::U16a1b5g5r5 => gl::UNSIGNED_SHORT_1_5_5_5_REV,
            TexturePixelType::U32r8g8b8a8 => gl::UNSIGNED_INT_8_8_8_8,
            TexturePixelType::U32a8b8g8r8 => gl::UNSIGNED_INT_8_8_8_8_REV,
            TexturePixelType::U32r10g10b10a2 => gl::UNSIGNED_INT_10_10_10_2,
            TexturePixelType::U32a2b10g10r10 => gl::UNSIGNED_INT_2_10_10_10_REV,
            TexturePixelType::U32a5b9g9r9 => gl::UNSIGNED_INT_5_9_9_9_REV,
            TexturePixelType::U32b10fg11fr11f => gl::UNSIGNED_INT_10F_11F_11F_REV,
        }
    }
}

pub enum TextureData<'data, DataType> {
    Data(&'data [DataType]),
    Offset(u64),
    Reserve,
}

/// See [texture_target_wrap]
pub enum TextureWrapTarget {
    S,
    T,
    R,
}

impl From<TextureWrapTarget> for u32 {
    fn from(value: TextureWrapTarget) -> Self {
        match value {
            TextureWrapTarget::S => gl::TEXTURE_WRAP_S,
            TextureWrapTarget::T => gl::TEXTURE_WRAP_T,
            TextureWrapTarget::R => gl::TEXTURE_WRAP_R,
        }
    }
}

pub enum TextureWrapMode {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    ClampToBorder,
    MirrorClampToEdge,
}

impl From<TextureWrapMode> for u32 {
    fn from(value: TextureWrapMode) -> Self {
        match value {
            TextureWrapMode::Repeat => gl::REPEAT,
            TextureWrapMode::MirroredRepeat => gl::MIRRORED_REPEAT,
            TextureWrapMode::ClampToEdge => gl::CLAMP_TO_EDGE,
            TextureWrapMode::ClampToBorder => gl::CLAMP_TO_BORDER,
            TextureWrapMode::MirrorClampToEdge => gl::MIRROR_CLAMP_TO_EDGE,
        }
    }
}

/// # The texture minifying function
/// see [texture_target_min_filter]
pub enum TextureMinFilter {
    /// Returns the value of the texture element that is nearest (in Manhattan distance) to the
    /// specified texture coordinates.
    Nearest,

    /// Returns the weighted average of the four texture elements that are closest to the specified
    /// texture coordinates. These can include items wrapped or repeated from other parts of a
    /// texture, depending on the values of [TextureWrapTarget::S] and [TextureWrapTarget::T], and
    /// on the exact mapping.
    Linear,

    /// Chooses the mipmap that most closely matches the size of the pixel being textured and uses
    /// the [TextureMinFilter::Nearest] criterion (the texture element closest to the specified
    /// texture coordinates) to produce a texture value.
    NearestMipmapNearest,

    /// Chooses the mipmap that most closely matches the size of the pixel being textured and uses
    /// the [TextureMinFilter::Linear] criterion (a weighted average of the four texture elements
    /// that are closest to the specified texture coordinates) to produce a texture value.
    LinearMipmapNearest,

    /// Chooses the two mipmaps that most closely match the size of the pixel being textured and
    /// uses the [TextureMinFilter::Nearest] criterion (the texture element closest to the specified
    /// texture coordinates) to produce a texture value from each mipmap. The final texture value is
    /// a weighted average of those two values.
    NearestMipmapLinear,

    /// Chooses the two mipmaps that most closely match the size of the pixel being textured and
    /// uses the [TextureMinFilter::Nearest] criterion (a weighted average of the texture elements
    /// that are closest to the specified texture coordinates) to produce a texture value from each
    /// mipmap. The final texture value is a weighted average of those two values.
    LinearMipmapLinear,
}

impl From<TextureMinFilter> for u32 {
    fn from(value: TextureMinFilter) -> Self {
        match value {
            TextureMinFilter::Nearest => gl::NEAREST,
            TextureMinFilter::Linear => gl::LINEAR,
            TextureMinFilter::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
            TextureMinFilter::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
            TextureMinFilter::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
            TextureMinFilter::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
        }
    }
}

/// # The texture magnification function
/// see [texture_target_mag_filter]
pub enum TextureMagFilter {
    /// Returns the value of the texture element that is nearest (in Manhattan distance) to the
    /// specified texture coordinates.
    Nearest,

    /// Returns the weighted average of the texture elements that are closest to the specified
    /// texture coordinates. These can include items wrapped or repeated from other parts of a
    /// texture, depending on the values of [TextureWrapTarget::S] and [TextureWrapTarget::T], and
    /// on the exact mapping.
    Linear,
}

impl From<TextureMagFilter> for u32 {
    fn from(value: TextureMagFilter) -> Self {
        match value {
            TextureMagFilter::Nearest => gl::NEAREST,
            TextureMagFilter::Linear => gl::LINEAR,
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

/// # Determine if a name corresponds to a texture
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glIsTexture.xhtml>
///
/// # Arguments
/// * `texture` - Specifies a value that may be the name of a texture.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// assert!(is_texture(Texture(42)));
/// assert!(!is_texture(Texture(0)));
/// ```
///
/// # Description
/// [is_texture] returns true if `texture` is currently the name of a texture. If `texture` is zero,
/// or is a non-zero value that is not currently the name of a texture, or if an error occurs,
/// [is_texture] returns false.
///
/// A name returned by [gen_textures], but not yet associated with a texture by calling
/// [bind_texture], is not the name of a texture.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [is_texture] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_texture]
/// * [copy_tex_image_1d], [copy_tex_image_2d]
/// * [delete_textures]
/// * [gen_textures]
/// * [get_ex_parameter]
/// * [tex_image_1d], [tex_image_2d], [tex_image_3d]
/// * [tex_parameter]
pub fn is_texture(texture: Texture) -> bool {
    let texture = texture.0;

    // SAFE: synchronous integer copy
    let val = unsafe { gl::IsTexture(texture) };
    val == gl::TRUE
}

/// # Specify a two-dimensional texture image
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target texture.
/// * `level` - Specifies the level-of-detail number. Level 0 is the base image level. Level `n` is
/// the `n`th mipmap reduction image. If target is [TextureBinding2DTarget::Rectangle] or
/// [TextureBinding2DTarget::ProxyRectangle], `level` must be 0.
/// * `internal_format` - Specifies the number of color components in the texture.
/// * `width` - Specifies the width of the texture image. All implementations support texture images
/// that are at least 1024 texels wide.
/// * `height` - Specifies the height of the texture image, or the number of layers in a texture
/// array, in the case of the [TextureBinding2DTarget::Image1DArray] and
/// [TextureBinding2DTarget::Proxy1DArray] targets. All implementations support 2D texture images
/// that are at least 1024 texels high, and texture arrays that are at least 256 layers deep.
/// * `format` - Specifies the format of the pixel data.
/// * `pixel_data_type` - Specifies the data type of the pixel data.
/// * `data` - Specifies a slice to the image data in memory, or single `u64` offset to a
/// [BufferBindingTarget::PixelUnpack] buffer.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// tex_image_2d(
///     TextureBinding2DTarget::Image2D,
///     0,
///     TextureInternalFormat::RGB,
///     1920,
///     1080,
///     TextureFormat::RGBA,
///     TexturePixelType::U8,
///     TextureData::Data(&[0, 0, 1, 1, 0, 1, 0, 1]),
/// );
/// ```
///
/// # Description
/// Texturing allows elements of an image array to be read by shaders.
///
/// To define texture images, call [tex_image_2d]. The arguments describe the parameters of the
/// texture image, such as `height`, `width`, level-of-detail number (see [tex_parameter]), and
/// number of colour components provided. The last three arguments describe how the image is
/// represented in memory.
///
/// If `target` is [TextureBinding2DTarget::Proxy2D], [TextureBinding2DTarget::Proxy1DArray],
/// [TextureBinding2DTarget::ProxyCubeMap], or [TextureBinding2DTarget::ProxyRectangle], no data is
/// read from data, but all of the texture image state is recalculated, checked for consistency, and
/// checked against the implementation's capabilities. If the implementation cannot handle a texture
/// of the requested texture size, it sets all of the image state to 0, but does not generate an
/// error (see [get_error]). To query for an entire mipmap array, use an image array level greater
/// than or equal to 1.
///
/// If `target` is [TextureBinding2DTarget::Image2D], [TextureBinding2DTarget::Rectangle] or one of
/// the `TextureBinding2DTarget::CubeMap*` targets, data is read from data as a sequence of `i8`,
/// `u8`, `i16`, `u16`, `i32`, `u32`, or `f32`, depending on type. These values are grouped into
/// sets of one, two, three, or four values, depending on format, to form elements. Each data byte
/// is treated as eight 1-bit elements, with bit ordering determined by
/// [pixel_store_unpack_lsb_first].
///
/// If `target` is [TextureBinding2DTarget::Image1DArray], data is interpreted as an array of
/// one-dimensional images.
///
/// If a non-zero named buffer object is bound to the [BufferBindingTarget::PixelUnpack] target
/// (see [bind_buffer]) while a texture image is specified, `data` is treated as a byte offset into
/// the buffer object's data store. This is handled by using a single offset `u64` values as data
/// instead of a slice.
///
/// The first element corresponds to the lower left corner of the texture image. Subsequent elements
/// progress left-to-right through the remaining texels in the lowest row of the texture image, and
/// then in successively higher rows of the texture image. The final element corresponds to the
/// upper right corner of the texture image.
///
/// `format` determines the composition of each element in `data`.
///
/// If an application wants to store the texture at a certain resolution or in a certain format, it
/// can request the resolution and format with `internal_format`. The GL will choose an internal
/// representation that closely approximates that requested by `internal_format`, but it may not
/// match exactly. (The representations specified by [TextureInternalFormat::R],
/// [TextureInternalFormat::RG], [TextureInternalFormat::RGB], and [TextureInternalFormat::RGB] must
/// match exactly.)
///
/// If the `internal_format` parameter is one of the generic compressed formats,
/// [TextureInternalFormat::CompressedR], [TextureInternalFormat::CompressedRG],
/// [TextureInternalFormat::CompressedRGB], or [TextureInternalFormat::CompressedRGBA], the GL will
/// replace the internal format with the symbolic constant for a specific internal format and
/// compress the texture before storage. If no corresponding internal format is available, or the GL
/// can not compress that image for any reason, the internal format is instead replaced with a
/// corresponding base internal format.
///
/// If the `internal_format` parameter is [TextureInternalFormat::SRGB],
/// [TextureInternalFormat::SRGB8], [TextureInternalFormat::SRGBA], or
/// [TextureInternalFormat::SRGBA8], the texture is treated as if the red, green, or blue components
/// are encoded in the sRGB color space. Any alpha component is left unchanged. The conversion from
/// the sRGB encoded component c<sub>s</sub> to a linear component c<sub>l</sub> is:
///
/// TODO (latex like using https://docs.rs/rustdoc-katex-demo/0.1.5/rustdoc_katex_demo/index.html)
/// Assume c<sub>s</sub> is the sRGB component in the range \[0,1\].
///
/// Use the [TextureBinding2DTarget::Proxy2D], [TextureBinding2DTarget::Proxy1DArray],
/// [TextureBinding2DTarget::ProxyRectangle], or [TextureBinding2DTarget::ProxyCubeMap] target to
/// try out a resolution and format. The implementation will update and recompute its best match for
/// the requested storage resolution and format. To then query this state, call
/// [get_tex_level_parameter]. If the texture cannot be accommodated, texture state is set to 0.
///
/// A one-component texture image uses only the red component of the RGBA colour extracted from
/// data. A two-component image uses the R and G values. A three-component image uses the R, G, and
/// B values. A four-component image uses all of the RGBA components.
///
/// Image-based shadowing can be enabled by comparing texture r coordinates to depth texture values
/// to generate a boolean result. See [tex_parameter] for details on texture comparison.
///
/// The [pixel_store] mode affects texture images.
///
/// `data` may be a single `u64` value. In this case, texture memory is allocated to accommodate a
/// texture of width `width` and height `height`. You can then download subtextures to initialize
/// this texture memory. The image is undefined if the user tries to apply an uninitialized portion
/// of the texture image to a primitive.
///
/// [tex_image_2d] specifies the two-dimensional texture for the current texture unit, specified
/// with [active_texture].
///
/// # Compatability
/// * 3.0 - [TexturePixelType::F16]
/// * 4.4 - [TextureFormat::StencilIndex]
///
/// # Errors
/// * [Error::InvalidEnum] - if `target` is one of the six cube map 2D image targets and the `width`
/// and `height` parameters are not equal.
/// * [Error::InvalidValue] - if `width` or `height` is greater than [get_max_texture_size]
/// * [Error::InvalidValue] - if `target` is [TextureBinding2DTarget::Image1DArray] or
/// [TextureBinding2DTarget::Proxy1DArray] and `height` is greater than
/// [get_max_array_texture_layers]
/// * [Error::InvalidValue] - if `level` is greater than log<sub>2</sub>([get_max_texture_size])
/// * [Error::InvalidOperation] - if `pixel_data_type` is one of [TexturePixelType::U8r3g3b2],
/// [TexturePixelType::U8b2g3r3], [TexturePixelType::U16r5g6b5], [TexturePixelType::U16b5g6r5], or
/// [TexturePixelType::U32b10fg11fr11f], and `format` is not [TextureFormat::RGB].
/// * [Error::InvalidOperation] - if `pixel_data_type` is one of [TexturePixelType::U16r4g4b4a4],
/// [TexturePixelType::U16a4b4g4r4], [TexturePixelType::U16r5g5b5a1],
/// [TexturePixelType::U16a1b5g5r5], [TexturePixelType::U32r8g8b8a8],
/// [TexturePixelType::U32a8b8g8r8], [TexturePixelType::U32r10g10b10a2],
/// [TexturePixelType::U32a2b10g10r10], [TexturePixelType::U32a5b9g9r9], and `format` is neither
/// [TextureFormat::RGBA] or [TextureFormat::BGRA].
/// * TODO - there is more ...
///
/// # Associated Gets
/// * [get_tex_image]
/// * [get_pixel_unpack_buffer_binding]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [tex_image_2d] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [active_texture]
/// * [copy_tex_image_1d], [copy_tex_image_2d],
/// * [copy_tex_sub_image_1d], [copy_tex_sub_image_2d], [copy_tex_sub_image_3d]
/// * [pixel_store]
/// * [tex_image_1d], [tex_image_3d]
/// * [tex_sub_image_1d], [tex_sub_image_2d], [tex_sub_image_2d]
/// * [tex_parameter]
#[allow(clippy::too_many_arguments)]
pub fn tex_image_2d<DataType>(
    target: TextureBinding2DTarget,
    level: u32,
    internal_format: TextureInternalFormat,
    width: u32,
    height: u32,
    format: TextureFormat,
    pixel_data_type: TexturePixelType,
    data: TextureData<DataType>,
) {
    let target = GLenum::from(target);
    let level = level as GLint;
    let internal_format = GLenum::from(internal_format) as GLint;
    let width = width as GLsizei;
    let height = height as GLsizei;
    let format = GLenum::from(format);
    let type_ = GLenum::from(pixel_data_type);

    let data = match data {
        TextureData::Data(data) => data.as_ptr(),
        TextureData::Offset(offset) => offset as *const _,
        TextureData::Reserve => std::ptr::null(),
    } as *const std::os::raw::c_void;

    // SAFE: synchronous read of `data`, no memory retained
    unsafe {
        gl::TexImage2D(
            target,
            level,
            internal_format,
            width,
            height,
            0,
            format,
            type_,
            data,
        )
    }
}

/// # Set texture parameters
pub mod tex_parameter {
    use crate::prelude::*;
    use gl::types::*;

    fn tex_param_i32(target: TextureBindingTarget, pname: GLenum, param: i32) {
        let target = GLenum::from(target);

        // SAFE: synchronous integer copy
        unsafe { gl::TexParameteri(target, pname, param) }
    }

    pub fn texture_target_wrap(
        target: TextureBindingTarget,
        wrap_target: TextureWrapTarget,
        mode: TextureWrapMode,
    ) {
        let pname = GLenum::from(wrap_target);
        let param = GLenum::from(mode) as i32;
        tex_param_i32(target, pname, param)
    }

    pub fn texture_target_min_filter(target: TextureBindingTarget, filter: TextureMinFilter) {
        let param = GLenum::from(filter) as i32;
        tex_param_i32(target, gl::TEXTURE_MIN_FILTER, param)
    }

    pub fn texture_target_mag_filter(target: TextureBindingTarget, filter: TextureMagFilter) {
        let param = GLenum::from(filter) as i32;
        tex_param_i32(target, gl::TEXTURE_MAG_FILTER, param)
    }
}
pub use tex_parameter::*;

use crate::*;
use gl::types::*;

#[derive(Debug, Copy, Clone)]
pub enum TextureFormat {
    Red,
    RG,
    RGB,
    BGR,
    RGBA,
    BGRA,
    RedInteger,
    RGInteger,
    RGBInteger,
    BGRInteger,
    RGBAInteger,
    BGRAInteger,
    StencilIndex,
    DepthComponent,
    DepthStencil,
}

impl From<TextureFormat> for GLenum {
    fn from(value: TextureFormat) -> Self {
        match value {
            TextureFormat::Red => gl::RED,
            TextureFormat::RG => gl::RG,
            TextureFormat::RGB => gl::RGB,
            TextureFormat::BGR => gl::BGR,
            TextureFormat::RGBA => gl::RGBA,
            TextureFormat::BGRA => gl::BGRA,
            TextureFormat::RedInteger => gl::RED_INTEGER,
            TextureFormat::RGInteger => gl::RG_INTEGER,
            TextureFormat::RGBInteger => gl::RGB_INTEGER,
            TextureFormat::BGRInteger => gl::BGR_INTEGER,
            TextureFormat::RGBAInteger => gl::RGBA_INTEGER,
            TextureFormat::BGRAInteger => gl::BGRA_INTEGER,
            TextureFormat::StencilIndex => gl::STENCIL_INDEX,
            TextureFormat::DepthComponent => gl::DEPTH_COMPONENT,
            TextureFormat::DepthStencil => gl::DEPTH_STENCIL,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Texture2DTarget {
    Image2D,
    Proxy2D,
    Image1DArray,
    Proxy1DArray,
    CubeMapPositiveX,
    CubeMapNegativeX,
    CubeMapPositiveY,
    CubeMapNegativeY,
    CubeMapPositiveZ,
    CubeMapNegativeZ,
    ProxyCubeMap,
}

impl From<Texture2DTarget> for GLenum {
    fn from(value: Texture2DTarget) -> Self {
        match value {
            Texture2DTarget::Image2D => gl::TEXTURE_2D,
            Texture2DTarget::Proxy2D => gl::PROXY_TEXTURE_2D,
            Texture2DTarget::Image1DArray => gl::TEXTURE_1D_ARRAY,
            Texture2DTarget::Proxy1DArray => gl::PROXY_TEXTURE_1D_ARRAY,
            Texture2DTarget::CubeMapPositiveX => gl::TEXTURE_CUBE_MAP_POSITIVE_X,
            Texture2DTarget::CubeMapNegativeX => gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
            Texture2DTarget::CubeMapPositiveY => gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
            Texture2DTarget::CubeMapNegativeY => gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
            Texture2DTarget::CubeMapPositiveZ => gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
            Texture2DTarget::CubeMapNegativeZ => gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
            Texture2DTarget::ProxyCubeMap => gl::PROXY_TEXTURE_CUBE_MAP,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TextureInternalFormat {
    DepthComponent,
    DepthStencil,
    Red,
    RG,
    RGB,
    RGBA,
}

impl From<TextureInternalFormat> for GLenum {
    fn from(value: TextureInternalFormat) -> Self {
        match value {
            TextureInternalFormat::DepthComponent => gl::DEPTH_COMPONENT,
            TextureInternalFormat::DepthStencil => gl::DEPTH_STENCIL,
            TextureInternalFormat::Red => gl::RED,
            TextureInternalFormat::RG => gl::RG,
            TextureInternalFormat::RGB => gl::RGB,
            TextureInternalFormat::RGBA => gl::RGBA,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TexturePixelDataType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F16,
    F32,
    R3G3B2,
    B2G3R3,
    R5G6B5,
    B5G6R5,
    R4G4B4A4,
    A4B4G4R4,
    R5G5B5A1,
    A1B5G5R5,
    R8G8B8A8,
    A8B8G8R8,
    R10G10B10A2,
    A2B10G10R10,
}

impl From<TexturePixelDataType> for GLenum {
    fn from(value: TexturePixelDataType) -> Self {
        match value {
            TexturePixelDataType::U8 => gl::UNSIGNED_BYTE,
            TexturePixelDataType::I8 => gl::BYTE,
            TexturePixelDataType::U16 => gl::UNSIGNED_SHORT,
            TexturePixelDataType::I16 => gl::SHORT,
            TexturePixelDataType::U32 => gl::UNSIGNED_INT,
            TexturePixelDataType::I32 => gl::INT,
            TexturePixelDataType::F16 => gl::HALF_FLOAT,
            TexturePixelDataType::F32 => gl::FLOAT,
            TexturePixelDataType::R3G3B2 => gl::UNSIGNED_BYTE_3_3_2,
            TexturePixelDataType::B2G3R3 => gl::UNSIGNED_BYTE_2_3_3_REV,
            TexturePixelDataType::R5G6B5 => gl::UNSIGNED_SHORT_5_6_5,
            TexturePixelDataType::B5G6R5 => gl::UNSIGNED_SHORT_5_6_5_REV,
            TexturePixelDataType::R4G4B4A4 => gl::UNSIGNED_SHORT_4_4_4_4,
            TexturePixelDataType::A4B4G4R4 => gl::UNSIGNED_SHORT_4_4_4_4_REV,
            TexturePixelDataType::R5G5B5A1 => gl::UNSIGNED_SHORT_5_5_5_1,
            TexturePixelDataType::A1B5G5R5 => gl::UNSIGNED_SHORT_1_5_5_5_REV,
            TexturePixelDataType::R8G8B8A8 => gl::UNSIGNED_INT_8_8_8_8,
            TexturePixelDataType::A8B8G8R8 => gl::UNSIGNED_INT_8_8_8_8_REV,
            TexturePixelDataType::R10G10B10A2 => gl::UNSIGNED_INT_10_10_10_2,
            TexturePixelDataType::A2B10G10R10 => gl::UNSIGNED_INT_2_10_10_10_REV,
        }
    }
}

pub fn texture_image_2d<Data>(
    target: Texture2DTarget,
    level: u16,
    internal_format: TextureInternalFormat,
    width: u32,
    height: u32,
    format: TextureFormat,
    pixel_data_type: TexturePixelDataType,
    data: &[Data],
) -> Result<(), Error> {
    let target = GLenum::from(target);
    let level = level as GLint;
    let internal_format = GLenum::from(internal_format) as GLint;
    let width = width as GLint;
    let height = height as GLint;
    let format = GLenum::from(format);
    let type_ = GLenum::from(pixel_data_type);
    let pixels = data.as_ptr() as *const std::os::raw::c_void;

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
            pixels,
        )
    };

    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        _ => todo!(),
    }
}

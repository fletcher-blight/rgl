//! # Buffer Objects
//! <https://www.khronos.org/opengl/wiki/Category:Core_API_Ref_Buffer_Objects>
//!
//! # Description
//! The core OpenGL API reference for functions that create, manage, and delete
//! [buffer objects](https://www.khronos.org/opengl/wiki/Buffer_Object). These functions are only
//! for getting data into and out of the buffers; the
//! [different uses of buffer objects](https://www.khronos.org/opengl/wiki/Buffer_Object#General_use)
//! are not covered by these functions.

use crate::*;
use gl::types::*;

#[derive(Debug, Copy, Clone)]
pub enum BufferAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

#[derive(Debug, Copy, Clone)]
pub enum BufferError {
    Unexpected(Error),

    UnboundTarget(BufferBindingTarget),
    InvalidBuffer(Buffer),
    InvalidParameterValue(i64),
}

/// # The target to which a buffer object is bound
/// see [bind_buffer] for more details
#[derive(Debug, Copy, Clone)]
pub enum BufferBindingTarget {
    /// Vertex attributes
    Array,

    /// Atomic counter storage
    AtomicCounter,

    /// Buffer copy source
    CopyRead,

    /// Buffer copy destination
    CopyWrite,

    /// Indirect compute dispatch commands
    DispatchIndirect,

    /// Indirect command arguments
    DrawIndirect,

    /// Vertex array indices
    ElementArray,

    /// Pixel read target
    PixelPack,

    /// Texture data source
    PixelUnpack,

    /// Query result buffer
    Query,

    /// Read-write storage for shaders
    ShaderStorage,

    /// Texture data buffer
    Texture,

    /// Transform feedback buffer
    TransformFeedback,

    /// Uniform block storage
    Uniform,
}

impl From<BufferBindingTarget> for GLenum {
    fn from(target: BufferBindingTarget) -> Self {
        match target {
            BufferBindingTarget::Array => gl::ARRAY_BUFFER,
            BufferBindingTarget::AtomicCounter => gl::ATOMIC_COUNTER_BUFFER,
            BufferBindingTarget::CopyRead => gl::COPY_READ_BUFFER,
            BufferBindingTarget::CopyWrite => gl::COPY_WRITE_BUFFER,
            BufferBindingTarget::DispatchIndirect => gl::DISPATCH_INDIRECT_BUFFER,
            BufferBindingTarget::DrawIndirect => gl::DRAW_INDIRECT_BUFFER,
            BufferBindingTarget::ElementArray => gl::ELEMENT_ARRAY_BUFFER,
            BufferBindingTarget::PixelPack => gl::PIXEL_PACK_BUFFER,
            BufferBindingTarget::PixelUnpack => gl::PIXEL_UNPACK_BUFFER,
            BufferBindingTarget::Query => gl::QUERY_BUFFER,
            BufferBindingTarget::ShaderStorage => gl::SHADER_STORAGE_BUFFER,
            BufferBindingTarget::Texture => gl::TEXTURE_BUFFER,
            BufferBindingTarget::TransformFeedback => gl::TRANSFORM_FEEDBACK_BUFFER,
            BufferBindingTarget::Uniform => gl::UNIFORM_BUFFER,
        }
    }
}

pub enum BufferAccessFrequency {
    Stream,
    Static,
    Dynamic,
}

pub enum BufferAccessNature {
    Draw,
    Read,
    Copy,
}

struct BufferUsage(BufferAccessFrequency, BufferAccessNature);
impl From<BufferUsage> for GLenum {
    fn from(BufferUsage(frequency, nature): BufferUsage) -> Self {
        match (frequency, nature) {
            (BufferAccessFrequency::Stream, BufferAccessNature::Draw) => gl::STREAM_DRAW,
            (BufferAccessFrequency::Stream, BufferAccessNature::Read) => gl::STREAM_READ,
            (BufferAccessFrequency::Stream, BufferAccessNature::Copy) => gl::STREAM_COPY,
            (BufferAccessFrequency::Static, BufferAccessNature::Draw) => gl::STATIC_DRAW,
            (BufferAccessFrequency::Static, BufferAccessNature::Read) => gl::STATIC_READ,
            (BufferAccessFrequency::Static, BufferAccessNature::Copy) => gl::STATIC_COPY,
            (BufferAccessFrequency::Dynamic, BufferAccessNature::Draw) => gl::DYNAMIC_DRAW,
            (BufferAccessFrequency::Dynamic, BufferAccessNature::Read) => gl::DYNAMIC_READ,
            (BufferAccessFrequency::Dynamic, BufferAccessNature::Copy) => gl::DYNAMIC_COPY,
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Buffer(pub u32);

/// # Bind a named buffer object
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBindBuffer.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
/// * `buffer` - Specifies the name of a buffer object
///
/// # Description
/// [bind_buffer]] binds a buffer object to the specified buffer binding point. Calling
/// [bind_buffer] with `buffer` set to the name of a buffer object binds that buffer object name to
/// the `target`. If no buffer object with name `buffer` exists, one is created with that name. When
/// a buffer object is bound to a target, the previous binding for that target is automatically
/// broken.
///
/// Buffer object names are unsigned integers. The value zero is reserved, but there is no default
/// buffer object for each buffer object target. Instead, `buffer` set to zero effectively unbinds
/// any buffer object previously bound, and restores client memory usage for that buffer object
/// target (if supported for that target). Buffer object names and the corresponding buffer object
/// contents are local to the shared object space of the current GL rendering context; two rendering
/// contexts share buffer object names only if they explicitly enable sharing between contexts
/// through the appropriate GL windows interfaces functions.
///
/// [gen_buffers] must be used to generate a set of unused buffer object names.
///
/// The state of a buffer object immediately after it is first bound is an unmapped zero-sized
/// memory buffer with [BufferAccess::ReadWrite] access and ([BufferAccessFrequency::Static],
/// [BufferAccessNature::Draw]) usage.
///
/// While a non-zero buffer object name is bound, GL operations on the target to which it is bound
/// affect the bound buffer object, and queries of the target to which it is bound return state from
/// the bound buffer object. While buffer object name zero is bound, as in the initial state,
/// attempts to modify or query state on the target to which it is bound generates an
/// [Error::InvalidOperation] error.
///
/// When a non-zero buffer object is bound to the [BufferBindingTarget::Array] target, the vertex
/// array pointer parameter is interpreted as an offset within the buffer object measured in basic
/// machine units.
///
/// When a non-zero buffer object is bound to the [BufferBindingTarget::DrawIndirect] target,
/// parameters for draws issued through [draw_arrays_indirect] and [draw_elements_indirect] are
/// sourced from the specified offset in that buffer object's data store.
///
/// When a non-zero buffer object is bound to the [BufferBindingTarget::DispatchIndirect] target,
/// the parameters for compute dispatches issued through [dispatch_compute_indirect] are sourced
/// from the specified offset in that buffer object's data store.
///
/// While a non-zero buffer object is bound to the [BufferBindingTarget::ElementArray] target, the
/// indices parameter of [draw_elements], [draw_elements_instanced], [draw_elements_base_vertex],
/// [draw_range_elements], [draw_range_elements_base_vertex], [multi_draw_elements], or
/// [multi_draw_elements_base_vertex] is interpreted as an offset within the buffer object measured
/// in basic machine units.
///
/// While a non-zero buffer object is bound to the [BufferBindingTarget::PixelPack] target, the
/// following commands are affected: [get_compressed_tex_image], [get_tex_image], and [read_pixels].
/// The pointer parameter is interpreted as an offset within the buffer object measured in basic
/// machine units.
///
/// While a non-zero buffer object is bound to the [BufferBindingTarget::PixelUnpack] target, the
/// following commands are affected: [compressed_tex_image_1d], [compressed_tex_image_2d],
/// [compressed_tex_image_3d], [compressed_tex_sub_image_1d], [compressed_tex_sub_image_2d],
/// [compressed_tex_sub_image_3d], [tex_image_1d], [tex_image_2d], [tex_image_3d],
/// [tex_sub_image_1d], [tex_sub_image_2d], and [tex_sub_image_3d]. The pointer parameter is
/// interpreted as an offset within the buffer object measured in basic machine units.
///
/// The buffer targets [BufferBindingTarget::CopyRead] and [BufferBindingTarget::CopyWrite] are
/// provided to allow [copy_buffer_sub_data] to be used without disturbing the state of other
/// bindings. However, [copy_buffer_sub_data] may be used with any pair of buffer binding points.
///
/// The [BufferBindingTarget::TransformFeedback] buffer binding point may be passed to
/// [bind_buffer], but will not directly affect transform feedback state. Instead, the indexed
/// [BufferBindingTarget::TransformFeedback] bindings must be used through a call to
/// [bind_buffer_base] or [bind_buffer_range]. This will affect the generic
/// [BufferBindingTarget::TransformFeedback] binding.
///
/// Likewise, the [BufferBindingTarget::Uniform], [BufferBindingTarget::AtomicCounter] and
/// [BufferBindingTarget::ShaderStorage] buffer binding points may be used, but do not directly
/// affect uniform buffer, atomic counter buffer or shader storage buffer state, respectively.
/// [bind_buffer_base] or [bind_buffer_range] must be used to bind a buffer to an indexed uniform
/// buffer, atomic counter buffer or shader storage buffer binding point.
///
/// The [BufferBindingTarget::Query] binding point is used to specify a buffer object that is to
/// receive the results of query objects through calls to the [get_query_object] family of commands.
///
/// A buffer object binding created with [bind_buffer] remains active until a different buffer
/// object name is bound to the same target, or until the bound buffer object is deleted with
/// [delete_buffers].
///
/// Once created, a named buffer object may be re-bound to any target as often as needed. However,
/// the GL implementation may make choices about how to optimize the storage of a buffer object
/// based on its initial binding target.
///
/// # Compatability
/// * 3.1 - [BufferBindingTarget::CopyRead], [BufferBindingTarget::Uniform],
/// [BufferBindingTarget::Texture]
/// * 4.2 - [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidValue] - if buffer is not a name previously returned from a call to
/// [gen_buffers].
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// bind_buffer(BufferBindingTarget::Array, Buffer(0));
/// ```
///
/// # Associated Gets
/// TODO
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [bind_buffer] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [gen_buffers]
/// * [bind_buffer_base]
/// * [bind_buffer_range]
/// * [map_buffer]
/// * [unmap_buffer]
/// * [delete_buffers]
/// * [is_buffer]
pub fn bind_buffer(target: BufferBindingTarget, buffer: Buffer) {
    let target = GLenum::from(target);
    let buffer = buffer.0;

    // SAFE: integers copied by value
    unsafe { gl::BindBuffer(target, buffer) }
}

/// # Error handled [bind_buffer]
pub fn checked_bind_buffer(target: BufferBindingTarget, buffer: Buffer) -> Result<(), BufferError> {
    bind_buffer(target, buffer);
    match get_error() {
        Error::NoError => Ok(()),
        Error::InvalidValue => Err(BufferError::InvalidBuffer(buffer)),
        other => Err(BufferError::Unexpected(other)),
    }
}

/// # Creates and initializes a buffer object's data store
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBufferData.xhtml>
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
/// buffer_data(BufferBindingTarget::Array, &[1, 2, 3], BufferAccessFrequency::Static, BufferAccessNature::Draw);
/// ```
pub fn buffer_data<DataType: Sized>(
    target: BufferBindingTarget,
    data: &[DataType],
    access_frequency: BufferAccessFrequency,
    access_nature: BufferAccessNature,
) {
    let target = GLenum::from(target);
    let size = data.len() as GLsizeiptr;
    let data = data.as_ptr() as *const std::os::raw::c_void;
    let usage = GLenum::from(BufferUsage(access_frequency, access_nature));

    // SAFE: the data memory is synchronously copied into the GL context, never holding onto `data`
    unsafe { gl::BufferData(target, size, data, usage) };
}

pub fn checked_buffer_data<DataType: Sized>(
    target: BufferBindingTarget,
    data: &[DataType],
    access_frequency: BufferAccessFrequency,
    access_nature: BufferAccessNature,
) -> Result<(), BufferError> {
    buffer_data(target, data, access_frequency, access_nature);
    match get_error() {
        Error::NoError => Ok(()),
        Error::InvalidEnum => checked_get_buffer_access(target).map(|_| ()),
        Error::InvalidOperation => Err(BufferError::UnboundTarget(target)),
        other => Err(BufferError::Unexpected(other)),
    }
}

fn get_buffer_parameter_i32(target: BufferBindingTarget, value: GLenum) -> i32 {
    let target = GLenum::from(target);
    let mut param: i32 = 0;
    // SAFE: `param` is an out-param and not retained
    unsafe { gl::GetBufferParameteriv(target, value, &mut param) };
    param
}

/// # Get Buffer Object Access Usage
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGetBufferParameter.xhtml>
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
/// assert_eq!(get_buffer_access(BufferBindingTarget::Array), (BufferAccessFrequency::Static, BufferAccessNature::Draw));
/// ```
pub fn get_buffer_access(
    target: BufferBindingTarget,
) -> Result<(BufferAccessFrequency, BufferAccessNature), BufferError> {
    let param = get_buffer_parameter_i32(target, gl::BUFFER_ACCESS);
    match param as u32 {
        gl::STREAM_DRAW => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::STREAM_READ => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::STREAM_COPY => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::STATIC_DRAW => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::STATIC_READ => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::STATIC_COPY => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::DYNAMIC_DRAW => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::DYNAMIC_READ => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        gl::DYNAMIC_COPY => Ok((BufferAccessFrequency::Stream, BufferAccessNature::Draw)),
        other => Err(BufferError::InvalidParameterValue(other as i64)),
    }
}

pub fn checked_get_buffer_access(
    target: BufferBindingTarget,
) -> Result<(BufferAccessFrequency, BufferAccessNature), BufferError> {
    let buffer_access = get_buffer_access(target)?;
    match get_error() {
        Error::NoError => Ok(buffer_access),
        Error::InvalidOperation => Err(BufferError::UnboundTarget(target)),
        other => Err(BufferError::Unexpected(other)),
    }
}

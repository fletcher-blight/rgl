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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BufferAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

/// # The target to which a buffer object is bound
/// see [bind_buffer]
#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BufferError {
    Unexpected(Error),

    UnboundTarget(BufferBindingTarget),
    InvalidBuffer(Buffer),
    InvalidParameterValue(i64),
    ImmutableBufferTarget(BufferBindingTarget),
}

/// # The frequency of buffer access (modification and usage)
/// see [buffer_data]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BufferUsageFrequency {
    /// The data store contents will be modified once and used at most a few times.
    Stream,

    /// The data store contents will be modified once and used many times.
    Static,

    /// The data store contents will be modified repeatedly and used many times.
    Dynamic,
}

/// # The nature of the buffer access
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BufferUsageNature {
    /// The data store contents are modified by the application, and used as the source for GL drawing and image specification commands.
    Draw,

    /// The data store contents are modified by reading data from the GL, and used to return that data when queried by the application.
    Read,

    /// The data store contents are modified by reading data from the GL, and used as the source for GL drawing and image specification commands.
    Copy,
}

struct BufferUsage(BufferUsageFrequency, BufferUsageNature);
impl From<BufferUsage> for GLenum {
    fn from(BufferUsage(frequency, nature): BufferUsage) -> Self {
        match (frequency, nature) {
            (BufferUsageFrequency::Stream, BufferUsageNature::Draw) => gl::STREAM_DRAW,
            (BufferUsageFrequency::Stream, BufferUsageNature::Read) => gl::STREAM_READ,
            (BufferUsageFrequency::Stream, BufferUsageNature::Copy) => gl::STREAM_COPY,
            (BufferUsageFrequency::Static, BufferUsageNature::Draw) => gl::STATIC_DRAW,
            (BufferUsageFrequency::Static, BufferUsageNature::Read) => gl::STATIC_READ,
            (BufferUsageFrequency::Static, BufferUsageNature::Copy) => gl::STATIC_COPY,
            (BufferUsageFrequency::Dynamic, BufferUsageNature::Draw) => gl::DYNAMIC_DRAW,
            (BufferUsageFrequency::Dynamic, BufferUsageNature::Read) => gl::DYNAMIC_READ,
            (BufferUsageFrequency::Dynamic, BufferUsageNature::Copy) => gl::DYNAMIC_COPY,
        }
    }
}

/// # Buffered data requirements
/// see:
/// * [bind_buffer]
/// * [named_bind_buffer]
pub trait BufferData {
    fn get_size(&self) -> u64;
    fn get_raw_data_pointer(&self) -> *const std::os::raw::c_void;
}

impl<DataType: Sized> BufferData for &[DataType] {
    fn get_size(&self) -> u64 {
        self.len() as u64
    }

    fn get_raw_data_pointer(&self) -> *const std::os::raw::c_void {
        self.as_ptr() as *const std::os::raw::c_void
    }
}

impl<DataType: Sized, const N: usize> BufferData for &[DataType; N] {
    fn get_size(&self) -> u64 {
        self.len() as u64
    }

    fn get_raw_data_pointer(&self) -> *const std::os::raw::c_void {
        self.as_ptr() as *const std::os::raw::c_void
    }
}

impl BufferData for u64 {
    fn get_size(&self) -> u64 {
        *self
    }

    fn get_raw_data_pointer(&self) -> *const std::os::raw::c_void {
        std::ptr::null()
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Buffer(pub u32);

/// # Bind a named buffer object
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBindBuffer.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
/// * `buffer` - Specifies the name of a buffer object
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// bind_buffer(BufferBindingTarget::Array, Buffer(0));
/// ```
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
/// memory buffer with [BufferAccess::ReadWrite] access and ([BufferUsageFrequency::Static],
/// [BufferUsageNature::Draw]) usage.
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

/// # Error mapped bind buffer
/// see [bind_buffer]
pub fn bind_buffer_checked(target: BufferBindingTarget, buffer: Buffer) -> Result<(), BufferError> {
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
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound for [buffer_data]
/// * `data` - Specifies some data that will be copied into the data store for initialization,
/// or a size reservation if no data is to be copied.
/// * `access_frequency`, `access_nature` - Specifies the expected usage pattern of the data store.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// buffer_data(BufferBindingTarget::Array, &[1, 2, 3], BufferUsageFrequency::Static, BufferUsageNature::Draw);
/// ```
///
/// # Description
/// [buffer_data] and [named_buffer_data] create a new data store for a buffer object. In case of
/// [buffer_data], the buffer object currently bound to `target` is used. For [named_buffer_data], a
/// buffer object associated with ID specified by the caller in `buffer` will be used instead.
///
/// While creating the new storage, any pre-existing data store is deleted. The new data store is
/// created with the specified [BufferData::get_size] in bytes and usage. If `data` is a slice, the
/// data store is initialized with data from that view. In its initial state, the new data store is
/// not mapped, it has a NULL mapped pointer, and its mapped access is [BufferAccess::ReadWrite].
///
/// usage ([BufferUsageFrequency], [BufferUsageNature]) is a hint to the GL implementation as to
/// how a buffer object's data store will be accessed. This enables the GL implementation to make
/// more intelligent decisions that may significantly impact buffer object performance. It does not,
/// however, constrain the actual usage of the data store. Usage can be broken down into two parts:
/// first, the frequency of access (modification and usage), and second, the nature of that access.
///
/// If `data` is just a size, a data store of the specified size is still created, but its contents
/// remain uninitialized and thus undefined.
///
/// Clients must align data elements consistently with the requirements of the client platform, with
/// an additional base-level requirement that an offset within a buffer to a datum comprising N
/// bytes be a multiple of N.
///
/// # Compatability
/// * 4.2 - [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidOperation] - by [buffer_data] if the reserved buffer object name 0 is bound to
/// `target`
/// * [Error::InvalidOperation] - by [named_buffer_data] if `buffer` is not the name of an existing
/// buffer object.
/// * [Error::InvalidOperation] - if [get_buffer_immutable_storage] of the buffer object is true.
/// * [Error::OutOfMemory] - if the GL is unable to create a data store with the specified
/// [BufferData::get_size].
///
/// # Associated Gets
/// * [get_buffer_sub_data]
/// * [get_buffer_size], [get_buffer_usage]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [buffer_data] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [named_buffer_data] | N | N | N | N | N | N | N | N | N | N | N | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [buffer_sub_data]
/// * [map_buffer]
/// * [unmap_buffer]
pub fn buffer_data<Data: BufferData>(
    target: BufferBindingTarget,
    data: Data,
    access_frequency: BufferUsageFrequency,
    access_nature: BufferUsageNature,
) {
    let target = GLenum::from(target);
    let size = data.get_size() as GLsizeiptr;
    let data = data.get_raw_data_pointer();
    let usage = GLenum::from(BufferUsage(access_frequency, access_nature));

    // SAFE: the data memory is synchronously copied into the GL context, never holding onto `data`
    unsafe { gl::BufferData(target, size, data, usage) };
}

/// # Error mapped buffer data
/// see [buffer_data]
pub fn buffer_data_checked<Data: BufferData>(
    target: BufferBindingTarget,
    data: Data,
    access_frequency: BufferUsageFrequency,
    access_nature: BufferUsageNature,
) -> Result<(), BufferError> {
    buffer_data(target, data, access_frequency, access_nature);
    match get_error() {
        Error::NoError => Ok(()),
        Error::InvalidEnum => {
            if is_buffer_immutable_storage(target)? {
                Err(BufferError::ImmutableBufferTarget(target))
            } else {
                Err(BufferError::UnboundTarget(target))
            }
        }
        Error::InvalidOperation => Err(BufferError::UnboundTarget(target)),
        other => Err(BufferError::Unexpected(other)),
    }
}

/// # Creates and initializes a buffer object's data store
/// See [buffer_data]
///
/// # Arguments
/// * `buffer` - Specifies the target to which the buffer object is bound for [named_buffer_data]
pub fn named_buffer_data<Data: BufferData>(
    buffer: Buffer,
    data: Data,
    access_frequency: BufferUsageFrequency,
    access_nature: BufferUsageNature,
) {
    let buffer = buffer.0;
    let size = data.get_size() as GLsizeiptr;
    let data = data.get_raw_data_pointer();
    let usage = GLenum::from(BufferUsage(access_frequency, access_nature));

    // SAFE: the data memory is synchronously copied into the GL context, never holding onto `data`
    unsafe { gl::NamedBufferData(buffer, size, data, usage) };
}

/// # Delete named buffer objects
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glDeleteBuffers.xhtml>
///
/// # Arguments
/// * `buffers` - Specifies a slice of buffer objects to be deleted.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// delete_buffers(&[Buffer(42), Buffer(7)]);
/// ```
///
/// # Description
/// [delete_buffers] deletes all buffer objects named by the elements of the slice `buffers`. After
/// a buffer object is deleted, it has no contents, and its name is free for reuse (for example by
/// [gen_buffers]). If a buffer object that is currently bound is deleted, the binding reverts to 0
/// (the absence of any buffer object).
///
/// [delete_buffers] silently ignores 0's and names that do not correspond to existing buffer
/// objects.
///
/// # Associated Gets
/// * [is_buffer]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [delete_buffers] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [gen_buffers]
pub fn delete_buffers(buffers: &[Buffer]) {
    let n = buffers.len() as GLsizei;
    let buffers = buffers.as_ptr() as *const u32;

    // SAFE: synchronously reads `buffers`, where `n` is valid size, and the pointer is not retained
    unsafe { gl::DeleteBuffers(n, buffers) }
}

/// # Generate buffer object names
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGenBuffers.xhtml>
///
/// # Arguments
/// * `buffers` - Specifies a mutable slice in which the generated buffer object names are stored.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// let mut buffer = Default::default();
/// gen_buffers(std::slice::from_mut(&mut buffer));
/// ```
///
/// # Description
/// [gen_buffers] fills buffer object names in `buffers`. There is no guarantee that the names form
/// a contiguous set of integers; however, it is guaranteed that none of the returned names was in
/// use immediately before the call to [gen_buffers].
///
/// Buffer object names returned by a call to [gen_buffers] are not returned by subsequent calls,
/// unless they are first deleted with [delete_buffers].
///
/// No buffer objects are associated with the returned buffer object names until they are first
/// bound by calling [bind_buffer].
///
/// # Associated Gets
/// * [is_buffer]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [my_func] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [delete_buffers]
pub fn gen_buffers(buffers: &mut [Buffer]) {
    let n = buffers.len() as GLsizei;
    let buffers = buffers.as_ptr() as *mut u32;

    // SAFE: synchronously writes to `buffers`, where `n` is valid size, and the pointer is not
    // retained
    unsafe { gl::GenBuffers(n, buffers) };
}

fn get_buffer_parameter_i32(target: BufferBindingTarget, value: GLenum) -> i32 {
    let target = GLenum::from(target);
    let mut param: i32 = 0;
    // SAFE: `param` is an out-param and not retained
    unsafe { gl::GetBufferParameteriv(target, value, &mut param) };
    param
}

fn get_buffer_parameter_i64(target: BufferBindingTarget, value: GLenum) -> i64 {
    let target = GLenum::from(target);
    let mut param: i64 = 0;
    // SAFE: `param` is an out-param and not retained
    unsafe { gl::GetBufferParameteri64v(target, value, &mut param) };
    param
}

/// # Returns the access policy
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGetBufferParameter.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// assert_eq!(
///     get_buffer_access(BufferBindingTarget::ElementArray),
///     Ok(BufferAccess::ReadWrite)
/// );
/// ```
///
/// # Description
/// returns the access policy set while mapping the buffer object (the value of the access parameter
/// enum passed to [map_buffer]). If the buffer was mapped with [map_buffer_range], the access
/// policy is determined by translating the bits in that access parameter to one of the supported
/// enums for [map_buffer] as described in the OpenGL Specification.
///
/// # Compatability
/// * 4.2 - [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidOperation] - if the reserved buffer object name 0 is bound to `target`
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [get_buffer_access] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [buffer_data]
/// * [get_buffer_pointer]
/// * [map_buffer]
/// * [unmap_buffer]
pub fn get_buffer_access(target: BufferBindingTarget) -> Result<BufferAccess, BufferError> {
    let val = get_buffer_parameter_i32(target, gl::BUFFER_ACCESS);
    match val as u32 {
        gl::READ_ONLY => Ok(BufferAccess::ReadOnly),
        gl::WRITE_ONLY => Ok(BufferAccess::WriteOnly),
        gl::READ_WRITE => Ok(BufferAccess::ReadWrite),
        _ => Err(BufferError::InvalidParameterValue(val as i64)),
    }
}

/// # Check if buffer object is immutable
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGetBufferParameter.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// assert!(is_buffer_immutable_storage(BufferBindingTarget::ElementArray).unwrap());
/// ```
///
/// # Description
/// returns a boolean indicating whether the buffer object is immutable. The initial value is false.
///
/// # Compatability
/// * 4.2 - [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidOperation] - if the reserved buffer object name 0 is bound to `target`
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [is_buffer_immutable_storage] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [buffer_data]
/// * [get_buffer_pointer]
/// * [map_buffer]
/// * [unmap_buffer]
pub fn is_buffer_immutable_storage(target: BufferBindingTarget) -> Result<bool, BufferError> {
    let val = get_buffer_parameter_i32(target, gl::BUFFER_IMMUTABLE_STORAGE);
    match val as u8 {
        gl::FALSE => Ok(false),
        gl::TRUE => Ok(true),
        _ => Err(BufferError::InvalidParameterValue(val as i64)),
    }
}

/// # Check if buffer object is mapped
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGetBufferParameter.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// assert!(is_buffer_immutable_storage(BufferBindingTarget::ElementArray).unwrap());
/// ```
///
/// # Description
/// returns a boolean indicating whether the buffer object is mapped. The initial value is false.
///
/// # Compatability
/// * 4.2 - [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidOperation] - if the reserved buffer object name 0 is bound to `target`
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [is_buffer_mapped] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [buffer_data]
/// * [get_buffer_pointer]
/// * [map_buffer]
/// * [unmap_buffer]
pub fn is_buffer_mapped(target: BufferBindingTarget) -> Result<bool, BufferError> {
    let val = get_buffer_parameter_i32(target, gl::BUFFER_MAPPED);
    match val as u8 {
        gl::FALSE => Ok(false),
        gl::TRUE => Ok(true),
        _ => Err(BufferError::InvalidParameterValue(val as i64)),
    }
}

/// # Returns the length of the mapping
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGetBufferParameter.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// assert_eq!(get_buffer_map_length(BufferBindingTarget::ElementArray), 42);
/// ```
///
/// # Description
/// returns the length of the mapping into the buffer object established with [map_buffer]. The
/// initial value is zero.
///
/// # Compatability
/// * 4.2 - [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidOperation] - if the reserved buffer object name 0 is bound to `target`
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [get_buffer_map_length] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [buffer_data]
/// * [get_buffer_pointer]
/// * [map_buffer]
/// * [unmap_buffer]
pub fn get_buffer_map_length(target: BufferBindingTarget) -> u64 {
    let val = get_buffer_parameter_i64(target, gl::BUFFER_MAP_LENGTH);
    val as u64
}

/// # Returns the offset of the mapping
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGetBufferParameter.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// assert_eq!(get_buffer_map_offset(BufferBindingTarget::ElementArray), 42);
/// ```
///
/// # Description
/// returns the offset of the mapping into the buffer object established with [map_buffer]. The
/// initial value is zero.
///
/// # Compatability
/// * 4.2 - [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidOperation] - if the reserved buffer object name 0 is bound to `target`
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [get_buffer_map_offset] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [buffer_data]
/// * [get_buffer_pointer]
/// * [map_buffer]
/// * [unmap_buffer]
pub fn get_buffer_map_offset(target: BufferBindingTarget) -> u64 {
    let val = get_buffer_parameter_i64(target, gl::BUFFER_MAP_OFFSET);
    val as u64
}

/// # Size of buffer
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGetBufferParameter.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// assert_eq!(get_buffer_size(BufferBindingTarget::ElementArray), 42);
/// ```
///
/// # Description
/// returns the size of the buffer object, measured in bytes. The initial value is 0.
///
/// # Compatability
/// * 4.2 - [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidOperation] - if the reserved buffer object name 0 is bound to `target`
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [get_buffer_size] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [buffer_data]
/// * [get_buffer_pointer]
/// * [map_buffer]
/// * [unmap_buffer]
pub fn get_buffer_size(target: BufferBindingTarget) -> u64 {
    let val = get_buffer_parameter_i64(target, gl::BUFFER_SIZE);
    val as u64
}

/// # Returns the buffer object's usage pattern
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGetBufferParameter.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// assert_eq!(
///     get_buffer_usage(BufferBindingTarget::Array),
///     Ok((BufferUsageFrequency::Static, BufferUsageNature::Draw))
/// );
/// ```
///
/// # Description
/// Returns the buffer object's usage pattern. The initial value is ([BufferUsageFrequency::Static],
/// [BufferUsageNature::Draw]).
///
/// # Compatability
/// * 4.2 - [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidOperation] - if the reserved buffer object name 0 is bound to `target`
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [get_buffer_usage] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [buffer_data]
/// * [get_buffer_pointer]
/// * [map_buffer]
/// * [unmap_buffer]
pub fn get_buffer_usage(
    target: BufferBindingTarget,
) -> Result<(BufferUsageFrequency, BufferUsageNature), BufferError> {
    let param = get_buffer_parameter_i32(target, gl::BUFFER_USAGE);
    match param as u32 {
        gl::STREAM_DRAW => Ok((BufferUsageFrequency::Stream, BufferUsageNature::Draw)),
        gl::STREAM_READ => Ok((BufferUsageFrequency::Stream, BufferUsageNature::Draw)),
        gl::STREAM_COPY => Ok((BufferUsageFrequency::Stream, BufferUsageNature::Draw)),
        gl::STATIC_DRAW => Ok((BufferUsageFrequency::Stream, BufferUsageNature::Draw)),
        gl::STATIC_READ => Ok((BufferUsageFrequency::Stream, BufferUsageNature::Draw)),
        gl::STATIC_COPY => Ok((BufferUsageFrequency::Stream, BufferUsageNature::Draw)),
        gl::DYNAMIC_DRAW => Ok((BufferUsageFrequency::Stream, BufferUsageNature::Draw)),
        gl::DYNAMIC_READ => Ok((BufferUsageFrequency::Stream, BufferUsageNature::Draw)),
        gl::DYNAMIC_COPY => Ok((BufferUsageFrequency::Stream, BufferUsageNature::Draw)),
        other => Err(BufferError::InvalidParameterValue(other as i64)),
    }
}

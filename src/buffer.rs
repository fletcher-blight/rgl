//! # Buffer Objects
//! <https://www.khronos.org/opengl/wiki/Category:Core_API_Ref_Buffer_Objects>
//!
//! # Description
//! The core OpenGL API reference for functions that create, manage, and delete
//! [buffer objects](https://www.khronos.org/opengl/wiki/Buffer_Object). These functions are only
//! for getting data into and out of the buffers; the
//! [different uses of buffer objects](https://www.khronos.org/opengl/wiki/Buffer_Object#General_use)
//! are not covered by these functions.

use crate::prelude::*;
use gl::types::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BufferAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl From<BufferAccess> for u32 {
    fn from(value: BufferAccess) -> Self {
        match value {
            BufferAccess::ReadOnly => gl::READ_ONLY,
            BufferAccess::WriteOnly => gl::WRITE_ONLY,
            BufferAccess::ReadWrite => gl::READ_WRITE,
        }
    }
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

/// # The target to which a buffer object is bound for range based bindings
/// see [bind_buffer]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BufferBindingRangeTarget {
    /// Atomic counter storage
    AtomicCounter,

    /// Read-write storage for shaders
    ShaderStorage,

    /// Transform feedback buffer
    TransformFeedback,

    /// Uniform block storage
    Uniform,
}

impl From<BufferBindingRangeTarget> for BufferBindingTarget {
    fn from(value: BufferBindingRangeTarget) -> Self {
        match value {
            BufferBindingRangeTarget::AtomicCounter => BufferBindingTarget::AtomicCounter,
            BufferBindingRangeTarget::ShaderStorage => BufferBindingTarget::ShaderStorage,
            BufferBindingRangeTarget::TransformFeedback => BufferBindingTarget::TransformFeedback,
            BufferBindingRangeTarget::Uniform => BufferBindingTarget::Uniform,
        }
    }
}

impl TryFrom<BufferBindingTarget> for BufferBindingRangeTarget {
    type Error = ();
    fn try_from(value: BufferBindingTarget) -> Result<Self, Self::Error> {
        match value {
            BufferBindingTarget::Array => Err(()),
            BufferBindingTarget::AtomicCounter => Ok(BufferBindingRangeTarget::AtomicCounter),
            BufferBindingTarget::CopyRead => Err(()),
            BufferBindingTarget::CopyWrite => Err(()),
            BufferBindingTarget::DispatchIndirect => Err(()),
            BufferBindingTarget::DrawIndirect => Err(()),
            BufferBindingTarget::ElementArray => Err(()),
            BufferBindingTarget::PixelPack => Err(()),
            BufferBindingTarget::PixelUnpack => Err(()),
            BufferBindingTarget::Query => Err(()),
            BufferBindingTarget::ShaderStorage => Ok(BufferBindingRangeTarget::ShaderStorage),
            BufferBindingTarget::Texture => Err(()),
            BufferBindingTarget::TransformFeedback => {
                Ok(BufferBindingRangeTarget::TransformFeedback)
            }
            BufferBindingTarget::Uniform => Ok(BufferBindingRangeTarget::Uniform),
        }
    }
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

bitflags::bitflags! {
    pub struct BufferMapFlags: u32 {
        /// indicates that the returned pointer may be used to read buffer object data. No GL error
        /// is generated if the pointer is used to query a mapping which excludes this flag, but the
        /// result is undefined and system errors (possibly including program termination) may
        /// occur.
        const READ = gl::MAP_READ_BIT;

        /// indicates that the returned pointer may be used to modify buffer object data. No GL
        /// error is generated if the pointer is used to modify a mapping which excludes this flag,
        /// but the result is undefined and system errors (possibly including program termination)
        /// may occur.
        const WRITE = gl::MAP_WRITE_BIT;

        /// indicates that the mapping is to be made in a persistent fashion and that the client
        /// intends to hold and use the returned pointer during subsequent GL operation. It is not
        /// an error to call drawing commands (render) while buffers are mapped using this flag. It
        /// is an error to specify this flag if the buffer's data store was not allocated through a
        /// call to the [buffer_storage] command in which the [BufferMapFlags::PERSISTENT] was also
        /// set.
        const PERSISTENT = gl::MAP_PERSISTENT_BIT;

        /// indicates that a persistent mapping is also to be coherent. Coherent maps guarantee that
        /// the effect of writes to a buffer's data store by either the client or server will
        /// eventually become visible to the other without further intervention from the
        /// application. In the absence of this bit, persistent mappings are not coherent and
        /// modified ranges of the buffer store must be explicitly communicated to the GL, either by
        /// unmapping the buffer, or through a call to [flush_mapped_buffer_range] or
        /// [memory_barrier].
        const COHERENT = gl::MAP_COHERENT_BIT;

        /// indicates that the previous contents of the specified range may be discarded. Data
        /// within this range are undefined with the exception of subsequently written data. No GL
        /// error is generated if subsequent GL operations access unwritten data, but the result is
        /// undefined and system errors (possibly including program termination) may occur. This
        /// flag may not be used in combination with [BufferMapFlags::READ].
        const INVALIDATE_RANGE = gl::MAP_INVALIDATE_RANGE_BIT;

        /// indicates that the previous contents of the entire buffer may be discarded. Data within
        /// the entire buffer are undefined with the exception of subsequently written data. No GL
        /// error is generated if subsequent GL operations access unwritten data, but the result is
        /// undefined and system errors (possibly including program termination) may occur. This
        /// flag may not be used in combination with [BufferMapFlags::READ].
        const INVALIDATE_BUFFER = gl::MAP_INVALIDATE_BUFFER_BIT;

        ///  indicates that one or more discrete subranges of the mapping may be modified. When this
        /// flag is set, modifications to each subrange must be explicitly flushed by calling
        /// [flush_mapped_buffer_range]. No GL error is set if a subrange of the mapping is modified
        /// and not flushed, but data within the corresponding subrange of the buffer are undefined.
        /// This flag may only be used in conjunction with [BufferMapFlags::WRITE]. When this option
        /// is selected, flushing is strictly limited to regions that are explicitly indicated with
        /// calls to [flush_mapped_buffer_range] prior to unmap; if this option is not selected
        /// [unmap_buffer] will automatically flush the entire mapped range when called.
        const FLUSH_EXPLICIT = gl::MAP_FLUSH_EXPLICIT_BIT;

        /// indicates that the GL should not attempt to synchronize pending operations on the buffer
        /// prior to returning from [map_buffer_range] or [map_named_buffer_range]. No GL error is
        /// generated if pending operations which source or modify the buffer overlap the mapped
        /// region, but the result of such previous and any subsequent operations is undefined.
        const UNSYNCHRONISED = gl::MAP_UNSYNCHRONIZED_BIT;
    }
}

bitflags::bitflags! {
    /// # Storage Bitfield Flags for a Buffer
    /// see [buffer_storage]
    ///
    /// The allowed combinations of flags are subject to certain restrictions. They are as follows:
    /// * If [BufferStorageFlags::PERSISTENT] is set, at least one of [BufferStorageFlags::READ] or
    /// [BufferStorageFlags::WRITE] must be set.
    /// * If [BufferStorageFlags::COHERENT] is set, [BufferStorageFlags::PERSISTENT] must also be
    /// set.
    pub struct BufferStorageFlags: u32 {
        /// The contents of the data store may be updated after creation through calls to
        /// [buffer_sub_data]. If this bit is not set, the buffer content may not be directly
        /// updated by the client. The data argument may be used to specify the initial content of
        /// the buffer's data store regardless of the presence of the [BufferStorageFlags::DYNAMIC].
        /// Regardless of the presence of this bit, buffers may always be updated with server-side
        /// calls such as [copy_buffer_sub_data] and [clear_buffer_sub_data].
        const DYNAMIC = gl::DYNAMIC_STORAGE_BIT;

        /// The data store may be mapped by the client for read access and a pointer in the client's
        /// address space obtained that may be read from.
        const READ = gl::MAP_READ_BIT;

        /// The data store may be mapped by the client for write access and a pointer in the
        /// client's address space obtained that may be written through.
        const WRITE = gl::MAP_WRITE_BIT;

        /// The client may request that the server read from or write to the buffer while it is
        /// mapped. The client's pointer to the data store remains valid so long as the data store
        /// is mapped, even during execution of drawing or dispatch commands.
        const PERSISTENT = gl::MAP_PERSISTENT_BIT;

        /// Shared access to buffers that are simultaneously mapped for client access and are used
        /// by the server will be coherent, so long as that mapping is performed using
        /// [map_buffer_range]. That is, data written to the store by either the client or server
        /// will be immediately visible to the other with no further action taken by the
        /// application. In particular:
        /// * If [BufferStorageFlags::COHERENT] is not set and the client performs a write followed
        /// by a call to the [memory_barrier] command with [MemoryBarrierFlags::CLIENT_MAPPED] set,
        /// then in subsequent commands the server will see the writes.
        /// * If [BufferStorageFlags::COHERENT] is set and the client performs a write, then in
        /// subsequent commands the server will see the writes.
        /// * If [BufferStorageFlags::COHERENT] is not set and the server performs a write, the
        /// application must call [memory_barrier] with [MemoryBarrierFlags::CLIENT_MAPPED] set and
        /// then call [fence_sync] (or [finish]). Then the CPU will see the writes after the sync is
        /// complete.
        /// * If [BufferStorageFlags::COHERENT] is set and the server does a write, the app must
        /// call [fence_sync] (or [finish]). Then the CPU will see the writes after the sync is
        /// complete.
        const COHERENT = gl::MAP_COHERENT_BIT;

        /// When all other criteria for the buffer storage allocation are met, this bit may be used
        /// by an implementation to determine whether to use storage that is local to the server or
        /// to the client to serve as the backing store for the buffer.
        const CLIENT = gl::CLIENT_STORAGE_BIT;
    }
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
/// * [buffer_data]
/// * [named_buffer_data]
pub trait BufferData {
    fn get_size(&self) -> u64;
    fn get_raw_data_pointer(&self) -> *const std::os::raw::c_void;
}

impl<DataType: Sized> BufferData for &[DataType] {
    fn get_size(&self) -> u64 {
        (self.len() * std::mem::size_of::<DataType>()) as u64
    }

    fn get_raw_data_pointer(&self) -> *const std::os::raw::c_void {
        self.as_ptr() as *const std::os::raw::c_void
    }
}

impl<DataType: Sized, const N: usize> BufferData for &[DataType; N] {
    fn get_size(&self) -> u64 {
        (N * std::mem::size_of::<DataType>()) as u64
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
/// [bind_buffer] binds a buffer object to the specified buffer binding point. Calling
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

/// # Bind a buffer object to an indexed buffer target
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBindBufferBase.xhtml>
///
/// # Arguments
/// * `target` - Specify the target of the bind operation.
/// * `index` - Specify the index of the binding point within the array specified by `target`.
/// * `buffer` - The name of a buffer object to bind to the specified binding point.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// bind_buffer_base(BufferBindingRangeTarget::ShaderStorage, 0, Buffer(42));
/// ```
///
/// # Description
/// [bind_buffer_base] binds the buffer object `buffer` to the binding point at `index` index of the
/// array of targets specified by `target`. Each target represents an indexed array of buffer
/// binding points, as well as a single general binding point that can be used by other buffer
/// manipulation functions such as [bind_buffer] or [map_buffer]. In addition to binding `buffer` to
/// the indexed buffer binding target, [bind_buffer_base] also binds `buffer` to the generic buffer
/// binding point specified by `target`.
///
/// Calling [bind_buffer_base] is equivalent to calling [bind_buffer_range] like so:
/// ```no_run
/// # use rgl::prelude::*;
/// fn equivalent_bind_buffer_base(target: BufferBindingRangeTarget, index: u32, buffer: Buffer) {
///     bind_buffer_range(target, index, buffer, 0, get_buffer_size(BufferBindingTarget::from(target)))
/// }
/// ```
///
/// # Compatability
/// * 4.2 - [BufferBindingRangeTarget::AtomicCounter]
/// * 4.3 - [BufferBindingRangeTarget::ShaderStorage]
///
/// # Errors
/// * [Error::InvalidValue] - if `index` is greater than or equal to the number of `target`-specific
/// indexed binding points.
/// * [Error::InvalidValue] - if `buffer` does not have an associated data store, or if the size of
/// that store is zero.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [bind_buffer_base] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [gen_buffers]
/// * [delete_buffers]
/// * [bind_buffer]
/// * [bind_buffer_range]
/// * [map_buffer]
/// * [unmap_buffer]
pub fn bind_buffer_base(target: BufferBindingRangeTarget, index: u32, buffer: Buffer) {
    let target = GLenum::from(BufferBindingTarget::from(target));
    let buffer = buffer.0;

    // SAFE: synchronous integer copy
    unsafe { gl::BindBufferBase(target, index, buffer) }
}

/// # Bind a range within a buffer object to an indexed buffer target
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBindBufferRange.xhtml>
///
/// # Arguments
/// * `target` - Specify the target of the bind operation.
/// * `index` - Specify the index of the binding point within the array specified by `target`.
/// * `buffer` - The name of a buffer object to bind to the specified binding point.
/// * `offset` - The starting offset in basic machine units into the buffer object `buffer`.
/// * `size` - The amount of data in machine units that can be read from the buffer object while
/// used as an indexed target.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// bind_buffer_range(BufferBindingRangeTarget::Uniform, 0, Buffer(42), 7, 69);
/// ```
///
/// # Description
/// [bind_buffer_range] binds a range the buffer object `buffer` represented by `offset` and `size`
/// to the binding point at index `index` of the array of targets specified by `target`. Each target
/// represents an indexed array of buffer binding points, as well as a single general binding point
/// that can be used by other buffer manipulation functions such as [bind_buffer] or [map_buffer].
/// In addition to binding a range of buffer to the indexed buffer binding `target`,
/// [bind_buffer_range] also binds the range to the generic buffer binding point specified by
/// `target`.
///
/// `offset` specifies the offset in basic machine units into the buffer object `buffer` and `size`
/// specifies the amount of data that can be read from the buffer object while used as an indexed
/// target.
///
/// # Compatability
/// * 4.2 - [BufferBindingRangeTarget::AtomicCounter]
/// * 4.3 - [BufferBindingRangeTarget::ShaderStorage]
///
/// # Errors
/// * [Error::InvalidValue] - if `index` is greater than or equal to the number of `target`-specific
/// indexed binding points.
/// * [Error::InvalidValue] - if `size` is less than or equal to zero, or if `offset` + `size` is
/// greater than the value of [get_buffer_size].
/// * Additional errors may be generated if `offset` violates any `target`-specific alignment
/// restrictions.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [bind_buffer_range] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [gen_buffers]
/// * [delete_buffers]
/// * [bind_buffer]
/// * [bind_buffer_base]
/// * [map_buffer]
/// * [unmap_buffer]
pub fn bind_buffer_range(
    target: BufferBindingRangeTarget,
    index: u32,
    buffer: Buffer,
    offset: i64,
    size: u64,
) {
    let target = GLenum::from(BufferBindingTarget::from(target));
    let buffer = buffer.0;
    let offset = offset as GLintptr;
    let size = size as GLsizeiptr;

    // SAFE: synchronous integer copy
    unsafe { gl::BindBufferRange(target, index, buffer, offset, size) }
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
/// buffer_data(BufferBindingTarget::Array, 1024, BufferUsageFrequency::Static, BufferUsageNature::Draw);
///
/// named_buffer_data(Buffer(42), &[1, 2, 3], BufferUsageFrequency::Static, BufferUsageNature::Draw);
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
/// not mapped, it has a null mapped pointer, and its mapped access is [BufferAccess::ReadWrite].
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
/// * [Error::InvalidOperation] - if [is_buffer_immutable_storage] of the buffer object is true.
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

/// # Creates and initializes a buffer object's immutable data store
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBufferStorage.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
/// * `data` - Specifies data that will be copied into the data store for initialization,
/// or a size if no data is to be copied.
/// * `flags` - Specifies the intended usage of the buffer's data store.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// buffer_storage(BufferBindingTarget::Array, &[1, 2, 3], BufferStorageFlags::WRITE);
/// buffer_storage(BufferBindingTarget::Array, 1024, BufferStorageFlags::WRITE | BufferStorageFlags::PERSISTENT);
/// ```
///
/// # Description
/// [buffer_storage] and [named_buffer_storage] create a new immutable data store. For
/// [buffer_storage], the buffer object currently bound to target will be initialized. For
/// [named_buffer_storage], `buffer` is the name of the buffer object that will be configured. The
/// size of the data store is specified by `data`. If an initial data is available, its slice may
/// be supplied in `data`. Otherwise, to create an uninitialized data store, `data` should be a
/// size.
///
/// The `flags` parameters specifies the intended usage of the buffer's data store, see
/// [BufferStorageFlags].
///
/// If `data` is just a size, a data store of the specified size is still created, but its contents
/// remain uninitialized and thus undefined.
///
/// # Errors
/// * [Error::InvalidOperation] - if the reserved buffer object name 0 is bound to `target`.
/// * [Error::OutOfMemory] - if the GL is unable to create a data store with the properties
/// requested in `flags`.
/// * [Error::InvalidValue] - if `flags` is an invalid bitfield (see [BufferStorageFlags])
/// * [Error::InvalidOperation] - if [is_buffer_immutable_storage] of the buffer bound to `target`
/// is true.
///
/// # Associated Gets
/// * [get_buffer_sub_data]
/// * [get_buffer_size], [get_buffer_usage]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [buffer_storage] | N | N | N | N | N | N | N | N | N | N | Y | Y |
/// | [named_buffer_storage] | N | N | N | N | N | N | N | N | N | N | N | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [buffer_sub_data]
/// * [map_buffer]
/// * [unmap_buffer]
pub fn buffer_storage<Data: BufferData>(
    target: BufferBindingTarget,
    data: Data,
    flags: BufferStorageFlags,
) {
    let target = GLenum::from(target);
    let size = data.get_size() as GLsizeiptr;
    let data = data.get_raw_data_pointer();
    let flags = flags.bits;

    // SAFE: the data memory is synchronously copied into the GL context, never holding onto `data`
    unsafe { gl::BufferStorage(target, size, data, flags) }
}

/// # Creates and initializes a buffer object's immutable data store
/// see [buffer_storage]
///
/// # Arguments
/// * `buffer` - Specifies the name of the buffer object
///
/// # Errors
/// * [Error::InvalidOperation] - if `buffer` is not the name of an existing buffer object.
pub fn named_buffer_storage<Data: BufferData>(
    buffer: Buffer,
    data: Data,
    flags: BufferStorageFlags,
) {
    let buffer = buffer.0;
    let size = data.get_size() as GLsizeiptr;
    let data = data.get_raw_data_pointer();
    let flags = flags.bits;

    // SAFE: the data memory is synchronously copied into the GL context, never holding onto `data`
    unsafe { gl::NamedBufferStorage(buffer, size, data, flags) }
}

/// # Updates a subset of a buffer object's data store
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glBufferSubData.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
/// * `offset` - Specifies the offset into the buffer object's data store where data replacement
/// will begin, measured in bytes.
/// * `data` - Specifies a slice to the new data that will be copied into the data store.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// buffer_sub_data(BufferBindingTarget::Array, 42, &[1, 2, 3]);
/// ```
///
/// # Description
/// [buffer_sub_data] and [named_buffer_sub_data] redefine some or all of the data store for the
/// specified buffer object. Data starting at byte offset `offset` and extending for the size of
/// `data` is copied to the data store from the memory pointed to by `data`. `offset` must define a
/// range lying entirely within the buffer object's data store.
///
/// When replacing the entire data store, consider using [buffer_sub_data] rather than completely
/// recreating the data store with [buffer_data]. This avoids the cost of reallocating the data
/// store.
///
/// Consider using multiple buffer objects to avoid stalling the rendering pipeline during data
/// store updates. If any rendering in the pipeline makes reference to data in the buffer object
/// being updated by [buffer_sub_data], especially from the specific region being updated, that
/// rendering must drain from the pipeline before the data store can be updated.
///
/// Clients must align data elements consistent with the requirements of the client platform, with
/// an additional base-level requirement that an offset within a buffer to a datum comprising N
/// bytes be a multiple of N.
///
/// # Compatability
/// * 4.2 - [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidOperation] - if zero is bound to `target`
/// * [Error::InvalidValue] - if `offset`+ size is greater than the value of [get_buffer_size] for
/// the specified buffer object.
/// * [Error::InvalidOperation] - if any part of the specified range of the buffer object is mapped
/// with [map_buffer_range] or [map_buffer], unless it was mapped with the
/// [BufferMapFlags::PERSISTENT] bit set in the [map_buffer_range] access flags.
/// * [Error::InvalidOperation] - if [is_buffer_immutable_storage] and the value of
/// [get_buffer_storage_flags] for the buffer object does not have the [BufferStorageFlags::DYNAMIC]
/// bit set.
///
/// # Associated Gets
/// * [get_buffer_sub_data]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [buffer_sub_data] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [named_buffer_sub_data] | N | N | N | N | N | N | N | N | N | N | N | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [buffer_data]
/// * [map_buffer]
/// * [map_buffer_range]
/// * [unmap_buffer]
pub fn buffer_sub_data<DataType: Sized>(
    target: BufferBindingTarget,
    offset: u64,
    data: &[DataType],
) {
    let target = GLenum::from(target);
    let offset = offset as GLintptr;
    let size = (data.len() * std::mem::size_of::<DataType>()) as GLsizeiptr;
    let data = data.as_ptr() as *const std::os::raw::c_void;

    // SAFE: synchronous read of data, and integer copy, no pointers retained
    unsafe { gl::BufferSubData(target, offset, size, data) }
}

/// # Updates a subset of a buffer object's data store
/// see [buffer_sub_data]
///
/// # Arguments
/// * `buffer` - Specifies the name of the buffer object
///
/// # Errors
/// * [Error::InvalidOperation] - if `buffer` is not the name of an existing buffer object.
pub fn named_buffer_sub_data<DataType: Sized>(buffer: Buffer, offset: u64, data: &[DataType]) {
    let buffer = buffer.0;
    let offset = offset as GLintptr;
    let size = (data.len() * std::mem::size_of::<DataType>()) as GLsizeiptr;
    let data = data.as_ptr() as *const std::os::raw::c_void;

    // SAFE: synchronous read of data, and integer copy, no pointers retained
    unsafe { gl::NamedBufferSubData(buffer, offset, size, data) }
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
/// | [gen_buffers] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
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
/// * [Error::InvalidOperation] - if zero is bound to `target`
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
/// * [Error::InvalidOperation] - if zero is bound to `target`
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
/// * [Error::InvalidOperation] - if zero is bound to `target`
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
/// * [Error::InvalidOperation] - if zero is bound to `target`
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [get_buffer_map_length] | N | N | N | N | Y | Y | Y | Y | Y | Y | Y | Y |
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
/// * [Error::InvalidOperation] - if zero is bound to `target`
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [get_buffer_map_offset] | N | N | N | N | Y | Y | Y | Y | Y | Y | Y | Y |
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
/// * [Error::InvalidOperation] - if zero is bound to `target`
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [get_buffer_size] | N | N | N | N | Y | Y | Y | Y | Y | Y | Y | Y |
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

/// # Returns a bitfield indicating the storage flags
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGetBufferParameter.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// assert_eq!(
///     get_buffer_storage_flags(BufferBindingTarget::Array),
///     Ok(BufferStorageFlags::READ | BufferStorageFlags::WRITE)
/// );
/// ```
///
/// # Description
/// Returns a bitfield indicating the storage flags for the buffer object. If the buffer object is
/// immutable, the value returned will be that specified when the data store was established with
/// [buffer_storage]. If the data store was established with [buffer_data], the value will be
/// [BufferStorageFlags::READ] | [BufferStorageFlags::WRITE] | [BufferStorageFlags::DYNAMIC]. The
/// initial value is zero.
///
/// # Compatability
/// * 4.2 - [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidOperation] - if zero is bound to `target`
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [get_buffer_storage_flags] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [buffer_data]
/// * [get_buffer_pointer]
/// * [map_buffer]
/// * [unmap_buffer]
pub fn get_buffer_storage_flags(
    target: BufferBindingTarget,
) -> Result<BufferStorageFlags, BufferError> {
    let param = get_buffer_parameter_i32(target, gl::BUFFER_USAGE);
    let storage_flags = BufferStorageFlags::from_bits(param as u32);
    storage_flags.ok_or(BufferError::InvalidParameterValue(param as i64))
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
/// * [Error::InvalidOperation] - if zero is bound to `target`
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

/// # Determine if a name corresponds to a buffer object
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glIsBuffer.xhtml>
///
/// # Arguments
/// * `buffer` - Specifies a value that may be the name of a buffer object.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// assert!(is_buffer(Buffer(42)));
/// ```
///
/// # Description
/// [is_buffer] returns true if `buffer` is currently the name of a buffer object. If `buffer` is
/// zero, or is a non-zero value that is not currently the name of a buffer object, or if an error
/// occurs, [is_buffer] returns false.
///
/// A name returned by [gen_buffers], but not yet associated with a buffer object by calling
/// [bind_buffer], is not the name of a buffer object.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [is_buffer] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [delete_buffers]
/// * [gen_buffers]
pub fn is_buffer(buffer: Buffer) -> bool {
    let buffer = buffer.0;
    let val = unsafe { gl::IsBuffer(buffer) };
    val == gl::TRUE
}

/// # Return the pointer to a mapped buffer object's data store
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGetBufferPointerv.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// let data: *const std::os::raw::c_void = get_buffer_pointer(BufferBindingTarget::Array);
/// ```
///
/// # Description
/// [get_buffer_pointer] and [get_named_buffer_pointer] return the buffer pointer. The single buffer
/// map pointer is returned. A null pointer is returned if the buffer object's data store is not
/// currently mapped; or if the requesting context did not map the buffer object's data store, and
/// the implementation is unable to support mappings on multiple clients.
///
/// The initial value for the pointer is null.
///
/// # Compatability
/// * 4.2 - [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidOperation] - if zero is bound to `target`
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [get_buffer_pointer] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [get_named_buffer_pointer] | N | N | N | N | N | N | N | N | N | N | N | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [map_buffer]
pub fn get_buffer_pointer(target: BufferBindingTarget) -> *const std::os::raw::c_void {
    let target = GLenum::from(target);
    let mut params: *mut std::os::raw::c_void = std::ptr::null_mut();
    unsafe {
        gl::GetBufferPointerv(
            target,
            gl::BUFFER_MAP_POINTER,
            &mut params as *const *mut std::os::raw::c_void,
        )
    }
    params
}

/// # Return the pointer to a mapped buffer object's data store
/// see [get_buffer_pointer]
///
/// # Arguments
/// * `buffer` - Specifies the name of the buffer object
///
/// # Errors
/// * [Error::InvalidOperation] - if `buffer` is not the name of an existing buffer object.
pub fn get_named_buffer_pointer(buffer: Buffer) -> *const std::os::raw::c_void {
    let buffer = buffer.0;
    let mut params: *mut std::os::raw::c_void = std::ptr::null_mut();
    unsafe {
        gl::GetNamedBufferPointerv(
            buffer,
            gl::BUFFER_MAP_POINTER,
            &mut params as *const *mut std::os::raw::c_void,
        )
    }
    params
}

/// # Returns a subset of a buffer object's data store
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGetBufferSubData.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
/// * `offset` - Specifies the offset into the buffer object's data store from which data will be
/// returned, measured in bytes.
/// * `data` - Specifies a mut slice to the location where buffer object data is returned.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// let mut buffer = [0; 1024];
/// get_buffer_sub_data(BufferBindingTarget::Array, 0, &mut buffer);
/// ```
///
/// # Description
/// [get_buffer_sub_data] and [get_named_buffer_sub_data] return some or all of the data contents of
/// the data store of the specified buffer object. Data starting at byte offset `offset` and
/// extending for size of `data` bytes is copied from the buffer object's data store to the memory
/// pointed to by `data`. An error is thrown if the buffer object is currently mapped, or if
/// `offset` and size together define a range beyond the bounds of the buffer object's data store.
///
/// If an error is generated, no change is made to the contents of `data`.
///
/// # Compatability
/// * 4.2 - [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidOperation] - if zero is bound to `target`
/// * [Error::InvalidValue] - if `offset`+ `data.len() * std::mem::size_of::<DataType>()` is greater
/// than the value of [get_buffer_size] for the buffer object.
/// * [Error::InvalidOperation] - if the buffer object is mapped with [map_buffer_range] or
/// [map_buffer], unless it was mapped with the [BufferMapFlags::PERSISTENT] bit set in the
/// [map_buffer_range] `access` flags.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [get_buffer_sub_data] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [get_named_buffer_sub_data] | N | N | N | N | N | N | N | N | N | N | N | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [buffer_data]
/// * [buffer_sub_data]
/// * [map_buffer]
/// * [unmap_buffer]
pub fn get_buffer_sub_data<DataType: Sized + Copy>(
    target: BufferBindingTarget,
    offset: u64,
    data: &mut [DataType],
) {
    let target = GLenum::from(target);
    let offset = offset as GLintptr;
    let size = (data.len() * std::mem::size_of::<DataType>()) as GLsizeiptr;
    let data = data.as_mut_ptr() as *mut std::os::raw::c_void;

    // SAFE: TODO, is this data transmute? writing arbitrary bytes into a statically known type
    unsafe { gl::GetBufferSubData(target, offset, size, data) }
}

/// # Returns a subset of a buffer object's data store
/// see [get_buffer_sub_data]
///
/// # Arguments
/// * `buffer` - Specifies the name of the buffer object
///
/// # Errors
/// * [Error::InvalidOperation] - if `buffer` is not the name of an existing buffer object.
pub fn get_named_buffer_sub_data<DataType: Sized + Copy>(
    buffer: Buffer,
    offset: u64,
    data: &mut [DataType],
) {
    let buffer = buffer.0;
    let offset = offset as GLintptr;
    let size = (data.len() * std::mem::size_of::<DataType>()) as GLsizeiptr;
    let data = data.as_mut_ptr() as *mut std::os::raw::c_void;

    // SAFE: TODO, is this data transmute? writing arbitrary bytes into a statically known type
    unsafe { gl::GetNamedBufferSubData(buffer, offset, size, data) }
}

/// # Map all of a buffer object's data store into the client's address space
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glMapBuffer.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
/// * `access` - Specifies the access policy, indicating whether it will be possible to read from,
/// write to, or both read from and write to the buffer object's mapped data store.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// let data: *const std::os::raw::c_void = map_buffer(BufferBindingTarget::Array, BufferAccess::ReadOnly);
/// let data: *const std::os::raw::c_void = map_named_buffer(Buffer(42), BufferAccess::ReadOnly);
/// ```
///
/// # Description
/// [map_buffer] and [map_named_buffer] map the entire data store of a specified buffer object into
/// the client's address space. The data can then be directly read and/or written relative to the
/// returned pointer, depending on the specified access policy.
///
/// A pointer to the beginning of the mapped range is returned once all pending operations on that
/// buffer object have completed, and may be used to modify and/or query the corresponding range of
/// the data store according to the value of access.
///
/// If an error is generated, a null pointer is returned.
///
/// If no error occurs, the returned pointer will reflect an allocation aligned to the value of
/// [get_min_map_buffer_alignment] basic machine units.
///
/// The returned pointer values may not be passed as parameter values to GL commands. For example,
/// they may not be used to specify array pointers, or to specify or query pixel or texture image
/// data; such actions produce undefined results, although implementations may not check for such
/// behavior for performance reasons.
///
/// No GL error is generated if the returned pointer is accessed in a way inconsistent with access
/// (e.g. used to read from a mapping made with access [BufferAccess::WriteOnly] or write to a
/// mapping made with access [BufferAccess::ReadOnly]), but the result is undefined and system
/// errors (possibly including program termination) may occur.
///
/// Mappings to the data stores of buffer objects may have nonstandard performance characteristics.
/// For example, such mappings may be marked as uncacheable regions of memory, and in such cases
/// reading from them may be very slow. To ensure optimal performance, the client should use the
/// mapping in a fashion consistent with the values of [get_buffer_usage] for the buffer object and
/// of access. Using a mapping in a fashion inconsistent with these values is liable to be multiple
/// orders of magnitude slower than using normal memory.
///
/// # Compatability
/// * 4.2 - Alignment of the returned pointer is guaranteed, and
/// [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidOperation] - if zero is bound to `target`
///
/// # Associated Gets
/// * [get_buffer_pointer]
/// * [is_buffer_mapped], [get_buffer_access], [get_buffer_usage]
/// * [get_min_map_buffer_alignment]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [map_buffer] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [map_named_buffer] | N | N | N | N | N | N | N | N | N | N | N | Y |
///
/// # See Also
/// * [bind_buffer]
/// * [bind_buffer_base]
/// * [bind_buffer_range]
/// * [buffer_data]
/// * [buffer_sub_data]
/// * [delete_buffers]
/// * [map_buffer_range]
/// * [unmap_buffer]
pub fn map_buffer(
    target: BufferBindingTarget,
    access: BufferAccess,
) -> *const std::os::raw::c_void {
    let target = GLenum::from(target);
    let access = GLenum::from(access);

    // SAFE: synchronous integer copy
    unsafe { gl::MapBuffer(target, access) }
}

/// # Map all of a buffer object's data store into the client's address space
/// see [map_buffer]
///
/// # Arguments
/// * `buffer` - Specifies the name of the buffer object.
pub fn map_named_buffer(buffer: Buffer, access: BufferAccess) -> *const std::os::raw::c_void {
    let buffer = buffer.0;
    let access = GLenum::from(access);

    // SAFE: synchronous integer copy
    unsafe { gl::MapNamedBuffer(buffer, access) }
}

/// # Map all or part of a buffer object's data store into the client's address space
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glMapBufferRange.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
/// * `offset` - Specifies the starting offset within the buffer of the range to be mapped.
/// * `length` - Specifies the length of the range to be mapped.
/// * `access` - Specifies a combination of access flags indicating the desired access to the mapped
/// range.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// let data: *const std::os::raw::c_void = map_buffer_range(BufferBindingTarget::Array, 0, 42, BufferMapFlags::READ);
/// ```
///
/// # Description
/// [map_buffer_range] and [map_named_buffer_range] map all or part of the data store of a specified
/// buffer object into the client's address space. `offset` and `length` indicate the range of data
/// in the buffer object that is to be mapped, in terms of basic machine units. `access` is a
/// [bitfield](BufferMapFlags) containing flags which describe the requested mapping.
///
/// A pointer to the beginning of the mapped range is returned once all pending operations on the
/// buffer object have completed, and may be used to modify and/or query the corresponding range of
/// the data store according to the flag bits set in `access`.
///
/// If an error occurs, a null pointer is returned.
///
/// If no error occurs, the returned pointer will reflect an allocation aligned to the value of
/// [get_min_map_buffer_alignment] basic machine units. Subtracting `offset` from this returned
/// pointer will always produce a multiple of the value of [get_min_map_buffer_alignment].
///
/// The returned pointer values may not be passed as parameter values to GL commands. For example,
/// they may not be used to specify array pointers, or to specify or query pixel or texture image
/// data; such actions produce undefined results, although implementations may not check for such
/// behavior for performance reasons.
///
/// Mappings to the data stores of buffer objects may have nonstandard performance characteristics.
/// For example, such mappings may be marked as uncacheable regions of memory, and in such cases
/// reading from them may be very slow. To ensure optimal performance, the client should use the
/// mapping in a fashion consistent with the values of GL_BUFFER_USAGE for the buffer object and of
/// access. Using a mapping in a fashion inconsistent with these values is liable to be multiple
/// orders of magnitude slower than using normal memory.
///
/// No error is generated if memory outside the mapped range is modified or queried, but the result
/// is undefined and system errors (possibly including program termination) may occur.
///
/// # Compatability
/// * 4.2 - Alignment of the returned pointer is guaranteed, and
/// [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidOperation] - if zero is bound to `target`
/// * [Error::InvalidValue] - if `offset`+`length` is greater than the value of [get_buffer_size]
/// for the buffer object, or if access has any bits set other than those defined above.
/// * [Error::InvalidOperation] - is generated for any of the following conditions:
///     * `length` is zero
///     * the buffer object is already in a mapped state
///     * Neither [BufferMapFlags::READ] nor [BufferMapFlags::WRITE] is set.
///     * [BufferMapFlags::READ] is set and any of [BufferMapFlags::INVALIDATE_RANGE],
/// [BufferMapFlags::INVALIDATE_BUFFER] or [BufferMapFlags::UNSYNCHRONISED] is set.
///     * [BufferMapFlags::FLUSH_EXPLICIT] is set and [BufferMapFlags::WRITE] is not set.
///     * Any of [BufferMapFlags::READ], [BufferMapFlags::WRITE], [BufferMapFlags::PERSISTENT], or
/// [BufferMapFlags::COHERENT] are set, but the same bit is not included in the buffer's storage
/// flags.
///
/// # Associated Gets
/// * [get_min_map_buffer_alignment] - The value must be a power of two that is at least 64.
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [map_buffer_range] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [map_named_buffer_range] | N | N | N | N | N | N | N | N | N | N | N | Y |
///
/// # See Also
pub fn map_buffer_range(
    target: BufferBindingTarget,
    offset: u64,
    length: u64,
    access: BufferMapFlags,
) -> *const std::os::raw::c_void {
    let target = GLenum::from(target);
    let offset = offset as GLintptr;
    let length = length as GLsizeiptr;
    let access = access.bits;

    // SAFE: synchronous integer copy
    unsafe { gl::MapBufferRange(target, offset, length, access) }
}

/// # Map all or part of a buffer object's data store into the client's address space
/// see [map_buffer_range]
///
/// # Arguments
/// * `buffer` - Specifies the name of the buffer object
///
/// # Errors
/// * [Error::InvalidOperation] - if `buffer` is not the name of an existing buffer object
pub fn map_named_buffer_range(
    buffer: Buffer,
    offset: u64,
    length: u64,
    access: BufferMapFlags,
) -> *const std::os::raw::c_void {
    let buffer = buffer.0;
    let offset = offset as GLintptr;
    let length = length as GLsizeiptr;
    let access = access.bits;

    // SAFE: synchronous integer copy
    unsafe { gl::MapNamedBufferRange(buffer, offset, length, access) }
}

/// # Release the mapping of a buffer object's data store into the client's address space
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glUnmapBuffer.xhtml>
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// assert!(unmap_buffer(BufferBindingTarget::ElementArray));
/// assert!(unmap_named_buffer(Buffer(42)));
/// ```
///
/// # Description
/// [unmap_buffer] and [unmap_named_buffer] unmap (release) any mapping of a specified buffer object
/// into the client's address space (see [map_buffer_range] and [map_buffer]).
///
/// If a mapping is not unmapped before the corresponding buffer object's data store is used by the
/// GL, an error will be generated by any GL command that attempts to dereference the buffer
/// object's data store, unless the buffer was successfully mapped with [BufferMapFlags::PERSISTENT]
/// (see [map_buffer_range]). When a data store is unmapped, the mapped pointer becomes invalid.
///
/// [unmap_buffer] returns true unless the data store contents have become corrupt during the time
/// the data store was mapped. This can occur for system-specific reasons that affect the
/// availability of graphics memory, such as screen mode changes. In such situations, false is
/// returned and the data store contents are undefined. An application must detect this rare
/// condition and reinitialize the data store.
///
/// A buffer object's mapped data store is automatically unmapped when the buffer object is deleted
/// or its data store is recreated with [buffer_data].
///
/// If an error is generated, [unmap_buffer] returns false.
///
/// # Compatability
/// * 4.2 - [BufferBindingTarget::AtomicCounter]
/// * 4.3 - [BufferBindingTarget::DispatchIndirect], [BufferBindingTarget::ShaderStorage]
/// * 4.4 - [BufferBindingTarget::Query]
///
/// # Errors
/// * [Error::InvalidOperation] - if zero is bound to `target`
/// * [Error::InvalidOperation] - if the buffer object is not in a mapped state.
///
/// # Associated Gets
/// * [is_buffer_mapped]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [unmap_buffer] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [unmap_named_buffer] | N | N | N | N | N | N | N | N | N | N | N | Y |
///
/// # See Also
pub fn unmap_buffer(target: BufferBindingTarget) -> bool {
    let target = GLenum::from(target);

    // SAFE: synchronous integer copy
    let val = unsafe { gl::UnmapBuffer(target) };
    val == gl::TRUE
}

/// # Release the mapping of a buffer object's data store into the client's address space
/// see [unmap_buffer]
///
/// # Arguments
/// * `buffer` - Specifies the name of the buffer object.
pub fn unmap_named_buffer(buffer: Buffer) -> bool {
    let buffer = buffer.0;

    // SAFE: synchronous integer copy
    let val = unsafe { gl::UnmapNamedBuffer(buffer) };
    val == gl::TRUE
}

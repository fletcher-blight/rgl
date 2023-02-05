use gl::types::*;

/// Buffer Name Object
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Buffer(GLuint);

/// Vertex Array Name Object
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct VertexArray(GLuint);

/// Buffer Name Target Type
#[derive(Debug, Clone, Copy)]
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
    TransformFeedbackBuffer,

    /// Uniform block storage
    Uniform,
}

impl From<BufferBindingTarget> for GLenum {
    fn from(value: BufferBindingTarget) -> Self {
        match value {
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
            BufferBindingTarget::TransformFeedbackBuffer => gl::TRANSFORM_FEEDBACK_BUFFER,
            BufferBindingTarget::Uniform => gl::UNIFORM_BUFFER,
        }
    }
}

/// Primitive Render Draw Type
#[derive(Debug, Clone, Copy)]
pub enum DrawMode {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdjacency,
    LinesAdjacency,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdjacency,
    TrianglesAdjacency,
    Patches,
}

impl From<DrawMode> for GLenum {
    fn from(value: DrawMode) -> Self {
        match value {
            DrawMode::Points => gl::POINTS,
            DrawMode::LineStrip => gl::LINE_STRIP,
            DrawMode::LineLoop => gl::LINE_LOOP,
            DrawMode::Lines => gl::LINES,
            DrawMode::LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
            DrawMode::LinesAdjacency => gl::LINES_ADJACENCY,
            DrawMode::TriangleStrip => gl::TRIANGLE_STRIP,
            DrawMode::TriangleFan => gl::TRIANGLE_FAN,
            DrawMode::Triangles => gl::TRIANGLES,
            DrawMode::TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
            DrawMode::TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
            DrawMode::Patches => gl::PATCHES,
        }
    }
}

/// Index Buffer Integer Type
#[derive(Debug, Clone, Copy)]
pub enum IndicesType {
    U8,
    U16,
    U32,
}

impl From<IndicesType> for GLenum {
    fn from(value: IndicesType) -> Self {
        match value {
            IndicesType::U8 => gl::UNSIGNED_BYTE,
            IndicesType::U16 => gl::UNSIGNED_SHORT,
            IndicesType::U32 => gl::UNSIGNED_INT,
        }
    }
}

/// Index Buffer Data Type Map
pub trait Indices<IndexType> {
    fn indices_type() -> IndicesType;
}

impl Indices<u32> for u32 {
    fn indices_type() -> IndicesType {
        IndicesType::U32
    }
}

/// Possible errors of [bind_buffer]
#[derive(Debug, Clone, Copy)]
pub enum ErrorBindBuffer {
    /// `buffer` is not a name previously returned from a call to [gen_buffers]
    InvalidBuffer(Buffer),
}

/// bind a named buffer object
///
/// Binds a buffer object to the specified buffer binding point. Calling [bind_buffer] with `buffer`
/// set to the name of a buffer object binds that buffer object name to the `target`. If no buffer
/// object with name `buffer` exists, one is created with that name. When a buffer object is bound
/// to a target, the previous binding for that target is automatically broken.
///
/// Buffer object names are unsigned integers. The value zero is reserved, but there is no default
/// buffer object for each buffer object target. Instead, `buffer` set to zero effectively unbinds
/// any buffer object previously bound, and restores client memory usage for that buffer object target
/// (if supported for that target). Buffer object names and the corresponding buffer object contents
/// are local to the shared object space of the current GL rendering context;
/// two rendering contexts share buffer object names only if they explicitly enable sharing between
/// contexts through the appropriate GL windows interfaces functions.
///
/// [gen_buffers] must be used to generate a set of unused buffer object names.
///
/// The state of a buffer object immediately after it is first bound is an unmapped zero-sized memory
/// buffer with [ReadWrite] access and [StaticDraw] usage.
///
/// While a non-zero buffer object name is bound, GL operations on the target to which it is bound
/// affect the bound buffer object, and queries of the target to which it is bound return state from
/// the bound buffer object. While buffer object name zero is bound, as in the initial state,
/// attempts to modify or query state on the target to which it is bound generates an
/// [InvalidOperation](ErrorOpenGL::InvalidOperation) error.
///
/// A buffer object binding created with [bind_buffer] remains active until a different buffer
/// object name is bound to the same target, or until the bound buffer object is deleted with
/// [delete_buffers].
///
/// Once created, a named buffer object may be re-bound to any target as often as needed. However,
/// the GL implementation may make choices about how to optimize the storage of a buffer object based
/// on its initial binding target.
///
/// # Arguments
/// * `target` - Specifies the target to which the buffer object is bound
/// * `buffer` - Specifies the name of a buffer object
///
/// # Target Usage
/// - When a non-zero buffer object is bound to the [Array](BufferBindingTarget::Array) target,
/// the vertex array pointer parameter is interpreted as an offset within the buffer object measured
/// in basic machine units.
/// - While a non-zero buffer object is bound to the [ElementArray](BufferBindingTarget::ElementArray)
/// target, the indices parameter of [draw_elements], [draw_elements_instanced], [draw_elements_base_vertex],
/// [draw_range_elements], [draw_range_elements_base_vertex], [multi_draw_elements], or
/// [multi_draw_elements_base_vertex] is interpreted as an offset within the buffer object measured
/// in basic machine units.
///
/// # Compatability
/// - 3.1 or greater is required for: [CopyRead](BufferBindingTarget::CopyRead),
/// [Uniform](BufferBindingTarget::Uniform), [Texture](BufferBindingTarget::Texture)
/// - 4.2 or greater is required for: [AtomicCounter](BufferBindingTarget::AtomicCounter)
/// - 4.3 or greater is required for: [DispatchIndirect](BufferBindingTarget::DispatchIndirect) and
/// [ShaderStorage](BufferBindingTarget::ShaderStorage)
/// - 4.4 or greater is required for: [Query](BufferBindingTarget::Query)
pub fn bind_buffer(target: BufferBindingTarget, buffer: Buffer) -> Result<(), ErrorBindBuffer> {
    let target: GLenum = target.into();
    unsafe { gl::BindBuffer(target, buffer.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(ErrorBindBuffer::InvalidBuffer(buffer)),
        _ => unreachable!(),
    }
}

/// Possible errors of [bind_vertex_array]
#[derive(Debug, Clone, Copy)]
pub enum ErrorBindVertexArray {
    /// `array` is not zero or the name of a vertex array object previously returned from a call to [gen_vertex_arrays]
    InvalidArray(VertexArray),
}

/// bind a vertex array object
///
/// Binds the vertex array object with name `array`. `array` is the name of a vertex array object
/// previously returned from a call to [gen_vertex_arrays], or zero to break the existing
/// vertex array object binding.
///
/// If no vertex array object with name `array` exists, one is created when array is first bound.
/// If the bind is successful no change is made to the state of the vertex array object,
/// and any previous vertex array object binding is broken.
///
/// # Arguments
/// * `array` - Specifies the name of the vertex array to bind
pub fn bind_vertex_array(array: VertexArray) -> Result<(), ErrorBindVertexArray> {
    unsafe { gl::BindVertexArray(array.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => Err(ErrorBindVertexArray::InvalidArray(array)),
        _ => unreachable!(),
    }
}

/// delete named buffer objects
///
/// glDeleteBuffers deletes all buffer objects named by `buffers`. After a buffer object is deleted,
/// it has no contents, and its name is free for reuse (for example by [gen_buffers]). If a buffer
/// object that is currently bound is deleted, the binding reverts to 0 (the absence of any buffer object).
///
/// [delete_buffers] silently ignores 0's and names that do not correspond to existing buffer objects.
///
/// # Arguments
/// * `buffers` - Specifies an array of buffer objects to be deleted
pub fn delete_buffers(buffers: &[Buffer]) -> () {
    let n = buffers.len() as GLsizei;
    let buffers = buffers.as_ptr() as *const GLuint;
    unsafe { gl::DeleteBuffers(n, buffers) }
}

/// Possible errors of [draw_elements]
#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// a geometry shader is active and `mode` is incompatible with the input primitive type of the
    /// geometry shader in the currently installed program object
    IncompatibleGeometryShaderInputMode,

    BufferObjectBoundToEnabledArray,

    ElementArrayAndBufferObjectDataCurrentlyMapped,
}

/// render primitives from array data
///
/// [draw_elements] specifies multiple geometric primitives with very few subroutine calls.
/// Instead of calling a GL function to pass each individual vertex, normal, texture coordinate,
/// edge flag, or color, you can pre-specify separate arrays of vertices, normals, and so on,
/// and use them to construct a sequence of primitives with a single call to [draw_elements].
///
/// When [draw_elements] is called, it uses `count` sequential elements from an enabled array,
/// starting at `offset` to construct a sequence of geometric primitives. `mode` specifies what kind
/// of primitives are constructed and how the array elements construct these primitives.
/// If more than one array is enabled, each is used.
///
/// Vertex attributes that are modified by [draw_elements] have an unspecified value after
/// [draw_elements] returns. Attributes that aren't modified maintain their previous values.
///
/// # Arguments
/// * `mode` - Specifies what kind of primitives to render
/// * `count` - Specifies the number of elements to be rendered
/// * `indices_type` - Specifies the type of the values in the element array buffer
/// * `offset` - Specifies the starting offset from the enabled array
///
/// # Compatability
/// - 3.2 or greater is required for: [LineStripAdjacency](DrawMode::LineStripAdjacency),
/// [LinesAdjacency](DrawMode::LinesAdjacency), [TriangleStripAdjacency](DrawMode::TriangleStripAdjacency)
/// and [TrianglesAdjacency](DrawMode::TrianglesAdjacency)
pub fn draw_elements(
    mode: DrawMode,
    count: usize,
    indices_type: IndicesType,
    offset: usize,
) -> Result<(), Error> {
    let mode: GLenum = mode.into();
    let count = count as GLsizei;
    let type_: GLenum = indices_type.into();
    let indices = offset as *const std::os::raw::c_void;
    unsafe { gl::DrawElements(mode, count, type_, indices) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => todo!(),
        _ => unreachable!(),
    }
}

/// generate buffer object names
///
/// [gen_buffers] fills all buffer object names in `buffers`. There is no guarantee that the names
/// form a contiguous set of integers; however, it is guaranteed that none of the returned names
/// was in use immediately before the call to [gen_buffers].
///
/// Buffer object names returned by a call to [gen_buffers] are not returned by subsequent calls,
/// unless they are first deleted with [delete_buffers].
///
/// No buffer objects are associated with the returned buffer object names until they are first
/// bound by calling [bind_buffer].
///
/// # Arguments
/// * `buffers` - Specifies an array in which the generated buffer object names are stored
pub fn gen_buffers(buffers: &mut [Buffer]) -> () {
    unsafe {
        gl::GenBuffers(
            buffers.len() as GLsizei,
            buffers.as_mut_ptr() as *mut GLuint,
        )
    }
}

/// Currently defined OpenGL errors
#[derive(Debug)]
pub enum ErrorOpenGL {
    /// No error has been recorded
    NoError,

    /// An unacceptable value is specified for an enumerated argument. The offending command is
    /// ignored and has no other side effect than to set the error flag
    InvalidEnum,

    /// A numeric argument is out of range. The offending command is ignored and has no other
    /// side effect than to set the error flag
    InvalidValue,

    /// The specified operation is not allowed in the current state. The offending command is
    /// ignored and has no other side effect than to set the error flag
    InvalidOperation,

    /// The framebuffer object is not complete. The offending command is ignored and has no other
    /// side effect than to set the error flag
    InvalidFrameBufferOperation,

    /// There is not enough memory left to execute the command. The state of the GL is undefined,
    /// except for the state of the error flags, after this error is recorded
    OutOfMemory,

    /// An attempt has been made to perform an operation that would cause an internal stack to underflow
    StackUnderflow,

    /// An attempt has been made to perform an operation that would cause an internal stack to overflow
    StackOverflow,

    /// GL returned a non-standard error code
    Unknown(u32),
}

/// return error information
///
/// Returns the value of the error flag. Each detectable error is assigned a numeric code and symbolic name.
/// When an error occurs, the error flag is set to the appropriate error code value. No other errors
/// are recorded until [get_error] is called, the error code is returned, and the flag is reset
/// to [NoError](ErrorOpenGL::NoError). If a call to [get_error] returns
/// [NoError](ErrorOpenGL::NoError), there has been no detectable error since the last call to [get_error],
/// or since the GL was initialized.
///
/// To allow for distributed implementations, there may be several error flags. If any single
/// error flag has recorded an error, the value of that flag is returned and that flag is reset to
/// [NoError](ErrorOpenGL::NoError) when [get_error] is called. If more than one flag has recorded
/// an error, [get_error] returns and clears an arbitrary error flag value. Thus, [get_error] should
/// always be called in a loop, until it returns [NoError](ErrorOpenGL::NoError),
/// if all error flags are to be reset.
///
/// When an error flag is set, results of a GL operation are undefined only if
/// [OutOfMemory](ErrorOpenGL::OutOfMemory) has occurred. In all other cases, the command generating
/// the error is ignored and has no effect on the GL state or frame buffer contents. If the generating
/// command returns a value, it returns 0. If [get_error] itself generates an error, it returns 0.
pub fn get_error() -> ErrorOpenGL {
    match unsafe { gl::GetError() } {
        gl::NO_ERROR => ErrorOpenGL::NoError,
        gl::INVALID_ENUM => ErrorOpenGL::InvalidEnum,
        gl::INVALID_VALUE => ErrorOpenGL::InvalidValue,
        gl::INVALID_OPERATION => ErrorOpenGL::InvalidOperation,
        gl::INVALID_FRAMEBUFFER_OPERATION => ErrorOpenGL::InvalidFrameBufferOperation,
        gl::OUT_OF_MEMORY => ErrorOpenGL::OutOfMemory,
        gl::STACK_UNDERFLOW => ErrorOpenGL::StackUnderflow,
        gl::STACK_OVERFLOW => ErrorOpenGL::StackUnderflow,
        unknown => ErrorOpenGL::Unknown(unknown),
    }
}

fn internal_get_error() -> ErrorOpenGL {
    // TODO: create a feature control so this will always return ErrorOpenGL::NoError if per-function error checking is disabled
    get_error()
}

pub use gl::load_with;

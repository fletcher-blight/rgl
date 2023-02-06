use gl::types::*;

/// Buffer Name Object
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Buffer(GLuint);

/// Framebuffer Name Object
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Framebuffer(GLuint);

/// Program Object
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Program(GLuint);

/// Shader Object
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Shader(GLuint);

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

/// Bitmask for [clear] to specify the desired buffer(s) to clear  
#[bitmask_enum::bitmask(u32)]
pub enum ClearMask {
    /// Indicates the buffers currently enabled for color writing
    Colour = gl::COLOR_BUFFER_BIT,
    /// Indicates the depth buffer
    Depth = gl::DEPTH_BUFFER_BIT,
    /// Indicates the stencil buffer
    Stencil = gl::STENCIL_BUFFER_BIT,
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

/// Framebuffer Name Target Type
#[derive(Debug, Clone, Copy)]
pub enum FramebufferBindingTarget {
    Draw,
    Read,
    ReadDraw,
}

impl From<FramebufferBindingTarget> for GLenum {
    fn from(value: FramebufferBindingTarget) -> Self {
        match value {
            FramebufferBindingTarget::Draw => gl::DRAW_FRAMEBUFFER,
            FramebufferBindingTarget::Read => gl::READ_FRAMEBUFFER,
            FramebufferBindingTarget::ReadDraw => gl::FRAMEBUFFER,
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

/// Shader Object Types
#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    /// intended to run on the programmable compute processor
    Compute,
    /// intended to run on the programmable vertex processor
    Vertex,
    /// intended to run on the programmable tessellation processor in the control stage
    TessControl,
    /// intended to run on the programmable tessellation processor in the evaluation stage
    TessEvaluation,
    /// intended to run on the programmable geometry processor
    Geometry,
    /// intended to run on the programmable fragment processor
    Fragment,
}

impl From<ShaderType> for GLenum {
    fn from(value: ShaderType) -> Self {
        match value {
            ShaderType::Compute => gl::COMPUTE_SHADER,
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::TessControl => gl::TESS_CONTROL_SHADER,
            ShaderType::TessEvaluation => gl::TESS_EVALUATION_SHADER,
            ShaderType::Geometry => gl::GEOMETRY_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

/// Attaches a shader object to a program object
///
/// In order to create a complete shader program, there must be a way to specify the list of things
/// that will be linked together. Program objects provide this mechanism. Shaders that are to be
/// linked together in a program object must first be attached to that program object. [attach_shader]
/// attaches the shader object specified by shader to the program object specified by program.
/// This indicates that `shader` will be included in link operations that will be performed on `program`.
///
/// All operations that can be performed on a shader object are valid whether or not the shader
/// object is attached to a program object. It is permissible to attach a shader object to a program
/// object before source code has been loaded into the shader object or before the shader object has
/// been compiled. It is permissible to attach multiple shader objects of the same type because each
/// may contain a portion of the complete shader. It is also permissible to attach a shader object
/// to more than one program object. If a shader object is deleted while it is attached to a program
/// object, it will be flagged for deletion, and deletion will not occur until [detach_shader] is
/// called to detach it from all program objects to which it is attached.
///
/// # Arguments
/// * `program` - Specifies the program object to which a shader object will be attached
/// * `shader` - Specifies the shader object that is to be attached
pub fn attach_shader(program: Program, shader: Shader) -> Result<(), ErrorAttachShader> {
    unsafe { gl::AttachShader(program.0, shader.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(ErrorAttachShader::NonOpenGLName(program, shader)),
        ErrorOpenGL::InvalidOperation => {
            if !is_program(program) {
                Err(ErrorAttachShader::NotAProgram(program))
            } else if !is_shader(shader) {
                Err(ErrorAttachShader::NotAShader(shader))
            } else {
                Err(ErrorAttachShader::ShaderAlreadyAttachedToProgram(
                    program, shader,
                ))
            }
        }
        _ => unreachable!(),
    }
}

/// Possible errors of [attach_shader]
#[derive(Debug, Clone, Copy)]
pub enum ErrorAttachShader {
    NonOpenGLName(Program, Shader),
    NotAProgram(Program),
    NotAShader(Shader),
    ShaderAlreadyAttachedToProgram(Program, Shader),
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

/// Possible errors of [bind_buffer]
#[derive(Debug, Clone, Copy)]
pub enum ErrorBindBuffer {
    /// `buffer` is not a name previously returned from a call to [gen_buffers]
    InvalidBuffer(Buffer),
}

/// bind a framebuffer to a framebuffer target
///
/// [bind_framebuffer] binds the framebuffer object with name `framebuffer` to the framebuffer target
/// specified by `target`. If a framebuffer object is bound to [Draw](FramebufferBindingTarget::Draw)
/// or [Read](FramebufferBindingTarget::Read), it becomes the target for rendering or readback
/// operations, respectively, until it is deleted or another framebuffer is bound to the corresponding
/// bind point. Calling [bind_framebuffer] with target set to [ReadDraw](FramebufferBindingTarget::ReadDraw)
/// binds `framebuffer` to both the read and draw framebuffer targets. `framebuffer` is the name of
/// a framebuffer object previously returned from a call to [gen_frame_buffers], or zero to break the
/// existing binding of a framebuffer object to target.
///
/// # Arguments
/// * `target` - Specifies the framebuffer target of the binding operation
/// * `framebuffer` - Specifies the name of the framebuffer object to bind
pub fn bind_framebuffer(
    target: FramebufferBindingTarget,
    framebuffer: Framebuffer,
) -> Result<(), ErrorBindFramebuffer> {
    let target: GLenum = target.into();
    unsafe { gl::BindFramebuffer(target, framebuffer.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => Err(ErrorBindFramebuffer::InvalidFramebuffer(framebuffer)),
        _ => unreachable!(),
    }
}

/// Possible errors of [bind_framebuffer]
#[derive(Debug, Clone, Copy)]
pub enum ErrorBindFramebuffer {
    /// `framebuffer` is not zero or the name of a framebuffer previously returned from a call to [gen_frame_buffers]
    InvalidFramebuffer(Framebuffer),
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

/// Possible errors of [bind_vertex_array]
#[derive(Debug, Clone, Copy)]
pub enum ErrorBindVertexArray {
    /// `array` is not zero or the name of a vertex array object previously returned from a call to [gen_vertex_arrays]
    InvalidArray(VertexArray),
}

/// clear buffers to preset values
///
/// [clear] sets the bitplane area of the window to values previously selected by
/// [Colour](ClearMask::Colour), [Depth](ClearMask::Depth), and [Stencil](ClearMask::Stencil).
/// Multiple colour buffers can be cleared simultaneously by selecting more than one buffer at a time
/// using [draw_buffer].
///
/// The pixel ownership test, the scissor test, dithering, and the buffer writemasks affect the
/// operation of [clear]. The scissor box bounds the cleared region. Alpha function, blend function,
/// logical operation, stenciling, texture mapping, and depth-buffering are ignored by [clear].
///
/// [clear] takes a single argument that is the bitwise OR of several values indicating which
/// buffer is to be cleared.
///
/// The value to which each buffer is cleared depends on the setting of the clear value for that buffer.
pub fn clear(mask: ClearMask) -> () {
    unsafe { gl::Clear(mask.into()) }
}

/// specify clear values for the color buffers
///
/// [clear_colour] specifies the `red`, `green`, `blue`, and `alpha` values used by [clear] to clear
/// the color buffers. Values specified by [clear_colour] are clamped to the range \[0,1\].
pub fn clear_colour(red: f32, green: f32, blue: f32, alpha: f32) -> () {
    unsafe { gl::ClearColor(red, green, blue, alpha) }
}

/// Creates a program object
///
/// [create_program] creates an empty program object and returns a non-zero value by which it can
/// be referenced. A program object is an object to which shader objects can be attached.
/// This provides a mechanism to specify the shader objects that will be linked to create a program.
/// It also provides a means for checking the compatibility of the shaders that will be used to
/// create a program (for instance, checking the compatibility between a vertex shader
/// and a fragment shader). When no longer needed as part of a program object,
/// shader objects can be detached.
///
/// One or more executables are created in a program object by successfully attaching shader objects
/// to it with [attach_shader], successfully compiling the shader objects with [compile_shader],
/// and successfully linking the program object with [link_program]. These executables are made part
/// of current state when [use_program] is called. Program objects can be deleted by calling [delete_program].
/// The memory associated with the program object will be deleted when it is no longer part of
/// current rendering state for any context.
///
/// # Notes
/// Like buffer and texture objects, the name space for program objects may be shared across a set
/// of contexts, as long as the server sides of the contexts share the same address space. If the
/// name space is shared across contexts, any attached objects and the data associated with those
/// attached objects are shared as well.
///
/// Applications are responsible for providing the synchronization across API calls when objects
/// are accessed from different execution threads.
///
/// # Errors
/// This function returns 0 if an error occurs creating the program object.
pub fn create_program() -> Program {
    let id = unsafe { gl::CreateProgram() };
    Program(id)
}

/// Creates a shader object
///
/// [create_shader] creates an empty shader object and returns a non-zero value by which it can
/// be referenced. A shader object is used to maintain the source code strings that define a shader.
/// `shader_type` indicates the type of shader to be created.
///
/// # Notes
/// - Like buffer and texture objects, the name space for shader objects may be shared across a set
/// of contexts, as long as the server sides of the contexts share the same address space.
/// If the name space is shared across contexts, any attached objects and the data associated with
/// those attached objects are shared as well.
///
/// - Applications are responsible for providing the synchronization across API calls when objects
/// are accessed from different execution threads.
pub fn create_shader(shader_type: ShaderType) -> Shader {
    let type_: GLuint = shader_type.into();
    let id = unsafe { gl::CreateShader(type_) };
    Shader(id)
}

/// delete named buffer objects
///
/// [delete_buffers] deletes all buffer objects named by `buffers`. After a buffer object is deleted,
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

/// delete framebuffer objects
///
/// [delete_frame_buffers] deletes all framebuffer objects whose names are stored in the array
/// addressed by `frame_buffers`. The name zero is reserved by the GL and is silently ignored,
/// should it occur in `frame_buffers`, as are other unused names. Once a framebuffer object is deleted,
/// its name is again unused and it has no attachments. If a framebuffer that is currently bound to
/// one or more of the targets [Draw](FramebufferBindingTarget::Draw) or [Read](FramebufferBindingTarget::Read)
/// is deleted, it is as though [bind_framebuffer] had been executed with the corresponding target
/// and framebuffer zero.
///
/// # Arguments
/// * `frame_buffers` - Specifies an array of framebuffer objects to be deleted
pub fn delete_frame_buffers(frame_buffers: &[Framebuffer]) -> () {
    let n = frame_buffers.len() as GLsizei;
    let framebuffers = frame_buffers.as_ptr() as *const GLuint;
    unsafe { gl::DeleteFramebuffers(n, framebuffers) }
}

/// delete vertex array objects
///
/// [delete_vertex_arrays] deletes all vertex array objects whose names are stored in the array
/// addressed by `arrays`. Once a vertex array object is deleted it has no contents and its name is
/// again unused. If a vertex array object that is currently bound is deleted, the binding for that
/// object reverts to zero and the default vertex array becomes current. Unused names in `arrays`
/// are silently ignored, as is the value zero.
///
/// # Arguments
/// * `arrays` - Specifies an array of vertex array objects to be deleted
pub fn delete_vertex_arrays(arrays: &[VertexArray]) -> () {
    let n = arrays.len() as GLsizei;
    let arrays = arrays.as_ptr() as *const GLuint;
    unsafe { gl::DeleteVertexArrays(n, arrays) }
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
) -> Result<(), ErrorDrawElements> {
    let mode: GLenum = mode.into();
    let count = count as GLsizei;
    let type_: GLenum = indices_type.into();
    let indices = offset as *const std::os::raw::c_void;
    unsafe { gl::DrawElements(mode, count, type_, indices) };
    internal_handle_draw_elements_error()
}

fn internal_handle_draw_elements_error() -> Result<(), ErrorDrawElements> {
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => todo!(),
        _ => unreachable!(),
    }
}

/// Possible errors of [draw_elements] and variants
#[derive(Debug, Clone, Copy)]
pub enum ErrorDrawElements {
    /// a geometry shader is active and `mode` is incompatible with the input primitive type of the
    /// geometry shader in the currently installed program object
    IncompatibleGeometryShaderInputMode,

    BufferObjectBoundToEnabledArray,

    ElementArrayAndBufferObjectDataCurrentlyMapped,
}

/// draw multiple instances of a set of elements
///
/// [draw_elements_instanced] behaves identically to [draw_elements] except that `instance_count`
/// instances of the set of elements are executed and the value of the internal counter `instanceID`
/// advances for each iteration. `instanceID` is an internal 32-bit integer counter that may be
/// read by a vertex shader as `gl_InstanceID`.
///
/// [draw_elements_instanced] has the same effect as:
/// ```
/// # use rgl::*;
/// # fn draw_elements_instanced(
/// #     mode: DrawMode,
/// #     count: usize,
/// #     indices_type: IndicesType,
/// #     offset: usize,
/// #     instance_count: usize) -> Result<(), ErrorDrawElements> {
/// for instance_id in 0..instance_count {
///     draw_elements(mode, count, indices_type, offset)?;
/// }
/// #    Ok(())
/// # }
/// ```
///
/// # Arguments
/// * `mode` - Specifies what kind of primitives to render
/// * `count` - Specifies the number of elements to be rendered
/// * `indices_type` - Specifies the type of the values in the element array buffer
/// * `offset` - Specifies the starting offset from the enabled array
/// * `instance_count` - Specifies the number of instances of the specified range of indices to be rendered
///
/// # Compatability
/// - 3.1 or greater is required for [draw_elements_instanced]
/// - 3.2 or greater is required for: [LineStripAdjacency](DrawMode::LineStripAdjacency),
/// [LinesAdjacency](DrawMode::LinesAdjacency), [TriangleStripAdjacency](DrawMode::TriangleStripAdjacency)
/// and [TrianglesAdjacency](DrawMode::TrianglesAdjacency)
pub fn draw_elements_instanced(
    mode: DrawMode,
    count: usize,
    indices_type: IndicesType,
    offset: usize,
    instance_count: usize,
) -> Result<(), ErrorDrawElements> {
    let mode: GLenum = mode.into();
    let count = count as GLsizei;
    let type_: GLenum = indices_type.into();
    let indices = offset as *const std::os::raw::c_void;
    let instancecount = instance_count as GLsizei;
    unsafe { gl::DrawElementsInstanced(mode, count, type_, indices, instancecount) };
    internal_handle_draw_elements_error()
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
    let n = buffers.len() as GLsizei;
    let buffers = buffers.as_mut_ptr() as *mut GLuint;
    unsafe { gl::GenBuffers(n, buffers) }
}

/// generate framebuffer object names
///
/// [gen_frame_buffers] fills all framebuffer object names in `ids`. There is no guarantee that the
/// names form a contiguous set of integers; however, it is guaranteed that none of the returned
/// names was in use immediately before the call to [gen_frame_buffers].
///
/// Framebuffer object names returned by a call to [gen_frame_buffers] are not returned by subsequent
/// calls, unless they are first deleted with [delete_frame_buffers].
///
/// The names returned in ids are marked as used, for the purposes of [gen_frame_buffers] only,
/// but they acquire state and type only when they are first bound.
///
/// # Arguments
/// * `ids` - Specifies an array in which the generated framebuffer object names are storeda
pub fn gen_frame_buffers(ids: &mut [Framebuffer]) -> () {
    let n = ids.len() as GLsizei;
    let ids = ids.as_mut_ptr() as *mut GLuint;
    unsafe { gl::GenFramebuffers(n, ids) }
}

/// generate vertex array object names
///
/// [gen_vertex_arrays] fills all vertex array object names in `arrays`. There is no guarantee that
/// the names form a contiguous set of integers; however, it is guaranteed that none of the returned
/// names was in use immediately before the call to [gen_vertex_arrays].
///
/// Vertex array object names returned by a call to [gen_vertex_arrays] are not returned by subsequent
/// calls, unless they are first deleted with [delete_vertex_arrays].
///
/// The names returned in `arrays` are marked as used, for the purposes of [gen_vertex_arrays] only,
/// but they acquire state and type only when they are first bound.
///
/// # Arguments
/// * `arrays` - Specifies an array in which the generated vertex array object names are stored
pub fn gen_vertex_arrays(arrays: &mut [VertexArray]) -> () {
    let n = arrays.len() as GLsizei;
    let arrays = arrays.as_mut_ptr() as *mut GLuint;
    unsafe { gl::GenVertexArrays(n, arrays) }
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

/// Determines if a name corresponds to a program object
///
/// [is_program] returns true if `program` is the name of a program object previously created with
/// [create_program] and not yet deleted with [delete_program]. If `program` is zero or a non-zero
/// value that is not the name of a program object, or if an error occurs, [is_program] returns false.
///
/// # Arguments
/// * `program` - Specifies a potential program object
///
/// # Notes
/// - No error is generated if program is not a valid program object name.
/// - A program object marked for deletion with [delete_program] but still in use as part of current
/// rendering state is still considered a program object and [is_program] will return true.
pub fn is_program(program: Program) -> bool {
    (unsafe { gl::IsProgram(program.0) }) == gl::TRUE
}

/// Determines if a name corresponds to a shader object
///
/// [is_shader] returns true if `shader` is the name of a shader object previously created with
/// [create_shader] and not yet deleted with [delete_shader]. If `shader` is zero or a non-zero
/// value that is not the name of a shader object, or if an error occurs, [is_shader] returns false.
///
/// # Arguments
/// * `shader` - Specifies a potential shader object
///
/// # Notes
/// - No error is generated if shader is not a valid shader object name.
/// - A shader object marked for deletion with [delete_shader] but still attached to a program object
/// is still considered a shader object and [is_shader] will return true.
pub fn is_shader(shader: Shader) -> bool {
    (unsafe { gl::IsShader(shader.0) }) == gl::TRUE
}

fn internal_get_error() -> ErrorOpenGL {
    // TODO: create a feature control so this will always return ErrorOpenGL::NoError if per-function error checking is disabled
    get_error()
}

pub use gl::load_with;

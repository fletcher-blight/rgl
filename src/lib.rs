use gl::types::*;

pub use gl::load_with;

pub const MAX_VERTEX_ATTRIBUTES: u32 = gl::MAX_VERTEX_ATTRIBS as u32;
pub const MAX_DRAW_BUFFERS: u32 = gl::MAX_DRAW_BUFFERS as u32;
pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: u32 = gl::MAX_DUAL_SOURCE_DRAW_BUFFERS as u32;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: u32 =
    gl::MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS as u32;

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

/// Location of a specific uniform variable within a program
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct UniformLocation(GLint);

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

impl TryFrom<GLenum> for DrawMode {
    type Error = ();
    fn try_from(value: GLenum) -> Result<Self, Self::Error> {
        match value {
            gl::POINTS => Ok(DrawMode::Points),
            gl::LINE_STRIP => Ok(DrawMode::LineStrip),
            gl::LINE_LOOP => Ok(DrawMode::LineLoop),
            gl::LINES => Ok(DrawMode::Lines),
            gl::LINE_STRIP_ADJACENCY => Ok(DrawMode::LineStripAdjacency),
            gl::LINES_ADJACENCY => Ok(DrawMode::LinesAdjacency),
            gl::TRIANGLE_STRIP => Ok(DrawMode::TriangleStrip),
            gl::TRIANGLE_FAN => Ok(DrawMode::TriangleFan),
            gl::TRIANGLES => Ok(DrawMode::Triangles),
            gl::TRIANGLE_STRIP_ADJACENCY => Ok(DrawMode::TriangleStripAdjacency),
            gl::TRIANGLES_ADJACENCY => Ok(DrawMode::TrianglesAdjacency),
            gl::PATCHES => Ok(DrawMode::Patches),
            _ => Err(()),
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

impl TryFrom<GLenum> for ShaderType {
    type Error = ();
    fn try_from(value: GLenum) -> Result<Self, Self::Error> {
        match value {
            gl::COMPUTE_SHADER => Ok(ShaderType::Compute),
            gl::VERTEX_SHADER => Ok(ShaderType::Vertex),
            gl::TESS_CONTROL_SHADER => Ok(ShaderType::TessControl),
            gl::TESS_EVALUATION_SHADER => Ok(ShaderType::TessEvaluation),
            gl::GEOMETRY_SHADER => Ok(ShaderType::Geometry),
            gl::FRAGMENT_SHADER => Ok(ShaderType::Fragment),
            _ => Err(()),
        }
    }
}

/// Transform Feedback Buffer Capturing Mode
#[derive(Debug, Clone, Copy)]
pub enum TransformFeedbackBufferMode {
    Interleaved,
    Separate,
}

impl From<TransformFeedbackBufferMode> for GLenum {
    fn from(value: TransformFeedbackBufferMode) -> Self {
        match value {
            TransformFeedbackBufferMode::Interleaved => gl::INTERLEAVED_ATTRIBS,
            TransformFeedbackBufferMode::Separate => gl::SEPARATE_ATTRIBS,
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
    internal_handle_attach_shader_error(program, shader)
}

fn internal_handle_attach_shader_error(
    program: Program,
    shader: Shader,
) -> Result<(), ErrorAttachShader> {
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

/// Compiles a shader object
///
/// [compile_shader] compiles the source code strings that have been stored in the shader object
/// specified by `shader`.
///
/// The compilation status will be stored as part of the shader object's state. This value will be
/// set to true if the shader was compiled without errors and is ready for use, and false otherwise.
/// It can be queried by calling [get_shader::compile_status].
///
/// Compilation of a shader can fail for a number of reasons as specified by the OpenGL Shading
/// Language Specification. Whether or not the compilation was successful, information about the
/// compilation can be obtained from the shader object's information log by calling [get_shader_info_log].
pub fn compile_shader(shader: Shader) -> Result<(), ErrorCompileShader> {
    unsafe { gl::CompileShader(shader.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(ErrorCompileShader::NonOpenGLName(shader)),
        ErrorOpenGL::InvalidOperation => Err(ErrorCompileShader::NotAShaderObject(shader)),
        _ => unreachable!(),
    }
}

/// Possible errors of [compile_shader]
#[derive(Debug, Clone, Copy)]
pub enum ErrorCompileShader {
    NonOpenGLName(Shader),
    NotAShaderObject(Shader),
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

/// Deletes a program object
///
/// [delete_program] frees the memory and invalidates the name associated with the program object
/// specified by program. This command effectively undoes the effects of a call to [create_program].
///
/// If a program object is in use as part of current rendering state, it will be flagged for deletion,
/// but it will not be deleted until it is no longer part of current state for any rendering context.
/// If a program object to be deleted has shader objects attached to it, those shader objects will
/// be automatically detached but not deleted unless they have already been flagged for deletion by
/// a previous call to [delete_shader]. A value of 0 for program will be silently ignored.
///
/// To determine whether a program object has been flagged for deletion, call
/// [get_program]\([ProgramParam::DeleteStatus]\).
pub fn delete_program(program: Program) -> Result<(), ErrorDeleteProgram> {
    unsafe { gl::DeleteProgram(program.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(ErrorDeleteProgram::NonOpenGLName(program)),
        _ => unreachable!(),
    }
}

/// Possible errors of [delete_program]
#[derive(Debug, Clone, Copy)]
pub enum ErrorDeleteProgram {
    NonOpenGLName(Program),
}

/// Deletes a shader object
///
/// [delete_shader] frees the memory and invalidates the name associated with the shader object
/// specified by shader. This command effectively undoes the effects of a call to [create_shader].
///
/// If a shader object to be deleted is attached to a program object, it will be flagged for deletion,
/// but it will not be deleted until it is no longer attached to any program object, for any rendering
/// context (i.e., it must be detached from wherever it was attached before it will be deleted).
/// A value of 0 for shader will be silently ignored.
///
/// To determine whether an object has been flagged for deletion, call [get_shader::delete_status].
pub fn delete_shader(shader: Shader) -> Result<(), ErrorDeleteShader> {
    unsafe { gl::DeleteShader(shader.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(ErrorDeleteShader::NonOpenGLName(shader)),
        _ => unreachable!(),
    }
}

/// Possible errors of [delete_shader]
#[derive(Debug, Clone, Copy)]
pub enum ErrorDeleteShader {
    NonOpenGLName(Shader),
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

/// Detaches a shader object from a program object to which it is attached
///
/// [detach_shader] detaches the shader object specified by `shader` from the program object specified
/// by `program`. This command can be used to undo the effect of the command [attach_shader].
///
/// If `shader` has already been flagged for deletion by a call to [delete_shader] and it is not
/// attached to any other program object, it will be deleted after it has been detached.
pub fn detach_shader(program: Program, shader: Shader) -> Result<(), ErrorAttachShader> {
    unsafe { gl::DetachShader(program.0, shader.0) };
    internal_handle_attach_shader_error(program, shader)
}

/// render primitives from array data
///
/// [draw_arrays] specifies multiple geometric primitives with very few subroutine calls. Instead of
/// calling a GL procedure to pass each individual vertex, normal, texture coordinate, edge flag,
/// or color, you can prespecify separate arrays of vertices, normals, and colors and use them to
/// construct a sequence of primitives with a single call to [draw_arrays].
///
/// When [draw_arrays] is called, it uses `count` sequential elements from each enabled array to
/// construct a sequence of geometric primitives, beginning with element `first`. `mode` specifies
/// what kind of primitives are constructed and how the array elements construct those primitives.
///
/// Vertex attributes that are modified by [draw_arrays] have an unspecified value after
/// [draw_arrays] returns. Attributes that aren't modified remain well defined.
///
/// # Arguments
/// * `mode` - Specifies what kind of primitives to render
/// * `first` - Specifies the starting index in the enabled arrays
/// * `count` - Specifies the number of indices to be rendered
///
/// # Notes
/// - 3.2 or greater is required for: [LineStripAdjacency](DrawMode::LineStripAdjacency),
/// [LinesAdjacency](DrawMode::LinesAdjacency), [TriangleStripAdjacency](DrawMode::TriangleStripAdjacency)
/// and [TrianglesAdjacency](DrawMode::TrianglesAdjacency)
pub fn draw_arrays(mode: DrawMode, first: u32, count: u32) -> Result<(), ErrorDrawArrays> {
    let mode: GLenum = mode.into();
    let first = first as GLint;
    let count = count as GLsizei;
    unsafe { gl::DrawArrays(mode, first, count) };
    internal_handle_draw_arrays_error()
}

fn internal_handle_draw_arrays_error() -> Result<(), ErrorDrawArrays> {
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidOperation => todo!(),
        _ => unreachable!(),
    }
}

/// Possible errors of [draw_arrays] and variants
#[derive(Debug, Clone, Copy)]
pub enum ErrorDrawArrays {
    /// a non-zero buffer object name is bound to an enabled array and the buffer object's
    /// data store is currently mapped
    BufferObjectDataCurrentlyMapped,
    /// a geometry shader is active and `mode` is incompatible with the input primitive type of the
    /// geometry shader in the currently installed program object
    IncompatibleGeometryShaderInputMode,
}

/// draw multiple instances of a range of elements
///
/// [draw_arrays_instanced] behaves identically to [draw_arrays] except that `instance_count` instances
/// of the range of elements are executed and the value of the internal counter `instanceID` advances
/// for each iteration. `instanceID` is an internal 32-bit integer counter that may be read by a
/// vertex shader as `gl_InstanceID`.
///
/// [draw_arrays_instanced] has the same effect as:
/// ```
/// # use rgl::*;
/// # fn draw_arrays_instanced(
/// #     mode: DrawMode,
/// #     first: u32,
/// #     count: u32,
/// #     instance_count: u32) -> Result<(), ErrorDrawArrays> {
/// for instance_id in 0..instance_count {
///     draw_arrays(mode, first, count)?;
/// }
/// #    Ok(())
/// # }
/// ```
pub fn draw_arrays_instanced(
    mode: DrawMode,
    first: u32,
    count: u32,
    instance_count: u32,
) -> Result<(), ErrorDrawArrays> {
    let mode: GLenum = mode.into();
    let first = first as GLint;
    let count = count as GLsizei;
    let instancecount = instance_count as GLsizei;
    unsafe { gl::DrawArraysInstanced(mode, first, count, instancecount) };
    internal_handle_draw_arrays_error()
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
    count: u32,
    indices_type: IndicesType,
    offset: u32,
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
/// #     count: u32,
/// #     indices_type: IndicesType,
/// #     offset: u32,
/// #     instance_count: u32) -> Result<(), ErrorDrawElements> {
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
    count: u32,
    indices_type: IndicesType,
    offset: u32,
    instance_count: u32,
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

fn glint_as_u32(i: GLint) -> u32 {
    i as u32
}

fn glint_as_draw_mode(i: GLint) -> DrawMode {
    (i as GLenum).try_into().expect(&format!(
        "Internal OpenGL Failure, invalid DrawMode value: {i}"
    ))
}

pub trait GLenumParamConstantType<GLType> {
    type DesiredType;
    fn convert(gl_value: GLType) -> Self::DesiredType;
    fn gl_enum_value() -> GLenum;
}

struct GLenumParamConstant<DesiredType, const GL_VALUE: GLenum> {
    _unused: std::marker::PhantomData<DesiredType>,
}

impl<const GL_VALUE: GLenum> GLenumParamConstantType<GLint>
    for GLenumParamConstant<bool, GL_VALUE>
{
    type DesiredType = bool;
    fn convert(gl_value: GLint) -> Self::DesiredType {
        match gl_value as GLboolean {
            gl::TRUE => true,
            gl::FALSE => false,
            _ => unreachable!(),
        }
    }
    fn gl_enum_value() -> GLenum {
        GL_VALUE
    }
}

macro_rules! create_params {
    (
        $(#[$group_name_docs:meta])*
        pub struct $group_name:ident {
            $(
                $(#[$param_name_docs:meta])*
                const $name:ident: $T:ty;
            )+
        }
    ) => {
        $(#[$group_name_docs])*
        pub struct $group_name {}
        impl $group_name {
            $(
                $(#[$param_name_docs])*
                pub const $name: GLenumParamConstant<$T, { gl::$gl_name }> = GLenumParamConstant::<$T, { gl::$name }>{ _unused: std::marker::PhantomData::<$T>{} };
            )+
        }
    };
}

create_params!(
pub struct ProgramParam {
    /// returns true if program is currently flagged for deletion, and false otherwise.
    const DeleteStatus: bool = DELETE_STATUS;

    /// returns true if the last link operation on program was successful, and false otherwise.
    const LinkStatus: bool = LINK_STATUS;

    /// returns true or if the last validation operation on program was successful, and false otherwise.
    const ValidateStatus: bool = VALIDATE_STATUS;

    /// returns the number of characters in the information log for program including the null
    /// termination character (i.e., the size of the character buffer required to store the
    /// information log). If program has no information log, a value of 0 is returned.
    const InfoLogLength: u32 = INFO_LOG_LENGTH;

    /// returns the number of shader objects attached to program.
    const AttachedShaders: u32 = ATTACHED_SHADERS;

    /// returns the number of active attribute atomic counter buffers used by program.
    const ActiveAtomicCounterBuffers: u32 = ACTIVE_ATOMIC_COUNTER_BUFFERS;

    /// returns the number of active attribute variables for program.
    const ActiveAttributes: u32 = ACTIVE_ATTRIBUTES;

    /// returns the length of the longest active attribute name for program, including the null
    /// termination character (i.e., the size of the character buffer required to store the longest
    /// attribute name). If no active attributes exist, 0 is returned.
    const ActiveAttributeMaxLength: u32 = ACTIVE_ATTRIBUTE_MAX_LENGTH;

    /// returns the number of active uniform variables for program.
    const ActiveUniforms: bool = ACTIVE_UNIFORMS;

    /// returns the length of the longest active uniform variable name for program, including the
    /// null termination character (i.e., the size of the character buffer required to store the
    /// longest uniform variable name). If no active uniform variables exist, 0 is returned.
    const ActiveUniformMaxLength: bool = ACTIVE_UNIFORM_MAX_LENGTH;

    /// returns the number of active uniform blocks for program.
    const ActiveUniformBlocks: bool = ACTIVE_UNIFORM_BLOCKS;

    /// returns the length of the longest active uniform block name for program, including the
    /// null termination character (i.e., the size of the character buffer required to store the
    /// longest uniform block name). If no active uniform block exist, 0 is returned.
    const ActiveUniformBlockMaxNameLength: bool = ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH;

    /// returns the length of the program binary, in bytes that will be returned by a call to
    /// [get_program_binary](super::get_program_binary). When a [get_program]\([LinkStatus]\) is false,
    /// its program binary length is zero.
    const ProgramBinaryLength: bool = PROGRAM_BINARY_LENGTH;

    /// returns an array of three integers containing the local work group size of the compute
    /// program as specified by its input layout qualifier(s). program must be the name of a program
    /// object that has been previously linked successfully and contains a binary for the
    /// compute shader stage.
    const ComputeWorkGroupSize: bool = COMPUTE_WORK_GROUP_SIZE;

    /// returns a symbolic constant indicating the buffer mode used when transform feedback is active.
    const TransformFeedbackBufferMode: bool = TRANSFORM_FEEDBACK_BUFFER_MODE;

    /// returns the number of varying variables to capture in transform feedback mode for the program.
    const TransformFeedbackVaryings: bool = TRANSFORM_FEEDBACK_VARYINGS;

    /// returns the length of the longest variable name to be used for transform feedback,
    /// including the null-terminator.
    const TransformFeedbackVaryingMaxLength: bool = TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH;

    /// returns the maximum number of vertices that the geometry shader in program will output.
    const GeometryVerticesOut: bool = GEOMETRY_VERTICES_OUT;

    /// returns the primitive draw mode type accepted as input to the geometry shader contained in program.
    const GeometryInputType: bool = GEOMETRY_INPUT_TYPE;

    /// returns the primitive draw mode type that will be output by the geometry shader contained in program.
    const GeometryOutputType: bool = GEOMETRY_OUTPUT_TYPE;
});

/// Returns a parameter from a program object
///
/// # Notes
/// - 3.1 or greater is required for: [ActiveUniformBlocks](ProgramParam::ActiveUniformBlocks)
/// and [ActiveUniformBlockMaxNameLength](ProgramParam::ActiveUniformBlockMaxNameLength)
/// - 3.2 or greater is required for: [GeometryVerticesOut](ProgramParam::GeometryVerticesOut)
/// [GeometryInputType](ProgramParam::GeometryInputType) and
/// [GeometryOutputType](ProgramParam::GeometryOutputType)
/// - 4.3 or greater is required for: [ComputeWorkGroupSize](ProgramParam::ComputeWorkGroupSize)
pub fn get_program<Param>(
    program: Program,
    _param: Param,
) -> Result<Param::DesiredType, ErrorGetProgram>
where
    Param: GLenumParamConstantType<GLint>,
{
    let mut params: GLint = 0;
    let pname: GLenum = Param::gl_enum_value();
    unsafe { gl::GetProgramiv(program.0, pname, &mut params) };

    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(Param::convert(params)),
        ErrorOpenGL::InvalidValue => Err(ErrorGetProgram::NonOpenGLName(program)),
        ErrorOpenGL::InvalidOperation => {
            if !is_program(program) {
                Err(ErrorGetProgram::NotAProgram(program))
            } else if pname == gl::COMPUTE_WORK_GROUP_SIZE {
                Err(ErrorGetProgram::ComputeQueryWithoutComputeShader(program))
            } else {
                Err(ErrorGetProgram::GeometryQueryWithoutGeometryShader(program))
            }
        }
        _ => unreachable!(),
    }
}

/// Possible errors of [program] queries
#[derive(Debug, Clone, Copy)]
pub enum ErrorGetProgram {
    NonOpenGLName(Program),
    NotAProgram(Program),
    GeometryQueryWithoutGeometryShader(Program),
    ComputeQueryWithoutComputeShader(Program),
}

pub mod program {
    use super::*;

    fn glint(program: Program, pname: GLenum) -> Result<GLint, ErrorGetProgram> {
        let mut params: GLint = 0;
        unsafe { gl::GetProgramiv(program.0, pname, &mut params) };
        match internal_get_error() {
            ErrorOpenGL::NoError => Ok(params.into()),
            ErrorOpenGL::InvalidValue => Err(ErrorGetProgram::NonOpenGLName(program)),
            ErrorOpenGL::InvalidOperation => {
                if !is_program(program) {
                    Err(ErrorGetProgram::NotAProgram(program))
                } else if pname == gl::COMPUTE_WORK_GROUP_SIZE {
                    Err(ErrorGetProgram::ComputeQueryWithoutComputeShader(program))
                } else {
                    Err(ErrorGetProgram::GeometryQueryWithoutGeometryShader(program))
                }
            }
            _ => unreachable!(),
        }
    }

    fn as_bool(i: GLint) -> bool {
        match i as GLboolean {
            gl::TRUE => true,
            gl::FALSE => false,
            _ => unreachable!(),
        }
    }

    fn as_u32(i: GLint) -> u32 {
        i as u32
    }

    fn as_draw_mode(i: GLint) -> DrawMode {
        (i as GLenum).try_into().expect(&format!(
            "Internal OpenGL Failure, invalid DrawMode value: {i}"
        ))
    }

    /// returns true if program is currently flagged for deletion, and false otherwise.
    pub fn delete_status(program: Program) -> Result<bool, ErrorGetProgram> {
        glint(program, gl::DELETE_STATUS).map(as_bool)
    }

    /// returns true if the last link operation on program was successful, and false otherwise.
    pub fn link_status(program: Program) -> Result<bool, ErrorGetProgram> {
        glint(program, gl::LINK_STATUS).map(as_bool)
    }

    /// returns true or if the last validation operation on program was successful, and false otherwise.
    pub fn validate_status(program: Program) -> Result<bool, ErrorGetProgram> {
        glint(program, gl::VALIDATE_STATUS).map(as_bool)
    }

    /// returns the number of characters in the information log for program including the null
    /// termination character (i.e., the size of the character buffer required to store the
    /// information log). If program has no information log, a value of 0 is returned.
    pub fn info_log_length(program: Program) -> Result<u32, ErrorGetProgram> {
        glint(program, gl::INFO_LOG_LENGTH).map(as_u32)
    }

    /// returns the number of shader objects attached to program.
    pub fn attached_shaders(program: Program) -> Result<u32, ErrorGetProgram> {
        glint(program, gl::ATTACHED_SHADERS).map(as_u32)
    }

    /// returns the number of active attribute atomic counter buffers used by program.
    pub fn active_atomic_counter_buffers(program: Program) -> Result<u32, ErrorGetProgram> {
        glint(program, gl::ACTIVE_ATOMIC_COUNTER_BUFFERS).map(as_u32)
    }

    /// returns the number of active attribute variables for program.
    pub fn active_attributes(program: Program) -> Result<u32, ErrorGetProgram> {
        glint(program, gl::ACTIVE_ATTRIBUTES).map(as_u32)
    }

    /// returns the length of the longest active attribute name for program, including the null
    /// termination character (i.e., the size of the character buffer required to store the longest
    /// attribute name). If no active attributes exist, 0 is returned.
    pub fn active_attribute_max_length(program: Program) -> Result<u32, ErrorGetProgram> {
        glint(program, gl::ACTIVE_ATTRIBUTE_MAX_LENGTH).map(as_u32)
    }

    /// returns the number of active uniform variables for program.
    pub fn active_uniforms(program: Program) -> Result<u32, ErrorGetProgram> {
        glint(program, gl::ACTIVE_UNIFORMS).map(as_u32)
    }

    /// returns the length of the longest active uniform variable name for program, including the
    /// null termination character (i.e., the size of the character buffer required to store the
    /// longest uniform variable name). If no active uniform variables exist, 0 is returned.
    pub fn active_uniform_max_length(program: Program) -> Result<u32, ErrorGetProgram> {
        glint(program, gl::ACTIVE_UNIFORM_MAX_LENGTH).map(as_u32)
    }

    /// returns the length of the program binary, in bytes that will be returned by a call to
    /// [get_program_binary](super::get_program_binary). When a [link_status] is false,
    /// its program binary length is zero.
    pub fn program_binary_length(program: Program) -> Result<u32, ErrorGetProgram> {
        glint(program, gl::PROGRAM_BINARY_LENGTH).map(as_u32)
    }

    /// returns an array of three integers containing the local work group size of the compute
    /// program as specified by its input layout qualifier(s). program must be the name of a program
    /// object that has been previously linked successfully and contains a binary for the
    /// compute shader stage.
    pub fn compute_work_group_size(program: Program) -> Result<u32, ErrorGetProgram> {
        glint(program, gl::COMPUTE_WORK_GROUP_SIZE).map(as_u32)
    }

    /// returns a symbolic constant indicating the buffer mode used when transform feedback is active.
    pub fn transform_feedback_buffer_mode(
        program: Program,
    ) -> Result<Option<TransformFeedbackBufferMode>, ErrorGetProgram> {
        glint(program, gl::TRANSFORM_FEEDBACK_BUFFER_MODE).map(|res| match res as GLenum {
            gl::INTERLEAVED_ATTRIBS => Some(TransformFeedbackBufferMode::Interleaved),
            gl::SEPARATE_ATTRIBS => Some(TransformFeedbackBufferMode::Separate),
            _ => None,
        })
    }

    /// returns the number of varying variables to capture in transform feedback mode for the program.
    pub fn transform_feedback_varyings(program: Program) -> Result<u32, ErrorGetProgram> {
        glint(program, gl::TRANSFORM_FEEDBACK_VARYINGS).map(as_u32)
    }

    /// returns the length of the longest variable name to be used for transform feedback,
    /// including the null-terminator.
    pub fn transform_feedback_varying_max_length(program: Program) -> Result<u32, ErrorGetProgram> {
        glint(program, gl::TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH).map(as_u32)
    }

    /// returns the maximum number of vertices that the geometry shader in program will output.
    pub fn geometry_vertices_out(program: Program) -> Result<u32, ErrorGetProgram> {
        glint(program, gl::GEOMETRY_VERTICES_OUT).map(as_u32)
    }

    /// returns the primitive draw mode type accepted as input to the geometry shader contained in program.
    pub fn geometry_input_type(program: Program) -> Result<DrawMode, ErrorGetProgram> {
        glint(program, gl::GEOMETRY_INPUT_TYPE).map(as_draw_mode)
    }

    /// returns the primitive draw mode type that will be output by the geometry shader contained in program.
    pub fn geometry_output_type(program: Program) -> Result<DrawMode, ErrorGetProgram> {
        glint(program, gl::GEOMETRY_OUTPUT_TYPE).map(as_draw_mode)
    }

    /// returns the number of active uniform blocks for program.
    pub fn active_uniform_blocks(program: Program) -> Result<u32, ErrorGetProgram> {
        glint(program, gl::ACTIVE_UNIFORM_BLOCKS).map(as_u32)
    }

    /// returns the length of the longest active uniform block name for program, including the
    /// null termination character (i.e., the size of the character buffer required to store the
    /// longest uniform block name). If no active uniform block exist, 0 is returned.
    pub fn active_uniform_block_max_name_length(program: Program) -> Result<u32, ErrorGetProgram> {
        glint(program, gl::ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH).map(as_u32)
    }
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

/// Returns the information log for a program object
///
/// [get_program_info_log] returns the information log for the specified program object.
/// The information log for a program object is modified when the program object is linked or validated.
/// The string that is returned will be null terminated.
///
/// [get_program_info_log] returns in `buffer` as much of the information log as it can. The number
/// of characters actually returned, excluding the null termination character, is specified by the
/// Ok return value. The size of the buffer required to store the returned information log can be
/// obtained by calling [get_program]\([ProgramParam::InfoLogLength]\).
///
/// The information log for a program object is either an empty string, or a string containing
/// information about the last link operation, or a string containing information about the last
/// validation operation. It may contain diagnostic messages, warning messages, and other information.
/// When a program object is created, its information log will be a string of length 0.
///
/// # Arguments
/// * `program` - Specifies the program object whose information log is to be queried
/// * `buffer` - Specifies an array of characters that is used to return the information log
///
/// # Notes
/// The information log for a program object is the OpenGL implementer's primary mechanism for
/// conveying information about linking and validating. Therefore, the information log can be helpful
/// to application developers during the development process, even when these operations are successful.
/// Application developers should not expect different OpenGL implementations to
/// produce identical information logs.
pub fn get_program_info_log(
    program: Program,
    buffer: &mut [u8],
) -> Result<u32, ErrorGetProgramInfoLog> {
    let buf_size = buffer.len() as GLsizei;
    let mut written_length: GLsizei = 0;
    let info_log = buffer.as_mut_ptr() as *mut GLchar;
    unsafe { gl::GetProgramInfoLog(program.0, buf_size, &mut written_length, info_log) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(written_length as u32),
        ErrorOpenGL::InvalidValue => Err(ErrorGetProgramInfoLog::NonOpenGLName(program)),
        ErrorOpenGL::InvalidOperation => Err(ErrorGetProgramInfoLog::NotAProgram(program)),
        _ => unreachable!(),
    }
}

/// Possible errors of [get_program_info_log]
#[derive(Debug, Clone, Copy)]
pub enum ErrorGetProgramInfoLog {
    NonOpenGLName(Program),
    NotAProgram(Program),
}

/// Returns a parameter from a shader object
pub mod get_shader {
    use super::*;

    fn glint(shader: Shader, pname: GLenum) -> Result<GLint, ErrorGetShader> {
        let mut params: GLint = 0;
        unsafe { gl::GetShaderiv(shader.0, pname, &mut params) };
        match internal_get_error() {
            ErrorOpenGL::NoError => Ok(params.into()),
            ErrorOpenGL::InvalidValue => Err(ErrorGetShader::NonOpenGLName(shader)),
            ErrorOpenGL::InvalidOperation => Err(ErrorGetShader::NotAShader(shader)),
            _ => unreachable!(),
        }
    }

    fn as_bool(i: GLint) -> bool {
        match i as GLboolean {
            gl::TRUE => true,
            gl::FALSE => false,
            _ => unreachable!(),
        }
    }

    fn as_u32(i: GLint) -> u32 {
        i as u32
    }

    pub fn shader_type(shader: Shader) -> Result<ShaderType, ErrorGetShader> {
        glint(shader, gl::SHADER_TYPE).map(|res| {
            (res as GLenum)
                .try_into()
                .expect("Internal OpenGL Failure, invalid ShaderType value: {i}")
        })
    }

    /// returns true if shader is currently flagged for deletion, and false otherwise.
    pub fn delete_status(shader: Shader) -> Result<bool, ErrorGetShader> {
        glint(shader, gl::DELETE_STATUS).map(as_bool)
    }

    /// returns true if the last compile operation on shader was successful, and false otherwise.
    pub fn compile_status(shader: Shader) -> Result<bool, ErrorGetShader> {
        glint(shader, gl::COMPILE_STATUS).map(as_bool)
    }

    /// returns the number of characters in the information log for shader including the null
    /// termination character (i.e., the size of the character buffer required to store the
    /// information log). If shader has no information log, a value of 0 is returned.
    pub fn info_log_length(shader: Shader) -> Result<u32, ErrorGetShader> {
        glint(shader, gl::INFO_LOG_LENGTH).map(as_u32)
    }

    /// returns the length of the concatenation of the source strings that make up the shader source
    /// for the shader, including the null termination character. (i.e., the size of the character
    /// buffer required to store the shader source). If no source code exists, 0 is returned.
    pub fn source_length(shader: Shader) -> Result<u32, ErrorGetShader> {
        glint(shader, gl::SHADER_SOURCE_LENGTH).map(as_u32)
    }
}

/// Possible errors of [get_shader] queries
#[derive(Debug, Clone, Copy)]
pub enum ErrorGetShader {
    NonOpenGLName(Shader),
    NotAShader(Shader),
}

/// Returns the information log for a shader object
///
/// [get_shader_info_log] returns the information log for the specified shader object.
/// The information log for a shader object is modified when the shader is compiled.
/// The string that is returned will be null terminated.
///
/// [get_shader_info_log] returns in `buffer` as much of the information log as it can. The number
/// of characters actually returned, excluding the null termination character, is specified by the
/// Ok return value. The size of the buffer required to store the returned information log can be
/// obtained by calling [get_shader::info_log_length].
///
/// The information log for a shader object is a string that may contain diagnostic messages,
/// warning messages, and other information about the last compile operation. When a shader object
/// is created, its information log will be a string of length 0.
///
/// # Arguments
/// * `shader` - Specifies the shader object whose information log is to be queried
/// * `buffer` - Specifies an array of characters that is used to return the information log
///
/// # Notes
/// The information log for a shader object is the OpenGL implementer's primary mechanism for conveying
/// information about the compilation process. Therefore, the information log can be helpful to
/// application developers during the development process, even when compilation is successful.
/// Application developers should not expect different OpenGL implementations to
/// produce identical information logs.
pub fn get_shader_info_log(
    shader: Shader,
    buffer: &mut [u8],
) -> Result<u32, ErrorGetShaderInfoLog> {
    let buf_size = buffer.len() as GLsizei;
    let mut written_length: GLsizei = 0;
    let info_log = buffer.as_mut_ptr() as *mut GLchar;
    unsafe { gl::GetShaderInfoLog(shader.0, buf_size, &mut written_length, info_log) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(written_length as u32),
        ErrorOpenGL::InvalidValue => Err(ErrorGetShaderInfoLog::NonOpenGLName(shader)),
        ErrorOpenGL::InvalidOperation => Err(ErrorGetShaderInfoLog::NotAShader(shader)),
        _ => unreachable!(),
    }
}

/// Possible errors of [get_shader_info_log]
#[derive(Debug, Clone, Copy)]
pub enum ErrorGetShaderInfoLog {
    NonOpenGLName(Shader),
    NotAShader(Shader),
}

/// Returns the location of a uniform variable
///
/// [get_uniform_location] returns [UniformLocation] that represents the location of a specific
/// uniform variable within a program object. `uniform_name` must be a null terminated string that
/// contains no white space. `uniform_name` must be an active uniform variable name in program that
/// is not a structure, an array of structures, or a subcomponent of a vector or a matrix.
/// This function errors with [UnknownUniformName](ErrorGetUniformLocation::UnknownUniformName)
/// if `name` does not correspond to an active uniform variable in program, if `name` starts with
/// the reserved prefix "gl_", or if `name` is associated with an atomic counter or a named uniform block.
///
/// Uniform variables that are structures or arrays of structures may be queried by calling
/// [get_uniform_location] for each field within the structure. The array element operator "\[\]"
/// and the structure field operator "." may be used in name in order to select elements within an
/// array or fields within a structure. The result of using these operators is not allowed to be
/// another structure, an array of structures, or a subcomponent of a vector or a matrix.
/// Except if the last part of name indicates a uniform variable array, the location of the first
/// element of an array can be retrieved by using the name of the array, or by using the
/// name appended by "\[0\]".
///
/// The actual locations assigned to uniform variables are not known until the program object is
/// linked successfully. After linking has occurred, the command [get_uniform_location] can be used
/// to obtain the location of a uniform variable. This location value can then be passed to
/// [uniform], to set the value of the uniform variable or to [get_uniform] in order to
/// query the current value of the uniform variable. After a program object has been linked successfully,
/// the index values for uniform variables remain fixed until the next link command occurs.
/// Uniform variable locations and values can only be queried after a link if the link was successful.
pub fn get_uniform_location(
    program: Program,
    uniform_name: &std::ffi::CStr,
) -> Result<UniformLocation, ErrorGetUniformLocation> {
    let name = uniform_name.as_ptr() as *const GLchar;
    let location = unsafe { gl::GetUniformLocation(program.0, name) };
    match internal_get_error() {
        ErrorOpenGL::NoError => {
            if location < 0 {
                Err(ErrorGetUniformLocation::UnknownUniformName(
                    uniform_name.into(),
                ))
            } else {
                Ok(UniformLocation(location))
            }
        }
        ErrorOpenGL::InvalidValue => Err(ErrorGetUniformLocation::NonOpenGLName(program)),
        ErrorOpenGL::InvalidOperation => {
            if !is_program(program) {
                Err(ErrorGetUniformLocation::NotAProgram(program))
            } else {
                Err(ErrorGetUniformLocation::UnlinkedProgram(program))
            }
        }
        _ => unreachable!(),
    }
}

/// Possible errors of [get_shader_info_log]
#[derive(Debug, Clone)]
pub enum ErrorGetUniformLocation {
    NonOpenGLName(Program),
    NotAProgram(Program),
    UnlinkedProgram(Program),
    UnknownUniformName(std::ffi::CString),
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

/// Links a program object
///
/// [link_program] links the program object specified by program. If any shader objects of type
/// [Vertex](ShaderType::Vertex) are attached to program, they will be used to create an executable
/// that will run on the programmable vertex processor. If any shader objects of type
/// [Geometry](ShaderType::Geometry) are attached to program, they will be used to create an executable
/// that will run on the programmable geometry processor. If any shader objects of type
/// [Fragment](ShaderType::Fragment) are attached to program, they will be used to create an executable
/// that will run on the programmable fragment processor.
///
/// The status of the link operation will be stored as part of the program object's state. This value
/// will be set to true if the program object was linked without errors and is ready for use,
/// and false otherwise. It can be queried by calling [get_program]\([ProgramParam::LinkStatus]\).
///
/// As a result of a successful link operation, all active user-defined uniform variables belonging to
/// program will be initialized to 0, and each of the program object's active uniform variables will
/// be assigned a location that can be queried by calling [get_uniform_location]. Also, any active
/// user-defined attribute variables that have not been bound to a generic vertex attribute index
/// will be bound to one at this time.
///
/// Linking of a program object can fail for a number of reasons as specified in the OpenGL Shading
/// Language Specification. The following lists some of the conditions that will cause a link error:
/// - The number of active attribute variables supported by the implementation has been exceeded.
/// - The storage limit for uniform variables has been exceeded.
/// - The number of active uniform variables supported by the implementation has been exceeded.
/// - The main function is missing for the vertex, geometry or fragment shader.
/// - A varying variable actually used in the fragment shader is not declared in the same way
/// (or is not declared at all) in the vertex shader, or geometry shader if present.
/// - A reference to a function or variable name is unresolved.
/// - A shared global is declared with two different types or two different initial values.
/// - One or more of the attached shader objects has not been successfully compiled.
/// - Binding a generic attribute matrix caused some rows of the matrix to fall outside the allowed
/// maximum of [MAX_VERTEX_ATTRIBUTES].
/// - Not enough contiguous vertex attribute slots could be found to bind attribute matrices.
/// - The program object contains objects to form a fragment shader but does not contain objects to
/// form a vertex shader.
/// - The program object contains objects to form a geometry shader but does not contain objects to
/// form a vertex shader.
/// - The program object contains objects to form a geometry shader and the input primitive type,
/// output primitive type, or maximum output vertex count is not specified in any compiled
/// geometry shader object.
/// - The program object contains objects to form a geometry shader and the input primitive type,
/// output primitive type, or maximum output vertex count is specified differently in multiple
/// geometry shader objects.
/// - The number of active outputs in the fragment shader is greater than the value of [MAX_DRAW_BUFFERS].
/// - The program has an active output assigned to a location greater than or equal to the value of
/// [MAX_DUAL_SOURCE_DRAW_BUFFERS] and has an active output assigned an index greater than or equal to one.
/// - More than one varying out variable is bound to the same number and index.
/// - The explicit binding assigments do not leave enough space for the linker to automatically
/// assign a location for a varying out array, which requires multiple contiguous locations.
/// - The `count` specified by [transform_feedback_varyings] is non-zero, but the program object
/// has no vertex or geometry shader.
/// - Any variable name specified to [transform_feedback_varyings] in the varyings array is not
/// declared as an output in the vertex shader (or the geometry shader, if active).
/// - Any two entries in the `varyings` array given [transform_feedback_varyings] specify the same
/// varying variable.
/// - The total number of components to capture in any transform feedback varying variable is greater
/// than the constant [MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS] and the buffer mode is
/// [TransformFeedbackBufferMode::Separate].
///
/// When a program object has been successfully linked, the program object can be made part of current
/// state by calling [use_program]. Whether or not the link operation was successful, the program
/// object's information log will be overwritten. The information log can be retrieved by calling
/// [get_program_info_log].
///
/// [link_program] will also install the generated executables as part of the current rendering
/// state if the link operation was successful and the specified program object is already currently
/// in use as a result of a previous call to [use_program]. If the program object currently in use
/// is relinked unsuccessfully, its link status will be set to false , but the executables and
/// associated state will remain part of the current state until a subsequent call to [use_program]
/// removes it from use. After it is removed from use, it cannot be made part of current state
/// until it has been successfully relinked.
///
/// If `program` contains shader objects of type [Vertex](ShaderType::Vertex), and optionally of type
/// [Geometry](ShaderType::Geometry), but does not contain shader objects of type
/// [Fragment](ShaderType::Fragment), the vertex shader executable will be installed on the programmable
/// vertex processor, the geometry shader executable, if present, will be installed on the programmable
/// geometry processor, but no executable will be installed on the fragment processor. The results
/// of rasterizing primitives with such a program will be undefined.
///
/// The program object's information log is updated and the program is generated at the time of the
/// link operation. After the link operation, applications are free to modify attached shader objects,
/// compile attached shader objects, detach shader objects, delete shader objects, and attach
/// additional shader objects. None of these operations affects the information log or the program
/// that is part of the program object.
///
/// # Notes
/// - If the link operation is unsuccessful, any information about a previous link operation on
/// program is lost (i.e., a failed link does not restore the old state of program ). Certain information
/// can still be retrieved from program even after an unsuccessful link operation. See for instance
/// [get_active_attribute] and [get_active_uniform].
pub fn link_program(program: Program) -> Result<(), ErrorLinkProgram> {
    unsafe { gl::LinkProgram(program.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(ErrorLinkProgram::NonOpenGLName(program)),
        ErrorOpenGL::InvalidOperation => {
            if !is_program(program) {
                Err(ErrorLinkProgram::NotAProgram(program))
            } else {
                Err(ErrorLinkProgram::ProgramIsActiveAndTransformFeedbackIsActive(program))
            }
        }
        _ => unreachable!(),
    }
}

/// Possible errors for [link_program]
#[derive(Debug, Clone, Copy)]
pub enum ErrorLinkProgram {
    NonOpenGLName(Program),
    NotAProgram(Program),
    ProgramIsActiveAndTransformFeedbackIsActive(Program),
}

/// Replaces the source code in a shader object
///
/// [shader_source] sets the source code in `shader` to the source code in `source`. Any source code
/// previously stored in the shader object is completely replaced. The source code strings are not
/// scanned or parsed at this time; they are simply copied into the specified shader object.
///
/// # Notes
/// OpenGL copies the shader source code strings when [shader_source] is called, so an application
/// may free its copy of the source code strings immediately after the function returns.
pub fn shader_source(shader: Shader, source: &[u8]) -> Result<(), ErrorShaderSource> {
    let source_length = source.len() as GLint;
    let source_ptr = source.as_ptr();
    let sources = std::slice::from_ref(&source_ptr);

    let count: GLsizei = 1;
    let string = sources.as_ptr() as *const *const GLchar;
    let length = &source_length as *const GLint;
    unsafe { gl::ShaderSource(shader.0, count, string, length) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(ErrorShaderSource::NonOpenGLName(shader)),
        ErrorOpenGL::InvalidOperation => Err(ErrorShaderSource::NotAShader(shader)),
        _ => unreachable!(),
    }
}

/// Possible errors for [shader_source]
#[derive(Debug, Clone, Copy)]
pub enum ErrorShaderSource {
    NonOpenGLName(Shader),
    NotAShader(Shader),
}

/// Specify the value of a uniform variable for the current program object
///
/// [uniform] functions modify the value of a uniform variable or a uniform variable array. The
/// location of the uniform variable to be modified is specified by location, which should be a
/// value returned by [get_uniform_location]. [uniform] functions operate on the program object
/// that was made part of current state by calling [use_program].
///
/// The commands uniform_{1|2|3|4}{f32|i32|u32} are used to change the value of the uniform variable
/// specified by `location` using the values passed as arguments. The number specified in the command
/// should match the number of components in the data type of the specified uniform variable
/// (e.g., 1 for float, int, unsigned int, bool; 2 for vec2, ivec2, uvec2, bvec2, etc.). The suffix
/// f32 indicates that floating-point values are being passed; the suffix i32 indicates that integer
/// values are being passed; the suffix u32 indicates that unsigned integer values are being passed,
/// and this type should also match the data type of the specified uniform variable. The i32 variants
/// of this function should be used to provide values for uniform variables defined as int, ivec2, ivec3,
/// ivec4, or arrays of these. The u32 variants of this function should be used to provide values for
/// uniform variables defined as unsigned int, uvec2, uvec3, uvec4, or arrays of these. The f32 variants
/// should be used to provide values for uniform variables of type float, vec2, vec3, vec4, or arrays
/// of these. Either the i32, u32 or f32 variants may be used to provide values for uniform variables
/// of type bool, bvec2, bvec3, bvec4, or arrays of these. The uniform variable will be set to
/// false if the input value is 0 or 0.0f32, and it will be set to true otherwise.
///
/// All active uniform variables defined in a program object are initialized to 0 when the program
/// object is linked successfully. They retain the values assigned to them by a call to a [uniform]
/// function until the next successful link operation occurs on the program object, when they are
/// once again initialized to 0.
///
/// The commands uniform_{1|2|3|4}{f32|i32|u32}v can be used to modify a single uniform variable or
/// a uniform variable array. These commands pass a slice of values to be loaded into a uniform
/// variable or a uniform variable array. A slice of 1 should be used if modifying the value of a
/// single uniform variable, and a slice of 1 or greater can be used to modify an entire array or
/// part of an array. When loading n elements starting at an arbitrary position m in a uniform variable
/// array, elements m + n - 1 in the array will be replaced with the new values. If m + n - 1 is
/// larger than the size of the uniform variable array, values for all array elements beyond the end
/// of the array will be ignored. The number specified in the name of the command indicates the number
/// of components for each element in value, and it should match the number of components in the
/// data type of the specified uniform variable (e.g., 1 for float, int, bool; 2 for vec2, ivec2,
/// bvec2, etc.). The data type specified in the name of the command must match the data type for
/// the specified uniform variable as described previously for uniform_{1|2|3|4}{f32|i32|u32}.
///
/// For uniform variable arrays, each element of the array is considered to be of the type indicated
/// in the name of the command (e.g., [uniform_3f32] or [uniform_3f32v] can be used to load a uniform
/// variable array of type vec3). The number of elements of the uniform variable array to be modified
/// is specified by the number of elements in the `value` slice.
///
/// The commands uniformMatrix_{2|3|4|2x3|3x2|2x4|4x2|3x4|4x3}f32v are used to modify a matrix or an
/// array of matrices. The numbers in the command name are interpreted as the dimensionality of the
/// matrix. The number 2 indicates a 2 × 2 matrix (i.e., 4 values), the number 3 indicates a 3 × 3
/// matrix (i.e., 9 values), and the number 4 indicates a 4 × 4 matrix (i.e., 16 values). Non-square
/// matrix dimensionality is explicit, with the first number representing the number of columns and
/// the second number representing the number of rows. For example, 2x4 indicates a 2 × 4 matrix with
/// 2 columns and 4 rows (i.e., 8 values). If transpose is false, each matrix is assumed to be supplied
/// in column major order. If transpose is true, each matrix is assumed to be supplied in row major order.
/// A slice of 1 should be used if modifying the value of a single matrix, and a slice greater than
/// 1 can be used to modify an array of matrices.
///
/// # Notes
/// - [uniform_1i32] and [uniform_1i32v] are the only two functions that may be used to load uniform
/// variables defined as sampler types. Loading samplers with any other function will result in a
/// [InvalidOperation](ErrorOpenGL::InvalidOperation) error.
/// - If slice len is greater than 1 and the indicated uniform variable is not an array, a
/// [InvalidOperation](ErrorOpenGL::InvalidOperation) error is generated and the specified uniform
/// variable will remain unchanged.
/// - Other than the preceding exceptions, if the type and size of the uniform variable as defined
/// in the shader do not match the type and size specified in the name of the command used to load
/// its value, a [InvalidOperation](ErrorOpenGL::InvalidOperation) error will be generated and the
/// specified uniform variable will remain unchanged.
/// - If `location` is a value other than -1 and it does not represent a valid uniform variable location
/// in the current program object, an error will be generated, and no changes will be made to the
/// uniform variable storage of the current program object. If `location` is equal to -1, the data
/// passed in will be silently ignored and the specified uniform variable will not be changed.
pub mod uniform {
    use super::*;

    /// Specify a single f32 value of a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_1f32(location: UniformLocation, v0: f32) -> Result<(), ErrorUniformSet> {
        unsafe { gl::Uniform1f(location.0, v0) }
        internal_handle_uniform_set_error()
    }

    /// Specify a dual f32 value for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_2f32(
        location: UniformLocation,
        v0: f32,
        v1: f32,
    ) -> Result<(), ErrorUniformSet> {
        unsafe { gl::Uniform2f(location.0, v0, v1) }
        internal_handle_uniform_set_error()
    }

    /// Specify a triplet f32 value for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_3f32(
        location: UniformLocation,
        v0: f32,
        v1: f32,
        v2: f32,
    ) -> Result<(), ErrorUniformSet> {
        unsafe { gl::Uniform3f(location.0, v0, v1, v2) }
        internal_handle_uniform_set_error()
    }

    /// Specify a quad f32 value for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_4f32(
        location: UniformLocation,
        v0: f32,
        v1: f32,
        v2: f32,
        v3: f32,
    ) -> Result<(), ErrorUniformSet> {
        unsafe { gl::Uniform4f(location.0, v0, v1, v2, v3) }
        internal_handle_uniform_set_error()
    }

    /// Specify a single i32 value for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_1i32(location: UniformLocation, v0: i32) -> Result<(), ErrorUniformSet> {
        unsafe { gl::Uniform1i(location.0, v0) }
        internal_handle_uniform_set_error()
    }

    /// Specify a dual i32 value for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_2i32(
        location: UniformLocation,
        v0: i32,
        v1: i32,
    ) -> Result<(), ErrorUniformSet> {
        unsafe { gl::Uniform2i(location.0, v0, v1) }
        internal_handle_uniform_set_error()
    }

    /// Specify a triplet i32 value for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_3i32(
        location: UniformLocation,
        v0: i32,
        v1: i32,
        v2: i32,
    ) -> Result<(), ErrorUniformSet> {
        unsafe { gl::Uniform3i(location.0, v0, v1, v2) }
        internal_handle_uniform_set_error()
    }

    /// Specify a quad i32 value for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_4i32(
        location: UniformLocation,
        v0: i32,
        v1: i32,
        v2: i32,
        v3: i32,
    ) -> Result<(), ErrorUniformSet> {
        unsafe { gl::Uniform4i(location.0, v0, v1, v2, v3) }
        internal_handle_uniform_set_error()
    }

    /// Specify a single u32 value for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_1u32(location: UniformLocation, v0: u32) -> Result<(), ErrorUniformSet> {
        unsafe { gl::Uniform1ui(location.0, v0) }
        internal_handle_uniform_set_error()
    }

    /// Specify a dual u32 value for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_2u32(
        location: UniformLocation,
        v0: u32,
        v1: u32,
    ) -> Result<(), ErrorUniformSet> {
        unsafe { gl::Uniform2ui(location.0, v0, v1) }
        internal_handle_uniform_set_error()
    }

    /// Specify a triplet u32 value for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_3u32(
        location: UniformLocation,
        v0: u32,
        v1: u32,
        v2: u32,
    ) -> Result<(), ErrorUniformSet> {
        unsafe { gl::Uniform3ui(location.0, v0, v1, v2) }
        internal_handle_uniform_set_error()
    }

    /// Specify a quad u32 value for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_4u32(
        location: UniformLocation,
        v0: u32,
        v1: u32,
        v2: u32,
        v3: u32,
    ) -> Result<(), ErrorUniformSet> {
        unsafe { gl::Uniform4ui(location.0, v0, v1, v2, v3) }
        internal_handle_uniform_set_error()
    }

    /// Specify a single f32 value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_1f32v(location: UniformLocation, value: &[f32]) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let value = value.as_ptr() as *const GLfloat;
        unsafe { gl::Uniform1fv(location.0, count, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a dual f32 value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_2f32v(
        location: UniformLocation,
        value: &[(f32, f32)],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let value = value.as_ptr() as *const GLfloat;
        unsafe { gl::Uniform2fv(location.0, count, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a triplet f32 value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_3f32v(
        location: UniformLocation,
        value: &[(f32, f32, f32)],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let value = value.as_ptr() as *const GLfloat;
        unsafe { gl::Uniform3fv(location.0, count, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a quad f32 value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_4f32v(
        location: UniformLocation,
        value: &[(f32, f32, f32, f32)],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let value = value.as_ptr() as *const GLfloat;
        unsafe { gl::Uniform4fv(location.0, count, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a single i32 value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_1i32v(location: UniformLocation, value: &[i32]) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let value = value.as_ptr() as *const GLint;
        unsafe { gl::Uniform1iv(location.0, count, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a dual i32 value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_2i32v(
        location: UniformLocation,
        value: &[(i32, i32)],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let value = value.as_ptr() as *const GLint;
        unsafe { gl::Uniform2iv(location.0, count, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a triplet i32 value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_3i32v(
        location: UniformLocation,
        value: &[(i32, i32, i32)],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let value = value.as_ptr() as *const GLint;
        unsafe { gl::Uniform3iv(location.0, count, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a quad i32 value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_4i32v(
        location: UniformLocation,
        value: &[(i32, i32, i32, i32)],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let value = value.as_ptr() as *const GLint;
        unsafe { gl::Uniform4iv(location.0, count, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a single u32 value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_1u32v(location: UniformLocation, value: &[u32]) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let value = value.as_ptr() as *const GLuint;
        unsafe { gl::Uniform1uiv(location.0, count, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a dual u32 value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_2u32v(
        location: UniformLocation,
        value: &[(u32, u32)],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let value = value.as_ptr() as *const GLuint;
        unsafe { gl::Uniform2uiv(location.0, count, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a triplet u32 value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_3u32v(
        location: UniformLocation,
        value: &[(u32, u32, u32)],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let value = value.as_ptr() as *const GLuint;
        unsafe { gl::Uniform3uiv(location.0, count, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a quad u32 value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_4u32v(
        location: UniformLocation,
        value: &[(u32, u32, u32, u32)],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let value = value.as_ptr() as *const GLuint;
        unsafe { gl::Uniform4uiv(location.0, count, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a 2x2 matrix value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_matrix_2f32v(
        location: UniformLocation,
        transpose: bool,
        value: &[[f32; 4]],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let transpose = as_gl_bool(transpose);
        let value = value.as_ptr() as *const GLfloat;
        unsafe { gl::UniformMatrix2fv(location.0, count, transpose, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a 3x3 matrix value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_matrix_3f32v(
        location: UniformLocation,
        transpose: bool,
        value: &[[f32; 9]],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let transpose = as_gl_bool(transpose);
        let value = value.as_ptr() as *const GLfloat;
        unsafe { gl::UniformMatrix3fv(location.0, count, transpose, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a 4x4 matrix value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_matrix_4f32v(
        location: UniformLocation,
        transpose: bool,
        value: &[[f32; 16]],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let transpose = as_gl_bool(transpose);
        let value = value.as_ptr() as *const GLfloat;
        unsafe { gl::UniformMatrix4fv(location.0, count, transpose, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a 2x3 matrix value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_matrix_2x3f32v(
        location: UniformLocation,
        transpose: bool,
        value: &[[f32; 6]],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let transpose = as_gl_bool(transpose);
        let value = value.as_ptr() as *const GLfloat;
        unsafe { gl::UniformMatrix2x3fv(location.0, count, transpose, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a 3x2 matrix value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_matrix_3x2f32v(
        location: UniformLocation,
        transpose: bool,
        value: &[[f32; 6]],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let transpose = as_gl_bool(transpose);
        let value = value.as_ptr() as *const GLfloat;
        unsafe { gl::UniformMatrix3x2fv(location.0, count, transpose, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a 2x4 matrix value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_matrix_2x4f32v(
        location: UniformLocation,
        transpose: bool,
        value: &[[f32; 8]],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let transpose = as_gl_bool(transpose);
        let value = value.as_ptr() as *const GLfloat;
        unsafe { gl::UniformMatrix2x4fv(location.0, count, transpose, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a 4x2 matrix value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_matrix_4x2f32v(
        location: UniformLocation,
        transpose: bool,
        value: &[[f32; 8]],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let transpose = as_gl_bool(transpose);
        let value = value.as_ptr() as *const GLfloat;
        unsafe { gl::UniformMatrix4x2fv(location.0, count, transpose, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a 3x4 matrix value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_matrix_3x4f32v(
        location: UniformLocation,
        transpose: bool,
        value: &[[f32; 12]],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let transpose = as_gl_bool(transpose);
        let value = value.as_ptr() as *const GLfloat;
        unsafe { gl::UniformMatrix3x4fv(location.0, count, transpose, value) }
        internal_handle_uniform_set_error()
    }

    /// Specify a 4x3 matrix value array for a uniform variable
    ///
    /// See [uniform] for more details
    pub fn uniform_matrix_4x3f32v(
        location: UniformLocation,
        transpose: bool,
        value: &[[f32; 12]],
    ) -> Result<(), ErrorUniformSet> {
        let count = value.len() as GLsizei;
        let transpose = as_gl_bool(transpose);
        let value = value.as_ptr() as *const GLfloat;
        unsafe { gl::UniformMatrix4x3fv(location.0, count, transpose, value) }
        internal_handle_uniform_set_error()
    }

    fn internal_handle_uniform_set_error() -> Result<(), ErrorUniformSet> {
        todo!()
    }

    /// Possible errors for [uniform]
    #[derive(Debug, Clone, Copy)]
    pub enum ErrorUniformSet {}
}

#[doc(inline)]
pub use uniform::*;

/// Installs a program object as part of current rendering state
///
/// [use_program] installs the program object specified by program as part of current rendering state.
/// One or more executables are created in a program object by successfully attaching shader objects
/// to it with [attach_shader], successfully compiling the shader objects with [compile_shader],
/// and successfully linking the program object with [link_program].
///
/// A program object will contain an executable that will run on the vertex processor if it contains
/// one or more shader objects of type [Vertex](ShaderType::Vertex) that have been successfully
/// compiled and linked. A program object will contain an executable that will run on the geometry
/// processor if it contains one or more shader objects of type [Geometry](ShaderType::Geometry) that
/// have been successfully compiled and linked. Similarly, a program object will contain an executable
/// that will run on the fragment processor if it contains one or more shader objects of type
/// [Fragment](ShaderType::Fragment) that have been successfully compiled and linked.
///
/// While a program object is in use, applications are free to modify attached shader objects,
/// compile attached shader objects, attach additional shader objects, and detach or delete
/// shader objects. None of these operations will affect the executables that are part of the
/// current state. However, relinking the program object that is currently in use will install the
/// program object as part of the current rendering state if the link operation was successful
/// (see [link_program]). If the program object currently in use is relinked unsuccessfully,
/// [get_program]\([ProgramParam::LinkStatus]\) will be false, but the executables and associated
/// state will remain part of the current state until a subsequent call to [use_program] removes
/// it from use. After it is removed from use, it cannot be made part of current state until it
/// has been successfully relinked.
///
/// If program is zero, then the current rendering state refers to an invalid program object and
/// the results of shader execution are undefined. However, this is not an error.
///
/// If program does not contain shader objects of type [Fragment](ShaderType::Fragment), an executable
/// will be installed on the vertex, and possibly geometry processors, but the results of
/// fragment shader execution will be undefined.
///
/// # Notes
/// - Like buffer and texture objects, the name space for program objects may be shared across a
/// set of contexts, as long as the server sides of the contexts share the same address space.
/// If the name space is shared across contexts, any attached objects and the data associated
/// with those attached objects are shared as well.
/// - Applications are responsible for providing the synchronization across API calls when objects
/// are accessed from different execution threads.
pub fn use_program(program: Program) -> Result<(), ErrorUseProgram> {
    unsafe { gl::UseProgram(program.0) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(ErrorUseProgram::NonOpenGLName(program)),
        ErrorOpenGL::InvalidOperation => {
            if !is_program(program) {
                Err(ErrorUseProgram::NotAProgram(program))
            } else if let Ok(_) = get_program(program, ProgramParam::TransformFeedbackBufferMode) {
                Err(ErrorUseProgram::TransportFeedbackModeActive(program))
            } else {
                Err(ErrorUseProgram::CouldNotBeMadePartOfCurrentState(program))
            }
        }
        _ => unreachable!(),
    }
}

/// Possible errors for [use_program]
#[derive(Debug, Clone, Copy)]
pub enum ErrorUseProgram {
    NonOpenGLName(Program),
    NotAProgram(Program),
    CouldNotBeMadePartOfCurrentState(Program),
    TransportFeedbackModeActive(Program),
}

fn as_gl_bool(value: bool) -> GLboolean {
    match value {
        true => gl::TRUE,
        false => gl::FALSE,
    }
}

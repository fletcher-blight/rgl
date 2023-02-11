use crate::*;
use gl::types::*;

/// returns true if program is currently flagged for deletion, and false otherwise.
pub fn get_program_delete_status(program: Program) -> Result<bool, Error> {
    get_program_iv(program, gl::DELETE_STATUS).map(as_bool)?
}

/// returns true if the last link operation on program was successful, and false otherwise.
pub fn get_program_link_status(program: Program) -> Result<bool, Error> {
    get_program_iv(program, gl::LINK_STATUS).map(as_bool)?
}

/// returns true or if the last validation operation on program was successful, and false otherwise.
pub fn get_program_validate_status(program: Program) -> Result<bool, Error> {
    get_program_iv(program, gl::VALIDATE_STATUS).map(as_bool)?
}

/// returns the number of characters in the information log for program including the null
/// termination character (i.e., the size of the character buffer required to store the
/// information log). If program has no information log, a value of 0 is returned.
pub fn get_program_info_log_length(program: Program) -> Result<u32, Error> {
    get_program_iv(program, gl::INFO_LOG_LENGTH).map(as_u32)
}

/// returns the number of shader objects attached to program.
pub fn get_program_attached_shaders(program: Program) -> Result<u32, Error> {
    get_program_iv(program, gl::ATTACHED_SHADERS).map(as_u32)
}

/// returns the number of active attribute atomic counter buffers used by program.
pub fn get_program_active_atomic_counter_buffers(program: Program) -> Result<u32, Error> {
    get_program_iv(program, gl::ACTIVE_ATOMIC_COUNTER_BUFFERS).map(as_u32)
}

/// returns the number of active attribute variables for program.
pub fn get_program_active_attributes(program: Program) -> Result<u32, Error> {
    get_program_iv(program, gl::ACTIVE_ATTRIBUTES).map(as_u32)
}

/// returns the length of the longest active attribute name for program, including the null
/// termination character (i.e., the size of the character buffer required to store the longest
/// attribute name). If no active attributes exist, 0 is returned.
pub fn get_program_active_attribute_max_length(program: Program) -> Result<u32, Error> {
    get_program_iv(program, gl::ACTIVE_ATTRIBUTE_MAX_LENGTH).map(as_u32)
}

/// returns the number of active uniform variables for program.
pub fn get_program_active_uniforms(program: Program) -> Result<u32, Error> {
    get_program_iv(program, gl::ACTIVE_UNIFORMS).map(as_u32)
}

/// returns the length of the longest active uniform variable name for program, including the
/// null termination character (i.e., the size of the character buffer required to store the
/// longest uniform variable name). If no active uniform variables exist, 0 is returned.
pub fn get_program_active_uniform_max_length(program: Program) -> Result<u32, Error> {
    get_program_iv(program, gl::ACTIVE_UNIFORM_MAX_LENGTH).map(as_u32)
}

/// returns the length of the program binary, in bytes that will be returned by a call to
/// [get_program_binary](super::get_program_binary). When a [get_program_link_status] is false,
/// its program binary length is zero.
pub fn get_program_binary_length(program: Program) -> Result<u32, Error> {
    get_program_iv(program, gl::PROGRAM_BINARY_LENGTH).map(as_u32)
}

/// returns an array of three integers containing the local work group size of the compute
/// program as specified by its input layout qualifier(s). program must be the name of a program
/// object that has been previously linked successfully and contains a binary for the
/// compute shader stage.
pub fn get_program_compute_work_group_size(program: Program) -> Result<u32, Error> {
    get_program_iv(program, gl::COMPUTE_WORK_GROUP_SIZE).map(as_u32)
}

/// returns a symbolic constant indicating the buffer mode used when transform feedback is active.
pub fn get_program_transform_feedback_buffer_mode(
    program: Program,
) -> Result<Option<TransformFeedbackBufferMode>, Error> {
    get_program_iv(program, gl::TRANSFORM_FEEDBACK_BUFFER_MODE).map(|res| match res as GLenum {
        gl::INTERLEAVED_ATTRIBS => Some(TransformFeedbackBufferMode::Interleaved),
        gl::SEPARATE_ATTRIBS => Some(TransformFeedbackBufferMode::Separate),
        _ => None,
    })
}

/// returns the number of varying variables to capture in transform feedback mode for the program.
pub fn get_program_transform_feedback_varyings(program: Program) -> Result<u32, Error> {
    get_program_iv(program, gl::TRANSFORM_FEEDBACK_VARYINGS).map(as_u32)
}

/// returns the length of the longest variable name to be used for transform feedback,
/// including the null-terminator.
pub fn get_program_transform_feedback_varying_max_length(program: Program) -> Result<u32, Error> {
    get_program_iv(program, gl::TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH).map(as_u32)
}

/// returns the maximum number of vertices that the geometry shader in program will output.
pub fn get_program_geometry_vertices_out(program: Program) -> Result<u32, Error> {
    get_program_iv(program, gl::GEOMETRY_VERTICES_OUT).map(as_u32)
}

/// returns the primitive draw mode type accepted as input to the geometry shader contained in program.
pub fn get_program_geometry_input_type(program: Program) -> Result<RenderPrimitive, Error> {
    get_program_iv(program, gl::GEOMETRY_INPUT_TYPE).map(as_render_primitive)
}

/// returns the primitive draw mode type that will be output by the geometry shader contained in program.
pub fn get_program_geometry_output_type(program: Program) -> Result<RenderPrimitive, Error> {
    get_program_iv(program, gl::GEOMETRY_OUTPUT_TYPE).map(as_render_primitive)
}

/// returns the number of active uniform blocks for program.
pub fn get_program_active_uniform_blocks(program: Program) -> Result<u32, Error> {
    get_program_iv(program, gl::ACTIVE_UNIFORM_BLOCKS).map(as_u32)
}

/// returns the length of the longest active uniform block name for program, including the
/// null termination character (i.e., the size of the character buffer required to store the
/// longest uniform block name). If no active uniform block exist, 0 is returned.
pub fn get_program_active_uniform_block_max_name_length(program: Program) -> Result<u32, Error> {
    get_program_iv(program, gl::ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH).map(as_u32)
}

fn get_program_iv(program: Program, pname: GLenum) -> Result<GLint, Error> {
    let mut params: GLint = 0;
    unsafe { gl::GetProgramiv(program.id, pname, &mut params) };
    match internal_get_error() {
        ErrorOpenGL::NoError => Ok(params.into()),
        ErrorOpenGL::InvalidValue => Err(Error::NonOpenGLProgram(program)),
        ErrorOpenGL::InvalidOperation => {
            if !is_program(program) {
                Err(Error::NotAProgram(program))
            } else if pname == gl::COMPUTE_WORK_GROUP_SIZE {
                Err(Error::MissingComputeShader(program))
            } else {
                Err(Error::MissingGeometryShader(program))
            }
        }
        error => Err(Error::Unreachable(error)),
    }
}

fn as_bool(i: GLint) -> Result<bool, Error> {
    match i as GLboolean {
        gl::TRUE => Ok(true),
        gl::FALSE => Ok(false),
        val => Err(Error::ConversionFailure(format!(
            "Invalid GLboolean value: {val}"
        ))),
    }
}

fn as_u32(i: GLint) -> u32 {
    i as u32
}

fn as_render_primitive(i: GLint) -> RenderPrimitive {
    (i as GLenum).try_into().expect(&format!(
        "Internal OpenGL Failure, invalid RenderPrimitive value: {i}"
    ))
}

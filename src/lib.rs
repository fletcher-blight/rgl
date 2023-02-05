
/// Currently defined OpenGL errors
pub enum ErrorOpenGL {
    /// No error has been recorded
    NoError,

    /// An unacceptable value is specified for an enumerated argument. The offending command is ignored and has no other side effect than to set the error flag.
    InvalidEnum,

    /// A numeric argument is out of range. The offending command is ignored and has no other side effect than to set the error flag.
    InvalidValue,

    /// The specified operation is not allowed in the current state. The offending command is ignored and has no other side effect than to set the error flag.
    InvalidOperation,

    /// The framebuffer object is not complete. The offending command is ignored and has no other side effect than to set the error flag.
    InvalidFrameBufferOperation,

    /// There is not enough memory left to execute the command. The state of the GL is undefined, except for the state of the error flags, after this error is recorded.
    OutOfMemory,

    /// An attempt has been made to perform an operation that would cause an internal stack to underflow.
    StackUnderflow,

    /// An attempt has been made to perform an operation that would cause an internal stack to overflow.
    StackOverflow,

    /// GL returned a non-standard error code.
    Unknown(u32),
}

/// return error information
///
/// Returns the value of the error flag. Each detectable error is assigned a numeric code and symbolic name.
/// When an error occurs, the error flag is set to the appropriate error code value.
/// No other errors are recorded until [get_error](get_error) is called, the error code is returned, and the flag is reset to [NoError](ErrorOpenGL::NoError).
/// If a call to [get_error](get_error) returns [NoError](ErrorOpenGL::NoError),
/// there has been no detectable error since the last call to [get_error](get_error),
/// or since the GL was initialized.
///
/// To allow for distributed implementations, there may be several error flags. If any single error flag has recorded an error,
/// the value of that flag is returned and that flag is reset to [NoError](ErrorOpenGL::NoError) when [get_error](get_error) is called.
/// If more than one flag has recorded an error, [get_error](get_error) returns and clears an arbitrary error flag value.
/// Thus, [get_error](get_error) should always be called in a loop, until it returns [NoError](ErrorOpenGL::NoError), if all error flags are to be reset.
///
/// When an error flag is set, results of a GL operation are undefined only if [OutOfMemory](ErrorOpenGL::OutOfMemory) has occurred.
/// In all other cases, the command generating the error is ignored and has no effect on the GL state or frame buffer contents.
/// If the generating command returns a value, it returns 0.
/// If [get_error](get_error) itself generates an error, it returns 0.
pub fn get_error() -> ErrorOpenGL {
    match unsafe { gl::GetError() } {
        gl::NO_ERROR => ErrorOpenGL::NoError,
        gl::INVALID_ENUM => ErrorOpenGL::InvalidEnum,
        gl::INVALID_VALUE => ErrorOpenGL::InvalidValue,
        gl::INVALID_OPERATION => ErrorOpenGL::InvalidOperation,
        gl::INVALID_FRAME_BUFFER_OPERATION => ErrorOpenGL::InvalidFrameBufferOperation,
        gl::OUT_OF_MEMORY => ErrorOpenGL::OutOfMemory,
        gl::STACK_UNDERFLOW => ErrorOpenGL::StackUnderflow,
        gl::STACK_OVERFLOW => ErrorOpenGL::StackUnderflow,
        unknown => ErrorOpenGL::Unknown(unknown),
    }
}




















//
// pub mod uniform;
//
// pub struct Pipeline(GLuint);
// pub struct Program(GLuint);
//
// pub enum ErrorOpenGL {
//     /// An unacceptable value is specified for an enumerated argument. The offending command is ignored and has no other side effect than to set the error flag.
//     InvalidEnum,
//
//     /// A numeric argument is out of range. The offending command is ignored and has no other side effect than to set the error flag.
//     InvalidValue,
//
//     /// The specified operation is not allowed in the current state. The offending command is ignored and has no other side effect than to set the error flag.
//     InvalidOperation,
//
//     /// The framebuffer object is not complete. The offending command is ignored and has no other side effect than to set the error flag.
//     InvalidFrameBufferOperation,
//
//     /// There is not enough memory left to execute the command. The state of the GL is undefined, except for the state of the error flags, after this error is recorded.
//     OutOfMemory,
//
//     /// An attempt has been made to perform an operation that would cause an internal stack to underflow.
//     StackUnderflow,
//
//     /// An attempt has been made to perform an operation that would cause an internal stack to overflow.
//     StackOverflow,
// }
//
// /// return error information
// ///
// /// Returns the value of the error flag. Each detectable error is assigned a numeric code and symbolic name.
// /// When an error occurs, the error flag is set to the appropriate error code value.
// /// No other errors are recorded until [get_error](get_error) is called, the error code is returned, and the flag is reset to `None`.
// /// If a call to [get_error](get_error) returns `None`, there has been no detectable error since the last call to [get_error](get_error), or since the GL was initialized.
// pub fn get_error() -> Option<ErrorOpenGL> {
//     None
// }
//
// pub enum ErrorActiveShaderProgram {
//     Unknown(ErrorOpenGL),
//
// }
//
// /// set the active program object for a program pipeline object
// ///
// ///
// /// Sets the linked program named by `program` to be the active program for the program pipeline object `pipeline`.
// /// The active program in the active program pipeline object is the target of calls to [uniform](uniform) when no program has been made current through a call to [use_program](use_program).
// ///
// /// # Arguments
// /// * `pipeline` - Specifies the program pipeline object to set the active program object for.
// /// * `program` - Specifies the program object to set as the active program pipeline object pipeline.
// pub fn active_shader_program(pipeline: Pipeline, program: Program) -> Result<(), ErrorActiveShaderProgram> {
//     Ok(())
// }
//
//
//
// pub enum ErrorUseProgram {
//
// }
//
// pub fn use_program() -> Result<(), ErrorUseProgram> {
//     Ok(())
// }
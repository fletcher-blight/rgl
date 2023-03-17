//! # Return the value or values of a selected parameter
//! <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGet.xhtml>

use crate::prelude::*;
use gl::types::*;

fn get_bool(pname: GLenum) -> bool {
    let mut val = GLboolean::default();

    // SAFE: synchronous write into `val`, no memory retained
    unsafe { gl::GetBooleanv(pname, &mut val) };
    val == gl::TRUE
}

fn get_f64(pname: GLenum) -> f64 {
    let mut val = GLdouble::default();

    // SAFE: synchronous write into `val`, no memory retained
    unsafe { gl::GetDoublev(pname, &mut val) };
    val
}

fn get_f32(pname: GLenum) -> f32 {
    let mut val = GLfloat::default();

    // SAFE: synchronous write into `val`, no memory retained
    unsafe { gl::GetFloatv(pname, &mut val) };
    val
}

fn get_i32(pname: GLenum) -> i32 {
    let mut val = GLint::default();

    // SAFE: synchronous write into `val`, no memory retained
    unsafe { gl::GetIntegerv(pname, &mut val) };
    val
}

/// # Return the value or values of a selected parameter
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGet.xhtml>
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// active_texture(42);
/// assert_eq!(get_active_texture(), 42);
/// ```
///
/// # Description
/// Returns a single value indicating the active multitexture unit. The initial value is `0``. See
/// [active_texture].
///
/// # Compatability
///
/// # Errors
///
/// # Associated Gets
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [get_active_texture] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
pub fn get_active_texture() -> u32 {
    get_i32(gl::ACTIVE_TEXTURE) as u32
}

/// # OpenGL Error Values
/// see [get_error]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    /// No error has been recorded.
    NoError,

    /// An unacceptable value is specified for an enumerated argument. The offending command is
    /// ignored and has no other side effect than to set the error flag.
    InvalidEnum,

    /// A numeric argument is out of range. The offending command is ignored and has no other side
    /// effect than to set the error flag.
    InvalidValue,

    /// The specified operation is not allowed in the current state. The offending command is
    /// ignored and has no other side effect than to set the error flag.
    InvalidOperation,

    /// The framebuffer object is not complete. The offending command is ignored and has no other
    /// side effect than to set the error flag.
    InvalidFramebufferOperation,

    /// There is not enough memory left to execute the command. The state of the GL is undefined,
    /// except for the state of the error flags, after this error is recorded.
    OutOfMemory,

    /// An attempt has been made to perform an operation that would cause an internal stack to
    /// underflow.
    StackUnderflow,

    /// An attempt has been made to perform an operation that would cause an internal stack to overflow.
    StackOverflow,

    /// Allow for unknown, other implementation specific error codes
    ImplementationSpecific(u32),
}

impl From<GLenum> for Error {
    fn from(value: GLenum) -> Self {
        match value {
            gl::NO_ERROR => Error::NoError,
            gl::INVALID_ENUM => Error::InvalidEnum,
            other => Error::ImplementationSpecific(other),
        }
    }
}

/// # Return error information
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGetError.xhtml>
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// assert_eq!(get_error(), Error::NoError);
/// ```
///
/// # Description
/// [get_error] returns the value of the error flag. Each detectable error is assigned a numeric
/// code and symbolic name. When an error occurs, the error flag is set to the appropriate error
/// code value. No other errors are recorded until [get_error] is called, the error code is
/// returned, and the flag is reset to [Error::NoError]. If a call to [get_error] returns
/// [Error::NoError], there has been no detectable error since the last call to [get_error], or
/// since the GL was initialized.
///
/// To allow for distributed implementations, there may be several error flags. If any single error
/// flag has recorded an error, the value of that flag is returned and that flag is reset to
/// [Error::NoError] when [get_error] is called. If more than one flag has recorded an error,
/// [get_error] returns and clears an arbitrary error flag value. Thus, [get_error] should always be
/// called in a loop, until it returns [Error::NoError], if all error flags are to be reset.
///
/// Initially, all error flags are set to [Error::NoError].
///
/// When an error flag is set, results of a GL operation are undefined only if [Error::OutOfMemory]
/// has occurred. In all other cases, the command generating the error is ignored and has no effect
/// on the GL state or frame buffer contents. If the generating command returns a value, it returns
/// 0. If [get_error] itself generates an error, it returns [Error::NoError].
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [get_error] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
pub fn get_error() -> Error {
    // SAFE: just an integer copy
    let error = unsafe { gl::GetError() };
    Error::from(error)
}
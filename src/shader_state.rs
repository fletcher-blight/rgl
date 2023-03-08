//! # Shader Program Usage and State
//! <https://www.khronos.org/opengl/wiki/Category:Core_API_Ref_Shader_Program_Usage_and_State>
//!
//! # Description
//! The core OpenGL API reference for functions that modify GLSL program state and bind them to the context for rendering.

use crate::prelude::*;
use gl::types::*;

/// # Specify the value of a uniform variable for the current program object
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glUniform.xhtml>
///
/// # Arguments
/// * `location` - Specifies the location of the uniform variable to be modified.
/// * `count` -
/// * `order` - For the matrix commands, specifies whether the values are
/// [row major](MatrixOrderMajor::Row) or [column major](MatrixOrderMajor::Column).
/// * `v0`, `v1`, `v2`, `v3` - For the scalar commands, specifies the new values to be used for the
/// specified uniform variable.
/// * `values` - For the vector and matrix commands, specifies a slice to an array of values that
/// will be used to update the specified uniform variable. This should be an array to a single value
/// if the targeted uniform variable is not an array, and 1 or more if it is an array.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// let loc = UniformLocation(42);
///
/// uniform_1f32(loc, 42.0);
/// uniform_2f32(loc, 1.0, 2.0);
/// uniform_3f32(loc, 1.0, 2.0, 3.0);
/// uniform_4f32(loc, 1.0, 2.0, 3.0, 4.0);
///
/// uniform_1f32v(loc, &[1.0, 2.0, 3.0, 4.0]);
/// uniform_2f32v(loc, &[[1.0, 2.0], [3.0, 4.0]]);
/// uniform_3f32v(loc, &[[1.0, 2.0, 3.0]]);
/// uniform_4f32v(loc, &[[1.0, 2.0, 3.0, 4.0]]);
///
/// uniform_matrix_2f32v(loc, MatrixOrderMajor::Row, &[
///     [1.0, 2.0, 3.0, 4.0],
///     [5.0, 6.0, 7.0, 8.0],
/// ]);
/// uniform_matrix_3f32v(loc, MatrixOrderMajor::Row, &[
///     [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
///     [9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0],
/// ]);
/// uniform_matrix_4f32v(loc, MatrixOrderMajor::Row, &[
///     [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0],
///     [16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0],
/// ]);
/// ```
///
/// # Description
/// [uniform] functions modifies the value of a uniform variable or a uniform variable array. The
/// location of the uniform variable to be modified is specified by `location`, which should be a
/// value returned by [get_uniform_location]. [uniform] functions operates on the program object
/// that was made part of current state by calling [use_program].
///
/// The commands `uniform_{1|2|3|4}{f|i|u}32` are used to change the value of the uniform variable
/// specified by location using the values passed as arguments. The number specified in the command
/// should match the number of components in the data type of the specified uniform variable (e.g.,
/// 1 for float, int, unsigned int, bool; 2 for vec2, ivec2, uvec2, bvec2, etc.). The suffix `f32`
/// indicates that floating-point values are being passed; the suffix `i32` indicates that integer
/// values are being passed; the suffix `u32` indicates that unsigned integer values are being
/// passed, and this type should also match the data type of the specified uniform variable. The
/// `i32` variants of this function should be used to provide values for uniform variables defined
/// as int, ivec2, ivec3, ivec4, or arrays of these. The `u32` variants of this function should be
/// used to provide values for uniform variables defined as unsigned int, uvec2, uvec3, uvec4, or
/// arrays of these. The `f32` variants should be used to provide values for uniform variables of
/// type float, vec2, vec3, vec4, or arrays of these. Either the `i32`, `u32` or `f32` variants may
/// be used to provide values for uniform variables of type bool, bvec2, bvec3, bvec4, or arrays of
/// these. The uniform variable will be set to false if the input value is 0 or 0.0f, and it will be
/// set to true otherwise.
///
/// All active uniform variables defined in a program object are initialized to 0 when the program
/// object is linked successfully. They retain the values assigned to them by a call to [uniform]
/// until the next successful link operation occurs on the program object, when they are once again
/// initialized to 0.
///
/// The commands `uniform_{1|2|3|4}{f|i|u}32v` can be used to modify a single uniform variable or a
/// uniform variable array. These commands pass a slice of values to be loaded into a uniform
/// variable or a uniform variable array. A single element slice should be used if modifying the
/// value of a single uniform variable, and a multi-element slice can be used to modify an entire
/// array or part of an array. When loading `n` elements starting at an arbitrary position `m` in a
/// uniform variable array, elements `m + n - 1` in the array will be replaced with the new values.
/// If `m + n - 1` is larger than the size of the uniform variable array, values for all array
/// elements beyond the end of the array will be ignored. The number specified in the name of the
/// command indicates the number of components for each element in value, and it should match the
/// number of components in the data type of the specified uniform variable (e.g., 1 for float, int,
/// bool; 2 for vec2, ivec2, bvec2, etc.). The data type specified in the name of the command must
/// match the data type for the specified uniform variable as described previously for
/// `uniform_{1|2|3|4}{f|i|u}32`.
///
/// For uniform variable arrays, each element of the array is considered to be of the type indicated
/// in the name of the command (e.g., [uniform_3f32] or [uniform_3f32v] can be used to load a
/// uniform variable array of type vec3). The number of elements of the uniform variable array to be
/// modified is specified by the number of elements in the given slice `values`.
///
/// The commands `uniform_matrix_{2|3|4|2x3|3x2|2x4|4x2|3x4|4x3}f32v` are used to modify a matrix or
/// an array of matrices. The numbers in the command name are interpreted as the dimensionality of
/// the matrix. The number `2` indicates a `2 × 2` matrix (i.e., 4 values), the number `3` indicates
/// a `3 × 3` matrix (i.e., 9 values), and the number `4` indicates a `4 × 4` matrix (i.e., 16
/// values). Non-square matrix dimensionality is explicit, with the first number representing the
/// number of columns and the second number representing the number of rows. For example, `2x4`
/// indicates a `2 × 4` matrix with `2` columns and `4` rows (i.e., 8 values). A single element slice
/// should be used if modifying the value of a single matrix, and a multi-element slice can be used
/// to modify an array of matrices.
///
/// If location is a value other than -1 and it does not represent a valid uniform variable location
/// in the current program object, an error will be generated, and no changes will be made to the
/// uniform variable storage of the current program object. If location is equal to -1, the data
/// passed in will be silently ignored and the specified uniform variable will not be changed.
///
/// # Errors
/// * [uniform_1i32] and [uniform_1i32v] are the only two functions that may be used to load uniform
/// variables defined as sampler types. Loading samplers with any other function will result in a
/// [Error::InvalidOperation] error.
/// * If a multi-element slice used and the indicated uniform variable is not an array, a
/// [Error::InvalidOperation] error is generated and the specified uniform variable will remain
/// unchanged.
///
/// * [Error::InvalidOperation] - if there is no current program object.
/// * [Error::InvalidOperation] - if the size of the uniform variable declared in the shader does
/// not match the size indicated by the [uniform] command.
/// * [Error::InvalidOperation] - if one of the `i32` or `u32` variants of this function is used to
/// load a uniform variable of type float, vec2, vec3, vec4, or an array of these, or if one of the
/// floating-point variants of this function is used to load a uniform variable of type int, ivec2,
/// ivec3, ivec4, unsigned int, uvec2, uvec3, uvec4, or an array of these.
/// * [Error::InvalidOperation] - if one of the `i32` variants of this function is used to load a
/// uniform variable of type unsigned int, uvec2, uvec3, uvec4, or an array of these.
/// * [Error::InvalidOperation] - if one of the `u32` variants of this function is used to load a
/// uniform variable of type int, ivec2, ivec3, ivec4, or an array of these.
/// * [Error::InvalidOperation] - if `location` is an invalid uniform location for the current
/// program object and `location` is not equal to -1.
///
/// # Associated Gets
/// * [get_current_program]
/// * [get_active_uniform]
/// * [get_uniform]
/// * [get_uniform_location]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [uniform_1f32] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_2f32] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_3f32] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_4f32] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_1i32] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_2i32] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_3i32] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_4i32] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_1u32] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_2u32] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_3u32] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_4u32] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_1f32v] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_2f32v] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_3f32v] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_4f32v] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_1i32v] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_2i32v] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_3i32v] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_4i32v] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_1u32v] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_2u32v] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_3u32v] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_4u32v] | N | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_matrix_2f32v] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_matrix_3f32v] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_matrix_4f32v] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_matrix_2x3f32v] | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_matrix_2x4f32v] | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_matrix_3x2f32v] | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_matrix_3x4f32v] | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_matrix_4x2f32v] | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
/// | [uniform_matrix_4x3f32v] | N | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [link_program]
/// * [use_program]
pub mod uniform {
    use crate::prelude::*;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum MatrixOrderMajor {
        Row,
        Column,
    }

    impl From<MatrixOrderMajor> for GLboolean {
        fn from(value: MatrixOrderMajor) -> Self {
            match value {
                MatrixOrderMajor::Row => gl::TRUE,
                MatrixOrderMajor::Column => gl::FALSE,
            }
        }
    }

    /// # Set a f32 value of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_1f32(location: UniformLocation, v0: f32) {
        let location = location.0;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform1f(location, v0) }
    }

    /// # Set a f32 vec2 of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_2f32(location: UniformLocation, v0: f32, v1: f32) {
        let location = location.0;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform2f(location, v0, v1) }
    }

    /// # Set a f32 vec3 of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_3f32(location: UniformLocation, v0: f32, v1: f32, v2: f32) {
        let location = location.0;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform3f(location, v0, v1, v2) }
    }

    /// # Set a f32 vec4 of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_4f32(location: UniformLocation, v0: f32, v1: f32, v2: f32, v3: f32) {
        let location = location.0;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform4f(location, v0, v1, v2, v3) }
    }

    /// # Set a i32 value of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_1i32(location: UniformLocation, v0: i32) {
        let location = location.0;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform1i(location, v0) }
    }

    /// # Set a i32 vec2 of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_2i32(location: UniformLocation, v0: i32, v1: i32) {
        let location = location.0;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform2i(location, v0, v1) }
    }

    /// # Set a i32 vec3 of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_3i32(location: UniformLocation, v0: i32, v1: i32, v2: i32) {
        let location = location.0;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform3i(location, v0, v1, v2) }
    }

    /// # Set a i32 vec4 of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_4i32(location: UniformLocation, v0: i32, v1: i32, v2: i32, v3: i32) {
        let location = location.0;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform4i(location, v0, v1, v2, v3) }
    }

    /// # Set a u32 value of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_1u32(location: UniformLocation, v0: u32) {
        let location = location.0;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform1ui(location, v0) }
    }

    /// # Set a u32 vec2 of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_2u32(location: UniformLocation, v0: u32, v1: u32) {
        let location = location.0;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform2ui(location, v0, v1) }
    }

    /// # Set a u32 vec3 of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_3u32(location: UniformLocation, v0: u32, v1: u32, v2: u32) {
        let location = location.0;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform3ui(location, v0, v1, v2) }
    }

    /// # Set a u32 vec4 of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_4u32(location: UniformLocation, v0: u32, v1: u32, v2: u32, v3: u32) {
        let location = location.0;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform4ui(location, v0, v1, v2, v3) }
    }

    /// # Set an array of f32 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_1f32v(location: UniformLocation, values: &[f32]) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let value = values.as_ptr();

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform1fv(location, count, value) }
    }

    /// # Set an array of f32 vec2 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_2f32v(location: UniformLocation, values: &[[f32; 2]]) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let value = values.as_ptr() as *const f32;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform2fv(location, count, value) }
    }

    /// # Set an array of f32 vec3 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_3f32v(location: UniformLocation, values: &[[f32; 3]]) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let value = values.as_ptr() as *const f32;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform3fv(location, count, value) }
    }

    /// # Set an array of f32 vec4 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_4f32v(location: UniformLocation, values: &[[f32; 4]]) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let value = values.as_ptr() as *const f32;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform4fv(location, count, value) }
    }

    /// # Set an array of i32 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_1i32v(location: UniformLocation, values: &[i32]) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let value = values.as_ptr();

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform1iv(location, count, value) }
    }

    /// # Set an array of i32 vec2 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_2i32v(location: UniformLocation, values: &[[i32; 2]]) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let value = values.as_ptr() as *const i32;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform2iv(location, count, value) }
    }

    /// # Set an array of i32 vec3 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_3i32v(location: UniformLocation, values: &[[i32; 3]]) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let value = values.as_ptr() as *const i32;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform3iv(location, count, value) }
    }

    /// # Set an array of i32 vec4 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_4i32v(location: UniformLocation, values: &[[i32; 4]]) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let value = values.as_ptr() as *const i32;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform4iv(location, count, value) }
    }

    /// # Set an array of u32 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_1u32v(location: UniformLocation, values: &[u32]) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let value = values.as_ptr();

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform1uiv(location, count, value) }
    }

    /// # Set an array of u32 vec2 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_2u32v(location: UniformLocation, values: &[[u32; 2]]) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let value = values.as_ptr() as *const u32;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform2uiv(location, count, value) }
    }

    /// # Set an array of u32 vec3 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_3u32v(location: UniformLocation, values: &[[u32; 3]]) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let value = values.as_ptr() as *const u32;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform3uiv(location, count, value) }
    }

    /// # Set an array of u32 vec4 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_4u32v(location: UniformLocation, values: &[[u32; 4]]) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let value = values.as_ptr() as *const u32;

        // SAFE: synchronous integer copy
        unsafe { gl::Uniform4uiv(location, count, value) }
    }

    /// # Set an array of f32 mat2 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_matrix_2f32v(
        location: UniformLocation,
        order: MatrixOrderMajor,
        values: &[[f32; 4]],
    ) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let transpose = GLboolean::from(order);
        let value = values.as_ptr() as *const f32;

        // SAFE: synchronous integer copy
        unsafe { gl::UniformMatrix2fv(location, count, transpose, value) }
    }

    /// # Set an array of f32 mat3 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_matrix_3f32v(
        location: UniformLocation,
        order: MatrixOrderMajor,
        values: &[[f32; 9]],
    ) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let transpose = GLboolean::from(order);
        let value = values.as_ptr() as *const f32;

        // SAFE: synchronous integer copy
        unsafe { gl::UniformMatrix3fv(location, count, transpose, value) }
    }

    /// # Set an array of f32 mat4 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_matrix_4f32v(
        location: UniformLocation,
        order: MatrixOrderMajor,
        values: &[[f32; 16]],
    ) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let transpose = GLboolean::from(order);
        let value = values.as_ptr() as *const f32;

        // SAFE: synchronous integer copy
        unsafe { gl::UniformMatrix4fv(location, count, transpose, value) }
    }

    /// # Set an array of f32 mat2x3 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_matrix_2x3f32v(
        location: UniformLocation,
        order: MatrixOrderMajor,
        values: &[[f32; 6]],
    ) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let transpose = GLboolean::from(order);
        let value = values.as_ptr() as *const f32;

        // SAFE: synchronous integer copy
        unsafe { gl::UniformMatrix2x3fv(location, count, transpose, value) }
    }

    /// # Set an array of f32 mat3x2 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_matrix_3x2f32v(
        location: UniformLocation,
        order: MatrixOrderMajor,
        values: &[[f32; 6]],
    ) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let transpose = GLboolean::from(order);
        let value = values.as_ptr() as *const f32;

        // SAFE: synchronous integer copy
        unsafe { gl::UniformMatrix3x2fv(location, count, transpose, value) }
    }

    /// # Set an array of f32 mat2x4 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_matrix_2x4f32v(
        location: UniformLocation,
        order: MatrixOrderMajor,
        values: &[[f32; 8]],
    ) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let transpose = GLboolean::from(order);
        let value = values.as_ptr() as *const f32;

        // SAFE: synchronous integer copy
        unsafe { gl::UniformMatrix2x4fv(location, count, transpose, value) }
    }

    /// # Set an array of f32 mat4x2 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_matrix_4x2f32v(
        location: UniformLocation,
        order: MatrixOrderMajor,
        values: &[[f32; 8]],
    ) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let transpose = GLboolean::from(order);
        let value = values.as_ptr() as *const f32;

        // SAFE: synchronous integer copy
        unsafe { gl::UniformMatrix4x2fv(location, count, transpose, value) }
    }

    /// # Set an array of f32 mat3x4 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_matrix_3x4f32v(
        location: UniformLocation,
        order: MatrixOrderMajor,
        values: &[[f32; 12]],
    ) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let transpose = GLboolean::from(order);
        let value = values.as_ptr() as *const f32;

        // SAFE: synchronous integer copy
        unsafe { gl::UniformMatrix3x4fv(location, count, transpose, value) }
    }

    /// # Set an array of f32 mat4x3 values of a uniform variable for the current program object
    /// see [uniform]
    pub fn uniform_matrix_4x3f32v(
        location: UniformLocation,
        order: MatrixOrderMajor,
        values: &[[f32; 12]],
    ) {
        let location = location.0;
        let count = values.len() as GLsizei;
        let transpose = GLboolean::from(order);
        let value = values.as_ptr() as *const f32;

        // SAFE: synchronous integer copy
        unsafe { gl::UniformMatrix4x3fv(location, count, transpose, value) }
    }
}
pub use uniform::*;

/// # Installs a program object as part of current rendering state
/// <https://registry.khronos.org/OpenGL-Refpages/gl4/html/glUseProgram.xhtml>
///
/// # Arguments
/// * `program` - Specifies the handle of the program object whose executables are to be used as
/// part of current rendering state.
///
/// # Example
/// ```no_run
/// # use rgl::prelude::*;
/// use_program(Program(42));
/// ```
///
/// # Description
/// [use_program] installs the program object specified by program as part of current rendering
/// state. One or more executables are created in a program object by successfully attaching shader
/// objects to it with [attach_shader], successfully compiling the shader objects with
/// [compile_shader], and successfully linking the program object with [link_program].
///
/// While a program object is in use, applications are free to modify attached shader objects,
/// compile attached shader objects, attach additional shader objects, and detach or delete shader
/// objects. None of these operations will affect the executables that are part of the current
/// state. However, relinking the program object that is currently in use will install the program
/// object as part of the current rendering state if the link operation was successful (see
/// [link_program] ). If the program object currently in use is relinked unsuccessfully, its link
/// status will be set to false, but the executables and associated state will remain part of the
/// current state until a subsequent call to [use_program] removes it from use. After it is removed
/// from use, it cannot be made part of current state until it has been successfully relinked.
///
/// If program is zero, then the current rendering state refers to an invalid program object and the
/// results of shader execution are undefined. However, this is not an error.
///
/// If program does not contain shader objects of type [ShaderType::Fragment], an executable will be
/// installed on the vertex, and possibly geometry processors, but the results of fragment shader
/// execution will be undefined.
///
/// Like buffer and texture objects, the name space for program objects may be shared across a set
/// of contexts, as long as the server sides of the contexts share the same address space. If the
/// name space is shared across contexts, any attached objects and the data associated with those
/// attached objects are shared as well.
///
/// Applications are responsible for providing the synchronization across API calls when objects are
/// accessed from different execution threads.
///
/// # Errors
/// * [Error::InvalidValue] - if `program` is not a value generated by OpenGL.
/// * [Error::InvalidOperation] - if `program` is not a program object.
/// * [Error::InvalidOperation] - if `program` could not be made part of current state.
/// * [Error::InvalidOperation] - if `program` is the currently active program object and transform
/// feedback mode is active.
///
/// # Associated Gets
/// * [get_current_program]
/// * [get_active_attrib] with arguments `program` and the index of an active attribute variable
/// * [get_active_uniform] with arguments `program` and the index of an active uniform variable
/// * [get_attached_shaders] with arguments `program`
/// * [get_attrib_location] with arguments `program` and the name of an attribute variable
/// * all `get_program_*` variants
/// * [get_program_info_log] with arguments `program`
/// * [get_uniform] with arguments `program` and the location of a uniform variable
/// * [get_uniform_location] with arguments `program` and the name of a uniform variable
/// * [is_program]
///
/// # Version Support
///
/// | Function / Feature Name | 2.0 | 2.1 | 3.0 | 3.1 | 3.2 | 3.3 | 4.0 | 4.1 | 4.2 | 4.3 | 4.4 | 4.5 |
/// |-------------------------|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
/// | [use_program] | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
///
/// # See Also
/// * [attach_shader]
/// * [bind_attrib_location]
/// * [compile_shader]
/// * [create_program]
/// * [delete_program]
/// * [detach_shader]
/// * all `uniform_*` variants
/// * [use_program]
/// * [validate_program]
/// * [vertex_attrib]
pub fn use_program(program: Program) {
    let program = program.0;
    unsafe { gl::UseProgram(program) }
}

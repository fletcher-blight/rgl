use crate::*;

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
/// and false otherwise. It can be queried by calling [get_program_link_status]\).
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
pub fn link_program(program: Program) -> Result<(), Error> {
    unsafe { gl::LinkProgram(program.id) };

    let error = internal_get_error();
    match error {
        ErrorOpenGL::NoError => Ok(()),
        ErrorOpenGL::InvalidValue => Err(Error::NonOpenGLProgram(program)),
        ErrorOpenGL::InvalidOperation => {
            if !is_program(program) {
                Err(Error::NotAProgram(program))
            } else {
                Err(Error::TransportFeedbackModeActive(program))
            }
        }
        _ => Err(Error::Unreachable(error)),
    }
}

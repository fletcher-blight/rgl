use nalgebra_glm as glm;

fn main() -> anyhow::Result<()> {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
    }
    let window = video
        .window("LearnOpenGL: Basic Light Ex - Gouraud", 1920, 1080)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().unwrap();
    rgl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // ============================================================================================

    rgl::enable(rgl::Capability::DepthTest);

    let cube_shader_program = {
        let vertex_shader = rgl::create_shader(rgl::ShaderType::Vertex).unwrap();
        rgl::shader_source(vertex_shader, include_bytes!("phong.vert"))?;
        rgl::compile_shader(vertex_shader)?;
        assert!(rgl::get_shader_compile_status(vertex_shader)?);

        let fragment_shader = rgl::create_shader(rgl::ShaderType::Fragment).unwrap();
        rgl::shader_source(fragment_shader, include_bytes!("phong.frag"))?;
        rgl::compile_shader(fragment_shader)?;
        assert!(rgl::get_shader_compile_status(fragment_shader)?);

        let shader_program = rgl::create_program().unwrap();
        rgl::attach_shader(shader_program, vertex_shader)?;
        rgl::attach_shader(shader_program, fragment_shader)?;
        rgl::link_program(shader_program)?;
        assert!(rgl::get_program_link_status(shader_program)?);

        rgl::detach_shader(shader_program, vertex_shader)?;
        rgl::detach_shader(shader_program, fragment_shader)?;
        rgl::delete_shader(vertex_shader)?;
        rgl::delete_shader(fragment_shader)?;

        shader_program
    };

    let light_shader_program = {
        let vertex_shader = rgl::create_shader(rgl::ShaderType::Vertex).unwrap();
        rgl::shader_source(vertex_shader, include_bytes!("light.vert"))?;
        rgl::compile_shader(vertex_shader)?;
        assert!(rgl::get_shader_compile_status(vertex_shader)?);

        let fragment_shader = rgl::create_shader(rgl::ShaderType::Fragment).unwrap();
        rgl::shader_source(fragment_shader, include_bytes!("light.frag"))?;
        rgl::compile_shader(fragment_shader)?;
        assert!(rgl::get_shader_compile_status(fragment_shader)?);

        let shader_program = rgl::create_program().unwrap();
        rgl::attach_shader(shader_program, vertex_shader)?;
        rgl::attach_shader(shader_program, fragment_shader)?;
        rgl::link_program(shader_program)?;
        assert!(rgl::get_program_link_status(shader_program)?);

        rgl::detach_shader(shader_program, vertex_shader)?;
        rgl::detach_shader(shader_program, fragment_shader)?;
        rgl::delete_shader(vertex_shader)?;
        rgl::delete_shader(fragment_shader)?;

        shader_program
    };

    let vao = rgl::gen_vertex_array();
    let vbo = rgl::gen_buffer();

    rgl::bind_vertex_array(Some(vao))?;
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, Some(vbo))?;
    rgl::buffer_data(
        rgl::BufferBindingTarget::Array,
        &[
            // front
            [-0.5f32, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [-0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // left
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, -0.5, 0.5],
            // top
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
            [0.5, 0.5, 0.5],
            // right
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
            // bottom
            [0.5, -0.5, 0.5],
            [0.5, -0.5, -0.5],
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            // back
            [0.5, -0.5, -0.5],
            [0.5, 0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, -0.5, -0.5],
        ],
        rgl::BufferUsage(
            rgl::BufferUsageFrequency::Static,
            rgl::BufferUsageNature::Draw,
        ),
    )?;
    rgl::enable_vertex_attribute_array(0)?;
    rgl::vertex_attribute_float_pointer(
        0,
        rgl::VertexAttributeSize::Triple,
        rgl::VertexAttributeFloatType::F32,
        false,
        (std::mem::size_of::<f32>() * 3) as u32,
        0,
    )?;
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, None)?;

    const LIGHT_COLOUR: [f32; 3] = [1.0, 1.0, 1.0];
    const LIGHT_SCALE: [f32; 3] = [0.2; 3];
    const LIGHT_POS: [f32; 3] = [1.2, 1.0, 2.0];
    const EYE_POS: [f32; 3] = [1.5, 2.0, 5.0];

    let cube_light_pos = {
        rgl::use_program(cube_shader_program)?;
        let model = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"model\0")?,
        )?;
        let view = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"view\0")?,
        )?;
        let projection = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"projection\0")?,
        )?;
        let object_colour = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"object_colour\0")?,
        )?;
        let light_colour = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"light_colour\0")?,
        )?;
        let light_pos = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"light_pos\0")?,
        )?;
        let view_pos = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"view_pos\0")?,
        )?;

        rgl::uniform_matrix_4f32v(model, true, &[from_glm(&glm::one())])?;
        rgl::uniform_matrix_4f32v(
            view,
            true,
            &[from_glm(&glm::look_at(
                &glm::make_vec3(&EYE_POS),
                &glm::vec3(0.0, 0.0, 0.0),
                &glm::vec3(0.0, 1.0, 0.0),
            ))],
        )?;
        rgl::uniform_matrix_4f32v(
            projection,
            true,
            &[from_glm(&glm::perspective(
                window.size().0 as f32 / window.size().1 as f32,
                45.0f32.to_radians(),
                0.1,
                100.0,
            ))],
        )?;

        rgl::uniform_3f32(object_colour, 1.0, 0.5, 0.31)?;
        rgl::uniform_3f32(
            light_colour,
            LIGHT_COLOUR[0],
            LIGHT_COLOUR[1],
            LIGHT_COLOUR[2],
        )?;
        rgl::uniform_3f32(view_pos, EYE_POS[0], EYE_POS[1], EYE_POS[2])?;

        light_pos
    };

    let light_model = {
        rgl::use_program(light_shader_program)?;
        let model = rgl::get_uniform_location(
            light_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"model\0")?,
        )?;
        let view = rgl::get_uniform_location(
            light_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"view\0")?,
        )?;
        let projection = rgl::get_uniform_location(
            light_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"projection\0")?,
        )?;
        let light_colour = rgl::get_uniform_location(
            light_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"light_colour\0")?,
        )?;

        rgl::uniform_matrix_4f32v(
            model,
            true,
            &[from_glm(&glm::scale(
                &glm::translate(&glm::one(), &glm::make_vec3(&LIGHT_POS)),
                &glm::make_vec3(&LIGHT_SCALE),
            ))],
        )?;
        rgl::uniform_matrix_4f32v(
            view,
            true,
            &[from_glm(&glm::look_at(
                &glm::make_vec3(&EYE_POS),
                &glm::vec3(0.0, 0.0, 0.0),
                &glm::vec3(0.0, 1.0, 0.0),
            ))],
        )?;
        rgl::uniform_matrix_4f32v(
            projection,
            true,
            &[from_glm(&glm::perspective(
                window.size().0 as f32 / window.size().1 as f32,
                45.0f32.to_radians(),
                0.1,
                100.0,
            ))],
        )?;

        rgl::uniform_3f32(
            light_colour,
            LIGHT_COLOUR[0],
            LIGHT_COLOUR[1],
            LIGHT_COLOUR[2],
        )?;

        model
    };

    // ============================================================================================

    let mut event_pump = sdl.event_pump().unwrap();
    let start_time_point = std::time::Instant::now();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        let frame_time_point = std::time::Instant::now();
        let total_duration = frame_time_point - start_time_point;

        rgl::clear(rgl::ClearMask::COLOUR | rgl::ClearMask::DEPTH);

        let light_pos = glm::rotate_vec3(
            &glm::make_vec3(&LIGHT_POS),
            total_duration.as_secs_f32() / 2.0,
            &glm::vec3(0.0, 1.0, 0.0),
        );

        {
            rgl::use_program(cube_shader_program)?;
            rgl::uniform_3f32(cube_light_pos, light_pos[0], light_pos[1], light_pos[2])?;
            rgl::draw_arrays(rgl::RenderPrimitive::Triangles, 0, 36)?;
        }
        {
            rgl::use_program(light_shader_program)?;
            rgl::uniform_matrix_4f32v(
                light_model,
                true,
                &[from_glm(&glm::scale(
                    &glm::translate(&glm::one(), &light_pos),
                    &glm::make_vec3(&LIGHT_SCALE),
                ))],
            )?;
            rgl::draw_arrays(rgl::RenderPrimitive::Triangles, 0, 36)?;
        }

        window.gl_swap_window();
    }

    Ok(())
}

fn from_glm(mat: &glm::Mat4) -> [f32; 16] {
    #[rustfmt::skip]
    return [
        mat.m11, mat.m12, mat.m13, mat.m14,
        mat.m21, mat.m22, mat.m23, mat.m24,
        mat.m31, mat.m32, mat.m33, mat.m34,
        mat.m41, mat.m42, mat.m43, mat.m44,
    ];
}

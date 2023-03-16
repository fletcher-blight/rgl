use nalgebra_glm as glm;
use rgl::prelude as rgl;

fn main() -> anyhow::Result<()> {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
    }
    let window = video
        .window("LearnOpenGL: Materials Ex - Playground", 1920, 1080)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().unwrap();
    rgl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // ============================================================================================

    rgl::enable(rgl::Capability::DepthTest);

    let cube_shader_program = {
        let vertex_shader = rgl::create_shader(rgl::ShaderType::Vertex);
        rgl::shader_source(vertex_shader, include_str!("phong.vert"));
        rgl::compile_shader(vertex_shader);
        assert!(rgl::get_shader_compile_status(vertex_shader));

        let fragment_shader = rgl::create_shader(rgl::ShaderType::Fragment);
        rgl::shader_source(fragment_shader, include_str!("phong.frag"));
        rgl::compile_shader(fragment_shader);
        assert!(rgl::get_shader_compile_status(fragment_shader));

        let shader_program = rgl::create_program();
        rgl::attach_shader(shader_program, vertex_shader);
        rgl::attach_shader(shader_program, fragment_shader);
        rgl::link_program(shader_program);
        assert!(rgl::get_program_link_status(shader_program));

        shader_program
    };

    let light_shader_program = {
        let vertex_shader = rgl::create_shader(rgl::ShaderType::Vertex);
        rgl::shader_source(vertex_shader, include_str!("light.vert"));
        rgl::compile_shader(vertex_shader);
        assert!(rgl::get_shader_compile_status(vertex_shader));

        let fragment_shader = rgl::create_shader(rgl::ShaderType::Fragment);
        rgl::shader_source(fragment_shader, include_str!("light.frag"));
        rgl::compile_shader(fragment_shader);
        assert!(rgl::get_shader_compile_status(fragment_shader));

        let shader_program = rgl::create_program();
        rgl::attach_shader(shader_program, vertex_shader);
        rgl::attach_shader(shader_program, fragment_shader);
        rgl::link_program(shader_program);
        assert!(rgl::get_program_link_status(shader_program));

        shader_program
    };

    let mut vao = Default::default();
    rgl::gen_vertex_arrays(std::slice::from_mut(&mut vao));

    let mut vbo = Default::default();
    rgl::gen_buffers(std::slice::from_mut(&mut vbo));

    rgl::bind_vertex_array(vao);
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, vbo);
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
        rgl::BufferUsageFrequency::Static,
        rgl::BufferUsageNature::Draw,
    );
    rgl::enable_vertex_attrib_array(0);
    rgl::vertex_attrib_float_pointer(
        0,
        rgl::VertexAttribSize::Triple,
        rgl::VertexAttribFloatType::F32,
        false,
        (std::mem::size_of::<f32>() * 3) as u64,
        0,
    );
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, rgl::Buffer(0));

    const LIGHT_SCALE: [f32; 3] = [0.2; 3];
    const LIGHT_POS: [f32; 3] = [5.0, 1.0, 0.0];

    let (
        cube_view,
        cube_view_pos,
        cube_model,
        cube_material_ambient,
        cube_material_diffuse,
        cube_material_specular,
        cube_material_shininess,
        cube_light_pos,
    ) = {
        rgl::use_program(cube_shader_program);
        let model = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"model\0")?,
        );
        let view = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"view\0")?,
        );
        let projection = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"projection\0")?,
        );
        let material_ambient = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"material.ambient\0")?,
        );
        let material_diffuse = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"material.diffuse\0")?,
        );
        let material_specular = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"material.specular\0")?,
        );
        let material_shininess = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"material.shininess\0")?,
        );
        let light_pos = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"light.position\0")?,
        );
        let light_ambient = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"light.ambient\0")?,
        );
        let light_diffuse = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"light.diffuse\0")?,
        );
        let light_specular = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"light.specular\0")?,
        );
        let view_pos = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"view_pos\0")?,
        );

        rgl::uniform_matrix_4f32v(model, rgl::MatrixOrderMajor::Row, &[from_glm(&glm::one())]);
        rgl::uniform_matrix_4f32v(
            projection,
            rgl::MatrixOrderMajor::Row,
            &[from_glm(&glm::perspective(
                window.size().0 as f32 / window.size().1 as f32,
                45.0f32.to_radians(),
                0.1,
                100.0,
            ))],
        );

        rgl::uniform_3f32(light_ambient, 0.2, 0.2, 0.2);
        rgl::uniform_3f32(light_diffuse, 0.5, 0.5, 0.5);
        rgl::uniform_3f32(light_specular, 1.0, 1.0, 1.0);

        (
            view,
            view_pos,
            model,
            material_ambient,
            material_diffuse,
            material_specular,
            material_shininess,
            light_pos,
        )
    };

    let (light_view, light_model) = {
        rgl::use_program(light_shader_program);
        let model = rgl::get_uniform_location(
            light_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"model\0")?,
        );
        let view = rgl::get_uniform_location(
            light_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"view\0")?,
        );
        let projection = rgl::get_uniform_location(
            light_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"projection\0")?,
        );
        let light_colour = rgl::get_uniform_location(
            light_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"light_colour\0")?,
        );

        rgl::uniform_matrix_4f32v(
            model,
            rgl::MatrixOrderMajor::Row,
            &[from_glm(&glm::scale(
                &glm::translate(&glm::one(), &glm::make_vec3(&LIGHT_POS)),
                &glm::make_vec3(&LIGHT_SCALE),
            ))],
        );
        rgl::uniform_matrix_4f32v(
            projection,
            rgl::MatrixOrderMajor::Row,
            &[from_glm(&glm::perspective(
                window.size().0 as f32 / window.size().1 as f32,
                45.0f32.to_radians(),
                0.1,
                100.0,
            ))],
        );

        rgl::uniform_3f32(light_colour, 1.0, 1.0, 1.0);

        (view, model)
    };

    // ============================================================================================

    rgl::clear_colour(0.1, 0.1, 0.1, 0.1);

    let mut eye_pos = [0.0, 2.0, 5.0];

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
                sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Right),
                    ..
                } => {
                    let v = glm::rotate_y_vec3(&glm::make_vec3(&eye_pos), 15.0f32.to_radians());
                    eye_pos = [v[0], v[1], v[2]];
                }
                sdl2::event::Event::KeyUp {
                    keycode: Some(sdl2::keyboard::Keycode::Left),
                    ..
                } => {
                    let v = glm::rotate_y_vec3(&glm::make_vec3(&eye_pos), -15.0f32.to_radians());
                    eye_pos = [v[0], v[1], v[2]];
                }
                _ => {}
            }
        }

        let frame_time_point = std::time::Instant::now();
        let total_duration = frame_time_point - start_time_point;
        let total_duration_secs = total_duration.as_secs_f32();

        rgl::clear(rgl::ClearMask::COLOUR | rgl::ClearMask::DEPTH);

        let light_pos = glm::rotate_vec3(
            &glm::make_vec3(&LIGHT_POS),
            total_duration_secs / 2.0,
            &glm::vec3(0.0, 1.0, 0.0),
        );

        let view = glm::look_at(
            &glm::make_vec3(&eye_pos),
            &glm::vec3(0.0, 0.0, 0.0),
            &glm::vec3(0.0, 1.0, 0.0),
        );

        {
            rgl::use_program(cube_shader_program);
            rgl::uniform_matrix_4f32v(cube_view, rgl::MatrixOrderMajor::Row, &[from_glm(&view)]);
            rgl::uniform_3f32(cube_view_pos, eye_pos[0], eye_pos[1], eye_pos[2]);
            rgl::uniform_3f32(cube_light_pos, light_pos[0], light_pos[1], light_pos[2]);

            // Using http://devernay.free.fr/cours/opengl/materials.html

            // emerald
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(-3.0, -1.0, 2.25),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.0215, 0.1745, 0.0215);
                rgl::uniform_3f32(cube_material_diffuse, 0.07568, 0.61424, 0.07568);
                rgl::uniform_3f32(cube_material_specular, 0.633, 0.727811, 0.633);
                rgl::uniform_1f32(cube_material_shininess, 0.6);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // jade
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(-1.5, -1.0, 2.25),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.135, 0.2225, 0.1575);
                rgl::uniform_3f32(cube_material_diffuse, 0.54, 0.89, 0.63);
                rgl::uniform_3f32(cube_material_specular, 0.316228, 0.316228, 0.316228);
                rgl::uniform_1f32(cube_material_shininess, 0.1);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // obsidian
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(0.0, -1.0, 2.25),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.05375, 0.05, 0.06625);
                rgl::uniform_3f32(cube_material_diffuse, 0.18275, 0.17, 0.22525);
                rgl::uniform_3f32(cube_material_specular, 0.332741, 0.328634, 0.346435);
                rgl::uniform_1f32(cube_material_shininess, 0.3);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // pearl
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(1.5, -1.0, 2.25),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.25, 0.20725, 0.20725);
                rgl::uniform_3f32(cube_material_diffuse, 1.0, 0.829, 0.829);
                rgl::uniform_3f32(cube_material_specular, 0.296648, 0.296648, 0.296648);
                rgl::uniform_1f32(cube_material_shininess, 0.088);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // ruby
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(3.0, -1.0, 2.25),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.1745, 0.01175, 0.012175);
                rgl::uniform_3f32(cube_material_diffuse, 0.61424, 0.04136, 0.04136);
                rgl::uniform_3f32(cube_material_specular, 0.727811, 0.626959, 0.626959);
                rgl::uniform_1f32(cube_material_shininess, 0.6);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // ===================================================================================

            // turqoise
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(-3.0, -1.0, 0.75),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.1, 0.18725, 0.1745);
                rgl::uniform_3f32(cube_material_diffuse, 0.396, 0.74151, 0.69102);
                rgl::uniform_3f32(cube_material_specular, 0.297254, 0.30829, 0.306678);
                rgl::uniform_1f32(cube_material_shininess, 0.1);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // brass
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(-1.5, -1.0, 0.75),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.329412, 0.223529, 0.027451);
                rgl::uniform_3f32(cube_material_diffuse, 0.780392, 0.568627, 0.113725);
                rgl::uniform_3f32(cube_material_specular, 0.992157, 0.941176, 0.807843);
                rgl::uniform_1f32(cube_material_shininess, 0.21794872);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // bronze
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(0.0, -1.0, 0.75),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.2125, 0.1275, 0.054);
                rgl::uniform_3f32(cube_material_diffuse, 0.714, 0.4284, 0.18144);
                rgl::uniform_3f32(cube_material_specular, 0.393548, 0.271906, 0.166721);
                rgl::uniform_1f32(cube_material_shininess, 0.2);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // chrome
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(1.5, -1.0, 0.75),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.25, 0.25, 0.25);
                rgl::uniform_3f32(cube_material_diffuse, 0.4, 0.4, 0.4);
                rgl::uniform_3f32(cube_material_specular, 0.774597, 0.774597, 0.774597);
                rgl::uniform_1f32(cube_material_shininess, 0.6);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // copper
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(3.0, -1.0, 0.75),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.19125, 0.0735, 0.0225);
                rgl::uniform_3f32(cube_material_diffuse, 0.7038, 0.27048, 0.0828);
                rgl::uniform_3f32(cube_material_specular, 0.256777, 0.137622, 0.086014);
                rgl::uniform_1f32(cube_material_shininess, 0.1);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // ===================================================================================

            // gold
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(-3.0, -1.0, -0.75),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.24725, 0.1995, 0.0745);
                rgl::uniform_3f32(cube_material_diffuse, 0.75164, 0.60648, 0.22648);
                rgl::uniform_3f32(cube_material_specular, 0.628281, 0.555802, 0.366065);
                rgl::uniform_1f32(cube_material_shininess, 0.4);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // silver
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(-1.5, -1.0, -0.75),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.19225, 0.19225, 0.19225);
                rgl::uniform_3f32(cube_material_diffuse, 0.50754, 0.50754, 0.50754);
                rgl::uniform_3f32(cube_material_specular, 0.508273, 0.508273, 0.508273);
                rgl::uniform_1f32(cube_material_shininess, 0.4);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // black plastic
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(0.0, -1.0, -0.75),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.0, 0.0, 0.0);
                rgl::uniform_3f32(cube_material_diffuse, 0.01, 0.01, 0.01);
                rgl::uniform_3f32(cube_material_specular, 0.50, 0.50, 0.50);
                rgl::uniform_1f32(cube_material_shininess, 0.25);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // cyan plastic
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(1.5, -1.0, -0.75),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.0, 0.1, 0.06);
                rgl::uniform_3f32(cube_material_diffuse, 0.0, 0.50980392, 0.50980392);
                rgl::uniform_3f32(cube_material_specular, 0.50196078, 0.50196078, 0.50196078);
                rgl::uniform_1f32(cube_material_shininess, 0.25);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // white plastic
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(3.0, -1.0, -0.75),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.0, 0.0, 0.0);
                rgl::uniform_3f32(cube_material_diffuse, 0.55, 0.55, 0.55);
                rgl::uniform_3f32(cube_material_specular, 0.70, 0.70, 0.70);
                rgl::uniform_1f32(cube_material_shininess, 0.25);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // ===================================================================================

            // black rubber
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(-3.0, -1.0, -2.25),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.02, 0.02, 0.02);
                rgl::uniform_3f32(cube_material_diffuse, 0.01, 0.01, 0.01);
                rgl::uniform_3f32(cube_material_specular, 0.4, 0.4, 0.4);
                rgl::uniform_1f32(cube_material_shininess, 0.078125);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // green rubber
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(-1.5, -1.0, -2.25),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.0, 0.05, 0.0);
                rgl::uniform_3f32(cube_material_diffuse, 0.4, 0.5, 0.5);
                rgl::uniform_3f32(cube_material_specular, 0.04, 0.7, 0.04);
                rgl::uniform_1f32(cube_material_shininess, 0.078125);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // red rubber
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(0.0, -1.0, -2.25),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.05, 0.0, 0.0);
                rgl::uniform_3f32(cube_material_diffuse, 0.5, 0.4, 0.4);
                rgl::uniform_3f32(cube_material_specular, 0.7, 0.04, 0.04);
                rgl::uniform_1f32(cube_material_shininess, 0.078125);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // white rubber
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(1.5, -1.0, -2.25),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.05, 0.05, 0.05);
                rgl::uniform_3f32(cube_material_diffuse, 0.5, 0.5, 0.5);
                rgl::uniform_3f32(cube_material_specular, 0.7, 0.7, 0.7);
                rgl::uniform_1f32(cube_material_shininess, 0.078125);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }

            // yellow rubber
            {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    rgl::MatrixOrderMajor::Row,
                    &[from_glm(&glm::translate(
                        &glm::one(),
                        &glm::vec3(3.0, -1.0, -2.25),
                    ))],
                );
                rgl::uniform_3f32(cube_material_ambient, 0.05, 0.05, 0.0);
                rgl::uniform_3f32(cube_material_diffuse, 0.5, 0.5, 0.4);
                rgl::uniform_3f32(cube_material_specular, 0.7, 0.7, 0.04);
                rgl::uniform_1f32(cube_material_shininess, 0.078125);
                rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
            }
        }
        {
            rgl::use_program(light_shader_program);
            rgl::uniform_matrix_4f32v(light_view, rgl::MatrixOrderMajor::Row, &[from_glm(&view)]);
            rgl::uniform_matrix_4f32v(
                light_model,
                rgl::MatrixOrderMajor::Row,
                &[from_glm(&glm::scale(
                    &glm::translate(&glm::one(), &light_pos),
                    &glm::make_vec3(&LIGHT_SCALE),
                ))],
            );
            rgl::uniform_matrix_4f32v(
                light_model,
                rgl::MatrixOrderMajor::Row,
                &[from_glm(&glm::scale(
                    &glm::translate(&glm::one(), &light_pos),
                    &glm::make_vec3(&LIGHT_SCALE),
                ))],
            );
            rgl::draw_arrays(rgl::DrawMode::Triangles, 0, 36);
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

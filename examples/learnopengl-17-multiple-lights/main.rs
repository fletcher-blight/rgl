use nalgebra_glm as glm;

fn main() -> anyhow::Result<()> {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
    }
    sdl.mouse().set_relative_mouse_mode(true);
    let window = video
        .window("LearnOpenGL: Multiple Lights", 1920, 1080)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().unwrap();
    rgl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // ============================================================================================

    rgl::enable(rgl::Capability::DepthTest);

    let cube_shader_program = {
        let vertex_shader = rgl::create_shader(rgl::ShaderType::Vertex).unwrap();
        rgl::shader_source(vertex_shader, include_bytes!("cube.vert"))?;
        rgl::compile_shader(vertex_shader)?;
        assert!(rgl::get_shader_compile_status(vertex_shader)?);

        let fragment_shader = rgl::create_shader(rgl::ShaderType::Fragment).unwrap();
        rgl::shader_source(fragment_shader, include_bytes!("cube.frag"))?;
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
    rgl::use_program(cube_shader_program)?;

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

    let diffuse_texture = {
        let texture = rgl::gen_texture();
        rgl::bind_texture(rgl::TextureBindingTarget::Image2D, Some(texture))?;
        rgl::texture_target_wrap(
            rgl::TextureBindingTarget::Image2D,
            rgl::TextureWrapTarget::S,
            rgl::TextureWrapMode::Repeat,
        )?;
        rgl::texture_target_wrap(
            rgl::TextureBindingTarget::Image2D,
            rgl::TextureWrapTarget::T,
            rgl::TextureWrapMode::Repeat,
        )?;
        rgl::texture_target_min_filter(
            rgl::TextureBindingTarget::Image2D,
            rgl::TextureMinFilter::Linear,
        )?;
        rgl::texture_target_mag_filter(
            rgl::TextureBindingTarget::Image2D,
            rgl::TextureMagFilter::Linear,
        )?;

        let image = image::open("./assets/container2.png")?.flipv();
        rgl::texture_image_2d(
            rgl::Texture2DTarget::Image2D,
            0,
            rgl::TextureInternalFormat::RGB,
            image.width(),
            image.height(),
            rgl::TextureFormat::RGBA,
            rgl::TexturePixelDataType::U8,
            image.as_bytes(),
        )?;
        let loc = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"material.diffuse\0")?,
        )?;
        rgl::uniform_1i32(loc, 0)?;
        texture
    };

    let specular_texture = {
        let texture = rgl::gen_texture();
        rgl::bind_texture(rgl::TextureBindingTarget::Image2D, Some(texture))?;
        rgl::texture_target_wrap(
            rgl::TextureBindingTarget::Image2D,
            rgl::TextureWrapTarget::S,
            rgl::TextureWrapMode::Repeat,
        )?;
        rgl::texture_target_wrap(
            rgl::TextureBindingTarget::Image2D,
            rgl::TextureWrapTarget::T,
            rgl::TextureWrapMode::Repeat,
        )?;
        rgl::texture_target_min_filter(
            rgl::TextureBindingTarget::Image2D,
            rgl::TextureMinFilter::Linear,
        )?;
        rgl::texture_target_mag_filter(
            rgl::TextureBindingTarget::Image2D,
            rgl::TextureMagFilter::Linear,
        )?;

        let image = image::open("./assets/container2_specular.png")?.flipv();
        rgl::texture_image_2d(
            rgl::Texture2DTarget::Image2D,
            0,
            rgl::TextureInternalFormat::RGB,
            image.width(),
            image.height(),
            rgl::TextureFormat::RGBA,
            rgl::TexturePixelDataType::U8,
            image.as_bytes(),
        )?;
        let loc = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"material.specular\0")?,
        )?;
        rgl::uniform_1i32(loc, 1)?;
        texture
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

    rgl::use_program(light_shader_program)?;
    let light_model = rgl::get_uniform_location(
        light_shader_program,
        std::ffi::CStr::from_bytes_with_nul(b"model\0")?,
    )?;
    let light_view = rgl::get_uniform_location(
        light_shader_program,
        std::ffi::CStr::from_bytes_with_nul(b"view\0")?,
    )?;
    let light_projection = rgl::get_uniform_location(
        light_shader_program,
        std::ffi::CStr::from_bytes_with_nul(b"projection\0")?,
    )?;

    rgl::use_program(cube_shader_program)?;
    let cube_model = rgl::get_uniform_location(
        cube_shader_program,
        std::ffi::CStr::from_bytes_with_nul(b"model\0")?,
    )?;
    let cube_view = rgl::get_uniform_location(
        cube_shader_program,
        std::ffi::CStr::from_bytes_with_nul(b"view\0")?,
    )?;
    let cube_projection = rgl::get_uniform_location(
        cube_shader_program,
        std::ffi::CStr::from_bytes_with_nul(b"projection\0")?,
    )?;
    let cube_view_position = rgl::get_uniform_location(
        cube_shader_program,
        std::ffi::CStr::from_bytes_with_nul(b"view_position\0")?,
    )?;

    {
        let material_shininess = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"material.shininess\0")?,
        )?;
        rgl::uniform_1f32(material_shininess, 32.0)?;
    }

    // Directional Lights
    let cube_enable_directionals = rgl::get_uniform_location(
        cube_shader_program,
        std::ffi::CStr::from_bytes_with_nul(b"enable_directionals\0")?,
    )?;
    {
        for i in 0..2 {
            let light_direction = rgl::get_uniform_location(
                cube_shader_program,
                std::ffi::CString::new(format!("directional_lights[{i}].direction"))?.as_c_str(),
            )?;
            let light_ambient = rgl::get_uniform_location(
                cube_shader_program,
                std::ffi::CString::new(format!("directional_lights[{i}].ambient"))?.as_c_str(),
            )?;
            let light_diffuse = rgl::get_uniform_location(
                cube_shader_program,
                std::ffi::CString::new(format!("directional_lights[{i}].diffuse"))?.as_c_str(),
            )?;
            let light_specular = rgl::get_uniform_location(
                cube_shader_program,
                std::ffi::CString::new(format!("directional_lights[{i}].specular"))?.as_c_str(),
            )?;

            rgl::uniform_3f32(light_ambient, 0.01, 0.01, 0.01)?;
            rgl::uniform_3f32(light_diffuse, 0.4, 0.4, 0.4)?;
            rgl::uniform_3f32(light_specular, 1.0, 1.0, 1.0)?;

            match i {
                0 => {
                    rgl::uniform_3f32(light_direction, -1.0, -1.0, 0.0)?;
                }
                1 => {
                    rgl::uniform_3f32(light_direction, 1.0, -1.0, 0.0)?;
                }
                _ => {}
            }
        }
    }

    // point lights
    let point_light_2_position = glm::vec3(20.0, -10.0, -20.0);
    let point_light_3_position = glm::vec3(-20.0, -10.0, -20.0);
    let cube_enable_point_lights = rgl::get_uniform_location(
        cube_shader_program,
        std::ffi::CStr::from_bytes_with_nul(b"enable_point_lights\0")?,
    )?;
    {
        for i in 0..4 {
            let light_ambient = rgl::get_uniform_location(
                cube_shader_program,
                std::ffi::CString::new(format!("point_lights[{i}].ambient"))?.as_c_str(),
            )?;
            let light_diffuse = rgl::get_uniform_location(
                cube_shader_program,
                std::ffi::CString::new(format!("point_lights[{i}].diffuse"))?.as_c_str(),
            )?;
            let light_specular = rgl::get_uniform_location(
                cube_shader_program,
                std::ffi::CString::new(format!("point_lights[{i}].specular"))?.as_c_str(),
            )?;
            let light_attenuation_constant = rgl::get_uniform_location(
                cube_shader_program,
                std::ffi::CString::new(format!("point_lights[{i}].attenuation.constant"))?
                    .as_c_str(),
            )?;
            let light_attenuation_linear = rgl::get_uniform_location(
                cube_shader_program,
                std::ffi::CString::new(format!("point_lights[{i}].attenuation.linear"))?.as_c_str(),
            )?;
            let light_attenuation_quadratic = rgl::get_uniform_location(
                cube_shader_program,
                std::ffi::CString::new(format!("point_lights[{i}].attenuation.quadratic"))?
                    .as_c_str(),
            )?;

            rgl::uniform_1f32(light_attenuation_constant, 1.0)?;
            rgl::uniform_3f32(light_ambient, 0.2, 0.2, 0.2)?;
            rgl::uniform_3f32(light_diffuse, 0.5, 0.5, 0.5)?;
            rgl::uniform_3f32(light_specular, 1.0, 1.0, 1.0)?;

            match i {
                0 | 1 => {
                    rgl::uniform_1f32(light_attenuation_linear, 0.07)?;
                    rgl::uniform_1f32(light_attenuation_quadratic, 0.017)?;
                }
                _ => {
                    rgl::uniform_1f32(light_attenuation_linear, 0.027)?;
                    rgl::uniform_1f32(light_attenuation_quadratic, 0.0028)?;
                }
            }
        }

        let light_2_position = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"point_lights[2].position\0")?,
        )?;
        rgl::uniform_3f32(
            light_2_position,
            point_light_2_position[0],
            point_light_2_position[1],
            point_light_2_position[2],
        )?;

        let light_3_position = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"point_lights[3].position\0")?,
        )?;
        rgl::uniform_3f32(
            light_3_position,
            point_light_3_position[0],
            point_light_3_position[1],
            point_light_3_position[2],
        )?;
    };

    // flashlight
    let cube_enable_spotlight = rgl::get_uniform_location(
        cube_shader_program,
        std::ffi::CStr::from_bytes_with_nul(b"enable_spotlight\0")?,
    )?;
    let cube_spotlight_position = rgl::get_uniform_location(
        cube_shader_program,
        std::ffi::CStr::from_bytes_with_nul(b"spotlight.position\0")?,
    )?;
    let cube_spotlight_direction = rgl::get_uniform_location(
        cube_shader_program,
        std::ffi::CStr::from_bytes_with_nul(b"spotlight.direction\0")?,
    )?;
    {
        let light_cutoff = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"spotlight.cutoff\0")?,
        )?;
        let light_outer_cutoff = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"spotlight.outer_cutoff\0")?,
        )?;
        let light_ambient = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"spotlight.ambient\0")?,
        )?;
        let light_diffuse = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"spotlight.diffuse\0")?,
        )?;
        let light_specular = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"spotlight.specular\0")?,
        )?;
        let light_attenuation_constant = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"spotlight.attenuation.constant\0")?,
        )?;
        let light_attenuation_linear = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"spotlight.attenuation.linear\0")?,
        )?;
        let light_attenuation_quadratic = rgl::get_uniform_location(
            cube_shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"spotlight.attenuation.quadratic\0")?,
        )?;

        rgl::uniform_1f32(light_cutoff, 12.5f32.to_radians().cos())?;
        rgl::uniform_1f32(light_outer_cutoff, 17.5f32.to_radians().cos())?;
        rgl::uniform_1f32(light_attenuation_constant, 1.0)?;
        rgl::uniform_1f32(light_attenuation_linear, 0.07)?;
        rgl::uniform_1f32(light_attenuation_quadratic, 0.017)?;
        rgl::uniform_3f32(light_ambient, 0.0, 0.0, 0.0)?;
        rgl::uniform_3f32(light_diffuse, 0.8, 0.8, 0.8)?;
        rgl::uniform_3f32(light_specular, 1.0, 1.0, 1.0)?;
    }

    rgl::clear_colour(0.1, 0.1, 0.1, 0.1);

    // ============================================================================================

    let cube_positions = [
        [0.0f32, 0.0, 0.0],
        [2.0, 5.0, -15.0],
        [-1.5, -2.2, -2.5],
        [-3.8, -2.0, -12.3],
        [2.4, -0.4, -3.5],
        [-1.7, 3.0, -7.5],
        [1.3, -2.0, -2.5],
        [1.5, 2.0, -2.5],
        [1.5, 0.2, -1.5],
        [-1.3, 1.0, -1.5],
    ];

    let mut camera_position = glm::vec3(0.0, 0.0, 3.0f32);
    let camera_up = glm::vec3(0.0, 1.0, 0.0);

    let mut spotlight_enabled = true;
    let mut directionals_enabled = true;
    let mut point_lights_enabled = true;

    const MOUSE_SENSITIVITY: f32 = 0.1;
    let mut yaw = -90.0f32;
    let mut pitch = 0.0f32;
    let mut direction_request = [0f32; 4];

    let mut event_pump = sdl.event_pump().unwrap();
    let mut last_frame_time_point = std::time::Instant::now();
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
                    keycode: Some(keycode),
                    ..
                } => {
                    match keycode {
                        sdl2::keyboard::Keycode::W => direction_request[0] = 0.0,
                        sdl2::keyboard::Keycode::S => direction_request[1] = 0.0,
                        sdl2::keyboard::Keycode::A => direction_request[2] = 0.0,
                        sdl2::keyboard::Keycode::D => direction_request[3] = 0.0,
                        _ => {}
                    };
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    match keycode {
                        sdl2::keyboard::Keycode::W => direction_request[0] = 1.0,
                        sdl2::keyboard::Keycode::S => direction_request[1] = 1.0,
                        sdl2::keyboard::Keycode::A => direction_request[2] = 1.0,
                        sdl2::keyboard::Keycode::D => direction_request[3] = 1.0,
                        sdl2::keyboard::Keycode::F => spotlight_enabled = !spotlight_enabled,
                        sdl2::keyboard::Keycode::P => point_lights_enabled = !point_lights_enabled,
                        sdl2::keyboard::Keycode::O => directionals_enabled = !directionals_enabled,
                        _ => {}
                    };
                }
                sdl2::event::Event::MouseMotion { xrel, yrel, .. } => {
                    yaw += MOUSE_SENSITIVITY * (xrel as f32);
                    pitch -= MOUSE_SENSITIVITY * (yrel as f32);
                    pitch = pitch.clamp(-89.0, 89.0);
                }
                _ => {}
            }
        }

        let frame_time_point = std::time::Instant::now();
        let frame_duration = frame_time_point - last_frame_time_point;
        last_frame_time_point = frame_time_point;
        let total_duration = frame_time_point - start_time_point;

        let camera_front = glm::normalize(&glm::vec3(
            yaw.to_radians().cos() * pitch.to_radians().cos(),
            pitch.to_radians().sin(),
            yaw.to_radians().sin() * pitch.to_radians().cos(),
        ));

        {
            let camera_right = glm::normalize(&glm::cross(&camera_front, &camera_up));
            let camera_position_diff = (camera_right
                * (direction_request[3] - direction_request[2]))
                + (camera_front * (direction_request[0] - direction_request[1]));

            camera_position += 5.0 * frame_duration.as_secs_f32() * camera_position_diff;
        }

        let point_light_0_position = glm::rotate_vec3(
            &glm::vec3(0.0, 5.0, 0.0),
            total_duration.as_secs_f32() / 2.0,
            &glm::vec3(1.0, 0.0, 0.0),
        );

        let point_light_1_position = glm::rotate_vec3(
            &glm::vec3(5.0, 0.0, 0.0),
            total_duration.as_secs_f32() / 2.0,
            &glm::vec3(0.0, 1.0, 0.0),
        );

        let view = glm::look_at(
            &camera_position,
            &(camera_position + camera_front),
            &camera_up,
        );
        let projection = glm::perspective(
            window.size().0 as f32 / window.size().1 as f32,
            45.0f32.to_radians(),
            0.1,
            100.0,
        );

        rgl::clear(rgl::ClearMask::COLOUR | rgl::ClearMask::DEPTH);

        {
            rgl::use_program(cube_shader_program)?;
            rgl::bind_vertex_array(Some(vao))?;

            rgl::uniform_1i32(cube_enable_spotlight, spotlight_enabled as i32)?;
            rgl::uniform_1i32(cube_enable_directionals, directionals_enabled as i32)?;
            rgl::uniform_1i32(cube_enable_point_lights, point_lights_enabled as i32)?;

            rgl::uniform_3f32(
                cube_spotlight_position,
                camera_position[0],
                camera_position[1],
                camera_position[2],
            )?;

            rgl::uniform_3f32(
                cube_spotlight_direction,
                camera_front[0],
                camera_front[1],
                camera_front[2],
            )?;

            {
                let light_0_position = rgl::get_uniform_location(
                    cube_shader_program,
                    std::ffi::CStr::from_bytes_with_nul(b"point_lights[0].position\0")?,
                )?;
                let light_1_position = rgl::get_uniform_location(
                    cube_shader_program,
                    std::ffi::CStr::from_bytes_with_nul(b"point_lights[1].position\0")?,
                )?;

                rgl::uniform_3f32(
                    light_0_position,
                    point_light_0_position[0],
                    point_light_0_position[1],
                    point_light_0_position[2],
                )?;

                rgl::uniform_3f32(
                    light_1_position,
                    point_light_1_position[0],
                    point_light_1_position[1],
                    point_light_1_position[2],
                )?;
            }

            rgl::uniform_matrix_4f32v(cube_view, true, &[from_glm(&view)])?;
            rgl::uniform_matrix_4f32v(cube_projection, true, &[from_glm(&projection)])?;
            rgl::uniform_3f32(
                cube_view_position,
                camera_position[0],
                camera_position[1],
                camera_position[2],
            )?;

            rgl::active_texture(0)?;
            rgl::bind_texture(rgl::TextureBindingTarget::Image2D, Some(diffuse_texture))?;
            rgl::active_texture(1)?;
            rgl::bind_texture(rgl::TextureBindingTarget::Image2D, Some(specular_texture))?;

            for (i, cube_position) in cube_positions.iter().enumerate() {
                rgl::uniform_matrix_4f32v(
                    cube_model,
                    true,
                    &[from_glm(&glm::rotate(
                        &glm::translate(&glm::one(), &glm::make_vec3(cube_position)),
                        (i as f32 * 20.0).to_radians(),
                        &glm::vec3(1.0, 0.3, 0.5),
                    ))],
                )?;
                rgl::draw_arrays(rgl::RenderPrimitive::Triangles, 0, 36)?;
            }
        }

        if point_lights_enabled {
            rgl::use_program(light_shader_program)?;
            rgl::bind_vertex_array(Some(vao))?;
            rgl::uniform_matrix_4f32v(light_view, true, &[from_glm(&view)])?;
            rgl::uniform_matrix_4f32v(light_projection, true, &[from_glm(&projection)])?;

            for (i, position) in [
                point_light_0_position,
                point_light_1_position,
                point_light_2_position,
                point_light_3_position,
            ]
            .iter()
            .enumerate()
            {
                let scale = if i > 1 { 2.0 } else { 0.2 };
                rgl::uniform_matrix_4f32v(
                    light_model,
                    true,
                    &[from_glm(&glm::scale(
                        &glm::translate(&glm::one(), position),
                        &glm::vec3(scale, scale, scale),
                    ))],
                )?;
                rgl::draw_arrays(rgl::RenderPrimitive::Triangles, 0, 36)?;
            }
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

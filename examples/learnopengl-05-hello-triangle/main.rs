fn main() -> anyhow::Result<()> {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
    }
    let window = video
        .window("LearnOpenGL: Hello Triangle", 1920, 1080)
        .opengl()
        .resizable()
        .build()?;

    let _gl_context = window.gl_create_context().unwrap();
    rgl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // ============================================================================================

    let vertex_shader = rgl::create_shader(rgl::ShaderType::Vertex).unwrap();
    rgl::shader_source(vertex_shader, include_bytes!("shader.vert"))?;
    rgl::compile_shader(vertex_shader)?;
    assert!(rgl::get_shader_compile_status(vertex_shader)?);

    let fragment_shader = rgl::create_shader(rgl::ShaderType::Fragment).unwrap();
    rgl::shader_source(fragment_shader, include_bytes!("shader.frag"))?;
    rgl::compile_shader(fragment_shader)?;
    assert!(rgl::get_shader_compile_status(fragment_shader)?);

    let shader_program = rgl::create_program().unwrap();
    rgl::attach_shader(shader_program, vertex_shader)?;
    rgl::attach_shader(shader_program, fragment_shader)?;
    rgl::link_program(shader_program)?;
    assert!(rgl::get_program_link_status(shader_program)?);

    let vao = rgl::gen_vertex_array();
    let vbo = rgl::gen_buffer();

    rgl::bind_vertex_array(Some(vao))?;
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, Some(vbo))?;
    rgl::buffer_data(
        rgl::BufferBindingTarget::Array,
        &[[-0.5, -0.5], [0.0, 0.5], [0.5, -0.5f32]],
        rgl::BufferUsage(
            rgl::BufferUsageFrequency::Static,
            rgl::BufferUsageNature::Draw,
        ),
    )?;
    rgl::enable_vertex_attribute_array(0)?;
    rgl::vertex_attribute_float_pointer(
        0,
        rgl::VertexAttributeSize::Duple,
        rgl::VertexAttributeFloatType::F32,
        false,
        (std::mem::size_of::<f32>() * 2) as u32,
        0,
    )?;
    rgl::bind_buffer(rgl::BufferBindingTarget::Array, None)?;

    // ============================================================================================

    let mut event_pump = sdl.event_pump().unwrap();
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

        rgl::clear(rgl::ClearMask::COLOUR | rgl::ClearMask::DEPTH);
        rgl::use_program(shader_program)?;
        rgl::bind_vertex_array(Some(vao))?;
        rgl::draw_arrays(rgl::RenderPrimitive::Triangles, 0, 3)?;
        window.gl_swap_window();
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
    }
    let window = video
        .window("LearnOpenGL: Textures", 1920, 1080)
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
    rgl::use_program(shader_program)?;

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

    let texture1 = {
        let texture1 = rgl::gen_texture();
        rgl::active_texture(0)?;
        rgl::bind_texture(rgl::TextureBindingTarget::Image2D, Some(texture1))?;
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

        let image = image::open("./assets/splatoon-face.png")?.flipv();
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
            shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"tex1\0")?,
        )?;
        rgl::uniform_1i32(loc, 0)?;
        texture1
    };

    let texture2 = {
        let texture2 = rgl::gen_texture();
        rgl::active_texture(1)?;
        rgl::bind_texture(rgl::TextureBindingTarget::Image2D, Some(texture2))?;
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
            shader_program,
            std::ffi::CStr::from_bytes_with_nul(b"tex2\0")?,
        )?;
        rgl::uniform_1i32(loc, 1)?;
        texture2
    };

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
        rgl::active_texture(0)?;
        rgl::bind_texture(rgl::TextureBindingTarget::Image2D, Some(texture1))?;
        rgl::active_texture(1)?;
        rgl::bind_texture(rgl::TextureBindingTarget::Image2D, Some(texture2))?;
        rgl::bind_vertex_array(Some(vao))?;
        rgl::draw_arrays(rgl::RenderPrimitive::Triangles, 0, 3)?;
        window.gl_swap_window();
    }

    Ok(())
}

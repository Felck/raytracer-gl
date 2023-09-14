extern crate gl;
extern crate sdl2;

mod gl_wrapper;
mod scene;
mod vec3;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

use crate::scene::Scene;

fn main() {
    // sdl
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 2);

    let window = video_subsystem
        .window("Window", 1600, 900)
        .opengl()
        .build()
        .unwrap();

    // gl context

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    // shader program

    use std::ffi::CString;
    let vert_shader = gl_wrapper::Shader::from_source(
        &CString::new(include_str!("shaders/vert.glsl")).unwrap(),
        gl::VERTEX_SHADER,
    );

    let frag_shader = gl_wrapper::Shader::from_source(
        &CString::new(include_str!("shaders/frag.glsl")).unwrap(),
        gl::FRAGMENT_SHADER,
    );

    let shader_program = gl_wrapper::Program::from_shaders(&[vert_shader, frag_shader]);

    // vertex buffer object

    type Vertex = [f32; 2];

    let vertices: [Vertex; 6] = [
        [-1.0, 1.0],
        [-1.0, -1.0],
        [1.0, 1.0],
        [1.0, 1.0],
        [-1.0, -1.0],
        [1.0, -1.0],
    ];

    let vbo = gl_wrapper::ArrayBuffer::new(std::mem::size_of_val(&vertices), vertices.as_ptr());

    // vertex array object

    let vao = gl_wrapper::VertexArray::new(0, 2, std::mem::size_of::<Vertex>(), vbo.id);

    // uniform buffer object

    let scene = Scene::new();

    let _ubo = gl_wrapper::UniformBuffer::new(0, std::mem::size_of_val(&scene), &scene);

    // main loop

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // draw

        shader_program.use_program();
        unsafe {
            gl::BindVertexArray(vao.id);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                6,             // number of indices to be rendered
            );
        }

        window.gl_swap_window();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}

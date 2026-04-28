mod gl_wraper;
mod program;
mod shader;
mod window;

use gl_wraper::*;
use std::time::Instant;

use crate::{program::Program, shader::*};
use sdl2::*;

fn main() {
    let sdl = sdl2::init().expect("not sdl ?");
    let video = sdl.video().expect("no video ?");
    let _event = sdl.event().expect("no event ?");
    let mut event_pump = sdl.event_pump().expect("no event pump ?");

    video.gl_load_library_default().expect("no opengl ?");

    gl::load_with(|f_name| video.gl_get_proc_address(f_name) as *const _);

    unsafe { gl::Viewport(0, 0, 800, 900) };
    let mut win = window::get_window(&video).expect("no window ?");
    let context = win.gl_create_context().expect("no context ?");
    win.gl_make_current(&context).expect("no gl_make_current ?");

    video
        .gl_set_swap_interval(video::SwapInterval::VSync)
        .expect("no vsync ?");
    unsafe { gl::ClearColor(0.3, 0.3, 0.3, 1.) };

    let frag_shader = Shader::new(gl::FRAGMENT_SHADER)
        .expect("no shader ?")
        .source_file("./shaders/fragment.glsl")
        .expect("no file ?")
        .compile()
        .status()
        .unwrap();
    let vert_shader = Shader::new(gl::VERTEX_SHADER)
        .expect("no shader ?")
        .source_file("./shaders/vertex.glsl")
        .expect("no file ?")
        .compile()
        .status()
        .unwrap();

    let program = Program::new()
        .expect("no program ?")
        .attach_shader(&frag_shader)
        .attach_shader(&vert_shader)
        .link()
        .status()
        .unwrap()
        .detach_shader(&frag_shader)
        .attach_shader(&vert_shader);

    program.r#use();

    let loc = unsafe { gl::GetUniformLocation(program.0, "transform".as_ptr().cast()) };
    let transform = [
        [1.0f32, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    unsafe {
        gl::UniformMatrix4fv(loc, 1, gl::FALSE, transform.as_ptr() as *const _);
    }
    let vertices: Vec<f32> = vec![
        // positions      // colors
        0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
    ];
    let indices: Vec<u32> = vec![0, 1, 3];

    let vao = VertexArray::new().expect("Couldn't make a VAO");
    vao.bind();
    let vbo = Buffer::new().expect("Couldn't make a VBO");
    vbo.bind(BufferType::Array);
    buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(vertices.as_slice()),
        gl::STATIC_DRAW,
    );

    let ebo = Buffer::new().expect("no buffer?");
    ebo.bind(BufferType::ElementArray);
    buffer_data(
        BufferType::ElementArray,
        bytemuck::cast_slice(indices.as_slice()),
        gl::STATIC_DRAW,
    );
    unsafe {
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );
    }

    polygon_mode(PolygonMode::Fill);
    let mut avg = 0.;
    'main_loop: loop {
        // handle events this frame
        while let Some(ev) = event_pump.poll_event() {
            match ev {
                event::Event::Quit { timestamp: _ } => break 'main_loop,
                _ => (),
            }
        }
        let now = Instant::now();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // now the events are clear

        unsafe {
            //gl::UniformMatrix4fv(loc, 1, gl::FALSE, transform.as_ptr() as *const _);
            // gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, 0 as *const _);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        let elapsed_time = now.elapsed();
        let time = 1000000000. / elapsed_time.as_nanos() as f32;
        if avg == 0. {
            avg = time
        };
        avg = (avg * 60. + time) / 61.;
        let _ = win.set_title(format!("{} fps", avg.round()).as_str());
        // here's where we could change the world state and draw.
        win.gl_swap_window();
    }
}

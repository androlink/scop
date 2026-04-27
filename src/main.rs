mod gl_wraper;
mod shader;
mod window;

use gl_wraper::*;
use std::time::Instant;

use crate::shader::*;
use sdl2::*;

fn main() {
    let sdl = sdl2::init().expect("not sdl ?");
    let video = sdl.video().expect("no video ?");
    let _event = sdl.event().expect("no event ?");
    let mut event_pump = sdl.event_pump().expect("no event pump ?");

    video.gl_load_library_default().expect("no opengl ?");

    gl::load_with(|f_name| video.gl_get_proc_address(f_name) as *const _);

    let mut win = window::get_window(&video).expect("no window ?");
    let context = win.gl_create_context().expect("no context ?");
    win.gl_make_current(&context).expect("no gl_make_current ?");

    video
        .gl_set_swap_interval(video::SwapInterval::VSync)
        .expect("no vsync ?");
    unsafe { gl::ClearColor(0., 0., 0., 1.) };

    type TriIndexes = [u32; 3];
    type Vertex = [f32; 3];
    const VERTICES: [Vertex; 4] = [
        [0.5, 0.5, 0.0],
        [0.5, -0.5, 0.0],
        [-0.5, -0.5, 0.0],
        [-0.5, 0.5, 0.0],
    ];
    const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];

    let vao = VertexArray::new().expect("Couldn't make a VAO");
    vao.bind();
    let vbo = Buffer::new().expect("Couldn't make a VBO");
    vbo.bind(BufferType::Array);
    buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(&VERTICES),
        gl::STATIC_DRAW,
    );
    let ebo = Buffer::new().expect("no buffer?");
    ebo.bind(BufferType::ElementArray);
    buffer_data(
        BufferType::ElementArray,
        bytemuck::cast_slice(&INDICES),
        gl::STATIC_DRAW,
    );
    unsafe {
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
    }
    let vertex_shader = load_shader_file(gl::VERTEX_SHADER, "./shaders/vertex.glsl")
        .expect("cannot load vertex shader");
    let frag_shader = load_shader_file(gl::FRAGMENT_SHADER, "./shaders/fragment.glsl")
        .expect("cannot load fragment shader");

    unsafe {
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, frag_shader);
        gl::LinkProgram(shader_program);
    }

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
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        let elapsed_time = now.elapsed();
        let time = 1000000000. / elapsed_time.as_nanos() as f32;
        if avg == 0. {
            avg = time
        };
        avg = (avg * 5000. + time) / 5001.;
        let _ = win.set_title(format!("{} fps", avg.round()).as_str());
        // here's where we could change the world state and draw.
        win.gl_swap_window();
    }
    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(frag_shader);
    }
}

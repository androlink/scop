mod shader;
mod window;
use crate::shader::load_shader;
use sdl2::*;

const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 final_color;

  void main() {
    final_color = vec4(1.0, 0.5, 0.2, 1.0);
  }
"#;

fn main() {
    let sdl = sdl2::init().expect("not sdl ?");
    let video = sdl.video().expect("no video ?");
    let _event = sdl.event().expect("no event ?");
    let mut event_pump = sdl.event_pump().expect("no event pump ?");

    video.gl_load_library_default().expect("no opengl ?");

    gl::load_with(|f_name| video.gl_get_proc_address(f_name) as *const _);

    let win = window::get_window(&video).expect("no window ?");
    let context = win.gl_create_context().expect("no context ?");
    win.gl_make_current(&context).expect("no gl_make_current ?");

    video
        .gl_set_swap_interval(video::SwapInterval::VSync)
        .expect("no vsync ?");
    unsafe { gl::ClearColor(1., 1., 1., 1.) };

    let mut vao: gl::types::GLuint = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao) };
    assert_ne!(vao, 0);
    let mut vbo: gl::types::GLuint = 0;
    unsafe { gl::GenBuffers(1, &mut vbo) };
    assert_ne!(vbo, 0);
    unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vbo) };

    type Vertex = [f32; 3];
    const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

    unsafe {
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        )
    };

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
    let vertex_shader =
        load_shader(gl::VERTEX_SHADER, VERT_SHADER).expect("cannot load vertex shader");
    let frag_shader =
        load_shader(gl::FRAGMENT_SHADER, FRAG_SHADER).expect("cannot load fragment shader");

    unsafe {
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, frag_shader);
        gl::LinkProgram(shader_program);
    }

    'main_loop: loop {
        // handle events this frame
        while let Some(ev) = event_pump.poll_event() {
            match ev {
                event::Event::Quit { timestamp: _ } => break 'main_loop,
                _ => (),
            }
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // now the events are clear

        // here's where we could change the world state and draw.
        win.gl_swap_window();
    }
    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(frag_shader);
    }
}

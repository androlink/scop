mod gl_wraper;
mod object;
mod program;
mod scop_math;
mod shader;
mod vertex;
mod window;

use gl::NO_ERROR;
use gl_wraper::*;
use std::{ptr::null, thread::sleep, time::Instant};

use crate::{object::OBJLoader, program::Program, scop_math::Matrix, shader::*, vertex::*};
use sdl2::*;

fn main() {
    let sdl = sdl2::init().expect("not sdl ?");
    let video = sdl.video().expect("no video ?");
    let _event = sdl.event().expect("no event ?");
    let mut event_pump = sdl.event_pump().expect("no event pump ?");

    video.gl_load_library_default().expect("no opengl ?");

    gl::load_with(|f_name| video.gl_get_proc_address(f_name) as *const _);

    unsafe { gl::Viewport(0, 0, 800, 900) };
    unsafe { gl::Enable(gl::DEPTH_TEST) };
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
        .detach_shader(&vert_shader);

    program.r#use();

    let mut loader = OBJLoader::new();
    loader.path("./resources/");
    let obj = loader
        .load(std::env::args().collect::<Vec<String>>()[1].as_str())
        .expect("no object ?");

    let colors: Vec<SColor> = obj
        .get_verticles()
        .iter()
        .map(|_| SColor(rand::random(), rand::random(), rand::random()))
        .collect();
    // let indices: Vec<SIndice> = vec![SIndice(0, 1, 2), SIndice(1, 2, 3)];

    let vertex_array = VertexArray::new().expect("Couldn't make a VAO");
    vertex_array.bind();
    let vertex_buf: Buffer<Array> = Buffer::<Array>::new().expect("Couldn't make a VBO");
    vertex_buf.bind();
    vertex_buf.data(obj.get_verticles().as_slice(), gl::STATIC_DRAW);

    let color_buf: Buffer<Array> = Buffer::<Array>::new().expect("no colors ?");
    color_buf.bind();
    color_buf.data(colors.as_slice(), gl::STATIC_DRAW);

    let indice_buf: Buffer<Element_Array> = Buffer::<Element_Array>::new().expect("no buffer?");
    indice_buf.bind();
    indice_buf.data(obj.get_vertex_face().as_slice(), gl::STATIC_DRAW);

    unsafe {
        vertex_buf.bind();
        let loc = match gl::GetAttribLocation(program.0, "Position\0".as_ptr().cast()) {
            n if n < 0 => panic!("no loc pos ?"),
            n => n as u32,
        };
        gl::EnableVertexAttribArray(loc);
        gl::VertexAttribPointer(
            loc,
            4,
            gl::FLOAT,
            gl::FALSE,
            0 as gl::types::GLint,
            std::ptr::null(),
        );
        color_buf.bind();
        let loc = match gl::GetAttribLocation(program.0, "Color\0".as_ptr().cast()) {
            n if n < 0 => panic!("no loc col ?"),
            n => n as u32,
        };
        gl::EnableVertexAttribArray(loc);
        gl::VertexAttribPointer(loc, 3, gl::FLOAT, gl::FALSE, 0 as gl::types::GLint, null());
    }
    let mvp_loc = match unsafe { gl::GetUniformLocation(program.0, "Mvp\0".as_ptr().cast()) } {
        n if n < 0 => panic!("no loc MVP ?"),
        n => n as i32,
    };

    // unsafe {
    //     gl::UniformMatrix4fv(loc, 1, gl::FALSE, transform.as_ptr() as *const _);
    // }

    let mut scale_loop = (1..10).cycle();
    let mut translate_loop = (-100..100).cycle();
    polygon_mode(PolygonMode::Line);
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
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        let transform = Matrix::ident();
        let projection = Matrix::perspective(45., 900. / 800., 0.001, 1000.);
        let transform = transform.scale(scale_loop.next().expect("no cycle ?") as f32);
        let transform = transform.translate(
            0.,
            -100.,
            translate_loop.next().expect("no translate ?") as f32,
        );

        let mvp = projection * transform;

        // now the events are clear
        unsafe {
            gl::UniformMatrix4fv(mvp_loc, 1, gl::FALSE, mvp.data.as_ptr() as *const _);
            indice_buf.bind();
            gl::DrawElements(
                gl::TRIANGLES,
                1 * obj.get_vertex_face().len() as gl::types::GLsizei,
                gl::UNSIGNED_INT,
                0 as *const _,
            );
            let val = gl::GetError();
            if val != gl::NO_ERROR {
                println!("gl error {}", val);
            }
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        let elapsed_time = now.elapsed();
        let time = 1000000000. / elapsed_time.as_nanos() as f32;
        if avg == 0. {
            avg = time
        };
        avg = (avg * 100. + time) / 101.;
        let _ = win.set_title(format!("{} fps", avg.round()).as_str());
        // here's where we could change the world state and draw.
        win.gl_swap_window();
    }
}

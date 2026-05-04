mod gl_wraper;
mod obj;
mod scop_mat4;
mod shader;
mod vertex;
mod window;

use gl_wraper::*;
use std::{
    ptr::null,
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{Program, obj::OBJLoader, scop_mat4::Matrix4, shader::*, vertex::*};
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
    unsafe { gl::ClearColor(0.3, 0.3, 0.3, 1.) };

    unsafe { gl::Viewport(0, 0, 800, 900) };
    unsafe { gl::Enable(gl::DEPTH_TEST) };
    unsafe { gl::Enable(gl::DEPTH_CLAMP) };
    unsafe { gl::Enable(gl::CULL_FACE) };
    unsafe { gl::CullFace(gl::BACK) };
    unsafe { gl::FrontFace(gl::CW) };
    unsafe { gl::DepthFunc(gl::LESS) };

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

    // println!("{:#?}", obj);
    rand::random::<f32>();
    let colors: Vec<SColor> = obj
        .get_verticles()
        .iter()
        .map(|_| {
            SColor(
                rand::random::<f32>(),
                rand::random::<f32>(),
                rand::random::<f32>(),
            )
        })
        .collect();
    // let indices: Vec<SIndice> = vec![SIndice(0, 1, 2), SIndice(1, 2, 3)];

    let vertex_array = VertexArray::new().expect("Couldn't make a VAO");
    vertex_array.bind();
    let vertex_buf: Buffer<Array> = Buffer::<Array>::new().expect("Couldn't make a VBO");
    vertex_buf.data(obj.get_verticles().as_slice(), gl::STATIC_DRAW);

    let color_buf: Buffer<Array> = Buffer::<Array>::new().expect("no colors ?");
    color_buf.data(colors.as_slice(), gl::STATIC_DRAW);

    let indice_buf: Buffer<Element_Array> = Buffer::<Element_Array>::new().expect("no buffer?");
    indice_buf.data(obj.get_vertex_indices().as_slice(), gl::STATIC_DRAW);

    unsafe {
        let pos_loc = program
            .get_attribute_location(c"Position")
            .expect("position not found");
        gl::EnableVertexAttribArray(pos_loc);
        vertex_buf.bind();
        gl::VertexAttribPointer(
            pos_loc,
            4,
            gl::FLOAT,
            gl::FALSE,
            0 as gl::types::GLint,
            std::ptr::null(),
        );
        let loc = program
            .get_attribute_location(c"Color")
            .expect("color not found");
        gl::EnableVertexAttribArray(loc);
        color_buf.bind();
        gl::VertexAttribPointer(loc, 3, gl::FLOAT, gl::FALSE, 0 as gl::types::GLint, null());
    }
    let mvp_loc = match unsafe { gl::GetUniformLocation(program.0, "Mvp\0".as_ptr().cast()) } {
        n if n < 0 => panic!("no loc MVP ?"),
        n => n as i32,
    };

    let mut scale_loop = (1..100).cycle();
    let mut translate_z_loop = (10..100).cycle();
    let mut teta_y_loop = (0..200).cycle();
    let mut translate_y_loop = (-100..100).cycle();

    polygon_mode(PolygonMode::Fill);
    'main_loop: loop {
        // handle events this frame
        while let Some(ev) = event_pump.poll_event() {
            match ev {
                event::Event::Quit { timestamp: _ } => break 'main_loop,
                _ => (),
            }
        }
        let now = Instant::now();
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
        let model = Matrix4::ident();
        let translate = Matrix4::translate(0., 0., 0.);
        let rot = Matrix4::rotate_y((teta_y_loop.next().unwrap() as f32 / 200.) * 3.14149 * 2.);
        let scale = Matrix4::scale(2.);
        let model = translate * model;
        let model = rot * model;
        let model = scale * model;
        let projection = Matrix4::perspective(90., 900. / 800., 0.1, 1000.);
        let view = Matrix4::look_at(
            &(10., -10., 10.).into(),
            &(0., 0., 0.).into(),
            &(0., 1., 0.).into(),
        );
        // let transform = transform.translate(
        //     0.,
        //     translate_y_loop.next().expect("no translate ?") as f32,
        //     translate_z_loop.next().expect("no translate ?") as f32,
        // );

        let mvp = model * view * projection;

        // now the events are clear
        unsafe {
            gl::UniformMatrix4fv(mvp_loc, 1, gl::FALSE, mvp.data.as_ptr() as *const _);
            // indice_buf.draw(gl::TRIANGLES, obj.get_vertex_face().len() as i32);

            // vertex_array.draw(gl::TRIANGLES, obj.get_verticles().len() as i32);
            //
            polygon_mode(PolygonMode::Fill);
            indice_buf.draw(gl::TRIANGLES, obj.get_vertex_indices().len() as i32);

            let val = gl::GetError();
            if val != gl::NO_ERROR {
                println!("gl error {}", val);
            }
        }

        let elapsed_time = now.elapsed();
        let time = elapsed_time.as_micros() as f32;
        let _ = win.set_title(format!("{} us per frame", time.to_string()).as_str());
        // here's where we could change the world state and draw.
        sleep(elapsed_time.abs_diff(Duration::from_millis(((1. / 30.) * 1000.) as u64)));
        win.gl_swap_window();
    }
}

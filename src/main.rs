mod gl_wraper;
mod mat4;
mod obj;
mod shader;
mod vertex;
mod window;

use gl_wraper::*;
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{Program, mat4::Matrix4, obj::OBJLoader, shader::*, vertex::*};
use sdl2::{event::WindowEvent, keyboard::Keycode, *};

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

    video.gl_attr().set_context_major_version(3);
    video.gl_attr().set_context_minor_version(3);
    unsafe { gl::Viewport(0, 0, win.size().0 as i32, win.size().1 as i32) };
    unsafe { gl::Enable(gl::DEPTH_TEST) };
    unsafe { gl::Enable(gl::DEPTH_CLAMP) };
    unsafe { gl::Enable(gl::CULL_FACE) };
    unsafe { gl::CullFace(gl::BACK) };
    unsafe { gl::FrontFace(gl::CCW) };
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

    let pos_loc = program.get_attribute_location(c"Position").unwrap();
    pos_loc.enable();
    vertex_buf.bind();
    pos_loc.assign(4, gl::FLOAT);
    let color_loc = program.get_attribute_location(c"Color").unwrap();
    color_loc.enable();
    color_buf.bind();
    color_loc.assign(3, gl::FLOAT);

    let model_loc = program.get_matrix_location(c"model").unwrap();
    let view_loc = program.get_matrix_location(c"view").unwrap();
    let projection_loc = program.get_matrix_location(c"projection").unwrap();

    let mut scale_loop = (1..100).cycle();
    let mut teta_y_loop1 = (0..200).cycle();
    let mut teta_y_loop2 = (0..400).cycle();

    polygon_mode(PolygonMode::Fill);
    'main_loop: loop {
        // handle events this frame
        while let Some(ev) = event_pump.poll_event() {
            match ev {
                event::Event::Quit { .. }
                | event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                event::Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    unsafe { gl::Viewport(0, 0, w, h) };
                }
                _ => (),
            }
        }
        let now = Instant::now();
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
        let model = Matrix4::ident();
        // let translate = Matrix4::translate(0., 0., -1.);
        let rot = Matrix4::rotate_y((teta_y_loop1.next().unwrap() as f32 / 200.) * 3.14149 * 2.);
        let scale = Matrix4::scale(1.);
        // let model = model * translate;
        let model = rot * model;
        // let model = scale * model;
        let projection =
            Matrix4::perspective(90., win.size().1 as f32 / win.size().0 as f32, 0.1, 100.);
        let view = Matrix4::ident();
        let view = Matrix4::look_at(
            &(10., 10., 10.).into(),
            &(00., 00., 00.).into(),
            &(0., 1., 0.).into(),
        );

        model_loc.set(&model);
        view_loc.set(&view);
        projection_loc.set(&projection);
        polygon_mode(PolygonMode::Fill);
        indice_buf.draw(gl::TRIANGLES, obj.get_vertex_indices().len() as i32);
        let model = Matrix4::ident();
        let rot = Matrix4::rotate_y(-(teta_y_loop1.next().unwrap() as f32 / 200.) * 3.14149 * 2.);
        let model = rot * model;
        model_loc.set(&model);
        indice_buf.draw(gl::TRIANGLES, obj.get_vertex_indices().len() as i32);

        let val = unsafe { gl::GetError() };
        if val != gl::NO_ERROR {
            println!("gl error {}", val);
        }

        let elapsed_time = now.elapsed();
        let time = elapsed_time.as_micros() as f32;
        let _ = win.set_title(format!("{} us per frame", time.to_string()).as_str());
        // here's where we could change the world state and draw.
        sleep(elapsed_time.abs_diff(Duration::from_millis(((1. / 30.) * 1000.) as u64)));
        win.gl_swap_window();
    }
}

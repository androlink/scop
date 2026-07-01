mod app;
mod assets;
mod context;
mod graphics;
mod manager;
mod mat4;
mod platform;

mod rendering;

use platform::*;
use std::time::Instant;

use crate::{
    app::App,
    assets::{asset_manager::Assets, texture::bmp_loader::load_bmp},
    mat4::Matrix4,
};

use sdl2::{event::WindowEvent, keyboard::Keycode, *};

fn main() -> Result<(), String> {
    let mut app = App::new().expect("fail to load gl or sdl");
    let mut assets = Assets::new();

    assets
        .load_shaders(&[
            "assets/shaders/default",
            "assets/shaders/funny",
            "assets/shaders/gray_color",
            "assets/shaders/texture",
        ])
        .unwrap();
    assets
        .load_models(&[
            "assets/model/cube.obj",
            "assets/model/teapot.obj",
            "assets/model/teapot2.obj",
            "assets/model/42.obj",
            // "assets/model/bugatti.obj",
            "assets/model/square.obj",
        ])
        .unwrap();
    let cube_mesh =
        graphics::mesh::Mesh::new(assets.mesh("assets/model/cube.obj").unwrap()).unwrap();
    let teapot_mesh =
        graphics::mesh::Mesh::new(assets.mesh("assets/model/teapot.obj").unwrap()).unwrap();
    let teapot2_mesh =
        graphics::mesh::Mesh::new(assets.mesh("assets/model/teapot2.obj").unwrap()).unwrap();
    // let bugatti_mesh =
    //     graphics::mesh::Mesh::new(assets.mesh("assets/model/bugatti.obj").unwrap()).unwrap();
    let ft_mesh = graphics::mesh::Mesh::new(assets.mesh("assets/model/42.obj").unwrap()).unwrap();
    let square =
        graphics::mesh::Mesh::new(assets.mesh("assets/model/square.obj").unwrap()).unwrap();

    let texture_data = load_bmp("assets/textures/default.bmp").unwrap();
    let texture = platform::texture::Texture::new().unwrap();
    texture.generate(
        2,
        2,
        &[
            0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0,
        ],
    );
    // texture.generate(texture_data.width, texture_data.height, &texture_data.data);

    let meshs = [
        &square,
        &cube_mesh,
        &teapot_mesh,
        &teapot2_mesh,
        // &bugatti_mesh,
        &ft_mesh,
    ];

    let mut mesh_loop = meshs.iter().cycle();
    let mut draw_mesh = mesh_loop.next().unwrap();

    let default_shader = assets.shader("assets/shaders/default").unwrap();
    let gray_shader = assets.shader("assets/shaders/gray_color").unwrap();
    let texture_shader = assets.shader("assets/shaders/texture").unwrap();
    let shaders = [&default_shader, &gray_shader, &texture_shader];
    let mut shader_loop = shaders.iter().cycle();
    let mut draw_shader = shader_loop.next().unwrap();
    let time = Instant::now();
    'main_loop: loop {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
        while let Some(ev) = app.platform.event_pump.poll_event() {
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
                event::Event::KeyDown {
                    keycode: Some(Keycode::RETURN),
                    ..
                } => draw_mesh = mesh_loop.next().unwrap(),
                event::Event::KeyDown {
                    keycode: Some(Keycode::SPACE),
                    ..
                } => {
                    draw_shader = shader_loop.next().unwrap();
                    draw_shader.bind();
                }
                _ => (),
            }
        }

        let view = Matrix4::look_at(
            &(0., 0., 10.).into(),
            &(00., 00., 00.).into(),
            &(0., 1., 0.).into(),
        );

        let projection = Matrix4::perspective(
            90.,
            app.platform.window.size().1 as f32 / app.platform.window.size().0 as f32,
            0.1,
            100.,
        );
        draw_shader.set_matrix(c"view", &view);
        draw_shader.set_matrix(c"projection", &projection);

        draw_shader.bind();
        let model = Matrix4::ident();
        let roty = Matrix4::rotate_y((time.elapsed().as_millis() as f32 / 1000. * 0.5).sin() * 2.);
        let rotx = Matrix4::rotate_x((time.elapsed().as_millis() as f32 / 500. * 0.5).sin());
        // let scale = Matrix4::scale((time.elapsed().as_millis() as f32 / 1000.).sin());
        let scale = Matrix4::scale(1.);
        let model = scale * model;
        let model = roty * model;
        let model = rotx * model;
        draw_shader.set_matrix(c"model", &model);
        draw_mesh.draw();

        let val = unsafe { gl::GetError() };
        if val != gl::NO_ERROR {
            println!("gl error {}", val);
        }
        app.platform.window.gl_swap_window();
    }
    Ok(())
}

/*
fn main() {
    let mut app = App::new().expect("fail to load gl or sdl");

    let program = ShaderProgram::init("./shaders/funny.vert", "./shaders/funny.frag").unwrap();

    // loader
    //     .load(std::env::args().collect::<Vec<String>>()[1].as_str())
    //     .expect("no object ?");

    // println!("{:#?}", obj);

    let vertex_array = VertexArray::new().expect("Couldn't make a VAO");
    vertex_array.bind();
    let vertex_buf: Buffer<Array> = Buffer::<Array>::new().expect("Couldn't make a VBO");
    vertex_buf.data(object_buffer.verticles().as_slice(), gl::STATIC_DRAW);

    let indice_buf: Buffer<Element_Array> = Buffer::<Element_Array>::new().expect("no buffer?");
    indice_buf.data(object_buffer.vertex_indices().as_slice(), gl::STATIC_DRAW);

    let pos_loc = program.get_attribute_location(c"aPos").unwrap();
    pos_loc.enable();
    vertex_buf.bind();
    pos_loc.set(4, gl::FLOAT);

    let model_loc = program.get_matrix_location(c"model").unwrap();
    let view_loc = program.get_matrix_location(c"view").unwrap();
    let projection_loc = program.get_matrix_location(c"projection").unwrap();

    let iTimeLoc = program.get_float_location(c"iTime").unwrap();
    let iResolution_loc = program.get_float_location(c"iResolution").unwrap();
    let iMouse_loc = program.get_float_location(c"iMouse").unwrap();
    let i_quality = program.get_float_location(c"iSomething").unwrap();

    let mut model_switch = object_buffer.objects().iter().cycle().peekable();
    let mut scale_loop = (1..100).cycle();
    let mut teta_y_loop1 = (0..200).cycle();
    let mut teta_y_loop2 = (0..400).cycle();
    let ref_time = Instant::now();
    polygon_mode(PolygonMode::Fill);

    iMouse_loc.set4(400., 450., 0., 0.);
    i_quality.set1(150000.);
    iResolution_loc.set3(2000., 2000., 1.0);
    let mut quality_value = (0..500).step_by(50).cycle();
    let mut mouse_pos: (f32, f32) = Default::default();

    'main_loop: loop {
        // handle events this frame
        while let Some(ev) = app.platform.event_pump.poll_event() {
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
                    iResolution_loc.set3(w as _, h as _, 1.0);
                    unsafe { gl::Viewport(0, 0, w, h) };
                }
                event::Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    model_switch.next().expect("ah!?");
                }
                event::Event::KeyDown {
                    keycode: Some(Keycode::KP_ENTER),
                    ..
                } => {
                    i_quality.set1(quality_value.next().unwrap() as f32);
                }
                event::Event::MouseMotion { x, y, .. } => {
                    iMouse_loc.set4(x as _, y as _, 0., 0.);
                }
                _ => (),
            }
        }
        // Pour iTime
        iTimeLoc.set1(ref_time.elapsed().as_millis() as f32 / 1000.);

        // Pour iResolution

        // Pour iMouse

        let now = Instant::now();
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
        let model = Matrix4::ident();
        // let translate = Matrix4::translate(0., 0., -1.);
        let rot = Matrix4::rotate_y((teta_y_loop1.next().unwrap() as f32 / 200.) * 3.14149 * 2.);
        let scale = Matrix4::scale(1.);
        // let model = model * translate;
        let model = rot * model;
        // let model = scale * model;
        let projection = Matrix4::perspective(
            90.,
            app.platform.window.size().1 as f32 / app.platform.window.size().0 as f32,
            0.1,
            100.,
        );
        let view = Matrix4::ident();
        let view = Matrix4::look_at(
            &(10., 10., 10.).into(),
            &(00., 00., 00.).into(),
            &(0., 1., 0.).into(),
        );
        let obj = model_switch.peek().unwrap();

        let scale = Matrix4::scale(5.);
        let model = scale * model;
        model_loc.set(&model);
        view_loc.set(&view);
        projection_loc.set(&projection);
        polygon_mode(PolygonMode::Fill);
        // indice_buf.draw_object(obj);
        indice_buf.draw(gl::TRIANGLES, object_buffer.vertex_indices().len() as i32);
        let model = Matrix4::ident();
        let rot = Matrix4::rotate_x((teta_y_loop1.next().unwrap() as f32 / 200.) * 3.14149 * 2.);
        let translate = Matrix4::translate(0., 0., 10.);
        let model = translate * model;
        let model = rot * model;
        model_loc.set(&model);
        // indice_buf.draw_object(obj);
        indice_buf.draw(gl::TRIANGLES, object_buffer.vertex_indices().len() as i32);
        let model = Matrix4::ident();
        let rot = Matrix4::rotate_z((teta_y_loop1.next().unwrap() as f32 / 200.) * 3.14149 * 2.);
        let translate = Matrix4::translate(10., 0., 0.);
        let model = translate * model;
        let model = rot * model;
        model_loc.set(&model);
        polygon_mode(PolygonMode::Line);
        // indice_buf.draw_object(obj);
        indice_buf.draw(gl::TRIANGLES, object_buffer.vertex_indices().len() as i32);

        let val = unsafe { gl::GetError() };
        if val != gl::NO_ERROR {
            println!("gl error {}", val);
        }

        let elapsed_time = now.elapsed();
        let time = elapsed_time.as_micros() as f32;
        let _ = app.platform.window.set_title(
            format!(
                "{} us per frame: file:{} start: {} size: {}",
                time, obj.name, obj.start, obj.size
            )
            .as_str(),
        );

        // here's where we could change the world state and draw.
        sleep(elapsed_time.abs_diff(Duration::from_millis(((1. / 30.) * 1000.) as u64)));
        app.platform.window.gl_swap_window();
    }
}
*/

mod scop;

use scop::*;

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

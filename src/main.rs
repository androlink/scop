mod window;
use beryllium::{video::*, *};

fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    let win = window::get_window(&sdl);

    unsafe {
        gl::load_with(|f_name| win.get_proc_address(f_name.as_ptr()));
        gl::ClearColor(1., 1., 1., 1.);
    }
    // if let Err(e) = win.set_swap_interval(GlSwapInterval::Vsync) {
    //     panic!("skill issue");
    // }

    'main_loop: loop {
        // handle events this frame
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // now the events are clear

        // here's where we could change the world state and draw.
        win.swap_window();
    }
}

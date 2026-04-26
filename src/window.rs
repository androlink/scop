use beryllium::*;

pub fn get_window(sdl: &Sdl) -> video::GlWindow {
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    let win_args = video::CreateWinArgs {
        title: "scope",
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: true,
    };

    let _win: video::GlWindow = sdl
        .create_gl_window(win_args)
        .expect("couldn't make a window and context");

    _win
}

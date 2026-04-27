use sdl2::*;

pub fn get_window(sdl: &VideoSubsystem) -> Result<video::Window, video::WindowBuildError> {
    sdl.window("scop", 800, 600).opengl().resizable().build()
}

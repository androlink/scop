pub mod buffer;
pub mod location;
pub mod polygone;
pub mod program;
pub mod shader;
pub mod texture;
pub mod vertex_array;

use sdl2::{
    EventPump, Sdl, VideoSubsystem,
    video::{GLContext, SwapInterval, Window, WindowBuildError},
};

pub struct Platform {
    pub sdl: Sdl,
    pub window: Window,
    pub gl_context: GLContext,
    pub event_pump: EventPump,
}

impl Platform {
    pub fn new() -> Result<Self, String> {
        let sdl = sdl2::init().map_err(|e| e.to_string())?;
        let video = sdl.video().map_err(|e| e.to_string())?;
        video.gl_load_library_default().map_err(|e| e.to_string())?;

        gl::load_with(|f_name| video.gl_get_proc_address(f_name) as *const _);
        video.gl_attr().set_context_major_version(3);
        video.gl_attr().set_context_minor_version(3);
        let _event = sdl.event().map_err(|e| e.to_string())?;
        let event_pump = sdl.event_pump().map_err(|e| e.to_string())?;
        let window = Platform::get_window(&video).map_err(|e| e.to_string())?;
        let gl_context = window.gl_create_context().map_err(|e| e.to_string())?;
        window
            .gl_make_current(&gl_context)
            .map_err(|e| e.to_string())?;
        Platform::load_gl(&video).map_err(|e| e.to_string())?;
        Ok(Self {
            sdl,
            window,
            gl_context,
            event_pump,
        })
    }

    fn load_gl(video: &VideoSubsystem) -> Result<(), String> {
        unsafe { gl::ClearColor(0.3, 0.3, 0.3, 1.) };
        unsafe { gl::Enable(gl::DEPTH_TEST) };
        unsafe { gl::Enable(gl::DEPTH_CLAMP) };
        // unsafe { gl::Enable(gl::CULL_FACE) };
        unsafe { gl::CullFace(gl::BACK) };
        unsafe { gl::FrontFace(gl::CCW) };
        unsafe { gl::DepthFunc(gl::LESS) };
        video
            .gl_set_swap_interval(SwapInterval::VSync)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn get_window(sdl: &VideoSubsystem) -> Result<Window, WindowBuildError> {
        sdl.window("scop", 800, 600)
            .opengl()
            .resizable()
            .always_on_top()
            .build()
    }
}

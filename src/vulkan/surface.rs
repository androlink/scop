use ash::{
    Entry, Instance,
    vk::{self, PhysicalDevice, SurfaceKHR},
};
use sdl2::video::Window;

pub fn surface(window: Window) {
    let surface = vk::XlibSurfaceCreateInfoKHR::default().window(window.id() as u64);
}

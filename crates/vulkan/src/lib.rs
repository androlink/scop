use std::error::Error;

use crate::{graphic::Renderer, swapchain::SwapchainState, vulkan_context::VulkanContext};

pub mod graphic;
mod pipeline;
mod swapchain;
mod sync;
mod vulkan_context;

mod anythings;

type UError<T> = Result<T, Box<dyn Error>>;

pub struct VulkanApp {
    context: VulkanContext,
    render: Renderer,
}

impl VulkanApp {
    pub fn new(window: &winit::window::Window) -> Result<Self, Box<dyn Error>> {
        let context = VulkanContext::new(window)?;
        let swapchain = SwapchainState::new(window, &context);
        todo!()
    }
}

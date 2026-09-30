use std::error::Error;

use crate::{
    pipeline::PipelineState, swapchain::SwapchainState, sync::SyncObjects,
    vulkan_context::VulkanContext,
};

pub struct VulkanApp {
    context: VulkanContext,
    render: Renderer,
}

pub struct Renderer {
    pub swapchain: SwapchainState,
    pub pipeline: PipelineState,
    // pub commands: CommandState,
    pub sync: SyncObjects,
}

impl VulkanApp {
    pub fn init(window: &winit::window::Window) -> Result<Self, Box<dyn Error>> {
        let context = VulkanContext::new(window);
        todo!()
    }
}

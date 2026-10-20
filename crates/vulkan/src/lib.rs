use std::error::Error;

use crate::{
    command::Command, pipeline::PipelineState, swapchain::SwapchainState,
    vulkan_context::VulkanContext,
};

mod command;
mod pipeline;
mod swapchain;
mod sync;
mod vulkan_context;

mod anythings;

type UError<T> = Result<T, Box<dyn Error>>;

pub struct VulkanApp {
    context: VulkanContext,
    swapchain: SwapchainState,
    pipeline: PipelineState,
    command: Command,
}

impl VulkanApp {
    pub fn new(window: &winit::window::Window) -> Result<Self, Box<dyn Error>> {
        let context = VulkanContext::new(window)?;
        let swapchain = SwapchainState::new(window, &context)?;
        let pipeline = PipelineState::new(&context.device, swapchain.extent, swapchain.format)?;
        let command = Command::new(&context, &swapchain, &pipeline);
        todo!()
    }
}

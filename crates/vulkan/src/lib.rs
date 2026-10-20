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
        println!("vulkanapp: context finish");
        let swapchain = SwapchainState::new(window, &context)?;
        println!("vulkanapp: swapchain finish");
        let pipeline = PipelineState::new(&context.device, swapchain.extent, swapchain.format)?;
        println!("vulkanapp: pipeline finish");
        let command = Command::new(&context, &swapchain, &pipeline);
        println!("vulkanapp: command finish");
        todo!()
    }
}

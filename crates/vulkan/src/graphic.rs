use std::error::Error;

use crate::{
    pipeline::PipelineState, swapchain::SwapchainState, sync::SyncObjects,
    vulkan_context::VulkanContext,
};

pub struct Renderer {
    pub swapchain: SwapchainState,
    pub pipeline: PipelineState,
    // pub commands: CommandState,
    pub sync: SyncObjects,
}

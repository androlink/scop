use std::error::Error;

use vulkanalia::vk::{self, DeviceV1_0, Handle, HasBuilder, KhrSwapchainExtensionDeviceCommands};

use crate::{
    command::Command, pipeline::PipelineState, swapchain::SwapchainState, sync::SyncObject,
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
    pub fn destroy(&mut self) {
        self.command.destroy(&self.context.device);
        self.pipeline.destroy(&self.context.device);
        self.swapchain.destroy(&self.context.device);
    }
}

impl VulkanApp {
    pub fn new(window: &winit::window::Window) -> Result<Self, Box<dyn Error>> {
        let context = VulkanContext::new(window)?;
        println!("vulkanapp: context finish");
        let swapchain = SwapchainState::new(window, &context)?;
        println!("vulkanapp: swapchain finish");
        let pipeline = PipelineState::new(&context.device, swapchain.extent, swapchain.format)?;
        println!("vulkanapp: pipeline finish");
        let command = Command::new(&context, &swapchain, &pipeline)?;
        println!("vulkanapp: command finish");
        Ok(Self {
            context,
            swapchain,
            pipeline,
            command,
        })
    }
}

impl VulkanApp {
    fn render(&mut self, sync_object: SyncObject) -> UError<()> {
        // Wait until the previous frame has finished.
        unsafe {
            self.context
                .device
                .wait_for_fences(&[sync_object.in_flight], true, u64::MAX)
        }?;

        // Acquire a swapchain image.
        let (image_index, _suboptimal) = unsafe {
            self.context.device.acquire_next_image_khr(
                self.swapchain.swapchain,
                u64::MAX,
                sync_object.image_available,
                vk::Fence::null(),
            )
        }?;

        // Select the command buffer for the acquired image.
        let command_buffer = self.command.command_buffers[image_index as usize];

        let wait_semaphores = &[sync_object.image_available];

        let wait_stages = &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let command_buffers = &[command_buffer];

        let signal_semaphores = &[sync_object.render_finished];

        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores);

        // Reset immediately before submitting new work.
        unsafe { self.context.device.reset_fences(&[sync_object.in_flight]) }?;

        // Submit the command buffer.
        unsafe {
            self.context.device.queue_submit(
                self.context.graphics_queue,
                &[submit_info],
                sync_object.in_flight,
            )
        }?;

        // Prepare presentation.
        let swapchains = &[self.swapchain.swapchain];

        let image_indices = &[image_index];

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(signal_semaphores)
            .swapchains(swapchains)
            .image_indices(image_indices);

        // Present the rendered image.
        unsafe {
            self.context
                .device
                .queue_present_khr(self.context.present_queue, &present_info)
        }?;

        Ok(())
    }
}

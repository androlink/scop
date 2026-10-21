use vulkanalia::{
    Device, Instance,
    vk::{self, DeviceV1_0, HasBuilder},
};

use crate::{
    UError, anythings::QueueFamilyIndices, pipeline::PipelineState, swapchain::SwapchainState,
    vulkan_context::VulkanContext,
};

pub struct Command {
    pub framebuffers: Vec<vk::Framebuffer>,
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,
}

impl Command {
    pub fn destroy(&mut self, device: &Device) {
        for &framebuffer in &self.framebuffers {
            unsafe { device.destroy_framebuffer(framebuffer, None) };
        }
    }
}

impl Command {
    pub fn new(
        context: &VulkanContext,
        swapchain: &SwapchainState,
        pipeline: &PipelineState,
    ) -> UError<Self> {
        let framebuffers = create_framebuffers(
            &context.device,
            pipeline.render_pass,
            &swapchain.image_views,
            swapchain.extent,
        )?;

        let command_pool =
            create_command_pool(&context.instance, &context.device, context.surface)?;

        let command_buffers =
            allocate_command_buffers(&context.device, command_pool, framebuffers.len())?;

        record_command_buffers(
            &context.device,
            &command_buffers,
            &framebuffers,
            pipeline.render_pass,
            pipeline.graphics_pipeline,
            swapchain.extent,
        )?;
        Ok(Self {
            framebuffers,
            command_pool,
            command_buffers,
        })
    }
}

fn create_framebuffers(
    device: &Device,
    render_pass: vk::RenderPass,
    image_views: &[vk::ImageView],
    extent: vk::Extent2D,
) -> UError<Vec<vk::Framebuffer>> {
    let mut framebuffers = Vec::with_capacity(image_views.len());

    for &image_view in image_views {
        let attachments = [image_view];

        let create_info = vk::FramebufferCreateInfo::builder()
            .render_pass(render_pass)
            .attachments(&attachments)
            .width(extent.width)
            .height(extent.height)
            .layers(1);

        let framebuffer = unsafe { device.create_framebuffer(&create_info, None) }?;

        framebuffers.push(framebuffer);
    }

    Ok(framebuffers)
}

fn create_command_pool(
    instance: &Instance,
    device: &Device,
    surface: vk::SurfaceKHR,
) -> UError<vk::CommandPool> {
    let indices = QueueFamilyIndices::get(instance, device.physical_device(), surface)?;

    let create_info = vk::CommandPoolCreateInfo::builder()
        .flags(vk::CommandPoolCreateFlags::empty())
        .queue_family_index(indices.graphics);

    Ok(unsafe { device.create_command_pool(&create_info, None) }?)
}

fn allocate_command_buffers(
    device: &Device,
    command_pool: vk::CommandPool,
    count: usize,
) -> UError<Vec<vk::CommandBuffer>> {
    let command_count =
        u32::try_from(count).map_err(|_| String::from("Too many command buffers"))?;

    let allocate_info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(command_count);

    Ok(unsafe { device.allocate_command_buffers(&allocate_info) }?)
}

fn record_command_buffers(
    device: &Device,
    command_buffers: &[vk::CommandBuffer],
    framebuffers: &[vk::Framebuffer],
    render_pass: vk::RenderPass,
    graphics_pipeline: vk::Pipeline,
    extent: vk::Extent2D,
) -> UError<()> {
    for (index, &command_buffer) in command_buffers.iter().enumerate() {
        let begin_info =
            vk::CommandBufferBeginInfo::builder().flags(vk::CommandBufferUsageFlags::empty());

        unsafe { device.begin_command_buffer(command_buffer, &begin_info) }?;

        // The area of the framebuffer we will render into.
        let render_area = vk::Rect2D::builder()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(extent);

        // Clear the color attachment to opaque black.
        let clear_color = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        };

        let clear_values = [clear_color];

        let render_pass_info = vk::RenderPassBeginInfo::builder()
            .render_pass(render_pass)
            .framebuffer(framebuffers[index])
            .render_area(render_area)
            .clear_values(&clear_values);

        // Begin the render pass.
        unsafe {
            device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_info,
                vk::SubpassContents::INLINE,
            )
        };

        // Select our graphics pipeline.
        unsafe {
            device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                graphics_pipeline,
            )
        };

        // Draw our triangle.
        unsafe {
            device.cmd_draw(
                command_buffer,
                3, // vertex_count
                1, // instance_count
                0, // first_vertex
                0, // first_instance
            )
        };

        // Finish the render pass.
        unsafe { device.cmd_end_render_pass(command_buffer) };

        // Finish recording the command buffer.
        unsafe { device.end_command_buffer(command_buffer) }?;
    }

    Ok(())
}

use vulkanalia::{
    Device,
    vk::{self, DeviceV1_0, Handle, HasBuilder},
};

use crate::UError;

pub struct PipelineState {
    pub render_pass: vk::RenderPass,
    pub pipeline_layout: vk::PipelineLayout,
    pub graphics_pipeline: vk::Pipeline,
}

impl PipelineState {
    pub fn destroy(&mut self, device: &Device) {
        unsafe { device.destroy_pipeline(self.graphics_pipeline, None) };

        unsafe { device.destroy_pipeline_layout(self.pipeline_layout, None) };

        unsafe { device.destroy_render_pass(self.render_pass, None) };
    }
}

impl PipelineState {
    pub fn new(
        device: &Device,
        swapchain_extent: vk::Extent2D,
        swapchain_format: vk::Format,
    ) -> UError<Self> {
        let render_pass = create_render_pass(device, swapchain_format)?;

        let (pipeline_layout, graphics_pipeline) =
            create_graphics_pipeline(device, render_pass, swapchain_extent)?;
        Ok(Self {
            render_pass,
            pipeline_layout,
            graphics_pipeline,
        })
    }
}

fn create_render_pass(device: &Device, format: vk::Format) -> UError<vk::RenderPass> {
    // Describe the swapchain image.
    let color_attachment = vk::AttachmentDescription::builder()
        .format(format)
        .samples(vk::SampleCountFlags::_1)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

    // Reference the color attachment from our subpass.
    let color_attachment_ref = vk::AttachmentReference::builder()
        .attachment(0)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

    let color_attachments = [color_attachment_ref];

    // Describe the rendering subpass.
    let subpass = vk::SubpassDescription::builder()
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .color_attachments(&color_attachments);

    let subpasses = [subpass];

    // Synchronize color attachment writes with the
    // beginning of this rendering subpass.
    let dependency = vk::SubpassDependency::builder()
        .src_subpass(vk::SUBPASS_EXTERNAL)
        .dst_subpass(0)
        .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);

    let dependencies = [dependency];

    let create_info = vk::RenderPassCreateInfo::builder()
        .attachments(std::slice::from_ref(&color_attachment))
        .subpasses(&subpasses)
        .dependencies(&dependencies);

    Ok(unsafe { device.create_render_pass(&create_info, None) }?)
}

fn create_graphics_pipeline(
    device: &Device,
    render_pass: vk::RenderPass,
    extent: vk::Extent2D,
) -> UError<(vk::PipelineLayout, vk::Pipeline)> {
    use vulkanalia::bytecode::Bytecode;

    // Include the compiled SPIR-V in the executable.
    let vertex_code = include_bytes!("../shaders/triangle.vert.spv");

    let fragment_code = include_bytes!("../shaders/triangle.frag.spv");

    let vertex_bytecode =
        Bytecode::new(vertex_code).map_err(|e| format!("Invalid vertex SPIR-V: {e:?}"))?;

    let fragment_bytecode =
        Bytecode::new(fragment_code).map_err(|e| format!("Invalid fragment SPIR-V: {e:?}"))?;

    // Create the shader modules.
    let vertex_module_info = vk::ShaderModuleCreateInfo::builder()
        .code(vertex_bytecode.code())
        .code_size(vertex_bytecode.code_size());

    let vertex_module = unsafe { device.create_shader_module(&vertex_module_info, None) }?;

    let fragment_module_info = vk::ShaderModuleCreateInfo::builder()
        .code(fragment_bytecode.code())
        .code_size(fragment_bytecode.code_size());

    let fragment_module = match unsafe { device.create_shader_module(&fragment_module_info, None) }
    {
        Ok(module) => module,

        Err(error) => {
            unsafe { device.destroy_shader_module(vertex_module, None) };

            return Err(error.into());
        }
    };

    // -----------------------------------------------------
    // Shader stages
    // -----------------------------------------------------

    let vertex_stage = vk::PipelineShaderStageCreateInfo::builder()
        .stage(vk::ShaderStageFlags::VERTEX)
        .module(vertex_module)
        .name(b"main\0");

    let fragment_stage = vk::PipelineShaderStageCreateInfo::builder()
        .stage(vk::ShaderStageFlags::FRAGMENT)
        .module(fragment_module)
        .name(b"main\0");

    let shader_stages = [vertex_stage, fragment_stage];

    // -----------------------------------------------------
    // Vertex input
    // -----------------------------------------------------

    // The vertex shader generates positions internally,
    // so no vertex bindings or attributes are necessary.
    let vertex_input = vk::PipelineVertexInputStateCreateInfo::builder();

    // -----------------------------------------------------
    // Primitive assembly
    // -----------------------------------------------------

    let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false);

    // -----------------------------------------------------
    // Viewport and scissor
    // -----------------------------------------------------

    let viewport = vk::Viewport::builder()
        .x(0.0)
        .y(0.0)
        .width(extent.width as f32)
        .height(extent.height as f32)
        .min_depth(0.0)
        .max_depth(1.0);

    let scissor = vk::Rect2D::builder()
        .offset(vk::Offset2D::builder().x(0).y(0))
        .extent(extent);

    let viewports = [viewport];
    let scissors = [scissor];

    let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
        .viewports(&viewports)
        .scissors(&scissors);

    // -----------------------------------------------------
    // Rasterization
    // -----------------------------------------------------

    let rasterization = vk::PipelineRasterizationStateCreateInfo::builder()
        .depth_clamp_enable(false)
        .rasterizer_discard_enable(false)
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::NONE)
        .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
        .depth_bias_enable(false)
        .line_width(1.0);

    // -----------------------------------------------------
    // Multisampling
    // -----------------------------------------------------

    let multisampling = vk::PipelineMultisampleStateCreateInfo::builder()
        .rasterization_samples(vk::SampleCountFlags::_1)
        .sample_shading_enable(false);

    // -----------------------------------------------------
    // Color blending
    // -----------------------------------------------------

    let color_attachment = vk::PipelineColorBlendAttachmentState::builder()
        .blend_enable(false)
        .color_write_mask(
            vk::ColorComponentFlags::R
                | vk::ColorComponentFlags::G
                | vk::ColorComponentFlags::B
                | vk::ColorComponentFlags::A,
        );

    let color_attachments = [color_attachment];

    let color_blending = vk::PipelineColorBlendStateCreateInfo::builder()
        .logic_op_enable(false)
        .attachments(&color_attachments);

    // -----------------------------------------------------
    // Pipeline layout
    // -----------------------------------------------------

    // No descriptor sets or push constants are needed yet.
    let layout_info = vk::PipelineLayoutCreateInfo::builder();

    let pipeline_layout = unsafe { device.create_pipeline_layout(&layout_info, None) }?;

    // -----------------------------------------------------
    // Graphics pipeline
    // -----------------------------------------------------

    let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
        .stages(&shader_stages)
        .vertex_input_state(&vertex_input)
        .input_assembly_state(&input_assembly)
        .viewport_state(&viewport_state)
        .rasterization_state(&rasterization)
        .multisample_state(&multisampling)
        .color_blend_state(&color_blending)
        .layout(pipeline_layout)
        .render_pass(render_pass)
        .subpass(0);

    let pipeline_result = unsafe {
        device.create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
    };

    // Shader modules are no longer needed after pipeline
    // creation has completed, including if it fails.
    unsafe { device.destroy_shader_module(fragment_module, None) };

    unsafe { device.destroy_shader_module(vertex_module, None) };

    let mut pipelines =
        pipeline_result.map_err(|error| format!("Graphics pipeline creation failed: {error:?}"))?;

    let pipeline = pipelines.0.remove(0);

    Ok((pipeline_layout, pipeline))
}

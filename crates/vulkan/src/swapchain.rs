use vulkanalia::{
    Device, Instance,
    vk::{self, DeviceV1_0, Handle, HasBuilder, KhrSwapchainExtensionDeviceCommands},
};
use winit::window::Window;

use crate::{
    UError,
    anythings::{QueueFamilyIndices, SwapchainSupport},
    vulkan_context::VulkanContext,
};

pub struct SwapchainState {
    pub swapchain: vk::SwapchainKHR,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
}

impl SwapchainState {
    pub fn new(window: &Window, context: &VulkanContext) -> UError<Self> {
        let indices = QueueFamilyIndices::get(
            &context.instance,
            context.device.physical_device(),
            context.surface,
        )?;
        let support = SwapchainSupport::get(
            &context.instance,
            context.surface,
            context.device.physical_device(),
        )?;

        let surface_format = get_swapchain_surface_format(&support.formats)?;
        let present_mode = get_swapchain_present_mode(&support.present_modes);
        let extent = get_swapchain_extent(window, support.capabilities);
        let mut image_count = support.capabilities.min_image_count + 1;

        if support.capabilities.max_image_count != 0
            && image_count > support.capabilities.max_image_count
        {
            image_count = support.capabilities.max_image_count;
        }

        let queue_family_indices = [indices.graphics, indices.present];
        let (sharing_mode, family_indices): (vk::SharingMode, &[u32]) =
            if indices.graphics != indices.present {
                (vk::SharingMode::CONCURRENT, &queue_family_indices)
            } else {
                (vk::SharingMode::EXCLUSIVE, &[])
            };
        let create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(context.surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(sharing_mode)
            .queue_family_indices(family_indices)
            .pre_transform(support.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(vk::SwapchainKHR::null());

        let swapchain = unsafe { context.device.create_swapchain_khr(&create_info, None) }?;

        let images = unsafe { context.device.get_swapchain_images_khr(swapchain) }?;
        let image_views =
            create_swapchain_image_views(&context.device, &images, surface_format.format)?;
        Ok(Self {
            swapchain,
            images,
            image_views,
            format: surface_format.format,
            extent,
        })
    }
}

fn create_swapchain_image_views(
    device: &Device,
    images: &[vk::Image],
    format: vk::Format,
) -> UError<Vec<vk::ImageView>> {
    let mut views = Vec::with_capacity(images.len());

    for &image in images {
        let components = vk::ComponentMapping::builder()
            .r(vk::ComponentSwizzle::IDENTITY)
            .g(vk::ComponentSwizzle::IDENTITY)
            .b(vk::ComponentSwizzle::IDENTITY)
            .a(vk::ComponentSwizzle::IDENTITY);

        let subresource_range = vk::ImageSubresourceRange::builder()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1);

        let create_info = vk::ImageViewCreateInfo::builder()
            .image(image)
            .view_type(vk::ImageViewType::_2D)
            .format(format)
            .components(components)
            .subresource_range(subresource_range);

        let view = unsafe { device.create_image_view(&create_info, None) }?;

        views.push(view);
    }

    Ok(views)
}

fn get_swapchain_surface_format(formats: &[vk::SurfaceFormatKHR]) -> UError<vk::SurfaceFormatKHR> {
    formats
        .iter()
        .copied()
        .find(|format| {
            format.format == vk::Format::B8G8R8A8_SRGB
                && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        })
        .or_else(|| formats.first().copied())
        .ok_or_else(|| "No supported surface formats".into())
}

fn get_swapchain_present_mode(present_modes: &[vk::PresentModeKHR]) -> vk::PresentModeKHR {
    present_modes
        .iter()
        .cloned()
        .find(|mode| *mode == vk::PresentModeKHR::MAILBOX)
        .unwrap_or(vk::PresentModeKHR::FIFO)
}

fn get_swapchain_extent(window: &Window, capabilities: vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
    if capabilities.current_extent.width != u32::MAX {
        capabilities.current_extent
    } else {
        vk::Extent2D::builder()
            .width(window.inner_size().width.clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            ))
            .height(window.inner_size().height.clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            ))
            .build()
    }
}

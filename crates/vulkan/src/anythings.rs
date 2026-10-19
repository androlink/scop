use std::{collections::HashSet, ffi::CStr, os::raw::c_void};

use vulkanalia::{
    Device, Entry, Instance,
    vk::{
        self, DeviceV1_0, EntryV1_0, HasBuilder, InstanceV1_0, KhrSurfaceExtensionInstanceCommands,
        Queue, SurfaceKHR,
    },
    window::create_surface,
};

const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];

const VALIDATION_ENABLED: bool = cfg!(debug_assertions);

const VALIDATION_LAYER: vk::ExtensionName =
    vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");

use crate::UError;

pub fn instance(window: &winit::window::Window, entry: &Entry) -> UError<Instance> {
    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"Scop\0")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"No Engine\0")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));

    let mut extensions = vulkanalia::window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();
    if VALIDATION_ENABLED {
        extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
    }

    let available_layers = unsafe { entry.enumerate_instance_layer_properties() }?
        .iter()
        .map(|l| l.layer_name)
        .collect::<HashSet<_>>();

    if VALIDATION_ENABLED && !available_layers.contains(&VALIDATION_LAYER) {
        return Err("Validation layer requested but not supported.".into());
    }

    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        Vec::new()
    };

    let mut info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions);

    let mut debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .user_callback(Some(debug_callback));

    if VALIDATION_ENABLED {
        info = info.push_next(&mut debug_info);
    }

    let instance = unsafe { entry.create_instance(&info, None) }?;

    Ok(instance)
}

pub fn physical_device(instance: &Instance, surface: SurfaceKHR) -> UError<vk::PhysicalDevice> {
    fn check_device_extention(
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
    ) -> UError<()> {
        let extensions =
            unsafe { instance.enumerate_device_extension_properties(physical_device, None) }?
                .iter()
                .map(|e| e.extension_name)
                .collect::<HashSet<_>>();
        if DEVICE_EXTENSIONS.iter().all(|e| extensions.contains(e)) {
            Ok(())
        } else {
            Err("Missing required device extensions.".into())
        }
    }

    let check_device = |physical_device: vk::PhysicalDevice| -> UError<()> {
        let support = SwapchainSupport::get(instance, surface, physical_device)?;
        if support.formats.is_empty() || support.present_modes.is_empty() {
            return Err("Insufficient swapchain support.".into());
        }
        check_device_extention(instance, physical_device)?;
        Ok(())
    };

    for device in unsafe { instance.enumerate_physical_devices() }? {
        match check_device(device) {
            Ok(_) => return Ok(device),
            Err(e) => eprintln!("{e}"),
        }
    }

    Err("Failed to find suitable physical device.".into())
}

pub fn logical_device(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    indices: &QueueFamilyIndices,
) -> UError<Device> {
    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        vec![]
    };
    let mut _extensions = vec![];
    let features = vk::PhysicalDeviceFeatures::builder();
    let mut unique_indices = HashSet::new();
    unique_indices.insert(indices.graphics);
    unique_indices.insert(indices.present);

    let queue_priorities = &[1.0];
    let queue_infos = unique_indices
        .iter()
        .map(|i| {
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(*i)
                .queue_priorities(queue_priorities)
        })
        .collect::<Vec<_>>();
    let info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&_extensions)
        .enabled_features(&features);
    let device = unsafe { instance.create_device(physical_device, &info, None) }?;
    Ok(device)
}

pub fn graphics_queue(device: &Device, indices: &QueueFamilyIndices) -> UError<Queue> {
    Ok(unsafe { device.get_device_queue(indices.graphics, 0) })
}

pub fn present_queue(device: &Device, indices: &QueueFamilyIndices) -> UError<Queue> {
    Ok(unsafe { device.get_device_queue(indices.present, 0) })
}

pub fn surface(instance: &Instance, window: &winit::window::Window) -> UError<vk::SurfaceKHR> {
    Ok(unsafe { create_surface(instance, window, window) }?)
}

extern "system" fn debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    type_: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

    if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        println!("Error: ({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        println!("Warn: ({:?}) {}", type_, message);
    } else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        println!("Debug: ({:?}) {}", type_, message);
    } else {
        println!("Trace: ({:?}) {}", type_, message);
    }

    vk::FALSE
}
//
// struct
//

#[derive(Copy, Clone, Debug)]
pub struct QueueFamilyIndices {
    pub graphics: u32,
    pub present: u32,
}

impl QueueFamilyIndices {
    pub fn get(
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
        surface: SurfaceKHR,
    ) -> UError<Self> {
        let properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        let mut present = None;
        for (index, _properties) in properties.iter().enumerate() {
            if unsafe {
                instance.get_physical_device_surface_support_khr(
                    physical_device,
                    index as u32,
                    surface,
                )
            }? {
                present = Some(index as u32);
                break;
            }
        }

        if let (Some(graphics), Some(present)) = (graphics, present) {
            Ok(Self { graphics, present })
        } else {
            Err("Missing required queue families.".into())
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwapchainSupport {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

impl SwapchainSupport {
    pub fn get(
        instance: &Instance,
        surface: vk::SurfaceKHR,
        physical_device: vk::PhysicalDevice,
    ) -> UError<Self> {
        Ok(Self {
            capabilities: unsafe {
                instance.get_physical_device_surface_capabilities_khr(physical_device, surface)
            }?,
            formats: unsafe {
                instance.get_physical_device_surface_formats_khr(physical_device, surface)
            }?,
            present_modes: unsafe {
                instance.get_physical_device_surface_present_modes_khr(physical_device, surface)
            }?,
        })
    }
}

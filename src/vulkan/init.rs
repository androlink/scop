use std::ffi::CString;

use ash::{
    Entry, Instance,
    khr::external_semaphore_fd::Device,
    vk::{self, PhysicalDeviceType, QueueFamilyProperties, QueueFlags, TaggedStructure},
};
use sdl2::{libc::NFT_QUEUE_FLAG_MASK, sys::VkInstance};

pub fn vulkan_instance(entry: &Entry) -> Result<Instance, String> {
    let app_info = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_0);
    let create_info = vk::InstanceCreateInfo::default().application_info(&app_info);
    let extentions = log_extention(entry)?;
    let extention_names: Vec<_> = extentions.iter().map(|e| e.as_ptr()).collect();
    let create_info = create_info.enabled_extension_names(&extention_names);
    let instance = unsafe {
        entry
            .create_instance(&create_info, None)
            .map_err(|e| e.to_string())?
    };
    Ok(instance)
}

fn log_extention(entry: &Entry) -> Result<Vec<CString>, String> {
    let extentions = unsafe { entry.enumerate_instance_extension_properties(None) }
        .map_err(|e| e.to_string())?;

    // extentions.iter().for_each(|e| {
    //     let name = e.extension_name_as_c_str().unwrap();
    //     println!("[{}]", name.to_str().unwrap());
    // });
    let extentions = extentions
        .iter()
        .map(|e| CString::from(e.extension_name_as_c_str().unwrap()))
        .collect();
    println!("extentions: {:#?}", extentions);
    Ok(extentions)
}

pub fn vulkan_physical_device(instance: &Instance) -> Result<vk::PhysicalDevice, String> {
    let devices = unsafe { instance.enumerate_physical_devices() }.map_err(|e| e.to_string())?;

    for device in devices {
        print_device(instance, device);
        match check_device(instance, device) {
            Ok(()) => return Ok(device),
            Err(e) => println!("{}", e),
        }
    }
    Err("no device found".to_string())
}

fn check_device(instance: &Instance, physical_device: vk::PhysicalDevice) -> Result<(), String> {
    let props = unsafe { instance.get_physical_device_properties(physical_device) };
    let feats = unsafe { instance.get_physical_device_features(physical_device) };
    if props.device_type != PhysicalDeviceType::DISCRETE_GPU {
        Err("not gpu".to_string())
    } else if feats.geometry_shader == 0 {
        Err("not geometry shader".to_string())
    } else {
        Ok(())
    }
}

fn print_device(instance: &Instance, physical_device: vk::PhysicalDevice) {
    let props = unsafe { instance.get_physical_device_properties(physical_device) };

    println!(
        "device found: {}",
        props.device_name_as_c_str().unwrap().to_str().unwrap()
    );
}

pub fn vulkan_logical_device(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    index: u32,
) -> Result<ash::Device, String> {
    let priority = &[0.1];
    let queue_info = &[vk::DeviceQueueCreateInfo::default()
        .queue_family_index(index)
        .queue_priorities(priority)];
    let device_info = vk::DeviceCreateInfo::default().queue_create_infos(queue_info);
    let device = unsafe { instance.create_device(physical_device, &device_info, None) }
        .map_err(|e| e.to_string())?;
    Ok(device)
}

pub fn vulkan_queue_indice(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> Result<u32, String> {
    let queues = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
    for (index, q) in queues.iter().enumerate() {
        if q.queue_flags.contains(QueueFlags::GRAPHICS) {
            return Ok(index as u32);
        }
    }
    Err("queue not found".into())
}

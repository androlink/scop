use ash::{
    Entry, Instance,
    vk::{self, PhysicalDevice},
};

use super::init::*;

pub struct App {
    pub instance: Instance,
    pub entry: Entry,
    physical_device: PhysicalDevice,
    device: ash::Device,
    queue_index: u32,
    queue: ash::vk::Queue,
}

impl App {
    pub fn new() -> Result<Self, String> {
        let entry = unsafe { Entry::load() }.map_err(|e| e.to_string())?;
        let instance = vulkan_instance(&entry)?;
        let physical_device = vulkan_physical_device(&instance)?;
        let queue_index = vulkan_queue_indice(&instance, physical_device)?;
        let device = vulkan_logical_device(&instance, physical_device, queue_index)?;
        let queue = unsafe { device.get_device_queue(queue_index, 0) };
        Ok(App {
            instance,
            entry,
            physical_device,
            device,
            queue_index,
            queue,
        })
    }
}

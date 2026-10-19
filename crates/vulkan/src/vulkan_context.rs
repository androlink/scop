use std::{collections::HashSet, error::Error, ffi::CStr, os::raw::c_void};

use vulkanalia::{
    Device, Entry, Instance,
    loader::{LIBRARY, LibloadingLoader},
    vk::{self, InstanceV1_0},
};
use winit::window::Window;

use crate::{
    UError,
    anythings::{
        QueueFamilyIndices, graphics_queue, instance, logical_device, physical_device,
        present_queue, surface,
    },
};

pub struct VulkanContext {
    pub entry: Entry,
    pub instance: Instance,
    pub surface: vk::SurfaceKHR,
    pub device: Device,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
}

impl VulkanContext {
    pub fn new(window: &Window) -> UError<Self> {
        let loader = unsafe { LibloadingLoader::new(LIBRARY) }?;
        let entry = unsafe { Entry::new(loader).map_err(|b| b.to_string()) }?;
        let instance = instance(window, &entry)?;
        let surface = surface(&instance, window)?;
        let physical_device = physical_device(&instance, surface)?;
        let indices = QueueFamilyIndices::get(&instance, physical_device, surface)?;
        let device = logical_device(&instance, physical_device, &indices)?;
        let graphics_queue = graphics_queue(&device, &indices)?;
        let present_queue = present_queue(&device, &indices)?;
        Ok(Self {
            entry,
            instance,
            surface,
            device,
            graphics_queue,
            present_queue,
        })
    }
}

impl Drop for VulkanContext {
    fn drop(&mut self) {
        unsafe { self.instance.destroy_instance(None) };
    }
}

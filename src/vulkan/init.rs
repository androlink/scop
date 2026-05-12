use ash::vk::{CopyAccelerationStructureToMemoryInfoKHR, LayerProperties};
use ash::*;
use egui::ahash::HashMap;
use std::collections::HashSet;
use std::ffi::CString;
use std::sync::Arc;

const VALIDATION_ENABLED: bool = cfg!(debug_assertions);

pub struct Vulkan {
    instance: Instance,
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        unsafe { self.instance.destroy_instance(None) };
    }
}

pub fn init_vk() {
    let vk_entry = unsafe { ash::Entry::load() }.expect("no lib ?");
    let instance = create_info(&vk_entry);
}

fn create_info(vk_entry: &Entry) -> Result<Instance, String> {
    let app_info = vk::ApplicationInfo {
        api_version: ash::vk::make_api_version(0, 1, 0, 0),
        ..Default::default()
    };
    let validation_layer = if VALIDATION_ENABLED {
        check_validation_layer(vk_entry)?
    } else {
        Vec::default()
    };
    let create_info = ash::vk::InstanceCreateInfo::default().application_info(&app_info);
    let create_info = validation_layer
        .iter()
        .fold(create_info, |ci, vp| ci.enabled_layer_names(vp.as_bytes()));
    unsafe { vk_entry.create_instance(&create_info, None) }.map_err(|o| o.to_string())
}

fn get_validation_layer() -> Vec<CString> {
    let mut set = Vec::new();
    set.push(c"VK_LAYER_KHRONOS_validation".into());
    set
}

fn check_validation_layer(vk_entry: &Entry) -> Result<Vec<CString>, String> {
    let validation_set = get_validation_layer();
    let available_layers =
        unsafe { vk_entry.enumerate_instance_layer_properties() }.map_err(|o| o.to_string())?;
    for layer in available_layers {
        let layer_name: [u8; 256] = layer.layer_name.map(|c| c as u8);
        let layer_name: CString = unsafe { CString::from_vec_unchecked(layer_name.to_vec()) };
        if !validation_set.contains(&layer_name) {
            return Err(format!("layer not found: {:?}", layer_name));
        }
    }

    Ok(validation_set)
}

pub fn get_queue(instance: &Instance) {}

use std::ffi::CString;

use ash::{
    Entry, Instance,
    vk::{self, ExtensionProperties},
};

pub fn init_vulkan() -> Result<Instance, String> {
    let entry = unsafe { Entry::load() }.map_err(|e| e.to_string())?;
    let app_info = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_0);
    let create_info = vk::InstanceCreateInfo::default().application_info(&app_info);
    let extentions = log_extention(&entry)?;
    // let box_extention = extentions
    //     .iter()
    //     .map(|e| e.as_ptr())
    //     .collect::<Vec<*const i8>>()
    //     .into_boxed_slice();
    // let create_info = create_info.enabled_extension_names(&box_extention);
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
    println!("{:#?}", extentions);
    Ok(extentions)
}

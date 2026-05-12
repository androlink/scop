use ash::{Entry, Instance, vk};

pub fn init_vulkan() -> Result<Instance, String> {
    let entry = unsafe { Entry::load() }.map_err(|e| e.to_string())?;
    let app_info = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_0);
    let create_info = vk::InstanceCreateInfo::default().application_info(&app_info);
    get_extention(&entry)?;
    let instance = unsafe {
        entry
            .create_instance(&create_info, None)
            .map_err(|e| e.to_string())?
    };

    Ok(instance)
}

fn get_extention(entry: &Entry) -> Result<(), String> {
    let extentions = unsafe { entry.enumerate_instance_extension_properties(None) }
        .map_err(|e| e.to_string())?;
    extentions.iter().for_each(|e| {
        let name = e.extension_name_as_c_str().unwrap();
        println!("[{}]", name.to_str().unwrap());
    });
    Ok(())
}

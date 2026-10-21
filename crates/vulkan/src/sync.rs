use vulkanalia::{
    Device,
    vk::{self, DeviceV1_0, HasBuilder},
};

use crate::UError;

pub struct SyncObject {
    pub image_available: vk::Semaphore,
    pub render_finished: vk::Semaphore,
    pub in_flight: vk::Fence,
}

impl SyncObject {
    pub fn new(device: &Device) -> UError<Self> {
        let (image_available, render_finished, in_flight) = create_sync_objects(device)?;
        Ok(Self {
            image_available,
            render_finished,
            in_flight,
        })
    }
}

impl SyncObject {
    pub fn destroy(&mut self, device: &Device) {
        unsafe { device.destroy_fence(self.in_flight, None) };

        unsafe { device.destroy_semaphore(self.render_finished, None) };

        unsafe { device.destroy_semaphore(self.image_available, None) };
    }
}

fn create_sync_objects(device: &Device) -> UError<(vk::Semaphore, vk::Semaphore, vk::Fence)> {
    let semaphore_info = vk::SemaphoreCreateInfo::builder();

    let image_available_semaphore = unsafe { device.create_semaphore(&semaphore_info, None) }?;

    let render_finished_semaphore = match unsafe { device.create_semaphore(&semaphore_info, None) }
    {
        Ok(semaphore) => semaphore,

        Err(error) => {
            unsafe { device.destroy_semaphore(image_available_semaphore, None) };

            return Err(error.into());
        }
    };

    let fence_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

    let in_flight_fence = match unsafe { device.create_fence(&fence_info, None) } {
        Ok(fence) => fence,

        Err(error) => {
            unsafe { device.destroy_semaphore(render_finished_semaphore, None) };

            unsafe { device.destroy_semaphore(image_available_semaphore, None) };

            return Err(error.into());
        }
    };

    Ok((
        image_available_semaphore,
        render_finished_semaphore,
        in_flight_fence,
    ))
}

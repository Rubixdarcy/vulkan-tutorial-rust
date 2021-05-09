use ash::{
    vk,
    Device,
    Entry,
    Instance,
};
use device_extensions::DeviceExtensions;
use prelude::*;
use std::ffi::CString;
use validation::Validation;

pub mod constant;
pub mod device_extensions;
pub mod prelude;
pub mod tool;
pub mod validation;

pub struct VulkanApp {
    _entry: Entry,
    instance: Instance,
    _validation: Validation,
    device: Device,
}

impl VulkanApp {
    pub fn create() -> Self {
        //////////////////// ENTRY ////////////////////
        let entry = Entry::new().unwrap();

        let mut create_info = vk::InstanceCreateInfo::builder();

        //////////////////// SHOW EXTENSIONS ////////////////////
        entry
            .enumerate_instance_extension_properties()
            .iter()
            .flat_map(|p| p.iter())
            .for_each(|p| println!("{:?}", p));

        //////////////////// APP INFO ////////////////////
        let app_name = CString::new(APPLICATION_NAME).unwrap();
        let engine_name = CString::new(ENGINE_NAME).unwrap();

        let app_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(APPLICATION_VERSION)
            .engine_name(&engine_name)
            .engine_version(ENGINE_VERSION)
            .api_version(API_VERSION);

        //////////////////// CREATE INSTANCE ////////////////////
        create_info = create_info.application_info(&app_info);
        let instance =
            unsafe { entry.create_instance(&create_info, None) }.expect("Instance creation error");

        //////////////////// VALIDATION ////////////////////
        let mut validation = Self::validation(&entry, &instance);
        let layers = validation.get_ptrs();
        create_info = create_info.enabled_layer_names(&layers);

        //////////////////// PHYSICAL DEVICE ////////////////////
        let physical_device = Self::pick_physical_device(&instance);

        //////////////////// LOGICAL DEVICE ////////////////////
        let device = Self::create_logical_device(&instance, physical_device, &mut validation);

        //////////////////// RETURN VALUE ////////////////////
        Self {
            _entry: entry,
            instance,
            _validation: validation,
            device,
        }
    }

    fn validation(entry: &Entry, instance: &Instance) -> Validation {
        let layers = vec!["VK_LAYER_KHRONOS_validation"];

        Validation::new(&layers, entry)
    }

    fn pick_physical_device(instance: &Instance) -> vk::PhysicalDevice {
        unsafe { instance.enumerate_physical_devices() }
            .unwrap()
            .into_iter()
            .filter(|&device| Self::is_device_suitable(instance, device))
            .next()
            .expect("suitable physical device")
    }

    fn is_device_suitable(instance: &Instance, device: vk::PhysicalDevice) -> bool {
        QueueFamilyIndicies::for_device(instance, device).is_complete()
    }

    fn create_logical_device(
        instance: &Instance,
        device: vk::PhysicalDevice,
        validation: &mut Validation,
    ) -> Device {
        let indicies = QueueFamilyIndicies::for_device(instance, device);

        let queue_create_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(indicies.graphics.expect("graphics queue family"))
            .queue_priorities(&[1.0]);

        let extensions = DeviceExtensions::new(instance, device);

        let layer_names = validation.get_ptrs();
        let extension_names = extensions.get_ptrs();
        let create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(std::slice::from_ref(&queue_create_info))
            .enabled_layer_names(&layer_names)
            .enabled_extension_names(&extension_names);

        unsafe { instance.create_device(device, &create_info, None) }.expect("logical device")
    }
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct QueueFamilyIndicies {
    graphics: Option<u32>,
}

impl QueueFamilyIndicies {
    fn for_device(instance: &Instance, device: vk::PhysicalDevice) -> Self {
        let mut indicies = Self::default();

        let families = unsafe { instance.get_physical_device_queue_family_properties(device) };

        for (i, queue_family) in families.into_iter().enumerate() {
            if indicies.is_complete() {
                break;
            }
            if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                indicies.graphics = Some(i as u32);
            }
        }

        indicies
    }

    fn is_complete(self) -> bool {
        self.graphics.is_some()
    }
}

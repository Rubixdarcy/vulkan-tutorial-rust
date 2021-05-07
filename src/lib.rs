use ash::{
    vk,
    Entry,
    Instance,
};
use validation::Validation;
use std::ffi::CString;
use prelude::*;

pub mod prelude;
pub mod constant;
pub mod tool;
pub mod validation;

const VALIDATION_ENABLED: bool = true;
pub struct VulkanApp {
    _entry: Entry,
    instance: Instance,
    _validation: Validation,
}

impl VulkanApp {

    pub fn create() -> Self {
        let entry = Entry::new().unwrap();
        let instance: Instance;
        let mut app_info = vk::ApplicationInfo::builder();
        let mut create_info = vk::InstanceCreateInfo::builder();


        //////////////////// SHOW EXTENSIONS ////////////////////
        entry.enumerate_instance_extension_properties()
            .iter()
            .flat_map(|p| p.iter())
            .for_each(|p| println!("{:?}", p));


        //////////////////// APP INFO ////////////////////
        let app_name = CString::new(APPLICATION_NAME).unwrap();
        let engine_name = CString::new(ENGINE_NAME).unwrap();

        app_info = app_info
            .application_name(&app_name)
            .application_version(APPLICATION_VERSION)
            .engine_name(&engine_name)
            .engine_version(ENGINE_VERSION)
            .api_version(API_VERSION);


        //////////////////// VALIDATION ////////////////////
        let validation = Self::default_validation();
        let validation_names_cstrings: Vec<CString> = validation.layers.iter()
            .cloned()
            .map(|s| CString::new(s).unwrap())
            .collect();
        let validation_names_ptrs: Vec<*const i8> = validation_names_cstrings.iter()
            .map(|cstr| cstr.as_ptr())
            .collect();

        if validation.enabled {
            println!("Using validation");
            if !validation.check_validation_layer_support(&entry) {
                panic!("Validation layers requested but not available.")
            }

            create_info = create_info.enabled_layer_names(&validation_names_ptrs);
        }
            

        //////////////////// CREATE INSTANCE ////////////////////
        create_info = create_info.application_info(&app_info);
        unsafe {
            instance = entry
                .create_instance(&create_info, None)
                .expect("Instance creation error");
        }

        //////////////////// PHYSICAL DEVICE ////////////////////
        let _physical_device = Self::pick_physical_device(&instance);

        Self { _entry: entry, instance, _validation: validation }
    }

    fn default_validation() -> Validation {
        validation::Validation {
            layers: vec![
                "VK_LAYER_KHRONOS_validation".to_owned()
            ],
            enabled: VALIDATION_ENABLED,
        }
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

    // fn pick_physical_device(instance: &Instance) -> vk::PhysicalDevice {
    //     unsafe { instance.enumerate_physical_devices() }
    //         .unwrap()
    //         .into_iter()
    //         .map(|device| (device, Self::rate_device_suitability(instance, device)))
    //         .max_by_key(|&(_d, s)| s)
    //         .filter(|&(_d, s)| s > 0)
    //         .map(|(d, _s)| d)
    //         .expect("suitable physical device")
    // }

    // fn rate_device_suitability(instance: &Instance, device: vk::PhysicalDevice) -> u32 {
    //     let props = unsafe { instance.get_physical_device_properties(device) };
    //     let features = unsafe { instance.get_physical_device_features(device) };

    //     if features.geometry_shader == 0 {
    //         return 0;
    //     }

    //     let discrete = if props.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
    //         1000
    //     } else {
    //         0
    //     };
    //     let dimension = props.limits.max_image_dimension2_d;

    //     discrete + dimension
    // }


}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct QueueFamilyIndicies {
    graphics: Option<usize>,
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
                indicies.graphics = Some(i);
            }
        }

        indicies
    }

    fn is_complete(self) -> bool {
        self.graphics.is_some()
    }
}

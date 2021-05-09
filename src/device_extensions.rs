use ash::{
    vk::{self,},
    Instance,
};
use std::{
    borrow::Borrow,
    ffi::{
        CStr,
        CString,
    },
};

use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DeviceExtensions {
    extensions: Vec<CString>,
}

impl DeviceExtensions {
    pub fn new(instance: &Instance, device: vk::PhysicalDevice) -> Self {
        let available_extensions =
            unsafe { instance.enumerate_device_extension_properties(device) }
                .expect("physical device extensions");

        let extensions = get_required_extensions(&available_extensions);

        Self { extensions }
    }

    pub fn get_ptrs(&self) -> Vec<*const i8> {
        tool::string_vec_to_ptr_vec(&self.extensions)
    }
}

fn get_required_extensions(extensions: &[vk::ExtensionProperties]) -> Vec<CString> {
    const REQUIRED_LAYER_NAME: &str = "VK_KHR_portability_subset";

    extensions
        .into_iter()
        .map(|prop| prop.extension_name)
        .filter(|&name| tool::vk_to_string(&name) == REQUIRED_LAYER_NAME)
        .map(|name| unsafe { tool::vk_to_cstring(&name) })
        .collect()
}

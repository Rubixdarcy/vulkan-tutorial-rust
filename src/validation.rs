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
pub struct Validation {
    layers: Vec<CString>,
}

impl Validation {
    pub fn new(wanted_layers: &[&str], entry: &Entry) -> Self {
        let layers: Vec<CString> = wanted_layers
            .into_iter()
            .cloned()
            .map(|s| CString::new(s).unwrap())
            .collect();
        let layer_props = get_layer_props(entry);
        check_wanted_are_provided(&layers, &layer_props);

        Self { layers }
    }

    pub fn get_ptrs(&self) -> Vec<*const i8> {
        tool::string_vec_to_ptr_vec(&self.layers)
    }
}

fn get_layer_props(entry: &Entry) -> Vec<CString> {
    entry
        .enumerate_instance_layer_properties()
        .unwrap()
        .into_iter()
        .map(|prop| unsafe { tool::vk_to_cstring(&prop.layer_name) })
        .collect()
}

fn check_wanted_are_provided(wanted: &[CString], provided: &[CString]) {
    let missing: Vec<&CString> = wanted
        .iter()
        .filter(|&wanted_layer| {
            provided
                .iter()
                .all(|provided_layer| wanted_layer != provided_layer)
        })
        .collect();
    if missing.len() > 0 {
        panic!(
            "The following layers were wanted but not provided: {:?}",
            missing
        );
    }
}

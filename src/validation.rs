use crate::prelude::*;

pub struct Validation {
    pub layers: Vec<String>,
    pub enabled: bool,
}

impl Validation {
    pub fn check_validation_layer_support(&self, entry: &Entry) -> bool {
        let layer_props: Vec<String> = entry.enumerate_instance_layer_properties()
            .unwrap()
            .into_iter()
            .map(|prop| tool::vk_to_string(&prop.layer_name))
            .collect();

        self.layers.iter()
            .all(|expected| layer_props.iter()
                .any(|actual| actual == expected))
    }
}

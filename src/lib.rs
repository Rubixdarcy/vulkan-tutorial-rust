use ash::{
    vk,
    Entry,
    Instance,
};
use validation::Validation;
use std::ffi::CString;
use winit::event::{
    ElementState,
    Event,
    KeyboardInput,
    VirtualKeyCode,
    WindowEvent,
};
use winit::event_loop::{
    ControlFlow,
    EventLoop,
};
use prelude::*;

mod prelude;
mod constant;
pub mod tool;
mod validation;

const VALIDATION_ENABLED: bool = true;
pub struct VulkanApp {
    entry: Entry,
    instance: Instance,
    validation: Validation,
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

        Self { entry, instance, validation }
    }



    pub fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window {
        winit::window::WindowBuilder::new()
            .with_title(WINDOW_TITLE)
            .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .build(event_loop)
            .expect("Failed to create window.")
    }

    pub fn main_loop(event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        virtual_keycode,
                        state,
                        ..
                    } => match (virtual_keycode, state) {
                        (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                            dbg!();
                            *control_flow = ControlFlow::Exit
                        }
                        _ => {}
                    },
                },
                _ => {}
            },
            _ => (),
        })
    }

    fn default_validation() -> Validation {
        validation::Validation {
            layers: vec![
                "VK_LAYER_KHRONOS_validation".to_owned()
            ],
            enabled: VALIDATION_ENABLED,
        }
    }
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

pub use ash::version::{
    DeviceV1_0,
    EntryV1_0,
    InstanceV1_0,
};
use ash::{
    vk,
    Entry,
    Instance,
};
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
use constant::*;

mod constant;

pub struct VulkanApp {
    entry: Entry,
    instance: Instance,
}

impl VulkanApp {

    pub fn create() -> Self {
        let app_name = CString::new(APPLICATION_NAME).unwrap();
        let engine_name = CString::new(ENGINE_NAME).unwrap();

        let app_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(APPLICATION_VERSION)
            .engine_name(&engine_name)
            .engine_version(ENGINE_VERSION)
            .api_version(API_VERSION);

        let create_info = vk::InstanceCreateInfo::builder().application_info(&app_info);

        let entry = Entry::new().unwrap();

        let instance: Instance;

        unsafe {
            instance = entry
                .create_instance(&create_info, None)
                .expect("Instance creation error");
        }

        for prop in entry.enumerate_instance_extension_properties().iter().flat_map(|p| p.iter()) {
            println!("{:?}", prop);
        }

        Self { entry, instance }
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
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

use darcy_vulkan_tutorial_rust::prelude::*;
use winit::{event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}};

fn main() {
    let event_loop = EventLoop::new();
    let _window = init_window(&event_loop);
    let _app = VulkanApp::create();

    main_loop(event_loop);
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

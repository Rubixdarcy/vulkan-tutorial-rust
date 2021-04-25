use darcy_vulkan_tutorial_rust::VulkanApp;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();
    let _window = VulkanApp::init_window(&event_loop);
    let app = VulkanApp::create();

    VulkanApp::main_loop(event_loop);
}

use ash::vk_make_version;

pub const WINDOW_TITLE: &str = "Vulkan Tutorial";
pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;

pub const APPLICATION_NAME: &str = "Hello Triangle";
pub const ENGINE_NAME: &str = "No Engine";

pub const APPLICATION_VERSION: u32 = vk_make_version!(1, 0, 0);
pub const ENGINE_VERSION: u32 = vk_make_version!(1, 0, 0);
pub const API_VERSION: u32 = vk_make_version!(1, 0, 92);

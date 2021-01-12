mod app; // Load the dropped submodule
use app::App; // Use the trait app::App
use truck_platform::{wgpu::AdapterInfo, DeviceHandler};

// Declare an empty struct
struct MyApp {}

// Implement App to the empty struct
impl App for MyApp {
    fn init(_: &DeviceHandler, _: AdapterInfo) -> Self { MyApp {} }
}

// Run!
fn main() { MyApp::run() }

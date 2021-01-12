mod app; // Load the dropped submodule
use app::App; // Use the trait app::App
use truck_platform::{wgpu::AdapterInfo, Camera, DeviceHandler, Scene};

// Declare an empty struct
struct MyApp {
    scene: Scene,
}

// Implement App to the empty struct
impl App for MyApp {
    fn init(device_handler: &DeviceHandler, _: AdapterInfo) -> Self {
        let scene = Scene::new(device_handler.clone(), &Default::default());
        MyApp { scene }
    }
}

// Run!
fn main() { MyApp::run() }

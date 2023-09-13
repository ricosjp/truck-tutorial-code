mod app; // Load the dropped submodule
use app::*; // Use the trait app::App

use std::sync::Arc;
use winit::window::Window;

// Declare an empty struct
struct MyApp {}

// Implement App to the empty struct
#[async_trait(?Send)]
impl App for MyApp {
    // the constructor of the empty struct
    async fn init(_: Arc<Window>) -> Self { MyApp {} }
}

// Run!
fn main() { MyApp::run() }

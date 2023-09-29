mod app; // Load the dropped submodule
use app::*; // Use the trait app::App
use truck_platform::*;
use truck_rendimpl::*;

use std::sync::Arc;
use winit::window::Window;

// Declare the application handler, a struct with a scene
struct MyApp {
    scene: WindowScene,
}

// Make MyApp an application handler by implementing App
#[async_trait(?Send)]
impl App for MyApp {
    // constructor
    async fn init(window: Arc<Window>) -> Self {
        // Use default setting except position and posture
        let mut camera: Camera = Camera::default();
        // specify position and posture
        camera.matrix = Matrix4::look_at_rh(
            // camera position
            Point3::new(5.0, 6.0, 5.0),
            // The camera looks to the center of the model.
            Point3::new(0.0, 1.5, 0.0),
            // the y-up coordinate
            Vector3::unit_y(),
        )
        // The matrix output from `look_at` needs to be inverted,
        // since cgmath uses the "self-centric" theory of moving the world with respect to the camera,
        // while truck uses the "world-centric" theory of moving the camera with respect to the world.
        .invert()
        .unwrap();

        // Use default setting except the position
        let mut light: Light = Light::default();
        // It is safe to place the camera in the same position as the flash.
        light.position = camera.position();

        let scene_desc = WindowSceneDescriptor {
            studio: StudioConfig {
                camera,
                lights: vec![light],
                ..Default::default()
            },
            ..Default::default()
        };

        // Create the scene
        let mut scene = WindowScene::from_window(window, &scene_desc).await;

        // Load the polygon from a wavefront obj file.
        let polygon: PolygonMesh =
            polymesh::obj::read(include_bytes!("teapot.obj").as_ref()).unwrap();
        // Once the polygon data is in the form of an "instance".
        // This may seem wasteful to the beginning user, but this redundancy is useful for saving memory.
        let instance: PolygonInstance = scene
            .instance_creator() // <- instance is created by instance creator.
            .create_instance(&polygon, &Default::default());
        // Sign up the polygon to the scene.
        scene.add_object(&instance);

        // Return the application handler
        MyApp { scene }
    }

    // This method is called every frame.
    fn render(&mut self) {
        // scene draws a picture to the window.
        self.scene.render_frame();
    }
}

// Run!
fn main() { MyApp::run() }

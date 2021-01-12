mod app; // Load the dropped submodule
use app::App; // Use the trait app::App
use std::f64::consts::PI;
use truck_platform::*;
use truck_rendimpl::*;
use wgpu::{AdapterInfo, SwapChainFrame};

// Declare the application handler, a struct with a scene
struct MyApp {
    scene: Scene,
}

// Implement App to the empty struct
impl App for MyApp {
    fn init(device_handler: &DeviceHandler, _: AdapterInfo) -> Self {
        // Create camera matrix
        let camera_matrix = Matrix4::look_at(
            // camera position
            Point3::new(6.0, 7.0, 6.0),
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

        // Create a perspective camera
        let camera = Camera::perspective_camera(
            // camera matrix
            camera_matrix,
            // the field of view
            Rad(PI / 4.0),
            // distance with the near clipping plane
            0.1,
            // distance with the far clipping plane
            100.0,
        );

        // Create a light
        let light = Light {
            // It is safe to place the camera in the same position as the flash.
            position: camera.position(),
            // There are the other options. Look later!
            ..Default::default()
        };

        // Create the scene
        let mut scene = Scene::new(
            // `DeviceHandler` is the toolchain of the structs provided from wgpu.
            // This allows the Scene to pass the information it receives from the CPU to the GPU.
            device_handler.clone(),
            // This passes only a reference. In fact, it would be better to pass the entity,
            // but we are trying to match the operability to wgpu.
            &SceneDescriptor {
                // A scene has only one camera.
                camera,
                // The argument is `Vec` since a scene can have several lights.
                lights: vec![light],
                // There are the other options. Look later!
                ..Default::default()
            },
        );

        // Load the polygon from a wavefront obj file.
        let polygon = polymesh::obj::read(include_bytes!("teapot.obj").as_ref()).unwrap();        
        // Once the polygon data is in the form of an "instance".
        // This may seem wasteful to the beginning user, but this redundancy is useful for saving memory.
        let instance = scene.create_instance(&polygon, &Default::default());
        // Sign up the polygon to the scene.
        scene.add_object(&instance);

        // Return the application handler
        MyApp { scene }
    }

    // This method is called every frame.
    fn render(&mut self, frame: &SwapChainFrame) { self.scene.render_scene(&frame.output.view) }
}

// Run!
fn main() { MyApp::run() }

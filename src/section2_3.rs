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
    // constructor
    fn init(device_handler: &DeviceHandler, _: AdapterInfo) -> Self {
        // Use default setting except position and posture
        let camera = Camera::perspective_camera(
            // We will update it later, so we leave it as a unit matrix here.
            Matrix4::identity(),
            // the field of view. Default is Rad(PI / 4.0).
            Rad(PI / 4.5),
            // the distance to the near clipping plane
            0.1,
            // the distance to the far clipping plane
            10.0,
        );

        // Useful constants for lights placement.
        let radius = 5.0 * f64::sqrt(2.0);
        let omega = [0.5, f64::sqrt(3.0) * 0.5];

        // positions of lights
        let position0 = Point3::new(radius * omega[0], 6.0, radius * omega[1]);
        let position1 = Point3::new(-radius, 6.0, 0.0);
        let position2 = Point3::new(radius * omega[0], 6.0, -radius * omega[1]);

        // red light
        let red_light = Light {
            // the position of red light
            position: position0,
            // red light
            color: Vector3::new(1.0, 0.0, 0.0),
            // point light
            light_type: LightType::Point,
        };

        // green light
        let green_light = Light {
            // the position of green light
            position: position1,
            // green light
            color: Vector3::new(0.0, 1.0, 0.0),
            // point light
            light_type: LightType::Point,
        };

        // blue light
        let blue_light = Light {
            // the position of the third light
            position: position2,
            // blue light
            color: Vector3::new(0.0, 0.0, 1.0),
            // point light
            light_type: LightType::Point,
        };

        // lights
        let lights = vec![red_light, green_light, blue_light];

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
                lights,
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

    // This meshod is called every frame
    fn update(&mut self, _handler: &DeviceHandler) {
        // the time since the application started.
        let time = self.scene.elapsed().as_secs_f64();

        // the mutable references of camera and lights.
        let (camera, lights) = {
            let desc = self.scene.descriptor_mut();
            (&mut desc.camera, &mut desc.lights)
        };

        // rotation matrix
        let rot = Matrix4::from_axis_angle(Vector3::unit_y(), Rad(time));

        // update camera matrix
        camera.matrix = rot
            * Matrix4::look_at(
                Point3::new(5.0, 6.0, 5.0),
                Point3::new(0.0, 1.5, 0.0),
                Vector3::unit_y(),
            )
            .invert()
            .unwrap();

        // update light positions
        lights[0].position[1] = 6.0 * (time * 0.8).cos();
        lights[1].position[1] = -6.0 * (time * 0.8).cos();
        lights[2].position[1] = 6.0 * (time * 0.8).cos();
    }

    // This method is called every frame.
    fn render(&mut self, frame: &SwapChainFrame) { self.scene.render_scene(&frame.output.view) }
}

// Run!
fn main() { MyApp::run() }

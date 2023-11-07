mod app; // Load the dropped submodule
use app::*; // Use the trait app::App
use truck_platform::*;
use truck_rendimpl::*;

use std::sync::Arc;
use winit::window::Window;

use std::f64::consts::PI;

// Declare the application handler, a struct with a scene
struct MyApp {
    scene: WindowScene,
}

// Implement App to the empty struct
#[async_trait(?Send)]
impl App for MyApp {
    // constructor
    async fn init(window: Arc<Window>) -> Self {
        // Use default setting except position and posture
        let camera: Camera = Camera::perspective_camera(
            // We will update it later, so we leave it as a unit matrix here.
            Matrix4::identity(),
            // the field of view. Default is Rad(PI / 4.0). This case, a little telescope.
            Rad(PI / 4.5),
            // the distance to the near clipping plane
            0.1,
            // the distance to the far clipping plane
            20.0,
        );

        // radius of circumscribed circle
        let radius: f64 = 5.0 * f64::sqrt(2.0);
        // Useful constants for lights placement.
        let omega: [f64; 2] = [0.5, f64::sqrt(3.0) * 0.5];

        // positions of lights, the vertices of regular triangle
        let position0: Point3 = Point3::new(radius * omega[0], 6.0, radius * omega[1]);
        let position1: Point3 = Point3::new(-radius, 6.0, 0.0);
        let position2: Point3 = Point3::new(radius * omega[0], 6.0, -radius * omega[1]);

        // red light
        let red_light: Light = Light {
            // position of the red light
            position: position0,
            // red
            color: Vector3::new(1.0, 0.0, 0.0),
            // point light
            light_type: LightType::Point,
        };

        // green light
        let green_light: Light = Light {
            // position of the green light
            position: position1,
            // green
            color: Vector3::new(0.0, 1.0, 0.0),
            // point light
            light_type: LightType::Point,
        };

        // blue light
        let blue_light: Light = Light {
            // position of the blue light
            position: position2,
            // blue
            color: Vector3::new(0.0, 0.0, 1.0),
            // point light
            light_type: LightType::Point,
        };

        // the vector of lights
        let lights: Vec<Light> = vec![red_light, green_light, blue_light];

        // Create the scene
        let scene_desc = WindowSceneDescriptor {
            studio: StudioConfig {
                // the scene has only one camera.
                camera,
                // the scene can have several lights.
                lights,
                ..Default::default()
            },
            // There are the other options. Look later!
            ..Default::default()
        };

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
        // the seconds since the application started.
        let time: f64 = self.scene.elapsed().as_secs_f64();

        // the mutable references to camera and lights.
        let (camera, lights): (&mut Camera, &mut Vec<Light>) = {
            // Reget the mutable reference to StudioConfig.
            let studio = self.scene.studio_config_mut();
            // the mutable references to camera and lights.
            (&mut studio.camera, &mut studio.lights)
        };

        // rotation matrix
        let rot: Matrix4 = Matrix4::from_axis_angle(
            // the axis of rotation
            Vector3::unit_y(),
            // 1 radian per second
            Rad(time),
        );

        // update camera matrix
        camera.matrix = rot
            * Matrix4::look_at_rh(
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

        self.scene.render_frame()
    }
}

// Run!
fn main() { MyApp::run() }

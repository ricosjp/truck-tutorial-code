mod app;
use app::*;
use std::f64::consts::PI;
use truck_platform::*;
use truck_rendimpl::*;
use wgpu::*;

// size of the square
const SQUARE_SIZE: usize = 5;
// color of back ground
const BACKGROUND: [f64; 4] = [45.0 / 255.0, 36.0 / 255.0, 42.0 / 255.0, 1.0];
// color of boxes
const BOXCOLOR: [f64; 4] = [208.0 / 255.0, 176.0 / 255.0, 107.0 / 255.0, 1.0];

// application handler
struct MyApp {
    // scene
    scene: Scene,
    // instances for render
    instances: Vec<ShapeInstance>,
    // initial matricies of instances
    matrices: Vec<Matrix4>,
}

impl App for MyApp {
    fn init(device_handler: &DeviceHandler, _: AdapterInfo) -> MyApp {
        // side length of rendered square
        let side_length = (SQUARE_SIZE + 1) as f64 * 1.5;
        // disntace between camera and rendered square
        let camera_dist = side_length / 2.0 / (PI / 8.0).tan();

        // temporary constants for light positions
        let a = side_length / 2.0;
        let b = camera_dist / 2.0;
        let scene_desc = SceneDescriptor {
            camera: Camera::perspective_camera(
                Matrix4::from_translation(camera_dist * Vector3::unit_z()),
                Rad(PI / 4.0),
                0.1,
                100.0,
            ),
            lights: vec![
                Light {
                    position: Point3::new(-a, -a, b),
                    color: Vector3::new(0.5, 0.5, 0.5),
                    light_type: LightType::Point,
                },
                Light {
                    position: Point3::new(-a, a, b),
                    color: Vector3::new(0.5, 0.5, 0.5),
                    light_type: LightType::Point,
                },
                Light {
                    position: Point3::new(a, -a, b),
                    color: Vector3::new(0.5, 0.5, 0.5),
                    light_type: LightType::Point,
                },
                Light {
                    position: Point3::new(a, a, b),
                    color: Vector3::new(0.5, 0.5, 0.5),
                    light_type: LightType::Point,
                },
            ],
            // back ground color
            background: Color {
                r: BACKGROUND[0],
                g: BACKGROUND[1],
                b: BACKGROUND[2],
                a: BACKGROUND[3],
            },
            ..Default::default()
        };

        // create the scene
        let mut scene = Scene::new(device_handler.clone(), &scene_desc);

        // modeling a unit cube
        let vertex = builder::vertex(Point3::new(-0.5, -0.5, -0.5));
        let edge = builder::tsweep(&vertex, Vector3::unit_x());
        let face = builder::tsweep(&edge, Vector3::unit_y());
        let cube = builder::tsweep(&face, Vector3::unit_z());

        // create the original instance
        let original_instance = scene.create_instance(&cube, &Default::default());

        // vector for instances
        let mut instances = Vec::new();
        // initialize matrices and instances
        let mut matrices = Vec::new();

        // loop
        for i in 0..SQUARE_SIZE {
            for j in 0..SQUARE_SIZE {
                // create an initial matrix
                let matrix = Matrix4::from_translation(Vector3::new(
                    1.5 * (i + 1) as f64 - side_length / 2.0,
                    1.5 * (j + 1) as f64 - side_length / 2.0,
                    0.0,
                ));
                // push matrix into the vector
                matrices.push(matrix);

                // create instance for drawing
                let mut instance = original_instance.clone();
                // set material
                *instance.descriptor_mut() = InstanceDescriptor {
                    material: Material {
                        albedo: Vector4::from(BOXCOLOR),
                        reflectance: i as f64 / (SQUARE_SIZE - 1) as f64,
                        roughness: j as f64 / (SQUARE_SIZE - 1) as f64,
                        ambient_ratio: 0.02,
                    },
                    ..Default::default()
                };
                // sign up the object to the scene
                scene.add_objects(&instance.render_faces());
                // push instance into the vector
                instances.push(instance);
            }
        }
        
        // Returns the initialized application handler
        MyApp {
            scene,
            instances,
            matrices,
        }
    }
    fn update(&mut self, _: &DeviceHandler) {
        // the seconds since the application started.
        let time = self.scene.elapsed().as_secs_f64();

        for (i, instance) in self.instances.iter_mut().enumerate() {
            // the axis of the rotation
            let axis = if i % 2 == 0 {
                (-1.0_f64).powi(i as i32 / 2) * Vector3::unit_y()
            } else {
                -(-1.0_f64).powi(i as i32 / 2) * Vector3::unit_x()
            };

            // rotate the instances
            instance.descriptor_mut().matrix =
                self.matrices[i] * Matrix4::from_axis_angle(axis, Rad(time * PI / 2.0));

            // update the scene
            self.scene.update_bind_groups(&instance.render_faces());
        }
    }

    // This method is called every frame.
    fn render(&mut self, frame: &SwapChainFrame) { self.scene.render_scene(&frame.output.view); }
}

// Run!
fn main() { MyApp::run() }

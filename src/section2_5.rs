mod app;
use app::*;
use truck_meshalgo::tessellation::MeshableShape;

use std::f64::consts::PI;
use std::sync::Arc;
use winit::window::Window;

use truck_meshalgo::prelude::*;
use truck_modeling::*;
use truck_platform::*;
use truck_rendimpl::*;

// size of the square
const SQUARE_SIZE: usize = 5;
// color of back ground
const BACK_GROUND: [f64; 4] = [45.0 / 255.0, 36.0 / 255.0, 42.0 / 255.0, 1.0];
// color of cubes
const CUBE_COLOR: [f64; 4] = [208.0 / 255.0, 176.0 / 255.0, 107.0 / 255.0, 1.0];

// side length of cubes square
const SIDE_LENGTH: f64 = (SQUARE_SIZE + 1) as f64 * 1.5;

// application handler
struct MyApp {
    // scene
    scene: WindowScene,
    // instances for render
    instances: Vec<PolygonInstance>,
}

#[async_trait(?Send)]
impl App for MyApp {
    async fn init(window: Arc<Window>) -> MyApp {
        // disntace between camera and rendered square
        let camera_dist: f64 = SIDE_LENGTH / 2.0 / (PI / 8.0).tan();

        // temporary constants for light positions
        let a: f64 = SIDE_LENGTH / 2.0;
        let b: f64 = camera_dist / 2.0;
        let studio = StudioConfig {
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
            background: wgpu::Color {
                r: BACK_GROUND[0],
                g: BACK_GROUND[1],
                b: BACK_GROUND[2],
                a: BACK_GROUND[3],
            },
            ..Default::default()
        };

        // create the scene
        let mut scene = WindowScene::from_window(
            window,
            &WindowSceneDescriptor {
                studio,
                ..Default::default()
            },
        )
        .await;

        // modeling a unit cube
        let vertex: Vertex = builder::vertex(Point3::new(-0.5, -0.5, -0.5));
        let edge: Edge = builder::tsweep(&vertex, Vector3::unit_x());
        let face: Face = builder::tsweep(&edge, Vector3::unit_y());
        let cube: Solid = builder::tsweep(&face, Vector3::unit_z());
        let mesh_cube = cube.triangulation(0.01).to_polygon();

        // create the original instance
        let original_instance: PolygonInstance = scene
            .instance_creator()
            .create_instance(&mesh_cube, &Default::default());

        // vector for instances
        let mut instances: Vec<PolygonInstance> = Vec::with_capacity(SQUARE_SIZE * SQUARE_SIZE);

        // loop
        for i in 0..SQUARE_SIZE {
            for j in 0..SQUARE_SIZE {
                // create instance for drawing
                let mut instance: PolygonInstance = original_instance.clone_instance();
                // set material
                instance.instance_state_mut().material = Material {
                    albedo: Vector4::from(CUBE_COLOR),
                    reflectance: i as f64 / (SQUARE_SIZE - 1) as f64,
                    roughness: j as f64 / (SQUARE_SIZE - 1) as f64,
                    ambient_ratio: 0.02,
                    ..Default::default()
                };
                // sign up the object to the scene
                scene.add_object(&instance);
                // push instance into the vector
                instances.push(instance);
            }
        }
        // Returns the initialized application handler
        MyApp { scene, instances }
    }

    fn render(&mut self) {
        // the seconds since the application started.
        let time: f64 = self.scene.elapsed().as_secs_f64();

        for (idx, instance) in self.instances.iter_mut().enumerate() {
            // row index
            let i: usize = idx / SQUARE_SIZE;
            // column index
            let j: usize = idx % SQUARE_SIZE;
            // create an initial matrix
            let matrix: Matrix4 = Matrix4::from_translation(Vector3::new(
                1.5 * (i + 1) as f64 - SIDE_LENGTH / 2.0,
                1.5 * (j + 1) as f64 - SIDE_LENGTH / 2.0,
                0.0,
            ));
            // the axes of the rotation
            let axis: Vector3 = if idx % 2 == 0 {
                (-1.0_f64).powi(idx as i32 / 2) * Vector3::unit_y()
            } else {
                -(-1.0_f64).powi(idx as i32 / 2) * Vector3::unit_x()
            };

            // rotate the instances
            instance.instance_state_mut().matrix =
                matrix * Matrix4::from_axis_angle(axis, Rad(time * PI / 2.0));

            // update the scene
            self.scene.update_bind_group(instance);
        }
        self.scene.render_frame();
    }
}

// Run!
fn main() { MyApp::run() }

mod app; // Load the dropped submodule
use app::*; // Use the trait app::App
use std::f64::consts::PI;

use std::sync::Arc;
use winit::window::Window;

use truck_meshalgo::prelude::*;
use truck_modeling::*;
use truck_platform::*;
use truck_rendimpl::*;

// Declare the application handler
struct MyApp {
    // scene
    scene: WindowScene,
    // current drawn shape
    current_shape: i32,
    // the instance of cube
    cube: PolygonInstance,
    // the instance of torus
    torus: PolygonInstance,
    // the instance of cylinder
    cylinder: PolygonInstance,
}

// Implement App to the empty struct
#[async_trait(?Send)]
impl App for MyApp {
    // constructor
    async fn init(window: Arc<Window>) -> Self {
        // radius of circumscribed circle
        let radius: f64 = 5.0 * f64::sqrt(2.0);
        // Useful constants for lights placement.
        let omega: [f64; 2] = [0.5, f64::sqrt(3.0) * 0.5];

        // the vector of lights
        let lights: Vec<Light> = vec![
            Light {
                position: Point3::new(radius * omega[0], 6.0, radius * omega[1]),
                // The color vector should be divided by 3.0. If not, the white will be satiated.
                color: Vector3::new(1.0, 1.0, 1.0) / 3.0,
                ..Default::default()
            },
            Light {
                position: Point3::new(-radius, 5.0, 0.0),
                // The color vector should be divided by 3.0. If not, the white will be satiated.
                color: Vector3::new(1.0, 1.0, 1.0) / 3.0,
                ..Default::default()
            },
            Light {
                position: Point3::new(radius * omega[0], 4.0, -radius * omega[1]),
                // The color vector should be divided by 3.0. If not, the white will be satiated.
                color: Vector3::new(1.0, 1.0, 1.0) / 3.0,
                ..Default::default()
            },
        ];

        let scene_desc = WindowSceneDescriptor {
            studio: StudioConfig {
                camera: Default::default(),
                lights,
                ..Default::default()
            },
            ..Default::default()
        };

        // Create the scene
        let scene: WindowScene = WindowScene::from_window(window, &scene_desc).await;

        let mesh_cube = solid2mesh(&cube());
        let mesh_torus = solid2mesh(&torus());
        let mesh_cylinder = solid2mesh(&cylinder());

        // An instance is created by InstanceCreator.
        // This structure prepares the data necessary for instance creation at initialization time,
        // so using it around will improve performance.
        let creator: InstanceCreator = scene.instance_creator();
        // create cube instance
        let cube: PolygonInstance = creator.create_instance(&mesh_cube, &Default::default());
        // create torus instance
        let torus: PolygonInstance = creator.create_instance(&mesh_torus, &Default::default());
        // create cylinder instance
        let cylinder: PolygonInstance =
            creator.create_instance(&mesh_cylinder, &Default::default());

        // Return the application handler
        MyApp {
            scene,
            current_shape: -1,
            cube,
            torus,
            cylinder,
        }
    }

    // This meshod is called every frame
    fn render(&mut self) {
        // the seconds since the application started.
        let time: f64 = self.scene.elapsed().as_secs_f64();

        // the mutable references to the camera
        let camera: &mut Camera = &mut self.scene.studio_config_mut().camera;

        // update camera matrix
        camera.matrix = Matrix4::from_axis_angle(Vector3::unit_y(), Rad(time))
            * Matrix4::look_at_rh(
                Point3::new(4.0, 5.0, 4.0),
                Point3::new(0.0, 1.0, 0.0),
                Vector3::unit_y(),
            )
            .invert()
            .unwrap();

        // the number of the shape which should be displayed
        let laps: i32 = (time / (2.0 * PI)) as i32 % 3;

        // the timing for changing the drawn shape
        if laps != self.current_shape {
            // synchronize variables
            self.current_shape = laps;
            // clear all objects in the scene
            self.scene.clear_objects();
            // laps == 0 => cube, laps == 1 => torus, laps == 2 => cylinder
            match laps {
                0 => self.scene.add_object(&self.cube),
                1 => self.scene.add_object(&self.torus),
                _ => self.scene.add_object(&self.cylinder),
            };
        }
        self.scene.render_frame();
    }
}

// modeling a cube
fn cube() -> Solid {
    // put a vertex at the point (-1, 0, -1)
    let vertex: Vertex = builder::vertex(Point3::new(-1.0, 0.0, -1.0));
    // sweep the vertex along the z-axis
    let edge: Edge = builder::tsweep(
        // the reference to the vertex
        &vertex,
        // sweep along the z-axis for length 2
        2.0 * Vector3::unit_z(),
    );
    // sweep the edge along the x-axis
    let face: Face = builder::tsweep(
        // the reference to the edge
        &edge,
        // sweep along the x-axis for length 2
        2.0 * Vector3::unit_x(),
    );
    // sweep the face along the y-axis
    builder::tsweep(
        // the reference to the face
        &face,
        // sweep along the y-axis for length 2
        2.0 * Vector3::unit_y(),
    )
}

// modeling a torus
fn torus() -> Shell {
    // put a vertex at the point (0, 0, 1).
    let vertex = builder::vertex(Point3::new(0.0, 0.0, 1.0));
    // sweep the vertex along a circle
    let circle: Wire = builder::rsweep(
        // the reference to the vertex
        &vertex,
        // a point on the axis
        Point3::new(0.0, 0.5, 1.0),
        // the direction of the axis
        Vector3::unit_x(),
        // If the absolute value is no less than 2π radian, a closed shape will be generated.
        Rad(7.0),
    );
    // sweep the circle along a circle
    builder::rsweep(
        // the reference to the wire
        &circle,
        // a point on the axis
        Point3::origin(),
        // the direction of the axis
        Vector3::unit_y(),
        // If the absolute value is no less than 2π radian, a closed shape will be generated.
        Rad(7.0),
    )
}

// modeling a cylinder
fn cylinder() -> Solid {
    // put a vertex at the point (0, 0, -1).
    let vertex = builder::vertex(Point3::new(0.0, 0.0, -1.0));
    // sweep the vertex along circle
    let wire: Wire = builder::rsweep(
        // the reference to the vertex
        &vertex,
        // a point on the axis
        Point3::new(0.0, 1.0, -1.0),
        // the direction of the axis
        Vector3::unit_z(),
        // If a value greater than 2π radian is specified, a closed shape will be generated.
        Rad(7.0),
    );
    // make a disk by attaching a plane to the circle
    let face: Face = builder::try_attach_plane(&vec![wire]).expect("cannot attach plane");
    // sweep the face along the z-axis
    builder::tsweep(
        // the reference to the disk
        &face,
        // sweep along the z-axis
        2.0 * Vector3::unit_z(),
    )
}

fn solid2mesh<Shape: MeshableShape>(shape: &Shape) -> PolygonMesh {
    shape.triangulation(0.01).to_polygon()
}

// Run!
fn main() { MyApp::run() }

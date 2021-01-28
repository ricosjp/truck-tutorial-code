mod app; // Load the dropped submodule
use app::App; // Use the trait app::App
use std::f64::consts::PI;
use truck_platform::*;
use truck_rendimpl::*;
use wgpu::{AdapterInfo, SwapChainFrame};

// Declare the application handler, a struct with a scene
struct MyApp {
    scene: Scene,
    current_shape: i32,
    cube: ShapeInstance,
    torus: ShapeInstance,
    cylinder: ShapeInstance,
}

// Implement App to the empty struct
impl App for MyApp {
    // constructor
    fn init(device_handler: &DeviceHandler, _: AdapterInfo) -> Self {
        // radius of circumscribed circle
        let radius = 5.0 * f64::sqrt(2.0);
        // Useful constants for lights placement.
        let omega = [0.5, f64::sqrt(3.0) * 0.5];

        // the vector of lights
        let lights = vec![
            Light {
                position: Point3::new(radius * omega[0], 6.0, radius * omega[1]),
                color: Vector3::new(1.0, 1.0, 1.0) / 3.0,
                ..Default::default()
            },
            Light {
                position: Point3::new(-radius, 5.0, 0.0),
                color: Vector3::new(1.0, 1.0, 1.0) / 3.0,
                ..Default::default()
            },
            Light {
                position: Point3::new(radius * omega[0], 4.0, -radius * omega[1]),
                color: Vector3::new(1.0, 1.0, 1.0) / 3.0,
                ..Default::default()
            },
        ];

        // Create the scene
        let scene = Scene::new(
            // `DeviceHandler` is the toolchain of the structs provided from wgpu.
            // This allows the Scene to pass the information it receives from the CPU to the GPU.
            device_handler.clone(),
            // This passes only a reference. In fact, it would be better to pass the entity,
            // but we are trying to match the operability to wgpu.
            &SceneDescriptor {
                camera: Default::default(),
                // A scene can have several lights.
                lights,
                // There are the other options. Look later!
                ..Default::default()
            },
        );

        // create cube instance
        let cube = scene.create_instance(&cube(), &Default::default());
        // create torus instance
        let torus = scene.create_instance(&torus(), &Default::default());
        // create cylinder instance
        let cylinder = scene.create_instance(&cylinder(), &Default::default());

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
    fn update(&mut self, _handler: &DeviceHandler) {
        // the seconds since the application started.
        let time = self.scene.elapsed().as_secs_f64();

        // the mutable references of camera
        let camera = &mut self.scene.descriptor_mut().camera;

        // update camera matrix
        camera.matrix = Matrix4::from_axis_angle(Vector3::unit_y(), Rad(time))
            * Matrix4::look_at(
                Point3::new(4.0, 5.0, 4.0),
                Point3::new(0.0, 1.0, 0.0),
                Vector3::unit_y(),
            )
            .invert()
            .unwrap();

        // the discretized number of lap
        let laps = (time / (2.0 * PI)) as i32 % NUM_OF_SHAPES as i32;

        // the timing for changing the drawn shape
        if laps != self.current_shape {
            // synchronize variable
            self.current_shape = laps;
            // clear all objects in the scene
            self.scene.clear_objects();
            // laps == 0 => cube, laps == 1 => torus
            match laps {
                0 => self.scene.add_objects(&self.cube.render_faces()),
                1 => self.scene.add_objects(&self.torus.render_faces()),
                _ => self.scene.add_objects(&self.cylinder.render_faces()),
            };
        }
    }

    // This method is called every frame.
    fn render(&mut self, frame: &SwapChainFrame) {
        self.scene.render_scene(&frame.output.view)
    }
}

const NUM_OF_SHAPES: usize = 3;

// modeling a cube
fn cube() -> Solid {
    // put a vertex at the point (-1, 0, -1)
    let vertex: Vertex = builder::vertex(Point3::new(-1.0, 0.0, -1.0));
    // sweep the vertex along the z-axis
    let edge: Edge = builder::tsweep(
        // the reference of the vertex
        &vertex,
        // sweep along the z-axis for length 2
        2.0 * Vector3::unit_z()
    );
    // sweep the edge along the x-axis
    let face: Face = builder::tsweep(
        // the reference of the edge
        &edge,
        // sweep along the x-axis for length 2
        2.0 * Vector3::unit_x()
    );
    // sweep the face along the y-axis
    builder::tsweep(
        // the reference 0f the face
        &face,
        // sweep along the y-axis for length 2
        2.0 * Vector3::unit_y()
    )
}

// modeling a torus
fn torus() -> Shell {
    // put a vertex at the point (0, 0, 1).
    let vertex: Vertex = builder::vertex(Point3::new(0.0, 0.0, 1.0));
    // sweep the vertex along a circle
    let circle: Wire = builder::rsweep(
        // the reference of vertex
        &vertex,
        // the center of the rotation
        Point3::new(0.0, 0.5, 1.0),
        // the axis of the rotation
        Vector3::unit_x(),
        // If the specified value is greater than 2π radian, a closed shape will be generated.
        Rad(7.0),
    );
    // sweep the circle along a circle
    builder::rsweep(
        // the reference of wire
        &circle,
        // the center of the rotation
        Point3::origin(),
        // the axis of the rotation
        Vector3::unit_y(),
        // If a value greater than 2π radian is specified, a closed shape will be generated.
        Rad(7.0)
    )
}

// modeling a cylinder
fn cylinder() -> Solid {
    // put a vertex at the point (0, 0, -1).
    let vertex: Vertex = builder::vertex(Point3::new(0.0, 0.0, -1.0));
    // sweep the vertex along circle
    let wire: Wire = builder::rsweep(
        // the reference of the vertex
        &vertex,
        // the center of the rotation
        Point3::new(0.0, 1.0, -1.0),
        // the axis of the rotation
        Vector3::unit_z(),
        // If a value greater than 2π radian is specified, a closed shape will be generated.
        Rad(7.0),
    );
    // make a disk by attaching a plane to the circle
    let face: Face = builder::try_attach_plane(&vec![wire]).expect("cannot attach plane");
    // sweep the face along the z-axis
    builder::tsweep(
        // the reference of the disk
        &face,
        // sweep along the z-axis
        2.0 * Vector3::unit_z()
    )
}

// Run!
fn main() {
    MyApp::run()
}

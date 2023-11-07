mod app; // Load the dropped submodule
use app::*; // Use the trait app::App
use std::f64::consts::PI;
use std::sync::Arc;
use truck_platform::*;
use truck_rendimpl::*;
use winit::{dpi::*, event::*, event_loop::ControlFlow, window::Window};

// the application handler
struct MyApp {
    // scene
    scene: WindowScene,
    // dragging flag
    rotate_flag: bool,
    // position of the cursor at the previous frame.
    prev_cursor: Vector2,
}

#[async_trait(?Send)]
impl App for MyApp {
    // constructor
    async fn init(window: Arc<Window>) -> Self {
        let mut scene = WindowScene::from_window(
            window,
            &WindowSceneDescriptor {
                studio: StudioConfig {
                    camera: Camera::perspective_camera(
                        Matrix4::look_at_rh(
                            Point3::new(1.5, 1.5, 1.5),
                            Point3::origin(),
                            Vector3::unit_y(),
                        )
                        .invert()
                        .unwrap(),
                        Rad(PI / 4.0),
                        0.1,
                        40.0,
                    ),
                    lights: vec![Light {
                        position: Point3::new(1.5, 1.5, 1.5),
                        color: Vector3::new(1.0, 1.0, 1.0),
                        light_type: LightType::Point,
                    }],
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .await;

        // modeling the bottle and signup to the scene
        let mesh = polymesh::obj::read(include_bytes!("hexahedron.obj").as_slice()).unwrap();
        let instance: PolygonInstance = scene.instance_creator().create_instance(
            &mesh,
            &PolygonState {
                // smooth plastic texture
                material: Material {
                    albedo: Vector4::new(0.75, 0.75, 0.75, 1.0),
                    reflectance: 0.2,
                    roughness: 0.2,
                    ambient_ratio: 0.02,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        scene.add_object(&instance);

        // Return the application handler
        MyApp {
            scene,
            // The mouse is not dragged when the application starts.
            rotate_flag: false,
            prev_cursor: Vector2::zero(),
        }
    }

    /// Processing when the mouse wheel is moved.
    fn mouse_wheel(&mut self, delta: MouseScrollDelta, _: TouchPhase) -> ControlFlow {
        match delta {
            // use only y-delta
            MouseScrollDelta::LineDelta(_, y) => {
                // get the mutable references to camera and light
                let studio = self.scene.studio_config_mut();
                let (camera, light) = (&mut studio.camera, &mut studio.lights[0]);
                // Translation to the eye direction by 0.2 times the value obtained from the wheel.
                let trans = Matrix4::from_translation(camera.eye_direction() * 0.2 * y as f64);
                // move the camera and light
                camera.matrix = trans * camera.matrix;
                light.position = camera.position();
            }
            _ => {}
        };
        // Return a command to wait 1/60 second.
        Self::default_control_flow()
    }

    // Called when the mouse button is pressed and released.
    fn mouse_input(&mut self, state: ElementState, button: MouseButton) -> ControlFlow {
        match button {
            // Behavior when the left button is pressed or unpressed
            MouseButton::Left => {
                // pressed => start dragging, released => end dragging.
                self.rotate_flag = state == ElementState::Pressed;
            }
            _ => {}
        }
        // Return a command to wait 1/60 second.
        Self::default_control_flow()
    }

    // Called when the cursor is moved
    fn cursor_moved(&mut self, position: PhysicalPosition<f64>) -> ControlFlow {
        let position = Vector2::new(position.x, position.y);
        if self.rotate_flag {
            // get the mutable references of camera and light
            let studio = self.scene.studio_config_mut();
            let (camera, light) = (&mut studio.camera, &mut studio.lights[0]);
            // get the delta of cursor move
            let dir2d = position - self.prev_cursor;
            // Do nothing if the delta is so small.
            if dir2d.so_small() {
                return Self::default_control_flow();
            }
            // axis of rotation
            let axis = (dir2d[1] * camera.matrix[0].truncate()
                + dir2d[0] * camera.matrix[1].truncate())
            .normalize();
            // angle of rotation. 0.01 times the pixel distance.
            let angle = dir2d.magnitude() * 0.01;
            // rotation matrix. The rotation angle is minus, as the camera is moved.
            let mat = Matrix4::from_axis_angle(axis, Rad(-angle));
            // move the camera and light.
            camera.matrix = mat * camera.matrix;
            light.position = camera.position();
        }
        // assign the current cursor position to "previous cursor position"
        self.prev_cursor = position;
        // Return a command to wait 1/60 second.
        Self::default_control_flow()
    }

    /// called when some file is dropped to the window
    fn dropped_file(&mut self, path: std::path::PathBuf) -> ControlFlow {
        // read file
        let obj: Vec<u8> = match std::fs::read(path) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{e}");
                return Self::default_control_flow();
            }
        };
        // parse to mesh
        let mut mesh: PolygonMesh = match polymesh::obj::read(obj.as_slice()) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{e}");
                return Self::default_control_flow();
            }
        };

        // get bounding box
        let bbx: BoundingBox<Point3> = mesh.bounding_box();
        // the center of the bounding box
        let center: Vector3 = bbx.center().to_vec();
        // the diameter of the bounding box
        let diameter: f64 = bbx.diameter();
        // Subtract the coordinates of each vertex by the center of the bounding box
        // and divide by half the length of the diagonal to keep the object in view.
        mesh.positions_mut().iter_mut().for_each(|p| {
            *p = (*p - center) / (diameter / 2.0);
        });
        // create instance
        let instance: PolygonInstance = self.scene.instance_creator().create_instance(
            &mesh,
            &PolygonState {
                // smooth plastic texture
                material: Material {
                    albedo: Vector4::new(0.75, 0.75, 0.75, 1.0),
                    reflectance: 0.2,
                    roughness: 0.2,
                    ambient_ratio: 0.02,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        // delete all object in the scene at once
        self.scene.clear_objects();
        self.scene.add_object(&instance);
        Self::default_control_flow()
    }

    // This method is called every frame.
    fn render(&mut self) { self.scene.render_frame() }
}

// Run!
fn main() { MyApp::run() }

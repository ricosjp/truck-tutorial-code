mod app; // Load the dropped submodule
use app::App; // Use the trait app::App
use std::f64::consts::PI;
use truck_platform::*;
use truck_rendimpl::*;
use wgpu::{AdapterInfo, SwapChainFrame};
use winit::{dpi::*, event::*, event_loop::ControlFlow};

// the application handler
struct MyApp {
    // scene
    scene: Scene,
    // dragging flag
    rotate_flag: bool,
    // position of the cursor at the previous frame.
    prev_cursor: Vector2,
}

// Implement App to the empty struct
impl App for MyApp {
    // constructor
    fn init(device_handler: &DeviceHandler, _: AdapterInfo) -> Self {
        let mut scene = Scene::new(
            device_handler.clone(),
            &SceneDescriptor {
                camera: Camera::perspective_camera(
                    Matrix4::look_at(
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
        );

        // modeling the bottle and signup to the scene
        let bottle = bottle(1.4, 1.0, 0.6);
        let instance = scene.create_instance(
            &bottle,
            &InstanceDescriptor {
                // smooth plastic texture
                material: Material {
                    albedo: Vector4::new(0.75, 0.75, 0.75, 1.0),
                    reflectance: 0.2,
                    roughness: 0.2,
                    ambient_ratio: 0.02,
                },
                ..Default::default()
            },
        );
        scene.add_objects(&instance.render_faces());

        // Return the application handler
        MyApp {
            scene,
            // The mouse is not dragged when the application starts.
            rotate_flag: false,
            prev_cursor: Vector2::zero(),
        }
    }

    // Called when the mouse button is pressed and released.
    fn mouse_input(&mut self, state: ElementState, button: MouseButton) -> ControlFlow {
        match button {
            // Behavior when the left button is pressed or unpressed
            MouseButton::Left => {
                // pressed => start dragging, unpressed => end dragging.
                self.rotate_flag = state == ElementState::Pressed;
            }
            _ => {}
        }
        // Return a command to wait 1/60 second.
        Self::default_control_flow()
    }
    fn cursor_moved(&mut self, position: PhysicalPosition<f64>) -> ControlFlow {
        let position = Vector2::new(position.x, position.y);
        if self.rotate_flag {
            // get the mutable references of camera and light
            let desc = self.scene.descriptor_mut();
            let (camera, light) = (&mut desc.camera, &mut desc.lights[0]);
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
    /// Processing when the mouse wheel is moved.
    fn mouse_wheel(&mut self, delta: MouseScrollDelta, _: TouchPhase) -> ControlFlow {
        match delta {
            // use only y-delta
            MouseScrollDelta::LineDelta(_, y) => {
                // get the mutable references of camera and light
                let sc_desc = self.scene.descriptor_mut();
                let (camera, light) = (&mut sc_desc.camera, &mut sc_desc.lights[0]);
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

    // This method is called every frame.
    fn render(&mut self, frame: &SwapChainFrame) { self.scene.render_scene(&frame.output.view) }
}

// modeling the body shape
// # Arguments
// - bottom: y-coordinate of the bottom face
// - height: height of the body
// - width: width of the body
// - thickness: thickness of the body
fn body_shell(bottom: f64, height: f64, width: f64, thickness: f64) -> Shell {
    // draw a circle arc
    let vertex0 = builder::vertex(Point3::new(-width / 2.0, bottom, thickness / 4.0));
    let vertex1 = builder::vertex(Point3::new(width / 2.0, bottom, thickness / 4.0));
    let transit = Point3::new(0.0, bottom, thickness / 2.0);
    let arc0 = builder::circle_arc(&vertex0, &vertex1, transit);
    // copy and rotate the circle arc
    let arc1 = builder::rotated(&arc0, Point3::origin(), Vector3::unit_y(), Rad(PI));
    // create the homotopy from arc0 to arc1.inverse()
    let face = builder::homotopy(&arc0, &arc1.inverse());
    // create the body
    let solid = builder::tsweep(&face, Vector3::new(0.0, height, 0.0));
    // Return the solid as a boundary shell for easier processing later.
    solid.into_boundaries().pop().unwrap()
}

// modeling a cylinder
// # Arguments
// - bottom: y-coordinate of the bottom disk
// - height: height of the cylinder
// - radius: radius of the bottom disk
fn cylinder(bottom: f64, height: f64, radius: f64) -> Shell {
    // make a solid cylinder
    let vertex = builder::vertex(Point3::new(0.0, bottom, radius));
    let circle = builder::rsweep(&vertex, Point3::origin(), Vector3::unit_y(), Rad(7.0));
    let disk = builder::try_attach_plane(&vec![circle]).unwrap();
    let solid = builder::tsweep(&disk, Vector3::new(0.0, height, 0.0));
    // Return the solid as a boundary shell for easier processing later.
    solid.into_boundaries().pop().unwrap()
}

// sew the body and the neck
fn grue_body_neck(body: &mut Shell, neck: Shell) {
    // get the body's seiling
    let body_seiling = body.last_mut().unwrap();
    // the boundary of the neck's bottom
    let wire = neck[0].boundaries()[0].clone();
    // drill a hole in the body using the boundary of the neck's bottom
    body_seiling.add_boundary(wire);
    // add the faces of the neck to the body other than the bottom
    body.extend(neck.into_iter().skip(1));
}

// modeling a bottle
fn bottle(height: f64, width: f64, thickness: f64) -> Solid {
    // create the body of the bottle
    let mut body = body_shell(-height / 2.0, height, width, thickness);
    // create the neck of the bottle
    let neck = cylinder(height / 2.0, height / 10.0, thickness / 4.0);
    // sew the body and the neck
    grue_body_neck(&mut body, neck);

    // distance between outer and inner surface, i.e. the thickness of the faces.
    let eps = height / 50.0;
    // inner body. Make it small enough to account for thickness.
    let mut inner_body = body_shell(
        -height / 2.0 + eps,
        height - 2.0 * eps,
        width - 2.0 * eps,
        thickness - 2.0 * eps,
    );
    // inner neck. Make it long and narrow to account for thickness.
    let inner_neck = cylinder(
        height / 2.0 - eps,
        height / 10.0 + eps,
        thickness / 4.0 - eps,
    );
    // sew the inner body and the inner neck
    grue_body_neck(&mut inner_body, inner_neck);

    // invert all faces of the inner body
    inner_body.face_iter_mut().for_each(|face| {
        face.invert();
    });
    // pop the seiling of the inner body
    let inner_seiling = inner_body.pop().unwrap();
    // make the inner seiling the boundary wire
    let wire = inner_seiling.into_boundaries().pop().unwrap();
    // the mutable reference of the outer seiling
    let seiling = body.last_mut().unwrap();
    // drill a hole in the outer seiling using the boundary of inner seiling
    seiling.add_boundary(wire);
    // add the faces of the neck to the body
    body.extend(inner_body.into_iter());
    // returns the solid
    Solid::new(vec![body])
}

// Run!
fn main() { MyApp::run() }

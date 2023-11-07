use truck_meshalgo::prelude::*;
use truck_modeling::*;
use truck_stepio::out::*;

/// Construct a cube
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

/// save solid with obj format
fn save_obj(solid: &Solid, path: &str) {
    // output to polygonmesh
    // Convert as boundary representation. Argument 0.01 is a rough estimate of the error when approximating with a mesh
    let mesh_with_topology = solid.triangulation(0.01);
    // merges meshes of faces into a single mesh.
    let mesh = mesh_with_topology.to_polygon();
    // save an obj file
    let mut obj = std::fs::File::create(path).unwrap();
    obj::write(&mesh, &mut obj).unwrap();
}

/// save solid with step format
fn save_step(solid: &Solid, path: &str) {
    // compress solid data.
    let compressed = solid.compress();
    // step format display
    let display = CompleteStepDisplay::new(StepModel::from(&compressed), Default::default());
    // content of step file
    let step_string: String = display.to_string();
    std::fs::write(path, &step_string).unwrap();
}

fn main() {
    let cube = cube();
    save_obj(&cube, "cube.obj");
    save_step(&cube, "cube.step");
}

use truck_meshalgo::prelude::*;
use truck_modeling::*;
use truck_stepio::out::*;

fn main() {
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
    let cube: Solid = builder::tsweep(
        // the reference to the face
        &face,
        // sweep along the y-axis for length 2
        2.0 * Vector3::unit_y(),
    );

    // output to polygonmesh
    let mesh_with_topology = cube.triangulation(0.01);
    let mesh = mesh_with_topology.to_polygon();
    let mut obj = std::fs::File::create("cube.obj").unwrap();
    obj::write(&mesh, &mut obj).unwrap();

    // compress solid data.
    let compressed = cube.compress();

    // step format display
    let display = CompleteStepDisplay::new(StepModel::from(&compressed), Default::default());
    // content of step file
    let step_string: String = display.to_string();
    std::fs::write("cube.step", &step_string).unwrap();
}

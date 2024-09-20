use truck_meshalgo::prelude::*;
use truck_modeling::*;
use truck_stepio::out::*;

// modeling a torus
fn torus() -> Solid {
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
    let boundary = builder::rsweep(
        // the reference to the wire
        &circle,
        // a point on the axis
        Point3::origin(),
        // the direction of the axis
        Vector3::unit_y(),
        // If the absolute value is no less than 2π radian, a closed shape will be generated.
        Rad(7.0),
    );
    Solid::new(vec![boundary])
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

fn save_shape(solid: &Solid, filename: &str) {
    // output to polygonmesh
    let mesh_with_topology = solid.triangulation(0.01);
    let mesh = mesh_with_topology.to_polygon();
    let obj_path = filename.to_string() + ".obj";
    let mut obj = std::fs::File::create(&obj_path).unwrap();
    obj::write(&mesh, &mut obj).unwrap();

    // compress solid data.
    let compressed = solid.compress();

    // step format display
    let display = CompleteStepDisplay::new(StepModel::from(&compressed), Default::default());
    // content of step file
    let step_string: String = display.to_string();
    let step_path = filename.to_string() + ".step";
    std::fs::write(&step_path, &step_string).unwrap();
}

fn main() {
    save_shape(&torus(), "torus");
    save_shape(&cylinder(), "cylinder");
}

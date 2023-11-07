use std::iter::FromIterator;
use truck_meshalgo::prelude::*;

/// Create a mesh with one equilateral triangle registered and save it in obj format
fn main() {
    // the positions of vertices
    let positions = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.5, f64::sqrt(3.0) / 2.0, 0.0),
    ];
    // The store of attributes. This time, only the location information is registered.
    let attrs = StandardAttributes {
        positions,
        ..Default::default()
    };
    // construct face
    let faces = Faces::from_iter([[0, 1, 2]]);
    // create polygon
    let polygon = PolygonMesh::new(attrs, faces);

    // create obj file
    let mut obj = std::fs::File::create("triangle.obj").unwrap();
    // writing!
    obj::write(&polygon, &mut obj).unwrap();
}

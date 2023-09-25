use truck_meshalgo::prelude::*;
use std::iter::FromIterator;

fn main() {
    let positions = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.5, f64::sqrt(3.0) / 2.0, 0.0),
        Point3::new(0.5, f64::sqrt(3.0) / 6.0, f64::sqrt(6.0) / 3.0),
    ];
    let attrs = StandardAttributes {
        positions,
        ..Default::default()
    };
    let faces = Faces::from_iter([
        [2, 1, 0],
        [0, 1, 3],
        [1, 2, 3],
        [2, 0, 3],
    ]);
    // create polugon
    let polygon = PolygonMesh::new(attrs, faces);

    // create output obj file
    let mut obj = std::fs::File::create("tetrahedron.obj").unwrap();
    // output polygon to obj file.
    obj::write(&polygon, &mut obj).unwrap();
}
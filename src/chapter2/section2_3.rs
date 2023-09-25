use truck_meshalgo::prelude::*;
use std::iter::FromIterator;

fn main() {
    let positions = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 0.0, 1.0),
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(0.0, 1.0, 1.0),
    ];
    let attrs = StandardAttributes {
        positions,
        ..Default::default()
    };
    // `PolygonMesh` allows not only triangles but also quandrangle or more polygons.
    let faces = Faces::from_iter([
        [3, 2, 1, 0],
        [0, 1, 5, 4],
        [1, 2, 6, 5],
        [2, 3, 7, 6],
        [3, 0, 4, 7],
        [4, 5, 6, 7],
    ]);
    // create polugon
    let polygon = PolygonMesh::new(attrs, faces);

    // create output obj file
    let mut obj = std::fs::File::create("cube.obj").unwrap();
    // output polygon to obj file.
    obj::write(&polygon, &mut obj).unwrap();
    
}
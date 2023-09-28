use std::iter::FromIterator;
use truck_meshalgo::prelude::*;

fn main() {
    // set the positions of vertices
    let positions = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.5, f64::sqrt(3.0) / 2.0, 0.0),
    ];
    // create attributes of polygon
    let attrs = StandardAttributes {
        positions,
        ..Default::default()
    };
    // set the faces of vertices
    let faces = Faces::from_iter([[0, 1, 2]]);
    // create polygon
    let polygon = PolygonMesh::new(attrs, faces);

    // create output obj file
    let mut obj = std::fs::File::create("triangle.obj").unwrap();
    // output polygon to obj file.
    obj::write(&polygon, &mut obj).unwrap();
}

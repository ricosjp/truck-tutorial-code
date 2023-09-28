use truck_meshalgo::prelude::*;
use std::iter::FromIterator;

fn write_polygon(polygon: &PolygonMesh, path: &str) {
    // create output obj file
    let mut obj = std::fs::File::create(path).unwrap();
    // output polygon to obj file.
    obj::write(polygon, &mut obj).unwrap();
}

fn trigonal_pyramid() -> PolygonMesh {
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
    // create polygon
    PolygonMesh::new(attrs, faces)
}

fn cube() -> PolygonMesh {
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
    // create polygon
    PolygonMesh::new(attrs, faces)
}

fn main() {
    write_polygon(&trigonal_pyramid(), "trigonal-pyramid.obj");    
    write_polygon(&cube(), "cube.obj");    
}

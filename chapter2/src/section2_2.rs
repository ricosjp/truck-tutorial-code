use std::iter::FromIterator;
use truck_meshalgo::prelude::*;

/// Create an equilateral triangular pyramid
fn trigonal_pyramid() -> PolygonMesh {
    // the positions of vertices
    let positions = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.5, f64::sqrt(3.0) / 2.0, 0.0),
        Point3::new(0.5, f64::sqrt(3.0) / 6.0, f64::sqrt(6.0) / 3.0),
    ];
    // the entire vertex information, now only coordinates
    let attrs = StandardAttributes {
        positions,
        ..Default::default()
    };
    // Specify the vertices of each face using the indices of the coordinate array
    let faces = Faces::from_iter([[2, 1, 0], [0, 1, 3], [1, 2, 3], [2, 0, 3]]);
    // create polygon
    PolygonMesh::new(attrs, faces)
}

/// Create a cube
fn cube() -> PolygonMesh {
    // the positions of vertices
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
    // the entire vertex information, now only coordinates
    let attrs = StandardAttributes {
        positions,
        ..Default::default()
    };
    // Specify the vertices of each face using the indices of the coordinate array
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

/// Output the contents of `polygon` to the file specified by `path`.
fn write_polygon(polygon: &PolygonMesh, path: &str) {
    // create output obj file
    let mut obj = std::fs::File::create(path).unwrap();
    // output polygon to obj file.
    obj::write(polygon, &mut obj).unwrap();
}

fn main() {
    write_polygon(&trigonal_pyramid(), "trigonal-pyramid.obj");
    write_polygon(&cube(), "cube.obj");
}

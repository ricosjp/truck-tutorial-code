use std::iter::FromIterator;
use truck_meshalgo::prelude::*;

fn write_polygon(polygon: &PolygonMesh, path: &str) {
    // create output obj file
    let mut obj = std::fs::File::create(path).unwrap();
    // output polygon to obj file.
    obj::write(polygon, &mut obj).unwrap();
}

/// create hexahedron polygon mesh
fn hexahedron() -> PolygonMesh {
    let a = f64::sqrt(3.0) / 3.0;
    let positions = vec![
        Point3::new(-a, -a, -a),
        Point3::new(a, -a, -a),
        Point3::new(a, a, -a),
        Point3::new(-a, a, -a),
        Point3::new(-a, -a, a),
        Point3::new(a, -a, a),
        Point3::new(a, a, a),
        Point3::new(-a, a, a),
    ];
    let attrs = StandardAttributes {
        positions,
        ..Default::default()
    };
    let faces = Faces::from_iter([
        [3, 2, 1, 0],
        [0, 1, 5, 4],
        [1, 2, 6, 5],
        [2, 3, 7, 6],
        [3, 0, 4, 7],
        [4, 5, 6, 7],
    ]);
    PolygonMesh::new(attrs, faces)
}

fn main() {
    // hexahedron polygon
    let hexa = hexahedron();
    // Number to divide each side of a hexahedron
    const DIVISION: usize = 8;
    let positions: Vec<Point3> = hexa
        .face_iter()
        .flat_map(|face| {
            let v: Vec<Vector3> = face
                .iter()
                .map(|vertex| hexa.positions()[vertex.pos].to_vec())
                .collect();
            (0..=DIVISION)
                .flat_map(move |i| (0..=DIVISION).map(move |j| (i, j)))
                .map(move |(i, j)| {
                    let s = i as f64 / DIVISION as f64;
                    let t = j as f64 / DIVISION as f64;
                    v[0] * (1.0 - s) * (1.0 - t)
                        + v[1] * s * (1.0 - t)
                        + v[3] * (1.0 - s) * t
                        + v[2] * s * t
                })
        })
        .map(|vec| Point3::from_vec(vec.normalize()))
        .collect();
    let normals = positions.iter().copied().map(Point3::to_vec).collect();
    let attrs = StandardAttributes {
        positions,
        normals,
        ..Default::default()
    };
    let faces: Faces = (0..6)
        .flat_map(|face_idx| {
            let base = face_idx * (DIVISION + 1) * (DIVISION + 1);
            let to_index = move |i: usize, j: usize| {
                let idx = base + (DIVISION + 1) * i + j;
                (idx, None, Some(idx))
            };
            (0..DIVISION)
                .flat_map(move |i| (0..DIVISION).map(move |j| (i, j)))
                .map(move |(i, j)| {
                    [
                        to_index(i, j),
                        to_index(i + 1, j),
                        to_index(i + 1, j + 1),
                        to_index(i, j + 1),
                    ]
                })
        })
        .collect();
    let sphere = PolygonMesh::new(attrs, faces);
    write_polygon(&sphere, "sphere.obj");
}

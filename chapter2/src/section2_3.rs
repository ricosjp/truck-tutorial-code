use std::iter::FromIterator;
use truck_meshalgo::prelude::*;

fn tetrahedron() -> PolygonMesh {
    let a = f64::sqrt(3.0) / 3.0;
    let positions = vec![
        Point3::new(-a, -a, -a),
        Point3::new(a, a, -a),
        Point3::new(a, -a, a),
        Point3::new(-a, a, a),
    ];
    let attrs = StandardAttributes {
        positions,
        ..Default::default()
    };
    let faces = Faces::from_iter([[0, 1, 2], [1, 3, 2], [1, 0, 3], [3, 0, 2]]);
    PolygonMesh::new(attrs, faces)
}

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

fn octahedron() -> PolygonMesh {
    let positions = vec![
        Point3::new(-1.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, -1.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 0.0, -1.0),
        Point3::new(0.0, 0.0, 1.0),
    ];
    let attrs = StandardAttributes {
        positions,
        ..Default::default()
    };
    let faces = Faces::from_iter([
        [0, 2, 5],
        [3, 0, 5],
        [1, 3, 5],
        [2, 1, 5],
        [0, 4, 2],
        [3, 4, 0],
        [1, 4, 3],
        [2, 4, 1],
    ]);
    PolygonMesh::new(attrs, faces)
}

fn dodecahedron() -> PolygonMesh {
    // the half of the length of edges of hexahedron
    let a = f64::sqrt(3.0) / 3.0;
    // the half of the length of edges of dodecahedron
    let l = 2.0 * a / (1.0 + f64::sqrt(5.0));
    // the length of projection vector
    let d = f64::sqrt(1.0 - l * l);
    let positions = vec![
        Point3::new(-a, -a, -a),
        Point3::new(a, -a, -a),
        Point3::new(a, a, -a),
        Point3::new(-a, a, -a),
        Point3::new(-a, -a, a),
        Point3::new(a, -a, a),
        Point3::new(a, a, a),
        Point3::new(-a, a, a),
        Point3::new(d, -l, 0.0),
        Point3::new(d, l, 0.0),
        Point3::new(-d, l, 0.0),
        Point3::new(-d, -l, 0.0),
        Point3::new(0.0, d, -l),
        Point3::new(0.0, d, l),
        Point3::new(0.0, -d, l),
        Point3::new(0.0, -d, -l),
        Point3::new(-l, 0.0, d),
        Point3::new(l, 0.0, d),
        Point3::new(l, 0.0, -d),
        Point3::new(-l, 0.0, -d),
    ];
    let attrs = StandardAttributes {
        positions,
        ..Default::default()
    };
    let faces = Faces::from_iter([
        [4, 14, 5, 17, 16],
        [6, 13, 7, 16, 17],
        [6, 17, 5, 8, 9],
        [4, 16, 7, 10, 11],
        [4, 11, 0, 15, 14],
        [1, 8, 5, 14, 15],
        [6, 9, 2, 12, 13],
        [3, 10, 7, 13, 12],
        [1, 15, 0, 19, 18],
        [1, 18, 2, 9, 8],
        [3, 12, 2, 18, 19],
        [3, 19, 0, 11, 10],
    ]);
    PolygonMesh::new(attrs, faces)
}

fn icosahedron() -> PolygonMesh {
    let dodeca: PolygonMesh = dodecahedron();
    // the positions of dodecahedron
    let dodeca_positions = dodeca.positions();
    // the vertices of isoahedron is the normalized vector of center of gravity
    let positions: Vec<Point3> = dodeca
        // iterator on all faces of the dodecahedron
        .face_iter()
        .map(|face| {
            // If we add the coordinates of the vertices of a face and normalize them, we can normalize the center of gravity.
            let normalized_gravity = face
                .iter()
                // we can obtain the coordinate index by `vertex.pos`
                // Convert coordinate data to `Vector3` with `Point3::to_vec` for easy operation
                .map(|vertex| dodeca_positions[vertex.pos].to_vec())
                .sum::<Vector3>()
                .normalize();
            Point3::from_vec(normalized_gravity)
        })
        .collect();
    let mut faces: Faces = (0..20)
        .map(|i| {
            // enumerate indices of all faces of dodecahedron which contains `i`
            dodeca
                .face_iter()
                .enumerate()
                // Convert `usize` to `StandardVertex` by `Into::into()`, and checks whether the vertex is included in the face.
                .filter(|(_, dodeca_face)| dodeca_face.contains(&i.into()))
                .map(|(idx, _)| idx)
                .collect::<Vec<usize>>()
        })
        .collect();
    faces.face_iter_mut().for_each(|face| {
        let p: Vec<Point3> = face.iter().map(|vertex| positions[vertex.pos]).collect();
        let face_center = p[0].to_vec() + p[1].to_vec() + p[2].to_vec();
        let face_normal = (p[1] - p[0]).cross(p[2] - p[0]).normalize();
        if face_center.dot(face_normal) < 0.0 {
            face.swap(0, 1);
        }
    });
    PolygonMesh::new(
        StandardAttributes {
            positions,
            ..Default::default()
        },
        faces,
    )
}

fn write_polyhedron(mut polygon: PolygonMesh, path: &str) {
    // create output obj file
    let mut obj = std::fs::File::create(path).unwrap();
    // add a normal to polyhedron. This will be learned in the later sections.
    polygon.add_naive_normals(true);
    // output polygon to obj file.
    obj::write(&polygon, &mut obj).unwrap();
}

fn main() {
    write_polyhedron(tetrahedron(), "tetrahedron.obj");
    write_polyhedron(hexahedron(), "hexahedron.obj");
    write_polyhedron(octahedron(), "octahedron.obj");
    write_polyhedron(dodecahedron(), "dodecahedron.obj");
    write_polyhedron(icosahedron(), "icosahedron.obj");
}

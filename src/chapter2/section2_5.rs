use truck_meshalgo::prelude::*;

fn write_polygon(polygon: &PolygonMesh, path: &str) {
    // create output obj file
    let mut obj = std::fs::File::create(path).unwrap();
    // output polygon to obj file.
    obj::write(polygon, &mut obj).unwrap();
}

fn main() {
    let mut mirror_ball = obj::read(include_bytes!("sphere.obj").as_slice()).unwrap();
    println!(
        "default mirror ball shell condition: {:?}",
        mirror_ball.shell_condition()
    );

    mirror_ball.put_together_same_attrs();
    println!(
        "after apply filter `put_together_same_attrs`: {:?}",
        mirror_ball.shell_condition()
    );

    mirror_ball.add_naive_normals(true);
    write_polygon(&mirror_ball, "mirror-ball.obj");

    mirror_ball.add_smooth_normals(1.0, true);
    write_polygon(&mirror_ball, "mirror-ball-with-smooth-normal.obj");
}

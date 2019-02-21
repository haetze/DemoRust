#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point3D {
    x_y: Point,
    z: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };
    let point3d = Point3D { x_y: point, z: 3 };
    // Convert the Point to a JSON string.
    let serialized = json::to_string(&point3d).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point3D = json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}


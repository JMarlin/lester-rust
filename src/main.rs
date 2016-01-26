struct Point {
    x: f32,
    y: f32
}

struct ScreenPoint {
    x: u32,
    y: u32,
    z: u16
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

struct Vertex<'a> {
    x: u32,
    y: u32,
    z: u32,
    c: &'a Color
}

struct Triangle<'a> {
    v: [Vertex<'a>; 3]
}

struct Body<'a> {
    tri_list: Vec<Triangle<'a>>,
    x: f32,
    y: f32,
    z: f32
}

fn main() {
    println!("Hello, world!");
}

pub struct Vector3 {
    x: f32, 
    y: f32, 
    z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }
}

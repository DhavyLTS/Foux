#[derive(Debug)]
pub struct Vector4<T> where T: Clone + Copy { 
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector4<T> where T: Clone + Copy {
    pub fn new(x:T, y:T, z:T, w: T) -> Vector4<T> {
        Vector4 { x, y, z, w }
    } 

    pub fn from_array(array: [T; 4]) -> Vector4<T> {
        Vector4 { 
            x: array[0], 
            y: array[1], 
            z: array[2], 
            w: array[3],
        }
    }

    pub fn as_array(&self) -> [T;4] {
        [self.x, self.y, self.z, self.w]
    }
}

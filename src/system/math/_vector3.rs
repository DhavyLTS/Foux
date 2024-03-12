#[derive(Debug)]
pub struct Vector3<T> where T: Clone + Copy { 
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> where T: Clone + Copy {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    } 

    pub fn from_array(array: [T; 3]) -> Vector3<T> {
        Vector3 { 
            x: array[0],
            y: array[1], 
            z: array[3],
        }
    }

    pub fn as_array(&self) -> [T; 3] {
        [ self.x, self.y, self.z ]
    }

}

use super::_vector4::Vector4;
use std::ops::AddAssign;


#[derive(Debug)]
pub struct Matrix4<T> where T: Clone + Copy + AddAssign {
    x: Vector4<T>,
    y: Vector4<T>,
    z: Vector4<T>,
    w: Vector4<T>,
}

impl<T> Matrix4<T> where T: Clone + Copy + AddAssign {
    pub fn new(x: T) -> Matrix4<T> {
        Matrix4 {
            x: Vector4::new(x, x, x, x),
            y: Vector4::new(x, x, x, x),
            z: Vector4::new(x, x, x, x),
            w: Vector4::new(x, x, x, x),
        }
    }


    pub fn from_array(array: [[T; 4]; 4]) -> Matrix4<T> {
        let x = Vector4::from_array(array[0]);
        let y = Vector4::from_array(array[1]);
        let z = Vector4::from_array(array[2]);
        let w = Vector4::from_array(array[3]);

        Matrix4 { x, y, z, w }
    }

    pub fn as_array(&self) -> [[T; 4]; 4] {
        [
            self.x.as_array(),
            self.y.as_array(),
            self.z.as_array(),
            self.w.as_array(),
        ]
    }   
} 

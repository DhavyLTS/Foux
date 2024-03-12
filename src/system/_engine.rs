extern crate glutin;
extern crate gl;

use super::{shaders::ShaderProgram, Window};

pub struct Engine {
    window: Window,
}

impl Engine { 
    pub fn new(window: Window) -> Engine { 
        Engine { window }
    }

    pub fn start(&mut self, shaders: ShaderProgram, vao: u32) {
       // self.window.run(shaders, vao);
    }
}

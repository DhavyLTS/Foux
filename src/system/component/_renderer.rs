use gl::types::{GLvoid, GLint, GLsizeiptr};
use crate::system::shaders::ShaderProgram;
use std::mem::size_of; 
use super::Mesh;

pub struct Renderer {
    
    shaders: ShaderProgram,
    mesh: Mesh,
    vao: u32,
    vbo: u32,
    ebo: u32,
    
}

impl Renderer {
    pub fn new(mesh: Mesh, shaders: ShaderProgram) -> Renderer {
        
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0; 
 
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            // Bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
            gl::BindVertexArray(vao);

            // Bind the Vertex Buffer Object, copy the defined vertex data into the buffer's memory
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (mesh.vertices().len() * size_of::<f32>()) as GLsizeiptr, mesh.vertices().as_ptr() as *const GLvoid, gl::STATIC_DRAW);

            // Bind the Element Buffer Object, copy the defined index data into the buffer's memory
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (mesh.indices().len() * size_of::<u32>()) as GLsizeiptr, mesh.indices().as_ptr() as *const GLvoid, gl::STATIC_DRAW);

            // Set the vertex attribute pointers
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * size_of::<f32>() as GLint, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            // Unbind VAO, VBO, and EBO
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Renderer { shaders, mesh, vao, vbo, ebo }
    }

    pub fn draw(&self) {
        unsafe {
            self.shaders.use_program();
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.mesh.indices().len() as i32, gl::UNSIGNED_INT, std::ptr::null());    
        }
    }

} 

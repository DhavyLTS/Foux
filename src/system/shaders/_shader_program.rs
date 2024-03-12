use super::_shader_compiler::compile_shader;

#[derive(Debug)]
pub struct ShaderProgram {
    id: gl::types::GLuint,
}

impl ShaderProgram { 
    pub fn new(vertex_src: &str, fragment_src: &str) -> ShaderProgram {
        unsafe {
            let id = gl::CreateProgram();

            let vertex_shader = compile_shader(vertex_src, gl::VERTEX_SHADER);
            let fragment_shader = compile_shader(fragment_src, gl::FRAGMENT_SHADER);
            
            gl::AttachShader(id, vertex_shader);
            gl::AttachShader(id, fragment_shader);
            
            gl::LinkProgram(id);
            
            gl::DetachShader(id, vertex_shader);
            gl::DetachShader(id, fragment_shader);
           
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            ShaderProgram { id }
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram( self.id ); 
        }
    }
}

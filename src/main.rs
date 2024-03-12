use foux::system::{component::{Mesh, Renderer}, shaders::ShaderProgram, Window};

fn main() {
    
    let mut window = Window::new(800, 600, "foux engine", glfw::WindowMode::Windowed);

    let fragment_shaders = std::fs::read_to_string("shaders/fragment.glsl").expect("Couldn't Read Shaders File!");
    let vertex_shaders = std::fs::read_to_string("shaders/vertex.glsl").expect("Couldn't Read Shaders File!");
    
    let shaders = ShaderProgram::new(&vertex_shaders, &fragment_shaders);

    let vertices: [f32; 12] = [
        -0.5, -0.5, 0.0, // Bottom-left vertex
         0.5, -0.5, 0.0,  // Bottom-right vertex
         0.5,  0.5, 0.0,   // Top-right vertex
        -0.5,  0.5, 0.0,  // Top-left vertex
    ];

    // Set up index data
    let indices: [u32; 6] = [
        0, 1, 2, // First triangle
        2, 0, 3, // Second triangle
    ];

    let mesh = Mesh::new(vertices.to_vec(), indices.to_vec());
    let renderer = Renderer::new(mesh, shaders); 
    
    let mut renderer_vector = Vec::new();
    renderer_vector.push(renderer);

    window.run(renderer_vector);
}

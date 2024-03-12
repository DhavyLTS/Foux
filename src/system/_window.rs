extern crate glfw;

use glfw::{Action, Context, Glfw, GlfwReceiver, PWindow, WindowEvent};
use super::component::Renderer;

pub struct Window { 
    _events: GlfwReceiver<(f64, WindowEvent)>,
    _window: PWindow,
    _glfw: Glfw,
}

impl Window {
    pub fn new(height: u32, width: u32, title: &str, mode: glfw::WindowMode) -> Window {
        
        let mut _glfw = glfw::init_no_callbacks().expect("Couldn't Init GLFW!");
        
        _glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        _glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        _glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        let (mut _window, _events) = _glfw.create_window(height, width, title, mode)
            .expect("Couldn't Create GLFW Window!");
        
        _window.make_current(); 
        _window.set_key_polling(true);

        gl::load_with(|symbol| _window.get_proc_address(symbol) as *const _);
        Window { _window, _events, _glfw  }
    }

    fn clear(&self) { 
        unsafe { gl::ClearColor(0.0, 0.0, 0.0, 0.0); }
    }

    pub fn run(&mut self, renderer_vector: Vec<Renderer>) {
        while !self._window.should_close() {
            self._glfw.poll_events();
            self.clear();    
             
            for renderer in &renderer_vector {
                renderer.draw();
            }
            
            if self._window.get_key(glfw::Key::Escape) == Action::Press {
                self._window.set_should_close(true);
            } 
            self.swap_buffers();
        }
    }

    fn swap_buffers(&mut self) {
        self._window.swap_buffers();
    }
}

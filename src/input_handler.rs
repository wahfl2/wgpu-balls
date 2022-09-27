use cgmath::Vector2;
use winit::{event::{KeyboardInput, ElementState}, dpi::PhysicalPosition};

pub struct InputHandler {
    mouse_pos: Option<PhysicalPosition<f64>>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            mouse_pos: None
        }
    }

    pub fn handle_kb_input(&self, input: &KeyboardInput) {

    }

    pub fn handle_cursor_movement(&mut self, input: &PhysicalPosition<f64>) {
        self.mouse_pos = Some(input.to_owned());
    }

    pub fn handle_mouse_button(&self, button_id: u32, state: ElementState ) {
        
    }
}
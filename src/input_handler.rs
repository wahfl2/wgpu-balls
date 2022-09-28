use winit::{event::{KeyboardInput, ElementState}, dpi::PhysicalPosition};

pub struct InputHandler {
    mouse_pos: Option<PhysicalPosition<f64>>,
    button_states: [bool; 32],
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            mouse_pos: None,
            button_states: [false; 32],
        }
    }

    pub fn handle_kb_input(&self, input: &KeyboardInput) {

    }

    pub fn handle_cursor_movement(&mut self, input: &PhysicalPosition<f64>) {
        self.mouse_pos = Some(input.to_owned());
    }

    pub fn handle_mouse_button(&mut self, button_id: u32, state: ElementState ) {
        if button_id >= 32 { panic!("There's no way you have 33 buttons on your mouse wtf") };
        let pressed = state == ElementState::Pressed;
        self.button_states[button_id as usize] = pressed;

        
    }
}
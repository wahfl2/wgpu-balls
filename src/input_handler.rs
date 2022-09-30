use cgmath::{Vector2, Quaternion, AbsDiffEq, Vector3};
use winit::{event::{KeyboardInput, ElementState}, dpi::PhysicalPosition};

use crate::{render_state::RenderState, instance::Instance};

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

    pub fn handle_mouse_button(&mut self, button_id: u32, state: ElementState, render_state: &mut RenderState ) {
        if button_id >= 32 { panic!("There's no way you have 33 buttons on your mouse wtf") };
        let pressed = state == ElementState::Pressed;
        self.button_states[button_id as usize] = pressed;

        if button_id == 1 && pressed {
            let pos = self.mouse_pos.unwrap_or_default();
    
            let instance_add = Instance {
                position: Vector2::new(pos.x as f32, pos.y as f32),
                rotation: Quaternion::from_sv(1.0, Vector3::unit_y()),
                scale: 100.0,
            };

            render_state.add_instance(instance_add);
        }
    }
}
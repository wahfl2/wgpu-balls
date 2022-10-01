use state::State;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder},
};

pub mod vertex;
pub mod state;
pub mod instance;
pub mod render_state;
pub mod input_handler;
pub mod util;
pub mod uniform;
pub mod physics;

pub fn main() {
    pollster::block_on(run());
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_maximized(true);

    let mut app_state = State::new(&window).await;

    event_loop.run(move |event, _, control_flow| match event {
        // Window events
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            // Close window on close or ESC
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,

            WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
                app_state.input_handler.handle_kb_input(input);
            },

            WindowEvent::CursorMoved { device_id: _, position, modifiers: _ } => {
                app_state.input_handler.handle_cursor_movement(position);
            }

            WindowEvent::Resized(physical_size) => {
                app_state.render_state.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                app_state.render_state.resize(**new_inner_size);
            }
            _ => {}
        },

        Event::DeviceEvent { 
            device_id, 
            event
        } => match event {
            DeviceEvent::Button { button, state } => {
                app_state.input_handler.handle_mouse_button(button, state, &mut app_state.render_state);
            }
            _ => {}
        }

        Event::RedrawRequested(window_id) if window_id == window.id() => {
            app_state.update(control_flow);
        },
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        },
        _ => {}
    });
}

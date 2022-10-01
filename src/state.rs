use winit::{window::Window, event_loop::ControlFlow};

use crate::{render_state::RenderState, input_handler::InputHandler, physics::Physics};

pub struct State {
    pub(crate) render_state: RenderState,
    pub(crate) input_handler: InputHandler,
    pub(crate) physics: Physics,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        Self {
            render_state: RenderState::new(window).await,
            input_handler: InputHandler::new(),
            physics: Physics::new(),
        }
    }

    pub fn update(&mut self, control_flow: &mut ControlFlow) {
        self.physics.update();
        match self.render_state.render() {
            Ok(_) => {}
            // Reconfigure the surface if lost
            Err(wgpu::SurfaceError::Lost) => self.render_state.resize(self.render_state.size),
            // The system is out of memory, we should probably quit
            Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
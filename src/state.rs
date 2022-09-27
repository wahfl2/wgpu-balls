use winit::window::Window;

use crate::{render_state::RenderState, input_handler::InputHandler};

pub struct State {
    pub(crate) render_state: RenderState,
    pub(crate) input_handler: InputHandler,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        Self {
            render_state: RenderState::new(window).await,
            input_handler: InputHandler::new(),
        }
    }
}
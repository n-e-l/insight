mod graphics_pass;

use std::sync::{Arc, Mutex};
use cen::app::app::AppConfig;
use cen::app::Cen;
use cen::app::component::{Component, ComponentRegistry};
use cen::app::engine::InitContext;
use cen::graphics::renderer::{RenderComponent, RenderContext};
use dotenv::dotenv;
use crate::graphics_pass::{GraphicsPass, PassParameters};

struct Application {
    pass: GraphicsPass
}

impl Application {

    fn new(_ctx: &mut InitContext) -> Application {
        Self {
            pass: GraphicsPass::default()
        }
    }
}

impl RenderComponent for Application {
    fn render(&mut self, ctx: &mut RenderContext) {
        let params = PassParameters {
            command_buffer: ctx.command_buffer,
            output_image: ctx.swapchain_image
        };
        self.pass.render(
            params
        );
    }
}

fn main() {
    // Initialize .env environment variables
    dotenv().ok();

    Cen::run(
        AppConfig::default()
            .width(1180)
            .height(1180)
            .log_fps(true)
            .resizable(true)
            .vsync(false),
        Box::new(|ctx| {
            let compute = Arc::new(Mutex::new(Application::new(ctx)));
            ComponentRegistry::new()
                .register(Component::Render(compute))
        })
    );
}

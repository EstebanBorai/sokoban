use ggez::graphics::{Color, DrawParam, Drawable, FilterMode, Image, Text, clear, draw, draw, draw_queued_text, present, queue_text};
use ggez::Context;
use specs::{Join, ReadStorage, System};

use crate::component::{Position, Renderable};

const TILE_SIZE_FACTOR: f32 = 32.;

pub struct RenderingSystem<'a> {
    context: &'a mut Context,
}

impl<'a> RenderingSystem<'a> {
    pub fn new(context: &'a mut Context) -> Self {
        RenderingSystem { context }
    }

    pub fn draw_text(&mut self, message: &str, x: f32, y: f32) {
        let text = Text::new(message);
        let color = Color::new(0., 0., 0., 1.);
        let dimensions = (0., 0.20);
        let destination = (x, y);
        let draw_param = DrawParam::new();

        queue_text(self.context, &text, dimensions.into(), Some(color));
        draw_queued_text(
            self.context,
                draw_param.dest([x, y]),
            None,
            FilterMode::Linear,
            )
        .expect("Failed to draw text");
    }
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        // Clear the screen
        clear(self.context, Color::new(0.95, 0.95, 0.95, 1.));

        // Get all the renderables with their positions and sort by the
        // position z.
        // This will allow us to have entities layered visually.
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            let image = Image::new(self.context, renderable.path.clone()).expect(&format!(
                "Expected image file on {}",
                renderable.path.clone()
            ));
            let x = position.x as f32 * TILE_SIZE_FACTOR;
            let y = position.y as f32 * TILE_SIZE_FACTOR;
            let draw_param = DrawParam::new();

            // Finally, present the context, this will actually display
            // everything on the screen.
            draw(self.context, &image, draw_param.dest([x, y])).expect("Failed to render");
        }

        present(self.context).expect("Failed to present");
    }
}

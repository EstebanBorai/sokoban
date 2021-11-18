use ggez::graphics::{
    clear, draw, draw_queued_text, present, queue_text, Color, DrawParam, FilterMode, Font, Image,
    PxScale, Text,
};
use ggez::Context;
use specs::{Join, Read, ReadStorage, System};
use std::path::PathBuf;
use std::str::FromStr;

use crate::component::{Position, Renderable};
use crate::resource::Gameplay;

const TILE_SIZE_FACTOR: f32 = 32.;

pub struct RenderingSystem<'a> {
    context: &'a mut Context,
    font: Font,
}

impl<'a> RenderingSystem<'a> {
    pub fn new(context: &'a mut Context) -> Self {
        let font = Font::new(
            context,
            PathBuf::from_str("/PressStart2P-Regular.ttf").unwrap(),
        )
        .expect("Failed to find \"PressStart2P-Regular.ttf\" asset");

        RenderingSystem { context, font }
    }

    pub fn draw_text(&mut self, message: &str, x: f32, y: f32) {
        let mut text = Text::new(message);
        text.set_font(self.font, PxScale::from(24.));

        let color = Color::WHITE;
        let dimensions: [f32; 2] = [0., 0.20];
        let draw_param = DrawParam::new();

        queue_text::<[f32; 2]>(self.context, &text, dimensions, Some(color));
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
    type SystemData = (
        Read<'a, Gameplay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, positions, renderables) = data;

        // Clear the screen
        clear(self.context, Color::BLACK);

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

        self.draw_text(&gameplay.state.to_string(), 400., 550.);
        self.draw_text(&gameplay.moves.to_string(), 50., 550.);

        present(self.context).expect("Failed to present");
    }
}

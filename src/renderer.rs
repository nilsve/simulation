use macroquad::prelude::*;
use crate::{Creature, KillRegion};

const ROWS: f32 = 100.0;
const COLUMNS: f32 = 100.0;

pub async fn render_screen(creatures: &[Creature<'_>], kill_regions: &[KillRegion]) {
    clear_background(WHITE);

    let screen_height = screen_height();
    let screen_width = screen_width();

    let creature_height = screen_height / ROWS;
    let creature_width = screen_width / COLUMNS;

    kill_regions.iter().for_each(|region| {
        draw_rectangle(region.top_left.0 as f32 * creature_width, region.top_left.1 as f32 * creature_height, (region.bottom_right.0 - region.top_left.0) as f32 * creature_width, (region.bottom_right.1 - region.top_left.1) as f32 * creature_height, RED);
    });

    creatures.iter().for_each(|creature| {
        let x = creature.creature_data.position.0 as f32 * creature_width;
        let y = creature.creature_data.position.1 as f32 * creature_height;

        draw_rectangle(x, y, creature_width, creature_height, BLACK);
    });

    next_frame().await
}

mod collisions;
mod entity_render;
mod map_render;
mod player_input;
mod random_moves;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(random_moves::random_moves_system())
        .build()
}

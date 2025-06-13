#![allow(unused)]
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{components::Bird, resources::is_game_active};

#[derive(Resource)]
pub struct MousePosition(Vec2);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(MousePosition(Vec2::default()))
        .add_systems(Update, handle_input.run_if(is_game_active));
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_position: Res<ButtonInput<MouseButton>>,
    mut bird_query: Query<&mut LinearVelocity, With<Bird>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut velocity in bird_query.iter_mut() {
            velocity.y = 350.0;
        }
    }
}

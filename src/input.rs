use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{components::Bird, resources::GameState};

#[derive(Resource)]
pub struct MousePosition(Vec2);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(MousePosition(Vec2::default()))
        .add_systems(
            Update,
            handle_bird_input.run_if(in_state(GameState::Active)),
        );
}

fn handle_bird_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut bird_query: Query<&mut LinearVelocity, With<Bird>>,
    current_state: Res<State<GameState>>,
    mut just_entered_active: Local<bool>,
) {
    // Check if we just entered the Active state
    if current_state.is_changed() && *current_state.get() == GameState::Active {
        *just_entered_active = true;
        return;
    }

    // Skip input processing for one frame after entering Active state
    if *just_entered_active {
        *just_entered_active = false;
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Space) || mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(mut velocity) = bird_query.single_mut() {
            velocity.y = 350.0;
        }
    }
}

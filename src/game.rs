#![allow(unused)]

use avian2d::{math::*, prelude::*};
use bevy::{app::App, prelude::*};

use crate::{
    components::*,
    default::{WINDOW_HEIGHT, WINDOW_WIDTH},
    resources::{Game, GameSpeed, is_game_active, is_game_not_active},
    utils::random_number,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, license_print)
        .add_systems(
            Update,
            (move_background, move_ground, animate_bird).run_if(is_game_active),
        )
        .add_systems(Update, start_game.run_if(is_game_not_active));
}

fn start_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut game_speed: ResMut<GameSpeed>,
    mut bird_query: Query<(&mut Transform, &mut LinearVelocity), With<Bird>>,
    mut visibility_query: Query<&mut Visibility, With<PressSpacebarText>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Change game state to Active
        game.state = GameState::Active;

        // Reset score
        game.score = 0;

        // Reset game speed
        game_speed.reset();

        // Reset bird position and velocity
        for (mut transform, mut velocity) in bird_query.iter_mut() {
            transform.translation = Vec3::new(-WINDOW_WIDTH / 2.0 + 100.0, 0.0, 0.0);
            velocity.x = 0.0;
            velocity.y = 0.0;
        }

        // Hide "Press Spacebar" text
        for mut visibility in visibility_query.iter_mut() {
            *visibility = Visibility::Hidden;
        }

        println!("Game Started!");
    }
}

fn move_background(
    time: Res<Time>,
    game_speed: Res<GameSpeed>,
    mut bg_query: Query<&mut Transform, With<Background>>,
) {
    let delta = time.delta_secs();
    let delta_x = game_speed.get_current_speed() * delta;

    for mut background_transform in bg_query.iter_mut() {
        background_transform.translation.x -= delta_x;

        if background_transform.translation.x <= -288.0 {
            background_transform.translation.x = 0.0;
        }
    }
}

fn move_ground(
    time: Res<Time>,
    game_speed: Res<GameSpeed>,
    mut base_query: Query<&mut Transform, With<Ground>>,
) {
    let delta = time.delta_secs();
    let delta_x = game_speed.get_current_speed() * delta;

    for mut ground_transform in base_query.iter_mut() {
        ground_transform.translation.x -= delta_x;

        if ground_transform.translation.x <= -288.0 {
            ground_transform.translation.x = 0.0;
        }
    }
}

pub fn animate_bird(time: Res<Time>, mut query: Query<(&mut Bird, &mut Sprite), With<Bird>>) {
    for (mut bird, mut sprite) in query.iter_mut() {
        bird.timer.tick(time.delta());

        if bird.timer.finished() {
            if let Some(ref mut texture_atlas) = sprite.texture_atlas {
                texture_atlas.index = if texture_atlas.index == 2 {
                    0
                } else {
                    texture_atlas.index + 1
                };
            }
        }
    }
}

fn license_print() {
    println!("Crabby Bird (c) 2025 CDzungx.");
}

use avian2d::prelude::*;
use bevy::{app::App, prelude::*};

use crate::{
    components::*,
    game_state::GameOverEvent,
    resources::{Game, GameSpeed, GameState},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, license_print)
        .add_systems(
            Update,
            (
                wrap_tiled_background,
                animate_bird,
                update_bird_angle,
                handle_collisions,
                handle_score_detection,
            )
                .run_if(in_state(GameState::Active)),
        );
}

fn wrap_tiled_background(
    mut query: Query<&mut Transform, (With<Scrolling>, Or<(With<Background>, With<Ground>)>)>,
) {
    for mut transform in query.iter_mut() {
        if transform.translation.x <= -288.0 {
            transform.translation.x = 0.0;
        }
    }
}

fn animate_bird(time: Res<Time>, mut query: Query<(&mut Bird, &mut Sprite)>) {
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

fn update_bird_angle(mut bird_query: Query<(&mut Transform, &LinearVelocity), With<Bird>>) {
    if let Ok((mut transform, velocity)) = bird_query.single_mut() {
        // Normalize velocity to a range
        let normalized_velocity = (velocity.y / 350.0).clamp(-1.0, 1.0);

        let max_angle = 0.7;
        let target_angle = normalized_velocity * max_angle;

        // Smoothly interpolate to the target angle
        let current_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        let new_angle = current_angle + (target_angle - current_angle) * 0.1;

        transform.rotation = Quat::from_rotation_z(new_angle);
    }
}

fn handle_collisions(
    collisions: Collisions,
    mut game_over_events: EventWriter<GameOverEvent>,
    bird_query: Query<Entity, With<Bird>>,
    collidable_query: Query<Entity, With<Collidable>>,
) {
    if let Ok(bird_entity) = bird_query.single() {
        for collidable_entity in collidable_query.iter() {
            if collisions.contains(bird_entity, collidable_entity) {
                println!("Collision detected!");
                game_over_events.write_default();
                // We only need one collision to end the game, so we can exit early.
                return;
            }
        }
    }
}

fn handle_score_detection(
    mut commands: Commands,
    collisions: Collisions,
    bird_query: Query<Entity, With<Bird>>,
    sensor_query: Query<Entity, With<ScoreSensor>>,
    mut game: ResMut<Game>,
    mut score_text_query: Query<&mut Text2d, With<ScoreText>>,
) {
    if let Ok(bird_entity) = bird_query.single() {
        for sensor_entity in sensor_query.iter() {
            if collisions.contains(bird_entity, sensor_entity) {
                // Remove the sensor component to prevent multiple score increments
                commands
                    .entity(sensor_entity)
                    .remove::<ScoreSensor>();

                // Increment the score
                game.score += 1;
                println!("Score: {}", game.score);

                // Update the score text
                if let Ok(mut text) = score_text_query.single_mut() {
                    text.0 = game.score.to_string();
                }
            }
        }
    }
}

fn license_print() {
    println!("Crabby Bird (c) 2025 CDzungx.");
}

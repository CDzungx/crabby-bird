use avian2d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

use crate::{
    components::{Collidable, LowerPipe, PipePair, ScoreSensor, Scrolling, UpperPipe},
    default::{WINDOW_HEIGHT, WINDOW_WIDTH},
    resources::{GameSpeed, GameState},
};

const PIPE_SPAWN_INTERVAL: f32 = 2.2;
const PIPE_WIDTH: f32 = 52.0;
const PIPE_HEIGHT: f32 = 320.0;
const MAX_GAP: f32 = 220.0;
const MIN_GAP: f32 = 120.0;

#[derive(Resource)]
struct PipeSpawnTimer(Timer);

impl Default for PipeSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            PIPE_SPAWN_INTERVAL,
            TimerMode::Repeating,
        ))
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PipeSpawnTimer>().add_systems(
        Update,
        (spawn_pipes, despawn_offscreen_pipes).run_if(in_state(GameState::Active)),
    );
}

fn spawn_pipes(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<PipeSpawnTimer>,
    asset_server: Res<AssetServer>,
    game_speed: Res<GameSpeed>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    let mut rng = rand::rng();
    // The ground is 100px high, and its center is at -256, so its top is at -206.
    // The ceiling is a line at 256.
    let ground_top = -WINDOW_HEIGHT / 2.0 + 50.0;
    let ceiling_bottom = WINDOW_HEIGHT / 2.0;

    // Add some padding to prevent pipes from spawning too close to the edges.
    let padding = 50.0;
    let pipe_gap = rng.random_range(MIN_GAP..MAX_GAP);
    let min_y = ground_top + pipe_gap / 2.0 + padding;
    let max_y = ceiling_bottom - pipe_gap / 2.0 - padding;

    let random_y = rng.random_range(min_y..max_y);
    let speed = game_speed.get_current_speed();

    commands.spawn((
        PipePair,
        Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2.0 + PIPE_WIDTH, random_y, -3.0)),
        Visibility::default(),
        Name::new("PipePair"),
        children![
            upper_pipe_bundle(&asset_server, speed, pipe_gap),
            lower_pipe_bundle(&asset_server, speed, pipe_gap),
            score_sensor_bundle(pipe_gap, speed)
        ],
    ));
}

fn despawn_offscreen_pipes(
    mut commands: Commands,
    pipe_query: Query<(Entity, &GlobalTransform), With<PipePair>>,
) {
    for (entity, transform) in pipe_query.iter() {
        // Despawn pipes once they are off-screen to the left
        if transform.translation().x < -WINDOW_WIDTH / 2.0 - PIPE_WIDTH {
            commands.entity(entity).despawn();
        }
    }
}

fn upper_pipe_bundle(asset_server: &AssetServer, speed: f32, pipe_gap: f32) -> impl Bundle {
    (
        Sprite {
            image: asset_server.load("textures/pipe.png"),
            flip_y: true,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, PIPE_HEIGHT / 2.0 + pipe_gap / 2.0, -2.0)),
        RigidBody::Kinematic,
        Collider::rectangle(PIPE_WIDTH, PIPE_HEIGHT),
        LinearVelocity(Vec2::new(-speed, 0.0)),
        UpperPipe,
        Collidable,
        Scrolling,
        Name::new("UpperPipe"),
    )
}

fn lower_pipe_bundle(asset_server: &AssetServer, speed: f32, pipe_gap: f32) -> impl Bundle {
    (
        Sprite {
            image: asset_server.load("textures/pipe.png"),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, -PIPE_HEIGHT / 2.0 - pipe_gap / 2.0, -2.0)),
        RigidBody::Kinematic,
        Collider::rectangle(PIPE_WIDTH, PIPE_HEIGHT),
        LinearVelocity(Vec2::new(-speed, 0.0)),
        LowerPipe,
        Collidable,
        Scrolling,
        Name::new("LowerPipe"),
    )
}

fn score_sensor_bundle(pipe_gap: f32, speed: f32) -> impl Bundle {
    (
        Sprite {
            color: Color::WHITE,
            ..default()
        },
        RigidBody::Kinematic,
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        LinearVelocity(Vec2::new(-speed, 0.0)),
        Collider::rectangle(PIPE_WIDTH, pipe_gap + 50.0),
        Sensor,
        ScoreSensor,
        Scrolling,
        Name::new("ScoreSensor"),
    )
}

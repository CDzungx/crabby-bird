use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    components::*,
    default::{GRAVITY, WINDOW_WIDTH},
    resources::{Game, GameSpeed, GameState},
};

pub(super) fn plugin(app: &mut App) {
    app
        // Add the event for game over
        .add_event::<GameOverEvent>()
        // Add systems that run on state transitions
        .add_systems(OnEnter(GameState::Active), on_enter_active)
        .add_systems(OnEnter(GameState::GameOver), on_enter_game_over)
        .add_systems(OnEnter(GameState::Inactive), on_enter_inactive)
        // Add systems that run during specific states
        .add_systems(
            Update,
            (
                transition_to_active_on_input.run_if(in_state(GameState::Inactive)),
                transition_to_inactive_on_input.run_if(in_state(GameState::GameOver)),
                handle_game_over_event,
            ),
        )
        // Add systems that control physics based on state
        .add_systems(Update, (control_bird_physics, control_scrolling_velocity));
}

/// Event triggered when the bird collides with something.
#[derive(Event, Default)]
pub struct GameOverEvent;

// --- State Transition Systems ---

fn transition_to_active_on_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Active);
    }
}

fn transition_to_inactive_on_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Inactive);
    }
}

fn handle_game_over_event(
    mut events: EventReader<GameOverEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !events.is_empty() {
        events.clear();
        next_state.set(GameState::GameOver);
    }
}

// --- OnEnter Systems ---

fn on_enter_active(
    mut game: ResMut<Game>,
    mut game_speed: ResMut<GameSpeed>,
    mut press_spacebar_query: Query<&mut Visibility, With<PressSpacebarText>>,
) {
    // Reset game data
    game.score = 0;
    game_speed.reset();

    // Hide the "Press Spacebar" text
    if let Ok(mut visibility) = press_spacebar_query.single_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn on_enter_game_over(
    mut commands: Commands,
    mut game_over_query: Query<&mut Visibility, With<GameOverText>>,
    sensor_query: Query<Entity, With<ScoreSensor>>,
) {
    // Show "Game Over" text
    if let Ok(mut visibility) = game_over_query.single_mut() {
        *visibility = Visibility::Visible;
    }

    // Despawn all score sensors
    for sensor in sensor_query.iter() {
        commands.entity(sensor).despawn();
    }
}

fn on_enter_inactive(
    mut commands: Commands,
    pipe_query: Query<Entity, With<PipePair>>,
    mut score_text_query: Query<&mut Text2d, With<ScoreText>>,
    mut game_over_query: Query<&mut Visibility, (With<GameOverText>, Without<PressSpacebarText>)>,
    mut bird_query: Query<(&mut Transform, &mut LinearVelocity, &mut AngularVelocity), With<Bird>>,
    mut press_spacebar_query: Query<
        &mut Visibility,
        (With<PressSpacebarText>, Without<GameOverText>),
    >,
) {
    // Clear all pipes
    for entity in pipe_query.iter() {
        commands.entity(entity).despawn();
    }

    // Reset score display
    if let Ok(mut text) = score_text_query.single_mut() {
        **text = "0".to_string();
    }

    // Reset bird
    if let Ok((mut transform, mut lin_vel, mut ang_vel)) = bird_query.single_mut() {
        transform.translation = Vec3::new(-WINDOW_WIDTH / 2.0 + 100.0, 0.0, 0.0);
        transform.rotation = Quat::IDENTITY;
        lin_vel.x = 0.0;
        lin_vel.y = 0.0;
        ang_vel.0 = 0.0;
    }

    // Hide "Game Over" text and show "Press Spacebar" text
    if let Ok(mut visibility) = game_over_query.single_mut() {
        *visibility = Visibility::Hidden;
    }
    if let Ok(mut visibility) = press_spacebar_query.single_mut() {
        *visibility = Visibility::Visible;
    }
}

// --- General Systems ---

fn control_bird_physics(
    mut bird_query: Query<
        (&mut LinearVelocity, &mut AngularVelocity, &mut GravityScale),
        With<Bird>,
    >,
    current_state: Res<State<GameState>>,
) {
    if let Ok((mut lin_vel, mut ang_vel, mut gravity)) = bird_query.single_mut() {
        match current_state.get() {
            GameState::Active => {
                gravity.0 = GRAVITY;
            },
            GameState::Inactive | GameState::GameOver => {
                gravity.0 = 0.0;
                lin_vel.x = 0.0;
                lin_vel.y = 0.0;
                ang_vel.0 = 0.0;
            },
        }
    }
}

fn control_scrolling_velocity(
    mut query: Query<&mut LinearVelocity, With<Scrolling>>,
    game_state: Res<State<GameState>>,
    game_speed: Res<GameSpeed>,
) {
    let new_x_vel = match game_state.get() {
        GameState::Active => -game_speed.get_current_speed(),
        GameState::Inactive | GameState::GameOver => 0.0,
    };

    for mut velocity in query.iter_mut() {
        velocity.x = new_x_vel;
    }
}

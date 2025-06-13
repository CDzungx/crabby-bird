use bevy::prelude::*;

#[derive(Resource)]
pub struct Game {
    pub score: u32,
    pub state: GameState,
}

#[derive(PartialEq)]
pub enum GameState {
    Inactive,
    Active,
    GameOver,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Inactive
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            score: 0,
            state: GameState::Inactive,
        }
    }
}

pub fn is_game_active(game: Res<Game>) -> bool {
    game.state == GameState::Active
}

pub fn is_game_not_active(game: Res<Game>) -> bool {
    game.state != GameState::Active
}

#[derive(Resource)]
pub struct GameSpeed {
    pub base_speed: f32,
    pub current_multiplier: f32,
    pub time_elapsed: f32,
    pub speed_increase_interval: f32, // How often to increase speed (in seconds)
    pub speed_increase_amount: f32,   // How much to increase each time
}

impl Default for GameSpeed {
    fn default() -> Self {
        Self {
            base_speed: 150.0,       // Base movement speed
            current_multiplier: 1.0, // Start at 1x speed
            time_elapsed: 0.0,
            speed_increase_interval: 10.0, // Increase every 10 seconds
            speed_increase_amount: 0.05,   // Increase by 5% each time
        }
    }
}

impl GameSpeed {
    pub fn get_current_speed(&self) -> f32 {
        self.base_speed * self.current_multiplier
    }

    pub fn reset(&mut self) {
        self.current_multiplier = 1.0;
        self.time_elapsed = 0.0;
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Game>()
        .init_resource::<GameSpeed>()
        .add_systems(Update, update_game_speed);
}

fn update_game_speed(mut game_speed: ResMut<GameSpeed>, time: Res<Time>) {
    game_speed.time_elapsed += time.delta_secs();

    // Check if it's time to increase speed
    if game_speed.time_elapsed >= game_speed.speed_increase_interval {
        game_speed.current_multiplier *= 1.0 + game_speed.speed_increase_amount;
        game_speed.time_elapsed = 0.0; // Reset timer

        // Optional: Cap the maximum speed
        game_speed.current_multiplier = game_speed.current_multiplier.min(3.0); // Max 3x speed

        println!(
            "Speed increased! Current multiplier: {:.1}x",
            game_speed.current_multiplier
        );
    }
}

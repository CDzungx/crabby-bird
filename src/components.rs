use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Ceiling;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct PressSpacebarText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Bird {
    pub timer: Timer,
}

#[derive(Component)]
pub struct UpperPipe;

#[derive(Component)]
pub struct LowerPipe;

#[derive(Component)]
pub struct PipePair;

#[derive(Component)]
pub struct ScoreSensor;

#[derive(Component)]
pub struct Collidable;

#[derive(Component)]
pub struct Scrolling;

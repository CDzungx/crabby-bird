use avian2d::prelude::*;
use bevy::{asset::AssetServer, prelude::*, sprite::Anchor};

use crate::{
    components::*,
    default::{WINDOW_HEIGHT, WINDOW_WIDTH},
};

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_camera);
}

fn initialize_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let font = asset_server.load("fonts/editundo.ttf");
    let text_font = TextFont {
        font: font.clone(),
        ..Default::default()
    };

    commands.spawn(MainCamera);

    // Spawn tiled background
    commands.spawn((
        Sprite {
            image: asset_server.load("textures/background.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.0,
            },
            custom_size: Some(Vec2::new(WINDOW_WIDTH + 288.0 * 2., WINDOW_HEIGHT)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, -5.0)),
        Background,
    ));

    // Spawn tiled ground
    commands.spawn((
        Sprite {
            image: asset_server.load("textures/base.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 1.0,
            },
            custom_size: Some(Vec2::new(WINDOW_WIDTH + 288.0 * 2., 100.0)),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, -WINDOW_HEIGHT / 2.0, -1.0)),
        RigidBody::Static,
        Collider::rectangle(WINDOW_WIDTH, 100.0),
        Ground,
    ));

    // Bird spawn
    commands.spawn((
        Sprite {
            image: asset_server.load("textures/bird.png"),
            texture_atlas: Some(TextureAtlas {
                index: 1,
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    UVec2 { x: 34, y: 24 },
                    3,
                    1,
                    None,
                    None,
                )),
            }),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2.0 + 100.0, 0.0, 0.0)),
        RigidBody::Dynamic,
        Collider::circle(16.0),
        LinearVelocity::ZERO,
        GravityScale(65.0),
        Bird {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        },
    ));

    // Upper Pipe
    commands.spawn((
        Sprite {
            image: asset_server.load("textures/pipe.png"),
            flip_y: true,
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(350.0, WINDOW_HEIGHT / 2.0, -3.0)),
        RigidBody::Kinematic,
        Collider::rectangle(52.0, 320.0),
        LinearVelocity(Vec2::new(-150.0, 0.0)),
        UpperPipe,
    ));

    // Lower Pipe
    commands.spawn((
        Sprite {
            image: asset_server.load("textures/pipe.png"),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(350.0, -WINDOW_HEIGHT / 2.0, -3.0)),
        RigidBody::Kinematic,
        Collider::rectangle(52.0, 320.0),
        LinearVelocity(Vec2::new(-150.0, 0.0)),
        LowerPipe,
    ));

    // Score Text
    commands.spawn((
        Text2d::new("1000"),
        text_font.clone().with_font_size(40.0),
        TextLayout::default(),
        Anchor::CenterLeft,
        Transform::from_translation(Vec3::new(
            -WINDOW_WIDTH / 2.0 + 20.0,
            WINDOW_HEIGHT / 2.0 - 50.0,
            1.0,
        )),
        ScoreText,
    ));

    // Gameover Text
    commands.spawn((
        Text2d::new("Game Over"),
        text_font.clone().with_font_size(50.0),
        TextLayout::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Visibility::Hidden,
        GameOverText,
    ));

    // Press Spacebar Text
    commands.spawn((
        Text2d::new("Press Spacebar"),
        text_font.clone().with_font_size(40.0),
        TextLayout::default(),
        Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
        PressSpacebarText,
    ));
}

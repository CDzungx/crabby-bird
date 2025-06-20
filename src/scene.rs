use avian2d::prelude::*;
use bevy::{asset::AssetServer, prelude::*, sprite::Anchor};

use crate::{
    components::*,
    default::{GRAVITY, WINDOW_HEIGHT, WINDOW_WIDTH},
};

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_scene);
}

fn initialize_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let font = asset_server.load("fonts/editundo.ttf");

    // Spawn all game entities
    commands.spawn(camera_bundle());
    commands.spawn(background_bundle(&asset_server));
    commands.spawn(ground_bundle(&asset_server));
    commands.spawn(ceiling_bundle());
    commands.spawn(bird_bundle(&asset_server, &mut texture_atlas_layouts));

    // Spawn UI elements as a group
    commands.spawn(ui_bundle(&font));
}

// Bundle functions for better organization
fn camera_bundle() -> impl Bundle {
    MainCamera
}

fn background_bundle(asset_server: &AssetServer) -> impl Bundle {
    (
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
        RigidBody::Kinematic,
        LinearVelocity::ZERO,
        Background,
        Scrolling,
    )
}

fn ground_bundle(asset_server: &AssetServer) -> impl Bundle {
    (
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
        RigidBody::Kinematic,
        Collider::rectangle(WINDOW_WIDTH, 100.0),
        Ground,
        Collidable,
        Scrolling,
    )
}

fn ceiling_bundle() -> impl Bundle {
    (
        // The ceiling is an invisible static body at the top of the screen
        Transform::from_translation(Vec3::new(0.0, WINDOW_HEIGHT / 2.0, 0.0)),
        RigidBody::Static,
        Collider::rectangle(WINDOW_WIDTH, 1.0),
        Ceiling,
        Collidable,
    )
}

fn bird_bundle(
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
) -> impl Bundle {
    (
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
        GravityScale(GRAVITY),
        Bird {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        },
    )
}

fn ui_bundle(font: &Handle<Font>) -> impl Bundle {
    (
        // Root UI container with proper visibility components
        Transform::from_translation(Vec3::ZERO),
        Visibility::Inherited,
        children![
            score_text_bundle(font),
            game_over_text_bundle(font),
            press_spacebar_text_bundle(font),
        ],
    )
}

fn score_text_bundle(font: &Handle<Font>) -> impl Bundle {
    (
        Text2d::new("0"),
        TextFont {
            font: font.clone(),
            font_size: 40.0,
            ..Default::default()
        },
        TextLayout::default(),
        Anchor::CenterLeft,
        Transform::from_translation(Vec3::new(
            -WINDOW_WIDTH / 2.0 + 20.0,
            WINDOW_HEIGHT / 2.0 - 50.0,
            1.0,
        )),
        ScoreText,
    )
}

fn game_over_text_bundle(font: &Handle<Font>) -> impl Bundle {
    (
        Text2d::new("Game Over"),
        TextFont {
            font: font.clone(),
            font_size: 50.0,
            ..Default::default()
        },
        TextLayout::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Visibility::Hidden,
        GameOverText,
    )
}

fn press_spacebar_text_bundle(font: &Handle<Font>) -> impl Bundle {
    (
        Text2d::new("Press Spacebar"),
        TextFont {
            font: font.clone(),
            font_size: 40.0,
            ..Default::default()
        },
        TextLayout::default(),
        Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
        PressSpacebarText,
    )
}

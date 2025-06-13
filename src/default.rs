use bevy::{prelude::*, window::PresentMode};

pub const WINDOW_WIDTH: f32 = 800.;
pub const WINDOW_HEIGHT: f32 = 512.;
// Sets up the default plugins like windows, assets, etc

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Crabby Bird".into(),
            resizable: false,
            resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
            canvas: Some("#bevy".to_owned()),
            desired_maximum_frame_latency: core::num::NonZero::new(1u32),
            fit_canvas_to_parent: true,
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    }));
}

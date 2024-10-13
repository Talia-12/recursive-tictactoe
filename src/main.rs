mod board;
mod ui;

use bevy::{
	input::common_conditions::input_toggle_active, prelude::*, render::camera::ScalingMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board::BoardPlugin;

fn main() {
	App::new()
		.add_plugins(
			DefaultPlugins
				// .set(ImagePlugin::default_nearest())
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: "Recursive TicTacToe".into(),
						..default()
					}),
					..default()
				}),
		)
		.insert_resource(ClearColor(Color::WHITE))
		.add_plugins(
			WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
		)
		.add_plugins((BoardPlugin,))
		.add_systems(Startup, setup)
		.run();
}

fn setup(mut commands: Commands) {
	let mut camera = Camera2dBundle::default();
	camera.projection.scaling_mode = ScalingMode::FixedVertical(1.0);
	commands.spawn(camera);
}

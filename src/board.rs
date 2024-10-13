use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub struct BoardPlugin;

const MAX_LEVEL: u8 = 0;
const MAX_BOARD_SIZE: f32 = 0.96;

impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<BoardSprites>()
			.add_plugins(DefaultPickingPlugins)
			.add_systems(Startup, spawn_full_board)
			.add_systems(Update, render_tiles);
	}
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct BoardSprites {
	board_border: Handle<Image>,
	empty: Handle<Image>,
	cross: Handle<Image>,
	circle: Handle<Image>,
}

impl FromWorld for BoardSprites {
	fn from_world(world: &mut World) -> Self {
		let board_border = world.load_asset("board-border.png");
		let empty = world.load_asset("empty.png");
		let cross = world.load_asset("cross.png");
		let circle = world.load_asset("circle.png");

		BoardSprites {
			board_border,
			empty,
			cross,
			circle,
		}
	}
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Board;

#[derive(Reflect, Clone, Copy, PartialEq, Eq)]
pub enum TileState {
	Empty,
	Cross,
	Circle,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Tile {
	state: TileState,
}

fn spawn_full_board(mut commands: Commands, board_sprites: Res<BoardSprites>) {
	commands
		.spawn((
			SpriteBundle {
				sprite: Sprite {
					custom_size: Some(Vec2::new(0.75, 0.75)),
					..default()
				},
				texture: board_sprites.board_border.clone(),
				..default()
			},
			Board,
			Name::new("Root Board"),
		))
		.with_children(|builder| {
			for x in -1..=1 {
				for y in -1..=1 {
					let offset = Vec2::new(
						MAX_BOARD_SIZE / 4.0 * (x as f32),
						MAX_BOARD_SIZE / 4.0 * (y as f32),
					);

					spawn_board(builder, 0, offset, board_sprites.board_border.clone());
				}
			}
		});
}

fn spawn_board(builder: &mut ChildBuilder, level: u8, offset: Vec2, board_border: Handle<Image>) {
	assert!(level <= MAX_LEVEL);

	if level == MAX_LEVEL {
		builder
			.spawn((
				SpriteBundle {
					sprite: Sprite {
						custom_size: Some(Vec2::new(0.75, 0.75)),
						..default()
					},
					texture: board_border.clone(),
					transform: Transform {
						translation: offset.extend(0.0),
						scale: Vec3::splat(MAX_BOARD_SIZE / 3.0),
						..default()
					},
					..default()
				},
				Board,
				Name::new("Leaf Board"),
			))
			.with_children(|builder| {
				for x in -1..=1 {
					for y in -1..=1 {
						let offset = Vec2::new(
							MAX_BOARD_SIZE / 4.0 * (x as f32),
							MAX_BOARD_SIZE / 4.0 * (y as f32),
						);

						builder.spawn((
							SpriteBundle {
								sprite: Sprite {
									custom_size: Some(Vec2::new(0.75, 0.75)),
									..default()
								},
								transform: Transform {
									translation: offset.extend(0.0),
									scale: Vec3::splat(MAX_BOARD_SIZE / 3.0),
									..default()
								},
								..default()
							},
							PickableBundle::default(),
							On::<Pointer<Click>>::run(tile_clicked),
							Tile {
								state: TileState::Empty,
							},
							Name::new("Tile"),
						));
					}
				}
			});
		return;
	}

	builder
		.spawn((
			SpriteBundle {
				sprite: Sprite {
					custom_size: Some(Vec2::new(0.75, 0.75)),
					..default()
				},
				texture: board_border.clone(),
				transform: Transform {
					translation: offset.extend(0.0),
					scale: Vec3::splat(MAX_BOARD_SIZE / 3.0),
					..default()
				},
				..default()
			},
			Board,
			Name::new("Parent Board"),
		))
		.with_children(|builder| {
			for x in -1..=1 {
				for y in -1..=1 {
					let offset = Vec2::new(
						MAX_BOARD_SIZE / 4.0 * (x as f32),
						MAX_BOARD_SIZE / 4.0 * (y as f32),
					);

					spawn_board(builder, level + 1, offset, board_border.clone());
				}
			}
		});
}

fn render_tiles(mut tiles: Query<(&mut Handle<Image>, &Tile)>, board_sprites: Res<BoardSprites>) {
	for (mut sprite, tile) in &mut tiles {
		match tile.state {
			TileState::Empty => sprite
				.set(Box::new(board_sprites.empty.clone()))
				.expect("Setting the texture of the leaftile failed."),
			TileState::Cross => sprite
				.set(Box::new(board_sprites.cross.clone()))
				.expect("Setting the texture of the leaftile failed."),
			TileState::Circle => sprite
				.set(Box::new(board_sprites.circle.clone()))
				.expect("Setting the texture of the leaftile failed."),
		}
	}
}

/// Called by an event listener when a leaf tile is clicked, cycle its state.
fn tile_clicked(event: Listener<Pointer<Click>>, mut tiles: Query<&mut Tile>) {
	let mut tile = tiles.get_mut(event.target).unwrap();
	let old_state = tile.state;

	tile.state = match old_state {
		TileState::Empty => TileState::Cross,
		TileState::Cross => TileState::Circle,
		TileState::Circle => TileState::Empty,
	}
}

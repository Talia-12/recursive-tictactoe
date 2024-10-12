use bevy::prelude::*;

pub struct BoardPlugin;

const MAX_LEVEL: u8 = 0;
const MAX_BOARD_SIZE: f32 = 0.92;

impl Plugin for BoardPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<BoardSprites>()
			.add_systems(Startup, spawn_full_board)
			.add_systems(Update, render_board);
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
pub enum Board {
	ParentBoard,
	LeafBoard { tiles: [BoardState; 9] },
}

#[derive(Reflect, Clone, Copy, PartialEq, Eq)]
pub enum BoardState {
	Empty,
	Cross,
	Circle,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LeafTile;

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
			Board::ParentBoard,
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
				Board::LeafBoard {
					tiles: [BoardState::Empty; 9],
				},
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
									custom_size: Some(Vec2::new(1.0, 1.0)),
									..default()
								},
								transform: Transform {
									translation: offset.extend(0.0),
									scale: Vec3::splat(MAX_BOARD_SIZE / 3.0),
									..default()
								},
								..default()
							},
							LeafTile,
							Name::new("Leaf Tile"),
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
			Board::ParentBoard,
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

fn render_board(
	mut leaf_tiles: Query<(Entity, &mut Handle<Image>, &Parent), With<LeafTile>>,
	boards: Query<(&Board, &Children)>,
	board_sprites: Res<BoardSprites>,
) {
	for (entity, mut sprite, parent) in &mut leaf_tiles {
		let (parent_board, children) = boards
			.get(parent.get())
			.unwrap_or_else(|_| panic!("A leaftile without a parent board happened"));

		match parent_board {
			Board::ParentBoard => panic!("A leaftile whose parent is a parentboard was found"),
			Board::LeafBoard { tiles } => {
				let index = children
					.iter()
					.position(|child| entity == *child)
					.unwrap_or_else(|| {
						panic!("None of the children of the leaftile's parent are the leaftile.")
					});

				match tiles[index] {
					BoardState::Empty => sprite
						.set(Box::new(board_sprites.empty.clone()))
						.expect("Setting the texture of the leaftile failed."),
					BoardState::Cross => sprite
						.set(Box::new(board_sprites.cross.clone()))
						.expect("Setting the texture of the leaftile failed."),
					BoardState::Circle => sprite
						.set(Box::new(board_sprites.circle.clone()))
						.expect("Setting the texture of the leaftile failed."),
				}
			}
		}
	}
}

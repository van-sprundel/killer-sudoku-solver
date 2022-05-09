use crate::data::board::Board;
use crate::data::cage::CageColor;
use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Board::sample_puzzle())
            .add_startup_system(setup_board);
    }
}

const TILE_SIZE: f32 = 1. / 9.;

fn setup_board(
    board: Res<Board>,
    windows: Res<Windows>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = windows.primary();
    commands
        .spawn()
        .insert(Name::new("Board"))
        .insert(Transform::from_xyz(
            window.width() / 2.,
            window.height() / 2.,
            0.,
        ))
        .insert(GlobalTransform::default())
        .with_children(|parent| {
            for section_x in 0..3 {
                for section_y in 0..3 {
                    parent
                        .spawn()
                        .insert(GlobalTransform::default())
                        .insert(Transform {
                            translation: Vec3::new(
                                3. * -(section_x as f32) * TILE_SIZE * window.width(),
                                3. * -(section_y as f32) * TILE_SIZE * window.height(),
                                0.,
                            ),
                            scale: Vec3::splat((window.height() - 5.) / window.height()),
                            ..default()
                        })
                        .with_children(|parent| {
                            for x in 0..3 {
                                for y in 0..3 {
                                    let (global_x, global_y) = (
                                        (section_x as usize * 3) + x,
                                        (section_y as usize * 3) + y,
                                    );
                                    let number = board.get_value(global_x, global_y);
                                    let sum = board.get_sum(global_x, global_y);

                                    let color = match board.get_color(global_x, global_y) {
                                        CageColor::Green => Color::rgba(0.8, 0.9, 0.7, 1.),
                                        CageColor::Blue => Color::rgba(0.8, 0.9, 1., 1.),
                                        CageColor::Yellow => Color::rgba(1., 1., 0.7, 1.),
                                        CageColor::Red => Color::rgba(1., 0.8, 0.9, 1.),
                                    };
                                    parent.spawn_bundle(SpriteBundle {
                                        sprite: Sprite {
                                            color,
                                            custom_size: Some(Vec2::splat(
                                                TILE_SIZE * window.height() - 1.,
                                            )),
                                            ..default()
                                        },
                                        transform: Transform::from_xyz(
                                            -(x as f32) * TILE_SIZE * window.width()
                                                - TILE_SIZE * window.width() / 2.,
                                            -(y as f32) * TILE_SIZE * window.height()
                                                - TILE_SIZE * window.height() / 2.,
                                            1.,
                                        ),
                                        ..default()
                                    });
                                    let transform = Transform::from_xyz(
                                        -(x as f32) * TILE_SIZE * window.width()
                                            - TILE_SIZE * window.width() / 2.,
                                        -(y as f32) * TILE_SIZE * window.height()
                                            - TILE_SIZE * window.height() / 2.,
                                        2.,
                                    );
                                    let style = TextStyle {
                                        font: asset_server.load("font.ttf"),
                                        color: Color::BLACK,
                                        ..default()
                                    };
                                    parent.spawn_bundle(Text2dBundle {
                                        text: Text {
                                            alignment: TextAlignment {
                                                horizontal: HorizontalAlign::Right,
                                                vertical: VerticalAlign::Bottom,
                                            },
                                            sections: vec![TextSection {
                                                value: sum.to_string(),
                                                style: TextStyle {
                                                    font_size: 12.,
                                                    ..style.clone()
                                                },
                                            }],
                                        },
                                        transform,
                                        ..default()
                                    });
                                    parent
                                        .spawn_bundle(Text2dBundle {
                                            text: Text {
                                                alignment: TextAlignment {
                                                    horizontal: HorizontalAlign::Center,
                                                    vertical: VerticalAlign::Center,
                                                },
                                                sections: vec![TextSection {
                                                    value: if number != 0 {
                                                        number.to_string()
                                                    } else {
                                                        "".to_string()
                                                    },
                                                    style: TextStyle {
                                                        font_size: 24.,
                                                        ..style.clone()
                                                    },
                                                }],
                                            },
                                            transform,
                                            ..default()
                                        })
                                        .insert(NumberComponent);
                                }
                            }
                        });
                }
            }
        });
}

#[derive(Component)]
struct NumberComponent;

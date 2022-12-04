use crate::{despawn_screen, DaySelectState, GameState, HOVER_BUTTON, PRESSED_BUTTON};
use bevy::prelude::*;
use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;

pub const MAX_DAY: usize = 4;
const BUTTON_BACKGROUND: Color = Color::rgb(0.2235, 0.2196, 0.2);
const LABEL_BACKGROUND: Color = Color::rgb(0.0549, 0.1765, 0.2549);
pub struct DaysPlugin;

impl Plugin for DaysPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Day).with_system(days_setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Day).with_system(despawn_screen::<OnDaysScreen>),
            )
            .add_system_set(SystemSet::on_update(GameState::Day).with_system(exit_system));
    }
}

#[derive(Component)]
struct OnDaysScreen;

fn days_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    day_select_state: Res<State<DaySelectState>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let day = day_select_state.current().0;
    let (part1, part2) = match day {
        1 => {
            let input: String = fs::read_to_string("assets/inputs/day01.txt").unwrap();
            day01::solve(&input)
        }
        2 => {
            let input: String = fs::read_to_string("assets/inputs/day02.txt").unwrap();
            day02::solve(&input)
        }
        3 => {
            let input: String = fs::read_to_string("assets/inputs/day03.txt").unwrap();
            day03::solve(&input)
        }
        4 => {
            let input: String = fs::read_to_string("assets/inputs/day04.txt").unwrap();
            day04::solve(&input)
        }
        _ => unimplemented!(),
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::rgb(0.075, 0.075, 0.075).into(),
                ..default()
            },
            OnDaysScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size {
                            width: Val::Percent(100.0),
                            height: Val::Percent(10.0),
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                size: Size {
                                    height: Val::Percent(75.0),
                                    width: Val::Auto,
                                },
                                aspect_ratio: Some(1.0),
                                margin: UiRect {
                                    right: Val::Percent(2.0),
                                    ..default()
                                },
                                ..default()
                            },
                            background_color: BUTTON_BACKGROUND.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "X",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 50.0,
                                    color: Color::rgb(1.0, 1.0, 1.0).into(),
                                },
                            ));
                        });
                    parent.spawn(TextBundle::from_section(
                        format!("Day {}", day),
                        TextStyle {
                            font: font.clone(),
                            font_size: 50.0,
                            color: Color::rgb(1.0, 1.0, 1.0).into(),
                        },
                    ));
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        size: Size {
                            width: Val::Percent(100.0),
                            height: Val::Percent(90.0),
                        },
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                margin: UiRect::all(Val::Auto),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                size: Size {
                                    width: Val::Percent(95.0),
                                    height: Val::Auto,
                                },
                                ..default()
                            },
                            background_color: LABEL_BACKGROUND.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Part 1",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 50.0,
                                        color: Color::rgb(1.0, 1.0, 1.0).into(),
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(50.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn(
                                TextBundle::from_section(
                                    format!("{}", part1),
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 50.0,
                                        color: Color::rgb(1.0, 1.0, 1.0).into(),
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(50.0)),
                                    ..default()
                                }),
                            );
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                margin: UiRect::all(Val::Auto),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                size: Size {
                                    width: Val::Percent(95.0),
                                    height: Val::Auto,
                                },
                                ..default()
                            },
                            background_color: LABEL_BACKGROUND.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Part 2",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 50.0,
                                        color: Color::rgb(1.0, 1.0, 1.0).into(),
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(50.0)),
                                    ..default()
                                }),
                            );
                            parent.spawn(
                                TextBundle::from_section(
                                    format!("{}", part2),
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 50.0,
                                        color: Color::rgb(1.0, 1.0, 1.0).into(),
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(50.0)),
                                    ..default()
                                }),
                            );
                        });
                });
        });
}

fn exit_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<State<GameState>>,
    mut day_select_state: ResMut<State<DaySelectState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            game_state.set(GameState::Menu).unwrap();
            day_select_state.set(DaySelectState(0)).unwrap();
        }
        *color = match *interaction {
            Interaction::Clicked => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVER_BUTTON.into(),
            Interaction::None => BUTTON_BACKGROUND.into(),
        }
    }
}

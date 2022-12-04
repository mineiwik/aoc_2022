use crate::{despawn_screen, DaySelectState, GameState, HOVER_BUTTON, PRESSED_BUTTON};
use bevy::prelude::*;
use bevy_egui::EguiClipboard;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, JsCast};
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlTextAreaElement;

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
        app.add_state(DayState::Disabled)
            .add_state(InputState("".to_owned()))
            .add_state(Part1State("".to_owned()))
            .add_state(Part2State("".to_owned()))
            .add_system_set(SystemSet::on_enter(GameState::Day).with_system(day_setup))
            .add_system_set(SystemSet::on_enter(DayState::Input).with_system(day_input_setup))
            .add_system_set(
                SystemSet::on_exit(DayState::Input).with_system(despawn_screen::<OnDayInputScreen>),
            )
            .add_system_set(SystemSet::on_enter(DayState::Show).with_system(day_show_setup))
            .add_system_set(
                SystemSet::on_exit(DayState::Show).with_system(despawn_screen::<OnDayShowScreen>),
            )
            .add_system_set(SystemSet::on_update(GameState::Day).with_system(exit_system));
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum DayState {
    Disabled,
    Input,
    Show,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct InputState(String);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Part1State(String);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Part2State(String);

#[derive(Component)]
enum ButtonAction {
    Exit,
    Paste,
    CopyPart1,
    CopyPart2,
}

#[derive(Component)]
struct OnDayInputScreen;

#[derive(Component)]
struct OnDayShowScreen;

fn day_setup(mut day_state: ResMut<State<DayState>>) {
    let _ = day_state.set(DayState::Input);
}

fn day_input_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    day_select_state: Res<State<DaySelectState>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let day = day_select_state.current().0;

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
            OnDayInputScreen,
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
                        .spawn((
                            ButtonBundle {
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
                            },
                            ButtonAction::Exit,
                        ))
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
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    margin: UiRect::all(Val::Auto),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    size: Size {
                                        width: Val::Percent(50.0),
                                        height: Val::Percent(50.0),
                                    },
                                    ..default()
                                },
                                background_color: LABEL_BACKGROUND.into(),
                                ..default()
                            },
                            ButtonAction::Paste,
                        ))
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Paste input",
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

fn day_show_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    day_select_state: Res<State<DaySelectState>>,
    mut part1_state: ResMut<State<Part1State>>,
    mut part2_state: ResMut<State<Part2State>>,
    input_state: Res<State<InputState>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let day = day_select_state.current().0;
    let input = &input_state.current().0;
    let (part1, part2) = match day {
        1 => day01::solve(input),
        2 => day02::solve(input),
        3 => day03::solve(input),
        4 => day04::solve(input),
        _ => unimplemented!(),
    };

    part1_state.set(Part1State(part1.to_string())).unwrap();
    part2_state.set(Part2State(part2.to_string())).unwrap();

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
            OnDayShowScreen,
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
                        .spawn((
                            ButtonBundle {
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
                            },
                            ButtonAction::Exit,
                        ))
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
                        .spawn((
                            ButtonBundle {
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
                            },
                            ButtonAction::CopyPart1,
                        ))
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
                        .spawn((
                            ButtonBundle {
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
                            },
                            ButtonAction::CopyPart2,
                        ))
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
        (&Interaction, &mut BackgroundColor, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut day_state: ResMut<State<DayState>>,
    mut game_state: ResMut<State<GameState>>,
    mut day_select_state: ResMut<State<DaySelectState>>,
    mut input_state: ResMut<State<InputState>>,
    mut part1_state: ResMut<State<Part1State>>,
    mut part2_state: ResMut<State<Part2State>>,
    mut egui_clipboard: ResMut<EguiClipboard>,
) {
    for (interaction, mut color, button_action) in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            match button_action {
                ButtonAction::Exit => {
                    day_state.set(DayState::Disabled).unwrap();
                    game_state.set(GameState::Menu).unwrap();
                    day_select_state.set(DaySelectState(0)).unwrap();
                    input_state.set(InputState("".to_string())).unwrap();
                    part1_state.set(Part1State("".to_string())).unwrap();
                    part2_state.set(Part2State("".to_string())).unwrap();
                }
                ButtonAction::Paste => {
                    #[cfg(target_arch = "wasm32")]
                    let input = {
                        {
                            let window = web_sys::window().expect("no global `window` exists");
                            let document =
                                window.document().expect("should have a document on window");

                            // Manufacture the element we're gonna append
                            let val = document.get_element_by_id("aoc2022-input").expect("msg");
                            let val = val.dyn_into::<HtmlTextAreaElement>().expect("msg");
                            val.value()
                        }
                    };
                    #[cfg(not(target_arch = "wasm32"))]
                    let input = egui_clipboard.get_contents().unwrap();
                    input_state.set(InputState(input)).unwrap();
                    day_state.set(DayState::Show).unwrap();
                }
                ButtonAction::CopyPart1 => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        let window = web_sys::window().expect("no global `window` exists");
                        let document = window.document().expect("should have a document on window");

                        // Manufacture the element we're gonna append
                        let val = document.get_element_by_id("aoc2022-input").expect("msg");
                        let val = val.dyn_into::<HtmlTextAreaElement>().expect("msg");
                        val.set_value(&part1_state.current().0);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    egui_clipboard.set_contents(&part1_state.current().0);
                }
                ButtonAction::CopyPart2 => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        let window = web_sys::window().expect("no global `window` exists");
                        let document = window.document().expect("should have a document on window");

                        // Manufacture the element we're gonna append
                        let val = document.get_element_by_id("aoc2022-input").expect("msg");
                        let val = val.dyn_into::<HtmlTextAreaElement>().expect("msg");
                        val.set_value(&part2_state.current().0);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    egui_clipboard.set_contents(&part2_state.current().0);
                }
            }
        }
        *color = match *interaction {
            Interaction::Clicked => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVER_BUTTON.into(),
            Interaction::None => BUTTON_BACKGROUND.into(),
        }
    }
}

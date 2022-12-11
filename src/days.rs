use crate::{despawn_screen, DaySelectState, GameState, HOVER_BUTTON, PRESSED_BUTTON};
use bevy::prelude::*;
use bevy_egui::EguiClipboard;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlTextAreaElement;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

pub const MAX_DAY: usize = 11;
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
            .add_system_set(
                SystemSet::on_exit(GameState::Day).with_system(despawn_screen::<OnDayScreen>),
            )
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
struct OnDayScreen;

#[derive(Component)]
struct OnDayInputScreen;

#[derive(Component)]
struct OnDayShowScreen;

fn day_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    day_select_state: Res<State<DaySelectState>>,
    mut day_state: ResMut<State<DayState>>,
) {
    day_state.set(DayState::Input).unwrap();
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
            OnDayScreen,
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
                                    color: Color::rgb(1.0, 1.0, 1.0),
                                },
                            ));
                        });
                    parent.spawn(TextBundle::from_section(
                        format!("Day {}", day),
                        TextStyle {
                            font: font.clone(),
                            font_size: 50.0,
                            color: Color::rgb(1.0, 1.0, 1.0),
                        },
                    ));
                });
        });
}

fn day_input_setup(
    mut commands: Commands,
    parent: Query<Entity, With<OnDayScreen>>,
    asset_server: Res<AssetServer>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let parent = parent.iter().next().unwrap();

    commands.entity(parent).with_children(|parent| {
        parent
            .spawn((
                NodeBundle {
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
                },
                OnDayInputScreen,
            ))
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
                                    color: Color::rgb(1.0, 1.0, 1.0),
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
    parent: Query<Entity, With<OnDayScreen>>,
    asset_server: Res<AssetServer>,
    day_select_state: Res<State<DaySelectState>>,
    mut part1_state: ResMut<State<Part1State>>,
    mut part2_state: ResMut<State<Part2State>>,
    input_state: Res<State<InputState>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    let day = day_select_state.current().0;
    let input = &input_state.current().0;
    let parent = parent.iter().next().unwrap();
    let (part1, part2) = match day {
        1 => day01::solve(input),
        2 => day02::solve(input),
        3 => day03::solve(input),
        4 => day04::solve(input),
        5 => day05::solve(input),
        6 => day06::solve(input),
        7 => day07::solve(input),
        8 => day08::solve(input),
        9 => day09::solve(input),
        10 => day10::solve(input),
        11 => day11::solve(input),
        _ => unimplemented!(),
    };

    part1_state.set(Part1State(part1.clone())).ok();
    part2_state.set(Part2State(part2.clone())).ok();
    commands.entity(parent).with_children(|parent| {
        parent
            .spawn((
                NodeBundle {
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
                },
                OnDayShowScreen,
            ))
            .with_children(|parent| {
                build_part_button(parent, &font, 1, &part1, ButtonAction::CopyPart1);
                build_part_button(parent, &font, 2, &part2, ButtonAction::CopyPart2);
            });
    });
}

fn build_part_button(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    part: usize,
    res: &str,
    action: ButtonAction,
) {
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
            action,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    format!("Part {}", part),
                    TextStyle {
                        font: font.clone(),
                        font_size: 50.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
            parent.spawn(
                TextBundle::from_section(
                    res.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 50.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
        });
}

#[allow(clippy::type_complexity)]
fn exit_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut day_state: ResMut<State<DayState>>,
    mut game_state: ResMut<State<GameState>>,
    mut input_state: ResMut<State<InputState>>,
    mut egui_clipboard: ResMut<EguiClipboard>,
    part1_state: Res<State<Part1State>>,
    part2_state: Res<State<Part2State>>,
) {
    for (interaction, mut color, button_action) in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            match button_action {
                ButtonAction::Exit => {
                    day_state.set(DayState::Disabled).unwrap();
                    game_state.set(GameState::Menu).unwrap();
                }
                ButtonAction::Paste => {
                    #[cfg(target_arch = "wasm32")]
                    let input = read_clipboard();
                    #[cfg(not(target_arch = "wasm32"))]
                    let input = read_clipboard(&egui_clipboard);

                    input_state.set(InputState(input)).ok();
                    day_state.set(DayState::Show).unwrap();
                }
                ButtonAction::CopyPart1 => {
                    #[cfg(target_arch = "wasm32")]
                    set_clipboard(&part1_state.current().0);
                    #[cfg(not(target_arch = "wasm32"))]
                    set_clipboard(&part1_state.current().0, &mut egui_clipboard);
                }
                ButtonAction::CopyPart2 => {
                    #[cfg(target_arch = "wasm32")]
                    set_clipboard(&part2_state.current().0);
                    #[cfg(not(target_arch = "wasm32"))]
                    set_clipboard(&part2_state.current().0, &mut egui_clipboard);
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

#[cfg(target_arch = "wasm32")]
fn read_clipboard() -> String {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let val = document.get_element_by_id("aoc2022-input").unwrap();
    let val = val.dyn_into::<HtmlTextAreaElement>().unwrap();
    val.value()
}

#[cfg(not(target_arch = "wasm32"))]
fn read_clipboard(clipboard: &ResMut<EguiClipboard>) -> String {
    clipboard.get_contents().unwrap()
}

#[cfg(target_arch = "wasm32")]
fn set_clipboard(value: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let val = document.get_element_by_id("aoc2022-input").unwrap();
    let val = val.dyn_into::<HtmlTextAreaElement>().unwrap();
    val.set_value(value);
}

#[cfg(not(target_arch = "wasm32"))]
fn set_clipboard(value: &str, clipboard: &mut ResMut<EguiClipboard>) {
    clipboard.set_contents(value);
}

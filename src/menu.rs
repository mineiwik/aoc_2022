use crate::{
    days::MAX_DAY, despawn_screen, DaySelectState, GameState, HOVER_BUTTON, IDLE_BUTTON,
    PRESSED_BUTTON,
};
use bevy::{prelude::*, ui::FocusPolicy};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu_setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Menu).with_system(despawn_screen::<OnMenuScreen>),
            )
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(menu_action));
    }
}

#[derive(Component)]
struct OnMenuScreen;

#[derive(Component)]
struct DayAction(usize);

fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.075, 0.075, 0.075).into(),
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "AoC 2022",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        }),
                    );
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(90.0), Val::Percent(90.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for i in 0..25 {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(15.0), Val::Percent(18.0)),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    background_color: IDLE_BUTTON.into(),
                                    ..default()
                                },
                                DayAction(i + 1),
                            ))
                            .with_children(|parent| {
                                let image_name = if i < MAX_DAY {
                                    "img/star.png"
                                } else {
                                    "img/snowflake.png"
                                };
                                let snowflake = asset_server.load(image_name);
                                parent.spawn(ImageBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(50.0), Val::Auto),
                                        ..default()
                                    },
                                    image: UiImage::from(snowflake),
                                    focus_policy: FocusPolicy::Pass,
                                    ..default()
                                });
                                parent.spawn(
                                    TextBundle::from_section(
                                        format!("{}", i + 1),
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_style(Style {
                                        position_type: PositionType::Absolute,
                                        position: UiRect {
                                            left: Val::Px(0.0),
                                            right: Val::Auto,
                                            top: Val::Px(0.0),
                                            bottom: Val::Auto,
                                        },
                                        margin: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    }),
                                );
                            });
                    }
                });
        });
}

#[allow(clippy::type_complexity)]
fn menu_action(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &DayAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<State<GameState>>,
    mut day_select_state: ResMut<State<DaySelectState>>,
) {
    for (interaction, mut color, day_action) in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            if let 1..=MAX_DAY = day_action.0 {
                game_state.set(GameState::Day).unwrap();
                day_select_state.set(DaySelectState(day_action.0)).ok();
            }
        }
        *color = match *interaction {
            Interaction::Clicked => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVER_BUTTON.into(),
            Interaction::None => IDLE_BUTTON.into(),
        }
    }
}

use bevy::{prelude::*, winit::WinitSettings};

mod days;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .add_plugin(days::DaysPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
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
                            .spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(15.0), Val::Px(100.0)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.1882, 0.2706, 0.4039).into(),
                                ..default()
                            })
                            .with_children(|parent| {
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
                                parent.spawn(ImageBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(75.0), Val::Px(75.0)),
                                        max_size: Size::new(Val::Percent(75.0), Val::Auto),
                                        ..default()
                                    },
                                    image: asset_server.load("img/snowflake.png").into(),
                                    ..default()
                                });
                            });
                    }
                });
        });
}

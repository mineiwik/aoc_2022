use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod days;
mod menu;

const IDLE_BUTTON: Color = Color::rgb(0.0549, 0.1765, 0.2549);
const PRESSED_BUTTON: Color = Color::rgb(0.5961, 0.1451, 0.1176);
const HOVER_BUTTON: Color = Color::rgb(0.0941, 0.2588, 0.3529);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Menu,
    Day,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct DaySelectState(usize);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_state(GameState::Menu)
        .add_state(DaySelectState(0))
        .add_plugin(menu::MenuPlugin)
        .add_plugin(days::DaysPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

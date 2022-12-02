use bevy::prelude::*;

pub mod day01;
pub mod day02;
pub struct DaysPlugin;

impl Plugin for DaysPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(day01::DayPlugin);
        app.add_plugin(day02::DayPlugin);
    }
}

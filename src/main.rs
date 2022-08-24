use actions::{movement_plugin::MovementPlugin, QUEUE_ACTIONS, TICK_ACTIONS};
use bevy::{app::AppExit, prelude::*};

use crossterm_plugin::CrosstermPlugin;
use levels_plugin::LevelsPlugin;
use player_plugin::{PlayerPlugin, CONTROL_PLAYER};

mod actions;
mod crossterm_plugin;
mod levels_plugin;
mod objects;
mod player_plugin;
mod point;

use crossterm_plugin::CrosstermKeyCode;

fn exit_on_esc(input: Res<Input<CrosstermKeyCode>>, mut app_exit_writer: EventWriter<AppExit>) {
    if input.just_pressed(CrosstermKeyCode::Esc) {
        app_exit_writer.send(AppExit);
    }
}

fn main() {
    App::new()
        .add_stage_after(
            CoreStage::PreUpdate,
            CONTROL_PLAYER,
            SystemStage::single_threaded(),
        )
        .add_stage_after(CONTROL_PLAYER, TICK_ACTIONS, SystemStage::parallel())
        .add_stage_after(TICK_ACTIONS, QUEUE_ACTIONS, SystemStage::parallel())
        .add_plugin(LevelsPlugin)
        .add_plugin(CrosstermPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MovementPlugin)
        .add_system(exit_on_esc)
        .run();
}

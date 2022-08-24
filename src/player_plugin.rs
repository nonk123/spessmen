use bevy::prelude::*;

use crate::{
    actions::{
        movement_plugin::{Direction, Move},
        ActionQueuedEvent, Ticks,
    },
    crossterm_plugin::{CrosstermKeyCode, CrosstermRunner},
    levels_plugin::{Level, Position},
    objects::{GameObjectBundle, LAYER_ENTITY},
    point::Point,
};

pub const CONTROL_PLAYER: &str = "control_player";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraPosition(Point::new(0, 0)))
            .add_startup_system(spawn_player)
            .add_system_to_stage(CONTROL_PLAYER, control_player);
    }
}

#[derive(Deref, DerefMut)]
pub struct CameraPosition(Point);

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, level: Res<Level>) {
    commands
        .spawn_bundle(GameObjectBundle::new(
            Point::from_usize(level.width() / 2, level.height() / 2),
            '@',
            LAYER_ENTITY,
        ))
        .insert(Player);
}

fn control_player(
    mut runner: ResMut<CrosstermRunner>,
    mut camera_position: ResMut<CameraPosition>,
    input: Res<Input<CrosstermKeyCode>>,
    players: Query<(Entity, &Position), With<Player>>,
    mut movement: EventWriter<ActionQueuedEvent<Move>>,
) {
    let player = players.iter().next();

    if player.is_none() {
        return;
    }

    let (player, position) = player.unwrap();
    camera_position.0 = position.0;

    if runner.is_world_tick() {
        return;
    }

    let direction = {
        if input.pressed(CrosstermKeyCode::Char('w')) {
            Some(Direction::North)
        } else if input.pressed(CrosstermKeyCode::Char('a')) {
            Some(Direction::West)
        } else if input.pressed(CrosstermKeyCode::Char('s')) {
            Some(Direction::South)
        } else if input.pressed(CrosstermKeyCode::Char('d')) {
            Some(Direction::East)
        } else {
            None
        }
    };

    if let Some(direction) = direction {
        movement.send(ActionQueuedEvent {
            performer: player,
            duration: Ticks::seconds(0.4),
            action: Move { direction },
        });
    } else if input.pressed(CrosstermKeyCode::Char('.')) {
        runner.advance_world = Some(Ticks(1));
    } else {
        return;
    }
}

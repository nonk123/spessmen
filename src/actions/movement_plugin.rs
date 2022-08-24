use bevy::prelude::*;

use crate::{
    levels_plugin::{Level, Position},
    point::{Coord, Point},
};

use super::{ActionPerformedEvent, ActionQueuedEvent, ActionTypePlugin, QUEUE_ACTIONS};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ActionTypePlugin::<Move>::new())
            .add_system_to_stage(QUEUE_ACTIONS, movement_queued)
            .add_system(perform_movement);
    }
}

#[derive(Clone)]
pub struct Move {
    pub direction: Direction,
}

#[derive(Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn as_point(&self) -> Point {
        match self {
            Self::North => Point::new(0, -1),
            Self::East => Point::new(1, 0),
            Self::South => Point::new(0, 1),
            Self::West => Point::new(-1, 0),
        }
    }
}

fn movement_queued(
    level: Res<Level>,
    mut event_reader: EventReader<ActionQueuedEvent<Move>>,
    performers: Query<&Position>,
    mut commands: Commands,
) {
    for event in event_reader.iter() {
        if let Ok(position) = performers.get(event.performer) {
            let destination = position.0 + event.action.direction.as_point();

            if is_walkable(&level, destination) {
                event.pass_down(&mut commands);
            }
        }
    }
}

fn perform_movement(
    level: Res<Level>,
    mut event_reader: EventReader<ActionPerformedEvent<Move>>,
    mut performers: Query<&mut Position>,
) {
    for event in event_reader.iter() {
        if let Ok(mut position) = performers.get_mut(event.performer) {
            let destination = position.0 + event.action.direction.as_point();

            if is_walkable(&level, destination) {
                position.0 = destination;
            }
        }
    }
}

fn is_walkable(level: &Res<Level>, destination: Point) -> bool {
    if destination.x < 0
        || destination.y < 0
        || destination.x >= level.width() as Coord
        || destination.y >= level.height() as Coord
    {
        return false;
    }

    let tile = level.tile(destination.x as usize, destination.y as usize);
    tile.floor.is_some() && tile.wall.is_none()
}

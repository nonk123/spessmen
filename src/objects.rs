use bevy::prelude::*;

use crate::{
    actions::Actor,
    crossterm_plugin::{LayerId, Renderable},
    levels_plugin::Position,
    point::Point,
};

pub const LAYER_ENTITY: LayerId = 0;

#[derive(Component)]
pub struct GameObject;

#[derive(Bundle)]
pub struct GameObjectBundle {
    position: Position,
    game_object: GameObject,
    renderable: Renderable,
    actor: Actor,
}

impl GameObjectBundle {
    pub fn new(position: Point, character: char, layer: LayerId) -> Self {
        Self {
            position: Position(position),
            game_object: GameObject,
            renderable: Renderable {
                character,
                visible: true,
                layer,
            },
            actor: Actor::new(),
        }
    }
}

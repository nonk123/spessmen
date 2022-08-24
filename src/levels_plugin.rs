use bevy::prelude::*;

use crate::point::Point;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level::new(32, 32))
            .add_startup_system(init_level);
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Position(pub Point);

pub struct Level {
    tiles: Vec<Vec<Tile>>,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![
                vec![
                    Tile {
                        wall: None,
                        floor: None,
                    };
                    width
                ];
                height
            ],
        }
    }

    pub fn width(&self) -> usize {
        if self.tiles.is_empty() {
            0
        } else {
            self.tiles[0].len()
        }
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn tile(&self, y: usize, x: usize) -> &Tile {
        &self.tiles[y][x]
    }

    pub fn tile_mut(&mut self, y: usize, x: usize) -> &mut Tile {
        &mut self.tiles[y][x]
    }
}

#[derive(Clone)]
pub struct Tile {
    pub wall: Option<Wall>,
    pub floor: Option<Floor>,
}

#[derive(Clone)]
pub struct Wall;

#[derive(Clone)]
pub struct Floor;

fn init_level(mut level: ResMut<Level>) {
    let width = level.width();
    let height = level.height();

    for y in 0..height {
        for x in 0..width {
            let tile = level.tile_mut(x, y);

            tile.floor = Some(Floor);

            if y == 0 || x == 0 || y == height - 1 || x == width - 1 {
                tile.wall = Some(Wall);
            }
        }
    }
}

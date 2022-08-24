use std::{
    io::{stdout, Write},
    thread::sleep,
    time::{Duration, Instant},
};

use bevy::{app::AppExit, ecs::event::ManualEventReader, prelude::*};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size},
    ExecutableCommand, QueueableCommand, Result,
};

pub use crossterm::event::KeyCode as CrosstermKeyCode;

use crate::{
    actions::Ticks,
    levels_plugin::{Level, Position},
    player_plugin::CameraPosition,
    point::{Coord, Point},
};

pub struct CrosstermPlugin;

impl Plugin for CrosstermPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Stdout(stdout()))
            .insert_resource(CrosstermRunner {
                advance_world: None,
            })
            .init_resource::<Input<CrosstermKeyCode>>()
            .set_runner(crossterm_runner)
            .add_system_to_stage(CoreStage::PreUpdate, crossterm_read_input)
            .add_system_to_stage(CoreStage::PostUpdate, crossterm_render);
    }
}

#[derive(Deref, DerefMut)]
pub struct Stdout(std::io::Stdout);

pub const FRAME_RATE: i32 = 60;
pub const FRAME_TIME: f64 = 1.0 / FRAME_RATE as f64;

pub struct CrosstermRunner {
    pub advance_world: Option<Ticks>,
}

impl CrosstermRunner {
    pub fn is_world_tick(&self) -> bool {
        self.advance_world.is_some()
    }
}

fn crossterm_runner(mut app: App) {
    let mut body = move || -> Result<()> {
        enable_raw_mode()?;

        {
            let mut stdout = app.world.get_resource_mut::<Stdout>().unwrap();
            stdout.execute(EnableMouseCapture)?.execute(Hide)?;
        }

        let mut app_exit_event_reader = ManualEventReader::<AppExit>::default();

        let mut frame_start;

        loop {
            frame_start = Instant::now();

            app.update();

            loop {
                {
                    let runner = app.world.resource::<CrosstermRunner>();

                    if !runner.is_world_tick() {
                        break;
                    }
                }

                app.update();

                {
                    let mut runner = app.world.resource_mut::<CrosstermRunner>();

                    if let Some(advance_world) = &mut runner.advance_world {
                        advance_world.0 -= 1;

                        if advance_world.0 == 0 {
                            runner.advance_world = None;
                            app.update();
                        }
                    }
                }
            }

            if let Some(app_exit_events) = app.world.get_resource_mut::<Events<AppExit>>() {
                if let Some(_) = app_exit_event_reader.iter(&app_exit_events).last() {
                    break;
                }
            }

            let delta = Instant::now().duration_since(frame_start).as_secs_f64();

            if delta < FRAME_TIME {
                sleep(Duration::from_secs_f64(FRAME_TIME - delta));
            }
        }

        disable_raw_mode()?;

        {
            let mut stdout = app.world.get_resource_mut::<Stdout>().unwrap();
            stdout.execute(DisableMouseCapture)?.execute(Show)?;
        }

        Ok(())
    };

    body().expect("crossterm_runner");
}

fn crossterm_read_input(runner: Res<CrosstermRunner>, mut input: ResMut<Input<CrosstermKeyCode>>) {
    if runner.is_world_tick() {
        return;
    }

    input.reset_all();

    let mut body = move || -> Result<()> {
        while poll(Duration::ZERO)? {
            let event = read()?;

            match event {
                Event::Key(key_event) => {
                    match key_event.kind {
                        KeyEventKind::Press => input.press(key_event.code),
                        // Unused.
                        KeyEventKind::Release => input.release(key_event.code),
                        _ => (),
                    };
                }
                _ => (),
            }
        }

        Ok(())
    };

    body().expect("crossterm_read_input")
}

#[derive(Component)]
pub struct Renderable {
    pub character: char,
    pub visible: bool,
    pub layer: LayerId,
}

pub type LayerId = i32;

fn crossterm_render(
    runner: Res<CrosstermRunner>,
    mut stdout: ResMut<Stdout>,
    camera_position: Res<CameraPosition>,
    level: Res<Level>,
    entities: Query<(&Position, &Renderable)>,
) {
    if runner.is_world_tick() {
        return;
    }

    let mut body = move || -> Result<()> {
        let (cols, rows) = size()?;

        let start = Point::new(
            camera_position.x - cols as Coord / 2,
            camera_position.y - rows as Coord / 2,
        );

        let mut end = Point::new(
            camera_position.x + cols as Coord / 2,
            camera_position.y + rows as Coord / 2,
        );

        if cols % 2 == 1 {
            end.x += 1;
        }

        if rows % 2 == 1 {
            end.y += 1;
        }

        let mut grid = vec![vec![' '; cols as usize]; rows as usize];

        let width = level.width();
        let height = level.height();

        let mut row = 0;

        for y in start.y..end.y {
            let mut col = 0;

            for x in start.x..end.x {
                if x < 0 || y < 0 || x >= width as Coord || y >= height as Coord {
                    col += 1;
                    continue;
                }

                let x = x as usize;
                let y = y as usize;

                let tile = level.tile(x, y);

                grid[row][col] = if tile.wall.is_some() {
                    '#'
                } else if tile.floor.is_some() {
                    '.'
                } else {
                    ' '
                };

                col += 1;
            }

            row += 1;
        }

        let mut entities = entities
            .iter()
            .filter(|(position, _)| {
                position.x >= start.x
                    && position.x < end.x
                    && position.y >= start.y
                    && position.y < end.y
            })
            .collect::<Vec<_>>();

        entities.sort_by_key(|(_, renderable)| renderable.layer);

        for (position, renderable) in entities {
            let position = position.0 - start;
            grid[position.y as usize][position.x as usize] = renderable.character;
        }

        for row in 0..rows {
            stdout.queue(MoveTo(0, row))?;

            for col in 0..cols {
                stdout.queue(Print(grid[row as usize][col as usize]))?;
            }
        }

        stdout.flush()?;

        Ok(())
    };

    body().expect("crossterm_render");
}

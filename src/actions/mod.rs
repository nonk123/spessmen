pub mod movement_plugin;

use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{crossterm_plugin::CrosstermRunner, player_plugin::Player};

pub const QUEUE_ACTIONS: &str = "queue_actions";
pub const TICK_ACTIONS: &str = "tick_actions";

pub struct ActionTypePlugin<T>(PhantomData<T>);

impl<T: Send + Sync + Clone + 'static> Plugin for ActionTypePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionQueuedEvent<T>>()
            .add_event::<ActionPerformedEvent<T>>()
            .add_system_to_stage(TICK_ACTIONS, tick_action::<T>);
    }
}

impl<T> ActionTypePlugin<T> {
    pub fn new() -> Self {
        Self(PhantomData::default())
    }
}

#[derive(Component)]
pub struct Actor {
    action_progress: Ticks,
}

impl Actor {
    pub fn new() -> Self {
        Self {
            action_progress: Ticks::ZERO,
        }
    }
}

#[derive(Component)]
pub struct Performer<T> {
    duration: Ticks,
    action_data: T,
}

pub const TICKS_PER_SECOND: u32 = 12;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ticks(pub u32);

impl Ticks {
    pub const ZERO: Ticks = Ticks(0);

    pub fn seconds(secs: f64) -> Self {
        Self((secs * TICKS_PER_SECOND as f64) as u32)
    }
}

pub struct ActionQueuedEvent<T> {
    pub performer: Entity,
    pub duration: Ticks,
    pub action: T,
}

impl<T: Send + Sync + Clone + 'static> ActionQueuedEvent<T> {
    pub fn pass_down(&self, commands: &mut Commands) {
        let performer = self.performer;
        let duration = self.duration;
        let action_data = self.action.clone();

        commands.add(move |world: &mut World| {
            let mut performer = world.entity_mut(performer);

            if !performer.contains::<Performer<T>>() {
                performer.insert(Performer::<T> {
                    duration,
                    action_data,
                });

                if performer.contains::<Player>() {
                    world.resource_mut::<CrosstermRunner>().advance_world = Some(duration);
                }
            }
        });
    }
}

pub struct ActionPerformedEvent<T> {
    pub performer: Entity,
    pub action: T,
}

pub fn tick_action<T: Send + Sync + Clone + 'static>(
    runner: Res<CrosstermRunner>,
    mut actors: Query<(Entity, &mut Actor, &Performer<T>)>,
    mut event_writer: EventWriter<ActionPerformedEvent<T>>,
    mut commands: Commands,
) {
    if !runner.is_world_tick() {
        return;
    }

    for (entity, mut actor, performer) in actors.iter_mut() {
        actor.action_progress.0 += 1;

        if actor.action_progress == performer.duration {
            actor.action_progress = Ticks::ZERO;

            commands.entity(entity).remove::<Performer<T>>();

            event_writer.send(ActionPerformedEvent::<T> {
                performer: entity,
                action: performer.action_data.clone(),
            });
        }
    }
}

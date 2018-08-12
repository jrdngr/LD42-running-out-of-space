use super::System;
use game::GameState;
use components::movement::{Position, Velocity};
use components::colliders::SquareCollider;
use utils::clamp;
use constants;

use recs::EntityId;

pub struct MovementSystem;

impl System for MovementSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Position, Velocity), &mut ids);

        for id in ids {
            let pos = state.ecs().get::<Position>(id).unwrap();
            let vel = state.ecs().get::<Velocity>(id).unwrap();
            let new_pos = Position{
                x: clamp(pos.x + vel.x, 0.0, constants::SCREEN_WIDTH as f64),
                y: clamp(pos.y + vel.y, 0.0, constants::SCREEN_HEIGHT as f64),
            };
            let _ = state.ecs().set(id, new_pos.clone());

            if let Ok(mut coll) = state.ecs().borrow_mut::<SquareCollider>(id) {
                coll.set_pos(&new_pos);
            }
        }
    }
}
use recs::EntityId;

use pixi::graphics::Graphics;
use systems::System;
use game::GameState;
use components::colliders::SquareCollider;
use components::movement::Position;
use components::graphics::Renderer;
use components::tags::{Projectile, IceBlock, Enemy};
use constants;

pub struct ProjectileSystem;

impl ProjectileSystem {
    fn splat(&self, state: &mut GameState, projectile: EntityId) {
        let renderer = state.ecs().get::<Renderer>(projectile).unwrap();
        state.app().remove_child(&renderer.graphics);

        let position = state.ecs().get::<Position>(projectile).unwrap();

        let circle = Graphics::new();
        circle.begin_fill(constants::ICE_BLOCK_COLOR);
        circle.draw_ellipse(position.x as f64, position.y as f64, constants::ICE_BLOCK_SIZE, constants::ICE_BLOCK_SIZE);
        state.app().add_child(&circle);
        
        let ice = state.ecs().create_entity();
        let _ = state.ecs().set(ice, IceBlock);
        let _ = state.ecs().set(ice, SquareCollider{position, width: constants::ICE_BLOCK_SIZE, height: constants::ICE_BLOCK_SIZE});
        

        let _ = state.ecs().destroy_entity(projectile);
    }
}

impl System for ProjectileSystem {
    fn run(&mut self, state: &mut GameState, _delta: f64) {
        let mut projectile_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Projectile), &mut projectile_ids);

        let mut blocker_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(IceBlock), &mut blocker_ids);
        let mut enemy_ids: Vec<EntityId> = Vec::new();
        state.ecs().collect_with(&component_filter!(Enemy), &mut enemy_ids);

        blocker_ids.append(&mut enemy_ids);

        for projectile in projectile_ids {
            let position = state.ecs().get::<Position>(projectile).unwrap();
            if position.x <= 0.0 || position.x >= constants::SCREEN_WIDTH as f64 || position.y <= 0.0 || position.y >= constants::SCREEN_HEIGHT as f64 {
                self.splat(state, projectile);
                continue;
            }

            let projectile_coll = state.ecs().get::<SquareCollider>(projectile).unwrap();
            for blocker in &blocker_ids {
                if let Ok(bc) = state.ecs().get::<SquareCollider>(*blocker) {
                    if bc.is_colliding(&projectile_coll) {
                        self.splat(state, projectile);
                        if let Ok(exists) = state.ecs().has::<Enemy>(*blocker) {
                            if exists {
                                let renderer = state.ecs().get::<Renderer>(*blocker).unwrap();
                                state.app().remove_child(&renderer.graphics);
                                let _ = state.ecs().destroy_entity(*blocker);
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
}

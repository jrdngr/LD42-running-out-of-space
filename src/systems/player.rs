use constants;
use systems::System;
use game::GameState;
use components::{Position, Renderer, Velocity, Player, KeyboardControls};
use pixi::graphics::Graphics;

pub struct PlayerSystem;

impl System for PlayerSystem {
    fn init(&mut self, state: &mut GameState) {
        let player_circle = Graphics::new();
        player_circle.begin_fill(constants::PLAYER_COLOR);
        player_circle.draw_ellipse(0, 0, constants::PLAYER_SIZE, constants::PLAYER_SIZE);
        state.app().add_child(&player_circle);

        let player = state.ecs().create_entity();
        let _ = state.ecs().set(player, Player);
        let _ = state.ecs().set(player, Position{x: constants::PLAYER_START_X, y: constants::PLAYER_START_Y});
        let _ = state.ecs().set(player, Velocity{x: 0.0, y: 0.0});
        let _ = state.ecs().set(player, Renderer{graphics: player_circle});
        let _ = state.ecs().set(player, KeyboardControls);
    }
    
    fn run(&mut self, _game: &mut GameState, _delta: f64) {
    }
}
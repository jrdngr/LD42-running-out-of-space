use stdweb::traits::*;
use stdweb::web::document;

use constants;
use recs::Ecs;
use components::{Position, Velocity, Renderer, KeyboardControls};
use systems::System;
use systems::movement::MovementSystem;
use systems::rendering::RenderingSystem;
use systems::controls::ControlSystem;

use pixi::application::Application;
use pixi::graphics::Graphics;

pub struct Game {
    app: Application,
    ecs: Ecs,
    systems: Vec<Box<System>>,
}

impl Game {
    pub fn new() -> Self {
        let body = document().body().unwrap();
        let div = document().create_element("div").unwrap();
        
        body.append_child(&div);

        let app = Application::new(constants::SCREEN_WIDTH, constants::SCREEN_HEIGHT, constants::BACKGROUND_COLOR);
        body.append_child(&app.view());

        let player_circle = Graphics::new();
        player_circle.begin_fill(constants::PLAYER_COLOR);
        player_circle.draw_ellipse(0, 0, constants::PLAYER_SIZE, constants::PLAYER_SIZE);
        app.add_child(&player_circle);

        let mut ecs = Ecs::new();
        let mut systems: Vec<Box<System>> = Vec::new();
        systems.push(Box::new(MovementSystem));
        systems.push(Box::new(RenderingSystem));
        systems.push(Box::new(ControlSystem::new()));

        let player = ecs.create_entity();
        let _ = ecs.set(player, Position{x: constants::PLAYER_START_X, y: constants::PLAYER_START_Y});
        let _ = ecs.set(player, Velocity{x: 0.0, y: 0.0});
        let _ = ecs.set(player, Renderer{graphics: player_circle});
        let _ = ecs.set(player, KeyboardControls{});

        Self {
            app,
            ecs,
            systems,
        }
    }

    pub fn update(&mut self, delta: f64) {
        for system in &mut self.systems {
            system.run(&mut self.ecs, delta);
        }
    }
}

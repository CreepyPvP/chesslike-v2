use bevy::prelude::{Plugin, Resource, Entity};

pub struct GameStatePlugin;


#[derive(PartialEq)]
pub enum GameStates {
    Placing(usize),
    Turn {
        player: usize,
        unit: Entity,
        did_move: bool,
    },
}

#[derive(PartialEq)]
pub enum Participant {
    Bot,
    Me,
}

#[derive(Resource)]
pub struct GameState {
    pub state: GameStates,
    pub participants: Vec<Participant>,
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(GameState {
            participants: vec!(Participant::Me, Participant::Bot),
            state: GameStates::Placing(0),
        });
    }
}

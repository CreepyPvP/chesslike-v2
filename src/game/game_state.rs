use bevy::prelude::{Entity, EventReader, EventWriter, IntoSystemConfig, Plugin, ResMut, Resource};

use super::{GameEvent, GameSystemSets};

pub struct GameStatePlugin;

pub enum GameStateEvent {
    SpawnedUnit(Entity),
}

#[derive(PartialEq)]
pub enum GameStates {
    // player id, round
    Placing(usize, u32),
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

    turn_order: Vec<Option<Entity>>,

    units_per_participant: u32,
}

impl GameState {
    fn place(&mut self, entity: Entity, event_writer: &mut EventWriter<GameEvent>) {
        let (mut player_id, mut turn) = match self.state {
            GameStates::Placing(player_id, turn) => (player_id, turn),
            GameStates::Turn {
                player: _,
                unit: _,
                did_move: _,
            } => return,
        };

        player_id += 1;
        if player_id >= self.participants.len() {
            turn += 1;
            player_id = 0;
            if turn >= self.units_per_participant {
                println!("end turn phase");
                // self.state = GameStates::Turn { player: 0, unit: 0 as usize, did_move: false };
                return;
            }
        }

        self.state = GameStates::Placing(player_id, turn);
        if let Participant::Bot = self.participants[player_id] {
            // do ai action here
            event_writer.send(GameEvent::PlaceAiUnit);
        }
    }
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<GameStateEvent>();

        app.insert_resource(GameState {
            participants: vec![Participant::Me, Participant::Bot],
            state: GameStates::Placing(0, 0),
            units_per_participant: 3,
            turn_order: vec![],
        });

        app.add_systems((update_game_state.in_set(GameSystemSets::Logic),));
    }
}

fn update_game_state(
    mut game_state: ResMut<GameState>,
    mut game_events: EventReader<GameStateEvent>,
    mut event_writer: EventWriter<GameEvent>,
) {
    for event in game_events.iter() {
        match event {
            GameStateEvent::SpawnedUnit(entity) => {
                game_state.place(entity.clone(), &mut event_writer)
            }
            _ => (),
        }
    }
}

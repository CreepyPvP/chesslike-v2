use std::collections::HashMap;

use bevy::prelude::{Entity, EventReader, EventWriter, IntoSystemConfig, Plugin, ResMut, Resource};

use super::{GameEvent, GameSystemSets};

pub struct GameStatePlugin;

pub enum GameStateEvent {
    SpawnedUnit(Entity),
    MovedUnit(Entity),
}

#[derive(PartialEq)]
pub enum GameStates {
    // player id, round
    Placing(usize, u32),
    // turn, did move
    Turn(usize, bool),
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
    pub turn_order: Vec<Option<(usize, Entity)>>,
    units: HashMap<usize, Vec<Entity>>,
    units_per_participant: u32,
}

impl GameState {
    fn place(&mut self, entity: Entity, event_writer: &mut EventWriter<GameEvent>) {
        let GameStates::Placing(mut player_id, mut turn) = self.state else {
            return;
        };

        if let Some(units) = self.units.get_mut(&player_id) {
            units.push(entity);
        } else {
            self.units.insert(player_id, vec![entity]);
        }

        player_id += 1;
        if player_id >= self.participants.len() {
            turn += 1;
            player_id = 0;
            if turn >= self.units_per_participant {
                self.end_place_phase();
                return;
            }
        }

        self.state = GameStates::Placing(player_id, turn);
        if let Participant::Bot = self.participants[player_id] {
            // do ai action here
            event_writer.send(GameEvent::PlaceAiUnit(player_id));
        }
    }

    fn end_place_phase(&mut self) {
        let all_units: Vec<Vec<(usize, Entity)>> = (0..self.participants.len())
            .map(|participant| {
                let units = self.units.get(&participant).cloned();
                let units =
                    units.map(|units| units.into_iter().map(|unit| (participant, unit)).collect());
                units
            })
            .collect::<Option<_>>()
            .unwrap_or(vec![]);
        let mut all_units: Vec<(usize, Entity)> = all_units.into_iter().flatten().collect();
        all_units.sort_unstable();
        self.turn_order = all_units.into_iter().map(|entity| Some(entity)).collect();
        self.state = GameStates::Turn(0, false);
    }

    fn next_turn(&mut self) {
        let GameStates::Turn(mut current_turn, _) = self.state else {
            return;
        };

        loop {
            current_turn = (current_turn + 1) % self.turn_order.len();
            if self.turn_order[current_turn].is_some() {
                break;
            }
        }
        self.state = GameStates::Turn(current_turn, false);
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
            units: HashMap::new(),
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
            GameStateEvent::MovedUnit(_) => {
                game_state.next_turn();
            }
        }
    }
}

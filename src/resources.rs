use std::fmt;
use std::fmt::Display;
use specs::World;
use crate::events::Event;

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

#[derive(PartialEq)]
pub enum GameplayState {
    Playing,
    Won,
}

impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
    pub keys_pressed_text: String,
}

pub fn register_resources(world: &mut World) {
    world.insert(EventQueue::default());
    world.insert(Gameplay::default());
}
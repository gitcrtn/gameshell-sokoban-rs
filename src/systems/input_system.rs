use std::collections::HashMap;
use gameshell_base::{Core, InputQueue, Keys, Position};
use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};
use specs::world::Index;
use crate::components::{Immovable, Movable, Player};
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::events::{EntityMoved, Event};
use crate::resources::{EventQueue, Gameplay, GameplayState};

pub struct InputSystem {
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, InputQueue>,
        Write<'a, Gameplay>,
        Write<'a, Core>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut events,
            mut input_queue,
            mut gameplay,
            mut core,
            entities,
            mut positions,
            players,
            movables,
            immovables,
        ) = data;

        let mut to_move = Vec::new();

        for (position, _player) in (&positions, &players).join() {
            // Get the first key pressed
            if let Some(key) = input_queue.keys_pressed.pop() {
                if key == Keys::A {
                    core.reset = true;
                    continue;
                }

                if gameplay.state == GameplayState::Won {
                    return;
                }

                // get all the movables and immovables
                let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                // Now iterate through current position to the end of the map
                // on the correct axis and check what needs to move.
                let (start, end, is_x) = match key {
                    Keys::Up => (position.y, 0, false),
                    Keys::Down => (position.y, MAP_HEIGHT, false),
                    Keys::Left => (position.x, 0, true),
                    Keys::Right => (position.x, MAP_WIDTH, true),
                    Keys::Select => {
                        events.events.push(Event::Beep);
                        continue;
                    },
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    // find a movable
                    // if it exists, we try to move it and continue
                    // if it doesn't exist, we continue and try to find an immovable instead
                    match mov.get(&pos) {
                        Some(id) => to_move.push((key, id.clone())),
                        None => {
                            // find an immovable
                            // if it exists, we need to stop and not move anything
                            // if it doesn't exist, we stop because we found a gap
                            match immov.get(&pos) {
                                Some(_id) => {
                                    to_move.clear();
                                    events.events.push(Event::PlayerHitObstacle {})
                                }
                                None => break,
                            }
                        }
                    }
                }
            }
        }

        // We've just moved, so let's increase the number of moves
        if to_move.len() > 0 {
            gameplay.moves_count += 1;
        }

        // Now actually move what needs to be moved
        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            if let Some(position) = position {
                match key {
                    Keys::Up => position.y -= 1,
                    Keys::Down => position.y += 1,
                    Keys::Left => position.x -= 1,
                    Keys::Right => position.x += 1,
                    _ => (),
                }
            }

            // Fire an event for the entity that just moved
            events.events.push(Event::EntityMoved(EntityMoved { id }));
        }

        // if input_queue.keys_downed.len() > 0 {
        //     let text = get_keys_text(&input_queue.keys_downed);
        //     events.events.push(Event::KeysDowned(KeysDowned { text }));
        // }
    }
}
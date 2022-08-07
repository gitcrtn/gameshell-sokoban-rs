use std::collections::HashMap;
use gameshell_base::{AudioContext, Position};
use specs::{Entities, Join, ReadStorage, System, Write};
use crate::audio::{AudioId, Sound};
use crate::components::{BoxContainer, BoxSpot};
use crate::events::{BoxPlacedOnSpot, EntityMoved, Event};
use crate::resources::EventQueue;

pub struct EventSystem<'a, 'b> {
    pub audio_context: &'a mut AudioContext<'b, Sound>,
}

impl<'a, 'b> System<'a> for EventSystem<'a, 'b> {
    type SystemData = (
        Write<'a, EventQueue>,
        Entities<'a>,
        ReadStorage<'a, BoxContainer>,
        ReadStorage<'a, BoxSpot>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut event_queue, entities, boxes, box_spots, positions) = data;

        let mut new_events = Vec::new();
        // let mut keys_pressed = false;

        for event in event_queue.events.drain(..) {
            match event {
                Event::Beep => {
                    self.audio_context.play_sound(AudioId::Correct);
                },
                Event::PlayerHitObstacle => {
                    self.audio_context.play_sound(AudioId::Wall);
                }
                Event::EntityMoved(EntityMoved { id }) => {
                    // An entity was just moved, check if it was a box and fire
                    // more events if it's been moved on a spot.
                    if let Some(the_box) = boxes.get(entities.entity(id)) {
                        let box_spots_with_positions: HashMap<(u8, u8), &BoxSpot> =
                            (&box_spots, &positions)
                                .join()
                                .map(|t| ((t.1.x, t.1.y), t.0))
                                .collect::<HashMap<_, _>>();

                        if let Some(box_position) = positions.get(entities.entity(id)) {
                            // Check if there is a spot on this position, and if there
                            // is if it's the correct or incorrect type
                            if let Some(box_spot) =
                            box_spots_with_positions.get(&(box_position.x, box_position.y))
                            {
                                new_events.push(Event::BoxPlacedOnSpot(BoxPlacedOnSpot {
                                    is_correct_spot: (box_spot.color == the_box.color),
                                }));
                            }
                        }
                    }
                }
                Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) => {
                    // play sound here
                    let sound = if is_correct_spot {
                        AudioId::Correct
                    } else {
                        AudioId::Incorrect
                    };
                    self.audio_context.play_sound(sound);
                }
                // Event::KeysDowned(KeysDowned { text }) => {
                //     keys_pressed = true;
                //     gameplay.keys_pressed_text = text;
                // },
                _ => {},
            }
        }

        // if !keys_pressed {
        //     gameplay.keys_pressed_text.clear();
        // }

        event_queue.events.append(&mut new_events);
    }
}
use gameshell_base::{Position, Renderable};
use crate::components::*;
use specs::{Builder, World, WorldExt};
use crate::Image;
use crate::texture::TextureId;

pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::<Image>::new_static(TextureId::Wall))
        .with(Wall {})
        .with(Immovable)
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable::<Image>::new_static(TextureId::Floor))
        .build();
}

pub fn create_box(world: &mut World, position: Position, color: BoxColor) {
    let texture_ids = match color {
        BoxColor::Red => vec![
            TextureId::BoxRed1,
            TextureId::BoxRed2,
        ],
        BoxColor::Blue => vec![
            TextureId::BoxBlue1,
            TextureId::BoxBlue2,
        ],
    };
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::<Image>::new_animated(texture_ids))
        .with(BoxContainer { color })
        .with(Movable)
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position, color: BoxColor) {
    let texture_id = match color {
      BoxColor::Red => TextureId::BoxSpotRed,
        BoxColor::Blue => TextureId::BoxSpotBlue,
    };
    world
        .create_entity()
        .with(Position { z: 9, ..position })
        .with(Renderable::<Image>::new_static(texture_id))
        .with(BoxSpot { color })
        .build();
}

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::<Image>::new_animated(vec![
            TextureId::Player1,
            TextureId::Player2,
            TextureId::Player3,
        ]))
        .with(Player {})
        .with(Movable)
        .build();
}
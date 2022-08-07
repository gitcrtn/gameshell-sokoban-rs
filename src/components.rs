use specs::{Component, NullStorage, VecStorage, World, WorldExt};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

#[derive(PartialEq)]
pub enum BoxColor {
    Red,
    Blue,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxContainer {
    pub color: BoxColor,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {
    pub color: BoxColor,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

pub fn register_components(world: &mut World) {
    world.register::<Player>();
    world.register::<Wall>();
    world.register::<BoxContainer>();
    world.register::<BoxSpot>();
    world.register::<Movable>();
    world.register::<Immovable>();
}
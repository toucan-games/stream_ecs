use core::time::Duration;

use stream_ecs::{
    component::{Component, storage::array::DenseArrayStorage},
    entity::{DefaultEntity, registry::array::DenseArrayRegistry},
    hlist::{HList, Nil, hlist},
    resource::Resource,
    world::World,
};

pub const MAX_ENTITIES: usize = 10;
pub const TIME_DELTA_FOR_60_FPS: f32 = 1.0 / 60.0;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Component)]
#[component(storage = DenseArrayStorage<Position, MAX_ENTITIES>)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Component)]
#[component(storage = DenseArrayStorage<Velocity, MAX_ENTITIES>)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, Resource)]
pub struct TimeDelta(pub Duration);

pub fn main() {
    let entities = DenseArrayRegistry::<MAX_ENTITIES>::new();
    let components = hlist![
        <Position as Component>::Storage::new(),
        <Velocity as Component>::Storage::new(),
    ];
    let resources = hlist![TimeDelta(Duration::from_secs_f32(TIME_DELTA_FOR_60_FPS))];

    let mut world = World::with(entities, components, resources);
    assert!(world.entities().is_empty());

    let TimeDelta(time_delta) = world.get_res::<TimeDelta>().unwrap();
    assert_eq!(time_delta.as_secs_f32(), TIME_DELTA_FOR_60_FPS);

    let entity = world
        .builder_from(Nil)
        .with(Position { x: 0.0, y: 0.0 })
        .build()
        .unwrap();
    assert!(world.contains(entity));

    let position = world.get::<Position>(entity).unwrap();
    assert_eq!(position, Some(&Position { x: 0.0, y: 0.0 }));

    let velocity = world.get::<Velocity>(entity).unwrap();
    assert_eq!(velocity, None);

    let view = world.view::<HList![DefaultEntity, &Position]>().unwrap();
    for hlist![entity, position] in &view {
        println!("Entity: {:?}, Position: {:?}", entity, position);
    }
}

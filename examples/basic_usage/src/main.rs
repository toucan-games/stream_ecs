use core::time::Duration;

use stream_ecs::{
    component::{Component, storage::array::DenseArrayStorage},
    entity::{DefaultEntity, registry::array::DenseArrayRegistry},
    hlist::{HList, Nil, hlist},
    lending_iterator::LendingIterator,
    resource::Resource,
    world::World,
};

pub const MAX_ENTITIES: usize = 10;
pub const TIME_DELTA_FOR_60_FPS: f32 = 1.0 / 60.0;
pub const SPEED: f32 = 100.0;

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
    let dt = time_delta.as_secs_f32();
    assert_eq!(dt, TIME_DELTA_FOR_60_FPS);

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

    let mut view = world
        .view_mut::<HList![DefaultEntity, &mut Position]>()
        .unwrap();
    view.iter_mut().for_each(|hlist![_entity, position]| {
        position.x += SPEED * dt;
        position.y += SPEED * dt;
    });

    let view = world.view::<HList![DefaultEntity, &Position]>().unwrap();
    view.iter().for_each(|hlist![entity, position]| {
        println!("entity is {:?}, position is {:?}", entity, position);
    });
}

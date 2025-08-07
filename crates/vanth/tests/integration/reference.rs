use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::{Event, EventWriter},
    system::Query,
};

#[derive(Event)]
struct LevelUpEvent<T> {
    inner: T,
}

#[derive(Component)]
struct FooTask {
    field: i32,
}

fn player_level_up(mut ev_levelup: EventWriter<LevelUpEvent<i32>>, query: Query<(Entity, &FooTask)>) {
    for (entity, xp) in query.iter() {
        ev_levelup.write(LevelUpEvent::<i32> { inner: 5 });
    }
}

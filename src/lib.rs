use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub struct LazyBundle<B: Bundle, F: Fn() -> B>(F);

pub trait InsertableLazyBundle {
    fn insert(self, entity: Entity, world: &mut World);
}

impl <B, F> InsertableLazyBundle for LazyBundle<B, F> where B: Bundle, F: Fn() -> B {
    fn insert(self, entity: Entity, world: &mut World) {
        world.entity_mut(entity).insert_bundle(self.0());
    }
}

pub trait InsertLazyBundleExt {
    fn insert_lazy_bundle<B>(&mut self, inserter: B) -> &mut Self
    where
        B: InsertableLazyBundle + 'static + Send + Sync;
}

impl InsertLazyBundleExt for EntityCommands<'_, '_, '_> {
    fn insert_lazy_bundle<B>(&mut self, inserter: B) -> &mut Self where
    B: InsertableLazyBundle + 'static + Send + Sync {
        let id = self.id();
        self.commands().add(move |world: &mut World| inserter.insert(id, world));
        self
    }
}

pub trait SpawnLazyBundleExt<'w, 's> {
    fn insert_lazy_bundle<B>(&mut self, insertable_bundle: B) -> EntityCommands<'w, 's, '_>
    where
        B: InsertableLazyBundle + 'static + Send + Sync;
}

impl <'w, 's>  SpawnLazyBundleExt<'w, 's>  for Commands<'w, 's> {
    fn insert_lazy_bundle<B>(&mut self, inserter: B) -> EntityCommands<'w, 's, '_>
    where
        B: InsertableLazyBundle + 'static + Send + Sync {
        let mut entity_commands = self.spawn();
        entity_commands.insert_lazy_bundle(inserter);
        entity_commands
    }
}
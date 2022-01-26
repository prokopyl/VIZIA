use std::{collections::HashSet, marker::PhantomData};

use better_any::{TidExt, TidAble, Tid};

use crate::{Data, Entity, Lens, ModelData};

pub trait LensWrap<'a> {
    fn update(&'a mut self, model: &'a dyn ModelData<'a>) -> bool;
    fn observers(&self) -> &HashSet<Entity>;
    fn add_observer(&mut self, observer: Entity);
    fn entity(&self) -> Entity;
}

pub struct StateStore<'a, L: Lens<'a>, T> {
    // The entity which declared the binding
    pub entity: Entity,
    pub lens: L,
    pub old: T,
    pub observers: HashSet<Entity>,
    p: PhantomData<&'a ()>,
}

impl<'a, L: Lens<'a>, T> LensWrap<'a> for StateStore<'a, L, T>
where
    L: Lens<'a, Target = T>,
    L::Source: Tid<'a>,
    L::Target: Data,
{
    fn entity(&self) -> Entity {
        self.entity
    }

    fn update(&'a mut self, model: &'a dyn ModelData<'a>) -> bool {
        if let Some(data) = model.downcast_ref::<L::Source>() {
            let state = self.lens.view(data);
            if !state.same(&self.old) {
                self.old = state.clone();
                return true;
            }
        }

        false
    }

    fn observers(&self) -> &HashSet<Entity> {
        &self.observers
    }

    fn add_observer(&mut self, observer: Entity) {
        self.observers.insert(observer);
    }
}

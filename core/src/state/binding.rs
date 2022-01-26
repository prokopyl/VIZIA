use std::any::TypeId;
use std::collections::HashSet;
use std::marker::{PhantomPinned, PhantomData};

use better_any::{TidExt, TidAble, Tid, typeid_of};
use morphorm::{LayoutType, PositionType};

use crate::{
    Color, Context, Display, Entity, Handle, StateStore, TreeExt, Units, View, Visibility,
};

use crate::{Data, Lens, Model};

#[derive(Tid)]
pub struct Binding<'a,L>
where
    L: Lens<'a>,
{
    lens: L,
    parent: Entity,
    count: usize,
    builder: Option<Box<dyn Fn(&mut Context, Field<L>)>>,
    p: PhantomData<&'a ()>,
}

impl<'a,L> Binding<'a,L>
where
    L: Lens<'a>,
    L::Source: TidAble<'a>,
    L::Target: Data,
{
    pub fn new<F>(cx: &'a mut Context<'a>, lens: L, builder: F)
    where
        F: Fn(&mut Context, Field<L>),
        L::Source: Model,
    {
        let parent = cx.current;

        let binding = Self { lens, parent, count: cx.count + 1, builder: Some(Box::new(builder)), p: PhantomData::default() };

        let id = if let Some(id) = cx.tree.get_child(cx.current, cx.count) {
            id
        } else {
            let id = cx.entity_manager.create();
            cx.tree.add(id, cx.current).expect("Failed to add to tree");
            cx.cache.add(id).expect("Failed to add to cache");
            cx.style.add(id);
            id
        };

        let ancestors = parent.parent_iter(&cx.tree).collect::<HashSet<_>>();

        for entity in id.parent_iter(&cx.tree) {
            if let Some(model_data_store) = cx.data.get_mut(entity) {
                if let Some(model_data) = model_data_store.data.get(&typeid_of::<L::Source>()) {
                    if let Some(lens_wrap) = model_data_store.lenses.get_mut(&TypeId::of::<L>()) {
                        let observers = lens_wrap.observers();

                        if ancestors.intersection(observers).next().is_none() {
                            lens_wrap.add_observer(id);
                        }
                    } else {
                        let mut observers = HashSet::new();
                        observers.insert(id);

                        let model = model_data.downcast_ref::<L::Source>().unwrap();

                        let old = lens.view(model);

                        model_data_store.lenses.insert(
                            TypeId::of::<L>(),
                            Box::new(StateStore { entity: id, lens, old: old.clone(), observers, p: PhantomData::default() }),
                        );
                    }

                    break;
                }
            }
        }

        cx.views.insert(id, Box::new(binding));

        cx.count += 1;

        // Call the body of the binding
        if let Some(mut view_handler) = cx.views.remove(&id) {
            view_handler.body(cx);
            cx.views.insert(id, view_handler);
        }

        let _: Handle<Self> = Handle { entity: id, p: Default::default(), cx }
            .width(Units::Stretch(1.0))
            .height(Units::Stretch(1.0))
            .background_color(Color::blue())
            .display(Display::None);
    }
}

impl<'a, L: Lens<'a>> View for Binding<'a, L> {
    fn body(&mut self, cx: &mut Context) {
        if let Some(builder) = self.builder.take() {
            //let prev = cx.current;
            //let count = cx.count;
            cx.current = self.parent;
            cx.count = self.count;
            (builder)(cx, Field { lens: self.lens.clone(), p: PhantomData::default()});
            //cx.current = prev;
            //cx.count = count;
            self.builder = Some(builder);
        }
    }
}

#[derive(Clone, Copy)]
pub struct Field<'a, L> {
    lens: L,
    p: PhantomData<&'a ()>,
}

impl<'a, L: Lens<'a>> Field<'a, L> 
where 
    L::Source: TidAble<'a>,
{
    pub fn get(&self, cx: &'a Context<'a>) -> &'a L::Target {
        self.lens.view(cx.data().expect(&format!(
            "Failed to get {:?} for entity: {:?}. Is the data in the tree?",
            self.lens, cx.current
        )))
    }
}

macro_rules! impl_res_simple {
    ($t:ty) => {
        impl<'a> Res<'a, $t> for $t {
            fn get(&'a self, _: &'a Context) -> &'a $t {
                self
            }
        }
    };
}

pub trait Res<'a,T> {
    fn get(&'a self, cx: &'a Context<'a>) -> &'a T;
}

impl_res_simple!(i8);
impl_res_simple!(i16);
impl_res_simple!(i32);
impl_res_simple!(i64);
impl_res_simple!(i128);
impl_res_simple!(isize);
impl_res_simple!(u8);
impl_res_simple!(u16);
impl_res_simple!(u32);
impl_res_simple!(u64);
impl_res_simple!(u128);
impl_res_simple!(usize);
impl_res_simple!(char);
impl_res_simple!(bool);
impl_res_simple!(f32);
impl_res_simple!(f64);

impl<'a, T, L> Res<'a, T> for Field<'a, L>
where
    L: Lens<'a, Target = T>,
{
    fn get(&'a self, cx: &'a Context<'a>) -> &'a T {
        self.get(cx)
    }
}

impl<'a> Res<'a, Color> for Color {
    fn get(&'a self, _: &'a Context) -> &'a Color {
        self
    }
}

impl<'a> Res<'a, Units> for Units {
    fn get(&'a self, _: &'a Context) -> &'a Units {
        self
    }
}

impl<'a> Res<'a, Visibility> for Visibility {
    fn get(&'a self, _: &'a Context) -> &'a Visibility {
        self
    }
}

impl<'a> Res<'a, Display> for Display {
    fn get(&'a self, _: &'a Context) -> &'a Display {
        self
    }
}

impl<'a> Res<'a, LayoutType> for LayoutType {
    fn get(&'a self, _: &'a Context) -> &'a LayoutType {
        self
    }
}

impl<'a> Res<'a, PositionType> for PositionType {
    fn get(&'a self, _: &'a Context) -> &'a PositionType {
        self
    }
}

impl<'a, T> Res<'a, (T, T)> for (T, T) {
    fn get(&'a self, _: &'a Context) -> &'a (T, T) {
        self
    }
}

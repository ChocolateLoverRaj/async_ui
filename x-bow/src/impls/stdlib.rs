use x_bow_macros::Track;

use crate::trackable::Trackable;

#[derive(Track)]
#[x_bow(module_prefix = crate::__private_macro_only)]
struct Test<T> {
    value: T,
}
macro_rules! leaf_primitive {
    ($primitive:ty) => {
        impl<E> Trackable<E> for $primitive
        where
            E: crate::edge::EdgeTrait<Data = $primitive>,
        {
            type Tracked = super::TrackedLeaf<$primitive, E>;
        }
    };
}
leaf_primitive!(bool);
leaf_primitive!(char);
leaf_primitive!(f32);
leaf_primitive!(f64);
leaf_primitive!(i128);
leaf_primitive!(i16);
leaf_primitive!(i32);
leaf_primitive!(i64);
leaf_primitive!(i8);
leaf_primitive!(isize);
leaf_primitive!(u128);
leaf_primitive!(u16);
leaf_primitive!(u32);
leaf_primitive!(u64);
leaf_primitive!(u8);
leaf_primitive!(usize);

mod option {
    use x_bow_macros::Track;
    #[allow(dead_code)]
    #[derive(Track)]
    #[x_bow(module_prefix = crate::__private_macro_only)]
    #[x_bow(remote_type = Option)]
    pub enum ImitateOption<T> {
        Some(T),
        None,
    }
}
mod vector {
    use std::{
        cell::RefCell,
        collections::{btree_map::Entry, BTreeMap},
        marker::PhantomData,
        rc::{Rc, Weak},
    };

    use crate::{
        edge::{Edge, EdgeTrait},
        mapper::Mapper,
        optional::OptionalYes,
        trackable::{HandlePart, Trackable},
        tracked::Tracked,
    };

    #[allow(non_camel_case_types)]
    pub struct XBowTracked_Vec<T, E>
    where
        E: EdgeTrait<Data = Vec<T>>,
        T: Trackable<Edge<E, MapperVec<T>, OptionalYes>>,
    {
        items: RefCell<BTreeMap<usize, Weak<HandlePart<T, Edge<E, MapperVec<T>, OptionalYes>>>>>,
        incoming_edge: Rc<E>,
    }

    pub struct MapperVec<T> {
        index: usize,
        _phantom: PhantomData<T>,
    }
    impl<T> Clone for MapperVec<T> {
        fn clone(&self) -> Self {
            Self {
                index: self.index,
                _phantom: PhantomData,
            }
        }
    }
    impl<T> Mapper for MapperVec<T> {
        type In = Vec<T>;
        type Out = T;
        fn map<'s, 'd>(&'s self, input: &'d Self::In) -> Option<&'d Self::Out> {
            input.get(self.index)
        }
        fn map_mut<'s, 'd>(&'s self, input: &'d mut Self::In) -> Option<&'d mut Self::Out> {
            input.get_mut(self.index)
        }
    }
    impl<T, E> Tracked for XBowTracked_Vec<T, E>
    where
        E: EdgeTrait<Data = Vec<T>>,
        T: Trackable<Edge<E, MapperVec<T>, OptionalYes>>,
    {
        type Edge = E;
        fn new(edge: Rc<Self::Edge>) -> Self {
            let items = RefCell::new(BTreeMap::new());
            Self {
                items,
                incoming_edge: edge,
            }
        }
        fn edge(&self) -> &Rc<Self::Edge> {
            &self.incoming_edge
        }
        fn invalidate_down_outside(&self) {
            self.edge().invalidate_here_outside();
            self.items.borrow_mut().retain(|_key, value| {
                if let Some(item) = value.upgrade() {
                    item.invalidate_down_outside();
                    true
                } else {
                    false
                }
            });
        }
    }

    impl<T, E> XBowTracked_Vec<T, E>
    where
        E: EdgeTrait<Data = Vec<T>>,
        T: Trackable<Edge<E, MapperVec<T>, OptionalYes>>,
    {
        fn create_item(
            &self,
            index: usize,
        ) -> Rc<HandlePart<T, Edge<E, MapperVec<T>, OptionalYes>>> {
            let edge = Edge::new(
                self.incoming_edge.clone(),
                MapperVec {
                    index,
                    _phantom: PhantomData,
                },
            );
            Rc::new(Tracked::new(Rc::new(edge)))
        }
        pub fn handle_at(
            &self,
            index: usize,
        ) -> Rc<HandlePart<T, Edge<E, MapperVec<T>, OptionalYes>>> {
            match self.items.borrow_mut().entry(index) {
                Entry::Vacant(vacant) => {
                    let tracked = self.create_item(index);
                    vacant.insert(Rc::downgrade(&tracked));
                    tracked
                }
                Entry::Occupied(mut occupied) => {
                    let value = occupied.get_mut();
                    if let Some(tracked) = value.upgrade() {
                        tracked
                    } else {
                        let tracked = self.create_item(index);
                        *value = Rc::downgrade(&tracked);
                        tracked
                    }
                }
            }
        }
    }
    impl<T, E> Trackable<E> for Vec<T>
    where
        E: EdgeTrait<Data = Vec<T>>,
        T: Trackable<Edge<E, MapperVec<T>, OptionalYes>>,
    {
        type Tracked = XBowTracked_Vec<T, E>;
    }
}

// #[allow(non_snake_case)]
// pub struct POption<T, E>
// where
//     T: Trackable<Edge<E, MapperOption<T>, OptionalYes>>,
//     E: EdgeTrait<Data = Option<T>>,
// {
//     pub Some: HandlePart<T, Edge<E, MapperOption<T>, OptionalYes>>,
//     incoming_edge: Rc<E>,
// }
// pub struct MapperOption<T>(PhantomData<T>);

// impl<T> Clone for MapperOption<T> {
//     fn clone(&self) -> Self {
//         Self(PhantomData)
//     }
// }
// impl<T> Mapper for MapperOption<T> {
//     type In = Option<T>;
//     type Out = T;
//     #[inline]
//     fn map<'s, 'd>(&'s self, input: &'d Self::In) -> Option<&'d Self::Out> {
//         input.as_ref()
//     }
//     #[inline]
//     fn map_mut<'s, 'd>(&'s self, input: &'d mut Self::In) -> Option<&'d mut Self::Out> {
//         input.as_mut()
//     }
// }
// impl<T, E> Tracked for POption<T, E>
// where
//     E: EdgeTrait<Data = Option<T>>,
//     T: Trackable<Edge<E, MapperOption<T>, OptionalYes>>,
// {
//     type Edge = E;

//     fn new(edge: Rc<E>) -> Self {
//         Self {
//             Some: Tracked::new(Rc::new(Edge::new(edge.clone(), MapperOption(PhantomData)))),
//             incoming_edge: edge,
//         }
//     }
//     fn edge(&self) -> &Rc<Self::Edge> {
//         &self.incoming_edge
//     }
//     fn invalidate_here_down(&self) {
//         self.edge().invalidate_here();
//     }
// }
// impl<T, E> Trackable<E> for Option<T>
// where
//     T: Trackable<Edge<E, MapperOption<T>, OptionalYes>>,
//     E: EdgeTrait<Data = Option<T>>,
// {
//     type Tracked = POption<T, E>;
// }

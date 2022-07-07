use std::{
    any::{Any, TypeId},
    collections::HashMap,
    slice::Iter
};

trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl<T> AsAny for Vec<Option<T>>
where
    T: 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Default)]
pub struct ComponentStorage {
    data: HashMap<TypeId, Box<dyn AsAny>>,
}

impl ComponentStorage {
    fn add_type<T>(&mut self)
    where
        T: 'static,
    {
        let id = TypeId::of::<T>();
        self.data.entry(id).or_insert_with(|| {
            let vec: Vec<Option<T>> = Vec::new();
            Box::new(vec)
        });
    }

    fn get_vec<T>(&self) -> Option<&Vec<Option<T>>>
    where
        T: 'static,
    {
        let id = TypeId::of::<T>();
        let boxed_vec = match self.data.get(&id) {
            Some(v) => v,
            None => return None,
        };

        boxed_vec.as_any().downcast_ref::<Vec<Option<T>>>()
    }

    fn get_mut_vec<T>(&mut self) -> Option<&mut Vec<Option<T>>>
    where
        T: 'static,
    {
        let id = TypeId::of::<T>();
        let boxed_vec = match self.data.get_mut(&id) {
            Some(v) => v,
            None => return None,
        };

        boxed_vec.as_mut_any().downcast_mut::<Vec<Option<T>>>()
    }

    pub fn add_component<T>(&mut self, index: usize, component: T)
    where
        T: 'static,
    {
        self.add_type::<T>();
        match self.get_mut_vec::<T>() {
            Some(vec_t) => {
                if index >= vec_t.len() {
                    vec_t.resize_with(index + 1, || None);
                    vec_t[index] = Some(component)
                }
            }
            None => {}
        }
    }

    pub fn get_component<T>(&self, index: usize) -> Option<&T>
    where
        T: 'static,
    {
        match self.get_vec::<T>() {
            Some(vec_t) => {
                if index < vec_t.len() {
                    vec_t[index].as_ref()
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn get_mut_component<T>(&mut self, index: usize) -> Option<&mut T>
    where
        T: 'static,
    {
        match self.get_mut_vec::<T>() {
            Some(vec_t) => {
                if index < vec_t.len() {
                    vec_t[index].as_mut()
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn get_components_iter<T>(&self) -> Option<Iter<'_, Option<T>>> where T: 'static { 
        self.get_vec::<T>().map(|vec_t| vec_t.iter())
    }

    pub fn remove_component<T>(&mut self, index: usize) where T: 'static { 
        match self.get_mut_vec::<T>() { 
            Some(vec_t) => { 
                vec_t[index] = None;
            }
            None => { }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentStorage;

    #[derive(Debug)]
    struct Foo;

    #[derive(Debug)]
    struct Bar {
        x: u32,
    }

    #[test]
    fn add_type() {
        let mut cs = ComponentStorage::default();
        assert_eq!(cs.data.len(), 0);
        cs.add_type::<Foo>();
        cs.add_type::<Bar>();
        assert_eq!(cs.data.len(), 2);
    }

    #[test]
    fn add_same_type() {
        let mut cs = ComponentStorage::default();
        assert_eq!(cs.data.len(), 0);
        cs.add_type::<Foo>();
        cs.add_type::<Foo>();
        assert_eq!(cs.data.len(), 1);
    }

    #[test]
    fn get_vec() {
        let mut cs = ComponentStorage::default();
        cs.add_type::<Foo>();
        let foo_vec = cs.get_vec::<Foo>();
        let bar_vec = cs.get_vec::<Bar>();
        assert!(foo_vec.is_some());
        assert!(bar_vec.is_none());
    }

    #[test]
    fn get_mut_vec() {
        let mut cs = ComponentStorage::default();
        cs.add_type::<Foo>();
        let foo_vec = cs.get_mut_vec::<Foo>();
        assert!(foo_vec.is_some());
        let bar_vec = cs.get_mut_vec::<Bar>();
        assert!(bar_vec.is_none());
    }

    #[test]
    fn add_component() {
        let mut cs = ComponentStorage::default();
        cs.add_component(0, Foo {});
        match cs.get_vec::<Foo>() {
            Some(vec_t) => assert_eq!(vec_t.len(), 1),
            None => panic!("Vec should exist'"),
        }
    }

    #[test]
    fn add_component_resize_check() { 
        let mut cs = ComponentStorage::default();
        cs.add_component(0, Foo {});
        match cs.get_vec::<Foo>() {
            Some(vec_t) => assert_eq!(vec_t.len(), 1),
            None => panic!("Vec should exist'"),
        }

        cs.add_component(10, Foo {});
        match cs.get_vec::<Foo>() {
            Some(vec_t) => assert_eq!(vec_t.len(), 11),
            None => panic!("Vec should exist'"),
        }

    }

    #[test]
    fn get_component() {
        let mut cs = ComponentStorage::default();
        cs.add_component(0, Foo {});
        match cs.get_component::<Foo>(0) {
            Some(_) => {}
            None => panic!("Component should exist"),
        }
    }

    #[test]
    #[should_panic]
    fn get_component_2() {
        let mut cs = ComponentStorage::default();
        cs.add_component(0, Foo {});
        match cs.get_component::<Bar>(0) {
            Some(_) => {}
            None => panic!("Component should exist"),
        }
    }

    #[test]
    fn get_mut_component() {
        let mut cs = ComponentStorage::default();
        cs.add_component(0, Foo {});
        match cs.get_mut_component::<Foo>(0) {
            Some(_) => {}
            None => panic!("Component should exist"),
        }
    }

    #[test]
    #[should_panic]
    fn get_mut_component_2() {
        let mut cs = ComponentStorage::default();
        cs.add_component(0, Foo {});
        match cs.get_mut_component::<Bar>(0) {
            Some(_) => {}
            None => panic!("Component should exist"),
        }
    }

    #[test]
    fn has_mut_component_changed() {
        let mut cs = ComponentStorage::default();
        cs.add_component(0, Bar { x: 100 });
        match cs.get_mut_component::<Bar>(0) {
            Some(bar) => {
                assert_eq!(bar.x, 100);
                bar.x = 200;
            }
            None => panic!("Component should exist"),
        }

        match cs.get_mut_component::<Bar>(0) {
            Some(bar) => {
                assert_eq!(bar.x, 200);
            }
            None => panic!("Component should exist"),
        }
    }

    #[test]
    fn get_components_iter() { 
        let mut cs = ComponentStorage::default();
        cs.add_component(0, Bar { x: 100 });
        match cs.get_components_iter::<Bar>() { 
            Some(_) => { }
            None => { panic!("Failed to get iter for components")}
        };
    }

    #[test]
    fn remove_component() { 
        let mut cs = ComponentStorage::default();
        cs.add_component(0, Foo { });
        match cs.get_component::<Foo>(0) {
            Some(_) => { },
            None => { panic!("Added the wrong component to entity")}
        };
        cs.remove_component::<Foo>(0);
        match cs.get_component::<Foo>(0) {
            Some(_) => { panic!("Failed to remove component") },
            None => { }
        };
    }
    
}

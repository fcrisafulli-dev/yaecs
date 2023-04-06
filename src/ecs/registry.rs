use super::{ComponentRegistry, MAX_ENTITY_CNT, ComponentMask, Mask};
use std::{any::{TypeId, Any}, collections::HashMap, cell::{RefCell, RefMut}, rc::Rc};
extern crate paste;



/// Main functionality for the registry
impl ComponentRegistry {
    fn _new_empty() -> ComponentRegistry {
        ComponentRegistry { 
            component_database: HashMap::new(),
            component_masks: HashMap::new(),
            entity_masks: Vec::with_capacity(MAX_ENTITY_CNT),
            num_component_types: 0,
            num_entities: 0
        }
    }

    fn _register_component(&mut self, id: TypeId) {

        self.component_database.insert(id, Vec::with_capacity(MAX_ENTITY_CNT));
        self.component_masks.insert(id, ComponentMask((1 as Mask) << self.num_component_types));

        self.num_component_types += 1;
    }

    /// Create a new registry with a fixed number of component types
    pub fn new(ids: Vec<TypeId>) -> ComponentRegistry {
        let mut new_registry = ComponentRegistry::_new_empty();

        for id in ids.into_iter(){
            new_registry._register_component(id);
        }

        new_registry
    }


    pub fn new_entity(&mut self) -> &mut Self{
        self.component_database.iter_mut().for_each(|(_, component_vec)|{
            //Doing this seems like a crime. How slow is this?
            component_vec.push(Rc::new(RefCell::new(0u8)));
        });

        self.entity_masks.push(ComponentMask::empty());
        self.num_entities += 1;

        self
    }

    /// Adds a component to the last created entity
    pub fn with_component<C>(&mut self, component: C) -> &mut Self
        where
            C: Any + 'static{

            let tid = &TypeId::of::<C>();
            // Get the last added component in the category `C`
            let last_component = self.component_database.get_mut(tid).unwrap().last_mut().unwrap();
            *last_component = Rc::new(RefCell::new(component));
            
            // Get the last added mask and mark this component as enabled
            let last_mask = self.entity_masks.last_mut().expect("Missing component mask");
            let mask_to_enable = self.component_masks.get(tid).unwrap();
            *last_mask |= *mask_to_enable;

            self
    }

    pub fn _mask_of<C>(&self) -> ComponentMask 
        where
            C: Any + 'static{
        
        *self.component_masks.get(&TypeId::of::<C>()).unwrap()
    }

    pub fn num_registered_components(&self) -> usize {
        self.num_component_types
    }

    pub fn num_entities(&self) -> usize {
        self.num_entities
    }

    
    pub fn save_id(&mut self) -> usize {
        self.num_entities - 1
    }

    pub fn entity_has_components(&self, eid: usize, mask: ComponentMask) -> bool {
        match self.entity_masks.get(eid) {
            Some(m) => {
                m.has(&mask)
            },
            None => false,
        }
        
    }

    pub fn mask_filter(&self, f_mask: ComponentMask) -> Vec<usize>{

        let idx:Vec<usize> = self.entity_masks.iter().enumerate().filter_map(|(idx,e_mask)|{
            if e_mask.has(&f_mask){
                Some(idx)
            } else {
                None
            }
        }).collect();

        idx
    }

    pub fn get_component_from_id<C>(&self, eid: usize) -> RefMut<C> where C: Any + 'static {
        let a = self.component_database.get(&TypeId::of::<C>()).unwrap().get(eid).expect("Queried for an entity but it did not exist");
        let component_downcasted: RefMut<C> = RefMut::map(a.borrow_mut(), |mut_item|{
            mut_item.downcast_mut::<C>().unwrap()
        });

        component_downcasted
    }

    //this function is cursed
    pub fn id_filter<C>(&self, ids: &Vec<usize>) -> Vec<RefMut<C>>
        where
            C: Any + 'static{
        
        let mut idi = ids.into_iter();
        let mut current_id = idi.next();

        let tid = &TypeId::of::<C>();
        let components: Vec<RefMut<C>> = self.component_database.get(tid).unwrap()
            .iter().enumerate()
            .filter_map(|(eid, rcrc)|{
                match current_id {
                    Some(cid) => {
                        if *cid == eid {
                
                            let rmm: RefMut<C> = RefMut::map(rcrc.borrow_mut(), |x|{
                                x.downcast_mut::<C>().unwrap()
                            });
                            
                            current_id = idi.next();
                            Some(rmm)

                        } else {
                            None::<RefMut<C>>
                        }
                    },
                    None => None::<RefMut<C>>,
                }
            }).collect();
        components
    }

    /// Tries to delete an entity, returns true if the entity was actually deleted
    pub fn delete_entity(&mut self, eid: usize) -> bool {
        match self.entity_masks.get_mut(eid) {
            Some(m) => {
                *m = ComponentMask::empty();
                true
            },
            None => {
                false
            },
        }
    }


}

#[macro_export]
macro_rules! build_registry {
    ($($c:ty),+) => {
        {
            use $crate::ecs::ComponentRegistry;
            let mut ids: Vec<std::any::TypeId> = Vec::with_capacity(32);
            $(
                let id = std::any::TypeId::of::<$c>();
                ids.push(id);
            )+

            ComponentRegistry::new(ids)
        }
    };
}


/// Main query macro which returns a vector of tuples of `RefMut<Component>`
/// # Example
/// ```ignore
/// // ALWAYS enclose a query in a scope, untill I figure out how to do magic with closures
/// { // start scope
///     let query = query_components!(some_registry: SomeComponent, AnotherComponent, ETC...);
/// } // end scope
/// ```
/// # Expansion
/// Here is an example for what this might expand to:
/// ```ignore
/// let q_mask = query_mask!(r: Hp, UVec2, Pos, Light);
/// dbg!(&q_mask);
///
/// let idxs = r.mask_filter(q_mask);
/// dbg!(&idxs);
///
/// let mut hp_request = r.id_filter::<Hp>(&idxs).into_iter();
/// let mut vec2_request = r.id_filter::<UVec2>(&idxs).into_iter();
/// let mut pos_request = r.id_filter::<Pos>(&idxs).into_iter();
/// let mut light_request = r.id_filter::<Light>(&idxs).into_iter();
///
///
/// let mut flat_zip: Vec<(
///     RefMut<Hp>,
///     RefMut<UVec2>,
///     RefMut<Pos>,
///     RefMut<Light>,
/// )> = Vec::with_capacity(idxs.len());
///
///
/// for _ in idxs {
///     flat_zip.push((
///         hp_request.next().unwrap(),
///         vec2_request.next().unwrap(),
///         pos_request.next().unwrap(),
///         light_request.next().unwrap(),
///     ))
/// }
/// ```
#[macro_export]
macro_rules! query_components {
    ($r:ident: $($c:ty),+) => {
        {
            use $crate::ecs::ComponentMask;
            use std::cell::RefMut;
            use paste::paste;
            //Get a component mask to filter with
            let mut component_mask = ComponentMask::empty();
            $(
                component_mask |= $r._mask_of::<$c>();
            )+
            
            let filtered_indices = $r.mask_filter(component_mask);

            //Request the components
            $(
                paste!{
                    #[allow(non_snake_case)]
                    let mut [<macro_generated_ $c _request>] = $r.id_filter::<$c>(&filtered_indices).into_iter();
                }
            )+

            let mut flat_zip: Vec<(
                $(
                    RefMut<$c>,
                )+
            )> = Vec::with_capacity(filtered_indices.len());


            for _ in filtered_indices {
                flat_zip.push(
                    (
                        $(
                            paste!{        
                                    [<macro_generated_ $c _request>].next().unwrap()
                            },
                        )+
                    )
                )
            }

            flat_zip
        }
    };

    ($s:ident.$r:ident: $($c:ty),+) => {
        {
            use $crate::ecs::ComponentMask;
            use std::cell::RefMut;
            use paste::paste;
            //Get a component mask to filter with
            let mut component_mask = ComponentMask::empty();
            $(
                component_mask |= $s.$r._mask_of::<$c>();
            )+
            
            let filtered_indices = $s.$r.mask_filter(component_mask);

            //Request the components
            $(
                paste!{
                    #[allow(non_snake_case)]
                    let mut [<macro_generated_ $c _request>] = $s.$r.id_filter::<$c>(&filtered_indices).into_iter();
                }
            )+

            let mut flat_zip: Vec<(
                $(
                    RefMut<$c>,
                )+
            )> = Vec::with_capacity(filtered_indices.len());


            for _ in filtered_indices {
                flat_zip.push(
                    (
                        $(
                            paste!{        
                                    [<macro_generated_ $c _request>].next().unwrap()
                            },
                        )+
                    )
                )
            }

            flat_zip
        }
    };
}

/// Query components for a single entity
#[macro_export]
macro_rules! query_entity {
    ($r:ident[$eid:expr]: $($c:ty),+) => {
        {
            use $crate::ecs::ComponentMask;
            use std::cell::RefMut;
            use paste::paste;

            //Get a component mask to filter with
            let mut component_mask = ComponentMask::empty();
            $(
                component_mask |= $r._mask_of::<$c>();
            )+
            
            if $r.entity_has_components($eid, component_mask) {
                let component_tup = (
                    $(
                        $r.get_component_from_id::<$c>($eid),
                    )+
                );
                Some(component_tup)
                
            } else {
                None
            }
        }
    };
}


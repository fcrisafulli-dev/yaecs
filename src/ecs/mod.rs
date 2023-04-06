use std::{collections::HashMap, any::{TypeId, Any}, cell::RefCell, rc::Rc, fmt::Debug, ops::{BitOrAssign, BitAnd}, process::Output};

pub mod registry;
pub mod registry_ext;
pub mod query;



pub const MAX_ENTITY_CNT:usize = 99;

type Mask = u32;
#[derive(PartialEq, Eq)]
pub struct ComponentMask(Mask);

#[derive(Debug)]
pub struct ComponentRegistry{
    component_database: HashMap<TypeId, Vec<Rc<RefCell<dyn Any>>>>,
    
    /// Maps a component type to its bitmask
    component_masks: HashMap<TypeId, ComponentMask>,

    entity_masks: Vec<ComponentMask>,

    num_component_types: usize,

    num_entities: usize
}


impl Debug for ComponentMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Mask: {:#032b}", self.0))
    }
}

impl Clone for ComponentMask{
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl Copy for ComponentMask {}

impl BitOrAssign for ComponentMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for ComponentMask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ComponentMask {
    pub fn empty() -> ComponentMask{
        ComponentMask(0 as Mask)
    }

    fn has(&self, f_mask: &ComponentMask) -> bool {
        if (self.0 & f_mask.0) == f_mask.0 {
            true
        } else {
            false
        }
    }
}

#[macro_export]
macro_rules! query_mask {
    ($r:ident: $($c:ty),+) => {
        {
            let mut out = ecs::ComponentMask::empty();
            
            $(
                out |= $r._mask_of::<$c>();
            )+

            out
        }
    };
}
use std::{any::{Any, TypeId}, fmt::Debug};

use super::ComponentRegistry;

/// Additional -- non neccesary functionality
impl ComponentRegistry {
    pub fn _peek_last_added_trait<C>(&self) -> std::rc::Rc<std::cell::RefCell<dyn Any>> 
        where
            C: Any +'static{
        
        let last = self.component_database.get(&TypeId::of::<C>()).unwrap().last().unwrap().clone();
        last
    }
}


use yaecs::{build_registry, query_entity, query_components, ComponentRegistry};

struct SimpleComponent(f32);

struct ComplexComponent(f32,f32,Vec<f32>);

struct FillerComponentA(i64);
struct FillerComponentB(i64);
struct FillerComponentC(i64);
struct FillerComponentD(i64);

#[test]
fn test_build_registry(){

    let registry = build_registry!(
        SimpleComponent, 
        ComplexComponent,
        FillerComponentA,
        FillerComponentB,
        FillerComponentC,
        FillerComponentD
    );

    assert!(registry.num_registered_components() == 6 as usize);
}

#[test]
fn test_adding_components() {
    let mut registry = build_registry!(
        SimpleComponent, 
        ComplexComponent,
        FillerComponentA,
        FillerComponentB,
        FillerComponentC,
        FillerComponentD
    );

    let my_entity = registry.new_entity()
        .with_component(SimpleComponent(7.7))
        .save_id();    

    {
        let q = query_entity!(registry[my_entity]: SimpleComponent);
        let q = q.unwrap().0;

        assert!(q.0 == 7.7);
    }
}

#[test]
fn test_modifying_components() {
    let mut registry = build_registry!(
        SimpleComponent, 
        ComplexComponent,
        FillerComponentA,
        FillerComponentB,
        FillerComponentC,
        FillerComponentD
    );

    let my_entity = registry.new_entity()
        .with_component(SimpleComponent(7.7))
        .with_component(ComplexComponent(22.22,33.33,vec![99.7,113.0]))
        .save_id();    

    {
        let q = query_entity!(registry[my_entity]: SimpleComponent);
        let mut q = q.unwrap().0;

        (*q).0 = 1234.4321;
    }
    
    {
        let q = query_entity!(registry[my_entity]: SimpleComponent);
        let q = q.unwrap().0;

        assert!(q.0 == 1234.4321);
    }

    {
        let q = query_entity!(registry[my_entity]: SimpleComponent, ComplexComponent);
        let (mut s, mut c) = q.unwrap();
       

        (*s).0 = 9.9;
        
        (*c).0 = 5.5;
        (*c).1 = 5.5;
    }
    
    {
        //uid 2 should not exist
        let q = query_entity!(registry[2]: SimpleComponent, ComplexComponent);
        let q2 = q;
        assert!(q2.is_none());
    }
}
#[test]
fn test_identifier() {
    struct Container{pub reg: ComponentRegistry}

    impl Container {
        pub fn do_query(&self){
            {
                let query = query_components!(self.reg => SimpleComponent, FillerComponentA);
        
                for (simple, ..) in query {
                    assert!((*simple).0 == 7.7);
                }
            }
        }
    }

    let mut container = Container{
        reg: build_registry!(
            SimpleComponent, 
            ComplexComponent,
            FillerComponentA,
            FillerComponentB,
            FillerComponentC,
            FillerComponentD )
    };

    container.reg.new_entity().with_component(SimpleComponent(7.7)).with_component(FillerComponentA(1));
    
    container.do_query();

    
}


#[test]
fn test_removing_components() {
    let mut registry = build_registry!(
        SimpleComponent, 
        ComplexComponent,
        FillerComponentA,
        FillerComponentB,
        FillerComponentC,
        FillerComponentD
    );

    let my_entity = registry.new_entity()
        .with_component(SimpleComponent(8.8))
        .save_id();
    
    registry.new_entity()
        .with_component(SimpleComponent(7.7))
        .save_id();
    registry.new_entity()
        .with_component(SimpleComponent(7.7))
        .save_id();
    registry.new_entity()
        .with_component(SimpleComponent(7.7))
        .save_id();
    registry.new_entity()
        .with_component(SimpleComponent(7.7))
        .save_id();
    registry.new_entity()
        .with_component(SimpleComponent(7.7))
        .save_id();
          

    {
        let query = query_entity!(registry[my_entity]: SimpleComponent);
        assert!(query.is_none() != true);
    }

    {
        let got_deleted = registry.delete_entity(my_entity);
        assert!(got_deleted); //Should delete

        let query = query_entity!(registry[my_entity]: SimpleComponent);
        assert!(query.is_none());
    }

    {
        let query = query_components!(registry => SimpleComponent);
        assert!(query.len() == 5);

        for simple in query {
            assert!((*simple.0).0 == 7.7);
        }
    }
}
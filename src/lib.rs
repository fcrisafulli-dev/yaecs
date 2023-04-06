
pub mod ecs;

pub use ecs::ComponentRegistry;
// use std::cell::RefMut;
// use ecs::ComponentRegistry;


#[derive(Debug)]
struct UVec2(u32,u32);

#[derive(Debug)]
struct Hp(f32);

#[derive(Debug)]
struct Light(f32);

#[derive(Debug)]
struct Pos(f64);


#[inline]
pub fn benchmark_4wide_query(){
    let mut r = build_registry!(UVec2, Hp, Light, Pos);
    
    r.new_entity()
        .with_component(Pos(1.1))
        .with_component(Light(1.1))
        .with_component(UVec2(1,1))
        .with_component(Hp(1.1));
    r.new_entity()
        .with_component(Hp(2.2));
    r.new_entity()
        .with_component(UVec2(3,3));

    for _ in 0..50 {
        r.new_entity()
        .with_component(Hp(4.4))
        .with_component(Pos(4.4))
        .with_component(Light(4.4))
        .with_component(UVec2(4,4));
    }
    

    {
        let qu = query_components!(r => Hp, UVec2, Pos, Light);
        for (mut a,mut b,mut c,mut d) in qu {
            (*a).0 = 99.99;
            (*b).0 = 99;
            (*c).0 = 99.1234;
            (*d).0 = 99.4312;
        }
    }
}
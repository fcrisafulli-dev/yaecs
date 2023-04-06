use std::cell::RefMut;

use crate::ecs::ComponentRegistry;

mod ecs;

#[derive(Debug)]
struct UVec2(u32,u32);

#[derive(Debug)]
struct Hp(f32);

#[derive(Debug)]
struct Light(f32);

#[derive(Debug)]
struct Pos(f64);

fn main() {
    println!("Hello, world!");

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
    r.new_entity()
        .with_component(Hp(4.4))
        .with_component(Pos(4.4))
        .with_component(Light(4.4))
        .with_component(UVec2(4,4));

    
    dbg!(&r);
    
    // {
    //     let last_hp_added = r._peek_last_added_trait::<Hp>();
    //     let h_ref = last_hp_added.borrow();
    //     let h_val = h_ref.downcast_ref::<Hp>().unwrap();
    //     dbg!(h_val);
    // }

    // {
    //     let q_mask = query_mask!(r: Hp);
    //     dbg!(&q_mask);

    //     let idxs = r.mask_filter(q_mask);
    //     dbg!(&idxs);

    //     let request = r.id_filter::<Hp>(idxs);

    //     dbg!(request);
    // }
    
    {
        
        let q_mask = query_mask!(r: Hp, UVec2, Pos, Light);
        dbg!(&q_mask);

        let idxs = r.mask_filter(q_mask);
        dbg!(&idxs);

        let mut hp_request = r.id_filter::<Hp>(&idxs).into_iter();
        let mut vec2_request = r.id_filter::<UVec2>(&idxs).into_iter();
        let mut pos_request = r.id_filter::<Pos>(&idxs).into_iter();
        let mut light_request = r.id_filter::<Light>(&idxs).into_iter();
        

        let mut flat_zip: Vec<(
            RefMut<Hp>,
            RefMut<UVec2>,
            RefMut<Pos>,
            RefMut<Light>,
        )> = Vec::with_capacity(idxs.len());


        for _ in idxs {
            flat_zip.push((
                hp_request.next().unwrap(),
                vec2_request.next().unwrap(),
                pos_request.next().unwrap(),
                light_request.next().unwrap(),
            ))
        }

        for (mut hp, mut uv, mut p, mut l) in flat_zip {
            (*hp).0 = 99.999;
        }

        // let z = hp_request.into_iter()
        //     .zip(vec2_request)
        //     .zip(light_request)
        //     .zip(pos_request);

        // for (((mut hp, uvec), mut light), mut v2) in z {
        //     (*hp).0 = 99.999;
        // }
    }
    
    {
        let q_mask = query_mask!(r: Hp, UVec2);
        dbg!(&q_mask);

        let idxs = r.mask_filter(q_mask);
        dbg!(&idxs);

        let request = r.id_filter::<Hp>(&idxs);
        dbg!(request);
    }

    {
        let qu = query_components!(r => Hp, UVec2);
        dbg!(qu);
    }
}

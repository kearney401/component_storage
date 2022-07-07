use component_storage::ComponentStorage;

#[derive(Debug)]
struct Position(f32, f32);

#[derive(Debug)]
struct Velocity(f32, f32);

fn main() {
    println!("Hello World");

    let mut cs = ComponentStorage::default();

    let pos = Position(1280.0, 720.0);
    println!("{:?}", pos);

    let vel = Velocity(0.0, 0.0);
    println!("{:?}", vel);

    cs.add_component(0, pos);
    cs.add_component(0, vel);

    cs.add_component(1, Position(100.0, 100.0));
    cs.add_component(2, Position(200.0, 200.0));
    cs.add_component(3, Position(300.0, 300.0));
    cs.add_component(4, Position(400.0, 400.0));
    cs.add_component(10, Position(0.0, 0.0));

    let pos_from_cs = cs.get_component::<Position>(0).unwrap();
    let vel_from_cs = cs.get_component::<Velocity>(0).unwrap();
    println!("Entity: 0 - {:?} {:?}", pos_from_cs, vel_from_cs);

    let pos_iter = cs.get_components_iter::<Position>().unwrap();
    let pos_iter_clone = pos_iter.clone();

    println!("---------Position Check---------");
    for (index, pos) in pos_iter.enumerate() { 
        println!("Entity: {} - {:?}", index, pos);
    } 

    println!("---------Zip Check---------");
    let vel_iter = cs.get_components_iter::<Velocity>().unwrap();
    let zip_iter = pos_iter_clone.zip(vel_iter);

    for (index, zip) in zip_iter.enumerate() { 
        println!("Entity: {} - {:?}", index, zip);
    }
    
    println!("Goodbye World");
}

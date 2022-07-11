use component_storage::ComponentStorage;

#[derive(Debug)]
struct Position(f32, f32);

#[derive(Debug)]
struct Velocity(f32, f32);

fn main() {
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

    println!("---------Position Check---------");
    if let Some(positions) = cs.get_components::<Position>() { 
        for (index, pos) in positions.iter().enumerate() { 
            println!("Entity: {} - {:?}", index, pos);

        }
    }

    println!("---------Zip Check w/Entities---------");
    let positions = cs.get_components::<Position>().unwrap();
    let velocities = cs.get_components::<Velocity>().unwrap();
    let zip = positions.iter().zip(velocities.iter());

    for (index, zip) in zip.enumerate() { 
        println!("Index: {} - {:?}", index, zip);
    }

    println!("---------Zip Check unwrapped----------");
    let positions = cs.get_components_unwrapped::<Position>().unwrap();
    let velocities = cs.get_components_unwrapped::<Velocity>().unwrap();
    let zip = positions.iter().zip(velocities.iter());

    for (index, zip) in zip.enumerate() { 
        println!("Index: {} - {:?}", index, zip);
    }


}

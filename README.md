Component Storage
=======

Basic and simple storage system for an ECS. Struct are tighly packed into vectors.

Example
-------
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

        let pos = cs.get_components::<Position>().unwrap();

        println!("---------Position Check---------");
        for (index, pos) in pos.iter().enumerate() { 
            println!("Entity: {} - {:?}", index, pos);
        } 

        println!("---------Zip Check---------");
        let zip_iter = pos.iter().zip(cs.get_components::<Velocity>().unwrap().iter());

        for (index, zip) in zip_iter.enumerate() { 
            println!("Entity: {} - {:?}", index, zip);
        }
    }


Output
------
    Position(1280.0, 720.0)
    Velocity(0.0, 0.0)
    Entity: 0 - Position(1280.0, 720.0) Velocity(0.0, 0.0)
    ---------Position Check---------
    Entity: 0 - Some(Position(1280.0, 720.0))
    Entity: 1 - Some(Position(100.0, 100.0))
    Entity: 2 - Some(Position(200.0, 200.0))
    Entity: 3 - Some(Position(300.0, 300.0))
    Entity: 4 - Some(Position(400.0, 400.0))
    Entity: 5 - None
    Entity: 6 - None
    Entity: 7 - None
    Entity: 8 - None
    Entity: 9 - None
    Entity: 10 - Some(Position(0.0, 0.0))
    ---------Zip Check---------
    Entity: 0 - (Some(Position(1280.0, 720.0)), Some(Velocity(0.0, 0.0)))

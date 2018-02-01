mod vectoralgebra;
mod gravity;
mod swarm;

use vectoralgebra::cartesian_vector::CartesianVector;
use vectoralgebra::point::Point;
use swarm::GlobalConstants;
use swarm::EntityType::Sheep;
use swarm::EntityType::DeadSheep;
use swarm::EntityType::Wolf;
use swarm::EntityType::Wall;

#[macro_use]
extern crate maplit;

fn main() {
    assert_eq!(127, i8::max_value());
    println!("Hello world");

    let v = CartesianVector { x: 3.0, y: 4.0 };
    assert_eq!(5.0, v.magnitude());

    {
        let ps = vec![Point::new(2.0, 3.0), Point::new(4.0, 5.0)];
        let weight = Point::weight_point(ps);

        assert_eq!(3.0, weight.x);
        assert_eq!(4.0, weight.y);
    }

    {
        gravity::gravity_vector(&Point::new(2.0, 3.0), &Point::new(4.0, 5.0), 9.0, 1.0);
    }

    let global_constants = GlobalConstants {
        minimum_proximity: 1.0,
        gravity_constants: hashmap! {
            Sheep => hashmap!{
                    Sheep => 1.0,
                    DeadSheep => 0.0,
                    Wolf => -2.0,
                    Wall => -1.0,
            },
            Wolf => hashmap!{
                    Sheep => 2.0,
                    DeadSheep => 0.0,
                    Wolf => 1.0,
                    Wall => -1.0
            }

        },
    };
}


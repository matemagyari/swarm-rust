use swarm::EntityType;
use swarm::Entity;
use swarm::RandomGenerator;
use vectoralgebra::point::Point;
use swarm::IdGenerator;
use swarm::GlobalConstants;
use swarm::StrayTendency;

pub fn create_entities(
    num: i32,
    global_constants: &GlobalConstants,
    stray_tendency: &StrayTendency,
    speed: f64, //todo could be a map
    entity_type: EntityType,
    board_dimensions: (i32, i32),
    id_generator: &IdGenerator,
    random_generator: &RandomGenerator) -> Vec<&'static Entity> {
    let coordinate = |dimension: i32| {
        random_generator.next() * dimension as f64
    };

    let position = || {
        Point { x: coordinate(board_dimensions.0), y: coordinate(board_dimensions.1) }
    };


    (0..num).map(|i| {
        Entity {
            id: id_generator.next(),
            position: position(),
            speed: speed,
            stray_tendency: *stray_tendency,
            constant: 1.0, //todo - what's this?
            entity_type: entity_type,
            gravity_map: global_constants.gravity_constants[entity_type],
        }
    });

//    fn position() -> Point {
//        Point { x: coordinate(board_dimensions.0), y: coordinate(board_dimensions.1) };
//    }

    vec![]
}
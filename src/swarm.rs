use vectoralgebra::point::Point;
use std::collections::HashMap;
use gravity;
use vectoralgebra::cartesian_vector::CartesianVector;

extern crate rand;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum EntityType {
    DeadSheep,
    Sheep,
    Wolf, //todo - wolf has a kill_range attribute. it will change the structs a bit
    Wall,
}

#[derive(Copy, Clone)]
pub struct StrayTendency {
    //defines the angle the entity can deviate from the gravitational vector
    random_max: f64,
    constant: f64,
}

#[derive(Clone)]
pub struct Entity {
    pub id: i32,
    pub position: Point,
    pub speed: f64,
    //defines the angle the entity can deviate from the gravitational vector
    pub stray_tendency: StrayTendency,
    pub constant: f64,
    //todo - what's this?
    pub entity_type: EntityType,
    pub gravity_map: HashMap<EntityType, f64>,
}

pub struct GlobalConstants {
    pub gravity_constants: HashMap<EntityType, HashMap<EntityType, f64>>,
    pub minimum_proximity: f64,
}

//Random double in [-max-deviaton +max-deviaton]
pub fn deviation(stray_tendency: &StrayTendency, rand_num: f64) -> f64 {
    let r_max = stray_tendency.random_max;
    (2.0 * r_max * rand_num) - r_max + stray_tendency.constant
}

//Calculation of the next position based on the entities around and a random element
pub fn next_position(global_constants: &GlobalConstants, entity: &Entity, entitites: Vec<Entity>, rand_num: f64) -> Point {
    let speed = entity.speed;
    let position = entity.position;
    if speed > 0.0 { //don't bother with entities with zero speed

        let adjusted_g_vector: CartesianVector = {
            let total_g_vector = gravity::sum_gravity_vector(entity, entitites, global_constants);
            let rotation = deviation(&entity.stray_tendency, rand_num);
            total_g_vector.rotate(rotation)
        };
        adjusted_g_vector.normalize().multiply(speed).add_to_point(&position)
    } else {
        position
    }
}

pub trait RandomGenerator {
    fn next(&self) -> f64 {
        rand::random::<f64>()
    }
}

pub trait IdGenerator {
    fn next(&self) -> i32;
}

//Calculate the next positions of the entities
pub fn next_positions(
    global_constants: &GlobalConstants,
    entities: Vec<Entity>,
    random_generator: &RandomGenerator) -> Vec<Point> {

    entities.iter().map(|entity| {
        let others: Vec<Entity> = entities.iter().filter(|e| e.id != entity.id).map(|e| e.clone()).collect();
        let random_number = random_generator.next();
        next_position(global_constants, entity, others, random_number)
    }).collect()
}

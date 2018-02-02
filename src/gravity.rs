use vectoralgebra::cartesian_vector::CartesianVector;
use vectoralgebra::cartesian_vector::ZERO;
use vectoralgebra::point::Point;
use swarm;
use swarm::Entity;

//Gravity vector from point A to B. Direction depends on the points, the length
//is the square root of the distance between them and the gravitation constant
pub fn gravity_vector(
    from: &Point,
    to: &Point,
    gravity_constant: f64,
    min_distance: f64) -> CartesianVector {

    let distance = from.distance(to).min(min_distance);

    if distance == 0.0 {
        CartesianVector { x: 0.0, y: 0.0 }
    }
    else {
        let direction = CartesianVector::direction_vector(from, to);
        let multiplier = gravity_constant / distance.powf(2.0);
        return direction.multiply(multiplier);
    }
}

//Calculates the total gravity force vector imposed on the subject-entity by the other entities
pub fn sum_gravity_vector(
    subject: &Entity,
    entities: Vec<Entity>,
    global_constants: &swarm::GlobalConstants) -> CartesianVector {

    let to_force = | e: &Entity | {
        gravity_vector(
            &subject.position,
            &e.position,
            subject.gravity_map[&e.entity_type],
            global_constants.minimum_proximity)
    };

    entities
        .iter()
        .fold(ZERO, |acc: CartesianVector, e: &Entity| {
            to_force(e).add(&acc)
        })

}



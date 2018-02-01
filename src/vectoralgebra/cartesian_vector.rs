use vectoralgebra::polar_vector::PolarVector;
use vectoralgebra::point::Point;

#[derive(Copy, Clone)]
pub struct CartesianVector {
    pub x: f64,
    pub y: f64,
}

pub const ZERO: CartesianVector = CartesianVector { x: 0.0, y: 0.0};

impl CartesianVector {
    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn add(&self, other: &CartesianVector) -> CartesianVector {
        CartesianVector { x: self.x + other.x, y: self.y + other.y }
    }

    pub fn addToPoint(&self, other: &Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }

    pub fn multiply(&self, scalar: f64) -> CartesianVector {
        CartesianVector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn to_polar_vector(&self) -> PolarVector {
        PolarVector {
            angle: self.x.atan2(self.y),
            magnitude: self.magnitude(),
        }
    }

    pub fn rotate(&self, angle: f64) -> CartesianVector {
        self.to_polar_vector().rotate(angle).to_cartesian_vector()
    }

    pub fn normalize(&self) -> CartesianVector {
        let len = self.magnitude();
        if 0.0 == len {
            //todo - how should it work instead?
            CartesianVector { x: self.x, y: self.y }
        } else {
            let div_len = |x: f64| x / len;
            CartesianVector { x: div_len(self.x), y: div_len(self.y) }
        }
    }

    pub fn direction_vector(from: &Point, to: &Point) -> CartesianVector {
        CartesianVector { x: to.x - from.x, y: to.y - from.y }.normalize()
    }

}
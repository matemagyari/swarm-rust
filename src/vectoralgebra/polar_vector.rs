use vectoralgebra::cartesian_vector::CartesianVector;

#[derive(Copy, Clone)]
pub struct PolarVector {
    pub angle: f64,
    pub magnitude: f64,
}

impl PolarVector {
    pub fn rotate(&self, angle: f64) -> PolarVector {
        PolarVector {
            angle: self.angle + angle,
            magnitude: self.magnitude
        }
    }
    pub fn to_cartesian_vector(&self) -> CartesianVector {
        CartesianVector {
            x: self.angle.cos() * self.magnitude,
            y: self.angle.sin() * self.magnitude,
        }
    }
}

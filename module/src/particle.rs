use std::fmt;
use crate::force::Force; // ルートからforceモジュールを辿る

// 構造体もpub, フィールドもpub
pub struct Particle {
    pub mass: f64,
    pub position: f64,
    pub velocity: f64,
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Mass: {:.1} P: {:.2} V: {:.2}]", self.mass, self.position, self.velocity)
    }
}

impl Particle {
    pub fn new(mass: f64, position: f64) -> Particle {
        Particle {
            mass,
            position,
            velocity:0.0,
        }
    }

    pub fn apply(&mut self, force_type: Force, dt: f64) {
        match force_type {
            Force::Gravity => self.velocity += -9.8 * dt,
            Force::Impulse(mag) => self.velocity += mag / self.mass,
            Force::Drag(c) => self.velocity -= c * self.velocity * dt / self.mass,
        }
    }

    pub fn update_position(&mut self, dt: f64) {
        self.position += self.velocity * dt;
    }
}

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

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*; // Particle, Force を使えるようにする

    #[test]
    fn test_new_particle() {
        let p = Particle::new(2.0, 10.0);
        assert_eq!(p.mass, 2.0);
        assert_eq!(p.position, 10.0);
        assert_eq!(p.velocity, 0.0);
    }

    #[test]
    fn test_gravity_application() {
        let mut p = Particle::new(1.0, 0.0);
        let dt = 1.0;
        
        // 重力を1秒間適用
        p.apply(Force::Gravity, dt);
        
        // v = v0 + a*dt = 0 + (-9.8)*1 = -9.8
        // 浮動小数は完全一致しないことがあるため，誤差を許容するか，
        // あるいは今回のように単純な計算なら一致を期待する
        assert_eq!(p.velocity, -9.8);
    }

    #[test]
    fn test_impulse_application() {
        let mut p = Particle::new(2.0, 0.0); // mass = 2.0
        
        // 撃力 10.0 を加える
        // dv = F / m = 10.0 / 2.0 = 5.0
        p.apply(Force::Impulse(10.0), 0.1); // dtは無視されるはず
        
        assert_eq!(p.velocity, 5.0);
    }
}

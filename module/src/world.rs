use crate::particle::Particle

pub struct World {
    particles: Vec<Particle>,
}

pub impl World {
    pub fn new() -> World {
        World {
            particles: Vec::new(),
        }
    }

    pub fn add(&mut self, particle: Particle) {
        self.particles.push(particle);
    }
    
    pub fn update(&mut self, dt: f64) {
        for p in &mut self.particles {
            p.apply(Force::Gravity, dt);
            p.apply(Force::Drag(0.1), dt); // 空気抵抗も常に入れておく
            p.update_position(dt);
        }
    }

    pub fn remove_fallen(&mut self) {
        self.particles.retain(|p| p.position >= 0);
    }

    pub fn show_status(&self) {
        for(i, p) in self.particles.iter().enumerate() {
            println!("Particle {}: {}", i, p);
        }
    }
}

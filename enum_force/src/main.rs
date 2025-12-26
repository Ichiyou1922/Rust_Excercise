struct Particle {
    mass: f64,
    position: f64,
    velocity: f64,
}

enum Force {
    Gravity,
    Impulse(f64),
    Drag(f64),
}

impl Particle {
    // コンストラクタ（関連関数）
    fn new(mass: f64, position: f64) -> Particle {
        Particle {
            mass,
            position,
            velocity: 0.0,
        }
    }

    // 力を加えて速度を更新するメソッド（書き換え: &mut self）
    // F = ma -> a = F/m -> v = v + a*dt
    fn apply(&mut self, force_type: Force, dt: f64) {
        match force_type{
            Force::Gravity => { self.velocity += -9.8 * dt },
            Force::Impulse(mag) => { self.velocity += mag / self.mass },
            Force::Drag(c) => { self.velocity -= c * self.velocity * dt / self.mass }
        }
    }

    fn update_position(&mut self, dt: f64) {
        self.position += self.velocity * dt;
    }
}

fn main() {
    let dt = 0.1;
    let mut t = 0.0;
    let mut p = Particle::new(10.0, 0.0);

    while t <= 10.0 {
        p.apply(Force::Gravity, dt);
        p.apply(Force::Drag(0.1), dt);
        if 4.9 < t && t < 5.1 { p.apply(Force::Impulse(20.0), dt) };
        p.update_position(dt);
        println!("t = {:.1}, P = {:.4}, V = {:.4}", t, p.position, p.velocity);
        t += dt;
    }
}

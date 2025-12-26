struct World {
    // 粒子の所有権を持つベクタ
    particles: Vec<Particle>,
}

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

impl World {
    fn new() -> World {
        World {
            particles: Vec::new(), // 空のベクタを作成
        }
    }

    fn add(&mut self, particle: Particle) {
        self.particles.push(particle); // 所有権がベクタ内に移動(Move)
    }

    fn update(&mut self, dt: f64) {
        // ベクタ内の全要素に対して可変参照を得て反復処理
        for p in &mut self.particles {
            p.apply(Force::Gravity, dt);
            p.update_position(dt);
        }
    }

    // y < 0.0 になった粒子（地面に落ちたもの）を削除する
    fn remove_fallen(&mut self) {
        // retainのクロージャ内で true を返せば残り，falseなら削除される
        self.particles.retain(|p| p.position >= 0.0);
        // |p| ... はクロージャ(無名関数)．ここでは「述語」として機能する
    }
}

fn main() {
    let dt = 0.1;

    let mut world = World::new();
    for i in 0..5 {
        world.add(Particle::new(1.0, 10.0 + i as f64 * 5.0));
    }

    while true {
        world.update(dt);
        world.remove_fallen();
        println!("Survive Particle: {}", world.particles.len());
        if world.particles.len() == 0 {
            println!("Simulation Finished");
            break;
        }
    }
}

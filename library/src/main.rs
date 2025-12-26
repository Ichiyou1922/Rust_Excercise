use rand::Rng; // 乱数生成用
use std::fmt;  // 表示フォーマット用

// --- 構造体定義 ---
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

struct World {
    particles: Vec<Particle>,
}

// --- Displayトレイトの実装 (演習3-1) ---
// これにより println!("{}", p) が可能になる
impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write! マクロでフォーマッタに書き込む
        write!(f, "[Mass: {:.1}] P: {:.2}, V: {:.2}", self.mass, self.position, self.velocity)
    }
}

// --- Particleの実装 ---
impl Particle {
    fn new(mass: f64, position: f64) -> Particle {
        Particle {
            mass,
            position,
            velocity: 0.0,
        }
    }

    fn apply(&mut self, force_type: Force, dt: f64) {
        match force_type {
            Force::Gravity => self.velocity += -9.8 * dt,
            Force::Impulse(mag) => self.velocity += mag / self.mass,
            Force::Drag(c) => self.velocity -= c * self.velocity * dt / self.mass,
        }
    }

    fn update_position(&mut self, dt: f64) {
        self.position += self.velocity * dt;
    }
}

// --- Worldの実装 ---
impl World {
    fn new() -> World {
        World {
            particles: Vec::new(),
        }
    }

    fn add(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    fn update(&mut self, dt: f64) {
        for p in &mut self.particles {
            p.apply(Force::Gravity, dt);
            p.apply(Force::Drag(0.1), dt); // 空気抵抗も常に入れておく
            p.update_position(dt);
        }
    }

    fn remove_fallen(&mut self) {
        // クロージャ: 位置が0以上のものだけを「保持(retain)」する
        self.particles.retain(|p| p.position >= 0.0);
    }

    // (演習3-1追加) 全粒子の状態を表示
    fn show_status(&self) {
        // iter() で不変参照を取り出し，enumerate() で番号を振る
        for (i, p) in self.particles.iter().enumerate() {
            println!("Particle {}: {}", i, p);
        }
    }
}

fn main() {
    let dt = 0.1;
    let mut world = World::new();
    let mut rng = rand::thread_rng();

    // ランダムな粒子を10個生成
    println!("--- Generating Particles ---");
    for _ in 0..10 {
        let m = rng.gen_range(0.5..2.0);  // 質量ランダム
        let y = rng.gen_range(5.0..50.0); // 高さランダム
        world.add(Particle::new(m, y));
    }

    // シミュレーションループ
    println!("--- Start Simulation ---");
    let mut step = 0;
    while world.particles.len() > 0 {
        world.update(dt);
        world.remove_fallen();

        // ログが多すぎるので，10ステップに1回だけ詳細表示
        if step % 10 == 0 {
            println!("Step {}: Alive = {}", step, world.particles.len());
            world.show_status(); // ここで実装したメソッドを使用
            println!("---------------------------");
        }
        
        step += 1;
    }
    println!("All particles have fallen. Simulation Finished.");
}

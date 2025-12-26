struct Particle {
    mass: f64,
    position: f64,
    velocity: f64,
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

    // 運動エネルギーを計算するメソッド（読み取りのみ: &self）
    // E = 0.5 * m * v^2
    fn kinetic_energy(&self) -> f64 {
        0.5 * self.mass * self.velocity * self.velocity
    }

    // 力を加えて速度を更新するメソッド（書き換え: &mut self）
    // F = ma -> a = F/m -> v = v + a*dt
    fn apply_force(&mut self, force: f64, dt: f64) {
        let a = force / self.mass;
        self.velocity += a * dt;
    }

    fn update_position(&mut self, dt: f64) {
        self.position += self.velocity * dt;
    }
}
fn main() {
    let dt = 0.1;
    let force = 10.0;

    let mut p = Particle::new(10.0, 0.0);

    for i in 0..5 {
        p.apply_force(force, dt);
        p.update_position(dt);
        let time = i as f64 * dt;
        println!("T = {:.1}, P = {:.4}, V = {:.4} E = {:.4}", time, p.position, p.velocity, p.kinetic_energy());
    }

}

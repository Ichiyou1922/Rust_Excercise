mod force;
mod particle;
mod world;

use rand::Rng;
use particle::Particle;
use world::World;

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
    while world.len() > 0 {
        world.update(dt);
        world.remove_fallen();

        // ログが多すぎるので，10ステップに1回だけ詳細表示
        if step % 10 == 0 {
            println!("Step {}: Alive = {}", step, world.len());
            world.show_status(); // ここで実装したメソッドを使用
            println!("---------------------------");
        }
        
        step += 1;
    }
    println!("All particles have fallen. Simulation Finished.");
}


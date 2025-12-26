fn main() {
    let g = 9.8;
    let dt = 0.1;
    let mut y = 100.0;
    let mut v = 0.0;
    let mut t = 0.0;

    println!("Start Simulation: y_0 = {}", y);

    loop {
        t += dt;
        v = v - g * dt;
        y = y + v * dt;

        // 衝突判定
        // 地面にめり込んだら "CRASH", まだなら"FALLING" をstatusに束縛
        let status = if y < 0.0 {
            "CRASH"
        } else {
            "FALLING"
        };

        println!("t: {:.2}, y: {:.2}, status: {}", t, y, status);

        // status を見てloopを抜ける
        if status == "CRASH" {
            println!("Touchdown at t = {:.2}", t);
            break;
        }
    }
}

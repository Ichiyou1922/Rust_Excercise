fn main() {
    // x_0 = 0.0
    // 変更が必要なため，mut キーワードで宣言
    let mut position = 0.0;

    // v = 2.0 (const)
    let velocity = 2.0;

    // 時間刻み dt = 1.0;
    let dt = 1.0;

    println!("Time: 0, Position: {}", position);

    // 1ステップ後の更新位置: x_{t+1} = x_t + v * dt;
    position = position + velocity * dt;

    println!("Time: 1, Position: {}", position);
}

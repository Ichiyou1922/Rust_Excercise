fn main() {
    println!("Hello, world!");

    let x = 10;
    println!("The value of x is: {}", x);

    // 以下の行を解除するとコンパイルエラーになる．
    // x = 20;
    // これがRustのImmutableの強制
}

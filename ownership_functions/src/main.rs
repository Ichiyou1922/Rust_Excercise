fn main() {
    let s = String::from("Physics"); // sが所有権を持つ

    take_ownership(s); // sの所有権が関数に移動する
    
    // ここでsは使えないことに注意
    // println!("{}",s)<-これはエラー

    let x = 5;
    make_copy(x); // xの値がコピーされて関数に渡される

    println!("x is still valid: {}", x);
}

fn take_ownership(some_string: String) {
    println!("I took ownership of: {}", some_string);
    // ここでsome_stringがスコープを抜け，メモリが解放(Drop)される
}

fn make_copy(some_integer: i32) {
    println!("I got a copy of: {}", some_integer);
}

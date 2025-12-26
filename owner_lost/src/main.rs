fn main() {
    // heap領域にメモリを確保(String型)
    let s1 = String::from("Rust");

    // 所有権の移動 (Move)
    // s1 が指していたヒープメモリの管理責任は s2 に移る
    let s2 = s1.clone();

    // Error
    println!("s1 says: {}, s2 says: {}", s1, s2);

    // s2はここでスコープを抜け，メモリは解放(Drop)される．
}

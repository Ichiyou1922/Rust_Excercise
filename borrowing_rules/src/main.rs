fn main() {
    let mut s = String::from("Hello");

    // 不変の借用(読者1)
    let r1 = &s;
    // 不変の借用(読者2)
    let r2 = &s;

    // ここまでOK
    // println!("{} and {}", r1, r2);
    println!("{} and {}", *r1, *r2); // *による参照外し

    // 可変の借用（書き手）を作成
    // ここでエラー！ r1, r2（読者）の生存期間と被る
    let r3 = &mut s;

    // r3を使って書き換え,というか追加
    r3.push_str(", world");

    // ここでr1, r2 を使おうとすると，書き換え中のデータを読むことになる
    // println!("{} and {}", r1, r2);
    println!("{}",r3);
}

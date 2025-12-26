fn main() {
    let mut s1 = String::from("Rust");

    // Give Reference (Borrowing)
    // 所有権は移動しない->戻り値を受け取る必要がない
    let len = caluculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 可変参照を渡して内容を変更させる
    add_suffix(&mut s1);

    println!("Modified string: {}", s1);
}

fn caluculate_length(s: &String) -> usize {
    s.len() // 式から値を返す->;をつけずにブロックの戻り値とする．
}

fn add_suffix(s: &mut String) {
    s.push_str(" is powerful"); // ユニット型()を返す->戻り値のない関数add_suffixに合わせ;をつけて文にして評価を捨てている．
}

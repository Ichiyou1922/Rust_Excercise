use proconio::input;

fn main() {
    // AtCoderでは入出力処理とロジックを分けるとテストがしやすくなります。
    // ここでは標準入力からの読み込みをシミュレーションします。
    // 実際の提出時は main 関数内の proconio::input! を使います。
    
    println!("Run `cargo test` to check your implementation!");
}

// ---------------------------------------------------------
// 練習問題 1: 文字列のパースとOption
// ---------------------------------------------------------

/// 文字列 `s` を受け取り、整数 `i32` に変換して返す関数を実装してください。
/// 変換に失敗した場合（数字以外の文字が含まれる等）は `None` を返してください。
/// 
/// ヒント:
/// - `str::parse::<i32>()` メソッドを使います。
/// - `parse` は `Result` を返しますが、`ok()` メソッドで `Option` に変換できます。
pub fn parse_string_to_int(s: &str) -> Option<i32> {
    todo!("Implement parse_string_to_int")
}

// ---------------------------------------------------------
// 練習問題 2: 基本的な計算とResult
// ---------------------------------------------------------

/// 2つの整数 `a`, `b` を受け取り、 `a / b` の結果を返してください。
/// ただし、 `b` が 0 の場合は "Division by zero" というエラーメッセージを持つ `Err` を返してください。
/// 
/// ヒント:
/// - 条件分岐 (`if`) を使います。
/// - 成功時は `Ok(結果)`、失敗時は `Err("エラーメッセージ".to_string())` を返します。
pub fn safe_division(a: i32, b: i32) -> Result<i32, String> {
    todo!("Implement safe_division")
}

// ---------------------------------------------------------
// 練習問題 3: 基本的な入出力 (proconio の練習)
// ---------------------------------------------------------

/// 半径 `r` が与えられるので、円の面積を求めてください。
/// 円周率には `std::f64::consts::PI` を使用してください。
pub fn circle_area(r: f64) -> f64 {
    todo!("Implement circle_area")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string_to_int() {
        assert_eq!(parse_string_to_int("42"), Some(42));
        assert_eq!(parse_string_to_int("-10"), Some(-10));
        assert_eq!(parse_string_to_int("abc"), None);
        assert_eq!(parse_string_to_int("12.5"), None);
        assert_eq!(parse_string_to_int(""), None);
    }

    #[test]
    fn test_safe_division() {
        assert_eq!(safe_division(10, 2), Ok(5));
        assert_eq!(safe_division(9, 3), Ok(3));
        assert_eq!(safe_division(5, 0), Err("Division by zero".to_string()));
    }

    #[test]
    fn test_circle_area() {
        let r = 2.0;
        let area = circle_area(r);
        // 誤差を許容して比較
        assert!((area - 12.566370614359172).abs() < 1e-9);
    }
}

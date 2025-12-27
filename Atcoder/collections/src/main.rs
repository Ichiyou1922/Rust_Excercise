use std::collections::{HashMap, HashSet};

fn main() {
    println!("Run `cargo test` to check your implementation!");
}

// ---------------------------------------------------------
// 練習問題 1: Vectorの操作
// ---------------------------------------------------------

/// 整数のリスト `nums` が与えられます。
/// 偶数のみを残し、それぞれの値を2倍にした新しいベクタを返してください。
/// 
/// ヒント:
/// - `iter()`, `filter()`, `map()`, `collect()` のチェーンを使うのがRustらしい書き方です。
/// - または `for` ループで `Vec::new()` に `push` しても構いません。
pub fn filter_and_double_evens(nums: &[i32]) -> Vec<i32> {
    todo!("Implement filter_and_double_evens")
}

// ---------------------------------------------------------
// 練習問題 2: HashMapによるカウント
// ---------------------------------------------------------

/// 文字列のリスト `words` が与えられます。
/// 各単語が何回出現したかをカウントし、HashMapとして返してください。
/// 
/// ヒント:
/// - `HashMap::new()` で作成します。
/// - `entry(key).or_insert(0)` パターンが非常に便利です。
pub fn count_words(words: &[&str]) -> HashMap<String, i32> {
    todo!("Implement count_words")
}

// ---------------------------------------------------------
// 練習問題 3: HashSetによる重複排除
// ---------------------------------------------------------

/// 整数のリスト `nums` が与えられます。
/// 重複を取り除いた要素の数を返してください。
/// 
/// ヒント:
/// - `HashSet` にすべての要素を挿入し、その `len()` を返します。
/// - `nums.iter().collect::<HashSet<_>>()` も使えます。
pub fn count_unique_numbers(nums: &[i32]) -> usize {
    todo!("Implement count_unique_numbers")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_and_double_evens() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let res = filter_and_double_evens(&nums);
        assert_eq!(res, vec![4, 8, 12]);
        
        let empty: Vec<i32> = vec![];
        assert_eq!(filter_and_double_evens(&empty), vec![]);
    }

    #[test]
    fn test_count_words() {
        let words = vec!["apple", "banana", "apple", "orange", "banana", "apple"];
        let counts = count_words(&words);
        
        assert_eq!(counts.get("apple"), Some(&3));
        assert_eq!(counts.get("banana"), Some(&2));
        assert_eq!(counts.get("orange"), Some(&1));
        assert_eq!(counts.get("grape"), None);
    }

    #[test]
    fn test_count_unique_numbers() {
        let nums = vec![1, 1, 2, 3, 4, 4, 5];
        assert_eq!(count_unique_numbers(&nums), 5); // 1, 2, 3, 4, 5
        
        let nums2 = vec![1, 1, 1];
        assert_eq!(count_unique_numbers(&nums2), 1);
    }
}

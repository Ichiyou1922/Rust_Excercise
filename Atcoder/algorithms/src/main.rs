use std::cmp::Ordering;

fn main() {
    println!("Run `cargo test` to check your implementation!");
}

// ---------------------------------------------------------
// 練習問題 1: カスタムソート
// ---------------------------------------------------------

/// 名前(String)とスコア(i32)を持つ構造体 `Player` があります。
/// これを以下のルールでソートしてください。
/// 1. スコアが高い順 (降順)
/// 2. スコアが同じなら名前の辞書順 (昇順)
/// 
/// ヒント:
/// - `sort_by` メソッドを使います。
/// - `b.score.cmp(&a.score)` で降順ソートになります。
/// - `then_with` を使うと第2条件を指定できます。
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Player {
    pub name: String,
    pub score: i32,
}

pub fn sort_players(players: &mut [Player]) {
    todo!("Implement sort_players")
}

// ---------------------------------------------------------
// 練習問題 2: 累積和 (Iterator::scan / fold)
// ---------------------------------------------------------

/// 整数のリスト `nums` が与えられます。
/// その累積和（Cumulative Sum）のリストを返してください。
/// 例: [1, 2, 3] -> [1, 3, 6]
/// 
/// ヒント:
/// - 手続き的に `for` ループで書いても良いです。
/// - Iteratorの `scan` メソッドを使うとかっこよく書けます。
pub fn cumulative_sum(nums: &[i32]) -> Vec<i32> {
    todo!("Implement cumulative_sum")
}

// ---------------------------------------------------------
// 練習問題 3: 二分探索 (Binary Search)
// ---------------------------------------------------------

/// ソート済みのリスト `sorted_nums` と ターゲット `target` が与えられます。
/// `target` がリストに含まれている場合、そのインデックスを `Some(index)` で返してください。
/// 含まれていない場合は `None` を返してください。
/// 
/// ヒント:
/// - `Vec::binary_search` を使います。
/// - `binary_search` は `Result<usize, usize>` を返します。 `Ok` なら見つかったということです。
pub fn find_index(sorted_nums: &[i32], target: i32) -> Option<usize> {
    todo!("Implement find_index")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_players() {
        let mut players = vec![
            Player { name: "Alice".to_string(), score: 100 },
            Player { name: "Bob".to_string(), score: 80 },
            Player { name: "Charlie".to_string(), score: 100 },
            Player { name: "Dave".to_string(), score: 90 },
        ];

        sort_players(&mut players);

        // Expected order:
        // 1. Alice (100)
        // 2. Charlie (100) - Same score, alphabetical
        // 3. Dave (90)
        // 4. Bob (80)
        assert_eq!(players[0].name, "Alice");
        assert_eq!(players[1].name, "Charlie");
        assert_eq!(players[2].name, "Dave");
        assert_eq!(players[3].name, "Bob");
    }

    #[test]
    fn test_cumulative_sum() {
        let nums = vec![1, 2, 3, 4, 5];
        let sums = cumulative_sum(&nums);
        assert_eq!(sums, vec![1, 3, 6, 10, 15]);

        let empty: Vec<i32> = vec![];
        assert_eq!(cumulative_sum(&empty), vec![]);
    }

    #[test]
    fn test_find_index() {
        let nums = vec![1, 3, 5, 7, 9];
        assert_eq!(find_index(&nums, 5), Some(2));
        assert_eq!(find_index(&nums, 1), Some(0));
        assert_eq!(find_index(&nums, 4), None);
    }
}

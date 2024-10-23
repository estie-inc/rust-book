#![allow(clippy::all, unused)]

fn main() {
    println!("Hello, world!");
}

fn solve(s: Vec<char>) -> bool {
    let n = s.len();

    // まだ対応する ) が見つかっていない ( の数を left とする。最初は left=0 である
    let mut left = 0;

    // 文字列を左から1文字ずつ見る
    for i in 0..n {
        if s[i] == '(' {
            // 見た文字が ( のとき、left に 1 を足す
            left += 1;
        } else {
            // 見た文字が ) のとき、left が 0 であれば文字列は正しいカッコ列ではない
            if left == 0 {
                return false;
            } else {
                // left が 0 より大きいとき、left から 1 を引く
                left -= 1;
            }
        }
    }

    // すべての文字を見終わったとき、left が 0 であれば文字列は正しいカッコ列である
    if left == 0 {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let ans = solve(vec!['(', ')', '(', ')', '(', ')', '(', ')', '(', ')']);
        assert_eq!(ans, true);
    }
}

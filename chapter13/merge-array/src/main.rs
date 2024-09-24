#![allow(clippy::all, unused)]

fn main() {
    println!("Hello, world!");
}

fn solve1(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    // a と b の要素数をそれぞれ取得しておく
    let n = a.len();
    let m = b.len();

    // ans という空の Vec を作る
    let mut ans = vec![];

    // a の要素をすべて ans に入れる
    for i in 0..n {
        ans.push(a[i]);
    }

    // b の要素をすべて ans に入れる
    for i in 0..m {
        ans.push(b[i]);
    }

    // ans の要素を小さい順に並べ替える
    ans.sort();
    ans
}

fn solve2(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    // a と b の要素数をそれぞれ取得しておく
    let n = a.len();
    let m = b.len();

    // ans という空の Vec を作る
    let mut ans = vec![];

    // a の先頭の要素を指す値を i として定義する
    // i に 1 を足すことで、a の先頭の要素を削除する操作に代える
    // i=n のとき、a が空になったとみなすことができる
    let mut i = 0;

    // 同様に、b の先頭の要素を指す値を j として定義する
    let mut j = 0;

    // a と b の両方が空になるまで、次の処理を繰り返す
    while i < n || j < m {
        if i == n {
            // a が空のとき、b の先頭の要素を ans に追加する
            ans.push(b[j]);

            // 追加した b の先頭の要素を削除する
            j += 1;
        } else if j == m {
            // b が空のとき、a の先頭の要素を ans に追加する
            ans.push(a[i]);

            // 追加した a の先頭の要素を削除する
            i += 1;
        } else if a[i] < b[j] {
            // a の先頭の要素が b の先頭の要素よりも小さいとき、
            // a の先頭の要素を ans に追加する
            ans.push(a[i]);

            // 追加した a の先頭の要素を削除する
            i += 1;
        } else {
            // b の先頭の要素が a の先頭の要素よりも小さい、または等しいとき、
            // b の先頭の要素を ans に追加する
            ans.push(b[j]);

            // 追加した b の先頭の要素を削除する
            j += 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let ans = solve1(vec![1, 2, 4, 5, 8], vec![2, 3, 4, 5, 6, 7]);
        assert_eq!(ans, vec![1, 2, 2, 3, 4, 4, 5, 5, 6, 7, 8]);
    }

    #[test]
    fn test_solve2() {
        let ans = solve2(vec![1, 2, 4, 5, 8], vec![2, 3, 4, 5, 6, 7]);
        assert_eq!(ans, vec![1, 2, 2, 3, 4, 4, 5, 5, 6, 7, 8]);
    }
}

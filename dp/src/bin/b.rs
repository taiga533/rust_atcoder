use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    input! {
        h: [i64; n],
    }
    let mut dp = vec![std::i64::MAX ;n];
    dp[0] = 0;

    for i in 0..n-1 {
        for j in 1..=k {
            if n >= j && i < n - j {
                dp[i+j] = std::cmp::min(dp[i+j], dp[i] + (h[i] - h[i+j]).abs());
            };
        }
    }

    println!("{}", dp[n-1]);
}

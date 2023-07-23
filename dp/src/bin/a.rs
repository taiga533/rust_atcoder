use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    input! {
        h: [i64; n],
    }
    let mut dp = vec![std::i64::MAX ;n];
    dp[0] = 0;

    for i in 0..n-1 {
        
        if i < n - 2 {
            dp[i+2] = std::cmp::min(dp[i+2], dp[i] + (h[i] - h[i+2]).abs());
        };
        
        if i < n - 1 {
            dp[i+1] = std::cmp::min(dp[i+1], dp[i] + (h[i] - h[i+1]).abs());
        };
    }

    println!("{}", dp[n-1]);
}

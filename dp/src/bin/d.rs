use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in 0..n {
        let (weight, value) = wv[i];
        for j in 0..w+1 {
            if j >= weight {
                dp[i+1][j] = std::cmp::max(dp[i][j-weight] + value, dp[i][j]);
            } else {
                dp[i+1][j] = dp[i][j];
            }
        }
    }
    println!("{:?}", dp[n][w]);
}

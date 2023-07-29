use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let s = s.chars().collect::<Vec<char>>();
    let mut dp = vec![vec![0 as usize;s.len()+1];s.len()+1];
    dp[0][0] = 1;
    for i in 0..s.len() {
        for j in 0..s.len() {
            if s[i] != ')' {
                dp[i + 1][j + 1] += dp[i][j];
                dp[i + 1][j + 1] %= 998244353;
            }
            if j != 0 && s[i] != '(' {
                dp[i + 1][j - 1] += dp[i][j];
                dp[i + 1][j - 1] %= 998244353;
            }
        }
    }
    println!("{}", dp[s.len()][0]);
}

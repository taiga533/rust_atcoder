use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    let atcoder = "atcoder";
    let mut dp = vec![vec![0; s.len()+1]; atcoder.chars().count()+1];

    dp[0] = dp[0].iter().map(|_| 1).collect();

    let chars = atcoder.chars().collect::<Vec<char>>();

    for i in 1..=atcoder.chars().count() {
        // println!("{:?}", i);
        for j in 1..=s.len() {
            if s[j-1] == chars[i-1] {
                dp[i][j] = (dp[i-1][j-1] + dp[i][j-1]) % 1_000_000_007;
            }
            else {
                dp[i][j] = dp[i][j-1];
            }
        }
    }

    println!("{:?}", dp[dp.len()-1][dp[0].len()-1]);
}

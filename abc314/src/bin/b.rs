use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut bets = vec![];
    for _ in 0..n {
        input!{
            c: usize,
            bet: [usize;c],
        }
        bets.push(bet);
    }
    input!{
        x: usize
    }
    let mut min_betted = 37;
    let mut hits = vec![];
    for (idx, bet) in bets.iter().enumerate() {
        let hit = bet.iter().find(|&&b| b == x);
        if hit.is_some() {
            if bet.len() <= min_betted {
                min_betted = bet.len();
            }
            hits.push((idx, bet.len()));
        }
    }

    let a: Vec<_> = hits.iter()
            .filter(|&&hit| hit.1 == min_betted)
            .map(|x| x.0 + 1).collect();
    
    println!("{}", a.len());
    println!("{}", a.iter().join(" "));
}

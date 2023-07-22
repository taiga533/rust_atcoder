use proconio::{fastout, input, marker::Chars};


fn main() {
    input! {
        n: usize,
    }
    input! {
        s: Chars,
    }

    let a = s.iter().position(|&c| c == 'A').unwrap_or(n+1);
    let b = s.iter().position(|&c| c == 'B').unwrap_or(n+1);
    let c = s.iter().position(|&c| c == 'C').unwrap_or(n+1);

    let max = std::cmp::max(a, std::cmp::max(b, c));
    println!("{}", max+1);
}

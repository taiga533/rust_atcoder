use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let target = p[0];

    let max_p = p.iter().enumerate().max_by_key(|&(_, &x)| x).unwrap();
    if max_p.0 == 0 {
        println!("0");
        return;
    }
    println!("{}", max_p.1 - target+1);
}

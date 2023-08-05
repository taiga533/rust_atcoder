use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut max = 0;
    let mut min = std::usize::MAX;
    let mut sum = 0;

    for &value in &a {
        max = max.max(value);
        min = min.min(value);
        sum += value;
    }

    let avg = sum / n as usize;
    let mut op = 0;

    if max - min > 1 {
        let mut excess = 0;
        let mut shortage = 0;

        for &value in &a {
            if value > avg + 1 {
                excess += value - (avg + 1);
            } else if value < avg {
                shortage += avg - value;
            }
        }

        op = std::cmp::max(excess, shortage);
    }

    println!("{}", op);
}

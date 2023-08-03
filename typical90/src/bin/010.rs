use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [(usize, usize); n],
        q: usize,
        b: [(usize, usize); q],
    }
    // 累積和
    let first_sum = a.iter().scan(0_usize, |cum, (c, score)| {
        if c == &1 {
            *cum += score;
        }
        Some(*cum)
    }).collect::<Vec<_>>();
    let second_sum = a.iter().scan(0_usize, |cum, (c, score)| {
        if c == &2 {
            *cum += score;
        }
        Some(*cum)
    }).collect::<Vec<_>>();

    // println!("{:?}", first_sum);
    // println!("{:?}", second_sum);

    for i in 0..q {
        let (from, to) = b[i];
        let ans = [&first_sum, &second_sum].iter().map(|sum| {
            let before_sum = if from == 1 {0_usize} else {sum[from-2]};
            sum[to-1] - before_sum
        }).collect::<Vec<_>>();
        println!("{:?} {:?}", ans[0], ans[1]);
    }
    
}

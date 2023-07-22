use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
    }
    let mut schedules: Vec<Vec<usize>> = vec![];
    for _ in 0..n {
        input! {
            mut s: Chars
        }
        let mut vac = 0;
        s.reverse();
        let mut a = s.iter().map(|c| -> usize {
            if *c == 'o' {
                vac += 1;
            } else {
                vac = 0;
            }
            vac
        }).collect::<Vec<usize>>();
        a.reverse();
        schedules.push(a);
    }

    // println!("{:?}", schedules);
    let mut max = 0;
    for i in 0..d {
        let mut matched = 0;
        let mut min = d;
        for j in 0..n {
            min = std::cmp::min(min, schedules[j][i]);
            matched += if schedules[j][i] > 0 { 1 } else { 0 };
        }
        if min > max && matched == n {
            max = min;
        }
    }
    println!("{:?}", max);
    // println!("{:?}", max_index + max);


}

use itertools::{Itertools, iproduct};
use proconio::{fastout, input};


#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let pattern_list = (0..n).map(|_| "()".chars()).multi_cartesian_product();

    let a = pattern_list.filter(|p| {
        let mut count = 0;
        for &c in p {
            if count < 0 {
                return false;
            }
            if c == '(' {
                count += 1;
            } else {
                count -= 1;
            }
        }
        count == 0
    }).map(|chars| {
        chars.iter().collect::<String>()
    }).join("\n");
    if a != "" {
        println!("{}", a);
    }
    // println!("{:?}");
    
}

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    println!("{:?}", search(0 as usize, (10_u64.pow(9) + (1 as u64) )as usize, &a, &b));
}

fn is_seller_ok(a: &Vec<usize>, b: &Vec<usize>, x: usize) -> bool {
    let seller_count = a.iter().filter(|&y| x >= *y).count();
    let buyer_count = b.iter().filter(|&y| x <= *y).count();
    // println!("{} {} {}", x, seller_count, buyer_count);
    if seller_count >= buyer_count {
        true
    } else {
        false
    }
}

fn search(left: usize, right: usize, a: &Vec<usize>, b: &Vec<usize>) -> usize {
    let mid = (left + right) / 2;
    let is_seller_ok = is_seller_ok(a, b, mid as usize);
    // println!("{} {} {}", left, mid, right);
    if right - left <= 1 {
        return right;
    }
    if is_seller_ok {
        search(left, mid, a, b)
    } else {
        search(mid, right, a, b)
    }
}

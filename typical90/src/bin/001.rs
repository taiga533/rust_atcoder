use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: isize,
        l: isize,
        k: isize,
        mut a: [isize;n]
    };

    a.insert(0, 0);
    a.push(l);

    
    println!("{}", search(0, l+1, &a, k));

}

fn search(left: isize, right: isize, a: &Vec<isize>, k: isize) -> isize {
    let mid = (left + right) / 2;
    let div = dividable(a, mid);
    if right - left <= 1 {
        return left;
    }
    if div > k {
        search(mid, right, a, k)
    } else {
        search(left, mid, a, k)
    }
}

fn dividable(a: &Vec<isize>, min_score: isize) -> isize {
    let mut begin = 0;
    let mut end = 0;
    let mut count = 0;
    // println!("{:?}", a);
    while begin < a.len() {
        // println!("{} {} {}", begin, end, a[end] - a[begin]);
        if a[end] - a[begin] >= min_score {
            count += 1;
            begin = end;
            continue;
        }
        if end == a.len() - 1 {
            break;
        }
        end += 1;
    }
    count
}
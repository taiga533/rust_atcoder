use proconio::{fastout, input};
use superslice::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut classes: [usize;n],
        wait_students: usize,
        mut students: [usize;wait_students]
    }

    classes.sort();

    for student in students {
        let min_over_idx = classes.lower_bound(&student);
        if min_over_idx == n {
            println!("{}", student - classes[n-1]);
            continue;
        }
        let min_over = classes[min_over_idx] as isize;
        let student = student as isize;
        if min_over_idx == 0 {
            println!("{}", min_over - student);
            continue;
        }
        let max_under = classes[min_over_idx-1]as isize;

        if min_over - student < student - max_under {
            println!("{}", min_over - student);
        } else {
            println!("{}", student - max_under);
        }

    }
}
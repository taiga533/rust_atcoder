use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    let row_sum = a.iter().map(|row| {
        row.iter().sum()
    }).collect::<Vec<usize>>();
    let col_sum = (0..w).map(|i| {
        (0..h).map(|j| a[j][i]).sum::<usize>()
    }).collect::<Vec<usize>>();

    for i in 0..h {
        for j in 0..w {
            print!("{}", row_sum[i] + col_sum[j] - a[i][j]);
            if j != w - 1 {
                print!(" ");
            }
        }
        println!();
    }
}

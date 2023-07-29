use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
    }
    let mut ans_list: Vec<(usize,usize)> = Vec::new();

    for i in 0..=n-9 {
        for j in 0..=m-9 {
            let t1 = &s[i][j..j+4];
            let t2 = &s[i+1][j..j+4];
            let t3 = &s[i+2][j..j+4];
            let t4 = &s[i+3][j..j+4];
            let d = i+5;
            let d1 = &s[d][j+5..j+9];
            let d2 = &s[d+1][j+5..j+9];
            let d3 = &s[d+2][j+5..j+9];
            let d4 = &s[d+3][j+5..j+9];
            // println!("{}:{}, {} {} {} {} {} {} {} {}",i, j, t1, t2, t3, t4, d1, d2, d3, d4);
            if t1 == "###."
                && t2 == "###."
                && t3 == "###."
                && t4 == "...."
                && d1 == "...."
                && d2 == ".###"
                && d3 == ".###"
                && d4 == ".###" {
                ans_list.push((i+1,j+1));
            }
        }
    }
    for ans in ans_list.iter() {
        println!("{} {}", ans.0, ans.1);
    }
    // println!("{:?}", ans_list);
}

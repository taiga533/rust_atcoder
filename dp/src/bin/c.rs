use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        days: usize,
        points: [[usize; 3];days],
    }
    
    let mut dp = vec![[0;3];days];
    for day_index in 0..days {
        let last_day = if day_index == 0 { [0, 0, 0] } else { dp[day_index - 1] };
        for today_action_index in 0..3 {
            for prev_action_index in 0..3 {
                if today_action_index == prev_action_index { continue; }
                let today_point = points[day_index][today_action_index] + last_day[prev_action_index];
                dp[day_index][today_action_index] = std::cmp::max(dp[day_index][today_action_index], today_point);
            }
        }
    }
    println!("{:?}", dp[days-1].iter().max().unwrap());
}
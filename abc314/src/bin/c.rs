use proconio::{fastout, input, marker::Chars};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        _: usize,
        s_chars: Chars,
        c: [usize; n],
    }


    let mut positions = HashMap::new();

    for (i, &color) in c.iter().enumerate() {
        positions.entry(color).or_insert_with(Vec::new).push(i);
    }

    let mut result = vec![' '; n];

    for (_, indices) in &positions {
        let len = indices.len();
        for (i, &pos) in indices.iter().enumerate() {
            let new_pos = indices[(i + 1) % len];
            result[new_pos] = s_chars[pos];
        }
    }

    println!("{}", result.iter().collect::<String>());
}

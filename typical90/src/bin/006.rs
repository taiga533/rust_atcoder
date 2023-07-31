use std::collections::HashMap;

use maplit::{hashmap, hashset};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: isize,
        s: Chars
    }
    let mut ans = vec![];
    let alphabets = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut alphabetNex: Vec<HashMap<&char, isize>> = vec![hashmap![];n+1];

    for (idx, c) in s.iter().enumerate().rev() {
        for _ in alphabets.iter() {
            alphabetNex[idx] = alphabetNex[idx+1].clone();
    
            alphabetNex[idx].entry(c).and_modify(|old_idx| {*old_idx = idx as isize}).or_insert(idx as isize);
        }
    }
    let mut last_idx: isize = -1;
    for i in 0..k {
        for alphabet in alphabets.iter() {
            if let Some(nex_idx) = alphabetNex[if last_idx == -1 {0} else {last_idx as usize +1}].get(alphabet) {
                if n as isize - nex_idx >= (k - i as isize) {
                    ans.push(alphabet);
                    last_idx = *nex_idx;
                    break;
                }
            }
        }
    }
    println!("{}", ans.into_iter().collect::<String>());


}

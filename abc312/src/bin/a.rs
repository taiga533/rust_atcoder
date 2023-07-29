use proconio::{fastout, input};


fn main() {
    input! {
        s: String
    };
    let t = vec![ "ACE","BDF","CEG","DFA","EGB","FAC","GBD"];
    let mut yesFlag = false;
    t.iter().for_each(|&x| {
        if x.contains(&s) {
            yesFlag = true;
            println!("Yes");
            return;
        }
    });
    if yesFlag == false {
        println!("No");
    }

}

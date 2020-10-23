use rand::{thread_rng, Rng};
use std::char;

fn main() {
    let mut words: Vec<char> = vec![];
    for i in 0x4E00..0x9FA5 {
        let c = char::from_u32(i);
        words.push(c.unwrap())
    }
    let len = words.len();
    println!("len {}", words.len());
    let word_len = 3;

    let mut rng = thread_rng();
    let name: String = (0..word_len)
        .map(move |_| {
            let idx = rng.gen_range(0, len);
            words[idx] as char
        })
        .collect();
    println!("{}", name);
}

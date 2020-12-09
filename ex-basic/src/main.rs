use std::fs::File;
use std::io::prelude::*;

fn main() {
    // t_string();

    let c = t_file().unwrap();
    println!("{:?}", c);
}

fn t_string() {
    let s = String::from("hello world");
    println!("s.starts_with() {}", s.starts_with("h"));

    // 字符串slice需要引用
    println!("s[2..5] {}", &s[2..5]);
}

fn t_file() -> std::io::Result<String> {
    let mut file = File::open("t.rs")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // assert_eq!(contents, "Hello, world!");
    Ok(contents)
}

// Careful, this example is not quite right.
use std::sync::{Arc, Mutex};
use std::thread;


fn test3() {
    let a: String =  String::from("aaa");
    let b = String::from("bbb");
    let c = "ccc".to_string();
    let mut d =  String::new();
    d.push_str("ddd");
    
    println!("{}",a);
    println!("{}",b);
    println!("{}",c);
    println!("{}",d);

    for i in a.chars() {
        println!("{}", i);
    }
    println!("{}", i)
}

fn test2() {

    let mut a = "aaa";
    a = "bbb";
    let c = "ccc";
    a = c;
    let mut d = c;

    println!("{:p}", c);
    println!("{:p}", d);
    d = "ddd";
    println!("{}", c);
    println!("{:p}", c);
    println!("{:p}", d);

    // // const a_con:str; // 
    // const a_con: str = "agc";
    // // a_con = "agc";
    // println!("{}", a_con);
}

fn test1() {
    let numbers = Arc::new(Mutex::new(vec![1, 2, 3]));

    for i in 0..3 {
        let number = numbers.clone();

        let _ = thread::spawn(move || {
            let mut array = number.lock().unwrap();

            array[i] += 1;

            println!("numbers[{}] is {}", i, array[i]);
        });
    }
    println!("{:#?}", numbers);
}

fn word() {
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

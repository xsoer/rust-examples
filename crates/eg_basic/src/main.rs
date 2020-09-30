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

use regex::{Regex, RegexSet};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    t_rs();
}

fn t_init() {
    println!("Hello, world!");
    let re = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
    let before = "2012-03-14, 2013-01-01 and 2014-07-05";
    let after = re.replace_all(before, "$m/$d/$y");
    assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");

    let re = Regex::new("[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap();
    let mat = re.find("phone: 111-222-3333").unwrap();
    // println!("{}", mat.text());
    assert_eq!((mat.start(), mat.end()), (7, 19));
    // println!("mat text{}", &mat.text[mat.start()..mat.end()]);
    println!("{}", mat.as_str());
}

fn t_rs() {
    let contents = t_file().unwrap();

    //匹配函数
    // let res = [r"use.*?(?=;)", r"fn.*?(?={)", r"pub fn.*?(?={)"];
    let res = [
        r"use.*?;",
        r"fn.*?\{",
        r"pub fn.*?\{",
        r"struct.*?\{",
        r"pub struct.*?(;|\{)",
        r"impl.*?\{",
        r"pub impl.*?\{",
        r"enum.*?;"
    ];
    let contents = contents.replace("\n", "");

    for pattern in res.iter() {
        let re = Regex::new(pattern).unwrap();
        for mat in re.find_iter(&contents) {
            let p = mat.as_str().to_string();
            println!("{:#?}", p);
        }
    }
}

fn t_file() -> std::io::Result<String> {
    let mut file = File::open("t.rs")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // assert_eq!(contents, "Hello, world!");
    Ok(contents)
}

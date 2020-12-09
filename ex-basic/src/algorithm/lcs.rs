// 最长公共子序列
// 给定两个字符串，求出它们之间最长的相同子字符串的长度。

fn lcs(l: String, r: String) -> usize {
    let l_len = l.len();
    let r_len = r.len();
    if l_len == 0 || r_len == 0 {
        return 0;
    }
    let dp = vec![vec![0; l_len + 1]; r_len + 1];
    println!("{:#?}", dp);

    1
}

fn test_lcs() -> usize {
    let a = String::from("helloworldhahahaha");
    let b = String::from("hellwworldhahahahfasdf");

    let res = lcs(a, b);
    println!("{}", res);
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcs_test() {
        assert_eq!(test_lcs(), 1)
    }
}
// 最大子序列

fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    // your code
    if k > 0 && strarr.len() >= k {
        strarr.windows(k).fold(String::new(), |acc, win| {
            let s = win.concat();
            if s.len() > acc.len() { s } else { acc }
        })
    } else {
        String::new()
    }
}

fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
    assert_eq!(&longest_consec(strarr, k), exp)
}

#[test]
fn basics_longest_consec() {
    testing(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2, "abigailtheta");
    testing(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1,
            "oocccffuucccjjjkkkjyyyeehh");
    testing(vec![], 3, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 3, "ixoyx3452zzzzzzzzzzzz");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
    testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
}
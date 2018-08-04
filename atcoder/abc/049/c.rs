use std::io::*;
use std::iter::Iterator;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char) 
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn main() {
    let p = ["eraser", "erase", "dreamer", "dream"];
    if p.iter().fold(read::<String>(), |s, x| s.replace(x, "")).is_empty() { println!("YES"); } else { println!("NO") }
}

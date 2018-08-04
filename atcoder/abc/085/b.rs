use std::io::*;
use std::str::FromStr;
use std::collections::BTreeSet;

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
    let n:u32 = read();
    let d:BTreeSet<u32> = (0..n).map(|_| read()).collect();
    println!("{}", d.len());
}

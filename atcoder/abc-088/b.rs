use std::io::*;
use std::str::FromStr;
use std::vec::Vec;

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
    let mut a:Vec<u32> = (0..n).map(|_| read()).collect();
    a.sort();
    a.reverse();
    let mut alice:u32 = 0;
    let mut bob:u32 = 0;
    for (i, &v) in a.iter().enumerate() {
        if i%2 == 0 { alice += v } else { bob += v };
    }
    println!("{}", alice - bob);
}

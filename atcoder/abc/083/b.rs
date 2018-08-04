use std::io::*;
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
    let n:u32 = read();
    let a:u32 = read();
    let b:u32 = read();
    let ans:u32 = (1..n+1)
                    .filter(|x| { let sum = x.to_string().chars().map(|c| (c as u8 - b'0') as u32).sum::<u32>();
                                  a <= sum && sum <= b })
                    .sum::<u32>();
    println!("{}", ans);

}

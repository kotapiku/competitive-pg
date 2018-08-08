#![allow(unused)]
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdin, stdout, BufWriter, Write};
use std::iter::{FromIterator, Iterator};

trait Read {
    fn read(s: &str) -> Self;
}

fn readln<T: Read>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    T::read(buf.trim_right())
}
 
macro_rules! read_impl{
    ($($t:ty)*) => ($(
        impl Read for $t {
            fn read(s: &str) -> $t{
                s.parse().unwrap()
            }
        }
    )*)
}
read_impl! { usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 }
 
impl Read for String {
    fn read(s: &str) -> Self {
        s.to_string()
    }
}
 
impl Read for Vec<char> {
    fn read(s: &str) -> Self {
        s.chars().collect()
    }
}
 
impl<T: Read> Read for Vec<T> {
    fn read(s: &str) -> Self {
        s.split_whitespace().map(T::read).collect()
    }
}
 
impl<A: Read, B: Read> Read for (A, B) {
    fn read(s: &str) -> Self {
        let tokens: Vec<_> = s.split_whitespace().collect();
        (A::read(tokens[0]), B::read(tokens[1]))
    }
}
 
impl<A: Read, B: Read, C: Read> Read for (A, B, C) {
    fn read(s: &str) -> Self {
        let tokens: Vec<_> = s.split_whitespace().collect();
        (A::read(tokens[0]), B::read(tokens[1]), C::read(tokens[2]))
    }
}
 
impl<A: Read, B: Read, C: Read, D: Read> Read for (A, B, C, D) {
    fn read(s: &str) -> Self {
        let tokens: Vec<_> = s.split_whitespace().collect();
        (A::read(tokens[0]), B::read(tokens[1]), C::read(tokens[2]), D::read(tokens[3]))
    }
}
macro_rules! max{($x:expr)=>{$x};($x:expr,$($xs:tt)+)=>{max($x,max!($($xs)+))};}
macro_rules! min{($x:expr)=>{$x};($x:expr,$($xs:tt)+)=>{min($x,min!($($xs)+))};}

fn main() {
    let (a, b):(usize, usize) = readln();
    let va:Vec<u32> = readln();
    let vb:Vec<u32> = readln();

    let mut dp = vec!(vec!(0; b+1); a+1);   // dp[i][j] = aが下からi個，bが下からj個のときのすぬけの合計

    for i in 1..a+1 {
        if (a+b-i)%2 == 0 { dp[i][0] = dp[i-1][0] + va[a-i]; }
        else { dp[i][0] = dp[i-1][0]; }
    }
    for j in 1..b+1 {
        if (a+b-j)%2 == 0 { dp[0][j] = dp[0][j-1] + vb[b-j]; }
        else { dp[0][j] = dp[0][j-1]; }
    }

    for i in 1..a+1 {
        for j in 1..b+1 {
            if (a+b-i-j)%2 == 0 { dp[i][j] = max(dp[i-1][j] + va[a-i], dp[i][j-1] + vb[b-j]); }
            else { dp[i][j] = min(dp[i-1][j], dp[i][j-1]); }
        }
    }

    println!("{}", dp[a][b]);

}

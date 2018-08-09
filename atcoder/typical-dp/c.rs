#![allow(unused)]
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdin, stdout, BufWriter, Write};
use std::iter::{FromIterator, Iterator};

macro_rules! max{($x:expr)=>{$x};($x:expr,$($xs:tt)+)=>{max($x,max!($($xs)+))};}
macro_rules! min{($x:expr)=>{$x};($x:expr,$($xs:tt)+)=>{min($x,min!($($xs)+))};}
macro_rules! dump{
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}
 
const INF: i64 = 0x3f3f3f3f3f3f3f3f;
const MOD: i64 = 1000000007;

fn f(rp: f64, rq: f64) -> f64 {
    (1.0+10f64.powf((rp-rq)/400.0)).recip()
}

fn main() {
    let k:usize = readln();
    let mut vr:Vec<i32> = Vec::new();
    let n = 2usize.pow(k as u32);
    for _ in 0..n { vr.push(readln::<i32>()); }
    let mut dp = vec![vec![0.0; k+1]; n]; // dp[i][j] = iさんがjラウンドで勝つ確率

    for i in 0..n {
        dp[i][0] = 1.0;
    }

    for j in 1..k+1 {
        for i in 0..n {
            dp[i][j] = dp[i][j-1] * ((0..n).filter(|&p| ((p >> (j-1)) & 1) != ((i >> (j-1)) & 1) && (p >> j == (i >> j)))
                                           .map(|p| f(vr[p] as f64, vr[i] as f64)*dp[p][j-1])
                                           .sum::<f64>());
        }
    }

    for i in 0..n { println!("{}", dp[i][k]); }
}

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


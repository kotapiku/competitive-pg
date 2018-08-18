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
 
const MOD: i64 = 1000000007;

fn f(v:&Vec<Vec<u64>>, l:usize, r:usize) -> bool {
    if l == 0 { (0..20).map(|i| v[r][i]).all(|x| x <= 1) }
    else { (0..20).map(|i| v[r][i]-v[l-1][i]).all(|x| x <= 1) }
}

fn main() {
    let n:usize = readln();
    let va:Vec<u64> = readln();

    let mut v:Vec<Vec<u64>> = vec![vec![0;20];n];
    for i in 0..n {
        let mut x = va[i];
        let mut j = 0;
        while x != 0 {
            v[i][j] = x % 2;
            j += 1; x = x >> 1;
        }
    }

    for i in 1..n { for j in 0..20 { v[i][j] += v[i-1][j] } }

    let mut ans = 0;
    let mut r = 0;

    for l in 0..n {
        while r < n-1 && f(&v, l, r+1) {
            r += 1;
        }
        ans += r-l+1;
    }
    println!("{}", ans);
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

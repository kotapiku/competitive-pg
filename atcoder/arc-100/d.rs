#![allow(unused)]
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdin, stdout, BufWriter, Write};
use std::iter::{FromIterator, Iterator};

macro_rules! max{($x:expr)=>{$x};($x:expr,$($xs:tt)+)=>{max($x,max!($($xs)+))};}
macro_rules! min{($x:expr)=>{$x};($x:expr,$($xs:tt)+)=>{min($x,min!($($xs)+))};}
macro_rules! debug{
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}
 
const MOD: i64 = 1000000007;

fn g(va:&Vec<i64>, a:usize, b:usize, c:usize, n:usize) -> i64 {
    let p = va[a];
    let q = va[b]-va[a];
    let r = va[c]-va[b];
    let s = va[n-1]-va[c];
    max!(p, q, r, s)-min!(p, q, r, s)
}

fn main() {
    let n:usize = readln();
    let mut va:Vec<i64> = readln();
    for i in 1..n { va[i] += va[i-1]; }
    let mut f1:Vec<usize> = vec![0;n-2];
    let mut f2:Vec<usize> = vec![0;n-2];

    let mut l = 0;
    let mut r = 2;
    for i in 1..n-2 {
        while (va[i]-2*va[l+1]).abs() < (va[i]-2*va[l]) { l += 1; }
        while (va[i]+va[n-1]-2*va[r+1]).abs() < (va[i]+va[n-1]-2*va[r]).abs() { r += 1; }
        f1[i] = l;
        f2[i] = r;
    }

    if n == 4 { println!("{}", g(&va, 0, 1, 2, 4)); }
    else { println!("{}", (1..n-1).map(|a| g(&va, f1[a], a, f2[a], n)).min().unwrap()); }

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

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

fn main() {
    let s:Vec<char> = readln();
    let mut v:Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut v2:Vec<char> = v.clone();

    v2.reverse();

    if s == v2 { println!("-1"); return }

    for &i in &s {
        v.retain(|&a| a != i);
    }

    if v.len() == 0 {
        let mut num = 24;
        while s[num+1] < s[num] {
            num -= 1;
        }

        let (s1, s2) = s.split_at(num);
        let x = s2.to_vec().into_iter().filter(|&y| (y as u8) > (s[num] as u8)).min();

        println!("{}{}", s1.to_vec().into_iter().collect::<String>(), x.unwrap());

    } else {
        println!("{}{}", s.into_iter().collect::<String>(), v[0]); 
    }
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

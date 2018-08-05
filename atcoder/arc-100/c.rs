#![allow(unused)]
use std::cmp::{max, min, Ordering};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdin, stdout, BufWriter, Write};
use std::iter::{FromIterator, Iterator};
mod util {
    use std::fmt::Debug;
    use std::io::{stdin, stdout, BufWriter, StdoutLock};
    use std::str::FromStr;
    #[allow(dead_code)]
    pub fn line() -> String {
        let mut line: String = String::new();
        stdin().read_line(&mut line).unwrap();
        line.trim().to_string()
    }
    #[allow(dead_code)]
    pub fn chars() -> Vec<char> {
        line().chars().collect()
    }
    #[allow(dead_code)]
    pub fn gets<T: FromStr>() -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
    {
        let mut line: String = String::new();
        stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect()
    }
    #[allow(dead_code)]
    pub fn with_bufwriter<F: FnOnce(BufWriter<StdoutLock>) -> ()>(f: F) {
        let out = stdout();
        let writer = BufWriter::new(out.lock());
        f(writer)
    }
}

macro_rules! max{($x:expr)=>{$x};($x:expr,$($xs:tt)+)=>{max($x,max!($($xs)+))};}
macro_rules! min{($x:expr)=>{$x};($x:expr,$($xs:tt)+)=>{min($x,min!($($xs)+))};}
macro_rules! get{($t:ty)=>{{let mut line:String=String::new();stdin().read_line(&mut line).unwrap();line.trim().parse::<$t>().unwrap()}};($($t:ty),*)=>{{let mut line:String=String::new();stdin().read_line(&mut line).unwrap();let mut iter=line.split_whitespace();($(iter.next().unwrap().parse::<$t>().unwrap(),)*)}};($t:ty;$n:expr)=>{(0..$n).map(|_|get!($t)).collect::<Vec<_>>()};($($t:ty),*;$n:expr)=>{(0..$n).map(|_|get!($($t),*)).collect::<Vec<_>>()};($t:ty;;)=>{{let mut line:String=String::new();stdin().read_line(&mut line).unwrap();line.split_whitespace().map(|t|t.parse::<$t>().unwrap()).collect::<Vec<_>>()}};($t:ty;;$n:expr)=>{(0..$n).map(|_|get!($t;;)).collect::<Vec<_>>()};}
macro_rules! debug{($($a:expr),*)=>{eprintln!(concat!($(stringify!($a),"={:?},"),*),$($a),*);}}

fn f(la:&Vec<i64>, a:i64) -> i64 {
    la.iter().map(|&x| (x-a).abs()).sum::<i64>()
}

fn main() {
    let n = get!(usize);
    let la:Vec<i64> = util::gets::<i64>();
    let mut la2:Vec<i64> = la.iter().enumerate().map(|(i, x)| x-(i as i64)-1).collect();
    la2.sort();
    if n%2 != 0 { println!("{}", f(&la2, la2[(n-1)/2])); } 
    else {
        let a = la2[n/2-1];
        let b = la2[n/2+1];
        println!("{}", min(f(&la2, a), f(&la2, b)));
    }
}

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

fn main() {
    let n = get!(usize);
    let mut s:Vec<char> = get!(String).chars().collect();

    let mut e:Vec<u32> = Vec::with_capacity(n);
    let mut w:Vec<u32> = Vec::with_capacity(n);

    for i in 0..n { e.push(0); }
    for i in 0..n { w.push(0); }

    for (i, &v) in s[0..n-1].iter().enumerate() {
        if v == 'W' { e[i+1] = e[i] + 1; } else { e[i+1] = e[i]; }
    }

    w[n-1] = 0;
    s.reverse();

    for (i, &v) in s[0..n-1].iter().enumerate() {
        if v == 'E' { w[n-i-2] = w[n-i-1] + 1; } else { w[n-i-2] = w[n-i-1]; }
    }

    println!("{}", e.iter().zip(w).map(|(i, j)| i+j).min().unwrap());
}

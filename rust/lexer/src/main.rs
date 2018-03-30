extern crate regex;
use regex::{Regex, RegexSet};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::*;
use std::fs::*;
use std::io::*;

enum Token<T> {
    Ident(T),
    Num(T),
    Sym(T), // +, -, =, ...
}
enum TokenKind<T> {
    Ident(T),
    Num(T),
    Sym(T), // +, -, =, ...
}
// a = 42_000_i32 + 12;
// Ident("a") Eq Num("42_000","i32") Plus Num("12","") Semicolon
//
// Could use Option<Token>.

fn main() {
    if env::args().len() <= 1 {
        println!("Needs a file to parse!");
        process::exit(1);
    }
    let filename = env::args().nth(1).unwrap();
    let mut f = File::open(filename).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let ss = s.split_whitespace();

    let mut toks = Vec::new();

    for s in ss {
        let set = RegexSet::new(&[
            r"[a-z]",
            r"\d"
        ]).unwrap();
        let matches = set.matches(s);
        let v : Vec<_> = matches.into_iter().collect();
        if v.len() <= 0 {
            toks.push(Tok::Sym(s.clone()));
            continue;
        }
        match v[0] {
            0 => toks.push(Tok::Ident(s.clone())),
            1 => toks.push(Tok::Num(s.clone())),
            _ => println!("(ignored token : `{}')", s),
        }
    }
    for tok in toks {
        match tok {
            Tok::Ident(s) => println!("{} : word", s),
            Tok::Num(s) => println!("{} : digit", s),
            Tok::Sym(s) => println!("{} : sym", s),
        }
    }
}
/*
fn main() {
    let thread_count = 200;
    let data = Arc::new(Mutex::new(0));
    let (tx, rx) = mpsc::channel();
    for i in 0..thread_count {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            println!("I'm thread n°{}!", i);
            *data += i;
            tx.send(i).unwrap();
        });
    }
    for _ in 0..thread_count {
        let i = rx.recv().unwrap();
        println!("Thread n°{} has completed!", i);
    }
    let data = data.clone();
    println!("{}", *data.lock().unwrap());
}
*/

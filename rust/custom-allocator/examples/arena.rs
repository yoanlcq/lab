#![feature(try_from)]

extern crate edit_distance as edd;
use edd::edit_distance;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Hat { Pointy, Helmet }
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Body { Thin, Fat }
#[derive(Debug, Clone, Hash, PartialEq)]
struct Mob {
    hat: Vec<Hat>,
    body: Vec<Body>,
}

use std::convert::TryFrom;

#[derive(Debug, Clone, Hash, PartialEq)]
struct Suggestion {
    ident: &'static str,
    distance: i32,
}

impl<S: AsRef<str>> TryFrom<S> for Hat {
    type Error = Vec<Suggestion>;
    fn try_from(s: S) -> Result<Self, Self::Error> {
        match s.as_ref() {
            "Pointy" => Ok(Hat::Pointy),
            "Helmet" => Ok(Hat::Helmet),
            s @ _ => Err({
                let idents = &["Pointy", "Helmet"];
                let mut out = Vec::with_capacity(idents.len());
                for ident in idents {
                    let distance = edit_distance(s, ident);
                    out.push(Suggestion { ident, distance });
                }
                out.sort_by_key(|entry| entry.distance);
                out
            })
        }
    }
}

fn main() {
    use Hat::*;
    use Body::*;
    let mut mobs = Mob {
        hat: vec![Pointy, Helmet, Pointy, Pointy],
        body: vec![Fat, Fat, Thin, Thin],
    };
    let mut buf = String::new();
    loop {
        use std::io::*;
        buf.clear();
        println!("{:?}", mobs);
        println!("Change a hat!");
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut buf).unwrap();
        let input = buf.trim_right();
        match Hat::try_from(input) {
            Ok(new_hat) => mobs.hat[0] = new_hat,
            Err(suggestions) => {
                println!("Whoops! That's not a hat we know of.");
                println!("You might be interested in:");
                for s in suggestions {
                    println!("- {:?}", s);
                }
            }
        };
    }
}

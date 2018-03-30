pub mod rps {

    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Rps {
        Rock = 0,
        Paper = 1,
        Scissor = 2,
    }
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub enum Outcome {
        Defeat,
        Draw,
        Victory,
    }

    use self::Rps::*;
    use self::Outcome::*;

    impl Rps {
        pub fn vs(self, other: Self) -> Outcome {
            match (self, other) {
                (Rock, Scissor) => Victory,
                (Scissor, Rock) => Defeat,
                (s,o) => if s==o { Draw } else if s>o { Victory } else { Defeat },
            }
        }
    }
    impl ::std::str::FromStr for Rps {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, ()> {
            match s.to_owned().to_lowercase().as_str() {
                "r" | "rock" => Ok(Rock),
                "p" | "paper" => Ok(Paper),
                "s" | "scissor" => Ok(Scissor),
                _ => Err(()),
            }
        }
    }


    impl From<Rps> for &'static str {
        fn from(o : Rps) -> Self {
            match o {
                Rock => "Rock",
                Paper => "Paper",
                Scissor => "Scissor",
            }
        }
    }
    impl From<Outcome> for &'static str {
        fn from(o : Outcome) -> Self {
            match o {
                Victory => "Victory",
                Defeat => "Defeat",
                Draw => "Draw",
            }
        }
    }

    use std::fmt::{self, Display, Formatter};

    impl Display for Rps where Self: Into<&'static str> {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{}", <&'static str>::from(*self))
        }
    }
    impl Display for Outcome where Self: Into<&'static str> {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{}", <&'static str>::from(*self))
        }
    }

    #[cfg(test)]
    mod test {
        
        use super::*;
        
        #[test]
        fn rps_fight() {
            assert_eq!(Rock   .vs(Rock)   , Draw   );
            assert_eq!(Rock   .vs(Paper)  , Defeat );
            assert_eq!(Rock   .vs(Scissor), Victory);
            assert_eq!(Paper  .vs(Rock)   , Victory);
            assert_eq!(Paper  .vs(Paper)  , Draw   );
            assert_eq!(Paper  .vs(Scissor), Defeat );
            assert_eq!(Scissor.vs(Rock)   , Defeat );
            assert_eq!(Scissor.vs(Paper)  , Victory);
            assert_eq!(Scissor.vs(Scissor), Draw   );
        }
        #[test]
        fn rps_from_str() {
            assert_eq!(Rps::from_str("R"),       Ok(Rock));
            assert_eq!(Rps::from_str("r"),       Ok(Rock));
            assert_eq!(Rps::from_str("Rock"),    Ok(Rock));
            assert_eq!(Rps::from_str("rock"),    Ok(Rock));
            assert_eq!(Rps::from_str("P"),       Ok(Paper));
            assert_eq!(Rps::from_str("p"),       Ok(Paper));
            assert_eq!(Rps::from_str("Paper"),   Ok(Paper));
            assert_eq!(Rps::from_str("paper"),   Ok(Paper));
            assert_eq!(Rps::from_str("S"),       Ok(Scissor));
            assert_eq!(Rps::from_str("s"),       Ok(Scissor));
            assert_eq!(Rps::from_str("Scissor"), Ok(Scissor));
            assert_eq!(Rps::from_str("scissor"), Ok(Scissor));
            assert_eq!(Rps::from_str("roc"),     Err(()));
        }
    }
}

use std::str::FromStr;
use std::io::{Write, stdin, stderr};
use rps::*;
use Rps::*;

fn main() {
    let time = std::time::Instant::now();
    let mut buf = String::new();
    loop {
        println!("Your action ? (Rock | Paper | Scissor | Quit)");
        buf.clear();
        if let Err(e) = stdin().read_line(&mut buf) {
            let _ = write!(stderr(), "Error reading line: {}", e);
            continue;
        }
        let input = buf.trim_right();
        match input.to_lowercase().as_str() {
            "q" | "x" | "quit" | "exit" | "bye" | "kthxbye" => break,
            _ => (),
        }
        let action = match Rps::from_str(input) {
            Ok(a) => a,
            Err(_) => {
                println!("Please choose between Rock, Paper and Scissor.");
                continue;
            }
        };
        let ai = match time.elapsed().subsec_nanos() % 3 {
            0 => Rock,
            1 => Paper,
            2 => Scissor,
            _ => unreachable!()
        };
        println!("{} VS {}... {} !", action, ai, action.vs(ai));
    }
}

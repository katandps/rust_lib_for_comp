//! # じゃんけん

use crate::prelude::*;

#[codesnip::entry("rock-paper-scissors")]
pub use rps_impl::RPS;
#[codesnip::entry("rock-paper-scissors", include("prelude"))]
mod rps_impl {
    use super::{Display, Formatter};
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum RPS {
        Rock,
        Paper,
        Scissors,
    }

    impl RPS {
        pub fn battle(self, rhs: Self) -> RPS {
            use RPS::*;
            match (self, rhs) {
                (Rock, Rock) => Rock,
                (Rock, Paper) => Paper,
                (Rock, Scissors) => Rock,
                (Paper, Rock) => Paper,
                (Paper, Paper) => Paper,
                (Paper, Scissors) => Scissors,
                (Scissors, Rock) => Rock,
                (Scissors, Paper) => Scissors,
                (Scissors, Scissors) => Scissors,
            }
        }
    }

    impl Display for RPS {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let c: char = (*self).into();
            write!(f, "{}", c)
        }
    }

    impl From<char> for RPS {
        fn from(c: char) -> Self {
            use RPS::*;
            match c {
                'R' => Rock,
                'P' => Paper,
                'S' => Scissors,
                c => panic!("変換できません: {}", c),
            }
        }
    }

    impl From<&char> for RPS {
        fn from(c: &char) -> Self {
            Self::from(*c)
        }
    }

    impl From<RPS> for char {
        fn from(rps: RPS) -> Self {
            use RPS::*;
            match rps {
                Rock => 'R',
                Paper => 'P',
                Scissors => 'S',
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format() {
        let r = RPS::Rock;
        let p = RPS::Paper;
        let s = RPS::Scissors;

        assert_eq!("R", format!("{}", r));
        assert_eq!("P", format!("{}", p));
        assert_eq!("S", format!("{}", s));
    }

    #[test]
    fn test_battle() {
        let r = RPS::Rock;
        let p = RPS::Paper;
        let s = RPS::Scissors;

        assert_eq!(r, r.battle(r));
        assert_eq!(p, r.battle(p));
        assert_eq!(r, r.battle(s));
        assert_eq!(p, p.battle(r));
        assert_eq!(p, p.battle(p));
        assert_eq!(s, p.battle(s));
        assert_eq!(r, s.battle(r));
        assert_eq!(s, s.battle(p));
        assert_eq!(s, s.battle(s));
    }

    #[test]
    fn convert_rps_vec() {
        use RPS::*;
        let v = "RPSPPP".chars().map(|c| c.into()).collect::<Vec<RPS>>();
        assert_eq!(v, vec![Rock, Paper, Scissors, Paper, Paper, Paper]);
    }
}

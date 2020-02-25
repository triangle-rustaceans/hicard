
use std::cmp::{Ord,PartialOrd, Eq, PartialEq};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Spade = 4,
    Heart = 3,
    Diamond = 2,
    Club = 1
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn suit_order() {
        let suits = [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club];
        for s in 0..3 {
            assert_eq!(true, suits[s] > suits[s+1]);
        }
    }
}

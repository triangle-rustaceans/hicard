
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

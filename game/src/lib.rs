
// use std::cmp::{Ord,PartialOrd, Eq, PartialEq};
use std::collections::HashMap;
use uuid::Uuid;
use deckofcards::{Card,Deck};

pub struct Player {
    name : String,
    id : Uuid,
    card : Option<Card>
}

impl Player {
    fn new(name : &str) -> Player {
        Player { 
            name: name.to_owned(), 
            id: Uuid::new_v4(),
            card : None
        }
    }
}

pub struct Game {
    deck : Deck,
    players : HashMap<String,Player>
}

impl Game {
    pub fn new() -> Game { 
        Game { 
            deck : Deck::new(), 
            players : HashMap::new()
        }
    }
    pub fn join(&mut self, name : &str) -> &Player {
        self.players.entry(name.to_owned()).or_insert(Player::new(name))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn play_game() {
        let game = Game::new();
    }
}

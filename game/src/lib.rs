
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
    fn new(name : &str, id : Uuid) -> Player {
        Player { 
            name: name.to_owned(), 
            id: id,
            card : None
        }
    }
}

pub struct Game {
    deck : Deck,
    players : HashMap<Uuid,Player>,
    currentPlayer : i32,
    playOrder : Vec<Uuid>
}

impl Game {
    pub fn new() -> Game { 
        Game { 
            deck : Deck::new(), 
            players : HashMap::new(),
            currentPlayer: -1,
            playOrder: Vec::new()
        }
    }
    pub fn join(&mut self, name : &str) -> &Player {
        let id = Uuid::new_v4();
        let player = Player::new(name, id);
        self.players.insert(id, player);
        self.players.get(&id).unwrap()
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

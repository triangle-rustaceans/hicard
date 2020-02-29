
// use std::cmp::{Ord,PartialOrd, Eq, PartialEq};
use std::collections::HashMap;
use uuid::Uuid;
use deckofcards::{Card,Deck};

#[derive(Clone, Debug)]
pub struct Player {
    pub name : String,
    pub id : Uuid,
    pub card : Option<Card>
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
    current_player : i32,
    play_order : Vec<Uuid>
}

impl Game {
    pub fn new() -> Game {
        Game {
            deck : Deck::new(),
            players : HashMap::new(),
            current_player: -1,
            play_order: Vec::new()
        }
    }
    pub fn join(&mut self, name : &str) -> &Player {
        let id = Uuid::new_v4();
        let player = Player::new(name, id);
        self.players.insert(id, player);
        let p = self.players.get(&id).unwrap();
        self.play_order.push(p.id);
        p
    }
    // pub fn move(player: Player&) -> Option<Card> {
        // If player is the current player draw a card and advance the turn
    // }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn play_game() {
        let game = Game::new();
    }
}

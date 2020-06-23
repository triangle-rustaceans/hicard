
// use std::cmp::{Ord,PartialOrd, Eq, PartialEq};
use std::collections::HashMap;
use uuid::Uuid;
use deckofcards::{Card, Deck, Suit};

use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
    pub name : String,
    pub id : Uuid,
    #[serde(skip_deserializing, serialize_with="serialize_optcard")]
    pub card : Option<Card>,
}

fn serialize_optcard<S: Serializer>(card: &Option<Card>, s: S) -> Result<S::Ok, S::Error> {
    match card {
        Some(card) => {
            let rank = card.rank.to_char();
            let suit = match card.suit {
                Suit::Clubs => '♣',
                Suit::Diamonds => '♢',
                Suit::Hearts => '♡',
                Suit::Spades => '♠',
            };
            let val = format!("{}{}", rank, suit);
            s.serialize_some(&val)
        }
        None => s.serialize_none(),
    }
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
    use serde_json::to_string;

    #[test]
    fn play_game() {
        let game = Game::new();
        let player1 = game.join("Cliff");
        let player2 = game.join("tom");
        game.move(player2).expect_none("player 2 should not be able to play first");
        let p1card = game.move(player1).expect("player 1 should get a card");
        game.move(player1).expect_none("player 1 should not be able to get another card");
        let p2card = game.move(player2).expect("player 2 should get a card");
    }

    #[test]
    fn serialize_player() {
        let mut player = Player {
            name: String::from("Player One"),
            id: uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            card: None,
        };
        assert_eq!(
            &to_string(&player).unwrap(),
            r#"{"name":"Player One","id":"550e8400-e29b-41d4-a716-446655440000","card":null}"#,
        );
        player.card.replace(deckofcards::card!("QH"));
        assert_eq!(
            &to_string(&player).unwrap(),
            r#"{"name":"Player One","id":"550e8400-e29b-41d4-a716-446655440000","card":"Q♡"}"#,
        );
    }
}

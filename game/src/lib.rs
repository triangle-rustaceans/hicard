// use std::cmp::{Ord,PartialOrd, Eq, PartialEq};
use deckofcards::{Card, Deck, Suit};
use std::collections::HashMap;
use uuid::Uuid;

use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
    pub name: String,
    pub id: Uuid,
    #[serde(skip_deserializing, serialize_with = "serialize_optcard")]
    pub card: Option<Card>,
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
    fn new(name: &str, id: Uuid) -> Player {
        Player {
            name: name.to_owned(),
            id,
            card: None,
        }
    }
}

pub struct Game {
    deck: Deck,
    players: HashMap<Uuid, Player>,
    current_player: Option<usize>,
    play_order: Vec<Uuid>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            deck: Deck::new(),
            players: HashMap::new(),
            current_player: None,
            play_order: Vec::new(),
        }
    }

    /// Create a player with the provided name, add them to
    /// the roster of players, and set the current player if needed.
    pub fn join(&mut self, name: &str) -> &Player {
        let id = Uuid::new_v4();
        let player = Player::new(name, id);
        self.players.insert(id, player);
        let p = self.players.get(&id).unwrap();
        self.play_order.push(p.id);
        if self.current_player.is_none() {
            self.current_player = Some(0);
        }
        p
    }

    /// If player is the current player deal a card and advance
    /// the current player.  Returns None if the requesting player is not
    /// the current player or an error occurs drawing a card.
    pub fn play(&mut self, player_id: &Uuid) -> Option<Card> {
        let current_player_id = self.current_player.and_then(|idx| self.play_order.get(idx));
        let card = if current_player_id == Some(player_id) {
            match self.deck.deal_one() {
                Ok(card) => Some(card),
                Err(e) => {
                    eprintln!("ERR: {}", e);
                    None
                }
            }
        } else {
            None
        };
        if card.is_some() {
            if let Some(player) = self.players.get_mut(&player_id) {
                player.card = card;
            }
            self.current_player = self.current_player.map(|idx| idx + 1);
        }
        card
    }

    /// Returns None if the game is not over.  Otherwise, returns
    /// a reference to the player with the highest card.
    pub fn winner(&self) -> Option<&Player> {
        let mut winner = None;
        let mut highcard = None;
        for  player in self.players.values() {
            player.card?;

            if highcard.is_none() || player.card > highcard {
                highcard = player.card;
                winner = Some(player);
            }
        }
        winner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_string;

    #[test]
    fn play_game() {
        let mut game = Game::new();
        let player1 = game.join("Cliff").clone();
        let player2 = game.join("Tom").clone();
        assert!(
            game.play(&player2.id).is_none(),
            "player 2 should not be able to play first"
        );
        let p1card = game.play(&player1.id).expect("player 1 should get a card");
        assert!(
            game.play(&player1.id).is_none(),
            "player 1 should not be able to get another card"
        );
        let p2card = game.play(&player2.id).expect("player 2 should get a card");

        assert!(
            game.play(&player1.id).is_none(),
            "player 1 should not be able to play after the game is over"
        );
        assert!(
            game.play(&player2.id).is_none(),
            "player 2 should not be able to play after the game is over"
        );

        if p1card > p2card {
            assert_eq!(game.winner().map(|p| p.id), Some(player1.id));
        } else if p2card > p1card {
            assert_eq!(game.winner().map(|p| p.id), Some(player2.id))
        }
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

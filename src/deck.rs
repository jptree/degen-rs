use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{Card, Rank, Suit};

struct Deck {
    cards: Vec<Card>,
    rng: ThreadRng,
}

impl Deck {
    fn new(n_decks: u8, rng: ThreadRng) -> Self {
        let mut cards = Vec::new();
        for _ in 0..n_decks {
            for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
                for rank in [
                    Rank::Two,
                    Rank::Three,
                    Rank::Four,
                    Rank::Five,
                    Rank::Six,
                    Rank::Seven,
                    Rank::Eight,
                    Rank::Nine,
                    Rank::Ten,
                    Rank::Jack,
                    Rank::Queen,
                    Rank::King,
                    Rank::Ace,
                ] {
                    cards.push(Card {
                        suit: suit.clone(),
                        rank,
                    });
                }
            }
        }
        Self { cards, rng }
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut self.rng);
    }

    fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

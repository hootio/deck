use rand::prelude::SliceRandom;
use std::cmp::Ordering;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait CardTrait {
    fn abs_rank(&self) -> u8;
    fn value(&self) -> u8;
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, PartialEq, PartialOrd)]
pub enum Suit {
    Clubs = 1,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone, Copy, Debug, EnumIter, Eq, PartialEq, PartialOrd)]

pub enum Rank {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct StandardCard {
    suit: Suit,
    rank: Rank,
}

impl fmt::Display for StandardCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum Color {
    Red,
    Black,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct JokerCard {
    color: Color,
}

impl fmt::Display for JokerCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} Joker", self.color)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum Card {
    StandardCard(StandardCard),
    JokerCard(JokerCard),
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Card::{:?}", self)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.abs_rank().cmp(&other.abs_rank())
    }
}

impl CardTrait for Card {
    fn abs_rank(&self) -> u8 {
        match self {
            Card::StandardCard(standard) => standard.suit as u8 * 10 + standard.rank as u8,
            Card::JokerCard(joker) => {
                if joker.color == Color::Black {
                    0
                } else {
                    1
                }
            }
        }
    }
    fn value(&self) -> u8 {
        match self {
            Card::StandardCard(standard) => {
                let rank = standard.rank as u8;
                if rank < 10 {
                    rank
                } else {
                    10
                }
            }
            Card::JokerCard(_joker) => 0,
        }
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    const STANDARD_SIZE: usize = 52;

    pub fn new() -> Self {
        let mut cards: Vec<Card> = Vec::with_capacity(Deck::STANDARD_SIZE);
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::StandardCard(StandardCard { suit, rank }));
            }
        }
        Deck { cards }
    }

    pub fn new_empty() -> Self {
        let cards: Vec<Card> = Vec::new();
        Deck { cards }
    }

    pub fn empty(&mut self) {
        self.cards.clear();
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn sort(&mut self) {
        self.cards.sort();
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_string() {
        let card = Card::StandardCard(StandardCard {
            suit: Suit::Hearts,
            rank: Rank::King,
        });
        assert_eq!(
            card.to_string(),
            "Card::StandardCard(StandardCard { suit: Hearts, rank: King })"
        );

        let standard = StandardCard {
            suit: Suit::Spades,
            rank: Rank::Six,
        };
        assert_eq!(standard.to_string(), "Six of Spades");

        let joker = JokerCard {
            color: Color::Black,
        };
        assert_eq!(joker.to_string(), "Black Joker");
    }

    #[test]
    fn card_value() {
        let six = Card::StandardCard(StandardCard {
            suit: Suit::Spades,
            rank: Rank::Six,
        });
        assert_eq!(six.value(), 6);

        let king = Card::StandardCard(StandardCard {
            suit: Suit::Hearts,
            rank: Rank::King,
        });
        assert_eq!(king.value(), 10);

        let joker = Card::JokerCard(JokerCard {
            color: Color::Black,
        });
        assert_eq!(joker.value(), 0);
    }

    #[test]
    fn deck() {
        let deck = Deck::new();
        assert_eq!(deck.size(), 52);
    }

    #[test]
    fn deck_empty() {
        let deck = Deck::new_empty();
        assert_eq!(deck.size(), 0);
    }

    #[test]
    fn draw() {
        let mut deck = Deck::new();
        let expected = Card::StandardCard(StandardCard {
            suit: Suit::Spades,
            rank: Rank::King,
        });
        let top = deck.draw().unwrap();
        assert_eq!(top, expected);
        assert_eq!(deck.size(), 51);
    }

    #[test]
    fn draw_empty() {
        let mut deck = Deck::new_empty();
        assert!(deck.draw().is_none());
        assert_eq!(deck.size(), 0);
    }

    #[test]
    fn sort() {
        let mut deck = Deck::new();
        let expected = Card::StandardCard(StandardCard {
            suit: Suit::Spades,
            rank: Rank::King,
        });
        deck.cards.reverse();
        let top = deck.draw().unwrap();
        assert_ne!(top, expected);
        deck.sort();
        let top = deck.draw().unwrap();
        assert_eq!(top, expected);
    }

    #[test]
    fn shuffle() {
        let sorted = Deck::new();
        let mut deck = Deck::new();
        deck.shuffle();
        assert_ne!(sorted.cards, deck.cards);
    }
}

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use std::fmt;

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub enum Suit {
    Clubs = 1,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StandardCard {
    suit: Suit,
    rank: Rank,
}

impl fmt::Display for StandardCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}

// impl StandardCard {
//     pub const MIN_RANK: Rank = Rank::Ace;
//     pub const MAX_RANK: Rank = Rank::King;
//     pub fn abs_rank(&self) -> u32 {
//         self.suit as u32 * PlayingCard::MAX_RANK as u32 + self.rank.unwrap() as u32
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JokerCard {
    color: Color,
}

impl fmt::Display for JokerCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} Joker", self.color)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Card {
    StandardCard(StandardCard),
    JokerCard(JokerCard),
}

pub trait PlayingCard {
    fn abs_rank(&self) -> u8;
}

impl fmt::Debug for dyn PlayingCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.abs_rank())
    }
}

impl PlayingCard for Card {
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
}

// impl Ord for dyn PlayingCard {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.abs_rank().cmp(&other.abs_rank())
//     }
// }

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

    // pub fn sort(&self) {
    //     self.cards.sort_by(|a, b| b.abs_rank().cmp(&a.abs_rank()));
    // }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
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
    fn good_card() {
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
    fn deck() {
        let deck = Deck::new();
        assert_eq!(deck.size(), 52);
    }

    #[test]
    fn empty_deck() {
        let deck = Deck::new_empty();
        assert_eq!(deck.size(), 0);
    }

    #[test]
    fn draw_card() {
        let mut deck = Deck::new();
        let top = deck.draw().unwrap();
        let expected = Card::StandardCard(StandardCard {
            suit: Suit::Spades,
            rank: Rank::King,
        });
        assert_eq!(top, expected);
        assert_eq!(deck.size(), 51);
    }

    #[test]
    fn draw_empty() {
        let mut deck = Deck::new_empty();
        assert!(deck.draw().is_none());
        assert_eq!(deck.size(), 0);
    }
}

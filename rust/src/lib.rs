use rand::prelude::SliceRandom;
use std::cmp::Ordering;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait CardTrait {
    fn abs_rank(&self) -> u8;
    fn value(&self, soft: bool) -> u8;
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
    fn value(&self, soft: bool) -> u8 {
        match self {
            Card::StandardCard(standard) => {
                let rank = standard.rank as u8;
                if rank == 1 && soft == true {
                    11
                } else if rank < 10 {
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

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut card_string = String::new();
        for card in &self.cards {
            card_string.push_str(&card.to_string());
        }
        write!(f, "{:?}", card_string)
    }
}

impl Deck {
    const STANDARD_SIZE: usize = 52;

    fn new_standard_52() -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::with_capacity(Deck::STANDARD_SIZE);
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::StandardCard(StandardCard { suit, rank }));
            }
        }
        cards
    }

    pub fn new() -> Self {
        Deck {
            cards: Self::new_standard_52(),
        }
    }

    pub fn new_count(count: usize) -> Self {
        let mut cards: Vec<Card> = Vec::with_capacity(Deck::STANDARD_SIZE * count);
        for _ in 0..count {
            cards.append(&mut Self::new_standard_52())
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

    pub fn peak(&self, index: usize) -> Option<Card> {
        if index >= self.size() {
            None
        } else {
            Some(self.cards[index])
        }
    }

    pub fn top(&self) -> Option<Card> {
        if self.size() == 0 {
            None
        } else {
            self.peak(self.size() - 1)
        }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn sort(&mut self) {
        self.cards.sort();
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    pub fn value(&self, soft: bool) -> u16 {
        self.cards.iter().map(|c| c.value(soft) as u16).sum()
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
        let eleven = Card::StandardCard(StandardCard {
            suit: Suit::Spades,
            rank: Rank::Ace,
        });
        assert_eq!(eleven.value(true), 11);

        let one = Card::StandardCard(StandardCard {
            suit: Suit::Spades,
            rank: Rank::Ace,
        });
        assert_eq!(one.value(false), 1);

        let six = Card::StandardCard(StandardCard {
            suit: Suit::Spades,
            rank: Rank::Six,
        });
        assert_eq!(six.value(true), 6);

        let king = Card::StandardCard(StandardCard {
            suit: Suit::Hearts,
            rank: Rank::King,
        });
        assert_eq!(king.value(true), 10);

        let joker = Card::JokerCard(JokerCard {
            color: Color::Black,
        });
        assert_eq!(joker.value(true), 0);
    }

    #[test]
    fn deck() {
        let deck = Deck::new();
        assert_eq!(deck.size(), 52);
    }

    #[test]
    fn deck_count() {
        let deck = Deck::new_count(6);
        assert_eq!(deck.size(), 52 * 6);
    }

    #[test]
    fn deck_empty() {
        let deck = Deck::new_empty();
        assert_eq!(deck.size(), 0);
    }

    #[test]
    fn deck_string() {
        let deck = Deck::new();
        assert_eq!(deck.to_string(), "\"Card::StandardCard(StandardCard { suit: Clubs, rank: Ace })Card::StandardCard(StandardCard { suit: Clubs, rank: Two })Card::StandardCard(StandardCard { suit: Clubs, rank: Three })Card::StandardCard(StandardCard { suit: Clubs, rank: Four })Card::StandardCard(StandardCard { suit: Clubs, rank: Five })Card::StandardCard(StandardCard { suit: Clubs, rank: Six })Card::StandardCard(StandardCard { suit: Clubs, rank: Seven })Card::StandardCard(StandardCard { suit: Clubs, rank: Eight })Card::StandardCard(StandardCard { suit: Clubs, rank: Nine })Card::StandardCard(StandardCard { suit: Clubs, rank: Ten })Card::StandardCard(StandardCard { suit: Clubs, rank: Jack })Card::StandardCard(StandardCard { suit: Clubs, rank: Queen })Card::StandardCard(StandardCard { suit: Clubs, rank: King })Card::StandardCard(StandardCard { suit: Diamonds, rank: Ace })Card::StandardCard(StandardCard { suit: Diamonds, rank: Two })Card::StandardCard(StandardCard { suit: Diamonds, rank: Three })Card::StandardCard(StandardCard { suit: Diamonds, rank: Four })Card::StandardCard(StandardCard { suit: Diamonds, rank: Five })Card::StandardCard(StandardCard { suit: Diamonds, rank: Six })Card::StandardCard(StandardCard { suit: Diamonds, rank: Seven })Card::StandardCard(StandardCard { suit: Diamonds, rank: Eight })Card::StandardCard(StandardCard { suit: Diamonds, rank: Nine })Card::StandardCard(StandardCard { suit: Diamonds, rank: Ten })Card::StandardCard(StandardCard { suit: Diamonds, rank: Jack })Card::StandardCard(StandardCard { suit: Diamonds, rank: Queen })Card::StandardCard(StandardCard { suit: Diamonds, rank: King })Card::StandardCard(StandardCard { suit: Hearts, rank: Ace })Card::StandardCard(StandardCard { suit: Hearts, rank: Two })Card::StandardCard(StandardCard { suit: Hearts, rank: Three })Card::StandardCard(StandardCard { suit: Hearts, rank: Four })Card::StandardCard(StandardCard { suit: Hearts, rank: Five })Card::StandardCard(StandardCard { suit: Hearts, rank: Six })Card::StandardCard(StandardCard { suit: Hearts, rank: Seven })Card::StandardCard(StandardCard { suit: Hearts, rank: Eight })Card::StandardCard(StandardCard { suit: Hearts, rank: Nine })Card::StandardCard(StandardCard { suit: Hearts, rank: Ten })Card::StandardCard(StandardCard { suit: Hearts, rank: Jack })Card::StandardCard(StandardCard { suit: Hearts, rank: Queen })Card::StandardCard(StandardCard { suit: Hearts, rank: King })Card::StandardCard(StandardCard { suit: Spades, rank: Ace })Card::StandardCard(StandardCard { suit: Spades, rank: Two })Card::StandardCard(StandardCard { suit: Spades, rank: Three })Card::StandardCard(StandardCard { suit: Spades, rank: Four })Card::StandardCard(StandardCard { suit: Spades, rank: Five })Card::StandardCard(StandardCard { suit: Spades, rank: Six })Card::StandardCard(StandardCard { suit: Spades, rank: Seven })Card::StandardCard(StandardCard { suit: Spades, rank: Eight })Card::StandardCard(StandardCard { suit: Spades, rank: Nine })Card::StandardCard(StandardCard { suit: Spades, rank: Ten })Card::StandardCard(StandardCard { suit: Spades, rank: Jack })Card::StandardCard(StandardCard { suit: Spades, rank: Queen })Card::StandardCard(StandardCard { suit: Spades, rank: King })\"");
    }

    #[test]
    fn peak() {
        let deck = Deck::new();
        let first = Card::StandardCard(StandardCard {
            suit: Suit::Clubs,
            rank: Rank::Ace,
        });
        let last = Card::StandardCard(StandardCard {
            suit: Suit::Spades,
            rank: Rank::King,
        });
        assert_eq!(deck.peak(0).unwrap(), first);
        assert_eq!(deck.peak(51).unwrap(), last);
        assert!(deck.peak(52).is_none());
        assert_eq!(deck.size(), 52);
    }

    #[test]
    fn peak_empty() {
        let deck = Deck::new_empty();
        assert!(deck.peak(0).is_none());
        assert!(deck.peak(1).is_none());
        assert_eq!(deck.size(), 0);
    }

    #[test]
    fn top() {
        let deck = Deck::new();
        let expected = Card::StandardCard(StandardCard {
            suit: Suit::Spades,
            rank: Rank::King,
        });
        let top = deck.top().unwrap();
        assert_eq!(top, expected);
        assert_eq!(deck.size(), 52);
    }

    #[test]
    fn top_empty() {
        let deck = Deck::new_empty();
        assert!(deck.top().is_none());
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
    fn add() {
        let mut deck = Deck::new_empty();
        let card = Card::StandardCard(StandardCard {
            suit: Suit::Spades,
            rank: Rank::King,
        });
        deck.add(card);
        assert_eq!(deck.size(), 1);
        let top = deck.draw().unwrap();
        assert_eq!(top, card);
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
        // the probability of this asssert failing is: 1/deck.size()!
        assert_ne!(sorted.cards, deck.cards);
    }

    #[test]
    fn value() {
        let deck = Deck::new();
        assert_eq!(deck.value(true), 380);
        assert_eq!(deck.value(false), 340);
    }

    #[test]
    fn value_empty() {
        let deck = Deck::new_empty();
        assert_eq!(deck.value(true), 0);
        assert_eq!(deck.value(false), 0);
    }
}

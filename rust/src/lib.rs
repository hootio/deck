use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Suit {
    Spade = 0,
    Diamond,
    Club,
    Heart,
    Joker,
}
// TODO: Add enum iterator instead
const SUITS: [Suit; 4] = [Suit::Spade, Suit::Diamond, Suit::Club, Suit::Heart];

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Rank {
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
// TODO: Add enum iterator instead
const RANKS: [Rank; 13] = [
    Rank::Ace,
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
];

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Card {
    suit: Suit,
    rank: Option<Rank>,
}

impl Card {
    pub const MIN_RANK: Rank = Rank::Ace;
    pub const MAX_RANK: Rank = Rank::King;
    pub fn new(suit: Suit, rank: Option<Rank>) -> Self {
        if (suit == Suit::Joker) != (rank == None) {
            // TODO: Handle this better?
            panic!("this is not allowed!");
        }

        Card { suit, rank }
    }
    pub fn abs_rank(&self) -> u32 {
        self.suit as u32 * Card::MAX_RANK as u32 + self.rank.unwrap() as u32
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.suit == Suit::Joker {
            write!(f, "Joker")
        } else {
            write!(
                f,
                "{} of {}s",
                self.rank.unwrap().to_string(),
                self.suit.to_string()
            )
        }
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    const TYPICAL_SIZE: usize = 54;

    pub fn new(num_decks: u32, num_jokers_per_deck: u32) -> Self {
        let mut cards: Vec<Card> = Vec::with_capacity(Deck::TYPICAL_SIZE * num_decks as usize);
        for _ in 0..num_decks {
            for s in SUITS {
                for r in RANKS {
                    cards.push(Card::new(s, Some(r)));
                }
            }
            for _ in 0..num_jokers_per_deck {
                cards.push(Card::new(Suit::Joker, None))
            }
        }
        Deck { cards }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn good_card() {
        let card = Card::new(Suit::Spade, Some(Rank::Six));
        assert_eq!(card.to_string(), "Six of Spades");
        let card = Card::new(Suit::Joker, None);
        assert_eq!(card.to_string(), "Joker");
    }
    #[test]
    #[should_panic]
    fn bad_card1() {
        Card::new(Suit::Joker, Some(Rank::Six));
    }
    #[test]
    #[should_panic]
    fn bad_card2() {
        Card::new(Suit::Spade, None);
    }

    #[test]
    fn deck() {
        let param_tupes: [(u32, u32); 4] = [(1, 0), (1, 2), (7, 0), (7, 2)];

        for (num_deck, num_jokers_per_deck) in param_tupes {
            let deck = Deck::new(num_deck, num_jokers_per_deck);
            let expected_len = (num_deck * (52 + num_jokers_per_deck)) as usize;
            assert_eq!(deck.cards.len(), expected_len);
        }
    }
}

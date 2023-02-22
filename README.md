# deck 🃏

Deck of cards package for various languages (inspired by https://github.com/gophercises/deck)

## API

### enum Suit

- ♣ Clubs
- ♦ Diamonds
- ♥ Hearts
- ♠ Spades
- 🃏 JokerBlack
- 🃟 JokerRed

### enum Rank

- Jo: Joker
- A: Ace
- 2: Two
- 3: Three
- 4: Four
- 5: Five
- 6: Six
- 7: Seven
- 8: Eight
- 9: Nine
- 10: Ten
- J: Jack
- Q: Queen
- K: King

### struct Card

#### Fields

- suit: Suit
- rank: Rank

#### Methods

- new(suit: Suit, rank: Rank)
  Creates new `Card` instance given `suit` and `rank`.
  For Joker, `rank` must be `Rank::Joker` and `suit` must be either `JokerBlack` or `JokerRed`.

#### Traits

- CardMath
  - Add
  - Difference
  - Equals
  - IsGreater

### struct Deck

#### Fields

- cards: Vec<Card>

#### Methods

- new()
  Creates new `Deck` instance with `cards` filled with the standard 52 playing cards.
- new_empty()
  Creates new `Deck` instance with empty `cards`.
- add(card: Card)
  Adds `card` to `cards`.
- empty()
  Removes all cards from `Deck`.
- shuffle()
  Resets `Deck` and shuffles `cards`.
- draw()
  Retrieves the top `Card` from `cards`.
- sort(ascending: bool)
  Sorts `cards` in ascending or descending order.
- shuffle()
  Shuffles `cards`.

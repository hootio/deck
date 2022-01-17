package deck

import (
	"fmt"
	"math/rand"
	"testing"
)

func ExampleCard() {
	fmt.Println(Card{Rank: Ace, Suit: Heart})
	fmt.Println(Card{Rank: Two, Suit: Spade})
	fmt.Println(Card{Rank: Nine, Suit: Diamond})
	fmt.Println(Card{Rank: Jack, Suit: Club})
	fmt.Println(Card{Suit: Joker})

	// Output:
	// Ace of Hearts
	// Two of Spades
	// Nine of Diamonds
	// Jack of Clubs
	// Joker
}

func TestNewDeck(t *testing.T) {
	deck := New(1, 0)
	if len(deck) != 4*13 { // 4 suits and 13 ranks
		t.Error("Wrong number of cards in new deck")
	}
}

func TestNewDeckWithJokers(t *testing.T) {
	deck := New(1, 2)
	if len(deck) != 4*13+2 { // 4 suits and 13 ranks and 2 jokers
		t.Error("Wrong number of cards in new deck")
	}
}

func Test4NewDecks(t *testing.T) {
	deck := New(4, 0)
	if len(deck) != 4*13*4 { // 4 suits and 13 ranks in 4 decks
		t.Error("Wrong number of cards in new deck")
	}
}

func Test4NewDecksWishJokers(t *testing.T) {
	deck := New(4, 2)
	if len(deck) != (4*13+2)*4 { // 4 suits, 13 ranks, and 2 jokers in 4 decks
		t.Error("Wrong number of cards in new deck")
	}
}

func TestNewDeckSortedDefault(t *testing.T) {
	deck := New(1, 2)
	deck.Sort(nil)
	expectedFirstCard := Card{Suit: Spade, Rank: Ace}
	if deck[0] != expectedFirstCard {
		t.Error("Expected Ace of Spades but got: ", deck[0])
	}
}

func TestNewDeckSortedCustom(t *testing.T) {
	deck := New(1, 2)
	deck.Sort(deck.Greater())
	expectedFirstCard := Card{Suit: Joker, Rank: Rank(1)}
	if deck[0] != expectedFirstCard {
		t.Error("Expected King of Hearts but got: ", deck[0])
	}
}

func TestShuffle(t *testing.T) {
	getRandIdx := func(d Deck) int {
		return rand.Intn(len(d))
	}
	deck := New(1, 2)
	i, j, k := getRandIdx(deck), getRandIdx(deck), getRandIdx(deck)
	a, b, c := deck[i], deck[j], deck[k]
	deck.Shuffle()
	if a == deck[i] && b == deck[j] && c == deck[k] {
		t.Error("Expected card positions to change")
	}
}

func TestFilter(t *testing.T) {
	deck := New(1, 2)
	deck = deck.Filter([]Suit{Joker}, []Rank{Two, Three})
	if len(deck) != 44 {
		t.Error("Expected 44 cards but got: ", len(deck))
	}
	for _, card := range deck {
		if card.Rank == Two || card.Rank == Three || card.Suit == Joker {
			t.Error("Expected all twos, threes, and jokers to be filtered out")
		}
	}
}

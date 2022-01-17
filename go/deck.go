//go:generate stringer -type=Suit,Rank

package deck

import (
	"fmt"
	"math/rand"
	"sort"
)

const TypicalDeckSize = 54

type Suit uint8

const (
	Spade Suit = iota
	Diamond
	Club
	Heart
	Joker // Special case
)

var suits = [...]Suit{Spade, Diamond, Club, Heart}

type Rank uint8

const (
	Ace Rank = iota + 1 // Start at 1 for simplicity
	Two
	Three
	Four
	Five
	Six
	Seven
	Eight
	Nine
	Ten
	Jack
	Queen
	King
)

const (
	minRank = Ace
	maxRank = King
)

type Card struct {
	Suit
	Rank
}

func (c Card) String() string {
	if c.Suit == Joker {
		return c.Suit.String()
	}
	return fmt.Sprintf("%s of %ss", c.Rank.String(), c.Suit.String())
}

func (c Card) absRank() int {
	return int(c.Suit)*int(maxRank) + int(c.Rank)
}

type Deck []Card

func New(numDecks, numJokersPerDeck uint) Deck {

	deck := make(Deck, 0, TypicalDeckSize*numDecks)

	for count := 0; uint(count) < numDecks; count++ {

		for _, suit := range suits {
			for rank := minRank; rank <= maxRank; rank++ {
				deck = append(deck, Card{Suit: suit, Rank: rank})
			}
		}

		for i := 0; uint(i) < numJokersPerDeck; i++ {
			deck = append(deck, Card{Suit: Joker, Rank: Rank(i)})
		}

	}

	return deck
}

func (d Deck) Less() func(i, j int) bool {
	return func(i, j int) bool {
		return d[i].absRank() < d[j].absRank()
	}
}

func (d Deck) Greater() func(i, j int) bool {
	return func(i, j int) bool {
		return d[i].absRank() > d[j].absRank()
	}
}

func (d Deck) Sort(comparator func(i, j int) bool) {
	if comparator == nil {
		comparator = d.Less()
	}
	sort.Slice(d, comparator)
}

func (d Deck) Shuffle() {
	rand.Shuffle(len(d), func(i, j int) {
		d[i], d[j] = d[j], d[i]
	})
}

func (d Deck) Filter(suits []Suit, ranks []Rank) Deck {

	suitSet := map[Suit]bool{}
	for _, s := range suits {
		suitSet[s] = true
	}

	rankSet := map[Rank]bool{}
	for _, r := range ranks {
		rankSet[r] = true
	}

	filtered := make(Deck, 0, len(d))

	for _, card := range d {
		_, okSuit := suitSet[card.Suit]
		_, okRank := rankSet[card.Rank]
		if okSuit || okRank {
			continue
		} else {
			filtered = append(filtered, card)
		}
	}

	return filtered
}

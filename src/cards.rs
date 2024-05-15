#[derive(Debug, PartialEq)]
enum Suit {
  Clubs,
  Diamonds,
  Hearts,
  Spades
}

#[derive(Debug, PartialEq)]
struct Card<'a, T> {
  value: &'a T,
  suit: Suit
}

struct Deck<'a, T> {
  cards: Vec<Card<'a, T>>
}

impl<'a, T> Deck<'a, T> {
  pub fn new() -> Deck<'a, T> {
    Deck {cards: Vec::new()}
  }

  pub fn add_card(&mut self, card: Card<'a, T>) {
    self.cards.push(card);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_card() {
    let mut deck: Deck<i32> = Deck::new();
    deck.add_card(Card {
      value: &1,
      suit: Suit::Clubs
    });
    assert_eq!(deck.cards, vec![Card { value: &1, suit: Suit::Clubs }]);
  }
}
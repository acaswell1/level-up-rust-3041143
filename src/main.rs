#![allow(dead_code)]

#[derive(Debug, PartialEq)]
enum Card {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand {
            cards: vec![],
        }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        let mut sum_cards : usize = 0;

        for card in &self.cards{
            match card {
                Card::Ace => {
                    if sum_cards + 11 <= 21 {
                        sum_cards += 11;
                    } else {
                        sum_cards += Card::Ace as usize;
                    } 
                },
                Card::Two => {sum_cards += Card::Two as usize },
                Card::Three => {sum_cards += Card::Three as usize },
                Card::Four => {sum_cards += Card::Four as usize },
                Card::Five => {sum_cards += Card::Five as usize },
                Card::Six => {sum_cards += Card::Six as usize },
                Card::Seven => {sum_cards += Card::Seven as usize },
                Card::Eight => {sum_cards += Card::Eight as usize },
                Card::Nine => {sum_cards += Card::Nine as usize },
                Card::Jack => {sum_cards += Card::Jack as usize },
                Card::Queen => {sum_cards += 10 },
                Card::King => {sum_cards += 10 },
            }
        }
        sum_cards
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Ace);

    hand.value();
}


#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Ace);
    
    assert_eq!(hand.value(), 21);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Five);

    assert!(hand.is_loosing_hand());
    assert_eq!(hand.value(), 22);
}

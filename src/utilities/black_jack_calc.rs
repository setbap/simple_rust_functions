#[derive(PartialEq, Eq)]
pub enum BlackJackCards {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    King,
    Jack,
    Queen,
    Ace,
}

pub struct Hand {
    cards: Vec<BlackJackCards>,
}

impl Hand {
    pub fn new(cards: Vec<BlackJackCards>) -> Self {
        Hand { cards: cards }
    }

    pub fn value(&self) -> i32 {
        let mut deck_value = 0;
        let mut number_of_ace = 0;
        for card in &self.cards {
            deck_value += card.get_card_value();
            if *card == BlackJackCards::Ace {
                number_of_ace += 1;
            }
        }
        if deck_value > 21 {
            deck_value -= number_of_ace * 10;
        }
        deck_value
    }
}

impl BlackJackCards {
    fn get_card_value(&self) -> i32 {
        match self {
            BlackJackCards::Two => 2,
            BlackJackCards::Three => 3,
            BlackJackCards::Four => 4,
            BlackJackCards::Five => 5,
            BlackJackCards::Six => 6,
            BlackJackCards::Seven => 7,
            BlackJackCards::Eight => 8,
            BlackJackCards::Nine => 9,
            BlackJackCards::Jack | BlackJackCards::Queen | BlackJackCards::King => 10,
            BlackJackCards::Ace => 11,
        }
    }
}

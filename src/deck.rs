use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    value1: i32,
    value2: i32,
	pub name: char,
    pub suit: Suit,
}                                                                                                                                                                    

#[derive(Debug)]

pub struct Deck{
	cards: Vec<Card>,
}

impl Card {
	pub fn new(number: i32, suit: Suit) -> Self{
		let mut value1 = 0;
		let mut value2 = 0;
		let mut name: char = char::MAX;

		// Just being thurough
		if number == 1 {
			value1 = 1;
			value2 = 11;
			name = 'A';
		}
		else if number >= 10 && number <= 13 {
			value1 = 10;
			value2 = i32::MAX;
			match number {
				10 => name = 'T',
				11 => name = 'J',
				12 => name = 'Q',
				13 => name = 'K',
				_ => ()

			}
		} 
		else {
			value1 	= number;
			value2 	= i32::MAX;
			let c = char::from_digit(number as u32, 10);
			match c {
				Some(c) => name = c,
				None => println!("Problem assigning name for {}", number),
			}
		}

		Card { 
			value1,
			value2,
			name,
			suit
		}
	}
}

impl Deck {
	// Generate card map, then shuffle into vector -> for self
	pub fn new() -> Self {
		Deck {
			cards : Vec::new(),
		}
	}

	pub fn draw_card(&mut self) -> Card {
		let mut rng = thread_rng();
		let pull: usize = rng.gen_range(0..self.cards.len());
		self.cards.swap_remove(pull) 
	}

	pub fn build_deck(&mut self) {
		for i in 0..NUM_SUITS {
			let suits: [Suit;4] =  [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
			for j in 0..CARDS_PER_SUIT {
				let card = Card::new((j % 13)+1, suits[i as usize]);
				self.cards.push(card);
			}
		}
	}

	pub fn print_deck(&mut self) {
		for c in &mut self.cards {
			println!("{:?}", c);
		}
	}

}
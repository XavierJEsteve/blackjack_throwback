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
    number: i32,
	ace: bool,
	face: bool,
    suit: Suit,
}                                                                                                                                                                    

#[derive(Debug)]

pub struct Deck{
	cards: Vec<Card>,
}

impl Card {
	pub fn new(number: i32, suit: Suit) -> Self{
		let mut ace = false;
		let mut face = false;

		if number == 1 {
			ace = true;
		}
		if number > 10 {
			face = true;
		}

		Card { 
			number,
			ace,
			face,
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
use crate::prelude::*;
use std::collections::HashMap;

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

fn card_mapping(card_map: &mut HashMap<i32,Card> ) {
	// For every suit
	for i in 0..NUM_SUITS {
		/* Want map organized as follows:
			0->12 Hearts: 1 - 13
			13->25 Diamonds: 1 - 13 
			26->38 Clubs: 1 - 13 
			39->51 Spades: 1 - 13         
		*/
		let suits: [Suit;4] =  [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
		for j in 0..CARDS_PER_SUIT {
			let card = Card::new((j % 13)+1, suits[i as usize]);
			card_map.insert(13*i+j, card);
		}
	}
}

fn print_map( map: &HashMap<i32,Card> ) {
	// Test card mapping
	for c in 0..NUM_CARDS{
		match map.get(&c) {
			Some(card) => println!("{c}: {:?}",card),
			None => println!("{c} is not in play.")
		}
	}
}

/* Todo: Actually shuffle all of these cards and retrun that vector */
fn shuffle_deck( map: &HashMap<i32,Card>) -> Vec<Card> {
	let mut cards = vec![];
	match map.get(&0) {
		Some(card) => cards.push(card.to_owned()),
		None => println!("Couldn't shuffle deck")
	}
	cards
}

impl Deck {
	// Generate card map, then shuffle into vector -> for self
	pub fn new() -> Self {
		let mut card_map = HashMap::<i32, Card>::new();
		card_mapping(&mut card_map);
		let cards = shuffle_deck(&card_map);
		Deck {
			cards: cards
		}
	}

	pub fn draw_card(&mut self) -> Option<Card> {
		self.cards.pop() // 
	}

}
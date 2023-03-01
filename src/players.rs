use crate::prelude::*;

#[derive(Debug)]
pub struct Hand {
    count: i32,
    busted: bool,
    cards: Vec<Card>
}

pub struct Player {
    pub hand: Hand,
}
pub struct Dealer {
    hand: Hand,
}


impl Hand{
    pub fn new() -> Self{
        Hand {
            count: 0,
            busted: false,
            cards: Vec::new()
        }
    }
    pub fn print_hand(&self) {
        for c in &self.cards {
            println!("{:?}",c);
        }
    }
}

impl Player{
    pub fn new() -> Self{
        Player {
            hand: Hand::new(),
        }
    }
}

impl Dealer{
    pub fn new() -> Self{
        Dealer {
            hand: Hand::new(),
        }
    }

    //Makes sense that the dealer knows who to deal to. Borrow mutable reference to the player, and populate their hand. Then populate self.hand
    pub fn deal_cards(&mut self, player: &mut Player, deck: &mut Deck) {  
        // everyone gets two cards to start.
        // Dealers cards
        for _ in 0..2 {
            self.hand.cards.push(deck.draw_card());
        }
        // Player cards
        for _ in 0..2 {
            player.hand.cards.push(deck.draw_card());
        }
    }    
}
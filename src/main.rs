mod players;
mod deck;

mod prelude {

    pub use rand::{thread_rng, Rng};

    pub use crate::players::*;
    pub use crate::deck::*;

    pub const NUM_SUITS: i32 = 4;
    pub const CARDS_PER_SUIT: i32 = 13;
    pub const NUM_CARDS: i32 = 52;
    pub const NUM_CARDS_usize: u32 = 52;
    pub const NUM_ACE_CARDS: i32 = 4;
    pub const NUM_FACE_CARDS: i32 = 12;
}

use prelude::*;

fn main() {
    println!("Welcome to blackjack!");
    println!("This is a throwback to my first programming project I made in Java (ew).");
    println!("In this game it's just you and the dealer. 1 deck is used");
    
    let mut handcount: i32 = 0;
    loop {
        handcount += 1;
        println!("*****************");
        println!("HAND {} ", handcount);
        let mut deck = Deck::new();
        deck.build_deck();





        println!("*****************");
        break;
    }
    
}
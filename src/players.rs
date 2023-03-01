use crate::prelude::*;

struct Player {
    hand: Vec<Card>,
    total: u32,
}
struct Dealer {
    hand: Vec<Card>,
    total: u32,
}

impl Player{
    pub fn new() -> Self{
        Player {
            hand: Vec::new(),
            total: 0,
        }
    }
}

impl Dealer{
    pub fn new() -> Self{
        Dealer {
            hand: Vec::new(),
            total: 0,
        }
    }
}
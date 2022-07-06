use chrono::prelude::*;

pub struct App {
    pub blocks: Vec<Block>,
}

impl App {
    fn new () -> Self {
        App {
            blocks: Vec::new()
        }
    }

    fn genesis_block (&mut self) {
        Block {
            id: 0,
            hash: String::from("0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43"),
            prev_hash: String::from("Genesis prev hash"),
            timestamp: Utc::now().timestamp(),
            data: String::from("Genesis block data"),
            nonce: 2836,
        };
        
    }
}

pub struct Block {
    pub id: u64,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

fn main() {
    let mut blockchain = App::new();
    blockchain.genesis_block();

    println!("Blockchaint init!");
}

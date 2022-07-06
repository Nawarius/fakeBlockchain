use chrono::prelude::*;
use log::{error, info, warn};

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
        let genesis_block = Block {
            id: 0,
            hash: String::from("0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43"),
            prev_hash: String::from("Genesis prev hash"),
            timestamp: Utc::now().timestamp(),
            data: String::from("Genesis block data"),
            nonce: 2836,
        };
        self.blocks.push(genesis_block);
    }

    fn try_add_block (&mut self, block: Block) {
        let last_block = self.blocks.last().expect("At least one block must be in chain");
        
        if self.is_block_valid(&block, &last_block) {
            self.blocks.push(block);
        } else {
            error!("Can`t add block. Reason: block is invalid");
        }

    }

    fn is_block_valid (&self, block: &Block, prev_block: &Block) -> bool {
        if block.prev_hash != prev_block.hash {
            warn!("Block with id {} has invalid id!", block.id);
            return false
        } 

        true
    }
}

#[derive(Debug)] 
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

fn hash_to_binary (hash: &String) -> String {
    let hex_hash = hex::decode(hash).expect("Can`t decode from hex");
    let mut res = String::new();

    for c in hex_hash { 
        res.push_str(&format!("{:b}", c)); 
    }

    res
}

fn main() {
    let mut blockchain = App::new();
    blockchain.genesis_block();

    println!("Blockchaint init!");
}

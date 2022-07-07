use chrono::prelude::*;
use log::{error, info, warn, LevelFilter};
use simplelog::{TermLogger, TerminalMode, Config};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const DIFFICULTY_PREFIX: &str = "00";

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
            warn!("Block with id {} has invalid prev hash!", block.id);
            return false
        } else if !hash_to_binary_string(&block.hash).starts_with(DIFFICULTY_PREFIX) {
            warn!("Block with id: {} has invalid difficulty!", block.id);
            return false
        } else if block.id != prev_block.id + 1 {
            warn!("Block with id: {} has invalid id!", block.id);
            return false
        } else if calc_hash(block.id, block.timestamp, &block.prev_hash, &block.data, block.nonce) != block.hash {
            warn!("Block with id: {} has invalid hash", block.id);
            return false
        }
        info!("Block was added successfull!");

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

impl Block {
    fn new (id: u64, prev_hash: String, data: String) -> Self {
        let now = Utc::now();
        let (nonce, hash) = mine_block(id, now.timestamp(), &prev_hash, &data);
        Block {
            id, hash, prev_hash, timestamp: now.timestamp(), data, nonce
        }
    }
}

fn mine_block (id: u64, timestamp: i64, prev_hash: &str, data: &str) -> (u64, String) {
    info!("Mining new block...");
    let mut nonce = 0;

    loop {
        if nonce % 100_000 == 0 {
            info!("Nonce is: {}", nonce);
        }

        let hash = calc_hash(id, timestamp, prev_hash, data, nonce);
        let bin_hash = hash_to_binary_string(&hash);

        if bin_hash.starts_with(DIFFICULTY_PREFIX) {
            info!("Mined! Hash is: {}, Binary hash is: {}, Nonce is: {}", hash, bin_hash, nonce);
            return (nonce, hash)
        }

        nonce += 1;

    }
}

fn hash_to_binary_string (hash: &String) -> String {
    let hex_hash = hex::decode(hash).expect("Can`t decode from hex");
    let mut res = String::new();

    for c in hex_hash { 
        res.push_str(&format!("{:b}", c)); 
    }
    
    res
}

fn calc_hash (id: u64, timestamp: i64, prev_hash: &str, data: &str, nonce: u64) -> String {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": prev_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });

    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    
    hex::encode(hasher.finalize().as_slice().to_owned())
}


fn main() {

    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Stdout);
    
    let mut blockchain = App::new();
    blockchain.genesis_block();

    // let fake_block = Block {
    //     id: 1,
    //     hash: calc_hash(1, 2, &blockchain.blocks[0].hash, "Data some", 1),
    //     prev_hash: blockchain.blocks[0].hash.clone(),
    //     timestamp: 12,
    //     data: "DAta".to_string(),
    //     nonce: 2,
    // };

    let block = Block::new(1, blockchain.blocks[0].hash.clone(), "Some data".to_string());

    blockchain.try_add_block(block);

    println!("Blockchaint init!");
}

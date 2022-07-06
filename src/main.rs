pub struct App {
    pub blocks: Vec<Block>,
}

pub struct Block {
    pub id: u64,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: u64,
    pub data: String,
    pub nonce: u64,
}

fn main() {
    println!("Hello, world!");
}

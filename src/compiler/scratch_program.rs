use serde_json::Value;

use crate::compiler::opcodes::*;

pub struct ScratchProgram {
    pub chains: Vec<BlockChain>,
}

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn parse(entry_point_id: &str, blocks: Value) -> BlockChain {
        let mut chain = BlockChain { blocks: Vec::new() };

        let mut block = blocks[entry_point_id].clone();

        loop {
            chain.blocks.push(Block::parse(block.clone()));

            if block["next"].is_null() {
                break;
            }
            else {
                block = blocks[block["next"].as_str().unwrap()].clone();
            }
        }

        BlockChain { blocks: Vec::new() }
    }
}


pub struct Block {
    pub opcode: BlockOpcode,
}

impl Block {
    pub fn parse(block: Value) -> Block {
        let og_opcode = block["opcode"].as_str().unwrap().to_owned();
        let opcode: String = og_opcode.chars().filter(|&c| c != '_').collect();
        let opcode = string_to_block_opcode(&opcode);

        if opcode == BlockOpcode::None {
            println!("WARNING: opcode {} is not recognized", og_opcode)
        }

        Block { opcode }
    }
}
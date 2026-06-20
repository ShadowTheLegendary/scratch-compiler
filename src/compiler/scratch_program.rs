use serde_json::Value;

use crate::compiler::opcodes::*;

pub struct ScratchProgram {
    pub sprites: Vec<Sprite>,
}

impl ScratchProgram {
    pub fn parse(json: &Value) -> ScratchProgram {
        let mut sprites: Vec<Sprite> = Vec::new();
        let targets = json["targets"].as_array().unwrap();

        for target in targets.iter() {
            sprites.push(Sprite::parse(target));
        }

        ScratchProgram { sprites }
    }
}


pub struct Sprite {
    pub chains: Vec<BlockChain>,
}

impl Sprite {
    pub fn parse(sprite: &Value) -> Sprite {
        let mut chains: Vec<BlockChain> = Vec::new();
        let blocks = &sprite["blocks"];
        let blocks_map = sprite["blocks"].as_object().unwrap();

        for (block_id, block) in blocks_map.iter() {
            if is_entry_point(block["opcode"].as_str().unwrap()) {
                chains.push(BlockChain::parse(block_id, blocks));
            }
        }

        Sprite { chains }
    }
}


pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn parse(entry_point_id: &str, blocks: &Value) -> BlockChain {
        let mut chain = BlockChain { blocks: Vec::new() };

        let mut block = &blocks[entry_point_id].clone();
        let mut block_id = entry_point_id;

        loop {
            if is_flow_control(block) {
                chain.blocks.append(&mut Block::parse_flow_control(block_id, blocks));
            } else {
                chain.blocks.push(Block::parse(&block));
            }

            if !block["next"].is_null() {
                break;
            }
            block_id = block["next"].as_str().unwrap();
            block = &blocks[block_id];
        }

        chain
    }
}

pub struct Block {
    pub opcode: BlockOpcode,
}

impl Block {
    pub fn parse(block: &Value) -> Block {
        let og_opcode = block["opcode"].as_str().unwrap().to_owned();
        let opcode: String = og_opcode.chars().filter(|&c| c != '_').collect();
        let opcode = string_to_block_opcode(&opcode);

        if opcode == BlockOpcode::None {
            println!("WARNING: opcode {} is not recognized", og_opcode)
        }

        Block { opcode }
    }

    pub fn parse_flow_control(block_id: &str, blocks: &Value) -> Vec<Block> {
        let mut chain = vec![Block::parse(&blocks[block_id])];

        let mut block = blocks[blocks[block_id]["inputs"]["SUBSTACK"].as_array().unwrap()[1].as_str().unwrap()].clone();

        loop {
            chain.push(Block::parse(&block));

            if block["inputs"]["SUBSTACK"].is_null() {
                break;
            } else {
                chain.append(&mut Block::parse_flow_control(blocks[block_id]["inputs"]["SUBSTACK"].as_array().unwrap()[1].as_str().unwrap(), blocks));
            }

            if !block["next"].is_null() {
                block = blocks[block["next"].as_str().unwrap()].clone();
            }
        }

        chain
    }
}
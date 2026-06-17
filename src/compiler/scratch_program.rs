use crate::compiler::opcodes::*;

pub struct ScratchProgram {
    pub chains: Vec<BlockChain>,
}

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

pub struct Block {
    pub opcode: BlockOpcode,
}

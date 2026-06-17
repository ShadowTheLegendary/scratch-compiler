use serde_json::Value;

pub mod opcodes;
pub mod scratch_program;

//use crate::BUILD_DIRECTORY;
use scratch_program::ScratchProgram;

pub fn compile(project_dir: &str, json: &Value) {
    let program = parser(project_dir, json);
    code_generator(&program, project_dir);
}

fn parser(_project_dir: &str, _json: &Value) -> ScratchProgram {
    ScratchProgram { chains: Vec::new() }
}

fn code_generator(_program: &ScratchProgram, _project_dir: &str) {}

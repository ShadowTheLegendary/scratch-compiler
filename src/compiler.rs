use serde_json::Value;

pub mod opcodes;
pub mod scratch_program;

use scratch_program::ScratchProgram;


pub fn compile(project_dir: &str, json: &Value) {
    let program = ScratchProgram::parse(json);
    code_generator(&program, project_dir);
}


fn code_generator(_program: &ScratchProgram, _project_dir: &str) {}

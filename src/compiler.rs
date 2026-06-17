pub mod opcodes;
pub mod scratch_program;

//use crate::BUILD_DIRECTORY;
use scratch_program::ScratchProgram;

pub fn compile(project_dir: &str) {
   let program = parser(project_dir);
   code_generator(&program, project_dir);
}

fn parser(_project_dir: &str) -> ScratchProgram { 
    //let file = std::fs::read_to_string(BUILD_DIRECTORY.to_owned() + project_dir + "project.json").expect("Error opening project.json.");


    ScratchProgram {} 
}

fn code_generator(_program: &ScratchProgram, _project_dir: &str) {

}

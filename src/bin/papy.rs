#![feature(if_let)]
extern crate papy;

#[cfg(not(test))]
use papy::{PapyProgram};
#[cfg(not(test))]
fn main() {
    let mut program = PapyProgram::new();

    //TODO REPL support
    //for stdin_line in std::io::stdin().lines() {
    //   let line = match stdin_line {
    //        Ok(line) => line,
    //        Err(e) => fail!("unexpected input \"{}\". Exiting", e)
    //    };
    //    program.add_instruction(line)
    //}
    let lines = vec![
        "3",
        "4",
        "5",
        "+",
        "+",
    ];
    for line in lines.into_iter() {
        program.add_instruction(line);
    }
}

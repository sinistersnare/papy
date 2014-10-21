extern crate papy;

#[cfg(not(test))]
use papy::{add_item, PapyState};

#[cfg(not(test))]
fn main() {

    //TODO REPL support
        //for stdin_line in std::io::stdin().lines() {
        //   let line = match stdin_line {
        //        Ok(line) => line,
        //        Err(e) => fail!("unexpected input \"{}\". Exiting...", e)
        //    };
        //    program.add_instruction(line)
        //}

    let lines = vec![
        "# def add x y begin x y + end",
        "3",
        "4",
        "5",
        "+",
        "*",
        "3",
        "switch",
        "/",
    ];

    let final_state = lines.into_iter()
        .fold(PapyState::new(), |old_state, line| add_item(old_state, line));

    println!("final stack: {}", final_state.tokens);
}

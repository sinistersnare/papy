#![feature(phase)]
extern crate papy;

#[phase(plugin)] extern crate brainfuck_macros;

#[cfg(not(test))]
use papy::{add_item, PapyState};
use std::io;

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
        "# def add x y: x y + end",
        "3",
        "4",
        "5",
        "+",
        "*",
        "3",
        "switch",
        "/"
    ];

    let final_state = lines.into_iter()
        .fold(PapyState::new(), |old_state, line| add_item(old_state, line));

    println!("final stack: {}", final_state.tokens);

    let hello_world = brainfuck!{
        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>
        ---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
    };

    hello_world(&mut io::stdin(), &mut io::stdout()).unwrap();

}

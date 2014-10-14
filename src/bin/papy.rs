#![feature(if_let)]
extern crate papy;

#[cfg(not(test))]
use papy::{LangString, LangName, LangNumber,Other,
           run_program, SymbolTable, tokenize_str};
#[cfg(not(test))]
fn main() {
    let mut program = PapyProgram::new();

    for line in std::io::stdin().lines() {
        program.add_instruction(line)
    }
    let lines = vec!["#def thing x y: x y +",
                     "1",
                     "2",
                     "+",
                     "3",
                     "+",
                     "# poop",
                    ];

    let mut tokens = vec![];
    for line in lines.into_iter() {
        tokens.extend(tokenize_str(line).into_iter())
    }
    println!("SMALLER: {}", tokens);
    let mut others: Vec<&str> = vec![];
    for token in tokens.iter() {
        if let Other(tok) = *token {
            others.push(tok);
        }
    }
    if !others.is_empty() {
        for token in others.iter() {
            println!("{} is not a valid token", token);
        }
        fail!("failed for having invalid tokens: {}", others);
    }

    let symbol_table = create_symbol_table(tokens.clone()); //.clone() bad?
    let answer = run_program(&mut tokens, symbol_table);

    match answer {
        LangNumber(num) => println!("program returns {}",num),
        LangString(string) => println!("program returns {}",string),
        LangName(name) => println!("program returns {}",name),
    }
}

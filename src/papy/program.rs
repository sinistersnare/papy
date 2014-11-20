
use interpreter::{SymbolTable, scan_str, Token, run_stack};
use interpreter::Token::{Definition, Item, Comment};

#[deriving(Show)]
pub struct PapyState<'a> {
    pub tokens: Vec<Token<'a>>,
    pub symbols: SymbolTable<'a>,
}

impl<'a> PapyState<'a> {
    pub fn new() -> PapyState<'a> {
        PapyState {
            tokens: vec![],
            symbols: SymbolTable::new(),
        }
    }
}

impl<'a> Clone for PapyState<'a> {
    fn clone(&self) -> PapyState<'a> {
        PapyState {
            tokens: self.tokens.clone(),
            symbols: self.symbols.clone(),
        }
    }
}

pub fn add_item<'a>(mut state: PapyState<'a>, line: &'a str) -> PapyState<'a> {
    let token = scan_str(line);
    println!("{}", token);
    match token {
        Definition(..) => {
            state.symbols.add_symbol(&token);
            state
        },
        Item(_) => {
            state.tokens.push(token.clone());
            state.tokens = run_stack(state.tokens, &state.symbols);
            state
        },
        Comment => state,
    }
}

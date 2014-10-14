use interpreter::{SymbolTable, tokenize_str, Token, run_stack, Definition, Item, Comment};

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

pub fn add_item<'a>(line: &'a str, mut state: PapyState<'a>) -> PapyState<'a> {
    let token = tokenize_str(line);
    state.tokens.push(token.clone());
    match token {
        Definition(..) => {
            state.symbols.add_symbol(&token);
            state
        },
        Item(_) => {
            state.tokens = run_stack(state.tokens.clone(), &state.symbols.clone());
            state
        },
        Comment(_) => state,
    }
}

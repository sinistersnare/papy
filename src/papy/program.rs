use interpreter::{SymbolTable, tokenize_str, Token, run_stack, Definition, Item, Comment};

pub struct PapyProgram<'a> {
    table: SymbolTable<'a>,
    stack: Vec<Token<'a>>,
}

impl<'a> PapyProgram<'a> {
    pub fn new() -> PapyProgram<'a>{
        PapyProgram {
            table: SymbolTable::new(),
            stack: vec![],
        }
    }

    /// adds the instruction to the stack, and executes the whole program.
    pub fn add_instruction(&'a mut self, line: &'a str) {
        let token = tokenize_str(line);
        match token {
            Definition(..) => {
                self.table.add_symbol(&token)
            }
            Item(_) => {
                self.stack.push(token)
            }
            Comment(_) => { }
        }
        self.stack = run_stack(self.stack.clone(), &self.table);
    }
}

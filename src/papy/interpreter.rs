#![feature(phase, if_let, default_type_params, struct_variant, slicing_syntax)]

use std::collections::HashSet;
use std::hash::{Hash, Writer};
use std::fmt;

#[deriving(Show, PartialEq, Eq, Clone)]
pub enum Token<'a> {
    Definition {
        name: &'a str,
        arity: u32,
        body: &'a str,
    },
    Item(LangItem<'a>),
    Comment(&'a str),
    Other(&'a str),
}

#[deriving(PartialEq, Eq, Show, Clone)]
pub enum LangItem<'a> {
    LangNumber(i32),
    LangString(&'a str),
    LangName(&'a str),
}

pub struct Symbol<'a> {
    name: &'a str,
    arity: u32,
    function: fn(args: &Vec<LangItem<'a>>, symbols: &SymbolTable<'a>) -> LangItem<'a>,
}

impl<'a> PartialEq for Symbol<'a> {
    fn eq(&self, other: &Symbol) -> bool {
        self.name == other.name && self.arity == other.arity
    }
}

impl<'a, H: Writer> Hash<H> for Symbol<'a> {
 fn hash(&self, state: &mut H) {
    //TODO: learn2Hash, need to distribute moar i guess... stract says so, so it must be.
    self.name.hash(state);// AM I DOING THIS RIGHT? HOW DO HASH
 }
}

impl<'a> Eq for Symbol<'a> {}

impl<'a> fmt::Show for Symbol<'a> {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Symbol {{name: {} }}", self.name)
    }
}

pub struct SymbolTable<'a> {
        symbols: HashSet<Symbol<'a>>,
}

impl<'a> SymbolTable<'a> {
    fn new() -> SymbolTable {
        let mut symbols: HashSet<Symbol<'a>> = HashSet::new();
        ////// builtin functions
        symbols.insert(Symbol {
            name: "+",
            arity: 2,
            function: { //need moar unboxed closures
                fn func<'a>(args: &Vec<LangItem<'a>>, symbols: SymbolTable) -> LangItem<'a> {
                    match (args[0], args[1]) {
                        (LangNumber(x), LangNumber(y)) => LangNumber(x + y),
                        (LangString(_), LangString(_)) => fail!("cant add strings! They also dont exist yet!"),
                        (_, _) => fail!("types need to be the same! Not LangNumber + LangNumber, or LangString + LangString!"),
                    }
                }
                func
            },
        });
        SymbolTable {
            symbols: symbols,
        }
    }

    fn add_symbol<'a>(&mut self, token: LangItem<'a>) {
        let symbol = Symbol {
            name: name,
            arity: arity,
            function: { //NEED MOAR UNBOXED CLOSURES to get rid of the symbols arg.
                fn func<'a>(args: &Vec<LangItem<'a>>, symbols: &SymbolTable<'a>) -> LangItem<'a> {
                    let mut local_stack: Vec<LangItem<'a>> = vec![];
                    for item in args.iter() {
                        match *item {
                            LangNumber(num) => local_stack.push(LangNumber(num)),
                            LangString(string) => local_stack.push(LangString(string)),
                            LangName(name) => {
                                let sym = match self.symbols.get(name) {
                                    Some(sym) => sym,
                                    None => fail!("symbol \"{}\" not found! ", name)
                                };
                                let mut args = vec![];
                                for i in range(0, sym.arity) {
                                    args.push(local_stack.pop());
                                }
                                (sym.function)(&args, self);
                            },
                        }
                    }
                };
                func
            }
        };

    }
    fn contains_name(&self, name: &'a str ) -> bool {
        self.symbols.iter().any(|symbol| symbol.name == name)
    }
    fn get(&self, name: &'a str) -> Option<Symbol<'a>> {
        *self.symbols.iter().filter(|symbol| symbol.name == name).last()
    }
}
pub fn tokenize_str<'a>(text: &'a str) -> Vec<Token<'a>> {
    let mut result = vec![];
    for cap in regex!(
                r##"(?:(?P<definition>^def .*:.*end)|(?P<comment>#.*)|(?P<item>\s?^[\w\+-\*\?!]*)|(?P<other>\W+))\s*"##
            ).captures_iter(text) {
        let token =
            if cap.pos(1).is_some() { //iterators instead?
                let whole_def = cap.name("definition").trim();
                let parts: Vec<&'a str> = whole_def.split_str(":").collect();
                let name = parts[0].split_str(" ").collect::<Vec<&'a str>>()[1]; // def NAME what ever
                let arity = parts[0].split_str(" ").collect::<Vec<&'a str>>()[2..].len() as u32;
                println!("args: {}, arity: {}", parts[0].split_str(" ").collect::<Vec<&'a str>>()[2..], arity)
                let body = parts[1];
                Definition {
                    name: name,
                    arity: arity,
                    body: body,
                }
            }
            else if cap.pos(2).is_some() {
                Comment(cap.name("comment").trim())
            }
            else if cap.pos(3).is_some() {
                let item = cap.name("item");
                match from_str(item) { //TODO string support
                    Some(val) => Item(LangNumber(val)),
                    None => Item(LangName(item)),
                }
            }
            else  {
                fail!("unknown token in \"{}\". try again!", text)
            };

    }
    result
}



pub fn run_program<'a>(tokens: &'a mut Vec<Token>, symbol_table: SymbolTable<'a>) -> LangItem<'a>{
    let mut stack: Vec<LangItem<'a>> = vec![];
    for token in tokens.iter() {
        if let Item(item) = *token {
            match item {
                LangNumber(num) => {stack.push(LangNumber(num))},
                LangString(string) => {stack.push(LangString(string))},
                LangName(name) => {
                    if !symbol_table.contains_name(name) { // making this line was a lot of work D:
                        fail!("undefined symbol: \"{}\". aborting!", name)
                    }
                    let mut args = vec![];
                    let symbol = symbol_table.get(name).unwrap(); //FIXME unwrap
                    for i in range(0, symbol.arity) {
                        args.push(match stack.pop() {
                            Some(x) => { x },
                            None => fail!("failure to pop from stack!"),
                        });
                    }
                    let result = (symbol.function)(&args, symbol_table);
                    stack.push(result);
                }
            }
        }
    }
    stack.pop().unwrap()
}

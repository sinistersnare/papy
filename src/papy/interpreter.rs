use std::collections::HashSet;
use std::hash::{Hash, Writer};
use std::fmt;
use self::Token::{Definition, Item, Comment};
use self::PapyItem::{PapyNumber, PapyString, PapyName};
#[deriving(Show, PartialEq, Clone)]
pub enum Token<'a> {
    Definition {
        name: &'a str,
        arity: uint,
        body: Vec<&'a str>,
    },
    Item(PapyItem<'a>),
    Comment,
    // Comment(&'a str),
}
#[deriving(PartialEq, Show, Clone)]
pub enum PapyItem<'a> {
    PapyNumber(i32),
    PapyString(&'a str),
    PapyName(&'a str),
}

struct Symbol<'a> {
    name: &'a str,
    arity: uint,
    function: fn(name: &'a str, arity: uint, body: Vec<Token<'a>>, symbols: &SymbolTable<'a>) -> Vec<Token<'a>>,
}

pub struct SymbolTable<'a> {
    symbols: HashSet<Symbol<'a>>,
}

impl<'a> Eq for Symbol<'a> {}

impl<'a> Clone for Symbol<'a> {
    fn clone(&self) -> Symbol<'a> {
        Symbol {
            name: self.name.clone(),
            arity: self.arity.clone(),
            function: self.function,
        }
    }
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

impl<'a> fmt::Show for Symbol<'a> {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Symbol {{name: {} }}", self.name)
    }
}

impl<'a> fmt::Show for SymbolTable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SymbolTable {{symbols: {}}}", self.symbols)
    }
}

impl<'a> Clone for SymbolTable<'a> {
    fn clone(&self) -> SymbolTable<'a> {
        SymbolTable {
            symbols: self.symbols.clone()
        }
    }
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> SymbolTable<'a> {
        let mut symbols: HashSet<Symbol<'a>> = HashSet::new();
        ////// builtin functions
        symbols.insert(Symbol {
            name: "+",
            arity: 2,
            function: { //need moar unboxed closures
                fn func<'a>(_name: &'a str, _arity: uint, body: Vec<Token<'a>>, _symbols: &SymbolTable<'a>) -> Vec<Token<'a>> {
                    let mut it = body.iter();
                    let first = it.next().unwrap();
                    let second= it.next().unwrap();
                    match (first, second) {
                        (&Item(PapyNumber(x)), &Item(PapyNumber(y))) => vec![Item(PapyNumber(x + y))],
                        _ => panic!("+ can not be applied to {} and {}", first, second),
                    }
                };

                func
            },
        });
        symbols.insert(Symbol {
            name: "*",
            arity: 2,
            function: {
                fn func<'a>(_name: &'a str, _arity: uint, body: Vec<Token<'a>>, _symbols: &SymbolTable<'a>) -> Vec<Token<'a>> {
                    let mut it = body.iter();
                    let first = it.next().unwrap();
                    let second= it.next().unwrap();
                    match (first, second) {
                        (&Item(PapyNumber(x)), &Item(PapyNumber(y))) => vec![Item(PapyNumber(x * y))],
                        _ => panic!("+ can not be applied to {} and {}", first, second),
                    }
                };                func
            },
        });
        symbols.insert(Symbol {
            name: "/",
            arity: 2,
            function: {
                fn func<'a>(_name: &'a str, _arity: uint, body: Vec<Token<'a>>, _symbols: &SymbolTable<'a>) -> Vec<Token<'a>> {
                    let mut it = body.iter();
                    let first = it.next().unwrap();
                    let second= it.next().unwrap();
                    println!("{} / {}", first, second)
                    match (first, second) {
                        (&Item(PapyNumber(x)), &Item(PapyNumber(y))) => vec![Item(PapyNumber(x / y))],
                        _ => panic!("+ can not be applied to {} and {}", first, second),
                    }
                };
                func
            },
        });
        symbols.insert(Symbol {
            name: "-",
            arity: 2,
            function: {
                fn func<'a>(_name: &'a str, _arity: uint, body: Vec<Token<'a>>, _symbols: &SymbolTable<'a>) -> Vec<Token<'a>> {
                    let mut it = body.iter();
                    let first = it.next().unwrap();
                    let second= it.next().unwrap();
                    match (first, second) {
                        (&Item(PapyNumber(x)), &Item(PapyNumber(y))) => vec![Item(PapyNumber(x - y))],
                        _ => panic!("+ can not be applied to {} and {}", first, second),
                    }
                };
                func
            },
        });
        symbols.insert(Symbol {
            name: "switch",
            arity: 2,
            function: {
                fn func<'a>(_name: &'a str, _arity: uint, body: Vec<Token<'a>>, _symbols: &SymbolTable<'a>) -> Vec<Token<'a>> {
                    let mut it = body.into_iter();
                    let first = it.next().unwrap();
                    let second= it.next().unwrap();
                    vec![first, second]
                };
                func
            },
        });

        SymbolTable {
            symbols: symbols,
        }
    }

    ///Takes a Definition Token and adds it to `self`
    pub fn add_symbol(&mut self, token: &Token<'a>) {

        self.symbols.insert(Symbol {
            //FIXME ugh
            name: match *token {
                Definition(name, _, _) => name,
                _ => panic!("token \"{}\" is not a definition!", token),
            },
            arity: match *token {
                Definition(_, arity, _) => arity,
                _ => panic!("token \"{}\" is not a definition!", token),
            },
            function: { //NEED MOAR UNBOXED CLOSURES to get rid of the symbols arg.
                fn func<'a>(_name: &'a str, _arity: uint, _body: Vec<Token<'a>>, _symbols: &SymbolTable<'a>) -> Vec<Token<'a>> {
                    let local_stack: Vec<Token<'a>> = vec![];
                    local_stack
                };
                func
            },

        });
    }
    fn contains_name(&self, name: &'a str ) -> bool {
        self.symbols.iter().any(|symbol| symbol.name == name)
    }
    fn get(&self, name: &'a str) -> Symbol<'a> { //TODO use Option instead of failure.
        match self.symbols.iter().filter(|symbol| symbol.name == name).last() {
            Some(val) => *val,
            None => panic!("symbol with name \"{}\" does not exist!", name)
        }
    }
}

/// Takes in an Str and returns a token representation of it
pub fn scan_str<'a, S: Str>(text: S) -> Token<'a> {
    match scan! {
        text,

        "#", .._tail => {
            //TODO do we need to have a Comment(&str)?
            Comment
        },

        #[tokenizer="SpaceDelimited"]
        "def" name arity ":" [(?!"end") body_tokens]* "end" => {
            Definition {
                name: name,
                arity: arity,
                body: body_tokens,
            }
        },
        num => Item(PapyNumber(num)),
        "`" string:&str "`" => {
            Item(PapyString(string))
        },

        name => {
            Item(PapyName(name))
        },

    } {
        Ok(tok) => tok,
        Err(reason) => panic!("could not parse input string: {}", reason)
    }
}


pub fn run_stack<'a>(tokens: Vec<Token<'a>>, symbol_table: &SymbolTable<'a>) -> Vec<Token<'a>>{
    let mut stack: Vec<Token<'a>> = vec![];
    for token in tokens.into_iter() {
        if let Item(item) = token {
            match item {
                PapyNumber(num) => {stack.push(Item(PapyNumber(num)))},
                PapyString(string) => {stack.push(Item(PapyString(string)))},
                PapyName(name) => {
                    if !symbol_table.contains_name(name) {
                        panic!("undefined symbol: \"{}\". aborting!", name)
                    }
                    let mut args: Vec<Token<'a>> = vec![];
                    let symbol = symbol_table.get(name);
                    for _ in range(0, symbol.arity) {
                        args.push(match stack.pop() {
                            Some(Item(PapyName(name))) => {
                                Item(PapyName(name))
                            }
                            Some(Item(x)) => {
                                Item(x)
                            },
                            Some(_) => panic!("HOW DID WE GET HERE!"),
                            None => panic!("failure to pop from stack!"),
                        });
                    }
                    let result = (symbol.function)(symbol.name, symbol.arity, args, symbol_table);
                    stack.extend(result.into_iter());
                }
            }
        }
    }
    stack
}

#[test]
fn test_tokenizer() {

    println!("def: {}", scan_str("def thing 2: %0 %1 + end"));
    assert!(scan_str("def thing 2: %0 %1 + end") == Definition {
        name: "thing",
        arity: 2,
        body: vec!["%0", "%1", "+"],
    })
    assert!(scan_str("# def thing 2: %0 %1 + end") == Comment)
    assert!(scan_str("1") == Item(PapyNumber(1)))
    assert!(scan_str("name") == Item(PapyName("name")))

}

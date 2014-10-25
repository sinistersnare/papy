use std::collections::HashSet;
use std::hash::{Hash, Writer};
use std::fmt;

#[deriving(Show, PartialEq, Clone)]
pub enum Token<'a> {
    Definition {
        name: &'a str,
        arity: uint,
        body: Vec<&'a str>,
    },
    Item(LangItem<'a>),
    Comment,
    // Comment(&'a str),
}

#[deriving(PartialEq, Show, Clone)]
pub enum LangItem<'a> {
    PapyNumber(i32),
    PapyString(&'a str),
    PapyName(&'a str),
}

pub struct Argument<'a> {
    name: &'a str,
    value: LangItem<'a>,
}

struct Symbol<'a> {
    name: &'a str,
    arity: uint,
    function: fn(name: &'a str, args: &Vec<Argument<'a>>, symbols: &SymbolTable<'a>) -> Vec<Token<'a>>,
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
                fn func<'a>(_name: &'a str, args: &Vec<Argument<'a>>, _symbols: &SymbolTable) -> Vec<Token<'a>> {
                    match (args[0], args[1]) {
                        (Argument {name: _, value: PapyNumber(x)}, Argument {name: _,value: PapyNumber(y)}) => {
                            vec![Item(PapyNumber(x + y))]
                        },
                        (Argument {name: _, value: PapyString(_) }, Argument {name: _, value: PapyString(_)}) => fail!("cant add strings! They also dont exist yet!"),
                        (_, _) => fail!("types need to be the same! Not PapyNumber + PapyNumber, or PapyString + PapyString!"),
                    }
                };
                func
            },
        });
        symbols.insert(Symbol {
            name: "*",
            arity: 2,
            function: {
                fn func<'a>(_name: &'a str, args: &Vec<Argument<'a>>, _symbols: &SymbolTable) -> Vec<Token<'a>> {
                    match (args[0], args[1]) {
                        (Argument {name: _, value: PapyNumber(x)}, Argument {name: _,value: PapyNumber(y)}) => {
                            vec![Item(PapyNumber(x * y))]
                        },
                        (Argument {name: _, value: PapyString(_) }, Argument {name: _, value: PapyString(_)}) => fail!("cant add strings! They also dont exist yet!"),
                        (_, _) => fail!("types need to be the same! Not PapyNumber + PapyNumber, or PapyString + PapyString!"),
                    }
                };
                func
            },
        });
        symbols.insert(Symbol {
            name: "/",
            arity: 2,
            function: {
                fn func<'a>(_name: &'a str, args: &Vec<Argument<'a>>, _symbols: &SymbolTable) -> Vec<Token<'a>> {
                    match (args[0], args[1]) {
                        (Argument {name: _, value: PapyNumber(x)}, Argument {name: _,value: PapyNumber(y)}) => {
                            vec![Item(PapyNumber(x / y))]
                        },
                        (Argument {name: _, value: PapyString(_) }, Argument {name: _, value: PapyString(_)}) => fail!("cant add strings! They also dont exist yet!"),
                        (_, _) => fail!("types need to be the same! Not PapyNumber + PapyNumber, or PapyString + PapyString!"),
                    }
                };
                func
            },
        });
        symbols.insert(Symbol {
            name: "-",
            arity: 2,
            function: {
                fn func<'a>(_name: &'a str, args: &Vec<Argument<'a>>, _symbols: &SymbolTable) -> Vec<Token<'a>> {
                    match (args[0], args[1]) {
                        (Argument {name: _, value: PapyNumber(x)}, Argument {name: _,value: PapyNumber(y)}) => {
                            vec![Item(PapyNumber(x - y))]
                        },
                        (Argument {name: _, value: PapyString(_) }, Argument {name: _, value: PapyString(_)}) => fail!("cant add strings! They also dont exist yet!"),
                        (_, _) => fail!("types need to be the same! Not PapyNumber + PapyNumber, or PapyString + PapyString!"),
                    }
                };
                func
            },
        });
        symbols.insert(Symbol {
            name: "switch",
            arity: 2,
            function: {
                fn func<'a>(_name: &'a str, args: &Vec<Argument<'a>>, _symbols: &SymbolTable) -> Vec<Token<'a>> {
                    match (args[0], args[1]) {
                        (Argument {name: _, value: PapyNumber(x)}, Argument {name: _,value: PapyNumber(y)}) => {
                            vec![Item(PapyNumber(x)), Item(PapyNumber(y))]
                        },
                        (Argument {name: _, value: PapyString(_) }, Argument {name: _, value: PapyString(_)}) => fail!("cant add strings! They also dont exist yet!"),
                        (_, _) => fail!("types need to be the same! Not PapyNumber + PapyNumber, or PapyString + PapyString!"),
                    }
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
                _ => fail!("token \"{}\" is not a definition!", token),
            },
            arity: match *token {
                Definition(_, arity, _) => arity,
                _ => fail!("token \"{}\" is not a definition!", token),
            },
            function: { //NEED MOAR UNBOXED CLOSURES to get rid of the symbols arg.
                fn func<'a>(_name: &'a str, args: &Vec<Argument<'a>>, symbols: &SymbolTable<'a>) -> Vec<Token<'a>> {
                    let mut local_stack: Vec<Token<'a>> = vec![];
                    for arg in args.iter() {
                        match *arg {
                            Argument {name: _, value: PapyNumber(num)} => local_stack.push(Item(PapyNumber(num))),
                            Argument {name: _, value: PapyString(string)}=> local_stack.push(Item(PapyString(string))) ,
                            Argument {name: arg_name, value: PapyName(name)} => {
                                let sym = symbols.get(name);
                                let mut local_args = vec![];
                                for _i in range(0, sym.arity) {
                                    local_args.push(match local_stack.pop() {
                                        Some(Item(val)) => {
                                            Argument {
                                                name: arg_name,
                                                value: val
                                            }
                                        },
                                        Some(_) => fail!("HOW DID WE GET HERE"),
                                        None => fail!("couldnt pop from local stack! not enough args")

                                    });
                                }
                                (sym.function)(sym.name, &local_args, symbols);
                            },
                        }
                    }
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
            None => fail!("symbol with name \"{}\" does not exist!", name)
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
        Err(reason) => fail!("could not parse input string: {}", reason)
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
                        fail!("undefined symbol: \"{}\". aborting!", name)
                    }
                    let mut args = vec![];
                    let symbol = symbol_table.get(name);
                    for _ in range(0, symbol.arity) {
                        args.push(match stack.pop() {
                            Some(Item(x)) => {
                                println!("X IS {}", x)
                                Argument {
                                    name: "",
                                    value: x,
                                }
                            },
                            Some(_) => fail!("HOW DID WE GET HERE!"),
                            None => fail!("failure to pop from stack!"),
                        });
                    }
                    let result = (symbol.function)(name, &args, symbol_table);
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

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
}

#[deriving(PartialEq, Eq, Show, Clone)]
pub enum LangItem<'a> {
    PapyNumber(i32),
    PapyString(&'a str),
    PapyName(&'a str),
}

struct Symbol<'a> {
    name: &'a str,
    arity: u32,
    function: fn(args: &Vec<Token<'a>>, symbols: &SymbolTable<'a>) -> Token<'a>,
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
    pub fn new() -> SymbolTable<'a> {
        let mut symbols: HashSet<Symbol<'a>> = HashSet::new();
        ////// builtin functions
        symbols.insert(Symbol {
            name: "+",
            arity: 2,
            function: { //need moar unboxed closures
                fn func<'a>(args: &Vec<Token<'a>>, _symbols: &SymbolTable) -> Token<'a> {
                    match (args[0], args[1]) {
                        (Item(PapyNumber(x)), Item(PapyNumber(y))) => Item(PapyNumber(x + y)),
                        (Item(PapyString(_)), Item(PapyString(_))) => fail!("cant add strings! They also dont exist yet!"),
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
                fn func<'a>(args: &Vec<Token<'a>>, symbols: &SymbolTable<'a>) -> Token<'a> {
                    let mut local_stack: Vec<Token<'a>> = vec![];
                    for item in args.iter() {
                        match *item {
                            Item(PapyNumber(num)) => local_stack.push(Item(PapyNumber(num))),
                            Item(PapyString(string)) => local_stack.push(Item(PapyString(string))),
                            Item(PapyName(name)) => {
                                let sym = symbols.get(name);
                                let mut args = vec![];
                                for i in range(0, sym.arity) {
                                    args.push(local_stack.pop().unwrap()); //FIXME unwrap
                                }
                                (sym.function)(&args, symbols);
                            },
                            _ => fail!("not an item! WATCHU DOIN WITH {}", item)
                        }
                    }
                    assert!(local_stack.len() == 1); // needs to be one size? correct?
                    local_stack.pop().unwrap()
                };
                func
            }
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
pub fn tokenize_str<'a>(text: &'a str) -> Token<'a> {
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
                    Some(val) => Item(PapyNumber(val)),
                    None => Item(PapyName(item)),
                }
            }
            else  {
                fail!("unknown token in \"{}\". try again!", text)
            };

    }
    result.pop().unwrap() //FIXME this function should return one Token, and dont need the pop.unwrap
}



pub fn run_stack<'a>(tokens: Vec<Token<'a>>, symbol_table: &SymbolTable<'a>) -> Vec<Token<'a>>{
    let mut stack: Vec<Token<'a>> = vec![];
    for token in tokens.into_iter() {
        if let Item(item) = token {
            match item {
                PapyNumber(num) => {stack.push(Item(PapyNumber(num)))},
                PapyString(string) => {stack.push(Item(PapyString(string)))},
                PapyName(name) => {
                    if !symbol_table.contains_name(name) { // making this line was a lot of work D:
                        fail!("undefined symbol: \"{}\". aborting!", name)
                    }
                    let mut args = vec![];
                    let symbol = symbol_table.get(name);
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
    stack
}

#![feature(phase, struct_variant)]

#[phase(plugin)] extern crate scan;
extern crate scan_util;

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

/// takes a line of input and returns a Vec<Token> from the input
/// tokenize_str("32") => Item(LangItem(32))
/// tokenize_str("def add x y: x y +") => ????
pub fn tokenize_str<'a>(text: &'a str) -> Token<'a> {
    match scan! {
        text,
        "def " name:&str " " [args:&str]* ":" body:&str => Definition {name: name, arity: 2, body: body},
        num:i32 => Item(LangItem(num)),
        "#" comment:&str => Comment(comment)
    } {
        Ok(value) => value,
        Err(err) => fail!("couldnt parse \"{}\" into a token", text)
    }

}

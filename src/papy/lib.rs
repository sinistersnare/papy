#![feature(phase, if_let, default_type_params, struct_variant)]

extern crate regex;
#[phase(plugin)] extern crate regex_macros;

pub use program::{add_item, PapyState};

mod program;
mod interpreter;


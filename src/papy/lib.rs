#![feature(phase, if_let, default_type_params, struct_variant, slicing_syntax)]

#[phase(plugin)] extern crate regex_macros;
extern crate regex;
#[phase(plugin)] extern crate spellck;
#[phase(plugin)] extern crate scan;
extern crate scan_util;

pub use program::{add_item, PapyState};

// pub use interpreter;

mod program;
mod interpreter;

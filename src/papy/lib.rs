#![feature(phase, if_let, default_type_params, slicing_syntax)]

#![spellck_extra_words="papy"]

#[phase(plugin)] extern crate regex_macros;
extern crate regex;
#[phase(plugin)] extern crate spellck;
#[phase(plugin)] extern crate scan;
extern crate scan_util;

pub use program::{add_item, PapyState};

mod program;
mod interpreter;

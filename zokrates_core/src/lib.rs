#![feature(box_patterns, box_syntax)]

extern crate num;
extern crate num_bigint;
extern crate reduce; // better reduce function than Iter.fold
extern crate serde; // serialization deserialization
extern crate serde_json;
extern crate typed_arena;
#[macro_use]
extern crate serde_derive;
extern crate bellman_ce as bellman;
extern crate bincode;
extern crate csv;
extern crate ff_ce as ff;
extern crate hex;
extern crate lazy_static;
extern crate pairing_ce as pairing;
extern crate regex;
extern crate zokrates_common;
extern crate zokrates_field;
extern crate zokrates_pest_ast;

mod embed;
mod flatten;
pub mod imports;
mod macros;
mod optimizer;
mod parser;
mod semantics;
mod solvers;
mod static_analysis;
mod zir;

pub mod absy;
pub mod compile;
pub mod flat_absy;
pub mod ir;
pub mod proof_system;
pub mod typed_absy;

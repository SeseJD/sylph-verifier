#![warn(unused_extern_crates)]
#![recursion_limit="128"]
#![feature(box_syntax, box_patterns, never_type, integer_atomics, optin_builtin_traits,
           const_fn, const_atomic_bool_new, const_atomic_usize_new, conservative_impl_trait)]
#![deny(unused_must_use)]

extern crate backtrace;
extern crate byteorder;
extern crate chrono;
extern crate constant_time_eq;
extern crate fs2;
extern crate hmac;
extern crate linefeed;
extern crate parking_lot;
extern crate percent_encoding;
extern crate r2d2;
extern crate rand;
extern crate regex;
extern crate reqwest;
extern crate rusqlite;
extern crate serde_json;
extern crate serenity;
extern crate sha2;
extern crate uuid;

#[allow(unused_extern_crates)] extern crate serde;

#[macro_use] extern crate enumset;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate nom;
#[macro_use] extern crate serde_derive;

#[macro_use] mod errors;

mod commands;
mod core;
mod error_report;
mod logger;
mod roblox;
mod startup;
mod util;

fn main() {
    println!("Sylph-Verifier v{} by LymeeFairy", env!("CARGO_PKG_VERSION"));
    println!("Licenced under the Apache license, version 2.0");
    println!();

    startup::start();
    std::process::exit(0); // Just in case.
}

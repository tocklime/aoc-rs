#![warn(
     clippy::all,
     clippy::cargo,
     clippy::pedantic,
     clippy::nursery,
 )]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::implicit_hasher)]

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

#[macro_use]
extern crate lazy_static;

pub mod sec1;
pub mod sec2;
pub mod sec3;
pub mod sec4;
pub mod sec5;
pub mod utils;

aoc_lib! { year = 2020 }

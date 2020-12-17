#![warn(
     clippy::all,
     clippy::cargo,
     clippy::pedantic,
     clippy::nursery,
 )]

#![allow(clippy::must_use_candidate)] //doesn't play nice with aoc part functions.
#![allow(clippy::cargo_common_metadata)] //I don't want to add this metadata.
#![allow(clippy::implicit_hasher)] //I don't want to deal with this.
#![allow(clippy::filter_map)] //I disagree with this lint - .filter().map() is often nicer than .filter_map()

#[allow(unused_macros)]
macro_rules! ix {
    ($e : expr) => {
        {let val = $e; debug_assert!(val >= 0); (val as usize)}
    };
}

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

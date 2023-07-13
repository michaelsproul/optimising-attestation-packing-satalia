#![feature(int_roundings)]
#![feature(iter_collect_into)]
#![feature(extract_if)]
#![feature(hash_extract_if)]

pub mod aapp;
pub mod algorithms;
pub mod approaches;

pub use aapp::instance::*;
pub use algorithms::{bron_kerbosh, WeightedMaximumCoverage};

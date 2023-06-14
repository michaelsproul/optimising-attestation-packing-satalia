#![feature(int_roundings)]
#![feature(iter_collect_into)]
#![feature(drain_filter)]
#![feature(option_result_contains)]
#![feature(hash_drain_filter)]

pub mod aapp;
pub mod algorithms;
pub mod approaches;

pub use aapp::instance::*;
pub use algorithms::{bron_kerbosh, WeightedMaximumCoverage};

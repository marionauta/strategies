//! A Rust library that implements many algorithm-solving strategies so you
//! don't have to. You'll only need to define your problem.
//!
//! Each module contains a strategy and its documentation:

pub mod bt;
pub mod dac;

/// Basic problem type: maximization, minimization or all.
///
/// This will define the solution you get. The same problem but with different
/// _types_ will get very different solutions. It depends on what your problem
/// sees as **best** and **worst**.
///
#[derive(PartialEq, Eq)]
pub enum Type {
    /// If you want to maximize a value (e.g. business' earnings).
    All,

    /// If you want to minimize (e.g. travel time).
    Max,

    /// If it isn't an optimization problem (i.e. we want all the valid solutions
    ///   we find) then use `All`.
    Min,
}

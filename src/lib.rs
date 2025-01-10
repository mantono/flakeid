#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
extern crate serde_test;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
/// Module which contains logic for generation of flake identifiers
pub mod gen;

/// Module for the [id::Flake] struct, i.e. the representation of the flake identifier
pub mod id;
mod seq;

#[cfg(feature = "serde")]
pub mod serde;

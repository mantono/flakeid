#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
extern crate serde_test;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod gen;
pub mod id;
mod seq;

#[cfg(feature = "serde")]
pub mod serde;

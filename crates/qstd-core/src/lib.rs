#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod aliases;
pub mod allocate;
pub mod arrays;
pub mod bits;
pub mod by;
pub mod clifford;
pub mod clifford_t;
pub mod decomposed;
pub mod decomposition;
pub mod machine;
pub mod macros;
pub mod measure;
pub mod pauli;
pub mod prelude;
pub mod quantum;
pub mod release;
pub mod rotations;
pub mod strictness;

#[cfg(any(feature = "std", feature = "alloc"))]
pub mod vec;

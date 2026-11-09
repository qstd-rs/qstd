use trait_aliases::trait_aliases;

use crate::{allocate::Allocate, measure::Measure, release::Release};

trait_aliases! {
    #[trait_alias(
        Q,
        doc = "Implemented for all types that are [`Allocate`], [`Measure`] and [`Release`]."
    )]
    pub trait Quantum = Allocate + Measure + Release;
}

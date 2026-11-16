use crate::machine::{Machine, QubitMut};

pub trait Pauli: Machine {
    fn x(&mut self, qubit: QubitMut<'_, Self>);

    fn y(&mut self, qubit: QubitMut<'_, Self>);

    fn z(&mut self, qubit: QubitMut<'_, Self>);
}

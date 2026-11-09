use crate::machine::Machine;

pub trait Pauli: Machine {
    fn x(&mut self, qubit: &mut Self::Qubit);

    fn y(&mut self, qubit: &mut Self::Qubit);

    fn z(&mut self, qubit: &mut Self::Qubit);
}

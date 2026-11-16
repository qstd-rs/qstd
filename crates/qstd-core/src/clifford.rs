use crate::{decomposition, machine::QubitMut, pauli::Pauli};

pub trait Clifford: Pauli {
    fn h(&mut self, qubit: QubitMut<'_, Self>);

    fn s(&mut self, qubit: QubitMut<'_, Self>);
    fn s_dagger(&mut self, qubit: QubitMut<'_, Self>);

    #[doc(alias = "cnot")]
    fn cx(&mut self, control: QubitMut<'_, Self>, target: QubitMut<'_, Self>);

    fn cy(&mut self, control: QubitMut<'_, Self>, target: QubitMut<'_, Self>) {
        decomposition::cy(self, control, target);
    }

    fn cz(&mut self, control: QubitMut<'_, Self>, target: QubitMut<'_, Self>) {
        decomposition::cz(self, control, target);
    }

    fn swap(&mut self, a: QubitMut<'_, Self>, b: QubitMut<'_, Self>) {
        decomposition::swap(self, a, b);
    }
}

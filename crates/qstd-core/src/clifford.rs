use crate::pauli::Pauli;

pub trait Clifford: Pauli {
    fn h(&mut self, qubit: &mut Self::Qubit);

    fn s(&mut self, qubit: &mut Self::Qubit);
    fn s_dagger(&mut self, qubit: &mut Self::Qubit);

    #[doc(alias = "cnot")]
    fn cx(&mut self, control: &mut Self::Qubit, target: &mut Self::Qubit);

    /// `CZ = (I ⊗ H) CX (I ⊗ H)`, exactly
    fn cz(&mut self, control: &mut Self::Qubit, target: &mut Self::Qubit) {
        self.h(target);

        self.cx(control, target);

        self.h(target);
    }

    /// ```
    /// fn swap(a: &mut bool, b: &mut bool) {
    ///     a ^= b;
    ///     b ^= a;
    ///     a ^= b;
    /// }
    /// ```
    fn swap(&mut self, a: &mut Self::Qubit, b: &mut Self::Qubit) {
        self.cx(a, b);
        self.cx(b, a);
        self.cx(a, b);
    }
}

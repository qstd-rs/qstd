use crate::clifford::Clifford;

pub trait CliffordT: Clifford {
    fn t(&mut self, qubit: &mut Self::Qubit);
    fn t_dagger(&mut self, qubit: &mut Self::Qubit);

    #[doc(alias = "fredkin")]
    fn cswap(&mut self, control: &mut Self::Qubit, a: &mut Self::Qubit, b: &mut Self::Qubit) {
        self.cx(b, a);
        self.ccx(control, a, b);
        self.cx(b, a);
    }

    #[doc(alias = "toffoli")]
    fn ccx(&mut self, a: &mut Self::Qubit, b: &mut Self::Qubit, t: &mut Self::Qubit) {
        self.h(t);
        self.cx(b, t);
        self.t_dagger(t);
        self.cx(a, t);
        self.t(t);
        self.cx(b, t);
        self.t_dagger(t);
        self.cx(a, t);
        self.t(b);
        self.t(t);
        self.h(t);
        self.cx(a, b);
        self.t(a);
        self.t_dagger(b);
        self.cx(a, b);
    }
}

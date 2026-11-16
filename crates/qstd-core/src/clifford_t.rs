use crate::{clifford::Clifford, decomposition, machine::QubitMut};

pub trait CliffordT: Clifford {
    fn t(&mut self, qubit: QubitMut<'_, Self>);
    fn t_dagger(&mut self, qubit: QubitMut<'_, Self>);

    fn ch(&mut self, control: QubitMut<'_, Self>, target: QubitMut<'_, Self>) {
        decomposition::ch(self, control, target);
    }

    fn cs(&mut self, control: QubitMut<'_, Self>, target: QubitMut<'_, Self>) {
        decomposition::cs(self, control, target);
    }

    fn cs_dagger(&mut self, control: QubitMut<'_, Self>, target: QubitMut<'_, Self>) {
        decomposition::cs_dagger(self, control, target);
    }

    #[doc(alias = "fredkin")]
    fn cswap(&mut self, c: QubitMut<'_, Self>, a: QubitMut<'_, Self>, b: QubitMut<'_, Self>) {
        decomposition::cswap(self, c, a, b);
    }

    #[doc(alias = "toffoli")]
    fn ccx(&mut self, a: QubitMut<'_, Self>, b: QubitMut<'_, Self>, t: QubitMut<'_, Self>) {
        decomposition::ccx(self, a, b, t);
    }
}

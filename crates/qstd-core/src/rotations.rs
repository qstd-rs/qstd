use qstd_num::{float::Float, real::Real};

use crate::{clifford_t::CliffordT, decomposition, machine::QubitMut};

pub trait Angles {
    type Float: Float;
}

pub type Angle<A> = Real<<A as Angles>::Float>;

pub trait Rotations: CliffordT + Angles {
    fn p(&mut self, phi: Angle<Self>, qubit: QubitMut<'_, Self>);

    fn ry(&mut self, theta: Angle<Self>, qubit: QubitMut<'_, Self>);

    fn u(
        &mut self,
        theta: Angle<Self>,
        phi: Angle<Self>,
        lambda: Angle<Self>,
        qubit: QubitMut<'_, Self>,
    ) {
        decomposition::u(self, theta, phi, lambda, qubit);
    }

    fn cp(&mut self, phi: Angle<Self>, control: QubitMut<'_, Self>, target: QubitMut<'_, Self>) {
        decomposition::cp(self, phi, control, target);
    }

    fn cry(&mut self, theta: Angle<Self>, control: QubitMut<'_, Self>, target: QubitMut<'_, Self>) {
        decomposition::cry(self, theta, control, target);
    }

    fn ct(&mut self, control: QubitMut<'_, Self>, target: QubitMut<'_, Self>) {
        decomposition::ct(self, control, target);
    }

    fn ct_dagger(&mut self, control: QubitMut<'_, Self>, target: QubitMut<'_, Self>) {
        decomposition::ct_dagger(self, control, target);
    }
}

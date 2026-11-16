use crate::{
    machine::{Machine, QubitMut},
    pauli::Pauli,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct Decomposed<Q: Machine + ?Sized> {
    machine: Q,
}

impl<Q: Machine + ?Sized> Decomposed<Q> {
    pub const fn from_ref(machine: &Q) -> &Self {
        todo!()
    }

    pub const fn from_mut(machine: &mut Q) -> &Self {
        todo!()
    }

    pub const fn machine(&self) -> &Q {
        &self.machine
    }

    // XXX: do not expose to public!
    const fn machine_mut(&mut self) -> &mut Q {
        &mut self.machine
    }
}

impl<Q: Machine + ?Sized> Machine for Decomposed<Q> {
    type Qubit = Q::Qubit;

    type Bit = Q::Bit;
}

impl<Q: Pauli + ?Sized> Pauli for Decomposed<Q> {
    fn x(&mut self, qubit: QubitMut<'_, Self>) {
        self.machine_mut().x(qubit);
    }

    fn y(&mut self, qubit: QubitMut<'_, Self>) {
        self.machine_mut().y(qubit);
    }

    fn z(&mut self, qubit: QubitMut<'_, Self>) {
        self.machine_mut().z(qubit);
    }
}

impl<Q: Machine + Default> Default for Decomposed<M> {
    fn default() -> Self {
        Self::new(Q::default())
    }
}

impl<Q: Machine> Decomposed<Q> {
    pub const fn new(machine: Q) -> Self {
        Self { machine }
    }

    pub fn get(self) -> Q {
        self.machine
    }
}

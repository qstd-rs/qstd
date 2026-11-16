use qstd_num::consts::frac_pi_4;

use crate::{
    clifford::Clifford,
    clifford_t::CliffordT,
    machine::QubitMut,
    rotations::{Angle, Rotations},
};

// CLIFFORD

/// `CY = (I ⊗ S) CX (I ⊗ S†)`, exactly
pub fn cy<Q: Clifford + ?Sized>(q: &mut Q, control: QubitMut<'_, Q>, target: QubitMut<'_, Q>) {
    q.s_dagger(target);

    q.cx(control, target);

    q.s(target);
}

/// `CZ = (I ⊗ H) CX (I ⊗ H)`, exactly
pub fn cz<Q: Clifford + ?Sized>(q: &mut Q, control: QubitMut<'_, Q>, target: QubitMut<'_, Q>) {
    q.h(target);

    q.cx(control, target);

    q.h(target);
}

/// ```ignore
/// fn swap(a: &mut bool, b: &mut bool) {
///     *a ^= *b;
///     *b ^= *a;
///     *a ^= *b;
/// }
/// ```
pub fn swap<Q: Clifford + ?Sized>(q: &mut Q, a: QubitMut<'_, Q>, b: QubitMut<'_, Q>) {
    q.cx(a, b);
    q.cx(b, a);
    q.cx(a, b);
}

// CLIFFORD + T

pub fn ch<Q: CliffordT + ?Sized>(q: &mut Q, control: QubitMut<'_, Q>, target: QubitMut<'_, Q>) {
    q.s(target);
    q.h(target);
    q.t(target);
    q.cx(control, target);
    q.t_dagger(target);
    q.h(target);
    q.s_dagger(target);
}

pub fn cs<Q: CliffordT + ?Sized>(q: &mut Q, control: QubitMut<'_, Q>, target: QubitMut<'_, Q>) {
    q.t(control);
    q.t(target);
    q.cx(control, target);
    q.t_dagger(target);
    q.cx(control, target);
}

pub fn cs_dagger<Q: CliffordT + ?Sized>(
    q: &mut Q,
    control: QubitMut<'_, Q>,
    target: QubitMut<'_, Q>,
) {
    q.t_dagger(control);
    q.t_dagger(target);
    q.cx(control, target);
    q.t(target);
    q.cx(control, target);
}

#[doc(alias = "fredkin")]
pub fn cswap<Q: CliffordT + ?Sized>(
    q: &mut Q,
    c: QubitMut<'_, Q>,
    a: QubitMut<'_, Q>,
    b: QubitMut<'_, Q>,
) {
    q.cx(b, a);
    q.ccx(c, a, b);
    q.cx(b, a);
}

#[doc(alias = "toffoli")]
pub fn ccx<Q: CliffordT + ?Sized>(
    q: &mut Q,
    a: QubitMut<'_, Q>,
    b: QubitMut<'_, Q>,
    t: QubitMut<'_, Q>,
) {
    q.h(t);
    q.cx(b, t);
    q.t_dagger(t);
    q.cx(a, t);
    q.t(t);
    q.cx(b, t);
    q.t_dagger(t);
    q.cx(a, t);
    q.t(b);
    q.t(t);
    q.h(t);
    q.cx(a, b);
    q.t(a);
    q.t_dagger(b);
    q.cx(a, b);
}

// ROTATIONS

pub fn u<Q: Rotations + ?Sized>(
    q: &mut Q,
    theta: Angle<Q>,
    phi: Angle<Q>,
    lambda: Angle<Q>,
    qubit: QubitMut<'_, Q>,
) {
    q.p(lambda, qubit);
    q.ry(theta, qubit);
    q.p(phi, qubit);
}

pub fn cp<Q: Rotations + ?Sized>(
    q: &mut Q,
    phi: Angle<Q>,
    control: QubitMut<'_, Q>,
    target: QubitMut<'_, Q>,
) {
    let half = phi.half();

    q.p(half, control);
    q.cx(control, target);
    q.p(-half, target);
    q.cx(control, target);
    q.p(half, target);
}

/// Not to be confused with what you do when a decomposition is only correct up to global phase.
pub fn cry<Q: Rotations + ?Sized>(
    q: &mut Q,
    theta: Angle<Q>,
    control: QubitMut<'_, Q>,
    target: QubitMut<'_, Q>,
) {
    let half = theta.half();

    q.ry(half, target);
    q.cx(control, target);
    q.ry(-half, target);
    q.cx(control, target);
}

pub fn ct<Q: Rotations + ?Sized>(q: &mut Q, control: QubitMut<'_, Q>, target: QubitMut<'_, Q>) {
    q.cp(frac_pi_4(), control, target);
}

pub fn ct_dagger<Q: Rotations + ?Sized>(
    q: &mut Q,
    control: QubitMut<'_, Q>,
    target: QubitMut<'_, Q>,
) {
    let angle: Angle<Q> = frac_pi_4();

    q.cp(-angle, control, target);
}

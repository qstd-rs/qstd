use qstd::prelude::{By, Clifford, Quantum, StateVec};

const ITERATIONS: usize = 1_000_000;

fn bell<Q: Quantum + Clifford>(q: &mut Q) -> (Q::Bit, Q::Bit) {
    let q_0 = q.qubit();
    let q_1 = q.qubit();

    q.h(q_0.by_mut());

    q.cx(q_0.by_mut(), q_1.by_mut());

    let m_0 = q.take(q_0);
    let m_1 = q.take(q_1);

    (m_0, m_1)
}

fn main() {
    let zero_zero: usize = 0;
    let zero_one: usize = 0;
    let one_zero: usize = 0;
    let one_one: usize = 0;

    let state_vector = StateVector::new();

    for _ in 0..ITERATIONS {
        let count = match bell(&mut state_vector) {
            (false, false) => zero_zero.by_mut(),
            (false, true) => zero_one.by_mut(),
            (true, false) => one_zero.by_mut(),
            (true, true) => one_one.by_mut(),
        };

        count += 1;
    }

    println!("00: {zero_zero}");
    println!("01: {zero_one}");
    println!("10: {one_zero}");
    println!("11: {one_one}");
}

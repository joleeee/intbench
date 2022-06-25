use std::ops::{Add, Div, Mul, Rem};

#[allow(dead_code)]
pub fn fib<N: Number>(times: u32) -> N {
    let mut last = N::from(1);
    let mut buffer;
    let mut current = N::from(1);

    for _ in 0..times {
        buffer = last;
        last = current;
        current = current.overflowing_add(buffer);
    }

    current
}

#[allow(dead_code)]
pub fn three_n_one<
    N: Number + PartialEq + Div<Output = N> + Mul<Output = N> + Add<Output = N> + Rem<Output = N>,
>(
    start: u32,
) -> N {
    let mut current = N::from(start);

    while current != N::from(1) {
        if current % N::from(2) == N::from(0) {
            current = current / N::from(2);
        } else {
            current = current * N::from(3) + N::from(1);
        }
    }

    current
}

pub trait Number: Clone + Copy {
    fn from(f: u32) -> Self;
    fn overflowing_add(self, rhs: Self) -> Self;
}

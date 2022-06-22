#![feature(test)]
extern crate test;

#[allow(dead_code)]
fn fib<N: Number>(times: u32) -> N{
    let mut last = N::one();
    let mut buffer;
    let mut current = N::zero();

    for _ in 0..times {
        buffer = last;
        last = current;
        current = current.overflowing_add(buffer);
    }

    current
}

trait Number: Copy {
    fn one() -> Self;
    fn zero() -> Self;
    fn overflowing_add(self, rhs: Self) -> Self;
}

impl Number for u32 {
    fn one() -> Self {
        1
    }
    fn zero() -> Self {
        0
    }
    fn overflowing_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }
}

impl Number for u128 {
    fn one() -> Self {
        1
    }
    fn zero() -> Self {
        0
    }
    fn overflowing_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }
}

impl Number for ethereum_types::U128 {
    fn overflowing_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }
    fn zero() -> Self {
        Self::zero()
    }
    fn one() -> Self {
        Self::one()
    }
}

impl Number for ethereum_types::U256 {
    fn overflowing_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }
    fn zero() -> Self {
        Self::zero()
    }
    fn one() -> Self {
        Self::one()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use test::{black_box, Bencher};

    const FIBN: u32 = 10_000;

    #[bench]
    fn fib_u32(b: &mut Bencher) {
        b.iter(|| {
            black_box(fib::<u32>(black_box(FIBN)));
        })
    }

    #[bench]
    fn fib_u128(b: &mut Bencher) {
        b.iter(|| {
            black_box(fib::<u32>(black_box(FIBN)));
        })
    }

    #[bench]
    fn fib_u128_big(b: &mut Bencher) {
        b.iter(|| {
            black_box(fib::<ethereum_types::U128>(black_box(FIBN)));
        })
    }

    #[bench]
    fn fib_u256_big(b: &mut Bencher) {
        b.iter(|| {
            black_box(fib::<ethereum_types::U256>(black_box(FIBN)));
        })
    }
}

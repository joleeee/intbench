#![feature(test)]
extern crate test;

#[allow(dead_code)]
fn fib<N: Number>(times: u32) -> N {
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

#[allow(unused_macros)]
macro_rules! impl_fib {
    ($type:ty) => {
        paste::paste! {
            #[bench]
            fn [<_ $type _>](b: &mut super::Bencher) {
                use super::*;
                b.iter(|| {
                    black_box(fib::<$type>(black_box(FIBN)));
                })
            }
        }
    };
}

#[cfg(test)]
mod fib {
    use crate::*;
    use test::{black_box, Bencher};

    const FIBN: u32 = 10_000;

    mod native {
        impl_fib!(u32);
        impl_fib!(u128);
    }
    mod ethereum {
        use {ethereum_types::U128 as u128, ethereum_types::U256 as u256};
        impl_fib!(u128);
        impl_fib!(u256);
    }
}

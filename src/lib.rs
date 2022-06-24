#![feature(test)]
extern crate test;

#[allow(dead_code)]
fn fib<N: Number>(times: u32) -> N {
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
fn three_n_one<N: Number>(start: u32) -> N {
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

trait Number:
    Copy
    + PartialEq
    + std::ops::Div<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::Rem<Output = Self>
{
    fn from(f: u32) -> Self;
    fn overflowing_add(self, rhs: Self) -> Self;
}

macro_rules! impl_num_native {
    ($type:ty) => {
        impl Number for $type {
            fn from(f: u32) -> Self {
                f as $type
            }
            fn overflowing_add(self, rhs: Self) -> Self {
                self.overflowing_add(rhs).0
            }
        }
    };
}

impl_num_native!(u64);
impl_num_native!(u128);

macro_rules! impl_num_eth {
    ($type:ty) => {
        impl Number for $type {
            fn overflowing_add(self, rhs: Self) -> Self {
                self.overflowing_add(rhs).0
            }
            fn from(f: u32) -> Self {
                <Self as From<u32>>::from(f)
            }
        }
    };
}

impl_num_eth!(ethereum_types::U64);
impl_num_eth!(ethereum_types::U128);
impl_num_eth!(ethereum_types::U256);

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

#[allow(unused_macros)]
macro_rules! impl_three_n_one {
    ($type:ty) => {
        paste::paste! {
            #[bench]
            fn [<_ $type _>](b: &mut super::Bencher) {
                use super::*;
                b.iter(|| {
                    black_box(three_n_one::<$type>(black_box(START)));
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
        impl_fib!(u64);
        impl_fib!(u128);
    }
    mod ethereum {
        use {
            ethereum_types::U128 as u128, ethereum_types::U256 as u256, ethereum_types::U64 as u64,
        };
        impl_fib!(u64);
        impl_fib!(u128);
        impl_fib!(u256);
    }
}

#[cfg(test)]
mod three_n_one {
    use crate::*;
    use test::{black_box, Bencher};

    const START: u32 = 27;

    mod native {
        impl_three_n_one!(u64);
        impl_three_n_one!(u128);
    }
    mod ethereum {
        use {
            ethereum_types::U128 as u128, ethereum_types::U256 as u256, ethereum_types::U64 as u64,
        };
        impl_three_n_one!(u64);
        impl_three_n_one!(u128);
        impl_three_n_one!(u256);
    }
}

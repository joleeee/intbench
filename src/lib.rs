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

trait Number: Copy {
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

impl_num_native!(u32);
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

impl_num_eth!(ethereum_types::U256);
impl_num_eth!(ethereum_types::U128);

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

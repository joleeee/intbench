#![feature(test)]
extern crate test;

mod algo;
use algo::*;

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

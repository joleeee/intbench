#![feature(test)]
extern crate test;

mod algo;
use algo::*;

mod types;

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
        use crate::types::ethereum::{u128, u256, u64};
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
        use crate::types::native::{u128, u64};
        impl_three_n_one!(u64);
        impl_three_n_one!(u128);
    }
    mod ethereum {
        use crate::types::ethereum::{u128, u256, u64};
        impl_three_n_one!(u64);
        impl_three_n_one!(u128);
        impl_three_n_one!(u256);
    }
}

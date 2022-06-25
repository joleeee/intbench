#![feature(test)]
extern crate test;

mod algo;
use algo::*;

mod types;

mod speedint;

#[allow(unused_macros)]
macro_rules! bench_fib {
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
macro_rules! bench_3np1 {
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
        bench_fib!(u64);
        bench_fib!(u128);
    }
    mod ethereum {
        use crate::types::ethereum::{u128, u256, u64};
        bench_fib!(u64);
        bench_fib!(u128);
        bench_fib!(u256);
    }
    mod speedint {
        use crate::types::speed::u256;
        bench_fib!(u256);
    }
}

#[cfg(test)]
mod fib_check {
    use crate::algo::fib;
    use crate::types;

    use types::ethereum::u256 as eth256;
    use types::speed::u256 as speed256;

    #[test]
    fn equal_test() {
        // ~41 bits
        println!("{}", fib::<eth256>(60));
        assert!(equal(60)); // fits in u64

        // ~84 bits
        println!("{}", fib::<eth256>(120));
        assert!(equal(120));

        // max
        println!("{}", fib::<eth256>(100_000));
        assert!(equal(100_000));
    }

    fn equal(fibn: u32) -> bool {
        let eth = fib::<eth256>(fibn).0;
        let speed = fib::<speed256>(fibn).0;

        let eth_lower = (eth[0] as u128) + ((eth[1] as u128) << 64);
        let eth_upper = (eth[2] as u128) + ((eth[3] as u128) << 64);

        eth_lower == speed[0] && eth_upper == speed[1]
    }
}

#[cfg(test)]
mod three_n_one {
    use crate::*;
    use test::{black_box, Bencher};

    const START: u32 = 27;

    mod native {
        use crate::types::native::{u128, u64};
        bench_3np1!(u64);
        bench_3np1!(u128);
    }
    mod ethereum {
        use crate::types::ethereum::{u128, u256, u64};
        bench_3np1!(u64);
        bench_3np1!(u128);
        bench_3np1!(u256);
    }
}

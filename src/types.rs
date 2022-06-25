pub mod ethereum {
    use crate::Number;
    pub use {
        ethereum_types::U128 as u128, ethereum_types::U256 as u256, ethereum_types::U64 as u64,
    };

    macro_rules! impl_num {
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

    impl_num!(u64);
    impl_num!(u128);
    impl_num!(u256);
}

pub mod native {
    use crate::Number;
    pub use {u128, u64};

    macro_rules! impl_num {
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

    impl_num!(u64);
    impl_num!(u128);
}

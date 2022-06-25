pub mod ethereum {
    pub use {
        ethereum_types::U128 as u128, ethereum_types::U256 as u256, ethereum_types::U64 as u64,
    };

    use crate::Number;

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

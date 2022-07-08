pub mod ethereum {
    use crate::Number;
    pub use {
        ethereum_types::U128 as u128, ethereum_types::U256 as u256, ethereum_types::U64 as u64,
    };

    macro_rules! impl_num {
        ($($type:ty )*) => {
            $(
            impl Number for $type {
                fn overflowing_add(self, rhs: Self) -> Self {
                    self.overflowing_add(rhs).0
                }
                fn from(f: u32) -> Self {
                    <Self as From<u32>>::from(f)
                }
            }
            )*
        };
    }

    impl_num!(u64 u128 u256);
}

pub mod native {
    use crate::Number;
    pub use {u128, u64};

    macro_rules! impl_num {
        ($($type:ty )*) => {
            $(
            impl Number for $type {
                fn from(f: u32) -> Self {
                    f as $type
                }
                fn overflowing_add(self, rhs: Self) -> Self {
                    self.overflowing_add(rhs).0
                }
            }
            )*
        };
    }

    impl_num!(u64 u128);
}

pub mod speed {
    pub use crate::speedint::U256 as u256;
    use crate::Number;

    impl Number for u256 {
        fn overflowing_add(self, rhs: Self) -> Self {
            self.add_inline(&rhs)
        }
        fn from(f: u32) -> Self {
            Self::from_limbs([f as u128, 0])
        }
    }
}

#[allow(non_camel_case_types)]
pub mod ru {
    pub type u256 = ruint::Uint<256, 4>;
    pub type u128 = ruint::Uint<128, 2>;
    use crate::Number;

    macro_rules! impl_ru {
        ($($type:ty )*) => {
            $(
            impl Number for $type {
                fn overflowing_add(self, rhs: Self) -> Self {
                    self.overflowing_add(rhs).0
                }
                fn from(f: u32) -> Self {
                    Self::from(f)
                }
            }
            )*
        };
    }

    impl_ru!(u128 u256);
}

#[derive(Debug, Copy, Clone)]
pub struct U256(pub [u128; 2]);

impl U256 {
    pub fn from_limbs(limbs: [u128; 2]) -> Self {
        Self(limbs)
    }

    pub fn add_inline(&self, rhs: &Self) -> Self {
        let (l0, c0) = self.0[0].overflowing_add(rhs.0[0]);

        let (l1, _) = self.0[1].overflowing_add(rhs.0[1]);

        let l1 = if c0 { l1.overflowing_add(1).0 } else { l1 };

        Self([l0, l1])
    }
}

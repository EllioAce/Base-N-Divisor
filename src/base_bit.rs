pub const HAS_DECIMALS: bool = false;
pub const DECIMAL_END_POINT: usize = 16;
pub const BASE_BITS_LENGTH: usize = DECIMAL_END_POINT * ((HAS_DECIMALS as usize) + 1);

#[derive(Copy, Clone)]
pub struct BaseBit {
    pub bits: [u8; BASE_BITS_LENGTH],
    pub base: u8,
}

impl BaseBit {
    pub fn new(bits: [u8; BASE_BITS_LENGTH], base: u8) -> BaseBit {
        return BaseBit {
            bits: bits,
            base: base,
        };
    }
}

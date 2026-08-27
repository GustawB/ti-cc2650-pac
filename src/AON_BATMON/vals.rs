#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PER {
    #[doc = "Internal. Only to be used through TI provided API."]
    CONT = 0x0,
    #[doc = "Internal. Only to be used through TI provided API."]
    _8CYC = 0x01,
    #[doc = "Internal. Only to be used through TI provided API."]
    _16CYC = 0x02,
    #[doc = "Internal. Only to be used through TI provided API."]
    _32CYC = 0x03,
}
impl PER {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PER {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PER {
    #[inline(always)]
    fn from(val: u8) -> PER {
        PER::from_bits(val)
    }
}
impl From<PER> for u8 {
    #[inline(always)]
    fn from(val: PER) -> u8 {
        PER::to_bits(val)
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum COMB_EV_MASK {
    #[doc = "No event is selected for combined event."]
    NONE = 0x0,
    #[doc = "Use Channel 0 delayed event in combined event."]
    CH0 = 0x01,
    #[doc = "Use Channel 1 delayed event in combined event."]
    CH1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Use Channel 2 delayed event in combined event."]
    CH2 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl COMB_EV_MASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> COMB_EV_MASK {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for COMB_EV_MASK {
    #[inline(always)]
    fn from(val: u8) -> COMB_EV_MASK {
        COMB_EV_MASK::from_bits(val)
    }
}
impl From<COMB_EV_MASK> for u8 {
    #[inline(always)]
    fn from(val: COMB_EV_MASK) -> u8 {
        COMB_EV_MASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EV_DELAY {
    #[doc = "No delay on delayed event."]
    D0 = 0x0,
    #[doc = "Delay by 1 clock cycles."]
    D1 = 0x01,
    #[doc = "Delay by 2 clock cycles."]
    D2 = 0x02,
    #[doc = "Delay by 4 clock cycles."]
    D4 = 0x03,
    #[doc = "Delay by 8 clock cycles."]
    D8 = 0x04,
    #[doc = "Delay by 16 clock cycles."]
    D16 = 0x05,
    #[doc = "Delay by 32 clock cycles."]
    D32 = 0x06,
    #[doc = "Delay by 48 clock cycles."]
    D48 = 0x07,
    #[doc = "Delay by 64 clock cycles."]
    D64 = 0x08,
    #[doc = "Delay by 80 clock cycles."]
    D80 = 0x09,
    #[doc = "Delay by 96 clock cycles."]
    D96 = 0x0a,
    #[doc = "Delay by 112 clock cycles."]
    D112 = 0x0b,
    #[doc = "Delay by 128 clock cycles."]
    D128 = 0x0c,
    #[doc = "Delay by 144 clock cycles."]
    D144 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl EV_DELAY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EV_DELAY {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EV_DELAY {
    #[inline(always)]
    fn from(val: u8) -> EV_DELAY {
        EV_DELAY::from_bits(val)
    }
}
impl From<EV_DELAY> for u8 {
    #[inline(always)]
    fn from(val: EV_DELAY) -> u8 {
        EV_DELAY::to_bits(val)
    }
}

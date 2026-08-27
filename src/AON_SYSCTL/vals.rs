#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RESET_SRC {
    #[doc = "Power on reset."]
    PWR_ON = 0x0,
    #[doc = "Reset pin."]
    PIN_RESET = 0x01,
    #[doc = "Brown out detect on VDDS."]
    VDDS_LOSS = 0x02,
    #[doc = "Brown out detect on VDD."]
    VDD_LOSS = 0x03,
    #[doc = "Brown out detect on VDDR."]
    VDDR_LOSS = 0x04,
    #[doc = "Clock loss detect."]
    CLK_LOSS = 0x05,
    #[doc = "Software reset via SYSRESET register."]
    SYSRESET = 0x06,
    #[doc = "Software reset via PRCM warm reset request."]
    WARMRESET = 0x07,
}
impl RESET_SRC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RESET_SRC {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RESET_SRC {
    #[inline(always)]
    fn from(val: u8) -> RESET_SRC {
        RESET_SRC::from_bits(val)
    }
}
impl From<RESET_SRC> for u8 {
    #[inline(always)]
    fn from(val: RESET_SRC) -> u8 {
        RESET_SRC::to_bits(val)
    }
}

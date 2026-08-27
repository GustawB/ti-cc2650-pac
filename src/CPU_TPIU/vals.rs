#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PROTOCOL {
    #[doc = "TracePort mode."]
    TRACEPORT = 0x0,
    #[doc = "SerialWire Output (Manchester). This is the reset value."]
    SWO_MANCHESTER = 0x01,
    #[doc = "SerialWire Output (NRZ)."]
    SWO_NRZ = 0x02,
    _RESERVED_3 = 0x03,
}
impl PROTOCOL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PROTOCOL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PROTOCOL {
    #[inline(always)]
    fn from(val: u8) -> PROTOCOL {
        PROTOCOL::from_bits(val)
    }
}
impl From<PROTOCOL> for u8 {
    #[inline(always)]
    fn from(val: PROTOCOL) -> u8 {
        PROTOCOL::to_bits(val)
    }
}

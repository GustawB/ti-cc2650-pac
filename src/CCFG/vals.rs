#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCLK_LF_OPTION {
    #[doc = "31.25kHz clock derived from 24MHz XOSC (dividing by 768 in HW). The RTC tick speed \\[AON_RTC.SUBSECINC.*\\] is updated to 0x8637BD, corresponding to a 31.25kHz clock (done in the trimDevice() xxWare boot function). Standby power mode is not supported when using this clock source."]
    XOSC_HF_DLF = 0x0,
    #[doc = "External low frequency clock on DIO defined by EXT_LF_CLK.DIO. The RTC tick speed AON_RTC:SUBSECINC is updated to EXT_LF_CLK.RTC_INCREMENT (done in the trimDevice() xxWare boot function). External clock must always be running when the chip is in standby for VDDR recharge timing."]
    EXTERNAL_LF = 0x01,
    #[doc = "32.768kHz low frequency XOSC."]
    XOSC_LF = 0x02,
    #[doc = "Low frequency RCOSC (default)."]
    RCOSC_LF = 0x03,
}
impl SCLK_LF_OPTION {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCLK_LF_OPTION {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCLK_LF_OPTION {
    #[inline(always)]
    fn from(val: u8) -> SCLK_LF_OPTION {
        SCLK_LF_OPTION::from_bits(val)
    }
}
impl From<SCLK_LF_OPTION> for u8 {
    #[inline(always)]
    fn from(val: SCLK_LF_OPTION) -> u8 {
        SCLK_LF_OPTION::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XOSC_FREQ {
    _RESERVED_0 = 0x0,
    #[doc = "HPOSC."]
    HPOSC = 0x01,
    #[doc = "48 MHz XOSC_HF."]
    _48M = 0x02,
    #[doc = "24 MHz XOSC_HF."]
    _24M = 0x03,
}
impl XOSC_FREQ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XOSC_FREQ {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XOSC_FREQ {
    #[inline(always)]
    fn from(val: u8) -> XOSC_FREQ {
        XOSC_FREQ::from_bits(val)
    }
}
impl From<XOSC_FREQ> for u8 {
    #[inline(always)]
    fn from(val: XOSC_FREQ) -> u8 {
        XOSC_FREQ::to_bits(val)
    }
}

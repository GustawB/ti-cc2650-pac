#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMD {
    #[doc = "Disable ADC interface."]
    DIS = 0x0,
    #[doc = "Enable ADC interface."]
    EN = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Flush ADC FIFO. You must set CMD to EN or DIS after flush. System CPU must wait two clock cycles before it sets CMD to EN or DIS."]
    FLUSH = 0x03,
}
impl CMD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMD {
    #[inline(always)]
    fn from(val: u8) -> CMD {
        CMD::from_bits(val)
    }
}
impl From<CMD> for u8 {
    #[inline(always)]
    fn from(val: CMD) -> u8 {
        CMD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum START_POL {
    #[doc = "Set ADC trigger on rising edge of event source."]
    RISE = 0x0,
    #[doc = "Set ADC trigger on falling edge of event source."]
    FALL = 0x01,
}
impl START_POL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> START_POL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for START_POL {
    #[inline(always)]
    fn from(val: u8) -> START_POL {
        START_POL::from_bits(val)
    }
}
impl From<START_POL> for u8 {
    #[inline(always)]
    fn from(val: START_POL) -> u8 {
        START_POL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum START_SRC {
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2."]
    RTC_CH2_EV = 0x0,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA."]
    AUX_COMPA = 0x01,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB."]
    AUX_COMPB = 0x02,
    #[doc = "AUX_EVCTL:EVSTAT0.TDC_DONE."]
    TDC_DONE = 0x03,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER0_EV."]
    TIMER0_EV = 0x04,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV."]
    TIMER1_EV = 0x05,
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE."]
    SMPH_AUTOTAKE_DONE = 0x06,
    #[doc = "Reserved - Do not use."]
    RESERVED0 = 0x07,
    #[doc = "Reserved - Do not use."]
    RESERVED1 = 0x08,
    #[doc = "No event."]
    NO_EVENT0 = 0x09,
    #[doc = "No event."]
    NO_EVENT1 = 0x0a,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW."]
    AON_SW = 0x0b,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU."]
    AON_PROG_WU = 0x0c,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0."]
    AUXIO0 = 0x0d,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1."]
    AUXIO1 = 0x0e,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2."]
    AUXIO2 = 0x0f,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3."]
    AUXIO3 = 0x10,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4."]
    AUXIO4 = 0x11,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5."]
    AUXIO5 = 0x12,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6."]
    AUXIO6 = 0x13,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7."]
    AUXIO7 = 0x14,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8."]
    AUXIO8 = 0x15,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9."]
    AUXIO9 = 0x16,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10."]
    AUXIO10 = 0x17,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11."]
    AUXIO11 = 0x18,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12."]
    AUXIO12 = 0x19,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13."]
    AUXIO13 = 0x1a,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14."]
    AUXIO14 = 0x1b,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15."]
    AUXIO15 = 0x1c,
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF."]
    ACLK_REF = 0x1d,
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV."]
    MCU_EV = 0x1e,
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ."]
    ADC_IRQ = 0x1f,
}
impl START_SRC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> START_SRC {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for START_SRC {
    #[inline(always)]
    fn from(val: u8) -> START_SRC {
        START_SRC::from_bits(val)
    }
}
impl From<START_SRC> for u8 {
    #[inline(always)]
    fn from(val: START_SRC) -> u8 {
        START_SRC::to_bits(val)
    }
}

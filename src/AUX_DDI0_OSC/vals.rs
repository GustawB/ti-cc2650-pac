#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AMPCOMP_FSM_UPDATE_RATE {
    #[doc = "Internal. Only to be used through TI provided API."]
    _2MHZ = 0x0,
    #[doc = "Internal. Only to be used through TI provided API."]
    _1MHZ = 0x01,
    #[doc = "Internal. Only to be used through TI provided API."]
    _500KHZ = 0x02,
    #[doc = "Internal. Only to be used through TI provided API."]
    _250KHZ = 0x03,
}
impl AMPCOMP_FSM_UPDATE_RATE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AMPCOMP_FSM_UPDATE_RATE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AMPCOMP_FSM_UPDATE_RATE {
    #[inline(always)]
    fn from(val: u8) -> AMPCOMP_FSM_UPDATE_RATE {
        AMPCOMP_FSM_UPDATE_RATE::from_bits(val)
    }
}
impl From<AMPCOMP_FSM_UPDATE_RATE> for u8 {
    #[inline(always)]
    fn from(val: AMPCOMP_FSM_UPDATE_RATE) -> u8 {
        AMPCOMP_FSM_UPDATE_RATE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAMPSTATE {
    #[doc = "RESET."]
    RESET = 0x0,
    #[doc = "INITIALIZATION."]
    INITIALIZATION = 0x01,
    #[doc = "HPM_RAMP1."]
    HPM_RAMP1 = 0x02,
    #[doc = "HPM_RAMP2."]
    HPM_RAMP2 = 0x03,
    #[doc = "HPM_RAMP3."]
    HPM_RAMP3 = 0x04,
    #[doc = "HPM_UPDATE."]
    HPM_UPDATE = 0x05,
    #[doc = "IDAC_INCREMENT."]
    IDAC_INCREMENT = 0x06,
    #[doc = "IBIAS_CAP_UPDATE."]
    IBIAS_CAP_UPDATE = 0x07,
    #[doc = "IBIAS_DECREMENT_WITH_MEASURE."]
    IBIAS_DEC_W_MEASURE = 0x08,
    #[doc = "LPM_UPDATE."]
    LPM_UPDATE = 0x09,
    #[doc = "IBIAS_INCREMENT."]
    IBIAS_INC = 0x0a,
    #[doc = "IDAC_DECREMENT_WITH_MEASURE."]
    IDAC_DEC_W_MEASURE = 0x0b,
    #[doc = "DUMMY_TO_INIT_1."]
    DUMMY_TO_INIT_1 = 0x0c,
    #[doc = "FAST_START."]
    FAST_START = 0x0d,
    #[doc = "FAST_START_SETTLE."]
    FAST_START_SETTLE = 0x0e,
    _RESERVED_f = 0x0f,
}
impl RAMPSTATE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAMPSTATE {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAMPSTATE {
    #[inline(always)]
    fn from(val: u8) -> RAMPSTATE {
        RAMPSTATE::from_bits(val)
    }
}
impl From<RAMPSTATE> for u8 {
    #[inline(always)]
    fn from(val: RAMPSTATE) -> u8 {
        RAMPSTATE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RCOSCLF_RTUNE_TRIM {
    #[doc = "Internal. Only to be used through TI provided API."]
    _7P5MEG = 0x0,
    #[doc = "Internal. Only to be used through TI provided API."]
    _7P0MEG = 0x01,
    #[doc = "Internal. Only to be used through TI provided API."]
    _6P5MEG = 0x02,
    #[doc = "Internal. Only to be used through TI provided API."]
    _6P0MEG = 0x03,
}
impl RCOSCLF_RTUNE_TRIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RCOSCLF_RTUNE_TRIM {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RCOSCLF_RTUNE_TRIM {
    #[inline(always)]
    fn from(val: u8) -> RCOSCLF_RTUNE_TRIM {
        RCOSCLF_RTUNE_TRIM::from_bits(val)
    }
}
impl From<RCOSCLF_RTUNE_TRIM> for u8 {
    #[inline(always)]
    fn from(val: RCOSCLF_RTUNE_TRIM) -> u8 {
        RCOSCLF_RTUNE_TRIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCLK_HF_SRC {
    #[doc = "High frequency RCOSC clock."]
    RCOSC = 0x0,
    #[doc = "High frequency XOSC."]
    XOSC = 0x01,
}
impl SCLK_HF_SRC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCLK_HF_SRC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCLK_HF_SRC {
    #[inline(always)]
    fn from(val: u8) -> SCLK_HF_SRC {
        SCLK_HF_SRC::from_bits(val)
    }
}
impl From<SCLK_HF_SRC> for u8 {
    #[inline(always)]
    fn from(val: SCLK_HF_SRC) -> u8 {
        SCLK_HF_SRC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCLK_HF_SRC_SEL {
    #[doc = "High frequency RCOSC clock."]
    RCOSC = 0x0,
    #[doc = "High frequency XOSC clk."]
    XOSC = 0x01,
}
impl SCLK_HF_SRC_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCLK_HF_SRC_SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCLK_HF_SRC_SEL {
    #[inline(always)]
    fn from(val: u8) -> SCLK_HF_SRC_SEL {
        SCLK_HF_SRC_SEL::from_bits(val)
    }
}
impl From<SCLK_HF_SRC_SEL> for u8 {
    #[inline(always)]
    fn from(val: SCLK_HF_SRC_SEL) -> u8 {
        SCLK_HF_SRC_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCLK_LF_SRC {
    #[doc = "Low frequency clock derived from High Frequency RCOSC."]
    RCOSCHFDLF = 0x0,
    #[doc = "Low frequency clock derived from High Frequency XOSC."]
    XOSCHFDLF = 0x01,
    #[doc = "Low frequency RCOSC."]
    RCOSCLF = 0x02,
    #[doc = "Low frequency XOSC."]
    XOSCLF = 0x03,
}
impl SCLK_LF_SRC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCLK_LF_SRC {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCLK_LF_SRC {
    #[inline(always)]
    fn from(val: u8) -> SCLK_LF_SRC {
        SCLK_LF_SRC::from_bits(val)
    }
}
impl From<SCLK_LF_SRC> for u8 {
    #[inline(always)]
    fn from(val: SCLK_LF_SRC) -> u8 {
        SCLK_LF_SRC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCLK_LF_SRC_SEL {
    #[doc = "Low frequency clock derived from High Frequency RCOSC."]
    RCOSCHFDLF = 0x0,
    #[doc = "Low frequency clock derived from High Frequency XOSC."]
    XOSCHFDLF = 0x01,
    #[doc = "Low frequency RCOSC."]
    RCOSCLF = 0x02,
    #[doc = "Low frequency XOSC."]
    XOSCLF = 0x03,
}
impl SCLK_LF_SRC_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCLK_LF_SRC_SEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCLK_LF_SRC_SEL {
    #[inline(always)]
    fn from(val: u8) -> SCLK_LF_SRC_SEL {
        SCLK_LF_SRC_SEL::from_bits(val)
    }
}
impl From<SCLK_LF_SRC_SEL> for u8 {
    #[inline(always)]
    fn from(val: SCLK_LF_SRC_SEL) -> u8 {
        SCLK_LF_SRC_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCLK_MF_SRC_SEL {
    #[doc = "Internal. Only to be used through TI provided API."]
    RCOSCHFDMF = 0x0,
    #[doc = "Medium frequency clock derived from high frequency XOSC."]
    XCOSCHFDMF = 0x01,
}
impl SCLK_MF_SRC_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCLK_MF_SRC_SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCLK_MF_SRC_SEL {
    #[inline(always)]
    fn from(val: u8) -> SCLK_MF_SRC_SEL {
        SCLK_MF_SRC_SEL::from_bits(val)
    }
}
impl From<SCLK_MF_SRC_SEL> for u8 {
    #[inline(always)]
    fn from(val: SCLK_MF_SRC_SEL) -> u8 {
        SCLK_MF_SRC_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XTAL_IS_24M {
    #[doc = "Internal. Only to be used through TI provided API."]
    _48M = 0x0,
    #[doc = "Internal. Only to be used through TI provided API."]
    _24M = 0x01,
}
impl XTAL_IS_24M {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XTAL_IS_24M {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XTAL_IS_24M {
    #[inline(always)]
    fn from(val: u8) -> XTAL_IS_24M {
        XTAL_IS_24M::from_bits(val)
    }
}
impl From<XTAL_IS_24M> for u8 {
    #[inline(always)]
    fn from(val: XTAL_IS_24M) -> u8 {
        XTAL_IS_24M::to_bits(val)
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EN {
    #[doc = "Latches are static ( closed )."]
    STATIC = 0x0,
    #[doc = "Latches are transparent ( open )."]
    TRANSP = 0x01,
}
impl EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EN {
    #[inline(always)]
    fn from(val: u8) -> EN {
        EN::from_bits(val)
    }
}
impl From<EN> for u8 {
    #[inline(always)]
    fn from(val: EN) -> u8 {
        EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODCLKEN0_AIODIO0 {
    #[doc = "System CPU has not requested clock for AIODIO0."]
    DIS = 0x0,
    #[doc = "System CPU has requested clock for AIODIO0."]
    EN = 0x01,
}
impl MODCLKEN0_AIODIO0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODCLKEN0_AIODIO0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODCLKEN0_AIODIO0 {
    #[inline(always)]
    fn from(val: u8) -> MODCLKEN0_AIODIO0 {
        MODCLKEN0_AIODIO0::from_bits(val)
    }
}
impl From<MODCLKEN0_AIODIO0> for u8 {
    #[inline(always)]
    fn from(val: MODCLKEN0_AIODIO0) -> u8 {
        MODCLKEN0_AIODIO0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODCLKEN0_AIODIO1 {
    #[doc = "System CPU has not requested clock for AIODIO1."]
    DIS = 0x0,
    #[doc = "System CPU has requested clock for AIODIO1."]
    EN = 0x01,
}
impl MODCLKEN0_AIODIO1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODCLKEN0_AIODIO1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODCLKEN0_AIODIO1 {
    #[inline(always)]
    fn from(val: u8) -> MODCLKEN0_AIODIO1 {
        MODCLKEN0_AIODIO1::from_bits(val)
    }
}
impl From<MODCLKEN0_AIODIO1> for u8 {
    #[inline(always)]
    fn from(val: MODCLKEN0_AIODIO1) -> u8 {
        MODCLKEN0_AIODIO1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODCLKEN0_ANAIF {
    #[doc = "System CPU has not requested clock for ANAIF."]
    DIS = 0x0,
    #[doc = "System CPU has requested clock for ANAIF."]
    EN = 0x01,
}
impl MODCLKEN0_ANAIF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODCLKEN0_ANAIF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODCLKEN0_ANAIF {
    #[inline(always)]
    fn from(val: u8) -> MODCLKEN0_ANAIF {
        MODCLKEN0_ANAIF::from_bits(val)
    }
}
impl From<MODCLKEN0_ANAIF> for u8 {
    #[inline(always)]
    fn from(val: MODCLKEN0_ANAIF) -> u8 {
        MODCLKEN0_ANAIF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODCLKEN0_AUX_ADI4 {
    #[doc = "System CPU has not requested clock for AUX_ADI4."]
    DIS = 0x0,
    #[doc = "System CPU has requested clock for AUX_ADI4."]
    EN = 0x01,
}
impl MODCLKEN0_AUX_ADI4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODCLKEN0_AUX_ADI4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODCLKEN0_AUX_ADI4 {
    #[inline(always)]
    fn from(val: u8) -> MODCLKEN0_AUX_ADI4 {
        MODCLKEN0_AUX_ADI4::from_bits(val)
    }
}
impl From<MODCLKEN0_AUX_ADI4> for u8 {
    #[inline(always)]
    fn from(val: MODCLKEN0_AUX_ADI4) -> u8 {
        MODCLKEN0_AUX_ADI4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODCLKEN0_AUX_DDI0_OSC {
    #[doc = "System CPU has not requested clock for AUX_DDI0_OSC."]
    DIS = 0x0,
    #[doc = "System CPU has requested clock for AUX_DDI0_OSC."]
    EN = 0x01,
}
impl MODCLKEN0_AUX_DDI0_OSC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODCLKEN0_AUX_DDI0_OSC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODCLKEN0_AUX_DDI0_OSC {
    #[inline(always)]
    fn from(val: u8) -> MODCLKEN0_AUX_DDI0_OSC {
        MODCLKEN0_AUX_DDI0_OSC::from_bits(val)
    }
}
impl From<MODCLKEN0_AUX_DDI0_OSC> for u8 {
    #[inline(always)]
    fn from(val: MODCLKEN0_AUX_DDI0_OSC) -> u8 {
        MODCLKEN0_AUX_DDI0_OSC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODCLKEN0_SMPH {
    #[doc = "System CPU has not requested clock for SMPH."]
    DIS = 0x0,
    #[doc = "System CPU has requested clock for SMPH."]
    EN = 0x01,
}
impl MODCLKEN0_SMPH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODCLKEN0_SMPH {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODCLKEN0_SMPH {
    #[inline(always)]
    fn from(val: u8) -> MODCLKEN0_SMPH {
        MODCLKEN0_SMPH::from_bits(val)
    }
}
impl From<MODCLKEN0_SMPH> for u8 {
    #[inline(always)]
    fn from(val: MODCLKEN0_SMPH) -> u8 {
        MODCLKEN0_SMPH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODCLKEN0_TIMER {
    #[doc = "System CPU has not requested clock for TIMER."]
    DIS = 0x0,
    #[doc = "System CPU has requested clock for TIMER."]
    EN = 0x01,
}
impl MODCLKEN0_TIMER {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODCLKEN0_TIMER {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODCLKEN0_TIMER {
    #[inline(always)]
    fn from(val: u8) -> MODCLKEN0_TIMER {
        MODCLKEN0_TIMER::from_bits(val)
    }
}
impl From<MODCLKEN0_TIMER> for u8 {
    #[inline(always)]
    fn from(val: MODCLKEN0_TIMER) -> u8 {
        MODCLKEN0_TIMER::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODCLKEN1_AIODIO0 {
    #[doc = "AUX_SCE has not requested clock for AIODIO0."]
    DIS = 0x0,
    #[doc = "AUX_SCE has requested clock for AIODIO0."]
    EN = 0x01,
}
impl MODCLKEN1_AIODIO0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODCLKEN1_AIODIO0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODCLKEN1_AIODIO0 {
    #[inline(always)]
    fn from(val: u8) -> MODCLKEN1_AIODIO0 {
        MODCLKEN1_AIODIO0::from_bits(val)
    }
}
impl From<MODCLKEN1_AIODIO0> for u8 {
    #[inline(always)]
    fn from(val: MODCLKEN1_AIODIO0) -> u8 {
        MODCLKEN1_AIODIO0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODCLKEN1_AIODIO1 {
    #[doc = "AUX_SCE has not requested clock for AIODIO1."]
    DIS = 0x0,
    #[doc = "AUX_SCE has requested clock for AIODIO1."]
    EN = 0x01,
}
impl MODCLKEN1_AIODIO1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODCLKEN1_AIODIO1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODCLKEN1_AIODIO1 {
    #[inline(always)]
    fn from(val: u8) -> MODCLKEN1_AIODIO1 {
        MODCLKEN1_AIODIO1::from_bits(val)
    }
}
impl From<MODCLKEN1_AIODIO1> for u8 {
    #[inline(always)]
    fn from(val: MODCLKEN1_AIODIO1) -> u8 {
        MODCLKEN1_AIODIO1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODCLKEN1_ANAIF {
    #[doc = "AUX_SCE has not requested clock for ANAIF."]
    DIS = 0x0,
    #[doc = "AUX_SCE has requested clock for ANAIF."]
    EN = 0x01,
}
impl MODCLKEN1_ANAIF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODCLKEN1_ANAIF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODCLKEN1_ANAIF {
    #[inline(always)]
    fn from(val: u8) -> MODCLKEN1_ANAIF {
        MODCLKEN1_ANAIF::from_bits(val)
    }
}
impl From<MODCLKEN1_ANAIF> for u8 {
    #[inline(always)]
    fn from(val: MODCLKEN1_ANAIF) -> u8 {
        MODCLKEN1_ANAIF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODCLKEN1_AUX_ADI4 {
    #[doc = "AUX_SCE has not requested clock for AUX_ADI4."]
    DIS = 0x0,
    #[doc = "AUX_SCE has requested clock for AUX_ADI4."]
    EN = 0x01,
}
impl MODCLKEN1_AUX_ADI4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODCLKEN1_AUX_ADI4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODCLKEN1_AUX_ADI4 {
    #[inline(always)]
    fn from(val: u8) -> MODCLKEN1_AUX_ADI4 {
        MODCLKEN1_AUX_ADI4::from_bits(val)
    }
}
impl From<MODCLKEN1_AUX_ADI4> for u8 {
    #[inline(always)]
    fn from(val: MODCLKEN1_AUX_ADI4) -> u8 {
        MODCLKEN1_AUX_ADI4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODCLKEN1_AUX_DDI0_OSC {
    #[doc = "AUX_SCE has not requested clock for AUX_DDI0_OSC."]
    DIS = 0x0,
    #[doc = "AUX_SCE has requested clock for AUX_DDI0_OSC."]
    EN = 0x01,
}
impl MODCLKEN1_AUX_DDI0_OSC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODCLKEN1_AUX_DDI0_OSC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODCLKEN1_AUX_DDI0_OSC {
    #[inline(always)]
    fn from(val: u8) -> MODCLKEN1_AUX_DDI0_OSC {
        MODCLKEN1_AUX_DDI0_OSC::from_bits(val)
    }
}
impl From<MODCLKEN1_AUX_DDI0_OSC> for u8 {
    #[inline(always)]
    fn from(val: MODCLKEN1_AUX_DDI0_OSC) -> u8 {
        MODCLKEN1_AUX_DDI0_OSC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODCLKEN1_SMPH {
    #[doc = "AUX_SCE has not requested clock for SMPH."]
    DIS = 0x0,
    #[doc = "AUX_SCE has requested clock for SMPH."]
    EN = 0x01,
}
impl MODCLKEN1_SMPH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODCLKEN1_SMPH {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODCLKEN1_SMPH {
    #[inline(always)]
    fn from(val: u8) -> MODCLKEN1_SMPH {
        MODCLKEN1_SMPH::from_bits(val)
    }
}
impl From<MODCLKEN1_SMPH> for u8 {
    #[inline(always)]
    fn from(val: MODCLKEN1_SMPH) -> u8 {
        MODCLKEN1_SMPH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODCLKEN1_TIMER {
    #[doc = "AUX_SCE has not requested clock for TIMER."]
    DIS = 0x0,
    #[doc = "AUX_SCE has requested clock for TIMER."]
    EN = 0x01,
}
impl MODCLKEN1_TIMER {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODCLKEN1_TIMER {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODCLKEN1_TIMER {
    #[inline(always)]
    fn from(val: u8) -> MODCLKEN1_TIMER {
        MODCLKEN1_TIMER::from_bits(val)
    }
}
impl From<MODCLKEN1_TIMER> for u8 {
    #[inline(always)]
    fn from(val: MODCLKEN1_TIMER) -> u8 {
        MODCLKEN1_TIMER::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TDC {
    #[doc = "System CPU has not requested clock for TDC."]
    DIS = 0x0,
    #[doc = "System CPU has requested clock for TDC."]
    EN = 0x01,
}
impl TDC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TDC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TDC {
    #[inline(always)]
    fn from(val: u8) -> TDC {
        TDC::from_bits(val)
    }
}
impl From<TDC> for u8 {
    #[inline(always)]
    fn from(val: TDC) -> u8 {
        TDC::to_bits(val)
    }
}

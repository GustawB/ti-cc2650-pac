#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ADC_FIFO_ALMOST_FULL {
    #[doc = "High level."]
    HIGH = 0x0,
    #[doc = "Low level."]
    LOW = 0x01,
}
impl ADC_FIFO_ALMOST_FULL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ADC_FIFO_ALMOST_FULL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ADC_FIFO_ALMOST_FULL {
    #[inline(always)]
    fn from(val: u8) -> ADC_FIFO_ALMOST_FULL {
        ADC_FIFO_ALMOST_FULL::from_bits(val)
    }
}
impl From<ADC_FIFO_ALMOST_FULL> for u8 {
    #[inline(always)]
    fn from(val: ADC_FIFO_ALMOST_FULL) -> u8 {
        ADC_FIFO_ALMOST_FULL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ADC_IRQ {
    #[doc = "High level."]
    HIGH = 0x0,
    #[doc = "Low level."]
    LOW = 0x01,
}
impl ADC_IRQ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ADC_IRQ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ADC_IRQ {
    #[inline(always)]
    fn from(val: u8) -> ADC_IRQ {
        ADC_IRQ::from_bits(val)
    }
}
impl From<ADC_IRQ> for u8 {
    #[inline(always)]
    fn from(val: ADC_IRQ) -> u8 {
        ADC_IRQ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AON_WU_EV {
    #[doc = "High level."]
    HIGH = 0x0,
    #[doc = "Low level."]
    LOW = 0x01,
}
impl AON_WU_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AON_WU_EV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AON_WU_EV {
    #[inline(always)]
    fn from(val: u8) -> AON_WU_EV {
        AON_WU_EV::from_bits(val)
    }
}
impl From<AON_WU_EV> for u8 {
    #[inline(always)]
    fn from(val: AON_WU_EV) -> u8 {
        AON_WU_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EVTOAONPOL_ADC_DONE {
    #[doc = "High level."]
    HIGH = 0x0,
    #[doc = "Low level."]
    LOW = 0x01,
}
impl EVTOAONPOL_ADC_DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EVTOAONPOL_ADC_DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EVTOAONPOL_ADC_DONE {
    #[inline(always)]
    fn from(val: u8) -> EVTOAONPOL_ADC_DONE {
        EVTOAONPOL_ADC_DONE::from_bits(val)
    }
}
impl From<EVTOAONPOL_ADC_DONE> for u8 {
    #[inline(always)]
    fn from(val: EVTOAONPOL_ADC_DONE) -> u8 {
        EVTOAONPOL_ADC_DONE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EVTOAONPOL_AUX_COMPA {
    #[doc = "Rising edge."]
    HIGH = 0x0,
    #[doc = "Falling edge."]
    LOW = 0x01,
}
impl EVTOAONPOL_AUX_COMPA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EVTOAONPOL_AUX_COMPA {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EVTOAONPOL_AUX_COMPA {
    #[inline(always)]
    fn from(val: u8) -> EVTOAONPOL_AUX_COMPA {
        EVTOAONPOL_AUX_COMPA::from_bits(val)
    }
}
impl From<EVTOAONPOL_AUX_COMPA> for u8 {
    #[inline(always)]
    fn from(val: EVTOAONPOL_AUX_COMPA) -> u8 {
        EVTOAONPOL_AUX_COMPA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EVTOAONPOL_AUX_COMPB {
    #[doc = "Rising edge."]
    HIGH = 0x0,
    #[doc = "Falling edge."]
    LOW = 0x01,
}
impl EVTOAONPOL_AUX_COMPB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EVTOAONPOL_AUX_COMPB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EVTOAONPOL_AUX_COMPB {
    #[inline(always)]
    fn from(val: u8) -> EVTOAONPOL_AUX_COMPB {
        EVTOAONPOL_AUX_COMPB::from_bits(val)
    }
}
impl From<EVTOAONPOL_AUX_COMPB> for u8 {
    #[inline(always)]
    fn from(val: EVTOAONPOL_AUX_COMPB) -> u8 {
        EVTOAONPOL_AUX_COMPB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EVTOAONPOL_TDC_DONE {
    #[doc = "High level."]
    HIGH = 0x0,
    #[doc = "Low level."]
    LOW = 0x01,
}
impl EVTOAONPOL_TDC_DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EVTOAONPOL_TDC_DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EVTOAONPOL_TDC_DONE {
    #[inline(always)]
    fn from(val: u8) -> EVTOAONPOL_TDC_DONE {
        EVTOAONPOL_TDC_DONE::from_bits(val)
    }
}
impl From<EVTOAONPOL_TDC_DONE> for u8 {
    #[inline(always)]
    fn from(val: EVTOAONPOL_TDC_DONE) -> u8 {
        EVTOAONPOL_TDC_DONE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EVTOAONPOL_TIMER0_EV {
    #[doc = "High level."]
    HIGH = 0x0,
    #[doc = "Low level."]
    LOW = 0x01,
}
impl EVTOAONPOL_TIMER0_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EVTOAONPOL_TIMER0_EV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EVTOAONPOL_TIMER0_EV {
    #[inline(always)]
    fn from(val: u8) -> EVTOAONPOL_TIMER0_EV {
        EVTOAONPOL_TIMER0_EV::from_bits(val)
    }
}
impl From<EVTOAONPOL_TIMER0_EV> for u8 {
    #[inline(always)]
    fn from(val: EVTOAONPOL_TIMER0_EV) -> u8 {
        EVTOAONPOL_TIMER0_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EVTOAONPOL_TIMER1_EV {
    #[doc = "High level."]
    HIGH = 0x0,
    #[doc = "Low level."]
    LOW = 0x01,
}
impl EVTOAONPOL_TIMER1_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EVTOAONPOL_TIMER1_EV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EVTOAONPOL_TIMER1_EV {
    #[inline(always)]
    fn from(val: u8) -> EVTOAONPOL_TIMER1_EV {
        EVTOAONPOL_TIMER1_EV::from_bits(val)
    }
}
impl From<EVTOAONPOL_TIMER1_EV> for u8 {
    #[inline(always)]
    fn from(val: EVTOAONPOL_TIMER1_EV) -> u8 {
        EVTOAONPOL_TIMER1_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EVTOMCUPOL_ADC_DONE {
    #[doc = "High level."]
    HIGH = 0x0,
    #[doc = "Low level."]
    LOW = 0x01,
}
impl EVTOMCUPOL_ADC_DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EVTOMCUPOL_ADC_DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EVTOMCUPOL_ADC_DONE {
    #[inline(always)]
    fn from(val: u8) -> EVTOMCUPOL_ADC_DONE {
        EVTOMCUPOL_ADC_DONE::from_bits(val)
    }
}
impl From<EVTOMCUPOL_ADC_DONE> for u8 {
    #[inline(always)]
    fn from(val: EVTOMCUPOL_ADC_DONE) -> u8 {
        EVTOMCUPOL_ADC_DONE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EVTOMCUPOL_AUX_COMPA {
    #[doc = "High level."]
    HIGH = 0x0,
    #[doc = "Low level."]
    LOW = 0x01,
}
impl EVTOMCUPOL_AUX_COMPA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EVTOMCUPOL_AUX_COMPA {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EVTOMCUPOL_AUX_COMPA {
    #[inline(always)]
    fn from(val: u8) -> EVTOMCUPOL_AUX_COMPA {
        EVTOMCUPOL_AUX_COMPA::from_bits(val)
    }
}
impl From<EVTOMCUPOL_AUX_COMPA> for u8 {
    #[inline(always)]
    fn from(val: EVTOMCUPOL_AUX_COMPA) -> u8 {
        EVTOMCUPOL_AUX_COMPA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EVTOMCUPOL_AUX_COMPB {
    #[doc = "High level."]
    HIGH = 0x0,
    #[doc = "Low level."]
    LOW = 0x01,
}
impl EVTOMCUPOL_AUX_COMPB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EVTOMCUPOL_AUX_COMPB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EVTOMCUPOL_AUX_COMPB {
    #[inline(always)]
    fn from(val: u8) -> EVTOMCUPOL_AUX_COMPB {
        EVTOMCUPOL_AUX_COMPB::from_bits(val)
    }
}
impl From<EVTOMCUPOL_AUX_COMPB> for u8 {
    #[inline(always)]
    fn from(val: EVTOMCUPOL_AUX_COMPB) -> u8 {
        EVTOMCUPOL_AUX_COMPB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EVTOMCUPOL_TDC_DONE {
    #[doc = "High level."]
    HIGH = 0x0,
    #[doc = "Low level."]
    LOW = 0x01,
}
impl EVTOMCUPOL_TDC_DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EVTOMCUPOL_TDC_DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EVTOMCUPOL_TDC_DONE {
    #[inline(always)]
    fn from(val: u8) -> EVTOMCUPOL_TDC_DONE {
        EVTOMCUPOL_TDC_DONE::from_bits(val)
    }
}
impl From<EVTOMCUPOL_TDC_DONE> for u8 {
    #[inline(always)]
    fn from(val: EVTOMCUPOL_TDC_DONE) -> u8 {
        EVTOMCUPOL_TDC_DONE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EVTOMCUPOL_TIMER0_EV {
    #[doc = "High level."]
    HIGH = 0x0,
    #[doc = "Low level."]
    LOW = 0x01,
}
impl EVTOMCUPOL_TIMER0_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EVTOMCUPOL_TIMER0_EV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EVTOMCUPOL_TIMER0_EV {
    #[inline(always)]
    fn from(val: u8) -> EVTOMCUPOL_TIMER0_EV {
        EVTOMCUPOL_TIMER0_EV::from_bits(val)
    }
}
impl From<EVTOMCUPOL_TIMER0_EV> for u8 {
    #[inline(always)]
    fn from(val: EVTOMCUPOL_TIMER0_EV) -> u8 {
        EVTOMCUPOL_TIMER0_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EVTOMCUPOL_TIMER1_EV {
    #[doc = "High level."]
    HIGH = 0x0,
    #[doc = "Low level."]
    LOW = 0x01,
}
impl EVTOMCUPOL_TIMER1_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EVTOMCUPOL_TIMER1_EV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EVTOMCUPOL_TIMER1_EV {
    #[inline(always)]
    fn from(val: u8) -> EVTOMCUPOL_TIMER1_EV {
        EVTOMCUPOL_TIMER1_EV::from_bits(val)
    }
}
impl From<EVTOMCUPOL_TIMER1_EV> for u8 {
    #[inline(always)]
    fn from(val: EVTOMCUPOL_TIMER1_EV) -> u8 {
        EVTOMCUPOL_TIMER1_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OBSMUX0 {
    #[doc = "High level."]
    HIGH = 0x0,
    #[doc = "Low level."]
    LOW = 0x01,
}
impl OBSMUX0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OBSMUX0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OBSMUX0 {
    #[inline(always)]
    fn from(val: u8) -> OBSMUX0 {
        OBSMUX0::from_bits(val)
    }
}
impl From<OBSMUX0> for u8 {
    #[inline(always)]
    fn from(val: OBSMUX0) -> u8 {
        OBSMUX0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum REQ_MODE {
    #[doc = "Burst requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    BURST = 0x0,
    #[doc = "Single requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    SINGLE = 0x01,
}
impl REQ_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> REQ_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for REQ_MODE {
    #[inline(always)]
    fn from(val: u8) -> REQ_MODE {
        REQ_MODE::from_bits(val)
    }
}
impl From<REQ_MODE> for u8 {
    #[inline(always)]
    fn from(val: REQ_MODE) -> u8 {
        REQ_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEL {
    #[doc = "UDMA0 trigger event will be generated when there are samples in the ADC FIFO."]
    FIFO_NOT_EMPTY = 0x0,
    #[doc = "UDMA0 trigger event will be generated when the ADC FIFO is almost full (3/4 full)."]
    FIFO_ALMOST_FULL = 0x01,
}
impl SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEL {
    #[inline(always)]
    fn from(val: u8) -> SEL {
        SEL::from_bits(val)
    }
}
impl From<SEL> for u8 {
    #[inline(always)]
    fn from(val: SEL) -> u8 {
        SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SMPH_AUTOTAKE_DONE {
    #[doc = "High level."]
    HIGH = 0x0,
    #[doc = "Low level."]
    LOW = 0x01,
}
impl SMPH_AUTOTAKE_DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SMPH_AUTOTAKE_DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SMPH_AUTOTAKE_DONE {
    #[inline(always)]
    fn from(val: u8) -> SMPH_AUTOTAKE_DONE {
        SMPH_AUTOTAKE_DONE::from_bits(val)
    }
}
impl From<SMPH_AUTOTAKE_DONE> for u8 {
    #[inline(always)]
    fn from(val: SMPH_AUTOTAKE_DONE) -> u8 {
        SMPH_AUTOTAKE_DONE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VEC0_EN {
    #[doc = "Disable vector 0 trigger."]
    DIS = 0x0,
    #[doc = "Enable vector 0 trigger."]
    EN = 0x01,
}
impl VEC0_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VEC0_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VEC0_EN {
    #[inline(always)]
    fn from(val: u8) -> VEC0_EN {
        VEC0_EN::from_bits(val)
    }
}
impl From<VEC0_EN> for u8 {
    #[inline(always)]
    fn from(val: VEC0_EN) -> u8 {
        VEC0_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VEC0_EV {
    #[doc = "EVSTAT0.AON_RTC_CH2."]
    AON_RTC_CH2 = 0x0,
    #[doc = "EVSTAT0.AUX_COMPA."]
    AUX_COMPA = 0x01,
    #[doc = "EVSTAT0.AUX_COMPB."]
    AUX_COMPB = 0x02,
    #[doc = "EVSTAT0.TDC_DONE."]
    TDC_DONE = 0x03,
    #[doc = "EVSTAT0.TIMER0_EV."]
    TIMER0_EV = 0x04,
    #[doc = "EVSTAT0.TIMER1_EV."]
    TIMER1_EV = 0x05,
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE."]
    SMPH_AUTOTAKE_DONE = 0x06,
    #[doc = "EVSTAT0.ADC_DONE."]
    ADC_DONE = 0x07,
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL."]
    ADC_FIFO_ALMOST_FULL = 0x08,
    #[doc = "EVSTAT0.OBSMUX0."]
    OBSMUX0 = 0x09,
    #[doc = "EVSTAT0.OBSMUX1."]
    OBSMUX1 = 0x0a,
    #[doc = "EVSTAT0.AON_SW."]
    AON_SW = 0x0b,
    #[doc = "EVSTAT0.AON_PROG_WU."]
    AON_PROG_WU = 0x0c,
    #[doc = "EVSTAT0.AUXIO0."]
    AUXIO0 = 0x0d,
    #[doc = "EVSTAT0.AUXIO1."]
    AUXIO1 = 0x0e,
    #[doc = "EVSTAT0.AUXIO2."]
    AUXIO2 = 0x0f,
    #[doc = "EVSTAT1.AUXIO3."]
    AUXIO3 = 0x10,
    #[doc = "EVSTAT1.AUXIO4."]
    AUXIO4 = 0x11,
    #[doc = "EVSTAT1.AUXIO5."]
    AUXIO5 = 0x12,
    #[doc = "EVSTAT1.AUXIO6."]
    AUXIO6 = 0x13,
    #[doc = "EVSTAT1.AUXIO7."]
    AUXIO7 = 0x14,
    #[doc = "EVSTAT1.AUXIO8."]
    AUXIO8 = 0x15,
    #[doc = "EVSTAT1.AUXIO9."]
    AUXIO9 = 0x16,
    #[doc = "EVSTAT1.AUXIO10."]
    AUXIO10 = 0x17,
    #[doc = "EVSTAT1.AUXIO11."]
    AUXIO11 = 0x18,
    #[doc = "EVSTAT1.AUXIO12."]
    AUXIO12 = 0x19,
    #[doc = "EVSTAT1.AUXIO13."]
    AUXIO13 = 0x1a,
    #[doc = "EVSTAT1.AUXIO14."]
    AUXIO14 = 0x1b,
    #[doc = "EVSTAT1.AUXIO15."]
    AUXIO15 = 0x1c,
    #[doc = "EVSTAT1.ACLK_REF."]
    ACLK_REF = 0x1d,
    #[doc = "EVSTAT1.MCU_EV."]
    MCU_EV = 0x1e,
    #[doc = "EVSTAT1.ADC_IRQ."]
    ADC_IRQ = 0x1f,
}
impl VEC0_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VEC0_EV {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VEC0_EV {
    #[inline(always)]
    fn from(val: u8) -> VEC0_EV {
        VEC0_EV::from_bits(val)
    }
}
impl From<VEC0_EV> for u8 {
    #[inline(always)]
    fn from(val: VEC0_EV) -> u8 {
        VEC0_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VEC0_POL {
    #[doc = "Rising edge triggers vector 0 execution."]
    RISE = 0x0,
    #[doc = "Falling edge triggers vector 0 execution."]
    FALL = 0x01,
}
impl VEC0_POL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VEC0_POL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VEC0_POL {
    #[inline(always)]
    fn from(val: u8) -> VEC0_POL {
        VEC0_POL::from_bits(val)
    }
}
impl From<VEC0_POL> for u8 {
    #[inline(always)]
    fn from(val: VEC0_POL) -> u8 {
        VEC0_POL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VEC1_EN {
    #[doc = "Disable vector 1 trigger."]
    DIS = 0x0,
    #[doc = "Enable vector 1 trigger."]
    EN = 0x01,
}
impl VEC1_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VEC1_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VEC1_EN {
    #[inline(always)]
    fn from(val: u8) -> VEC1_EN {
        VEC1_EN::from_bits(val)
    }
}
impl From<VEC1_EN> for u8 {
    #[inline(always)]
    fn from(val: VEC1_EN) -> u8 {
        VEC1_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VEC1_EV {
    #[doc = "EVSTAT0.AON_RTC_CH2."]
    AON_RTC_CH2 = 0x0,
    #[doc = "EVSTAT0.AUX_COMPA."]
    AUX_COMPA = 0x01,
    #[doc = "EVSTAT0.AUX_COMPB."]
    AUX_COMPB = 0x02,
    #[doc = "EVSTAT0.TDC_DONE."]
    TDC_DONE = 0x03,
    #[doc = "EVSTAT0.TIMER0_EV."]
    TIMER0_EV = 0x04,
    #[doc = "EVSTAT0.TIMER1_EV."]
    TIMER1_EV = 0x05,
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE."]
    SMPH_AUTOTAKE_DONE = 0x06,
    #[doc = "EVSTAT0.ADC_DONE."]
    ADC_DONE = 0x07,
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL."]
    ADC_FIFO_ALMOST_FULL = 0x08,
    #[doc = "EVSTAT0.OBSMUX0."]
    OBSMUX0 = 0x09,
    #[doc = "EVSTAT0.OBSMUX1."]
    OBSMUX1 = 0x0a,
    #[doc = "EVSTAT0.AON_SW."]
    AON_SW = 0x0b,
    #[doc = "EVSTAT0.AON_PROG_WU."]
    AON_PROG_WU = 0x0c,
    #[doc = "EVSTAT0.AUXIO0."]
    AUXIO0 = 0x0d,
    #[doc = "EVSTAT0.AUXIO1."]
    AUXIO1 = 0x0e,
    #[doc = "EVSTAT0.AUXIO2."]
    AUXIO2 = 0x0f,
    #[doc = "EVSTAT1.AUXIO3."]
    AUXIO3 = 0x10,
    #[doc = "EVSTAT1.AUXIO4."]
    AUXIO4 = 0x11,
    #[doc = "EVSTAT1.AUXIO5."]
    AUXIO5 = 0x12,
    #[doc = "EVSTAT1.AUXIO6."]
    AUXIO6 = 0x13,
    #[doc = "EVSTAT1.AUXIO7."]
    AUXIO7 = 0x14,
    #[doc = "EVSTAT1.AUXIO8."]
    AUXIO8 = 0x15,
    #[doc = "EVSTAT1.AUXIO9."]
    AUXIO9 = 0x16,
    #[doc = "EVSTAT1.AUXIO10."]
    AUXIO10 = 0x17,
    #[doc = "EVSTAT1.AUXIO11."]
    AUXIO11 = 0x18,
    #[doc = "EVSTAT1.AUXIO12."]
    AUXIO12 = 0x19,
    #[doc = "EVSTAT1.AUXIO13."]
    AUXIO13 = 0x1a,
    #[doc = "EVSTAT1.AUXIO14."]
    AUXIO14 = 0x1b,
    #[doc = "EVSTAT1.AUXIO15."]
    AUXIO15 = 0x1c,
    #[doc = "EVSTAT1.ACLK_REF."]
    ACLK_REF = 0x1d,
    #[doc = "EVSTAT1.MCU_EV."]
    MCU_EV = 0x1e,
    #[doc = "EVSTAT1.ADC_IRQ."]
    ADC_IRQ = 0x1f,
}
impl VEC1_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VEC1_EV {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VEC1_EV {
    #[inline(always)]
    fn from(val: u8) -> VEC1_EV {
        VEC1_EV::from_bits(val)
    }
}
impl From<VEC1_EV> for u8 {
    #[inline(always)]
    fn from(val: VEC1_EV) -> u8 {
        VEC1_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VEC1_POL {
    #[doc = "Rising edge triggers vector 1 execution."]
    RISE = 0x0,
    #[doc = "Falling edge triggers vector 1 execution."]
    FALL = 0x01,
}
impl VEC1_POL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VEC1_POL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VEC1_POL {
    #[inline(always)]
    fn from(val: u8) -> VEC1_POL {
        VEC1_POL::from_bits(val)
    }
}
impl From<VEC1_POL> for u8 {
    #[inline(always)]
    fn from(val: VEC1_POL) -> u8 {
        VEC1_POL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VEC2_EN {
    #[doc = "Disable vector 2 trigger."]
    DIS = 0x0,
    #[doc = "Enable vector 2 trigger."]
    EN = 0x01,
}
impl VEC2_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VEC2_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VEC2_EN {
    #[inline(always)]
    fn from(val: u8) -> VEC2_EN {
        VEC2_EN::from_bits(val)
    }
}
impl From<VEC2_EN> for u8 {
    #[inline(always)]
    fn from(val: VEC2_EN) -> u8 {
        VEC2_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VEC2_EV {
    #[doc = "EVSTAT0.AON_RTC_CH2."]
    AON_RTC_CH2 = 0x0,
    #[doc = "EVSTAT0.AUX_COMPA."]
    AUX_COMPA = 0x01,
    #[doc = "EVSTAT0.AUX_COMPB."]
    AUX_COMPB = 0x02,
    #[doc = "EVSTAT0.TDC_DONE."]
    TDC_DONE = 0x03,
    #[doc = "EVSTAT0.TIMER0_EV."]
    TIMER0_EV = 0x04,
    #[doc = "EVSTAT0.TIMER1_EV."]
    TIMER1_EV = 0x05,
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE."]
    SMPH_AUTOTAKE_DONE = 0x06,
    #[doc = "EVSTAT0.ADC_DONE."]
    ADC_DONE = 0x07,
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL."]
    ADC_FIFO_ALMOST_FULL = 0x08,
    #[doc = "EVSTAT0.OBSMUX0."]
    OBSMUX0 = 0x09,
    #[doc = "EVSTAT0.OBSMUX1."]
    OBSMUX1 = 0x0a,
    #[doc = "EVSTAT0.AON_SW."]
    AON_SW = 0x0b,
    #[doc = "EVSTAT0.AON_PROG_WU."]
    AON_PROG_WU = 0x0c,
    #[doc = "EVSTAT0.AUXIO0."]
    AUXIO0 = 0x0d,
    #[doc = "EVSTAT0.AUXIO1."]
    AUXIO1 = 0x0e,
    #[doc = "EVSTAT0.AUXIO2."]
    AUXIO2 = 0x0f,
    #[doc = "EVSTAT1.AUXIO3."]
    AUXIO3 = 0x10,
    #[doc = "EVSTAT1.AUXIO4."]
    AUXIO4 = 0x11,
    #[doc = "EVSTAT1.AUXIO5."]
    AUXIO5 = 0x12,
    #[doc = "EVSTAT1.AUXIO6."]
    AUXIO6 = 0x13,
    #[doc = "EVSTAT1.AUXIO7."]
    AUXIO7 = 0x14,
    #[doc = "EVSTAT1.AUXIO8."]
    AUXIO8 = 0x15,
    #[doc = "EVSTAT1.AUXIO9."]
    AUXIO9 = 0x16,
    #[doc = "EVSTAT1.AUXIO10."]
    AUXIO10 = 0x17,
    #[doc = "EVSTAT1.AUXIO11."]
    AUXIO11 = 0x18,
    #[doc = "EVSTAT1.AUXIO12."]
    AUXIO12 = 0x19,
    #[doc = "EVSTAT1.AUXIO13."]
    AUXIO13 = 0x1a,
    #[doc = "EVSTAT1.AUXIO14."]
    AUXIO14 = 0x1b,
    #[doc = "EVSTAT1.AUXIO15."]
    AUXIO15 = 0x1c,
    #[doc = "EVSTAT1.ACLK_REF."]
    ACLK_REF = 0x1d,
    #[doc = "EVSTAT1.MCU_EV."]
    MCU_EV = 0x1e,
    #[doc = "EVSTAT1.ADC_IRQ."]
    ADC_IRQ = 0x1f,
}
impl VEC2_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VEC2_EV {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VEC2_EV {
    #[inline(always)]
    fn from(val: u8) -> VEC2_EV {
        VEC2_EV::from_bits(val)
    }
}
impl From<VEC2_EV> for u8 {
    #[inline(always)]
    fn from(val: VEC2_EV) -> u8 {
        VEC2_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VEC2_POL {
    #[doc = "Rising edge triggers vector 2 execution."]
    RISE = 0x0,
    #[doc = "Falling edge triggers vector 2 execution."]
    FALL = 0x01,
}
impl VEC2_POL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VEC2_POL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VEC2_POL {
    #[inline(always)]
    fn from(val: u8) -> VEC2_POL {
        VEC2_POL::from_bits(val)
    }
}
impl From<VEC2_POL> for u8 {
    #[inline(always)]
    fn from(val: VEC2_POL) -> u8 {
        VEC2_POL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VEC3_EN {
    #[doc = "Disable vector 3 trigger."]
    DIS = 0x0,
    #[doc = "Enable vector 3 trigger."]
    EN = 0x01,
}
impl VEC3_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VEC3_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VEC3_EN {
    #[inline(always)]
    fn from(val: u8) -> VEC3_EN {
        VEC3_EN::from_bits(val)
    }
}
impl From<VEC3_EN> for u8 {
    #[inline(always)]
    fn from(val: VEC3_EN) -> u8 {
        VEC3_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VEC3_EV {
    #[doc = "EVSTAT0.AON_RTC_CH2."]
    AON_RTC_CH2 = 0x0,
    #[doc = "EVSTAT0.AUX_COMPA."]
    AUX_COMPA = 0x01,
    #[doc = "EVSTAT0.AUX_COMPB."]
    AUX_COMPB = 0x02,
    #[doc = "EVSTAT0.TDC_DONE."]
    TDC_DONE = 0x03,
    #[doc = "EVSTAT0.TIMER0_EV."]
    TIMER0_EV = 0x04,
    #[doc = "EVSTAT0.TIMER1_EV."]
    TIMER1_EV = 0x05,
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE."]
    SMPH_AUTOTAKE_DONE = 0x06,
    #[doc = "EVSTAT0.ADC_DONE."]
    ADC_DONE = 0x07,
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL."]
    ADC_FIFO_ALMOST_FULL = 0x08,
    #[doc = "EVSTAT0.OBSMUX0."]
    OBSMUX0 = 0x09,
    #[doc = "EVSTAT0.OBSMUX1."]
    OBSMUX1 = 0x0a,
    #[doc = "EVSTAT0.AON_SW."]
    AON_SW = 0x0b,
    #[doc = "EVSTAT0.AON_PROG_WU."]
    AON_PROG_WU = 0x0c,
    #[doc = "EVSTAT0.AUXIO0."]
    AUXIO0 = 0x0d,
    #[doc = "EVSTAT0.AUXIO1."]
    AUXIO1 = 0x0e,
    #[doc = "EVSTAT0.AUXIO2."]
    AUXIO2 = 0x0f,
    #[doc = "EVSTAT1.AUXIO3."]
    AUXIO3 = 0x10,
    #[doc = "EVSTAT1.AUXIO4."]
    AUXIO4 = 0x11,
    #[doc = "EVSTAT1.AUXIO5."]
    AUXIO5 = 0x12,
    #[doc = "EVSTAT1.AUXIO6."]
    AUXIO6 = 0x13,
    #[doc = "EVSTAT1.AUXIO7."]
    AUXIO7 = 0x14,
    #[doc = "EVSTAT1.AUXIO8."]
    AUXIO8 = 0x15,
    #[doc = "EVSTAT1.AUXIO9."]
    AUXIO9 = 0x16,
    #[doc = "EVSTAT1.AUXIO10."]
    AUXIO10 = 0x17,
    #[doc = "EVSTAT1.AUXIO11."]
    AUXIO11 = 0x18,
    #[doc = "EVSTAT1.AUXIO12."]
    AUXIO12 = 0x19,
    #[doc = "EVSTAT1.AUXIO13."]
    AUXIO13 = 0x1a,
    #[doc = "EVSTAT1.AUXIO14."]
    AUXIO14 = 0x1b,
    #[doc = "EVSTAT1.AUXIO15."]
    AUXIO15 = 0x1c,
    #[doc = "EVSTAT1.ACLK_REF."]
    ACLK_REF = 0x1d,
    #[doc = "EVSTAT1.MCU_EV."]
    MCU_EV = 0x1e,
    #[doc = "EVSTAT1.ADC_IRQ."]
    ADC_IRQ = 0x1f,
}
impl VEC3_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VEC3_EV {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VEC3_EV {
    #[inline(always)]
    fn from(val: u8) -> VEC3_EV {
        VEC3_EV::from_bits(val)
    }
}
impl From<VEC3_EV> for u8 {
    #[inline(always)]
    fn from(val: VEC3_EV) -> u8 {
        VEC3_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VEC3_POL {
    #[doc = "Rising edge triggers vector 3 execution."]
    RISE = 0x0,
    #[doc = "Falling edge triggers vector 3 execution."]
    FALL = 0x01,
}
impl VEC3_POL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VEC3_POL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VEC3_POL {
    #[inline(always)]
    fn from(val: u8) -> VEC3_POL {
        VEC3_POL::from_bits(val)
    }
}
impl From<VEC3_POL> for u8 {
    #[inline(always)]
    fn from(val: VEC3_POL) -> u8 {
        VEC3_POL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WEV7_EV {
    #[doc = "EVSTAT0.AON_RTC_CH2."]
    AON_RTC_CH2 = 0x0,
    #[doc = "EVSTAT0.AUX_COMPA."]
    AUX_COMPA = 0x01,
    #[doc = "EVSTAT0.AUX_COMPB."]
    AUX_COMPB = 0x02,
    #[doc = "EVSTAT0.TDC_DONE."]
    TDC_DONE = 0x03,
    #[doc = "EVSTAT0.TIMER0_EV."]
    TIMER0_EV = 0x04,
    #[doc = "EVSTAT0.TIMER1_EV."]
    TIMER1_EV = 0x05,
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE."]
    SMPH_AUTOTAKE_DONE = 0x06,
    #[doc = "EVSTAT0.ADC_DONE."]
    ADC_DONE = 0x07,
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL."]
    ADC_FIFO_ALMOST_FULL = 0x08,
    #[doc = "EVSTAT0.OBSMUX0."]
    OBSMUX0 = 0x09,
    #[doc = "EVSTAT0.OBSMUX1."]
    OBSMUX1 = 0x0a,
    #[doc = "EVSTAT0.AON_SW."]
    AON_SW = 0x0b,
    #[doc = "EVSTAT0.AON_PROG_WU."]
    AON_PROG_WU = 0x0c,
    #[doc = "EVSTAT0.AUXIO0."]
    AUXIO0 = 0x0d,
    #[doc = "EVSTAT0.AUXIO1."]
    AUXIO1 = 0x0e,
    #[doc = "EVSTAT0.AUXIO2."]
    AUXIO2 = 0x0f,
    #[doc = "EVSTAT1.AUXIO3."]
    AUXIO3 = 0x10,
    #[doc = "EVSTAT1.AUXIO4."]
    AUXIO4 = 0x11,
    #[doc = "EVSTAT1.AUXIO5."]
    AUXIO5 = 0x12,
    #[doc = "EVSTAT1.AUXIO6."]
    AUXIO6 = 0x13,
    #[doc = "EVSTAT1.AUXIO7."]
    AUXIO7 = 0x14,
    #[doc = "EVSTAT1.AUXIO8."]
    AUXIO8 = 0x15,
    #[doc = "EVSTAT1.AUXIO9."]
    AUXIO9 = 0x16,
    #[doc = "EVSTAT1.AUXIO10."]
    AUXIO10 = 0x17,
    #[doc = "EVSTAT1.AUXIO11."]
    AUXIO11 = 0x18,
    #[doc = "EVSTAT1.AUXIO12."]
    AUXIO12 = 0x19,
    #[doc = "EVSTAT1.AUXIO13."]
    AUXIO13 = 0x1a,
    #[doc = "EVSTAT1.AUXIO14."]
    AUXIO14 = 0x1b,
    #[doc = "EVSTAT1.AUXIO15."]
    AUXIO15 = 0x1c,
    #[doc = "EVSTAT1.ACLK_REF."]
    ACLK_REF = 0x1d,
    #[doc = "EVSTAT1.MCU_EV."]
    MCU_EV = 0x1e,
    #[doc = "EVSTAT1.ADC_IRQ."]
    ADC_IRQ = 0x1f,
}
impl WEV7_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WEV7_EV {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WEV7_EV {
    #[inline(always)]
    fn from(val: u8) -> WEV7_EV {
        WEV7_EV::from_bits(val)
    }
}
impl From<WEV7_EV> for u8 {
    #[inline(always)]
    fn from(val: WEV7_EV) -> u8 {
        WEV7_EV::to_bits(val)
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CAEIM {
    #[doc = "Disable Interrupt."]
    DIS = 0x0,
    #[doc = "Enable Interrupt."]
    EN = 0x01,
}
impl CAEIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CAEIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CAEIM {
    #[inline(always)]
    fn from(val: u8) -> CAEIM {
        CAEIM::from_bits(val)
    }
}
impl From<CAEIM> for u8 {
    #[inline(always)]
    fn from(val: CAEIM) -> u8 {
        CAEIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CAMIM {
    #[doc = "Disable Interrupt."]
    DIS = 0x0,
    #[doc = "Enable Interrupt."]
    EN = 0x01,
}
impl CAMIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CAMIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CAMIM {
    #[inline(always)]
    fn from(val: u8) -> CAMIM {
        CAMIM::from_bits(val)
    }
}
impl From<CAMIM> for u8 {
    #[inline(always)]
    fn from(val: CAMIM) -> u8 {
        CAMIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CBEIM {
    #[doc = "Disable Interrupt."]
    DIS = 0x0,
    #[doc = "Enable Interrupt."]
    EN = 0x01,
}
impl CBEIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CBEIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CBEIM {
    #[inline(always)]
    fn from(val: u8) -> CBEIM {
        CBEIM::from_bits(val)
    }
}
impl From<CBEIM> for u8 {
    #[inline(always)]
    fn from(val: CBEIM) -> u8 {
        CBEIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CBMIM {
    #[doc = "Disable Interrupt."]
    DIS = 0x0,
    #[doc = "Enable Interrupt."]
    EN = 0x01,
}
impl CBMIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CBMIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CBMIM {
    #[inline(always)]
    fn from(val: u8) -> CBMIM {
        CBMIM::from_bits(val)
    }
}
impl From<CBMIM> for u8 {
    #[inline(always)]
    fn from(val: CBMIM) -> u8 {
        CBMIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CFG {
    #[doc = "32-bit timer configuration."]
    _32BIT_TIMER = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "16-bit timer configuration. Configure for two 16-bit timers. Also see TAMR.TAMR and TBMR.TBMR."]
    _16BIT_TIMER = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl CFG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CFG {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CFG {
    #[inline(always)]
    fn from(val: u8) -> CFG {
        CFG::from_bits(val)
    }
}
impl From<CFG> for u8 {
    #[inline(always)]
    fn from(val: CFG) -> u8 {
        CFG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMAAIM {
    #[doc = "Disable Interrupt."]
    DIS = 0x0,
    #[doc = "Enable Interrupt."]
    EN = 0x01,
}
impl DMAAIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMAAIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMAAIM {
    #[inline(always)]
    fn from(val: u8) -> DMAAIM {
        DMAAIM::from_bits(val)
    }
}
impl From<DMAAIM> for u8 {
    #[inline(always)]
    fn from(val: DMAAIM) -> u8 {
        DMAAIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMABIM {
    #[doc = "Disable Interrupt."]
    DIS = 0x0,
    #[doc = "Enable Interrupt."]
    EN = 0x01,
}
impl DMABIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMABIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMABIM {
    #[inline(always)]
    fn from(val: u8) -> DMABIM {
        DMABIM::from_bits(val)
    }
}
impl From<DMABIM> for u8 {
    #[inline(always)]
    fn from(val: DMABIM) -> u8 {
        DMABIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYNC0 {
    #[doc = "No Sync. GPT0 is not affected."]
    NOSYNC = 0x0,
    #[doc = "A timeout event for Timer A of GPT0 is triggered."]
    TIMERA = 0x01,
    #[doc = "A timeout event for Timer B of GPT0 is triggered."]
    TIMERB = 0x02,
    #[doc = "A timeout event for both Timer A and Timer B of GPT0 is triggered."]
    BOTH = 0x03,
}
impl SYNC0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYNC0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYNC0 {
    #[inline(always)]
    fn from(val: u8) -> SYNC0 {
        SYNC0::from_bits(val)
    }
}
impl From<SYNC0> for u8 {
    #[inline(always)]
    fn from(val: SYNC0) -> u8 {
        SYNC0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYNC1 {
    #[doc = "No Sync. GPT1 is not affected."]
    NOSYNC = 0x0,
    #[doc = "A timeout event for Timer A of GPT1 is triggered."]
    TIMERA = 0x01,
    #[doc = "A timeout event for Timer B of GPT1 is triggered."]
    TIMERB = 0x02,
    #[doc = "A timeout event for both Timer A and Timer B of GPT1 is triggered."]
    BOTH = 0x03,
}
impl SYNC1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYNC1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYNC1 {
    #[inline(always)]
    fn from(val: u8) -> SYNC1 {
        SYNC1::from_bits(val)
    }
}
impl From<SYNC1> for u8 {
    #[inline(always)]
    fn from(val: SYNC1) -> u8 {
        SYNC1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYNC2 {
    #[doc = "No Sync. GPT2 is not affected."]
    NOSYNC = 0x0,
    #[doc = "A timeout event for Timer A of GPT2 is triggered."]
    TIMERA = 0x01,
    #[doc = "A timeout event for Timer B of GPT2 is triggered."]
    TIMERB = 0x02,
    #[doc = "A timeout event for both Timer A and Timer B of GPT2 is triggered."]
    BOTH = 0x03,
}
impl SYNC2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYNC2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYNC2 {
    #[inline(always)]
    fn from(val: u8) -> SYNC2 {
        SYNC2::from_bits(val)
    }
}
impl From<SYNC2> for u8 {
    #[inline(always)]
    fn from(val: SYNC2) -> u8 {
        SYNC2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYNC3 {
    #[doc = "No Sync. GPT3 is not affected."]
    NOSYNC = 0x0,
    #[doc = "A timeout event for Timer A of GPT3 is triggered."]
    TIMERA = 0x01,
    #[doc = "A timeout event for Timer B of GPT3 is triggered."]
    TIMERB = 0x02,
    #[doc = "A timeout event for both Timer A and Timer B of GPT3 is triggered."]
    BOTH = 0x03,
}
impl SYNC3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYNC3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYNC3 {
    #[inline(always)]
    fn from(val: u8) -> SYNC3 {
        SYNC3::from_bits(val)
    }
}
impl From<SYNC3> for u8 {
    #[inline(always)]
    fn from(val: SYNC3) -> u8 {
        SYNC3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TAAMS {
    #[doc = "Capture/Compare mode is enabled."]
    CAP_COMP = 0x0,
    #[doc = "PWM mode is enabled."]
    PWM = 0x01,
}
impl TAAMS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TAAMS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TAAMS {
    #[inline(always)]
    fn from(val: u8) -> TAAMS {
        TAAMS::from_bits(val)
    }
}
impl From<TAAMS> for u8 {
    #[inline(always)]
    fn from(val: TAAMS) -> u8 {
        TAAMS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TACDIR {
    #[doc = "The timer counts down."]
    DOWN = 0x0,
    #[doc = "The timer counts up. When counting up, the timer starts from a value of 0x0."]
    UP = 0x01,
}
impl TACDIR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TACDIR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TACDIR {
    #[inline(always)]
    fn from(val: u8) -> TACDIR {
        TACDIR::from_bits(val)
    }
}
impl From<TACDIR> for u8 {
    #[inline(always)]
    fn from(val: TACDIR) -> u8 {
        TACDIR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TACINTD {
    #[doc = "Time-out interrupt function as normal."]
    EN_TO_INTR = 0x0,
    #[doc = "Time-out interrupt are disabled."]
    DIS_TO_INTR = 0x01,
}
impl TACINTD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TACINTD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TACINTD {
    #[inline(always)]
    fn from(val: u8) -> TACINTD {
        TACINTD::from_bits(val)
    }
}
impl From<TACINTD> for u8 {
    #[inline(always)]
    fn from(val: TACINTD) -> u8 {
        TACINTD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TACM {
    #[doc = "Edge-Count mode."]
    EDGCNT = 0x0,
    #[doc = "Edge-Time mode."]
    EDGTIME = 0x01,
}
impl TACM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TACM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TACM {
    #[inline(always)]
    fn from(val: u8) -> TACM {
        TACM::from_bits(val)
    }
}
impl From<TACM> for u8 {
    #[inline(always)]
    fn from(val: TACM) -> u8 {
        TACM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TAEN {
    #[doc = "Timer A is disabled."]
    DIS = 0x0,
    #[doc = "Timer A is enabled and begins counting or the capture logic is enabled based on the CFG register."]
    EN = 0x01,
}
impl TAEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TAEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TAEN {
    #[inline(always)]
    fn from(val: u8) -> TAEN {
        TAEN::from_bits(val)
    }
}
impl From<TAEN> for u8 {
    #[inline(always)]
    fn from(val: TAEN) -> u8 {
        TAEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TAEVENT {
    #[doc = "Positive edge."]
    POS = 0x0,
    #[doc = "Negative edge."]
    NEG = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Both edges."]
    BOTH = 0x03,
}
impl TAEVENT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TAEVENT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TAEVENT {
    #[inline(always)]
    fn from(val: u8) -> TAEVENT {
        TAEVENT::from_bits(val)
    }
}
impl From<TAEVENT> for u8 {
    #[inline(always)]
    fn from(val: TAEVENT) -> u8 {
        TAEVENT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TAILD {
    #[doc = "Update the TAR register with the value in the TAILR register on the next clock cycle. If the pre-scaler is used, update the TAPS register with the value in the TAPR register on the next clock cycle."]
    CYCLEUPDATE = 0x0,
    #[doc = "Update the TAR register with the value in the TAILR register on the next timeout. If the prescaler is used, update the TAPS register with the value in the TAPR register on the next timeout."]
    TOUPDATE = 0x01,
}
impl TAILD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TAILD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TAILD {
    #[inline(always)]
    fn from(val: u8) -> TAILD {
        TAILD::from_bits(val)
    }
}
impl From<TAILD> for u8 {
    #[inline(always)]
    fn from(val: TAILD) -> u8 {
        TAILD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TAMIE {
    #[doc = "The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    DIS = 0x0,
    #[doc = "An interrupt is generated when the match value in TAMATCHR is reached in the one-shot and periodic modes."]
    EN = 0x01,
}
impl TAMIE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TAMIE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TAMIE {
    #[inline(always)]
    fn from(val: u8) -> TAMIE {
        TAMIE::from_bits(val)
    }
}
impl From<TAMIE> for u8 {
    #[inline(always)]
    fn from(val: TAMIE) -> u8 {
        TAMIE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TAMIM {
    #[doc = "Disable Interrupt."]
    DIS = 0x0,
    #[doc = "Enable Interrupt."]
    EN = 0x01,
}
impl TAMIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TAMIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TAMIM {
    #[inline(always)]
    fn from(val: u8) -> TAMIM {
        TAMIM::from_bits(val)
    }
}
impl From<TAMIM> for u8 {
    #[inline(always)]
    fn from(val: TAMIM) -> u8 {
        TAMIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TAMR {
    _RESERVED_0 = 0x0,
    #[doc = "One-Shot Timer mode."]
    ONE_SHOT = 0x01,
    #[doc = "Periodic Timer mode."]
    PERIODIC = 0x02,
    #[doc = "Capture mode."]
    CAPTURE = 0x03,
}
impl TAMR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TAMR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TAMR {
    #[inline(always)]
    fn from(val: u8) -> TAMR {
        TAMR::from_bits(val)
    }
}
impl From<TAMR> for u8 {
    #[inline(always)]
    fn from(val: TAMR) -> u8 {
        TAMR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TAMRSU {
    #[doc = "Update TAMATCHR and TAPR, if used, on the next cycle."]
    CYCLEUPDATE = 0x0,
    #[doc = "Update TAMATCHR and TAPR, if used, on the next time-out."]
    TOUPDATE = 0x01,
}
impl TAMRSU {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TAMRSU {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TAMRSU {
    #[inline(always)]
    fn from(val: u8) -> TAMRSU {
        TAMRSU::from_bits(val)
    }
}
impl From<TAMRSU> for u8 {
    #[inline(always)]
    fn from(val: TAMRSU) -> u8 {
        TAMRSU::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TAMR_TCACT {
    #[doc = "Disable compare operations."]
    DIS_CMP = 0x0,
    #[doc = "Toggle State on Time-Out."]
    TOG_ON_TO = 0x01,
    #[doc = "Clear CCP output pin on Time-Out."]
    CLR_ON_TO = 0x02,
    #[doc = "Set CCP output pin on Time-Out."]
    SET_ON_TO = 0x03,
    #[doc = "Set CCP output pin immediately and toggle on Time-Out."]
    SETTOG_ON_TO = 0x04,
    #[doc = "Clear CCP output pin immediately and toggle on Time-Out."]
    CLRTOG_ON_TO = 0x05,
    #[doc = "Set CCP output pin immediately and clear on Time-Out."]
    SETCLR_ON_TO = 0x06,
    #[doc = "Clear CCP output pin immediately and set on Time-Out."]
    CLRSET_ON_TO = 0x07,
}
impl TAMR_TCACT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TAMR_TCACT {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TAMR_TCACT {
    #[inline(always)]
    fn from(val: u8) -> TAMR_TCACT {
        TAMR_TCACT::from_bits(val)
    }
}
impl From<TAMR_TCACT> for u8 {
    #[inline(always)]
    fn from(val: TAMR_TCACT) -> u8 {
        TAMR_TCACT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TAPLO {
    #[doc = "Legacy operation."]
    LEGACY = 0x0,
    #[doc = "CCP output pin is set to 1 on time-out."]
    CCP_ON_TO = 0x01,
}
impl TAPLO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TAPLO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TAPLO {
    #[inline(always)]
    fn from(val: u8) -> TAPLO {
        TAPLO::from_bits(val)
    }
}
impl From<TAPLO> for u8 {
    #[inline(always)]
    fn from(val: TAPLO) -> u8 {
        TAPLO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TAPWMIE {
    #[doc = "Interrupt is disabled."]
    DIS = 0x0,
    #[doc = "Interrupt is enabled. This bit is only valid in PWM mode."]
    EN = 0x01,
}
impl TAPWMIE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TAPWMIE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TAPWMIE {
    #[inline(always)]
    fn from(val: u8) -> TAPWMIE {
        TAPWMIE::from_bits(val)
    }
}
impl From<TAPWMIE> for u8 {
    #[inline(always)]
    fn from(val: TAPWMIE) -> u8 {
        TAPWMIE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TAPWML {
    #[doc = "Not inverted."]
    NORMAL = 0x0,
    #[doc = "Inverted."]
    INVERTED = 0x01,
}
impl TAPWML {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TAPWML {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TAPWML {
    #[inline(always)]
    fn from(val: u8) -> TAPWML {
        TAPWML::from_bits(val)
    }
}
impl From<TAPWML> for u8 {
    #[inline(always)]
    fn from(val: TAPWML) -> u8 {
        TAPWML::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TASNAPS {
    #[doc = "Snap-shot mode is disabled."]
    DIS = 0x0,
    #[doc = "If Timer A is configured in the periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPT Timer A (TAR) register."]
    EN = 0x01,
}
impl TASNAPS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TASNAPS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TASNAPS {
    #[inline(always)]
    fn from(val: u8) -> TASNAPS {
        TASNAPS::from_bits(val)
    }
}
impl From<TASNAPS> for u8 {
    #[inline(always)]
    fn from(val: TASNAPS) -> u8 {
        TASNAPS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TASTALL {
    #[doc = "Timer A continues counting while the processor is halted by the debugger."]
    DIS = 0x0,
    #[doc = "Timer A freezes counting while the processor is halted by the debugger."]
    EN = 0x01,
}
impl TASTALL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TASTALL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TASTALL {
    #[inline(always)]
    fn from(val: u8) -> TASTALL {
        TASTALL::from_bits(val)
    }
}
impl From<TASTALL> for u8 {
    #[inline(always)]
    fn from(val: TASTALL) -> u8 {
        TASTALL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TATOIM {
    #[doc = "Disable Interrupt."]
    DIS = 0x0,
    #[doc = "Enable Interrupt."]
    EN = 0x01,
}
impl TATOIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TATOIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TATOIM {
    #[inline(always)]
    fn from(val: u8) -> TATOIM {
        TATOIM::from_bits(val)
    }
}
impl From<TATOIM> for u8 {
    #[inline(always)]
    fn from(val: TATOIM) -> u8 {
        TATOIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TAWOT {
    #[doc = "Timer A begins counting as soon as it is enabled."]
    NOWAIT = 0x0,
    #[doc = "If Timer A is enabled (CTL.TAEN = 1), Timer A does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This bit must be clear for GPT Module 0, Timer A. This function is valid for one-shot, periodic, and PWM modes."]
    WAIT = 0x01,
}
impl TAWOT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TAWOT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TAWOT {
    #[inline(always)]
    fn from(val: u8) -> TAWOT {
        TAWOT::from_bits(val)
    }
}
impl From<TAWOT> for u8 {
    #[inline(always)]
    fn from(val: TAWOT) -> u8 {
        TAWOT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBAMS {
    #[doc = "Capture/Compare mode is enabled."]
    CAP_COMP = 0x0,
    #[doc = "PWM mode is enabled."]
    PWM = 0x01,
}
impl TBAMS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBAMS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBAMS {
    #[inline(always)]
    fn from(val: u8) -> TBAMS {
        TBAMS::from_bits(val)
    }
}
impl From<TBAMS> for u8 {
    #[inline(always)]
    fn from(val: TBAMS) -> u8 {
        TBAMS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBCDIR {
    #[doc = "The timer counts down."]
    DOWN = 0x0,
    #[doc = "The timer counts up. When counting up, the timer starts from a value of 0x0."]
    UP = 0x01,
}
impl TBCDIR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBCDIR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBCDIR {
    #[inline(always)]
    fn from(val: u8) -> TBCDIR {
        TBCDIR::from_bits(val)
    }
}
impl From<TBCDIR> for u8 {
    #[inline(always)]
    fn from(val: TBCDIR) -> u8 {
        TBCDIR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBCINTD {
    #[doc = "Normal Time-Out Interrupt."]
    EN_TO_INTR = 0x0,
    #[doc = "Mask Time-Out Interrupt."]
    DIS_TO_INTR = 0x01,
}
impl TBCINTD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBCINTD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBCINTD {
    #[inline(always)]
    fn from(val: u8) -> TBCINTD {
        TBCINTD::from_bits(val)
    }
}
impl From<TBCINTD> for u8 {
    #[inline(always)]
    fn from(val: TBCINTD) -> u8 {
        TBCINTD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBCM {
    #[doc = "Edge-Count mode."]
    EDGCNT = 0x0,
    #[doc = "Edge-Time mode."]
    EDGTIME = 0x01,
}
impl TBCM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBCM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBCM {
    #[inline(always)]
    fn from(val: u8) -> TBCM {
        TBCM::from_bits(val)
    }
}
impl From<TBCM> for u8 {
    #[inline(always)]
    fn from(val: TBCM) -> u8 {
        TBCM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBEN {
    #[doc = "Timer B is disabled."]
    DIS = 0x0,
    #[doc = "Timer B is enabled and begins counting or the capture logic is enabled based on CFG register."]
    EN = 0x01,
}
impl TBEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBEN {
    #[inline(always)]
    fn from(val: u8) -> TBEN {
        TBEN::from_bits(val)
    }
}
impl From<TBEN> for u8 {
    #[inline(always)]
    fn from(val: TBEN) -> u8 {
        TBEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBEVENT {
    #[doc = "Positive edge."]
    POS = 0x0,
    #[doc = "Negative edge."]
    NEG = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Both edges."]
    BOTH = 0x03,
}
impl TBEVENT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBEVENT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBEVENT {
    #[inline(always)]
    fn from(val: u8) -> TBEVENT {
        TBEVENT::from_bits(val)
    }
}
impl From<TBEVENT> for u8 {
    #[inline(always)]
    fn from(val: TBEVENT) -> u8 {
        TBEVENT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBILD {
    #[doc = "Update the TBR register with the value in the TBILR register on the next clock cycle. If the pre-scaler is used, update the TBPS register with the value in the TBPR register on the next clock cycle."]
    CYCLEUPDATE = 0x0,
    #[doc = "Update the TBR register with the value in the TBILR register on the next timeout. If the prescaler is used, update the TBPS register with the value in the TBPR register on the next timeout."]
    TOUPDATE = 0x01,
}
impl TBILD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBILD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBILD {
    #[inline(always)]
    fn from(val: u8) -> TBILD {
        TBILD::from_bits(val)
    }
}
impl From<TBILD> for u8 {
    #[inline(always)]
    fn from(val: TBILD) -> u8 {
        TBILD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBMIE {
    #[doc = "The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    DIS = 0x0,
    #[doc = "An interrupt is generated when the match value in the TBMATCHR register is reached in the one-shot and periodic modes."]
    EN = 0x01,
}
impl TBMIE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBMIE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBMIE {
    #[inline(always)]
    fn from(val: u8) -> TBMIE {
        TBMIE::from_bits(val)
    }
}
impl From<TBMIE> for u8 {
    #[inline(always)]
    fn from(val: TBMIE) -> u8 {
        TBMIE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBMIM {
    #[doc = "Disable Interrupt."]
    DIS = 0x0,
    #[doc = "Enable Interrupt."]
    EN = 0x01,
}
impl TBMIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBMIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBMIM {
    #[inline(always)]
    fn from(val: u8) -> TBMIM {
        TBMIM::from_bits(val)
    }
}
impl From<TBMIM> for u8 {
    #[inline(always)]
    fn from(val: TBMIM) -> u8 {
        TBMIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBMR {
    _RESERVED_0 = 0x0,
    #[doc = "One-Shot Timer mode."]
    ONE_SHOT = 0x01,
    #[doc = "Periodic Timer mode."]
    PERIODIC = 0x02,
    #[doc = "Capture mode."]
    CAPTURE = 0x03,
}
impl TBMR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBMR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBMR {
    #[inline(always)]
    fn from(val: u8) -> TBMR {
        TBMR::from_bits(val)
    }
}
impl From<TBMR> for u8 {
    #[inline(always)]
    fn from(val: TBMR) -> u8 {
        TBMR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBMRSU {
    #[doc = "Update TBMATCHR and TBPR, if used, on the next cycle."]
    CYCLEUPDATE = 0x0,
    #[doc = "Update TBMATCHR and TBPR, if used, on the next time-out."]
    TOUPDATE = 0x01,
}
impl TBMRSU {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBMRSU {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBMRSU {
    #[inline(always)]
    fn from(val: u8) -> TBMRSU {
        TBMRSU::from_bits(val)
    }
}
impl From<TBMRSU> for u8 {
    #[inline(always)]
    fn from(val: TBMRSU) -> u8 {
        TBMRSU::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBMR_TCACT {
    #[doc = "Disable compare operations."]
    DIS_CMP = 0x0,
    #[doc = "Toggle State on Time-Out."]
    TOG_ON_TO = 0x01,
    #[doc = "Clear CCP output pin on Time-Out."]
    CLR_ON_TO = 0x02,
    #[doc = "Set CCP output pin on Time-Out."]
    SET_ON_TO = 0x03,
    #[doc = "Set CCP output pin immediately and toggle on Time-Out."]
    SETTOG_ON_TO = 0x04,
    #[doc = "Clear CCP output pin immediately and toggle on Time-Out."]
    CLRTOG_ON_TO = 0x05,
    #[doc = "Set CCP output pin immediately and clear on Time-Out."]
    SETCLR_ON_TO = 0x06,
    #[doc = "Clear CCP output pin immediately and set on Time-Out."]
    CLRSET_ON_TO = 0x07,
}
impl TBMR_TCACT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBMR_TCACT {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBMR_TCACT {
    #[inline(always)]
    fn from(val: u8) -> TBMR_TCACT {
        TBMR_TCACT::from_bits(val)
    }
}
impl From<TBMR_TCACT> for u8 {
    #[inline(always)]
    fn from(val: TBMR_TCACT) -> u8 {
        TBMR_TCACT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBPLO {
    #[doc = "Legacy operation."]
    LEGACY = 0x0,
    #[doc = "CCP output pin is set to 1 on time-out."]
    CCP_ON_TO = 0x01,
}
impl TBPLO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBPLO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBPLO {
    #[inline(always)]
    fn from(val: u8) -> TBPLO {
        TBPLO::from_bits(val)
    }
}
impl From<TBPLO> for u8 {
    #[inline(always)]
    fn from(val: TBPLO) -> u8 {
        TBPLO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBPWMIE {
    #[doc = "Interrupt is disabled."]
    DIS = 0x0,
    #[doc = "Interrupt is enabled. This bit is only valid in PWM mode."]
    EN = 0x01,
}
impl TBPWMIE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBPWMIE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBPWMIE {
    #[inline(always)]
    fn from(val: u8) -> TBPWMIE {
        TBPWMIE::from_bits(val)
    }
}
impl From<TBPWMIE> for u8 {
    #[inline(always)]
    fn from(val: TBPWMIE) -> u8 {
        TBPWMIE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBPWML {
    #[doc = "Not inverted."]
    NORMAL = 0x0,
    #[doc = "Inverted."]
    INVERTED = 0x01,
}
impl TBPWML {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBPWML {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBPWML {
    #[inline(always)]
    fn from(val: u8) -> TBPWML {
        TBPWML::from_bits(val)
    }
}
impl From<TBPWML> for u8 {
    #[inline(always)]
    fn from(val: TBPWML) -> u8 {
        TBPWML::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBSNAPS {
    #[doc = "Snap-shot mode is disabled."]
    DIS = 0x0,
    #[doc = "If Timer B is configured in the periodic mode."]
    EN = 0x01,
}
impl TBSNAPS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBSNAPS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBSNAPS {
    #[inline(always)]
    fn from(val: u8) -> TBSNAPS {
        TBSNAPS::from_bits(val)
    }
}
impl From<TBSNAPS> for u8 {
    #[inline(always)]
    fn from(val: TBSNAPS) -> u8 {
        TBSNAPS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBSTALL {
    #[doc = "Timer B continues counting while the processor is halted by the debugger."]
    DIS = 0x0,
    #[doc = "Timer B freezes counting while the processor is halted by the debugger."]
    EN = 0x01,
}
impl TBSTALL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBSTALL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBSTALL {
    #[inline(always)]
    fn from(val: u8) -> TBSTALL {
        TBSTALL::from_bits(val)
    }
}
impl From<TBSTALL> for u8 {
    #[inline(always)]
    fn from(val: TBSTALL) -> u8 {
        TBSTALL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBTOIM {
    #[doc = "Disable Interrupt."]
    DIS = 0x0,
    #[doc = "Enable Interrupt."]
    EN = 0x01,
}
impl TBTOIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBTOIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBTOIM {
    #[inline(always)]
    fn from(val: u8) -> TBTOIM {
        TBTOIM::from_bits(val)
    }
}
impl From<TBTOIM> for u8 {
    #[inline(always)]
    fn from(val: TBTOIM) -> u8 {
        TBTOIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TBWOT {
    #[doc = "Timer B begins counting as soon as it is enabled."]
    NOWAIT = 0x0,
    #[doc = "If Timer B is enabled (CTL.TBEN is set), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This function is valid for one-shot, periodic, and PWM modes."]
    WAIT = 0x01,
}
impl TBWOT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TBWOT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TBWOT {
    #[inline(always)]
    fn from(val: u8) -> TBWOT {
        TBWOT::from_bits(val)
    }
}
impl From<TBWOT> for u8 {
    #[inline(always)]
    fn from(val: TBWOT) -> u8 {
        TBWOT::to_bits(val)
    }
}

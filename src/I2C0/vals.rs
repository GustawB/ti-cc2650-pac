#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ACK {
    #[doc = "Disable acknowledge."]
    DIS = 0x0,
    #[doc = "Enable acknowledge."]
    EN = 0x01,
}
impl ACK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ACK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ACK {
    #[inline(always)]
    fn from(val: u8) -> ACK {
        ACK::from_bits(val)
    }
}
impl From<ACK> for u8 {
    #[inline(always)]
    fn from(val: ACK) -> u8 {
        ACK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IM {
    #[doc = "Disable Interrupt."]
    DIS = 0x0,
    #[doc = "Enable Interrupt."]
    EN = 0x01,
}
impl IM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IM {
    #[inline(always)]
    fn from(val: u8) -> IM {
        IM::from_bits(val)
    }
}
impl From<IM> for u8 {
    #[inline(always)]
    fn from(val: IM) -> u8 {
        IM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPBK {
    #[doc = "Disable Test Mode."]
    DIS = 0x0,
    #[doc = "Enable Test Mode."]
    EN = 0x01,
}
impl LPBK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPBK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPBK {
    #[inline(always)]
    fn from(val: u8) -> LPBK {
        LPBK::from_bits(val)
    }
}
impl From<LPBK> for u8 {
    #[inline(always)]
    fn from(val: LPBK) -> u8 {
        LPBK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MFE {
    #[doc = "Master mode is disabled."]
    DIS = 0x0,
    #[doc = "Master mode is enabled."]
    EN = 0x01,
}
impl MFE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MFE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MFE {
    #[inline(always)]
    fn from(val: u8) -> MFE {
        MFE::from_bits(val)
    }
}
impl From<MFE> for u8 {
    #[inline(always)]
    fn from(val: MFE) -> u8 {
        MFE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RS {
    #[doc = "Transmit/send data to slave."]
    TX = 0x0,
    #[doc = "Receive data from slave."]
    RX = 0x01,
}
impl RS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RS {
    #[inline(always)]
    fn from(val: u8) -> RS {
        RS::from_bits(val)
    }
}
impl From<RS> for u8 {
    #[inline(always)]
    fn from(val: RS) -> u8 {
        RS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RUN {
    #[doc = "Disable Master."]
    DIS = 0x0,
    #[doc = "Enable Master."]
    EN = 0x01,
}
impl RUN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RUN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RUN {
    #[inline(always)]
    fn from(val: u8) -> RUN {
        RUN::from_bits(val)
    }
}
impl From<RUN> for u8 {
    #[inline(always)]
    fn from(val: RUN) -> u8 {
        RUN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SFE {
    #[doc = "Slave mode is disabled."]
    DIS = 0x0,
    #[doc = "Slave mode is enabled."]
    EN = 0x01,
}
impl SFE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SFE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SFE {
    #[inline(always)]
    fn from(val: u8) -> SFE {
        SFE::from_bits(val)
    }
}
impl From<SFE> for u8 {
    #[inline(always)]
    fn from(val: SFE) -> u8 {
        SFE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum START {
    #[doc = "Disable START."]
    DIS = 0x0,
    #[doc = "Enable START."]
    EN = 0x01,
}
impl START {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> START {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for START {
    #[inline(always)]
    fn from(val: u8) -> START {
        START::from_bits(val)
    }
}
impl From<START> for u8 {
    #[inline(always)]
    fn from(val: START) -> u8 {
        START::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STARTIM {
    #[doc = "Disable Interrupt."]
    DIS = 0x0,
    #[doc = "Enable Interrupt."]
    EN = 0x01,
}
impl STARTIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STARTIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STARTIM {
    #[inline(always)]
    fn from(val: u8) -> STARTIM {
        STARTIM::from_bits(val)
    }
}
impl From<STARTIM> for u8 {
    #[inline(always)]
    fn from(val: STARTIM) -> u8 {
        STARTIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STOP {
    #[doc = "Disable STOP."]
    DIS = 0x0,
    #[doc = "Enable STOP."]
    EN = 0x01,
}
impl STOP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STOP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STOP {
    #[inline(always)]
    fn from(val: u8) -> STOP {
        STOP::from_bits(val)
    }
}
impl From<STOP> for u8 {
    #[inline(always)]
    fn from(val: STOP) -> u8 {
        STOP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STOPIM {
    #[doc = "Disable Interrupt."]
    DIS = 0x0,
    #[doc = "Enable Interrupt."]
    EN = 0x01,
}
impl STOPIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STOPIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STOPIM {
    #[inline(always)]
    fn from(val: u8) -> STOPIM {
        STOPIM::from_bits(val)
    }
}
impl From<STOPIM> for u8 {
    #[inline(always)]
    fn from(val: STOPIM) -> u8 {
        STOPIM::to_bits(val)
    }
}

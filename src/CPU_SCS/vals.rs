#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BUSFAULTACT {
    #[doc = "Exception is not active."]
    NOTACTIVE = 0x0,
    #[doc = "Exception is active."]
    ACTIVE = 0x01,
}
impl BUSFAULTACT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BUSFAULTACT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BUSFAULTACT {
    #[inline(always)]
    fn from(val: u8) -> BUSFAULTACT {
        BUSFAULTACT::from_bits(val)
    }
}
impl From<BUSFAULTACT> for u8 {
    #[inline(always)]
    fn from(val: BUSFAULTACT) -> u8 {
        BUSFAULTACT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BUSFAULTENA {
    #[doc = "Exception disabled."]
    DIS = 0x0,
    #[doc = "Exception enabled."]
    EN = 0x01,
}
impl BUSFAULTENA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BUSFAULTENA {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BUSFAULTENA {
    #[inline(always)]
    fn from(val: u8) -> BUSFAULTENA {
        BUSFAULTENA::from_bits(val)
    }
}
impl From<BUSFAULTENA> for u8 {
    #[inline(always)]
    fn from(val: BUSFAULTENA) -> u8 {
        BUSFAULTENA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BUSFAULTPENDED {
    #[doc = "Exception is not active."]
    NOTPENDING = 0x0,
    #[doc = "Exception is pending."]
    PENDING = 0x01,
}
impl BUSFAULTPENDED {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BUSFAULTPENDED {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BUSFAULTPENDED {
    #[inline(always)]
    fn from(val: u8) -> BUSFAULTPENDED {
        BUSFAULTPENDED::from_bits(val)
    }
}
impl From<BUSFAULTPENDED> for u8 {
    #[inline(always)]
    fn from(val: BUSFAULTPENDED) -> u8 {
        BUSFAULTPENDED::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDIANESS {
    #[doc = "Little endian."]
    LITTLE = 0x0,
    #[doc = "Big endian."]
    BIG = 0x01,
}
impl ENDIANESS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDIANESS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDIANESS {
    #[inline(always)]
    fn from(val: u8) -> ENDIANESS {
        ENDIANESS::from_bits(val)
    }
}
impl From<ENDIANESS> for u8 {
    #[inline(always)]
    fn from(val: ENDIANESS) -> u8 {
        ENDIANESS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MEMFAULTACT {
    #[doc = "Exception is not active."]
    NOTACTIVE = 0x0,
    #[doc = "Exception is active."]
    ACTIVE = 0x01,
}
impl MEMFAULTACT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MEMFAULTACT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MEMFAULTACT {
    #[inline(always)]
    fn from(val: u8) -> MEMFAULTACT {
        MEMFAULTACT::from_bits(val)
    }
}
impl From<MEMFAULTACT> for u8 {
    #[inline(always)]
    fn from(val: MEMFAULTACT) -> u8 {
        MEMFAULTACT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MEMFAULTENA {
    #[doc = "Exception disabled."]
    DIS = 0x0,
    #[doc = "Exception enabled."]
    EN = 0x01,
}
impl MEMFAULTENA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MEMFAULTENA {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MEMFAULTENA {
    #[inline(always)]
    fn from(val: u8) -> MEMFAULTENA {
        MEMFAULTENA::from_bits(val)
    }
}
impl From<MEMFAULTENA> for u8 {
    #[inline(always)]
    fn from(val: MEMFAULTENA) -> u8 {
        MEMFAULTENA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MEMFAULTPENDED {
    #[doc = "Exception is not active."]
    NOTPENDING = 0x0,
    #[doc = "Exception is pending."]
    PENDING = 0x01,
}
impl MEMFAULTPENDED {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MEMFAULTPENDED {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MEMFAULTPENDED {
    #[inline(always)]
    fn from(val: u8) -> MEMFAULTPENDED {
        MEMFAULTPENDED::from_bits(val)
    }
}
impl From<MEMFAULTPENDED> for u8 {
    #[inline(always)]
    fn from(val: MEMFAULTPENDED) -> u8 {
        MEMFAULTPENDED::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MONITORACT {
    #[doc = "Exception is not active."]
    NOTACTIVE = 0x0,
    #[doc = "Exception is active."]
    ACTIVE = 0x01,
}
impl MONITORACT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MONITORACT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MONITORACT {
    #[inline(always)]
    fn from(val: u8) -> MONITORACT {
        MONITORACT::from_bits(val)
    }
}
impl From<MONITORACT> for u8 {
    #[inline(always)]
    fn from(val: MONITORACT) -> u8 {
        MONITORACT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLEEPDEEP {
    #[doc = "Sleep."]
    SLEEP = 0x0,
    #[doc = "Deep sleep."]
    DEEPSLEEP = 0x01,
}
impl SLEEPDEEP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLEEPDEEP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLEEPDEEP {
    #[inline(always)]
    fn from(val: u8) -> SLEEPDEEP {
        SLEEPDEEP::from_bits(val)
    }
}
impl From<SLEEPDEEP> for u8 {
    #[inline(always)]
    fn from(val: SLEEPDEEP) -> u8 {
        SLEEPDEEP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SVCALLACT {
    #[doc = "Exception is not active."]
    NOTACTIVE = 0x0,
    #[doc = "Exception is active."]
    ACTIVE = 0x01,
}
impl SVCALLACT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SVCALLACT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SVCALLACT {
    #[inline(always)]
    fn from(val: u8) -> SVCALLACT {
        SVCALLACT::from_bits(val)
    }
}
impl From<SVCALLACT> for u8 {
    #[inline(always)]
    fn from(val: SVCALLACT) -> u8 {
        SVCALLACT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SVCALLPENDED {
    #[doc = "Exception is not active."]
    NOTPENDING = 0x0,
    #[doc = "Exception is pending."]
    PENDING = 0x01,
}
impl SVCALLPENDED {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SVCALLPENDED {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SVCALLPENDED {
    #[inline(always)]
    fn from(val: u8) -> SVCALLPENDED {
        SVCALLPENDED::from_bits(val)
    }
}
impl From<SVCALLPENDED> for u8 {
    #[inline(always)]
    fn from(val: SVCALLPENDED) -> u8 {
        SVCALLPENDED::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYSTICKACT {
    #[doc = "Exception is not active."]
    NOTACTIVE = 0x0,
    #[doc = "Exception is active."]
    ACTIVE = 0x01,
}
impl SYSTICKACT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYSTICKACT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYSTICKACT {
    #[inline(always)]
    fn from(val: u8) -> SYSTICKACT {
        SYSTICKACT::from_bits(val)
    }
}
impl From<SYSTICKACT> for u8 {
    #[inline(always)]
    fn from(val: SYSTICKACT) -> u8 {
        SYSTICKACT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USGFAULTACT {
    #[doc = "Exception is not active."]
    NOTACTIVE = 0x0,
    #[doc = "Exception is active."]
    ACTIVE = 0x01,
}
impl USGFAULTACT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USGFAULTACT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USGFAULTACT {
    #[inline(always)]
    fn from(val: u8) -> USGFAULTACT {
        USGFAULTACT::from_bits(val)
    }
}
impl From<USGFAULTACT> for u8 {
    #[inline(always)]
    fn from(val: USGFAULTACT) -> u8 {
        USGFAULTACT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USGFAULTENA {
    #[doc = "Exception disabled."]
    DIS = 0x0,
    #[doc = "Exception enabled."]
    EN = 0x01,
}
impl USGFAULTENA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USGFAULTENA {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USGFAULTENA {
    #[inline(always)]
    fn from(val: u8) -> USGFAULTENA {
        USGFAULTENA::from_bits(val)
    }
}
impl From<USGFAULTENA> for u8 {
    #[inline(always)]
    fn from(val: USGFAULTENA) -> u8 {
        USGFAULTENA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USGFAULTPENDED {
    #[doc = "Exception is not active."]
    NOTPENDING = 0x0,
    #[doc = "Exception is pending."]
    PENDING = 0x01,
}
impl USGFAULTPENDED {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USGFAULTPENDED {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USGFAULTPENDED {
    #[inline(always)]
    fn from(val: u8) -> USGFAULTPENDED {
        USGFAULTPENDED::from_bits(val)
    }
}
impl From<USGFAULTPENDED> for u8 {
    #[inline(always)]
    fn from(val: USGFAULTPENDED) -> u8 {
        USGFAULTPENDED::to_bits(val)
    }
}

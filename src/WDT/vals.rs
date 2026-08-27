#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTEN {
    #[doc = "Interrupt Disabled."]
    DIS = 0x0,
    #[doc = "Interrupt Enabled."]
    EN = 0x01,
}
impl INTEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTEN {
    #[inline(always)]
    fn from(val: u8) -> INTEN {
        INTEN::from_bits(val)
    }
}
impl From<INTEN> for u8 {
    #[inline(always)]
    fn from(val: INTEN) -> u8 {
        INTEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTTYPE {
    #[doc = "Maskable interrupt."]
    MASKABLE = 0x0,
    #[doc = "Non-maskable interrupt."]
    NONMASKABLE = 0x01,
}
impl INTTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTTYPE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTTYPE {
    #[inline(always)]
    fn from(val: u8) -> INTTYPE {
        INTTYPE::from_bits(val)
    }
}
impl From<INTTYPE> for u8 {
    #[inline(always)]
    fn from(val: INTTYPE) -> u8 {
        INTTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RESEN {
    #[doc = "Reset output Disabled."]
    DIS = 0x0,
    #[doc = "Reset output Enabled."]
    EN = 0x01,
}
impl RESEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RESEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RESEN {
    #[inline(always)]
    fn from(val: u8) -> RESEN {
        RESEN::from_bits(val)
    }
}
impl From<RESEN> for u8 {
    #[inline(always)]
    fn from(val: RESEN) -> u8 {
        RESEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STALL {
    #[doc = "Disable STALL."]
    DIS = 0x0,
    #[doc = "Enable STALL."]
    EN = 0x01,
}
impl STALL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STALL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STALL {
    #[inline(always)]
    fn from(val: u8) -> STALL {
        STALL::from_bits(val)
    }
}
impl From<STALL> for u8 {
    #[inline(always)]
    fn from(val: STALL) -> u8 {
        STALL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TEST_EN {
    #[doc = "Test mode Disabled."]
    DIS = 0x0,
    #[doc = "Test mode Enabled."]
    EN = 0x01,
}
impl TEST_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TEST_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TEST_EN {
    #[inline(always)]
    fn from(val: u8) -> TEST_EN {
        TEST_EN::from_bits(val)
    }
}
impl From<TEST_EN> for u8 {
    #[inline(always)]
    fn from(val: TEST_EN) -> u8 {
        TEST_EN::to_bits(val)
    }
}

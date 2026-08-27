#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IO0 {
    #[doc = "Output Mode: GPIODOUT bit 0 drives AUXIO\\[8i+0\\]."]
    OUT = 0x0,
    #[doc = "Input Mode: When GPIODIE bit 0 is 0: AUXIO\\[8i+0\\] is enabled for analog signal transfer. When GPIODIE bit 0 is 1: AUXIO\\[8i+0\\] is enabled for digital input."]
    IN = 0x01,
    #[doc = "Open-Drain Mode: When GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\] is driven low. When GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 0x02,
    #[doc = "Open-Source Mode: When GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\] is driven high."]
    OPEN_SOURCE = 0x03,
}
impl IO0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IO0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IO0 {
    #[inline(always)]
    fn from(val: u8) -> IO0 {
        IO0::from_bits(val)
    }
}
impl From<IO0> for u8 {
    #[inline(always)]
    fn from(val: IO0) -> u8 {
        IO0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IO1 {
    #[doc = "Output Mode: GPIODOUT bit 1 drives AUXIO\\[8i+1\\]."]
    OUT = 0x0,
    #[doc = "Input Mode: When GPIODIE bit 1 is 0: AUXIO\\[8i+1\\] is enabled for analog signal transfer. When GPIODIE bit 1 is 1: AUXIO\\[8i+1\\] is enabled for digital input."]
    IN = 0x01,
    #[doc = "Open-Drain Mode: When GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\] is driven low. When GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 0x02,
    #[doc = "Open-Source Mode: When GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\] is driven high."]
    OPEN_SOURCE = 0x03,
}
impl IO1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IO1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IO1 {
    #[inline(always)]
    fn from(val: u8) -> IO1 {
        IO1::from_bits(val)
    }
}
impl From<IO1> for u8 {
    #[inline(always)]
    fn from(val: IO1) -> u8 {
        IO1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IO2 {
    #[doc = "Output Mode: GPIODOUT bit 2 drives AUXIO\\[8i+2\\]."]
    OUT = 0x0,
    #[doc = "Input Mode: When GPIODIE bit 2 is 0: AUXIO\\[8i+2\\] is enabled for analog signal transfer. When GPIODIE bit 2 is 1: AUXIO\\[8i+2\\] is enabled for digital input."]
    IN = 0x01,
    #[doc = "Open-Drain Mode: When GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\] is driven low. When GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 0x02,
    #[doc = "Open-Source Mode: When GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\] is driven high."]
    OPEN_SOURCE = 0x03,
}
impl IO2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IO2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IO2 {
    #[inline(always)]
    fn from(val: u8) -> IO2 {
        IO2::from_bits(val)
    }
}
impl From<IO2> for u8 {
    #[inline(always)]
    fn from(val: IO2) -> u8 {
        IO2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IO3 {
    #[doc = "Output Mode: GPIODOUT bit 3 drives AUXIO\\[8i+3\\]."]
    OUT = 0x0,
    #[doc = "Input Mode: When GPIODIE bit 3 is 0: AUXIO\\[8i+3\\] is enabled for analog signal transfer. When GPIODIE bit 3 is 1: AUXIO\\[8i+3\\] is enabled for digital input."]
    IN = 0x01,
    #[doc = "Open-Drain Mode: When GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\] is driven low. When GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 0x02,
    #[doc = "Open-Source Mode: When GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\] is driven high."]
    OPEN_SOURCE = 0x03,
}
impl IO3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IO3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IO3 {
    #[inline(always)]
    fn from(val: u8) -> IO3 {
        IO3::from_bits(val)
    }
}
impl From<IO3> for u8 {
    #[inline(always)]
    fn from(val: IO3) -> u8 {
        IO3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IO4 {
    #[doc = "Output Mode: GPIODOUT bit 4 drives AUXIO\\[8i+4\\]."]
    OUT = 0x0,
    #[doc = "Input Mode: When GPIODIE bit 4 is 0: AUXIO\\[8i+4\\] is enabled for analog signal transfer. When GPIODIE bit 4 is 1: AUXIO\\[8i+4\\] is enabled for digital input."]
    IN = 0x01,
    #[doc = "Open-Drain Mode: When GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\] is driven low. When GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 0x02,
    #[doc = "Open-Source Mode: When GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\] is driven high."]
    OPEN_SOURCE = 0x03,
}
impl IO4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IO4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IO4 {
    #[inline(always)]
    fn from(val: u8) -> IO4 {
        IO4::from_bits(val)
    }
}
impl From<IO4> for u8 {
    #[inline(always)]
    fn from(val: IO4) -> u8 {
        IO4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IO5 {
    #[doc = "Output Mode: GPIODOUT bit 5 drives AUXIO\\[8i+5\\]."]
    OUT = 0x0,
    #[doc = "Input Mode: When GPIODIE bit 5 is 0: AUXIO\\[8i+5\\] is enabled for analog signal transfer. When GPIODIE bit 5 is 1: AUXIO\\[8i+5\\] is enabled for digital input."]
    IN = 0x01,
    #[doc = "Open-Drain Mode: When GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\] is driven low. When GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 0x02,
    #[doc = "Open-Source Mode: When GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\] is driven high."]
    OPEN_SOURCE = 0x03,
}
impl IO5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IO5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IO5 {
    #[inline(always)]
    fn from(val: u8) -> IO5 {
        IO5::from_bits(val)
    }
}
impl From<IO5> for u8 {
    #[inline(always)]
    fn from(val: IO5) -> u8 {
        IO5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IO6 {
    #[doc = "Output Mode: GPIODOUT bit 6 drives AUXIO\\[8i+6\\]."]
    OUT = 0x0,
    #[doc = "Input Mode: When GPIODIE bit 6 is 0: AUXIO\\[8i+6\\] is enabled for analog signal transfer. When GPIODIE bit 6 is 1: AUXIO\\[8i+6\\] is enabled for digital input."]
    IN = 0x01,
    #[doc = "Open-Drain Mode: When GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\] is driven low. When GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 0x02,
    #[doc = "Open-Source Mode: When GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\] is driven high."]
    OPEN_SOURCE = 0x03,
}
impl IO6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IO6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IO6 {
    #[inline(always)]
    fn from(val: u8) -> IO6 {
        IO6::from_bits(val)
    }
}
impl From<IO6> for u8 {
    #[inline(always)]
    fn from(val: IO6) -> u8 {
        IO6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IO7 {
    #[doc = "Output Mode: GPIODOUT bit 7 drives AUXIO\\[8i+7\\]."]
    OUT = 0x0,
    #[doc = "Input Mode: When GPIODIE bit 7 is 0: AUXIO\\[8i+7\\] is enabled for analog signal transfer. When GPIODIE bit 7 is 1: AUXIO\\[8i+7\\] is enabled for digital input."]
    IN = 0x01,
    #[doc = "Open-Drain Mode: When GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\] is driven low. When GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    OPEN_DRAIN = 0x02,
    #[doc = "Open-Source Mode: When GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\] is driven high."]
    OPEN_SOURCE = 0x03,
}
impl IO7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IO7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IO7 {
    #[inline(always)]
    fn from(val: u8) -> IO7 {
        IO7::from_bits(val)
    }
}
impl From<IO7> for u8 {
    #[inline(always)]
    fn from(val: IO7) -> u8 {
        IO7::to_bits(val)
    }
}

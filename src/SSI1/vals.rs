#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DSS {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "4-bit data."]
    _4_BIT = 0x03,
    #[doc = "5-bit data."]
    _5_BIT = 0x04,
    #[doc = "6-bit data."]
    _6_BIT = 0x05,
    #[doc = "7-bit data."]
    _7_BIT = 0x06,
    #[doc = "8-bit data."]
    _8_BIT = 0x07,
    #[doc = "9-bit data."]
    _9_BIT = 0x08,
    #[doc = "10-bit data."]
    _10_BIT = 0x09,
    #[doc = "11-bit data."]
    _11_BIT = 0x0a,
    #[doc = "12-bit data."]
    _12_BIT = 0x0b,
    #[doc = "13-bit data."]
    _13_BIT = 0x0c,
    #[doc = "14-bit data."]
    _14_BIT = 0x0d,
    #[doc = "15-bit data."]
    _15_BIT = 0x0e,
    #[doc = "16-bit data."]
    _16_BIT = 0x0f,
}
impl DSS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DSS {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DSS {
    #[inline(always)]
    fn from(val: u8) -> DSS {
        DSS::from_bits(val)
    }
}
impl From<DSS> for u8 {
    #[inline(always)]
    fn from(val: DSS) -> u8 {
        DSS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FRF {
    #[doc = "Motorola SPI frame format."]
    MOTOROLA_SPI = 0x0,
    #[doc = "TI synchronous serial frame format."]
    TI_SYNC_SERIAL = 0x01,
    #[doc = "National Microwire frame format."]
    NATIONAL_MICROWIRE = 0x02,
    _RESERVED_3 = 0x03,
}
impl FRF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FRF {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FRF {
    #[inline(always)]
    fn from(val: u8) -> FRF {
        FRF::from_bits(val)
    }
}
impl From<FRF> for u8 {
    #[inline(always)]
    fn from(val: FRF) -> u8 {
        FRF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MS {
    #[doc = "Device configured as master."]
    MASTER = 0x0,
    #[doc = "Device configured as slave."]
    SLAVE = 0x01,
}
impl MS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MS {
    #[inline(always)]
    fn from(val: u8) -> MS {
        MS::from_bits(val)
    }
}
impl From<MS> for u8 {
    #[inline(always)]
    fn from(val: MS) -> u8 {
        MS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SPH {
    #[doc = "Data is captured on the first clock edge transition."]
    _1ST_CLK_EDGE = 0x0,
    #[doc = "Data is captured on the second clock edge transition."]
    _2ND_CLK_EDGE = 0x01,
}
impl SPH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SPH {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SPH {
    #[inline(always)]
    fn from(val: u8) -> SPH {
        SPH::from_bits(val)
    }
}
impl From<SPH> for u8 {
    #[inline(always)]
    fn from(val: SPH) -> u8 {
        SPH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SPO {
    #[doc = "SSI produces a steady state LOW value on the CLKOUT pin when data is not being transferred."]
    LOW = 0x0,
    #[doc = "SSI produces a steady state HIGH value on the CLKOUT pin when data is not being transferred."]
    HIGH = 0x01,
}
impl SPO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SPO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SPO {
    #[inline(always)]
    fn from(val: u8) -> SPO {
        SPO::from_bits(val)
    }
}
impl From<SPO> for u8 {
    #[inline(always)]
    fn from(val: SPO) -> u8 {
        SPO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SSE {
    #[doc = "Operation disabled."]
    SSI_DISABLED = 0x0,
    #[doc = "Operation enabled."]
    SSI_ENABLED = 0x01,
}
impl SSE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SSE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SSE {
    #[inline(always)]
    fn from(val: u8) -> SSE {
        SSE::from_bits(val)
    }
}
impl From<SSE> for u8 {
    #[inline(always)]
    fn from(val: SSE) -> u8 {
        SSE::to_bits(val)
    }
}

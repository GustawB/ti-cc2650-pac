#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct COMPA_IN(u8);
impl COMPA_IN {
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const NC: Self = Self(0x0);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO7: Self = Self(0x01);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO6: Self = Self(0x02);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO5: Self = Self(0x04);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO4: Self = Self(0x08);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO3: Self = Self(0x10);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO2: Self = Self(0x20);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO1: Self = Self(0x40);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO0: Self = Self(0x80);
}
impl COMPA_IN {
    pub const fn from_bits(val: u8) -> COMPA_IN {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for COMPA_IN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NC"),
            0x01 => f.write_str("AUXIO7"),
            0x02 => f.write_str("AUXIO6"),
            0x04 => f.write_str("AUXIO5"),
            0x08 => f.write_str("AUXIO4"),
            0x10 => f.write_str("AUXIO3"),
            0x20 => f.write_str("AUXIO2"),
            0x40 => f.write_str("AUXIO1"),
            0x80 => f.write_str("AUXIO0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMPA_IN {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NC"),
            0x01 => defmt::write!(f, "AUXIO7"),
            0x02 => defmt::write!(f, "AUXIO6"),
            0x04 => defmt::write!(f, "AUXIO5"),
            0x08 => defmt::write!(f, "AUXIO4"),
            0x10 => defmt::write!(f, "AUXIO3"),
            0x20 => defmt::write!(f, "AUXIO2"),
            0x40 => defmt::write!(f, "AUXIO1"),
            0x80 => defmt::write!(f, "AUXIO0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for COMPA_IN {
    #[inline(always)]
    fn from(val: u8) -> COMPA_IN {
        COMPA_IN::from_bits(val)
    }
}
impl From<COMPA_IN> for u8 {
    #[inline(always)]
    fn from(val: COMPA_IN) -> u8 {
        COMPA_IN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum COMPB_REF {
    #[doc = "Internal. Only to be used through TI provided API."]
    NC = 0x0,
    #[doc = "Internal. Only to be used through TI provided API."]
    DCOUPL = 0x01,
    #[doc = "Internal. Only to be used through TI provided API."]
    VSS = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Internal. Only to be used through TI provided API."]
    VDDS = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl COMPB_REF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> COMPB_REF {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for COMPB_REF {
    #[inline(always)]
    fn from(val: u8) -> COMPB_REF {
        COMPB_REF::from_bits(val)
    }
}
impl From<COMPB_REF> for u8 {
    #[inline(always)]
    fn from(val: COMPB_REF) -> u8 {
        COMPB_REF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum COMPB_TRIM {
    #[doc = "No reference division."]
    DIV1 = 0x0,
    #[doc = "Divide reference by 2."]
    DIV2 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Divide reference by 3."]
    DIV3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Divide reference by 4."]
    DIV4 = 0x07,
}
impl COMPB_TRIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> COMPB_TRIM {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for COMPB_TRIM {
    #[inline(always)]
    fn from(val: u8) -> COMPB_TRIM {
        COMPB_TRIM::from_bits(val)
    }
}
impl From<COMPB_TRIM> for u8 {
    #[inline(always)]
    fn from(val: COMPB_TRIM) -> u8 {
        COMPB_TRIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MUX0_COMPA_REF {
    #[doc = "Internal. Only to be used through TI provided API."]
    NC = 0x0,
    #[doc = "Internal. Only to be used through TI provided API."]
    DCOUPL = 0x01,
    #[doc = "Internal. Only to be used through TI provided API."]
    VSS = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Internal. Only to be used through TI provided API."]
    VDDS = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Internal. Only to be used through TI provided API."]
    ADCVREFP = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl MUX0_COMPA_REF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MUX0_COMPA_REF {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MUX0_COMPA_REF {
    #[inline(always)]
    fn from(val: u8) -> MUX0_COMPA_REF {
        MUX0_COMPA_REF::from_bits(val)
    }
}
impl From<MUX0_COMPA_REF> for u8 {
    #[inline(always)]
    fn from(val: MUX0_COMPA_REF) -> u8 {
        MUX0_COMPA_REF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MUX2_ADCCOMPB_IN {
    #[doc = "Internal. Only to be used through TI provided API."]
    NC = 0x0,
    #[doc = "Internal. Only to be used through TI provided API."]
    ATEST0 = 0x01,
    #[doc = "Internal. Only to be used through TI provided API."]
    ATEST1 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Internal. Only to be used through TI provided API."]
    DCOUPL = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Internal. Only to be used through TI provided API."]
    VSS = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Internal. Only to be used through TI provided API."]
    VDDS = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl MUX2_ADCCOMPB_IN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MUX2_ADCCOMPB_IN {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MUX2_ADCCOMPB_IN {
    #[inline(always)]
    fn from(val: u8) -> MUX2_ADCCOMPB_IN {
        MUX2_ADCCOMPB_IN::from_bits(val)
    }
}
impl From<MUX2_ADCCOMPB_IN> for u8 {
    #[inline(always)]
    fn from(val: MUX2_ADCCOMPB_IN) -> u8 {
        MUX2_ADCCOMPB_IN::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MUX3_ADCCOMPB_IN(u8);
impl MUX3_ADCCOMPB_IN {
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const NC: Self = Self(0x0);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO7: Self = Self(0x01);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO6: Self = Self(0x02);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO5: Self = Self(0x04);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO4: Self = Self(0x08);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO3: Self = Self(0x10);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO2: Self = Self(0x20);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO1: Self = Self(0x40);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO0: Self = Self(0x80);
}
impl MUX3_ADCCOMPB_IN {
    pub const fn from_bits(val: u8) -> MUX3_ADCCOMPB_IN {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for MUX3_ADCCOMPB_IN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NC"),
            0x01 => f.write_str("AUXIO7"),
            0x02 => f.write_str("AUXIO6"),
            0x04 => f.write_str("AUXIO5"),
            0x08 => f.write_str("AUXIO4"),
            0x10 => f.write_str("AUXIO3"),
            0x20 => f.write_str("AUXIO2"),
            0x40 => f.write_str("AUXIO1"),
            0x80 => f.write_str("AUXIO0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MUX3_ADCCOMPB_IN {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NC"),
            0x01 => defmt::write!(f, "AUXIO7"),
            0x02 => defmt::write!(f, "AUXIO6"),
            0x04 => defmt::write!(f, "AUXIO5"),
            0x08 => defmt::write!(f, "AUXIO4"),
            0x10 => defmt::write!(f, "AUXIO3"),
            0x20 => defmt::write!(f, "AUXIO2"),
            0x40 => defmt::write!(f, "AUXIO1"),
            0x80 => defmt::write!(f, "AUXIO0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for MUX3_ADCCOMPB_IN {
    #[inline(always)]
    fn from(val: u8) -> MUX3_ADCCOMPB_IN {
        MUX3_ADCCOMPB_IN::from_bits(val)
    }
}
impl From<MUX3_ADCCOMPB_IN> for u8 {
    #[inline(always)]
    fn from(val: MUX3_ADCCOMPB_IN) -> u8 {
        MUX3_ADCCOMPB_IN::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MUX4_COMPA_REF(u8);
impl MUX4_COMPA_REF {
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const NC: Self = Self(0x0);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO7: Self = Self(0x01);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO6: Self = Self(0x02);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO5: Self = Self(0x04);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO4: Self = Self(0x08);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO3: Self = Self(0x10);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO2: Self = Self(0x20);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO1: Self = Self(0x40);
    #[doc = "Internal. Only to be used through TI provided API."]
    pub const AUXIO0: Self = Self(0x80);
}
impl MUX4_COMPA_REF {
    pub const fn from_bits(val: u8) -> MUX4_COMPA_REF {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for MUX4_COMPA_REF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NC"),
            0x01 => f.write_str("AUXIO7"),
            0x02 => f.write_str("AUXIO6"),
            0x04 => f.write_str("AUXIO5"),
            0x08 => f.write_str("AUXIO4"),
            0x10 => f.write_str("AUXIO3"),
            0x20 => f.write_str("AUXIO2"),
            0x40 => f.write_str("AUXIO1"),
            0x80 => f.write_str("AUXIO0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MUX4_COMPA_REF {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NC"),
            0x01 => defmt::write!(f, "AUXIO7"),
            0x02 => defmt::write!(f, "AUXIO6"),
            0x04 => defmt::write!(f, "AUXIO5"),
            0x08 => defmt::write!(f, "AUXIO4"),
            0x10 => defmt::write!(f, "AUXIO3"),
            0x20 => defmt::write!(f, "AUXIO2"),
            0x40 => defmt::write!(f, "AUXIO1"),
            0x80 => defmt::write!(f, "AUXIO0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for MUX4_COMPA_REF {
    #[inline(always)]
    fn from(val: u8) -> MUX4_COMPA_REF {
        MUX4_COMPA_REF::from_bits(val)
    }
}
impl From<MUX4_COMPA_REF> for u8 {
    #[inline(always)]
    fn from(val: MUX4_COMPA_REF) -> u8 {
        MUX4_COMPA_REF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SMPL_CYCLE_EXP {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "16x 6 MHz clock periods = 2.7us."]
    _2P7_US = 0x03,
    #[doc = "32x 6 MHz clock periods = 5.3us."]
    _5P3_US = 0x04,
    #[doc = "64x 6 MHz clock periods = 10.6us."]
    _10P6_US = 0x05,
    #[doc = "128x 6 MHz clock periods = 21.3us."]
    _21P3_US = 0x06,
    #[doc = "256x 6 MHz clock periods = 42.6us."]
    _42P6_US = 0x07,
    #[doc = "512x 6 MHz clock periods = 85.3us."]
    _85P3_US = 0x08,
    #[doc = "1024x 6 MHz clock periods = 170us."]
    _170_US = 0x09,
    #[doc = "2048x 6 MHz clock periods = 341us."]
    _341_US = 0x0a,
    #[doc = "4096x 6 MHz clock periods = 682us."]
    _682_US = 0x0b,
    #[doc = "8192x 6 MHz clock periods = 1.37ms."]
    _1P37_MS = 0x0c,
    #[doc = "16384x 6 MHz clock periods = 2.73ms."]
    _2P73_MS = 0x0d,
    #[doc = "32768x 6 MHz clock periods = 5.46ms."]
    _5P46_MS = 0x0e,
    #[doc = "65536x 6 MHz clock periods = 10.9ms."]
    _10P9_MS = 0x0f,
}
impl SMPL_CYCLE_EXP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SMPL_CYCLE_EXP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SMPL_CYCLE_EXP {
    #[inline(always)]
    fn from(val: u8) -> SMPL_CYCLE_EXP {
        SMPL_CYCLE_EXP::from_bits(val)
    }
}
impl From<SMPL_CYCLE_EXP> for u8 {
    #[inline(always)]
    fn from(val: SMPL_CYCLE_EXP) -> u8 {
        SMPL_CYCLE_EXP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TRIM {
    #[doc = "No current connected."]
    NC = 0x0,
    #[doc = "0.25 uA."]
    _0P25U = 0x01,
    #[doc = "0.5 uA."]
    _0P5U = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "1.0 uA."]
    _1P0U = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "2.0 uA."]
    _2P0U = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "4.5 uA."]
    _4P5U = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "11.75 uA."]
    _11P75U = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl TRIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TRIM {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TRIM {
    #[inline(always)]
    fn from(val: u8) -> TRIM {
        TRIM::from_bits(val)
    }
}
impl From<TRIM> for u8 {
    #[inline(always)]
    fn from(val: TRIM) -> u8 {
        TRIM::to_bits(val)
    }
}

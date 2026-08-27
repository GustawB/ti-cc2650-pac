#[doc = "Output Selection for AUX Subscriber 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUXSEL0(pub u32);
impl AUXSEL0 {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::AUXSEL0_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::AUXSEL0_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::AUXSEL0_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for AUXSEL0 {
    #[inline(always)]
    fn default() -> AUXSEL0 {
        AUXSEL0(0)
    }
}
impl core::fmt::Debug for AUXSEL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUXSEL0")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUXSEL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUXSEL0 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for NMI Subscriber 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CM3NMISEL0(pub u32);
impl CM3NMISEL0 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CM3NMISEL0_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CM3NMISEL0_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CM3NMISEL0_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CM3NMISEL0 {
    #[inline(always)]
    fn default() -> CM3NMISEL0 {
        CM3NMISEL0(0)
    }
}
impl core::fmt::Debug for CM3NMISEL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM3NMISEL0")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CM3NMISEL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CM3NMISEL0 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL0(pub u32);
impl CPUIRQSEL0 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL0_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL0_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL0_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL0 {
    #[inline(always)]
    fn default() -> CPUIRQSEL0 {
        CPUIRQSEL0(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL0")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL0 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL1(pub u32);
impl CPUIRQSEL1 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL1_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL1_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL1_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL1 {
    #[inline(always)]
    fn default() -> CPUIRQSEL1 {
        CPUIRQSEL1(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL1")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL1 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 10."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL10(pub u32);
impl CPUIRQSEL10 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL10_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL10_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL10_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL10 {
    #[inline(always)]
    fn default() -> CPUIRQSEL10 {
        CPUIRQSEL10(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL10")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL10 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 11."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL11(pub u32);
impl CPUIRQSEL11 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL11_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL11_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL11_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL11 {
    #[inline(always)]
    fn default() -> CPUIRQSEL11 {
        CPUIRQSEL11(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL11")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL11 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 12."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL12(pub u32);
impl CPUIRQSEL12 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL12_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL12_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL12_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL12 {
    #[inline(always)]
    fn default() -> CPUIRQSEL12 {
        CPUIRQSEL12(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL12")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL12 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 13."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL13(pub u32);
impl CPUIRQSEL13 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL13_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL13_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL13_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL13 {
    #[inline(always)]
    fn default() -> CPUIRQSEL13 {
        CPUIRQSEL13(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL13")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL13 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 14."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL14(pub u32);
impl CPUIRQSEL14 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL14_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL14_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL14_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL14 {
    #[inline(always)]
    fn default() -> CPUIRQSEL14 {
        CPUIRQSEL14(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL14")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL14 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 15."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL15(pub u32);
impl CPUIRQSEL15 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL15_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL15_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL15_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL15 {
    #[inline(always)]
    fn default() -> CPUIRQSEL15 {
        CPUIRQSEL15(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL15")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL15 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 16."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL16(pub u32);
impl CPUIRQSEL16 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL16_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL16_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL16_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL16 {
    #[inline(always)]
    fn default() -> CPUIRQSEL16 {
        CPUIRQSEL16(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL16")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL16 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 17."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL17(pub u32);
impl CPUIRQSEL17 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL17_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL17_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL17_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL17 {
    #[inline(always)]
    fn default() -> CPUIRQSEL17 {
        CPUIRQSEL17(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL17")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL17 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 18."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL18(pub u32);
impl CPUIRQSEL18 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL18_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL18_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL18_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL18 {
    #[inline(always)]
    fn default() -> CPUIRQSEL18 {
        CPUIRQSEL18(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL18")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL18 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 19."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL19(pub u32);
impl CPUIRQSEL19 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL19_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL19_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL19_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL19 {
    #[inline(always)]
    fn default() -> CPUIRQSEL19 {
        CPUIRQSEL19(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL19")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL19 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL2(pub u32);
impl CPUIRQSEL2 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL2_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL2_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL2_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL2 {
    #[inline(always)]
    fn default() -> CPUIRQSEL2 {
        CPUIRQSEL2(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL2")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL2 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 20."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL20(pub u32);
impl CPUIRQSEL20 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL20_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL20_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL20_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL20 {
    #[inline(always)]
    fn default() -> CPUIRQSEL20 {
        CPUIRQSEL20(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL20")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL20 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 21."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL21(pub u32);
impl CPUIRQSEL21 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL21_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL21_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL21_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL21 {
    #[inline(always)]
    fn default() -> CPUIRQSEL21 {
        CPUIRQSEL21(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL21")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL21 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 22."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL22(pub u32);
impl CPUIRQSEL22 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL22_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL22_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL22_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL22 {
    #[inline(always)]
    fn default() -> CPUIRQSEL22 {
        CPUIRQSEL22(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL22")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL22 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 23."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL23(pub u32);
impl CPUIRQSEL23 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL23_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL23_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL23_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL23 {
    #[inline(always)]
    fn default() -> CPUIRQSEL23 {
        CPUIRQSEL23(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL23")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL23 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 24."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL24(pub u32);
impl CPUIRQSEL24 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL24_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL24_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL24_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL24 {
    #[inline(always)]
    fn default() -> CPUIRQSEL24 {
        CPUIRQSEL24(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL24")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL24 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 25."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL25(pub u32);
impl CPUIRQSEL25 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL25_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL25_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL25_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL25 {
    #[inline(always)]
    fn default() -> CPUIRQSEL25 {
        CPUIRQSEL25(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL25")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL25 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 26."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL26(pub u32);
impl CPUIRQSEL26 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL26_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL26_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL26_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL26 {
    #[inline(always)]
    fn default() -> CPUIRQSEL26 {
        CPUIRQSEL26(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL26")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL26 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 27."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL27(pub u32);
impl CPUIRQSEL27 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL27_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL27_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL27_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL27 {
    #[inline(always)]
    fn default() -> CPUIRQSEL27 {
        CPUIRQSEL27(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL27")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL27 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 28."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL28(pub u32);
impl CPUIRQSEL28 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL28_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL28_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL28_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL28 {
    #[inline(always)]
    fn default() -> CPUIRQSEL28 {
        CPUIRQSEL28(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL28")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL28 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 29."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL29(pub u32);
impl CPUIRQSEL29 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL29_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL29_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL29_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL29 {
    #[inline(always)]
    fn default() -> CPUIRQSEL29 {
        CPUIRQSEL29(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL29")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL29 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL3(pub u32);
impl CPUIRQSEL3 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CPUIRQSEL3 {
    #[inline(always)]
    fn default() -> CPUIRQSEL3 {
        CPUIRQSEL3(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL3")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CPUIRQSEL3 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Output Selection for CPU Interrupt 30."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL30(pub u32);
impl CPUIRQSEL30 {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL30_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL30_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL30_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL30 {
    #[inline(always)]
    fn default() -> CPUIRQSEL30 {
        CPUIRQSEL30(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL30")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL30 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 31."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL31(pub u32);
impl CPUIRQSEL31 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL31_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL31_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL31_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL31 {
    #[inline(always)]
    fn default() -> CPUIRQSEL31 {
        CPUIRQSEL31(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL31")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL31 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 32."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL32(pub u32);
impl CPUIRQSEL32 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL32_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL32_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL32_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL32 {
    #[inline(always)]
    fn default() -> CPUIRQSEL32 {
        CPUIRQSEL32(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL32")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL32 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 33."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL33(pub u32);
impl CPUIRQSEL33 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL33_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL33_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL33_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL33 {
    #[inline(always)]
    fn default() -> CPUIRQSEL33 {
        CPUIRQSEL33(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL33")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL33 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL33 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL4(pub u32);
impl CPUIRQSEL4 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL4_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL4_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL4_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL4 {
    #[inline(always)]
    fn default() -> CPUIRQSEL4 {
        CPUIRQSEL4(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL4")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL4 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL5(pub u32);
impl CPUIRQSEL5 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL5_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL5_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL5_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL5 {
    #[inline(always)]
    fn default() -> CPUIRQSEL5 {
        CPUIRQSEL5(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL5")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL5 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL6(pub u32);
impl CPUIRQSEL6 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL6_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL6_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL6_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL6 {
    #[inline(always)]
    fn default() -> CPUIRQSEL6 {
        CPUIRQSEL6(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL6")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL6 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL7(pub u32);
impl CPUIRQSEL7 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL7_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL7_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL7_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL7 {
    #[inline(always)]
    fn default() -> CPUIRQSEL7 {
        CPUIRQSEL7(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL7")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL7 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 8."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL8(pub u32);
impl CPUIRQSEL8 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL8_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL8_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL8_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL8 {
    #[inline(always)]
    fn default() -> CPUIRQSEL8 {
        CPUIRQSEL8(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL8")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL8 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for CPU Interrupt 9."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUIRQSEL9(pub u32);
impl CPUIRQSEL9 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::CPUIRQSEL9_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::CPUIRQSEL9_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::CPUIRQSEL9_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for CPUIRQSEL9 {
    #[inline(always)]
    fn default() -> CPUIRQSEL9 {
        CPUIRQSEL9(0)
    }
}
impl core::fmt::Debug for CPUIRQSEL9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUIRQSEL9")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUIRQSEL9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUIRQSEL9 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for FRZ Subscriber The halted debug signal is passed to peripherals such as the General Purpose Timer, Sensor Controller with Digital and Analog Peripherals (AUX), Radio, and RTC. When the system CPU halts, the connected peripherals that have freeze enabled also halt. The programmable output can be set to static values of 0 or 1, and can also be set to pass the halted signal."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FRZSEL0(pub u32);
impl FRZSEL0 {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::FRZSEL0_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::FRZSEL0_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::FRZSEL0_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for FRZSEL0 {
    #[inline(always)]
    fn default() -> FRZSEL0 {
        FRZSEL0(0)
    }
}
impl core::fmt::Debug for FRZSEL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRZSEL0")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FRZSEL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FRZSEL0 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for GPT0 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPT0ACAPTSEL(pub u32);
impl GPT0ACAPTSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::GPT0ACAPTSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::GPT0ACAPTSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::GPT0ACAPTSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for GPT0ACAPTSEL {
    #[inline(always)]
    fn default() -> GPT0ACAPTSEL {
        GPT0ACAPTSEL(0)
    }
}
impl core::fmt::Debug for GPT0ACAPTSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPT0ACAPTSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPT0ACAPTSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPT0ACAPTSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for GPT0 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPT0BCAPTSEL(pub u32);
impl GPT0BCAPTSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::GPT0BCAPTSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::GPT0BCAPTSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::GPT0BCAPTSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for GPT0BCAPTSEL {
    #[inline(always)]
    fn default() -> GPT0BCAPTSEL {
        GPT0BCAPTSEL(0)
    }
}
impl core::fmt::Debug for GPT0BCAPTSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPT0BCAPTSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPT0BCAPTSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPT0BCAPTSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for GPT1 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPT1ACAPTSEL(pub u32);
impl GPT1ACAPTSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::GPT1ACAPTSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::GPT1ACAPTSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::GPT1ACAPTSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for GPT1ACAPTSEL {
    #[inline(always)]
    fn default() -> GPT1ACAPTSEL {
        GPT1ACAPTSEL(0)
    }
}
impl core::fmt::Debug for GPT1ACAPTSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPT1ACAPTSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPT1ACAPTSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPT1ACAPTSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for GPT1 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPT1BCAPTSEL(pub u32);
impl GPT1BCAPTSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::GPT1BCAPTSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::GPT1BCAPTSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::GPT1BCAPTSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for GPT1BCAPTSEL {
    #[inline(always)]
    fn default() -> GPT1BCAPTSEL {
        GPT1BCAPTSEL(0)
    }
}
impl core::fmt::Debug for GPT1BCAPTSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPT1BCAPTSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPT1BCAPTSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPT1BCAPTSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for GPT2 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPT2ACAPTSEL(pub u32);
impl GPT2ACAPTSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::GPT2ACAPTSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::GPT2ACAPTSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::GPT2ACAPTSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for GPT2ACAPTSEL {
    #[inline(always)]
    fn default() -> GPT2ACAPTSEL {
        GPT2ACAPTSEL(0)
    }
}
impl core::fmt::Debug for GPT2ACAPTSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPT2ACAPTSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPT2ACAPTSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPT2ACAPTSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for GPT2 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPT2BCAPTSEL(pub u32);
impl GPT2BCAPTSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::GPT2BCAPTSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::GPT2BCAPTSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::GPT2BCAPTSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for GPT2BCAPTSEL {
    #[inline(always)]
    fn default() -> GPT2BCAPTSEL {
        GPT2BCAPTSEL(0)
    }
}
impl core::fmt::Debug for GPT2BCAPTSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPT2BCAPTSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPT2BCAPTSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPT2BCAPTSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for GPT3 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPT3ACAPTSEL(pub u32);
impl GPT3ACAPTSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::GPT3ACAPTSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::GPT3ACAPTSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::GPT3ACAPTSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for GPT3ACAPTSEL {
    #[inline(always)]
    fn default() -> GPT3ACAPTSEL {
        GPT3ACAPTSEL(0)
    }
}
impl core::fmt::Debug for GPT3ACAPTSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPT3ACAPTSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPT3ACAPTSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPT3ACAPTSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for GPT3 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPT3BCAPTSEL(pub u32);
impl GPT3BCAPTSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::GPT3BCAPTSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::GPT3BCAPTSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::GPT3BCAPTSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for GPT3BCAPTSEL {
    #[inline(always)]
    fn default() -> GPT3BCAPTSEL {
        GPT3BCAPTSEL(0)
    }
}
impl core::fmt::Debug for GPT3BCAPTSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPT3BCAPTSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPT3BCAPTSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPT3BCAPTSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for I2S Subscriber 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2SSTMPSEL0(pub u32);
impl I2SSTMPSEL0 {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::I2SSTMPSEL0_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::I2SSTMPSEL0_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::I2SSTMPSEL0_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for I2SSTMPSEL0 {
    #[inline(always)]
    fn default() -> I2SSTMPSEL0 {
        I2SSTMPSEL0(0)
    }
}
impl core::fmt::Debug for I2SSTMPSEL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SSTMPSEL0")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2SSTMPSEL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2SSTMPSEL0 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for RFC Event 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCSEL0(pub u32);
impl RFCSEL0 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::RFCSEL0_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::RFCSEL0_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::RFCSEL0_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for RFCSEL0 {
    #[inline(always)]
    fn default() -> RFCSEL0 {
        RFCSEL0(0)
    }
}
impl core::fmt::Debug for RFCSEL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCSEL0")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCSEL0 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for RFC Event 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCSEL1(pub u32);
impl RFCSEL1 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::RFCSEL1_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::RFCSEL1_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::RFCSEL1_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for RFCSEL1 {
    #[inline(always)]
    fn default() -> RFCSEL1 {
        RFCSEL1(0)
    }
}
impl core::fmt::Debug for RFCSEL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCSEL1")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCSEL1 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for RFC Event 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCSEL2(pub u32);
impl RFCSEL2 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::RFCSEL2_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::RFCSEL2_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::RFCSEL2_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for RFCSEL2 {
    #[inline(always)]
    fn default() -> RFCSEL2 {
        RFCSEL2(0)
    }
}
impl core::fmt::Debug for RFCSEL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCSEL2")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCSEL2 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for RFC Event 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCSEL3(pub u32);
impl RFCSEL3 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::RFCSEL3_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::RFCSEL3_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::RFCSEL3_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for RFCSEL3 {
    #[inline(always)]
    fn default() -> RFCSEL3 {
        RFCSEL3(0)
    }
}
impl core::fmt::Debug for RFCSEL3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCSEL3")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCSEL3 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for RFC Event 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCSEL4(pub u32);
impl RFCSEL4 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::RFCSEL4_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::RFCSEL4_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::RFCSEL4_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for RFCSEL4 {
    #[inline(always)]
    fn default() -> RFCSEL4 {
        RFCSEL4(0)
    }
}
impl core::fmt::Debug for RFCSEL4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCSEL4")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCSEL4 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for RFC Event 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCSEL5(pub u32);
impl RFCSEL5 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::RFCSEL5_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::RFCSEL5_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::RFCSEL5_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for RFCSEL5 {
    #[inline(always)]
    fn default() -> RFCSEL5 {
        RFCSEL5(0)
    }
}
impl core::fmt::Debug for RFCSEL5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCSEL5")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCSEL5 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for RFC Event 6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCSEL6(pub u32);
impl RFCSEL6 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::RFCSEL6_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::RFCSEL6_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::RFCSEL6_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for RFCSEL6 {
    #[inline(always)]
    fn default() -> RFCSEL6 {
        RFCSEL6(0)
    }
}
impl core::fmt::Debug for RFCSEL6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCSEL6")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCSEL6 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for RFC Event 7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCSEL7(pub u32);
impl RFCSEL7 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::RFCSEL7_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::RFCSEL7_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::RFCSEL7_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for RFCSEL7 {
    #[inline(always)]
    fn default() -> RFCSEL7 {
        RFCSEL7(0)
    }
}
impl core::fmt::Debug for RFCSEL7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCSEL7")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCSEL7 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for RFC Event 8."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCSEL8(pub u32);
impl RFCSEL8 {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::RFCSEL8_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::RFCSEL8_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::RFCSEL8_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for RFCSEL8 {
    #[inline(always)]
    fn default() -> RFCSEL8 {
        RFCSEL8(0)
    }
}
impl core::fmt::Debug for RFCSEL8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCSEL8")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCSEL8 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for RFC Event 9."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCSEL9(pub u32);
impl RFCSEL9 {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::RFCSEL9_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::RFCSEL9_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::RFCSEL9_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for RFCSEL9 {
    #[inline(always)]
    fn default() -> RFCSEL9 {
        RFCSEL9(0)
    }
}
impl core::fmt::Debug for RFCSEL9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCSEL9")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCSEL9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCSEL9 {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Set or Clear Software Events."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SWEV(pub u32);
impl SWEV {
    #[doc = "0:0\\] Writing \"1\" to this bit when the value is \"0\" triggers the Software 0 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWEV0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Writing \"1\" to this bit when the value is \"0\" triggers the Software 0 event."]
    #[inline(always)]
    pub const fn set_SWEV0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "8:8\\] Writing \"1\" to this bit when the value is \"0\" triggers the Software 1 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWEV1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Writing \"1\" to this bit when the value is \"0\" triggers the Software 1 event."]
    #[inline(always)]
    pub const fn set_SWEV1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "16:16\\] Writing \"1\" to this bit when the value is \"0\" triggers the Software 2 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWEV2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Writing \"1\" to this bit when the value is \"0\" triggers the Software 2 event."]
    #[inline(always)]
    pub const fn set_SWEV2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "24:24\\] Writing \"1\" to this bit when the value is \"0\" triggers the Software 3 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWEV3(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Writing \"1\" to this bit when the value is \"0\" triggers the Software 3 event."]
    #[inline(always)]
    pub const fn set_SWEV3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for SWEV {
    #[inline(always)]
    fn default() -> SWEV {
        SWEV(0)
    }
}
impl core::fmt::Debug for SWEV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWEV")
            .field("SWEV0", &self.SWEV0())
            .field("RESERVED0", &self.RESERVED0())
            .field("SWEV1", &self.SWEV1())
            .field("RESERVED1", &self.RESERVED1())
            .field("SWEV2", &self.SWEV2())
            .field("RESERVED2", &self.RESERVED2())
            .field("SWEV3", &self.SWEV3())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SWEV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SWEV {{ SWEV0: {=bool:?}, RESERVED0: {=u8:?}, SWEV1: {=bool:?}, RESERVED1: {=u8:?}, SWEV2: {=bool:?}, RESERVED2: {=u8:?}, SWEV3: {=bool:?}, RESERVED3: {=u8:?} }}",
            self.SWEV0(),
            self.RESERVED0(),
            self.SWEV1(),
            self.RESERVED1(),
            self.SWEV2(),
            self.RESERVED2(),
            self.SWEV3(),
            self.RESERVED3()
        )
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH0BSEL(pub u32);
impl UDMACH0BSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH0BSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH0BSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH0BSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH0BSEL {
    #[inline(always)]
    fn default() -> UDMACH0BSEL {
        UDMACH0BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH0BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH0BSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH0BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH0BSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH0SSEL(pub u32);
impl UDMACH0SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH0SSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH0SSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH0SSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH0SSEL {
    #[inline(always)]
    fn default() -> UDMACH0SSEL {
        UDMACH0SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH0SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH0SSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH0SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH0SSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Output Selection for DMA Channel 10 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH10BSEL(pub u32);
impl UDMACH10BSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH10BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH10BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH10BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH10BSEL {
    #[inline(always)]
    fn default() -> UDMACH10BSEL {
        UDMACH10BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH10BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH10BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH10BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH10BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 10 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH10SSEL(pub u32);
impl UDMACH10SSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH10SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH10SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH10SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH10SSEL {
    #[inline(always)]
    fn default() -> UDMACH10SSEL {
        UDMACH10SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH10SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH10SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH10SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH10SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 11 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH11BSEL(pub u32);
impl UDMACH11BSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH11BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH11BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH11BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH11BSEL {
    #[inline(always)]
    fn default() -> UDMACH11BSEL {
        UDMACH11BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH11BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH11BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH11BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH11BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 11 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH11SSEL(pub u32);
impl UDMACH11SSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH11SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH11SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH11SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH11SSEL {
    #[inline(always)]
    fn default() -> UDMACH11SSEL {
        UDMACH11SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH11SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH11SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH11SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH11SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 12 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH12BSEL(pub u32);
impl UDMACH12BSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH12BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH12BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH12BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH12BSEL {
    #[inline(always)]
    fn default() -> UDMACH12BSEL {
        UDMACH12BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH12BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH12BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH12BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH12BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 12 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH12SSEL(pub u32);
impl UDMACH12SSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH12SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH12SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH12SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH12SSEL {
    #[inline(always)]
    fn default() -> UDMACH12SSEL {
        UDMACH12SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH12SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH12SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH12SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH12SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 13 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH13BSEL(pub u32);
impl UDMACH13BSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH13BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH13BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH13BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH13BSEL {
    #[inline(always)]
    fn default() -> UDMACH13BSEL {
        UDMACH13BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH13BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH13BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH13BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH13BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH13SSEL(pub u32);
impl UDMACH13SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH13SSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH13SSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH13SSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH13SSEL {
    #[inline(always)]
    fn default() -> UDMACH13SSEL {
        UDMACH13SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH13SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH13SSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH13SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH13SSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Output Selection for DMA Channel 14 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH14BSEL(pub u32);
impl UDMACH14BSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH14BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH14BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH14BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH14BSEL {
    #[inline(always)]
    fn default() -> UDMACH14BSEL {
        UDMACH14BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH14BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH14BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH14BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH14BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH14SSEL(pub u32);
impl UDMACH14SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH14SSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH14SSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH14SSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH14SSEL {
    #[inline(always)]
    fn default() -> UDMACH14SSEL {
        UDMACH14SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH14SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH14SSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH14SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH14SSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Output Selection for DMA Channel 15 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH15BSEL(pub u32);
impl UDMACH15BSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH15BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH15BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH15BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH15BSEL {
    #[inline(always)]
    fn default() -> UDMACH15BSEL {
        UDMACH15BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH15BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH15BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH15BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH15BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH15SSEL(pub u32);
impl UDMACH15SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH15SSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH15SSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH15SSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH15SSEL {
    #[inline(always)]
    fn default() -> UDMACH15SSEL {
        UDMACH15SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH15SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH15SSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH15SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH15SSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Output Selection for DMA Channel 16 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH16BSEL(pub u32);
impl UDMACH16BSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH16BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH16BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH16BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH16BSEL {
    #[inline(always)]
    fn default() -> UDMACH16BSEL {
        UDMACH16BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH16BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH16BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH16BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH16BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 16 SREQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH16SSEL(pub u32);
impl UDMACH16SSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH16SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH16SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH16SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH16SSEL {
    #[inline(always)]
    fn default() -> UDMACH16SSEL {
        UDMACH16SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH16SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH16SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH16SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH16SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 17 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH17BSEL(pub u32);
impl UDMACH17BSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH17BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH17BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH17BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH17BSEL {
    #[inline(always)]
    fn default() -> UDMACH17BSEL {
        UDMACH17BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH17BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH17BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH17BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH17BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 17 SREQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH17SSEL(pub u32);
impl UDMACH17SSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH17SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH17SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH17SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH17SSEL {
    #[inline(always)]
    fn default() -> UDMACH17SSEL {
        UDMACH17SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH17SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH17SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH17SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH17SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH18BSEL(pub u32);
impl UDMACH18BSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH18BSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH18BSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH18BSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH18BSEL {
    #[inline(always)]
    fn default() -> UDMACH18BSEL {
        UDMACH18BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH18BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH18BSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH18BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH18BSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH18SSEL(pub u32);
impl UDMACH18SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH18SSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH18SSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH18SSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH18SSEL {
    #[inline(always)]
    fn default() -> UDMACH18SSEL {
        UDMACH18SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH18SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH18SSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH18SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH18SSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH19BSEL(pub u32);
impl UDMACH19BSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH19BSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH19BSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH19BSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH19BSEL {
    #[inline(always)]
    fn default() -> UDMACH19BSEL {
        UDMACH19BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH19BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH19BSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH19BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH19BSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH19SSEL(pub u32);
impl UDMACH19SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH19SSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH19SSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH19SSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH19SSEL {
    #[inline(always)]
    fn default() -> UDMACH19SSEL {
        UDMACH19SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH19SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH19SSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH19SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH19SSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Output Selection for DMA Channel 1 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH1BSEL(pub u32);
impl UDMACH1BSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH1BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH1BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH1BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH1BSEL {
    #[inline(always)]
    fn default() -> UDMACH1BSEL {
        UDMACH1BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH1BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH1BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH1BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH1BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 1 SREQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH1SSEL(pub u32);
impl UDMACH1SSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH1SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH1SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH1SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH1SSEL {
    #[inline(always)]
    fn default() -> UDMACH1SSEL {
        UDMACH1SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH1SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH1SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH1SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH1SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH20BSEL(pub u32);
impl UDMACH20BSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH20BSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH20BSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH20BSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH20BSEL {
    #[inline(always)]
    fn default() -> UDMACH20BSEL {
        UDMACH20BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH20BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH20BSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH20BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH20BSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH20SSEL(pub u32);
impl UDMACH20SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH20SSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH20SSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH20SSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH20SSEL {
    #[inline(always)]
    fn default() -> UDMACH20SSEL {
        UDMACH20SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH20SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH20SSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH20SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH20SSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Output Selection for DMA Channel 21 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH21BSEL(pub u32);
impl UDMACH21BSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH21BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH21BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH21BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH21BSEL {
    #[inline(always)]
    fn default() -> UDMACH21BSEL {
        UDMACH21BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH21BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH21BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH21BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH21BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 21 SREQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH21SSEL(pub u32);
impl UDMACH21SSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH21SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH21SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH21SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH21SSEL {
    #[inline(always)]
    fn default() -> UDMACH21SSEL {
        UDMACH21SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH21SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH21SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH21SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH21SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 22 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH22BSEL(pub u32);
impl UDMACH22BSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH22BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH22BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH22BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH22BSEL {
    #[inline(always)]
    fn default() -> UDMACH22BSEL {
        UDMACH22BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH22BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH22BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH22BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH22BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 22 SREQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH22SSEL(pub u32);
impl UDMACH22SSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH22SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH22SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH22SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH22SSEL {
    #[inline(always)]
    fn default() -> UDMACH22SSEL {
        UDMACH22SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH22SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH22SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH22SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH22SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 23 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH23BSEL(pub u32);
impl UDMACH23BSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH23BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH23BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH23BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH23BSEL {
    #[inline(always)]
    fn default() -> UDMACH23BSEL {
        UDMACH23BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH23BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH23BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH23BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH23BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 23 SREQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH23SSEL(pub u32);
impl UDMACH23SSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH23SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH23SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH23SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH23SSEL {
    #[inline(always)]
    fn default() -> UDMACH23SSEL {
        UDMACH23SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH23SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH23SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH23SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH23SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 24 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH24BSEL(pub u32);
impl UDMACH24BSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH24BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH24BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH24BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH24BSEL {
    #[inline(always)]
    fn default() -> UDMACH24BSEL {
        UDMACH24BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH24BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH24BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH24BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH24BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 24 SREQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH24SSEL(pub u32);
impl UDMACH24SSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH24SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH24SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH24SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH24SSEL {
    #[inline(always)]
    fn default() -> UDMACH24SSEL {
        UDMACH24SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH24SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH24SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH24SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH24SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH25BSEL(pub u32);
impl UDMACH25BSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH25BSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH25BSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH25BSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH25BSEL {
    #[inline(always)]
    fn default() -> UDMACH25BSEL {
        UDMACH25BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH25BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH25BSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH25BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH25BSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH25SSEL(pub u32);
impl UDMACH25SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH25SSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH25SSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH25SSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH25SSEL {
    #[inline(always)]
    fn default() -> UDMACH25SSEL {
        UDMACH25SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH25SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH25SSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH25SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH25SSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH26BSEL(pub u32);
impl UDMACH26BSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH26BSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH26BSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH26BSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH26BSEL {
    #[inline(always)]
    fn default() -> UDMACH26BSEL {
        UDMACH26BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH26BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH26BSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH26BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH26BSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH26SSEL(pub u32);
impl UDMACH26SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH26SSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH26SSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH26SSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH26SSEL {
    #[inline(always)]
    fn default() -> UDMACH26SSEL {
        UDMACH26SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH26SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH26SSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH26SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH26SSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH27BSEL(pub u32);
impl UDMACH27BSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH27BSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH27BSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH27BSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH27BSEL {
    #[inline(always)]
    fn default() -> UDMACH27BSEL {
        UDMACH27BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH27BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH27BSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH27BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH27BSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH27SSEL(pub u32);
impl UDMACH27SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH27SSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH27SSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH27SSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH27SSEL {
    #[inline(always)]
    fn default() -> UDMACH27SSEL {
        UDMACH27SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH27SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH27SSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH27SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH27SSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH28BSEL(pub u32);
impl UDMACH28BSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH28BSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH28BSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH28BSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH28BSEL {
    #[inline(always)]
    fn default() -> UDMACH28BSEL {
        UDMACH28BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH28BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH28BSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH28BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH28BSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH28SSEL(pub u32);
impl UDMACH28SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH28SSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH28SSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH28SSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH28SSEL {
    #[inline(always)]
    fn default() -> UDMACH28SSEL {
        UDMACH28SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH28SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH28SSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH28SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH28SSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH29BSEL(pub u32);
impl UDMACH29BSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH29BSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH29BSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH29BSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH29BSEL {
    #[inline(always)]
    fn default() -> UDMACH29BSEL {
        UDMACH29BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH29BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH29BSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH29BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH29BSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH29SSEL(pub u32);
impl UDMACH29SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH29SSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH29SSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH29SSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH29SSEL {
    #[inline(always)]
    fn default() -> UDMACH29SSEL {
        UDMACH29SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH29SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH29SSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH29SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH29SSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Output Selection for DMA Channel 2 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH2BSEL(pub u32);
impl UDMACH2BSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH2BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH2BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH2BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH2BSEL {
    #[inline(always)]
    fn default() -> UDMACH2BSEL {
        UDMACH2BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH2BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH2BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH2BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH2BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 2 SREQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH2SSEL(pub u32);
impl UDMACH2SSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH2SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH2SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH2SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH2SSEL {
    #[inline(always)]
    fn default() -> UDMACH2SSEL {
        UDMACH2SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH2SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH2SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH2SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH2SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH30BSEL(pub u32);
impl UDMACH30BSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH30BSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH30BSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH30BSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH30BSEL {
    #[inline(always)]
    fn default() -> UDMACH30BSEL {
        UDMACH30BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH30BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH30BSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH30BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH30BSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH30SSEL(pub u32);
impl UDMACH30SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH30SSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH30SSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH30SSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH30SSEL {
    #[inline(always)]
    fn default() -> UDMACH30SSEL {
        UDMACH30SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH30SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH30SSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH30SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH30SSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH31BSEL(pub u32);
impl UDMACH31BSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH31BSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH31BSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH31BSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH31BSEL {
    #[inline(always)]
    fn default() -> UDMACH31BSEL {
        UDMACH31BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH31BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH31BSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH31BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH31BSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH31SSEL(pub u32);
impl UDMACH31SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH31SSEL_EV {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::UDMACH31SSEL_EV::from_bits(val as u32)
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH31SSEL_EV) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH31SSEL {
    #[inline(always)]
    fn default() -> UDMACH31SSEL {
        UDMACH31SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH31SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH31SSEL")
            .field("EV", &self.EV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH31SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH31SSEL {{ EV: {:?} }}", self.EV())
    }
}
#[doc = "Output Selection for DMA Channel 3 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH3BSEL(pub u32);
impl UDMACH3BSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH3BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH3BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH3BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH3BSEL {
    #[inline(always)]
    fn default() -> UDMACH3BSEL {
        UDMACH3BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH3BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH3BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH3BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH3BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 3 SREQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH3SSEL(pub u32);
impl UDMACH3SSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH3SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH3SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH3SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH3SSEL {
    #[inline(always)]
    fn default() -> UDMACH3SSEL {
        UDMACH3SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH3SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH3SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH3SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH3SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 4 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH4BSEL(pub u32);
impl UDMACH4BSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH4BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH4BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH4BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH4BSEL {
    #[inline(always)]
    fn default() -> UDMACH4BSEL {
        UDMACH4BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH4BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH4BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH4BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH4BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 4 SREQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH4SSEL(pub u32);
impl UDMACH4SSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH4SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH4SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH4SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH4SSEL {
    #[inline(always)]
    fn default() -> UDMACH4SSEL {
        UDMACH4SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH4SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH4SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH4SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH4SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 5 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH5BSEL(pub u32);
impl UDMACH5BSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH5BSEL {
    #[inline(always)]
    fn default() -> UDMACH5BSEL {
        UDMACH5BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH5BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH5BSEL")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH5BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH5BSEL {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Output Selection for DMA Channel 5 SREQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH5SSEL(pub u32);
impl UDMACH5SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH5SSEL {
    #[inline(always)]
    fn default() -> UDMACH5SSEL {
        UDMACH5SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH5SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH5SSEL")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH5SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH5SSEL {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Output Selection for DMA Channel 6 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH6BSEL(pub u32);
impl UDMACH6BSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH6BSEL {
    #[inline(always)]
    fn default() -> UDMACH6BSEL {
        UDMACH6BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH6BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH6BSEL")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH6BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH6BSEL {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Output Selection for DMA Channel 6 SREQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH6SSEL(pub u32);
impl UDMACH6SSEL {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UDMACH6SSEL {
    #[inline(always)]
    fn default() -> UDMACH6SSEL {
        UDMACH6SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH6SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH6SSEL")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH6SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UDMACH6SSEL {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Output Selection for DMA Channel 7 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH7BSEL(pub u32);
impl UDMACH7BSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH7BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH7BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH7BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH7BSEL {
    #[inline(always)]
    fn default() -> UDMACH7BSEL {
        UDMACH7BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH7BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH7BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH7BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH7BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 7 SREQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH7SSEL(pub u32);
impl UDMACH7SSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH7SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH7SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH7SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH7SSEL {
    #[inline(always)]
    fn default() -> UDMACH7SSEL {
        UDMACH7SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH7SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH7SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH7SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH7SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 8 REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH8BSEL(pub u32);
impl UDMACH8BSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH8BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH8BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH8BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH8BSEL {
    #[inline(always)]
    fn default() -> UDMACH8BSEL {
        UDMACH8BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH8BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH8BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH8BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH8BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 8 SREQ Single request is ignored for this channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH8SSEL(pub u32);
impl UDMACH8SSEL {
    #[doc = "6:0\\] Read only selection value."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH8SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH8SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read only selection value."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH8SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH8SSEL {
    #[inline(always)]
    fn default() -> UDMACH8SSEL {
        UDMACH8SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH8SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH8SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH8SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH8SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 9 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH9BSEL(pub u32);
impl UDMACH9BSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH9BSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH9BSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH9BSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH9BSEL {
    #[inline(always)]
    fn default() -> UDMACH9BSEL {
        UDMACH9BSEL(0)
    }
}
impl core::fmt::Debug for UDMACH9BSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH9BSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH9BSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH9BSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}
#[doc = "Output Selection for DMA Channel 9 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMACH9SSEL(pub u32);
impl UDMACH9SSEL {
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> super::vals::UDMACH9SSEL_EV {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::UDMACH9SSEL_EV::from_bits(val as u8)
    }
    #[doc = "6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: super::vals::UDMACH9SSEL_EV) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for UDMACH9SSEL {
    #[inline(always)]
    fn default() -> UDMACH9SSEL {
        UDMACH9SSEL(0)
    }
}
impl core::fmt::Debug for UDMACH9SSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDMACH9SSEL")
            .field("EV", &self.EV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UDMACH9SSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UDMACH9SSEL {{ EV: {:?}, RESERVED: {=u32:?} }}",
            self.EV(),
            self.RESERVED()
        )
    }
}

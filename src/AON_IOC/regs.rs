#[doc = "SCLK_LF External Output Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLK32KCTL(pub u32);
impl CLK32KCTL {
    #[doc = "0:0\\] 0: Output enable active. SCLK_LF output on IO pin that has PORT_ID (e.g. IOC:IOCFG0.PORT_ID) set to AON_CLK32K. 1: Output enable not active."]
    #[must_use]
    #[inline(always)]
    pub const fn OE_N(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Output enable active. SCLK_LF output on IO pin that has PORT_ID (e.g. IOC:IOCFG0.PORT_ID) set to AON_CLK32K. 1: Output enable not active."]
    #[inline(always)]
    pub const fn set_OE_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for CLK32KCTL {
    #[inline(always)]
    fn default() -> CLK32KCTL {
        CLK32KCTL(0)
    }
}
impl core::fmt::Debug for CLK32KCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK32KCTL")
            .field("OE_N", &self.OE_N())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLK32KCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CLK32KCTL {{ OE_N: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.OE_N(),
            self.RESERVED1()
        )
    }
}
#[doc = "IO Latch Control Controls transparency of all latches holding I/O or configuration state from the MCU IOC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCLATCH(pub u32);
impl IOCLATCH {
    #[doc = "0:0\\] Controls latches between MCU IOC and AON_IOC. The latches are transparent by default. They must be closed prior to power off the domain(s) controlling the IOs in order to preserve IO values on external pins."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> super::vals::EN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EN::from_bits(val as u8)
    }
    #[doc = "0:0\\] Controls latches between MCU IOC and AON_IOC. The latches are transparent by default. They must be closed prior to power off the domain(s) controlling the IOs in order to preserve IO values on external pins."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: super::vals::EN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for IOCLATCH {
    #[inline(always)]
    fn default() -> IOCLATCH {
        IOCLATCH(0)
    }
}
impl core::fmt::Debug for IOCLATCH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCLATCH")
            .field("EN", &self.EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCLATCH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCLATCH {{ EN: {:?}, RESERVED1: {=u32:?} }}",
            self.EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOSTRMAX(pub u32);
impl IOSTRMAX {
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn GRAY_CODE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_GRAY_CODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "31:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "31:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for IOSTRMAX {
    #[inline(always)]
    fn default() -> IOSTRMAX {
        IOSTRMAX(0)
    }
}
impl core::fmt::Debug for IOSTRMAX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOSTRMAX")
            .field("GRAY_CODE", &self.GRAY_CODE())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOSTRMAX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOSTRMAX {{ GRAY_CODE: {=u8:?}, RESERVED3: {=u32:?} }}",
            self.GRAY_CODE(),
            self.RESERVED3()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOSTRMED(pub u32);
impl IOSTRMED {
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn GRAY_CODE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_GRAY_CODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "31:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "31:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for IOSTRMED {
    #[inline(always)]
    fn default() -> IOSTRMED {
        IOSTRMED(0)
    }
}
impl core::fmt::Debug for IOSTRMED {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOSTRMED")
            .field("GRAY_CODE", &self.GRAY_CODE())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOSTRMED {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOSTRMED {{ GRAY_CODE: {=u8:?}, RESERVED3: {=u32:?} }}",
            self.GRAY_CODE(),
            self.RESERVED3()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOSTRMIN(pub u32);
impl IOSTRMIN {
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn GRAY_CODE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_GRAY_CODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "31:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "31:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for IOSTRMIN {
    #[inline(always)]
    fn default() -> IOSTRMIN {
        IOSTRMIN(0)
    }
}
impl core::fmt::Debug for IOSTRMIN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOSTRMIN")
            .field("GRAY_CODE", &self.GRAY_CODE())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOSTRMIN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOSTRMIN {{ GRAY_CODE: {=u8:?}, RESERVED3: {=u32:?} }}",
            self.GRAY_CODE(),
            self.RESERVED3()
        )
    }
}

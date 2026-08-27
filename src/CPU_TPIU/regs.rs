#[doc = "Async Clock Prescaler This register scales the baud rate of the asynchronous output."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ACPR(pub u32);
impl ACPR {
    #[doc = "12:0\\] Divisor for input trace clock is (PRESCALER + 1)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRESCALER(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "12:0\\] Divisor for input trace clock is (PRESCALER + 1)."]
    #[inline(always)]
    pub const fn set_PRESCALER(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "31:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED13(&self) -> u32 {
        let val = (self.0 >> 13usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "31:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED13(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 13usize)) | (((val as u32) & 0x0007_ffff) << 13usize);
    }
}
impl Default for ACPR {
    #[inline(always)]
    fn default() -> ACPR {
        ACPR(0)
    }
}
impl core::fmt::Debug for ACPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACPR")
            .field("PRESCALER", &self.PRESCALER())
            .field("RESERVED13", &self.RESERVED13())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ACPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ACPR {{ PRESCALER: {=u16:?}, RESERVED13: {=u32:?} }}",
            self.PRESCALER(),
            self.RESERVED13()
        )
    }
}
#[doc = "Claim Tag Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLAIMCLR(pub u32);
impl CLAIMCLR {
    #[doc = "31:0\\] This register forms one half of the Claim Tag value. Writing to this location enables individual bits to be cleared (each bit is considered separately): 0: No effect 1: Clear this bit in the claim tag. The behavior when reading from this location is described in CLAIMTAG."]
    #[must_use]
    #[inline(always)]
    pub const fn CLAIMCLR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] This register forms one half of the Claim Tag value. Writing to this location enables individual bits to be cleared (each bit is considered separately): 0: No effect 1: Clear this bit in the claim tag. The behavior when reading from this location is described in CLAIMTAG."]
    #[inline(always)]
    pub const fn set_CLAIMCLR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CLAIMCLR {
    #[inline(always)]
    fn default() -> CLAIMCLR {
        CLAIMCLR(0)
    }
}
impl core::fmt::Debug for CLAIMCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLAIMCLR")
            .field("CLAIMCLR", &self.CLAIMCLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLAIMCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLAIMCLR {{ CLAIMCLR: {=u32:?} }}", self.CLAIMCLR())
    }
}
#[doc = "Claim Tag Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLAIMMASK(pub u32);
impl CLAIMMASK {
    #[doc = "31:0\\] This register forms one half of the Claim Tag value. When reading this register returns the number of bits that can be set (each bit is considered separately): 0: This claim tag bit is not implemented 1: This claim tag bit is not implemented The behavior when writing to this register is described in CLAIMSET."]
    #[must_use]
    #[inline(always)]
    pub const fn CLAIMMASK(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] This register forms one half of the Claim Tag value. When reading this register returns the number of bits that can be set (each bit is considered separately): 0: This claim tag bit is not implemented 1: This claim tag bit is not implemented The behavior when writing to this register is described in CLAIMSET."]
    #[inline(always)]
    pub const fn set_CLAIMMASK(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CLAIMMASK {
    #[inline(always)]
    fn default() -> CLAIMMASK {
        CLAIMMASK(0)
    }
}
impl core::fmt::Debug for CLAIMMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLAIMMASK")
            .field("CLAIMMASK", &self.CLAIMMASK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLAIMMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLAIMMASK {{ CLAIMMASK: {=u32:?} }}", self.CLAIMMASK())
    }
}
#[doc = "Claim Tag Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLAIMSET(pub u32);
impl CLAIMSET {
    #[doc = "31:0\\] This register forms one half of the Claim Tag value. Writing to this location allows individual bits to be set (each bit is considered separately): 0: No effect 1: Set this bit in the claim tag The behavior when reading from this location is described in CLAIMMASK."]
    #[must_use]
    #[inline(always)]
    pub const fn CLAIMSET(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] This register forms one half of the Claim Tag value. Writing to this location allows individual bits to be set (each bit is considered separately): 0: No effect 1: Set this bit in the claim tag The behavior when reading from this location is described in CLAIMMASK."]
    #[inline(always)]
    pub const fn set_CLAIMSET(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CLAIMSET {
    #[inline(always)]
    fn default() -> CLAIMSET {
        CLAIMSET(0)
    }
}
impl core::fmt::Debug for CLAIMSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLAIMSET")
            .field("CLAIMSET", &self.CLAIMSET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLAIMSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLAIMSET {{ CLAIMSET: {=u32:?} }}", self.CLAIMSET())
    }
}
#[doc = "Current Claim Tag."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLAIMTAG(pub u32);
impl CLAIMTAG {
    #[doc = "31:0\\] This register forms one half of the Claim Tag value. Reading this register returns the current Claim Tag value. Reading CLAIMMASK determines how many bits from this register must be used. The behavior when writing to this register is described in CLAIMCLR."]
    #[must_use]
    #[inline(always)]
    pub const fn CLAIMTAG(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] This register forms one half of the Claim Tag value. Reading this register returns the current Claim Tag value. Reading CLAIMMASK determines how many bits from this register must be used. The behavior when writing to this register is described in CLAIMCLR."]
    #[inline(always)]
    pub const fn set_CLAIMTAG(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CLAIMTAG {
    #[inline(always)]
    fn default() -> CLAIMTAG {
        CLAIMTAG(0)
    }
}
impl core::fmt::Debug for CLAIMTAG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLAIMTAG")
            .field("CLAIMTAG", &self.CLAIMTAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLAIMTAG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLAIMTAG {{ CLAIMTAG: {=u32:?} }}", self.CLAIMTAG())
    }
}
#[doc = "Current Sync Port Size This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CSPSR(pub u32);
impl CSPSR {
    #[doc = "0:0\\] 1-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn ONE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 1-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub const fn set_ONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 2-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn TWO(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 2-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub const fn set_TWO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] 3-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn THREE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] 3-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub const fn set_THREE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] 4-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn FOUR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] 4-bit port enable Writing values with more than one bit set in CSPSR, or setting a bit that is not indicated as supported in SSPSR can cause Unpredictable behavior."]
    #[inline(always)]
    pub const fn set_FOUR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for CSPSR {
    #[inline(always)]
    fn default() -> CSPSR {
        CSPSR(0)
    }
}
impl core::fmt::Debug for CSPSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSPSR")
            .field("ONE", &self.ONE())
            .field("TWO", &self.TWO())
            .field("THREE", &self.THREE())
            .field("FOUR", &self.FOUR())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CSPSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CSPSR {{ ONE: {=bool:?}, TWO: {=bool:?}, THREE: {=bool:?}, FOUR: {=bool:?}, RESERVED4: {=u32:?} }}",
            self.ONE(),
            self.TWO(),
            self.THREE(),
            self.FOUR(),
            self.RESERVED4()
        )
    }
}
#[doc = "Device ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEVID(pub u32);
impl DEVID {
    #[doc = "31:0\\] This field returns: 0xCA1 if there is an ETM present. 0xCA0 if there is no ETM present."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] This field returns: 0xCA1 if there is an ETM present. 0xCA0 if there is no ETM present."]
    #[inline(always)]
    pub const fn set_DEVID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DEVID {
    #[inline(always)]
    fn default() -> DEVID {
        DEVID(0)
    }
}
impl core::fmt::Debug for DEVID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVID")
            .field("DEVID", &self.DEVID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DEVID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DEVID {{ DEVID: {=u32:?} }}", self.DEVID())
    }
}
#[doc = "Formatter and Flush Control When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption. Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FFCR(pub u32);
impl FFCR {
    #[doc = "0:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Enable continuous formatting: 0: Continuous formatting disabled 1: Continuous formatting enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ENFCONT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Enable continuous formatting: 0: Continuous formatting disabled 1: Continuous formatting enabled."]
    #[inline(always)]
    pub const fn set_ENFCONT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "7:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "7:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "8:8\\] Indicates that triggers are inserted when a trigger pin is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGIN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Indicates that triggers are inserted when a trigger pin is asserted."]
    #[inline(always)]
    pub const fn set_TRIGIN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "31:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "31:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for FFCR {
    #[inline(always)]
    fn default() -> FFCR {
        FFCR(0)
    }
}
impl core::fmt::Debug for FFCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FFCR")
            .field("RESERVED0", &self.RESERVED0())
            .field("ENFCONT", &self.ENFCONT())
            .field("RESERVED2", &self.RESERVED2())
            .field("TRIGIN", &self.TRIGIN())
            .field("RESERVED9", &self.RESERVED9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FFCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FFCR {{ RESERVED0: {=bool:?}, ENFCONT: {=bool:?}, RESERVED2: {=u8:?}, TRIGIN: {=bool:?}, RESERVED9: {=u32:?} }}",
            self.RESERVED0(),
            self.ENFCONT(),
            self.RESERVED2(),
            self.TRIGIN(),
            self.RESERVED9()
        )
    }
}
#[doc = "Formatter and Flush Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FFSR(pub u32);
impl FFSR {
    #[doc = "2:0\\] This field always reads as zero."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] This field always reads as zero."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "3:3\\] 0: Formatter can be stopped 1: Formatter cannot be stopped."]
    #[must_use]
    #[inline(always)]
    pub const fn FTNONSTOP(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] 0: Formatter can be stopped 1: Formatter cannot be stopped."]
    #[inline(always)]
    pub const fn set_FTNONSTOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for FFSR {
    #[inline(always)]
    fn default() -> FFSR {
        FFSR(0)
    }
}
impl core::fmt::Debug for FFSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FFSR")
            .field("RESERVED0", &self.RESERVED0())
            .field("FTNONSTOP", &self.FTNONSTOP())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FFSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FFSR {{ RESERVED0: {=u8:?}, FTNONSTOP: {=bool:?}, RESERVED4: {=u32:?} }}",
            self.RESERVED0(),
            self.FTNONSTOP(),
            self.RESERVED4()
        )
    }
}
#[doc = "Formatter Synchronization Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSCR(pub u32);
impl FSCR {
    #[doc = "31:0\\] The global synchronization trigger is generated by the Program Counter (PC) Sampler block. This means that there is no synchronization counter in the TPIU."]
    #[must_use]
    #[inline(always)]
    pub const fn FSCR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] The global synchronization trigger is generated by the Program Counter (PC) Sampler block. This means that there is no synchronization counter in the TPIU."]
    #[inline(always)]
    pub const fn set_FSCR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FSCR {
    #[inline(always)]
    fn default() -> FSCR {
        FSCR(0)
    }
}
impl core::fmt::Debug for FSCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSCR").field("FSCR", &self.FSCR()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FSCR {{ FSCR: {=u32:?} }}", self.FSCR())
    }
}
#[doc = "Selected Pin Protocol This register selects the protocol to be used for trace output. Note: If this register is changed while trace data is being output, data corruption occurs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPPR(pub u32);
impl SPPR {
    #[doc = "1:0\\] Trace output protocol."]
    #[must_use]
    #[inline(always)]
    pub const fn PROTOCOL(&self) -> super::vals::PROTOCOL {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::PROTOCOL::from_bits(val as u8)
    }
    #[doc = "1:0\\] Trace output protocol."]
    #[inline(always)]
    pub const fn set_PROTOCOL(&mut self, val: super::vals::PROTOCOL) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for SPPR {
    #[inline(always)]
    fn default() -> SPPR {
        SPPR(0)
    }
}
impl core::fmt::Debug for SPPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPPR")
            .field("PROTOCOL", &self.PROTOCOL())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SPPR {{ PROTOCOL: {:?}, RESERVED2: {=u32:?} }}",
            self.PROTOCOL(),
            self.RESERVED2()
        )
    }
}
#[doc = "Supported Sync Port Sizes This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSPSR(pub u32);
impl SSPSR {
    #[doc = "0:0\\] 1-bit port size support 0x0: Not supported 0x1: Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn ONE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 1-bit port size support 0x0: Not supported 0x1: Supported."]
    #[inline(always)]
    pub const fn set_ONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 2-bit port size support 0x0: Not supported 0x1: Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn TWO(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 2-bit port size support 0x0: Not supported 0x1: Supported."]
    #[inline(always)]
    pub const fn set_TWO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] 3-bit port size support 0x0: Not supported 0x1: Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn THREE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] 3-bit port size support 0x0: Not supported 0x1: Supported."]
    #[inline(always)]
    pub const fn set_THREE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] 4-bit port size support 0x0: Not supported 0x1: Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn FOUR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] 4-bit port size support 0x0: Not supported 0x1: Supported."]
    #[inline(always)]
    pub const fn set_FOUR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for SSPSR {
    #[inline(always)]
    fn default() -> SSPSR {
        SSPSR(0)
    }
}
impl core::fmt::Debug for SSPSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSPSR")
            .field("ONE", &self.ONE())
            .field("TWO", &self.TWO())
            .field("THREE", &self.THREE())
            .field("FOUR", &self.FOUR())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSPSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SSPSR {{ ONE: {=bool:?}, TWO: {=bool:?}, THREE: {=bool:?}, FOUR: {=bool:?}, RESERVED4: {=u32:?} }}",
            self.ONE(),
            self.TWO(),
            self.THREE(),
            self.FOUR(),
            self.RESERVED4()
        )
    }
}

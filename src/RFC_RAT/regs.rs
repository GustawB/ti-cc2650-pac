#[doc = "Timer Channel 0 Capture/Compare Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RATCH0VAL(pub u32);
impl RATCH0VAL {
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[must_use]
    #[inline(always)]
    pub const fn VAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[inline(always)]
    pub const fn set_VAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RATCH0VAL {
    #[inline(always)]
    fn default() -> RATCH0VAL {
        RATCH0VAL(0)
    }
}
impl core::fmt::Debug for RATCH0VAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RATCH0VAL")
            .field("VAL", &self.VAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RATCH0VAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RATCH0VAL {{ VAL: {=u32:?} }}", self.VAL())
    }
}
#[doc = "Timer Channel 1 Capture/Compare Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RATCH1VAL(pub u32);
impl RATCH1VAL {
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[must_use]
    #[inline(always)]
    pub const fn VAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[inline(always)]
    pub const fn set_VAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RATCH1VAL {
    #[inline(always)]
    fn default() -> RATCH1VAL {
        RATCH1VAL(0)
    }
}
impl core::fmt::Debug for RATCH1VAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RATCH1VAL")
            .field("VAL", &self.VAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RATCH1VAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RATCH1VAL {{ VAL: {=u32:?} }}", self.VAL())
    }
}
#[doc = "Timer Channel 2 Capture/Compare Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RATCH2VAL(pub u32);
impl RATCH2VAL {
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[must_use]
    #[inline(always)]
    pub const fn VAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[inline(always)]
    pub const fn set_VAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RATCH2VAL {
    #[inline(always)]
    fn default() -> RATCH2VAL {
        RATCH2VAL(0)
    }
}
impl core::fmt::Debug for RATCH2VAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RATCH2VAL")
            .field("VAL", &self.VAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RATCH2VAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RATCH2VAL {{ VAL: {=u32:?} }}", self.VAL())
    }
}
#[doc = "Timer Channel 3 Capture/Compare Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RATCH3VAL(pub u32);
impl RATCH3VAL {
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[must_use]
    #[inline(always)]
    pub const fn VAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[inline(always)]
    pub const fn set_VAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RATCH3VAL {
    #[inline(always)]
    fn default() -> RATCH3VAL {
        RATCH3VAL(0)
    }
}
impl core::fmt::Debug for RATCH3VAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RATCH3VAL")
            .field("VAL", &self.VAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RATCH3VAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RATCH3VAL {{ VAL: {=u32:?} }}", self.VAL())
    }
}
#[doc = "Timer Channel 4 Capture/Compare Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RATCH4VAL(pub u32);
impl RATCH4VAL {
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[must_use]
    #[inline(always)]
    pub const fn VAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[inline(always)]
    pub const fn set_VAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RATCH4VAL {
    #[inline(always)]
    fn default() -> RATCH4VAL {
        RATCH4VAL(0)
    }
}
impl core::fmt::Debug for RATCH4VAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RATCH4VAL")
            .field("VAL", &self.VAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RATCH4VAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RATCH4VAL {{ VAL: {=u32:?} }}", self.VAL())
    }
}
#[doc = "Timer Channel 5 Capture/Compare Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RATCH5VAL(pub u32);
impl RATCH5VAL {
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[must_use]
    #[inline(always)]
    pub const fn VAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[inline(always)]
    pub const fn set_VAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RATCH5VAL {
    #[inline(always)]
    fn default() -> RATCH5VAL {
        RATCH5VAL(0)
    }
}
impl core::fmt::Debug for RATCH5VAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RATCH5VAL")
            .field("VAL", &self.VAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RATCH5VAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RATCH5VAL {{ VAL: {=u32:?} }}", self.VAL())
    }
}
#[doc = "Timer Channel 6 Capture/Compare Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RATCH6VAL(pub u32);
impl RATCH6VAL {
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[must_use]
    #[inline(always)]
    pub const fn VAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[inline(always)]
    pub const fn set_VAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RATCH6VAL {
    #[inline(always)]
    fn default() -> RATCH6VAL {
        RATCH6VAL(0)
    }
}
impl core::fmt::Debug for RATCH6VAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RATCH6VAL")
            .field("VAL", &self.VAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RATCH6VAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RATCH6VAL {{ VAL: {=u32:?} }}", self.VAL())
    }
}
#[doc = "Timer Channel 7 Capture/Compare Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RATCH7VAL(pub u32);
impl RATCH7VAL {
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[must_use]
    #[inline(always)]
    pub const fn VAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[inline(always)]
    pub const fn set_VAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RATCH7VAL {
    #[inline(always)]
    fn default() -> RATCH7VAL {
        RATCH7VAL(0)
    }
}
impl core::fmt::Debug for RATCH7VAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RATCH7VAL")
            .field("VAL", &self.VAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RATCH7VAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RATCH7VAL {{ VAL: {=u32:?} }}", self.VAL())
    }
}
#[doc = "Radio Timer Counter Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RATCNT(pub u32);
impl RATCNT {
    #[doc = "31:0\\] Counter value. This is not writable while radio timer counter is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Counter value. This is not writable while radio timer counter is enabled."]
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RATCNT {
    #[inline(always)]
    fn default() -> RATCNT {
        RATCNT(0)
    }
}
impl core::fmt::Debug for RATCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RATCNT").field("CNT", &self.CNT()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RATCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RATCNT {{ CNT: {=u32:?} }}", self.CNT())
    }
}

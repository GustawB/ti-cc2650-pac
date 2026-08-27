#[doc = "Auto Take Sticky Request for Single Semaphore."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUTOTAKE(pub u32);
impl AUTOTAKE {
    #[doc = "2:0\\] Write the semaphore ID,0x0-0x7, to SMPH_ID to request this semaphore until it is granted. When semaphore SMPH_ID is granted, event AUX_EVCTL:EVSTAT0.AUX_SMPH_AUTOTAKE_DONE becomes 1. The event becomes 0 when software releases the semaphore or writes a new value to SMPH_ID. To avoid corrupted semaphores: - Usage of this functionality must be restricted to one CPU core. - Software must wait until AUX_EVCTL:EVSTAT0.AUX_SMPH_AUTOTAKE_DONE is 1 before it writes a new value to SMPH_ID."]
    #[must_use]
    #[inline(always)]
    pub const fn SMPH_ID(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Write the semaphore ID,0x0-0x7, to SMPH_ID to request this semaphore until it is granted. When semaphore SMPH_ID is granted, event AUX_EVCTL:EVSTAT0.AUX_SMPH_AUTOTAKE_DONE becomes 1. The event becomes 0 when software releases the semaphore or writes a new value to SMPH_ID. To avoid corrupted semaphores: - Usage of this functionality must be restricted to one CPU core. - Software must wait until AUX_EVCTL:EVSTAT0.AUX_SMPH_AUTOTAKE_DONE is 1 before it writes a new value to SMPH_ID."]
    #[inline(always)]
    pub const fn set_SMPH_ID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for AUTOTAKE {
    #[inline(always)]
    fn default() -> AUTOTAKE {
        AUTOTAKE(0)
    }
}
impl core::fmt::Debug for AUTOTAKE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOTAKE")
            .field("SMPH_ID", &self.SMPH_ID())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUTOTAKE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUTOTAKE {{ SMPH_ID: {=u8:?}, RESERVED3: {=u32:?} }}",
            self.SMPH_ID(),
            self.RESERVED3()
        )
    }
}
#[doc = "Semaphore 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH0(pub u32);
impl SMPH0 {
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
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
impl Default for SMPH0 {
    #[inline(always)]
    fn default() -> SMPH0 {
        SMPH0(0)
    }
}
impl core::fmt::Debug for SMPH0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH0")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH0 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "Semaphore 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH1(pub u32);
impl SMPH1 {
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
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
impl Default for SMPH1 {
    #[inline(always)]
    fn default() -> SMPH1 {
        SMPH1(0)
    }
}
impl core::fmt::Debug for SMPH1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH1")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH1 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "Semaphore 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH2(pub u32);
impl SMPH2 {
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
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
impl Default for SMPH2 {
    #[inline(always)]
    fn default() -> SMPH2 {
        SMPH2(0)
    }
}
impl core::fmt::Debug for SMPH2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH2")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH2 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "Semaphore 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH3(pub u32);
impl SMPH3 {
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
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
impl Default for SMPH3 {
    #[inline(always)]
    fn default() -> SMPH3 {
        SMPH3(0)
    }
}
impl core::fmt::Debug for SMPH3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH3")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH3 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "Semaphore 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH4(pub u32);
impl SMPH4 {
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
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
impl Default for SMPH4 {
    #[inline(always)]
    fn default() -> SMPH4 {
        SMPH4(0)
    }
}
impl core::fmt::Debug for SMPH4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH4")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH4 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "Semaphore 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH5(pub u32);
impl SMPH5 {
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
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
impl Default for SMPH5 {
    #[inline(always)]
    fn default() -> SMPH5 {
        SMPH5(0)
    }
}
impl core::fmt::Debug for SMPH5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH5")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH5 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "Semaphore 6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH6(pub u32);
impl SMPH6 {
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
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
impl Default for SMPH6 {
    #[inline(always)]
    fn default() -> SMPH6 {
        SMPH6(0)
    }
}
impl core::fmt::Debug for SMPH6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH6")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH6 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "Semaphore 7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH7(pub u32);
impl SMPH7 {
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Request or release of semaphore. Request by read: 0: Semaphore not available. 1: Semaphore granted. Release by write: 0: Do not use. 1: Release semaphore."]
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
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
impl Default for SMPH7 {
    #[inline(always)]
    fn default() -> SMPH7 {
        SMPH7(0)
    }
}
impl core::fmt::Debug for SMPH7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH7")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH7 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}

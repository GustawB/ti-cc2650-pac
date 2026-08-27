#[doc = "MCU SEMAPHORE 0 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK0(pub u32);
impl PEEK0 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK0 {
    #[inline(always)]
    fn default() -> PEEK0 {
        PEEK0(0)
    }
}
impl core::fmt::Debug for PEEK0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK0")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK0 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 1 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK1(pub u32);
impl PEEK1 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK1 {
    #[inline(always)]
    fn default() -> PEEK1 {
        PEEK1(0)
    }
}
impl core::fmt::Debug for PEEK1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK1")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK1 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 10 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK10(pub u32);
impl PEEK10 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK10 {
    #[inline(always)]
    fn default() -> PEEK10 {
        PEEK10(0)
    }
}
impl core::fmt::Debug for PEEK10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK10")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK10 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 11 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK11(pub u32);
impl PEEK11 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK11 {
    #[inline(always)]
    fn default() -> PEEK11 {
        PEEK11(0)
    }
}
impl core::fmt::Debug for PEEK11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK11")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK11 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 12 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK12(pub u32);
impl PEEK12 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK12 {
    #[inline(always)]
    fn default() -> PEEK12 {
        PEEK12(0)
    }
}
impl core::fmt::Debug for PEEK12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK12")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK12 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 13 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK13(pub u32);
impl PEEK13 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK13 {
    #[inline(always)]
    fn default() -> PEEK13 {
        PEEK13(0)
    }
}
impl core::fmt::Debug for PEEK13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK13")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK13 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 14 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK14(pub u32);
impl PEEK14 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK14 {
    #[inline(always)]
    fn default() -> PEEK14 {
        PEEK14(0)
    }
}
impl core::fmt::Debug for PEEK14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK14")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK14 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 15 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK15(pub u32);
impl PEEK15 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK15 {
    #[inline(always)]
    fn default() -> PEEK15 {
        PEEK15(0)
    }
}
impl core::fmt::Debug for PEEK15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK15")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK15 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 16 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK16(pub u32);
impl PEEK16 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK16 {
    #[inline(always)]
    fn default() -> PEEK16 {
        PEEK16(0)
    }
}
impl core::fmt::Debug for PEEK16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK16")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK16 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 17 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK17(pub u32);
impl PEEK17 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK17 {
    #[inline(always)]
    fn default() -> PEEK17 {
        PEEK17(0)
    }
}
impl core::fmt::Debug for PEEK17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK17")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK17 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 18 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK18(pub u32);
impl PEEK18 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK18 {
    #[inline(always)]
    fn default() -> PEEK18 {
        PEEK18(0)
    }
}
impl core::fmt::Debug for PEEK18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK18")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK18 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 19 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK19(pub u32);
impl PEEK19 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK19 {
    #[inline(always)]
    fn default() -> PEEK19 {
        PEEK19(0)
    }
}
impl core::fmt::Debug for PEEK19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK19")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK19 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 2 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK2(pub u32);
impl PEEK2 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK2 {
    #[inline(always)]
    fn default() -> PEEK2 {
        PEEK2(0)
    }
}
impl core::fmt::Debug for PEEK2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK2")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK2 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 20 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK20(pub u32);
impl PEEK20 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK20 {
    #[inline(always)]
    fn default() -> PEEK20 {
        PEEK20(0)
    }
}
impl core::fmt::Debug for PEEK20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK20")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK20 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 21 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK21(pub u32);
impl PEEK21 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK21 {
    #[inline(always)]
    fn default() -> PEEK21 {
        PEEK21(0)
    }
}
impl core::fmt::Debug for PEEK21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK21")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK21 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 22 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK22(pub u32);
impl PEEK22 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK22 {
    #[inline(always)]
    fn default() -> PEEK22 {
        PEEK22(0)
    }
}
impl core::fmt::Debug for PEEK22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK22")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK22 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 23 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK23(pub u32);
impl PEEK23 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK23 {
    #[inline(always)]
    fn default() -> PEEK23 {
        PEEK23(0)
    }
}
impl core::fmt::Debug for PEEK23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK23")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK23 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 24 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK24(pub u32);
impl PEEK24 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK24 {
    #[inline(always)]
    fn default() -> PEEK24 {
        PEEK24(0)
    }
}
impl core::fmt::Debug for PEEK24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK24")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK24 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 25 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK25(pub u32);
impl PEEK25 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK25 {
    #[inline(always)]
    fn default() -> PEEK25 {
        PEEK25(0)
    }
}
impl core::fmt::Debug for PEEK25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK25")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK25 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 26 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK26(pub u32);
impl PEEK26 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK26 {
    #[inline(always)]
    fn default() -> PEEK26 {
        PEEK26(0)
    }
}
impl core::fmt::Debug for PEEK26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK26")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK26 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 27 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK27(pub u32);
impl PEEK27 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK27 {
    #[inline(always)]
    fn default() -> PEEK27 {
        PEEK27(0)
    }
}
impl core::fmt::Debug for PEEK27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK27")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK27 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 28 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK28(pub u32);
impl PEEK28 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK28 {
    #[inline(always)]
    fn default() -> PEEK28 {
        PEEK28(0)
    }
}
impl core::fmt::Debug for PEEK28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK28")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK28 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 29 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK29(pub u32);
impl PEEK29 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK29 {
    #[inline(always)]
    fn default() -> PEEK29 {
        PEEK29(0)
    }
}
impl core::fmt::Debug for PEEK29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK29")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK29 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 3 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK3(pub u32);
impl PEEK3 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK3 {
    #[inline(always)]
    fn default() -> PEEK3 {
        PEEK3(0)
    }
}
impl core::fmt::Debug for PEEK3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK3")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK3 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 30 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK30(pub u32);
impl PEEK30 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK30 {
    #[inline(always)]
    fn default() -> PEEK30 {
        PEEK30(0)
    }
}
impl core::fmt::Debug for PEEK30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK30")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK30 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 31 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK31(pub u32);
impl PEEK31 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK31 {
    #[inline(always)]
    fn default() -> PEEK31 {
        PEEK31(0)
    }
}
impl core::fmt::Debug for PEEK31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK31")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK31 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 4 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK4(pub u32);
impl PEEK4 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK4 {
    #[inline(always)]
    fn default() -> PEEK4 {
        PEEK4(0)
    }
}
impl core::fmt::Debug for PEEK4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK4")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK4 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 5 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK5(pub u32);
impl PEEK5 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK5 {
    #[inline(always)]
    fn default() -> PEEK5 {
        PEEK5(0)
    }
}
impl core::fmt::Debug for PEEK5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK5")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK5 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 6 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK6(pub u32);
impl PEEK6 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK6 {
    #[inline(always)]
    fn default() -> PEEK6 {
        PEEK6(0)
    }
}
impl core::fmt::Debug for PEEK6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK6")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK6 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 7 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK7(pub u32);
impl PEEK7 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK7 {
    #[inline(always)]
    fn default() -> PEEK7 {
        PEEK7(0)
    }
}
impl core::fmt::Debug for PEEK7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK7")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK7 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 8 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK8(pub u32);
impl PEEK8 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK8 {
    #[inline(always)]
    fn default() -> PEEK8 {
        PEEK8(0)
    }
}
impl core::fmt::Debug for PEEK8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK8")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK8 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 9 ALIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PEEK9(pub u32);
impl PEEK9 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Used for semaphore debugging. A read operation will not change register value. Register writing is not possible."]
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
impl Default for PEEK9 {
    #[inline(always)]
    fn default() -> PEEK9 {
        PEEK9(0)
    }
}
impl core::fmt::Debug for PEEK9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK9")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PEEK9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PEEK9 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH0(pub u32);
impl SMPH0 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
#[doc = "MCU SEMAPHORE 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH1(pub u32);
impl SMPH1 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
#[doc = "MCU SEMAPHORE 10."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH10(pub u32);
impl SMPH10 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH10 {
    #[inline(always)]
    fn default() -> SMPH10 {
        SMPH10(0)
    }
}
impl core::fmt::Debug for SMPH10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH10")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH10 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 11."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH11(pub u32);
impl SMPH11 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH11 {
    #[inline(always)]
    fn default() -> SMPH11 {
        SMPH11(0)
    }
}
impl core::fmt::Debug for SMPH11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH11")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH11 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 12."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH12(pub u32);
impl SMPH12 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH12 {
    #[inline(always)]
    fn default() -> SMPH12 {
        SMPH12(0)
    }
}
impl core::fmt::Debug for SMPH12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH12")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH12 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 13."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH13(pub u32);
impl SMPH13 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH13 {
    #[inline(always)]
    fn default() -> SMPH13 {
        SMPH13(0)
    }
}
impl core::fmt::Debug for SMPH13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH13")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH13 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 14."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH14(pub u32);
impl SMPH14 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH14 {
    #[inline(always)]
    fn default() -> SMPH14 {
        SMPH14(0)
    }
}
impl core::fmt::Debug for SMPH14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH14")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH14 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 15."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH15(pub u32);
impl SMPH15 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH15 {
    #[inline(always)]
    fn default() -> SMPH15 {
        SMPH15(0)
    }
}
impl core::fmt::Debug for SMPH15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH15")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH15 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 16."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH16(pub u32);
impl SMPH16 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH16 {
    #[inline(always)]
    fn default() -> SMPH16 {
        SMPH16(0)
    }
}
impl core::fmt::Debug for SMPH16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH16")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH16 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 17."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH17(pub u32);
impl SMPH17 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH17 {
    #[inline(always)]
    fn default() -> SMPH17 {
        SMPH17(0)
    }
}
impl core::fmt::Debug for SMPH17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH17")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH17 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 18."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH18(pub u32);
impl SMPH18 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH18 {
    #[inline(always)]
    fn default() -> SMPH18 {
        SMPH18(0)
    }
}
impl core::fmt::Debug for SMPH18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH18")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH18 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 19."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH19(pub u32);
impl SMPH19 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH19 {
    #[inline(always)]
    fn default() -> SMPH19 {
        SMPH19(0)
    }
}
impl core::fmt::Debug for SMPH19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH19")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH19 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH2(pub u32);
impl SMPH2 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
#[doc = "MCU SEMAPHORE 20."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH20(pub u32);
impl SMPH20 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH20 {
    #[inline(always)]
    fn default() -> SMPH20 {
        SMPH20(0)
    }
}
impl core::fmt::Debug for SMPH20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH20")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH20 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 21."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH21(pub u32);
impl SMPH21 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH21 {
    #[inline(always)]
    fn default() -> SMPH21 {
        SMPH21(0)
    }
}
impl core::fmt::Debug for SMPH21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH21")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH21 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 22."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH22(pub u32);
impl SMPH22 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH22 {
    #[inline(always)]
    fn default() -> SMPH22 {
        SMPH22(0)
    }
}
impl core::fmt::Debug for SMPH22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH22")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH22 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 23."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH23(pub u32);
impl SMPH23 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH23 {
    #[inline(always)]
    fn default() -> SMPH23 {
        SMPH23(0)
    }
}
impl core::fmt::Debug for SMPH23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH23")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH23 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 24."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH24(pub u32);
impl SMPH24 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH24 {
    #[inline(always)]
    fn default() -> SMPH24 {
        SMPH24(0)
    }
}
impl core::fmt::Debug for SMPH24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH24")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH24 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 25."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH25(pub u32);
impl SMPH25 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH25 {
    #[inline(always)]
    fn default() -> SMPH25 {
        SMPH25(0)
    }
}
impl core::fmt::Debug for SMPH25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH25")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH25 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 26."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH26(pub u32);
impl SMPH26 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH26 {
    #[inline(always)]
    fn default() -> SMPH26 {
        SMPH26(0)
    }
}
impl core::fmt::Debug for SMPH26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH26")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH26 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 27."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH27(pub u32);
impl SMPH27 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH27 {
    #[inline(always)]
    fn default() -> SMPH27 {
        SMPH27(0)
    }
}
impl core::fmt::Debug for SMPH27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH27")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH27 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 28."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH28(pub u32);
impl SMPH28 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH28 {
    #[inline(always)]
    fn default() -> SMPH28 {
        SMPH28(0)
    }
}
impl core::fmt::Debug for SMPH28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH28")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH28 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 29."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH29(pub u32);
impl SMPH29 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH29 {
    #[inline(always)]
    fn default() -> SMPH29 {
        SMPH29(0)
    }
}
impl core::fmt::Debug for SMPH29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH29")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH29 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH3(pub u32);
impl SMPH3 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
#[doc = "MCU SEMAPHORE 30."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH30(pub u32);
impl SMPH30 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH30 {
    #[inline(always)]
    fn default() -> SMPH30 {
        SMPH30(0)
    }
}
impl core::fmt::Debug for SMPH30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH30")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH30 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 31."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH31(pub u32);
impl SMPH31 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH31 {
    #[inline(always)]
    fn default() -> SMPH31 {
        SMPH31(0)
    }
}
impl core::fmt::Debug for SMPH31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH31")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH31 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH4(pub u32);
impl SMPH4 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
#[doc = "MCU SEMAPHORE 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH5(pub u32);
impl SMPH5 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
#[doc = "MCU SEMAPHORE 6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH6(pub u32);
impl SMPH6 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
#[doc = "MCU SEMAPHORE 7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH7(pub u32);
impl SMPH7 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
#[doc = "MCU SEMAPHORE 8."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH8(pub u32);
impl SMPH8 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH8 {
    #[inline(always)]
    fn default() -> SMPH8 {
        SMPH8(0)
    }
}
impl core::fmt::Debug for SMPH8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH8")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH8 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU SEMAPHORE 9."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH9(pub u32);
impl SMPH9 {
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status when reading: 0: Semaphore is taken 1: Semaphore is available Reading the register causes it to change value to 0. Releasing the semaphore is done by writing 1."]
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
impl Default for SMPH9 {
    #[inline(always)]
    fn default() -> SMPH9 {
        SMPH9(0)
    }
}
impl core::fmt::Debug for SMPH9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPH9")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMPH9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMPH9 {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}

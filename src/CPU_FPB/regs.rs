#[doc = "Comparator 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP0(pub u32);
impl COMP0 {
    #[doc = "0:0\\] Compare and remap enable comparator 0. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 0 disabled 0x1: Compare and remap for comparator 0 enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Compare and remap enable comparator 0. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 0 disabled 0x1: Compare and remap for comparator 0 enabled."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "28:2\\] Comparison address."]
    #[must_use]
    #[inline(always)]
    pub const fn COMP(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "28:2\\] Comparison address."]
    #[inline(always)]
    pub const fn set_COMP(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 2usize)) | (((val as u32) & 0x07ff_ffff) << 2usize);
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[must_use]
    #[inline(always)]
    pub const fn REPLACE(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[inline(always)]
    pub const fn set_REPLACE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for COMP0 {
    #[inline(always)]
    fn default() -> COMP0 {
        COMP0(0)
    }
}
impl core::fmt::Debug for COMP0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP0")
            .field("ENABLE", &self.ENABLE())
            .field("RESERVED1", &self.RESERVED1())
            .field("COMP", &self.COMP())
            .field("RESERVED29", &self.RESERVED29())
            .field("REPLACE", &self.REPLACE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COMP0 {{ ENABLE: {=bool:?}, RESERVED1: {=bool:?}, COMP: {=u32:?}, RESERVED29: {=bool:?}, REPLACE: {=u8:?} }}",
            self.ENABLE(),
            self.RESERVED1(),
            self.COMP(),
            self.RESERVED29(),
            self.REPLACE()
        )
    }
}
#[doc = "Comparator 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP1(pub u32);
impl COMP1 {
    #[doc = "0:0\\] Compare and remap enable comparator 1. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 1 disabled 0x1: Compare and remap for comparator 1 enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Compare and remap enable comparator 1. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 1 disabled 0x1: Compare and remap for comparator 1 enabled."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "28:2\\] Comparison address."]
    #[must_use]
    #[inline(always)]
    pub const fn COMP(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "28:2\\] Comparison address."]
    #[inline(always)]
    pub const fn set_COMP(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 2usize)) | (((val as u32) & 0x07ff_ffff) << 2usize);
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[must_use]
    #[inline(always)]
    pub const fn REPLACE(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[inline(always)]
    pub const fn set_REPLACE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for COMP1 {
    #[inline(always)]
    fn default() -> COMP1 {
        COMP1(0)
    }
}
impl core::fmt::Debug for COMP1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP1")
            .field("ENABLE", &self.ENABLE())
            .field("RESERVED1", &self.RESERVED1())
            .field("COMP", &self.COMP())
            .field("RESERVED29", &self.RESERVED29())
            .field("REPLACE", &self.REPLACE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COMP1 {{ ENABLE: {=bool:?}, RESERVED1: {=bool:?}, COMP: {=u32:?}, RESERVED29: {=bool:?}, REPLACE: {=u8:?} }}",
            self.ENABLE(),
            self.RESERVED1(),
            self.COMP(),
            self.RESERVED29(),
            self.REPLACE()
        )
    }
}
#[doc = "Comparator 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP2(pub u32);
impl COMP2 {
    #[doc = "0:0\\] Compare and remap enable comparator 2. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 2 disabled 0x1: Compare and remap for comparator 2 enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Compare and remap enable comparator 2. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 2 disabled 0x1: Compare and remap for comparator 2 enabled."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "28:2\\] Comparison address."]
    #[must_use]
    #[inline(always)]
    pub const fn COMP(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "28:2\\] Comparison address."]
    #[inline(always)]
    pub const fn set_COMP(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 2usize)) | (((val as u32) & 0x07ff_ffff) << 2usize);
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[must_use]
    #[inline(always)]
    pub const fn REPLACE(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[inline(always)]
    pub const fn set_REPLACE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for COMP2 {
    #[inline(always)]
    fn default() -> COMP2 {
        COMP2(0)
    }
}
impl core::fmt::Debug for COMP2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP2")
            .field("ENABLE", &self.ENABLE())
            .field("RESERVED1", &self.RESERVED1())
            .field("COMP", &self.COMP())
            .field("RESERVED29", &self.RESERVED29())
            .field("REPLACE", &self.REPLACE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COMP2 {{ ENABLE: {=bool:?}, RESERVED1: {=bool:?}, COMP: {=u32:?}, RESERVED29: {=bool:?}, REPLACE: {=u8:?} }}",
            self.ENABLE(),
            self.RESERVED1(),
            self.COMP(),
            self.RESERVED29(),
            self.REPLACE()
        )
    }
}
#[doc = "Comparator 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP3(pub u32);
impl COMP3 {
    #[doc = "0:0\\] Compare and remap enable comparator 3. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 3 disabled 0x1: Compare and remap for comparator 3 enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Compare and remap enable comparator 3. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 3 disabled 0x1: Compare and remap for comparator 3 enabled."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "28:2\\] Comparison address."]
    #[must_use]
    #[inline(always)]
    pub const fn COMP(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "28:2\\] Comparison address."]
    #[inline(always)]
    pub const fn set_COMP(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 2usize)) | (((val as u32) & 0x07ff_ffff) << 2usize);
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[must_use]
    #[inline(always)]
    pub const fn REPLACE(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[inline(always)]
    pub const fn set_REPLACE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for COMP3 {
    #[inline(always)]
    fn default() -> COMP3 {
        COMP3(0)
    }
}
impl core::fmt::Debug for COMP3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP3")
            .field("ENABLE", &self.ENABLE())
            .field("RESERVED1", &self.RESERVED1())
            .field("COMP", &self.COMP())
            .field("RESERVED29", &self.RESERVED29())
            .field("REPLACE", &self.REPLACE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COMP3 {{ ENABLE: {=bool:?}, RESERVED1: {=bool:?}, COMP: {=u32:?}, RESERVED29: {=bool:?}, REPLACE: {=u8:?} }}",
            self.ENABLE(),
            self.RESERVED1(),
            self.COMP(),
            self.RESERVED29(),
            self.REPLACE()
        )
    }
}
#[doc = "Comparator 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP4(pub u32);
impl COMP4 {
    #[doc = "0:0\\] Compare and remap enable comparator 4. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 4 disabled 0x1: Compare and remap for comparator 4 enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Compare and remap enable comparator 4. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 4 disabled 0x1: Compare and remap for comparator 4 enabled."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "28:2\\] Comparison address."]
    #[must_use]
    #[inline(always)]
    pub const fn COMP(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "28:2\\] Comparison address."]
    #[inline(always)]
    pub const fn set_COMP(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 2usize)) | (((val as u32) & 0x07ff_ffff) << 2usize);
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[must_use]
    #[inline(always)]
    pub const fn REPLACE(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[inline(always)]
    pub const fn set_REPLACE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for COMP4 {
    #[inline(always)]
    fn default() -> COMP4 {
        COMP4(0)
    }
}
impl core::fmt::Debug for COMP4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP4")
            .field("ENABLE", &self.ENABLE())
            .field("RESERVED1", &self.RESERVED1())
            .field("COMP", &self.COMP())
            .field("RESERVED29", &self.RESERVED29())
            .field("REPLACE", &self.REPLACE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COMP4 {{ ENABLE: {=bool:?}, RESERVED1: {=bool:?}, COMP: {=u32:?}, RESERVED29: {=bool:?}, REPLACE: {=u8:?} }}",
            self.ENABLE(),
            self.RESERVED1(),
            self.COMP(),
            self.RESERVED29(),
            self.REPLACE()
        )
    }
}
#[doc = "Comparator 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP5(pub u32);
impl COMP5 {
    #[doc = "0:0\\] Compare and remap enable comparator 5. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 5 disabled 0x1: Compare and remap for comparator 5 enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Compare and remap enable comparator 5. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 5 disabled 0x1: Compare and remap for comparator 5 enabled."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "28:2\\] Comparison address."]
    #[must_use]
    #[inline(always)]
    pub const fn COMP(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "28:2\\] Comparison address."]
    #[inline(always)]
    pub const fn set_COMP(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 2usize)) | (((val as u32) & 0x07ff_ffff) << 2usize);
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[must_use]
    #[inline(always)]
    pub const fn REPLACE(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[inline(always)]
    pub const fn set_REPLACE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for COMP5 {
    #[inline(always)]
    fn default() -> COMP5 {
        COMP5(0)
    }
}
impl core::fmt::Debug for COMP5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP5")
            .field("ENABLE", &self.ENABLE())
            .field("RESERVED1", &self.RESERVED1())
            .field("COMP", &self.COMP())
            .field("RESERVED29", &self.RESERVED29())
            .field("REPLACE", &self.REPLACE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COMP5 {{ ENABLE: {=bool:?}, RESERVED1: {=bool:?}, COMP: {=u32:?}, RESERVED29: {=bool:?}, REPLACE: {=u8:?} }}",
            self.ENABLE(),
            self.RESERVED1(),
            self.COMP(),
            self.RESERVED29(),
            self.REPLACE()
        )
    }
}
#[doc = "Comparator 6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP6(pub u32);
impl COMP6 {
    #[doc = "0:0\\] Compare and remap enable comparator 6. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 6 disabled 0x1: Compare and remap for comparator 6 enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Compare and remap enable comparator 6. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 6 disabled 0x1: Compare and remap for comparator 6 enabled."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "28:2\\] Comparison address."]
    #[must_use]
    #[inline(always)]
    pub const fn COMP(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "28:2\\] Comparison address."]
    #[inline(always)]
    pub const fn set_COMP(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 2usize)) | (((val as u32) & 0x07ff_ffff) << 2usize);
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Comparator 6 is a literal comparator and the only supported setting is 0x0. Other settings will be ignored. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[must_use]
    #[inline(always)]
    pub const fn REPLACE(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Comparator 6 is a literal comparator and the only supported setting is 0x0. Other settings will be ignored. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[inline(always)]
    pub const fn set_REPLACE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for COMP6 {
    #[inline(always)]
    fn default() -> COMP6 {
        COMP6(0)
    }
}
impl core::fmt::Debug for COMP6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP6")
            .field("ENABLE", &self.ENABLE())
            .field("RESERVED1", &self.RESERVED1())
            .field("COMP", &self.COMP())
            .field("RESERVED29", &self.RESERVED29())
            .field("REPLACE", &self.REPLACE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COMP6 {{ ENABLE: {=bool:?}, RESERVED1: {=bool:?}, COMP: {=u32:?}, RESERVED29: {=bool:?}, REPLACE: {=u8:?} }}",
            self.ENABLE(),
            self.RESERVED1(),
            self.COMP(),
            self.RESERVED29(),
            self.REPLACE()
        )
    }
}
#[doc = "Comparator 7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP7(pub u32);
impl COMP7 {
    #[doc = "0:0\\] Compare and remap enable comparator 7. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 7 disabled 0x1: Compare and remap for comparator 7 enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Compare and remap enable comparator 7. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 7 disabled 0x1: Compare and remap for comparator 7 enabled."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "28:2\\] Comparison address."]
    #[must_use]
    #[inline(always)]
    pub const fn COMP(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "28:2\\] Comparison address."]
    #[inline(always)]
    pub const fn set_COMP(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 2usize)) | (((val as u32) & 0x07ff_ffff) << 2usize);
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Comparator 7 is a literal comparator and the only supported setting is 0x0. Other settings will be ignored. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[must_use]
    #[inline(always)]
    pub const fn REPLACE(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "31:30\\] This selects what happens when the COMP address is matched. Comparator 7 is a literal comparator and the only supported setting is 0x0. Other settings will be ignored. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[inline(always)]
    pub const fn set_REPLACE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for COMP7 {
    #[inline(always)]
    fn default() -> COMP7 {
        COMP7(0)
    }
}
impl core::fmt::Debug for COMP7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP7")
            .field("ENABLE", &self.ENABLE())
            .field("RESERVED1", &self.RESERVED1())
            .field("COMP", &self.COMP())
            .field("RESERVED29", &self.RESERVED29())
            .field("REPLACE", &self.REPLACE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COMP7 {{ ENABLE: {=bool:?}, RESERVED1: {=bool:?}, COMP: {=u32:?}, RESERVED29: {=bool:?}, REPLACE: {=u8:?} }}",
            self.ENABLE(),
            self.RESERVED1(),
            self.COMP(),
            self.RESERVED29(),
            self.REPLACE()
        )
    }
}
#[doc = "Control This register is used to enable the flash patch block."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "0:0\\] Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Flash patch unit enable bit 0x0: Flash patch unit disabled 0x1: Flash patch unit enabled."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Key field. In order to write to this register, this bit-field must be written to '1'. This bit always reads 0."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Key field. In order to write to this register, this bit-field must be written to '1'. This bit always reads 0."]
    #[inline(always)]
    pub const fn set_KEY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "7:4\\] Number of code slots field. 0x0: No code slots 0x2: Two code slots 0x6: Six code slots."]
    #[must_use]
    #[inline(always)]
    pub const fn NUM_CODE1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "7:4\\] Number of code slots field. 0x0: No code slots 0x2: Two code slots 0x6: Six code slots."]
    #[inline(always)]
    pub const fn set_NUM_CODE1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "11:8\\] Number of literal slots field. 0x0: No literal slots 0x2: Two literal slots."]
    #[must_use]
    #[inline(always)]
    pub const fn NUM_LIT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "11:8\\] Number of literal slots field. 0x0: No literal slots 0x2: Two literal slots."]
    #[inline(always)]
    pub const fn set_NUM_LIT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "13:12\\] Number of full banks of code comparators, sixteen comparators per bank. Where less than sixteen code comparators are provided, the bank count is zero, and the number present indicated by NUM_CODE1. This read only field contains 3'b000 to indicate 0 banks for Cortex-M processor."]
    #[must_use]
    #[inline(always)]
    pub const fn NUM_CODE2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "13:12\\] Number of full banks of code comparators, sixteen comparators per bank. Where less than sixteen code comparators are provided, the bank count is zero, and the number present indicated by NUM_CODE1. This read only field contains 3'b000 to indicate 0 banks for Cortex-M processor."]
    #[inline(always)]
    pub const fn set_NUM_CODE2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "31:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED14(&self) -> u32 {
        let val = (self.0 >> 14usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "31:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED14(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 14usize)) | (((val as u32) & 0x0003_ffff) << 14usize);
    }
}
impl Default for CTRL {
    #[inline(always)]
    fn default() -> CTRL {
        CTRL(0)
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("ENABLE", &self.ENABLE())
            .field("KEY", &self.KEY())
            .field("RESERVED2", &self.RESERVED2())
            .field("NUM_CODE1", &self.NUM_CODE1())
            .field("NUM_LIT", &self.NUM_LIT())
            .field("NUM_CODE2", &self.NUM_CODE2())
            .field("RESERVED14", &self.RESERVED14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ ENABLE: {=bool:?}, KEY: {=bool:?}, RESERVED2: {=u8:?}, NUM_CODE1: {=u8:?}, NUM_LIT: {=u8:?}, NUM_CODE2: {=u8:?}, RESERVED14: {=u32:?} }}",
            self.ENABLE(),
            self.KEY(),
            self.RESERVED2(),
            self.NUM_CODE1(),
            self.NUM_LIT(),
            self.NUM_CODE2(),
            self.RESERVED14()
        )
    }
}
#[doc = "Remap This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct REMAP(pub u32);
impl REMAP {
    #[doc = "4:0\\] This field always reads 0. Writing to this field is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "4:0\\] This field always reads 0. Writing to this field is ignored."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "28:5\\] Remap base address field."]
    #[must_use]
    #[inline(always)]
    pub const fn REMAP(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "28:5\\] Remap base address field."]
    #[inline(always)]
    pub const fn set_REMAP(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 5usize)) | (((val as u32) & 0x00ff_ffff) << 5usize);
    }
    #[doc = "31:29\\] This field always reads 3'b001. Writing to this field is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED29(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "31:29\\] This field always reads 3'b001. Writing to this field is ignored."]
    #[inline(always)]
    pub const fn set_RESERVED29(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for REMAP {
    #[inline(always)]
    fn default() -> REMAP {
        REMAP(0)
    }
}
impl core::fmt::Debug for REMAP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP")
            .field("RESERVED0", &self.RESERVED0())
            .field("REMAP", &self.REMAP())
            .field("RESERVED29", &self.RESERVED29())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for REMAP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "REMAP {{ RESERVED0: {=u8:?}, REMAP: {=u32:?}, RESERVED29: {=u8:?} }}",
            self.RESERVED0(),
            self.REMAP(),
            self.RESERVED29()
        )
    }
}

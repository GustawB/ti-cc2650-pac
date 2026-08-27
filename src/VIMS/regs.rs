#[doc = "Control Configure VIMS mode and line buffer settings."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTL(pub u32);
impl CTL {
    #[doc = "1:0\\] VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1. Note: Transaction from CACHE mode to GPRAM mode should be done through OFF mode to minimize flash block delay."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CTL_MODE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CTL_MODE::from_bits(val as u8)
    }
    #[doc = "1:0\\] VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1. Note: Transaction from CACHE mode to GPRAM mode should be done through OFF mode to minimize flash block delay."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CTL_MODE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "2:2\\] Tag prefetch control 0: Disabled 1: Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn PREF_EN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Tag prefetch control 0: Disabled 1: Enabled."]
    #[inline(always)]
    pub const fn set_PREF_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Icode/Dcode and sysbus arbitation scheme 0: Static arbitration (icode/docde > sysbus) 1: Round-robin arbitration."]
    #[must_use]
    #[inline(always)]
    pub const fn ARB_CFG(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Icode/Dcode and sysbus arbitation scheme 0: Static arbitration (icode/docde > sysbus) 1: Round-robin arbitration."]
    #[inline(always)]
    pub const fn set_ARB_CFG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Sysbus flash line buffer control 0: Enable 1: Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSBUS_LB_DIS(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Sysbus flash line buffer control 0: Enable 1: Disable."]
    #[inline(always)]
    pub const fn set_SYSBUS_LB_DIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Icode/Dcode flash line buffer control 0: Enable 1: Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn IDCODE_LB_DIS(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Icode/Dcode flash line buffer control 0: Enable 1: Disable."]
    #[inline(always)]
    pub const fn set_IDCODE_LB_DIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "28:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "28:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 6usize)) | (((val as u32) & 0x007f_ffff) << 6usize);
    }
    #[doc = "29:29\\] 0: The in-built clock gate functionality is bypassed. 1: The in-built clock gate functionality is enabled, automatically gating the clock when not needed."]
    #[must_use]
    #[inline(always)]
    pub const fn DYN_CG_EN(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: The in-built clock gate functionality is bypassed. 1: The in-built clock gate functionality is enabled, automatically gating the clock when not needed."]
    #[inline(always)]
    pub const fn set_DYN_CG_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Set this bit to enable statistic counters."]
    #[must_use]
    #[inline(always)]
    pub const fn STATS_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Set this bit to enable statistic counters."]
    #[inline(always)]
    pub const fn set_STATS_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Set this bit to clear statistic counters."]
    #[must_use]
    #[inline(always)]
    pub const fn STATS_CLR(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Set this bit to clear statistic counters."]
    #[inline(always)]
    pub const fn set_STATS_CLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CTL {
    #[inline(always)]
    fn default() -> CTL {
        CTL(0)
    }
}
impl core::fmt::Debug for CTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL")
            .field("MODE", &self.MODE())
            .field("PREF_EN", &self.PREF_EN())
            .field("ARB_CFG", &self.ARB_CFG())
            .field("SYSBUS_LB_DIS", &self.SYSBUS_LB_DIS())
            .field("IDCODE_LB_DIS", &self.IDCODE_LB_DIS())
            .field("RESERVED6", &self.RESERVED6())
            .field("DYN_CG_EN", &self.DYN_CG_EN())
            .field("STATS_EN", &self.STATS_EN())
            .field("STATS_CLR", &self.STATS_CLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTL {{ MODE: {:?}, PREF_EN: {=bool:?}, ARB_CFG: {=bool:?}, SYSBUS_LB_DIS: {=bool:?}, IDCODE_LB_DIS: {=bool:?}, RESERVED6: {=u32:?}, DYN_CG_EN: {=bool:?}, STATS_EN: {=bool:?}, STATS_CLR: {=bool:?} }}",
            self.MODE(),
            self.PREF_EN(),
            self.ARB_CFG(),
            self.SYSBUS_LB_DIS(),
            self.IDCODE_LB_DIS(),
            self.RESERVED6(),
            self.DYN_CG_EN(),
            self.STATS_EN(),
            self.STATS_CLR()
        )
    }
}
#[doc = "Status Displays current VIMS mode and line buffer status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STAT(pub u32);
impl STAT {
    #[doc = "1:0\\] Current VIMS mode."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::STAT_MODE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::STAT_MODE::from_bits(val as u8)
    }
    #[doc = "1:0\\] Current VIMS mode."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::STAT_MODE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "2:2\\] This bit is set when invalidation of the cache memory is active / ongoing."]
    #[must_use]
    #[inline(always)]
    pub const fn INV(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] This bit is set when invalidation of the cache memory is active / ongoing."]
    #[inline(always)]
    pub const fn set_INV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] VIMS mode change status 0: VIMS is in the mode defined by MODE 1: VIMS is in the process of changing to the mode given in CTL.MODE."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE_CHANGING(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] VIMS mode change status 0: VIMS is in the mode defined by MODE 1: VIMS is in the process of changing to the mode given in CTL.MODE."]
    #[inline(always)]
    pub const fn set_MODE_CHANGING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Sysbus flash line buffer control 0: Enabled or in transition to disabled 1: Disabled and flushed."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSBUS_LB_DIS(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Sysbus flash line buffer control 0: Enabled or in transition to disabled 1: Disabled and flushed."]
    #[inline(always)]
    pub const fn set_SYSBUS_LB_DIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Icode/Dcode flash line buffer status 0: Enabled or in transition to disabled 1: Disabled and flushed."]
    #[must_use]
    #[inline(always)]
    pub const fn IDCODE_LB_DIS(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Icode/Dcode flash line buffer status 0: Enabled or in transition to disabled 1: Disabled and flushed."]
    #[inline(always)]
    pub const fn set_IDCODE_LB_DIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for STAT {
    #[inline(always)]
    fn default() -> STAT {
        STAT(0)
    }
}
impl core::fmt::Debug for STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("MODE", &self.MODE())
            .field("INV", &self.INV())
            .field("MODE_CHANGING", &self.MODE_CHANGING())
            .field("SYSBUS_LB_DIS", &self.SYSBUS_LB_DIS())
            .field("IDCODE_LB_DIS", &self.IDCODE_LB_DIS())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STAT {{ MODE: {:?}, INV: {=bool:?}, MODE_CHANGING: {=bool:?}, SYSBUS_LB_DIS: {=bool:?}, IDCODE_LB_DIS: {=bool:?}, RESERVED6: {=u32:?} }}",
            self.MODE(),
            self.INV(),
            self.MODE_CHANGING(),
            self.SYSBUS_LB_DIS(),
            self.IDCODE_LB_DIS(),
            self.RESERVED6()
        )
    }
}

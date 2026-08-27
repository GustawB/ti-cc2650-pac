#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ACC(pub u32);
impl ACC {
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ACCUMULATOR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ACCUMULATOR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED24(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED24(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for ACC {
    #[inline(always)]
    fn default() -> ACC {
        ACC(0)
    }
}
impl core::fmt::Debug for ACC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACC")
            .field("ACCUMULATOR", &self.ACCUMULATOR())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ACC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ACC {{ ACCUMULATOR: {=u32:?}, RESERVED24: {=u8:?} }}",
            self.ACCUMULATOR(),
            self.RESERVED24()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BOUNDARY(pub u32);
impl BOUNDARY {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn INPUTENABLE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_INPUTENABLE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SYS_WS_READ_STATES(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SYS_WS_READ_STATES(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "9:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SYS_REPAIR_EN(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "9:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SYS_REPAIR_EN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "10:10\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SYS_DIEID_AUTOLOAD_EN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SYS_DIEID_AUTOLOAD_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFC_FDI(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFC_FDI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SYS_ECC_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SYS_ECC_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SYS_ECC_SELF_TEST_EN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SYS_ECC_SELF_TEST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "17:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn OUTPUTENABLE(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x0f;
        val as u8
    }
    #[doc = "17:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_OUTPUTENABLE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 14usize)) | (((val as u32) & 0x0f) << 14usize);
    }
    #[doc = "18:18\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFC_AUTOLOAD_ERROR(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFC_AUTOLOAD_ERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFC_INSTRUCTION_ERROR(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFC_INSTRUCTION_ERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFC_INSTRUCTION_INFO(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFC_INSTRUCTION_INFO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFC_SELF_TEST_ERROR(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFC_SELF_TEST_ERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SPARE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DISROW0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DISROW0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED24(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED24(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for BOUNDARY {
    #[inline(always)]
    fn default() -> BOUNDARY {
        BOUNDARY(0)
    }
}
impl core::fmt::Debug for BOUNDARY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOUNDARY")
            .field("INPUTENABLE", &self.INPUTENABLE())
            .field("SYS_WS_READ_STATES", &self.SYS_WS_READ_STATES())
            .field("SYS_REPAIR_EN", &self.SYS_REPAIR_EN())
            .field("SYS_DIEID_AUTOLOAD_EN", &self.SYS_DIEID_AUTOLOAD_EN())
            .field("EFC_FDI", &self.EFC_FDI())
            .field("SYS_ECC_OVERRIDE_EN", &self.SYS_ECC_OVERRIDE_EN())
            .field("SYS_ECC_SELF_TEST_EN", &self.SYS_ECC_SELF_TEST_EN())
            .field("OUTPUTENABLE", &self.OUTPUTENABLE())
            .field("EFC_AUTOLOAD_ERROR", &self.EFC_AUTOLOAD_ERROR())
            .field("EFC_INSTRUCTION_ERROR", &self.EFC_INSTRUCTION_ERROR())
            .field("EFC_INSTRUCTION_INFO", &self.EFC_INSTRUCTION_INFO())
            .field("EFC_SELF_TEST_ERROR", &self.EFC_SELF_TEST_ERROR())
            .field("SPARE", &self.SPARE())
            .field("DISROW0", &self.DISROW0())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BOUNDARY {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BOUNDARY {{ INPUTENABLE: {=u8:?}, SYS_WS_READ_STATES: {=u8:?}, SYS_REPAIR_EN: {=u8:?}, SYS_DIEID_AUTOLOAD_EN: {=bool:?}, EFC_FDI: {=bool:?}, SYS_ECC_OVERRIDE_EN: {=bool:?}, SYS_ECC_SELF_TEST_EN: {=bool:?}, OUTPUTENABLE: {=u8:?}, EFC_AUTOLOAD_ERROR: {=bool:?}, EFC_INSTRUCTION_ERROR: {=bool:?}, EFC_INSTRUCTION_INFO: {=bool:?}, EFC_SELF_TEST_ERROR: {=bool:?}, SPARE: {=bool:?}, DISROW0: {=bool:?}, RESERVED24: {=u8:?} }}",
            self.INPUTENABLE(),
            self.SYS_WS_READ_STATES(),
            self.SYS_REPAIR_EN(),
            self.SYS_DIEID_AUTOLOAD_EN(),
            self.EFC_FDI(),
            self.SYS_ECC_OVERRIDE_EN(),
            self.SYS_ECC_SELF_TEST_EN(),
            self.OUTPUTENABLE(),
            self.EFC_AUTOLOAD_ERROR(),
            self.EFC_INSTRUCTION_ERROR(),
            self.EFC_INSTRUCTION_INFO(),
            self.EFC_SELF_TEST_ERROR(),
            self.SPARE(),
            self.DISROW0(),
            self.RESERVED24()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG(pub u32);
impl CFG {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_IDLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DIS_IDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_STANDBY(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DIS_STANDBY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE_SWINTF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ENABLE_SWINTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_READACCESS(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DIS_READACCESS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_EFUSECLK(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DIS_EFUSECLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "7:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn STANDBY_PW_SEL(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "7:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_STANDBY_PW_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn STANDBY_MODE_SEL(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_STANDBY_MODE_SEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "31:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "31:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for CFG {
    #[inline(always)]
    fn default() -> CFG {
        CFG(0)
    }
}
impl core::fmt::Debug for CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("DIS_IDLE", &self.DIS_IDLE())
            .field("DIS_STANDBY", &self.DIS_STANDBY())
            .field("RESERVED2", &self.RESERVED2())
            .field("ENABLE_SWINTF", &self.ENABLE_SWINTF())
            .field("DIS_READACCESS", &self.DIS_READACCESS())
            .field("DIS_EFUSECLK", &self.DIS_EFUSECLK())
            .field("STANDBY_PW_SEL", &self.STANDBY_PW_SEL())
            .field("STANDBY_MODE_SEL", &self.STANDBY_MODE_SEL())
            .field("RESERVED9", &self.RESERVED9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFG {{ DIS_IDLE: {=bool:?}, DIS_STANDBY: {=bool:?}, RESERVED2: {=bool:?}, ENABLE_SWINTF: {=bool:?}, DIS_READACCESS: {=bool:?}, DIS_EFUSECLK: {=bool:?}, STANDBY_PW_SEL: {=u8:?}, STANDBY_MODE_SEL: {=bool:?}, RESERVED9: {=u32:?} }}",
            self.DIS_IDLE(),
            self.DIS_STANDBY(),
            self.RESERVED2(),
            self.ENABLE_SWINTF(),
            self.DIS_READACCESS(),
            self.DIS_EFUSECLK(),
            self.STANDBY_PW_SEL(),
            self.STANDBY_MODE_SEL(),
            self.RESERVED9()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DATALOWER(pub u32);
impl DATALOWER {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DATALOWER {
    #[inline(always)]
    fn default() -> DATALOWER {
        DATALOWER(0)
    }
}
impl core::fmt::Debug for DATALOWER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATALOWER")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DATALOWER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DATALOWER {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DATAUPPER(pub u32);
impl DATAUPPER {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn R(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_R(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn P(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_P(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "7:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "7:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SPARE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
    }
    #[doc = "31:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for DATAUPPER {
    #[inline(always)]
    fn default() -> DATAUPPER {
        DATAUPPER(0)
    }
}
impl core::fmt::Debug for DATAUPPER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATAUPPER")
            .field("EEN", &self.EEN())
            .field("R", &self.R())
            .field("P", &self.P())
            .field("SPARE", &self.SPARE())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DATAUPPER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DATAUPPER {{ EEN: {=bool:?}, R: {=bool:?}, P: {=bool:?}, SPARE: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.EEN(),
            self.R(),
            self.P(),
            self.SPARE(),
            self.RESERVED8()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EEPROM_CFG(pub u32);
impl EEPROM_CFG {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTOSTART_GRACE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_AUTOSTART_GRACE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for EEPROM_CFG {
    #[inline(always)]
    fn default() -> EEPROM_CFG {
        EEPROM_CFG(0)
    }
}
impl core::fmt::Debug for EEPROM_CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EEPROM_CFG")
            .field("AUTOSTART_GRACE", &self.AUTOSTART_GRACE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EEPROM_CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EEPROM_CFG {{ AUTOSTART_GRACE: {=u32:?} }}",
            self.AUTOSTART_GRACE()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EFUSE(pub u32);
impl EFUSE {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DUMPWORD(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DUMPWORD(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "28:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn INSTRUCTION(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "28:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_INSTRUCTION(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "31:29\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED29(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "31:29\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED29(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for EFUSE {
    #[inline(always)]
    fn default() -> EFUSE {
        EFUSE(0)
    }
}
impl core::fmt::Debug for EFUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSE")
            .field("DUMPWORD", &self.DUMPWORD())
            .field("RESERVED16", &self.RESERVED16())
            .field("INSTRUCTION", &self.INSTRUCTION())
            .field("RESERVED29", &self.RESERVED29())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EFUSE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EFUSE {{ DUMPWORD: {=u16:?}, RESERVED16: {=u8:?}, INSTRUCTION: {=u8:?}, RESERVED29: {=u8:?} }}",
            self.DUMPWORD(),
            self.RESERVED16(),
            self.INSTRUCTION(),
            self.RESERVED29()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EFUSEADDR(pub u32);
impl EFUSEADDR {
    #[doc = "10:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ROW(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "10:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ROW(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "15:11\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BLOCK(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "15:11\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BLOCK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for EFUSEADDR {
    #[inline(always)]
    fn default() -> EFUSEADDR {
        EFUSEADDR(0)
    }
}
impl core::fmt::Debug for EFUSEADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSEADDR")
            .field("ROW", &self.ROW())
            .field("BLOCK", &self.BLOCK())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EFUSEADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EFUSEADDR {{ ROW: {=u16:?}, BLOCK: {=u8:?}, RESERVED16: {=u16:?} }}",
            self.ROW(),
            self.BLOCK(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EFUSECFG(pub u32);
impl EFUSECFG {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn GATING(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_GATING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "2:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "2:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "4:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SLAVEPOWER(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "4:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SLAVEPOWER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "7:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "7:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IDLEGATING(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IDLEGATING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "31:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "31:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for EFUSECFG {
    #[inline(always)]
    fn default() -> EFUSECFG {
        EFUSECFG(0)
    }
}
impl core::fmt::Debug for EFUSECFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSECFG")
            .field("GATING", &self.GATING())
            .field("RESERVED1", &self.RESERVED1())
            .field("SLAVEPOWER", &self.SLAVEPOWER())
            .field("RESERVED5", &self.RESERVED5())
            .field("IDLEGATING", &self.IDLEGATING())
            .field("RESERVED9", &self.RESERVED9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EFUSECFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EFUSECFG {{ GATING: {=bool:?}, RESERVED1: {=u8:?}, SLAVEPOWER: {=u8:?}, RESERVED5: {=u8:?}, IDLEGATING: {=bool:?}, RESERVED9: {=u32:?} }}",
            self.GATING(),
            self.RESERVED1(),
            self.SLAVEPOWER(),
            self.RESERVED5(),
            self.IDLEGATING(),
            self.RESERVED9()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EFUSECRA(pub u32);
impl EFUSECRA {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "31:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "31:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for EFUSECRA {
    #[inline(always)]
    fn default() -> EFUSECRA {
        EFUSECRA(0)
    }
}
impl core::fmt::Debug for EFUSECRA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSECRA")
            .field("DATA", &self.DATA())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EFUSECRA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EFUSECRA {{ DATA: {=u8:?}, RESERVED6: {=u32:?} }}",
            self.DATA(),
            self.RESERVED6()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EFUSEERROR(pub u32);
impl EFUSEERROR {
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CODE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "31:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "31:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for EFUSEERROR {
    #[inline(always)]
    fn default() -> EFUSEERROR {
        EFUSEERROR(0)
    }
}
impl core::fmt::Debug for EFUSEERROR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSEERROR")
            .field("CODE", &self.CODE())
            .field("DONE", &self.DONE())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EFUSEERROR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EFUSEERROR {{ CODE: {=u8:?}, DONE: {=bool:?}, RESERVED6: {=u32:?} }}",
            self.CODE(),
            self.DONE(),
            self.RESERVED6()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EFUSEFLAG(pub u32);
impl EFUSEFLAG {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_KEY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for EFUSEFLAG {
    #[inline(always)]
    fn default() -> EFUSEFLAG {
        EFUSEFLAG(0)
    }
}
impl core::fmt::Debug for EFUSEFLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSEFLAG")
            .field("KEY", &self.KEY())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EFUSEFLAG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EFUSEFLAG {{ KEY: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.KEY(),
            self.RESERVED1()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EFUSEKEY(pub u32);
impl EFUSEKEY {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CODE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CODE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for EFUSEKEY {
    #[inline(always)]
    fn default() -> EFUSEKEY {
        EFUSEKEY(0)
    }
}
impl core::fmt::Debug for EFUSEKEY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSEKEY")
            .field("CODE", &self.CODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EFUSEKEY {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EFUSEKEY {{ CODE: {=u32:?} }}", self.CODE())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EFUSEPINS(pub u32);
impl EFUSEPINS {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SYS_WS_READ_STATES(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SYS_WS_READ_STATES(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "5:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SYS_REPAIR_EN(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "5:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SYS_REPAIR_EN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "6:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SYS_DIEID_AUTOLOAD_EN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SYS_DIEID_AUTOLOAD_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFC_FCLRZ(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFC_FCLRZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFC_READY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFC_READY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SYS_ECC_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SYS_ECC_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFC_AUTOLOAD_ERROR(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFC_AUTOLOAD_ERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFC_INSTRUCTION_ERROR(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFC_INSTRUCTION_ERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFC_INSTRUCTION_INFO(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFC_INSTRUCTION_INFO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SYS_ECC_SELF_TEST_EN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SYS_ECC_SELF_TEST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFC_SELF_TEST_ERROR(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFC_SELF_TEST_ERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFC_SELF_TEST_DONE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFC_SELF_TEST_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for EFUSEPINS {
    #[inline(always)]
    fn default() -> EFUSEPINS {
        EFUSEPINS(0)
    }
}
impl core::fmt::Debug for EFUSEPINS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSEPINS")
            .field("SYS_WS_READ_STATES", &self.SYS_WS_READ_STATES())
            .field("SYS_REPAIR_EN", &self.SYS_REPAIR_EN())
            .field("SYS_DIEID_AUTOLOAD_EN", &self.SYS_DIEID_AUTOLOAD_EN())
            .field("EFC_FCLRZ", &self.EFC_FCLRZ())
            .field("EFC_READY", &self.EFC_READY())
            .field("SYS_ECC_OVERRIDE_EN", &self.SYS_ECC_OVERRIDE_EN())
            .field("EFC_AUTOLOAD_ERROR", &self.EFC_AUTOLOAD_ERROR())
            .field("EFC_INSTRUCTION_ERROR", &self.EFC_INSTRUCTION_ERROR())
            .field("EFC_INSTRUCTION_INFO", &self.EFC_INSTRUCTION_INFO())
            .field("SYS_ECC_SELF_TEST_EN", &self.SYS_ECC_SELF_TEST_EN())
            .field("EFC_SELF_TEST_ERROR", &self.EFC_SELF_TEST_ERROR())
            .field("EFC_SELF_TEST_DONE", &self.EFC_SELF_TEST_DONE())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EFUSEPINS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EFUSEPINS {{ SYS_WS_READ_STATES: {=u8:?}, SYS_REPAIR_EN: {=u8:?}, SYS_DIEID_AUTOLOAD_EN: {=bool:?}, EFC_FCLRZ: {=bool:?}, EFC_READY: {=bool:?}, SYS_ECC_OVERRIDE_EN: {=bool:?}, EFC_AUTOLOAD_ERROR: {=bool:?}, EFC_INSTRUCTION_ERROR: {=bool:?}, EFC_INSTRUCTION_INFO: {=bool:?}, SYS_ECC_SELF_TEST_EN: {=bool:?}, EFC_SELF_TEST_ERROR: {=bool:?}, EFC_SELF_TEST_DONE: {=bool:?}, RESERVED16: {=u16:?} }}",
            self.SYS_WS_READ_STATES(),
            self.SYS_REPAIR_EN(),
            self.SYS_DIEID_AUTOLOAD_EN(),
            self.EFC_FCLRZ(),
            self.EFC_READY(),
            self.SYS_ECC_OVERRIDE_EN(),
            self.EFC_AUTOLOAD_ERROR(),
            self.EFC_INSTRUCTION_ERROR(),
            self.EFC_INSTRUCTION_INFO(),
            self.SYS_ECC_SELF_TEST_EN(),
            self.EFC_SELF_TEST_ERROR(),
            self.EFC_SELF_TEST_DONE(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EFUSEPROGRAM(pub u32);
impl EFUSEPROGRAM {
    #[doc = "8:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn WRITECLOCK(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "8:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_WRITECLOCK(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "12:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ITERATIONS(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "12:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ITERATIONS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
    }
    #[doc = "13:13\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VPPTOVDD(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VPPTOVDD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "29:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CLOCKSTALL(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0xffff;
        val as u16
    }
    #[doc = "29:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CLOCKSTALL(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 14usize)) | (((val as u32) & 0xffff) << 14usize);
    }
    #[doc = "30:30\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn COMPAREDISABLE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_COMPAREDISABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for EFUSEPROGRAM {
    #[inline(always)]
    fn default() -> EFUSEPROGRAM {
        EFUSEPROGRAM(0)
    }
}
impl core::fmt::Debug for EFUSEPROGRAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSEPROGRAM")
            .field("WRITECLOCK", &self.WRITECLOCK())
            .field("ITERATIONS", &self.ITERATIONS())
            .field("VPPTOVDD", &self.VPPTOVDD())
            .field("CLOCKSTALL", &self.CLOCKSTALL())
            .field("COMPAREDISABLE", &self.COMPAREDISABLE())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EFUSEPROGRAM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EFUSEPROGRAM {{ WRITECLOCK: {=u16:?}, ITERATIONS: {=u8:?}, VPPTOVDD: {=bool:?}, CLOCKSTALL: {=u16:?}, COMPAREDISABLE: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.WRITECLOCK(),
            self.ITERATIONS(),
            self.VPPTOVDD(),
            self.CLOCKSTALL(),
            self.COMPAREDISABLE(),
            self.RESERVED31()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EFUSEREAD(pub u32);
impl EFUSEREAD {
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MARGIN(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MARGIN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SPARE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DEBUG(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DEBUG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn READCLOCK(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_READCLOCK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "9:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DATABIT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "9:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DATABIT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "31:10\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED10(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "31:10\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED10(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for EFUSEREAD {
    #[inline(always)]
    fn default() -> EFUSEREAD {
        EFUSEREAD(0)
    }
}
impl core::fmt::Debug for EFUSEREAD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSEREAD")
            .field("MARGIN", &self.MARGIN())
            .field("SPARE", &self.SPARE())
            .field("DEBUG", &self.DEBUG())
            .field("READCLOCK", &self.READCLOCK())
            .field("DATABIT", &self.DATABIT())
            .field("RESERVED10", &self.RESERVED10())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EFUSEREAD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EFUSEREAD {{ MARGIN: {=u8:?}, SPARE: {=bool:?}, DEBUG: {=bool:?}, READCLOCK: {=u8:?}, DATABIT: {=u8:?}, RESERVED10: {=u32:?} }}",
            self.MARGIN(),
            self.SPARE(),
            self.DEBUG(),
            self.READCLOCK(),
            self.DATABIT(),
            self.RESERVED10()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EFUSERELEASE(pub u32);
impl EFUSERELEASE {
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFUSEDAY(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFUSEDAY(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "8:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFUSEMONTH(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x0f;
        val as u8
    }
    #[doc = "8:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFUSEMONTH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
    }
    #[doc = "15:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFUSEYEAR(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "15:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFUSEYEAR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "20:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ODPDAY(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "20:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ODPDAY(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "24:21\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ODPMONTH(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x0f;
        val as u8
    }
    #[doc = "24:21\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ODPMONTH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
    }
    #[doc = "31:25\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ODPYEAR(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "31:25\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ODPYEAR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for EFUSERELEASE {
    #[inline(always)]
    fn default() -> EFUSERELEASE {
        EFUSERELEASE(0)
    }
}
impl core::fmt::Debug for EFUSERELEASE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSERELEASE")
            .field("EFUSEDAY", &self.EFUSEDAY())
            .field("EFUSEMONTH", &self.EFUSEMONTH())
            .field("EFUSEYEAR", &self.EFUSEYEAR())
            .field("ODPDAY", &self.ODPDAY())
            .field("ODPMONTH", &self.ODPMONTH())
            .field("ODPYEAR", &self.ODPYEAR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EFUSERELEASE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EFUSERELEASE {{ EFUSEDAY: {=u8:?}, EFUSEMONTH: {=u8:?}, EFUSEYEAR: {=u8:?}, ODPDAY: {=u8:?}, ODPMONTH: {=u8:?}, ODPYEAR: {=u8:?} }}",
            self.EFUSEDAY(),
            self.EFUSEMONTH(),
            self.EFUSEYEAR(),
            self.ODPDAY(),
            self.ODPMONTH(),
            self.ODPYEAR()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EFUSESTAT(pub u32);
impl EFUSESTAT {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESETDONE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESETDONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for EFUSESTAT {
    #[inline(always)]
    fn default() -> EFUSESTAT {
        EFUSESTAT(0)
    }
}
impl core::fmt::Debug for EFUSESTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSESTAT")
            .field("RESETDONE", &self.RESETDONE())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EFUSESTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EFUSESTAT {{ RESETDONE: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.RESETDONE(),
            self.RESERVED1()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FADDR(pub u32);
impl FADDR {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FADDR {
    #[inline(always)]
    fn default() -> FADDR {
        FADDR(0)
    }
}
impl core::fmt::Debug for FADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FADDR")
            .field("FADDR", &self.FADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FADDR {{ FADDR: {=u32:?} }}", self.FADDR())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FBAC(pub u32);
impl FBAC {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VREADS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VREADS(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BAGP(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BAGP(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn OTPPROTDIS(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_OTPPROTDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for FBAC {
    #[inline(always)]
    fn default() -> FBAC {
        FBAC(0)
    }
}
impl core::fmt::Debug for FBAC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FBAC")
            .field("VREADS", &self.VREADS())
            .field("BAGP", &self.BAGP())
            .field("OTPPROTDIS", &self.OTPPROTDIS())
            .field("RESERVED17", &self.RESERVED17())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FBAC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FBAC {{ VREADS: {=u8:?}, BAGP: {=u8:?}, OTPPROTDIS: {=bool:?}, RESERVED17: {=u16:?} }}",
            self.VREADS(),
            self.BAGP(),
            self.OTPPROTDIS(),
            self.RESERVED17()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FBBUSY(pub u32);
impl FBBUSY {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for FBBUSY {
    #[inline(always)]
    fn default() -> FBBUSY {
        FBBUSY(0)
    }
}
impl core::fmt::Debug for FBBUSY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FBBUSY")
            .field("BUSY", &self.BUSY())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FBBUSY {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FBBUSY {{ BUSY: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.BUSY(),
            self.RESERVED8()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FBFALLBACK(pub u32);
impl FBFALLBACK {
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BANKPWR0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BANKPWR0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "3:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BANKPWR1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "3:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BANKPWR1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "5:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BANKPWR2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "5:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BANKPWR2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "7:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BANKPWR3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "7:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BANKPWR3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "9:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BANKPWR4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "9:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BANKPWR4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BANKPWR5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "11:10\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BANKPWR5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "13:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BANKPWR6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "13:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BANKPWR6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "15:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BANKPWR7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "15:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BANKPWR7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn REG_PWRSAV(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_REG_PWRSAV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED20(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED20(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_PWRSAV(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_PWRSAV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED28(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED28(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FBFALLBACK {
    #[inline(always)]
    fn default() -> FBFALLBACK {
        FBFALLBACK(0)
    }
}
impl core::fmt::Debug for FBFALLBACK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FBFALLBACK")
            .field("BANKPWR0", &self.BANKPWR0())
            .field("BANKPWR1", &self.BANKPWR1())
            .field("BANKPWR2", &self.BANKPWR2())
            .field("BANKPWR3", &self.BANKPWR3())
            .field("BANKPWR4", &self.BANKPWR4())
            .field("BANKPWR5", &self.BANKPWR5())
            .field("BANKPWR6", &self.BANKPWR6())
            .field("BANKPWR7", &self.BANKPWR7())
            .field("REG_PWRSAV", &self.REG_PWRSAV())
            .field("RESERVED20", &self.RESERVED20())
            .field("FSM_PWRSAV", &self.FSM_PWRSAV())
            .field("RESERVED28", &self.RESERVED28())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FBFALLBACK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FBFALLBACK {{ BANKPWR0: {=u8:?}, BANKPWR1: {=u8:?}, BANKPWR2: {=u8:?}, BANKPWR3: {=u8:?}, BANKPWR4: {=u8:?}, BANKPWR5: {=u8:?}, BANKPWR6: {=u8:?}, BANKPWR7: {=u8:?}, REG_PWRSAV: {=u8:?}, RESERVED20: {=u8:?}, FSM_PWRSAV: {=u8:?}, RESERVED28: {=u8:?} }}",
            self.BANKPWR0(),
            self.BANKPWR1(),
            self.BANKPWR2(),
            self.BANKPWR3(),
            self.BANKPWR4(),
            self.BANKPWR5(),
            self.BANKPWR6(),
            self.BANKPWR7(),
            self.REG_PWRSAV(),
            self.RESERVED20(),
            self.FSM_PWRSAV(),
            self.RESERVED28()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FBMODE(pub u32);
impl FBMODE {
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
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
impl Default for FBMODE {
    #[inline(always)]
    fn default() -> FBMODE {
        FBMODE(0)
    }
}
impl core::fmt::Debug for FBMODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FBMODE")
            .field("MODE", &self.MODE())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FBMODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FBMODE {{ MODE: {=u8:?}, RESERVED3: {=u32:?} }}",
            self.MODE(),
            self.RESERVED3()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FBPRDY(pub u32);
impl FBPRDY {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BANKRDY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BANKRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "14:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x3fff;
        val as u16
    }
    #[doc = "14:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 1usize)) | (((val as u32) & 0x3fff) << 1usize);
    }
    #[doc = "15:15\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PUMPRDY(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PUMPRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BANKBUSY(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BANKBUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for FBPRDY {
    #[inline(always)]
    fn default() -> FBPRDY {
        FBPRDY(0)
    }
}
impl core::fmt::Debug for FBPRDY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FBPRDY")
            .field("BANKRDY", &self.BANKRDY())
            .field("RESERVED1", &self.RESERVED1())
            .field("PUMPRDY", &self.PUMPRDY())
            .field("BANKBUSY", &self.BANKBUSY())
            .field("RESERVED17", &self.RESERVED17())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FBPRDY {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FBPRDY {{ BANKRDY: {=bool:?}, RESERVED1: {=u16:?}, PUMPRDY: {=bool:?}, BANKBUSY: {=bool:?}, RESERVED17: {=u16:?} }}",
            self.BANKRDY(),
            self.RESERVED1(),
            self.PUMPRDY(),
            self.BANKBUSY(),
            self.RESERVED17()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FBPROT(pub u32);
impl FBPROT {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PROTL1DIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PROTL1DIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for FBPROT {
    #[inline(always)]
    fn default() -> FBPROT {
        FBPROT(0)
    }
}
impl core::fmt::Debug for FBPROT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FBPROT")
            .field("PROTL1DIS", &self.PROTL1DIS())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FBPROT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FBPROT {{ PROTL1DIS: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.PROTL1DIS(),
            self.RESERVED1()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FBSE(pub u32);
impl FBSE {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BSE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BSE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FBSE {
    #[inline(always)]
    fn default() -> FBSE {
        FBSE(0)
    }
}
impl core::fmt::Debug for FBSE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FBSE")
            .field("BSE", &self.BSE())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FBSE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FBSE {{ BSE: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.BSE(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FBSTROBES(pub u32);
impl FBSTROBES {
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TEZ(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TEZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn OTP(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_OTP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TI_OTP(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TI_OTP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PRECOL(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PRECOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn NOCOLRED(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_NOCOLRED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CTRLENZ(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CTRLENZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "15:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "15:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FLCLKEN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FLCLKEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RWAIT_FLCLK(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RWAIT_FLCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RWAIT2_FLCLK(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RWAIT2_FLCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "24:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ECBIT(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ECBIT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "31:25\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED25(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "31:25\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for FBSTROBES {
    #[inline(always)]
    fn default() -> FBSTROBES {
        FBSTROBES(0)
    }
}
impl core::fmt::Debug for FBSTROBES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FBSTROBES")
            .field("RESERVED0", &self.RESERVED0())
            .field("TEZ", &self.TEZ())
            .field("OTP", &self.OTP())
            .field("TI_OTP", &self.TI_OTP())
            .field("PRECOL", &self.PRECOL())
            .field("NOCOLRED", &self.NOCOLRED())
            .field("RESERVED7", &self.RESERVED7())
            .field("CTRLENZ", &self.CTRLENZ())
            .field("RESERVED9", &self.RESERVED9())
            .field("FLCLKEN", &self.FLCLKEN())
            .field("RWAIT_FLCLK", &self.RWAIT_FLCLK())
            .field("RWAIT2_FLCLK", &self.RWAIT2_FLCLK())
            .field("RESERVED19", &self.RESERVED19())
            .field("ECBIT", &self.ECBIT())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FBSTROBES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FBSTROBES {{ RESERVED0: {=u8:?}, TEZ: {=bool:?}, OTP: {=bool:?}, TI_OTP: {=bool:?}, PRECOL: {=bool:?}, NOCOLRED: {=bool:?}, RESERVED7: {=bool:?}, CTRLENZ: {=bool:?}, RESERVED9: {=u8:?}, FLCLKEN: {=bool:?}, RWAIT_FLCLK: {=bool:?}, RWAIT2_FLCLK: {=bool:?}, RESERVED19: {=u8:?}, ECBIT: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.RESERVED0(),
            self.TEZ(),
            self.OTP(),
            self.TI_OTP(),
            self.PRECOL(),
            self.NOCOLRED(),
            self.RESERVED7(),
            self.CTRLENZ(),
            self.RESERVED9(),
            self.FLCLKEN(),
            self.RWAIT_FLCLK(),
            self.RWAIT2_FLCLK(),
            self.RESERVED19(),
            self.ECBIT(),
            self.RESERVED25()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B0_SSIZE0(pub u32);
impl FCFG_B0_SSIZE0 {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B0_SECT_SIZE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B0_SECT_SIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "15:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "15:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "27:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B0_NUM_SECTORS(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "27:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B0_NUM_SECTORS(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED28(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED28(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FCFG_B0_SSIZE0 {
    #[inline(always)]
    fn default() -> FCFG_B0_SSIZE0 {
        FCFG_B0_SSIZE0(0)
    }
}
impl core::fmt::Debug for FCFG_B0_SSIZE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B0_SSIZE0")
            .field("B0_SECT_SIZE", &self.B0_SECT_SIZE())
            .field("RESERVED4", &self.RESERVED4())
            .field("B0_NUM_SECTORS", &self.B0_NUM_SECTORS())
            .field("RESERVED28", &self.RESERVED28())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B0_SSIZE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B0_SSIZE0 {{ B0_SECT_SIZE: {=u8:?}, RESERVED4: {=u16:?}, B0_NUM_SECTORS: {=u16:?}, RESERVED28: {=u8:?} }}",
            self.B0_SECT_SIZE(),
            self.RESERVED4(),
            self.B0_NUM_SECTORS(),
            self.RESERVED28()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B0_SSIZE1(pub u32);
impl FCFG_B0_SSIZE1 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B0_SSIZE1 {
    #[inline(always)]
    fn default() -> FCFG_B0_SSIZE1 {
        FCFG_B0_SSIZE1(0)
    }
}
impl core::fmt::Debug for FCFG_B0_SSIZE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B0_SSIZE1")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B0_SSIZE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B0_SSIZE1 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B0_SSIZE2(pub u32);
impl FCFG_B0_SSIZE2 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B0_SSIZE2 {
    #[inline(always)]
    fn default() -> FCFG_B0_SSIZE2 {
        FCFG_B0_SSIZE2(0)
    }
}
impl core::fmt::Debug for FCFG_B0_SSIZE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B0_SSIZE2")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B0_SSIZE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B0_SSIZE2 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B0_SSIZE3(pub u32);
impl FCFG_B0_SSIZE3 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B0_SSIZE3 {
    #[inline(always)]
    fn default() -> FCFG_B0_SSIZE3 {
        FCFG_B0_SSIZE3(0)
    }
}
impl core::fmt::Debug for FCFG_B0_SSIZE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B0_SSIZE3")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B0_SSIZE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B0_SSIZE3 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B0_START(pub u32);
impl FCFG_B0_START {
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B0_START_ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B0_START_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B0_MUX_FACTOR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B0_MUX_FACTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B0_MAX_SECTOR(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B0_MAX_SECTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FCFG_B0_START {
    #[inline(always)]
    fn default() -> FCFG_B0_START {
        FCFG_B0_START(0)
    }
}
impl core::fmt::Debug for FCFG_B0_START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B0_START")
            .field("B0_START_ADDR", &self.B0_START_ADDR())
            .field("B0_MUX_FACTOR", &self.B0_MUX_FACTOR())
            .field("B0_MAX_SECTOR", &self.B0_MAX_SECTOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B0_START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B0_START {{ B0_START_ADDR: {=u32:?}, B0_MUX_FACTOR: {=u8:?}, B0_MAX_SECTOR: {=u8:?} }}",
            self.B0_START_ADDR(),
            self.B0_MUX_FACTOR(),
            self.B0_MAX_SECTOR()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B1_SSIZE0(pub u32);
impl FCFG_B1_SSIZE0 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B1_SECT_SIZE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B1_SECT_SIZE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B1_SSIZE0 {
    #[inline(always)]
    fn default() -> FCFG_B1_SSIZE0 {
        FCFG_B1_SSIZE0(0)
    }
}
impl core::fmt::Debug for FCFG_B1_SSIZE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B1_SSIZE0")
            .field("B1_SECT_SIZE", &self.B1_SECT_SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B1_SSIZE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B1_SSIZE0 {{ B1_SECT_SIZE: {=u32:?} }}",
            self.B1_SECT_SIZE()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B1_SSIZE1(pub u32);
impl FCFG_B1_SSIZE1 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B1_SSIZE1 {
    #[inline(always)]
    fn default() -> FCFG_B1_SSIZE1 {
        FCFG_B1_SSIZE1(0)
    }
}
impl core::fmt::Debug for FCFG_B1_SSIZE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B1_SSIZE1")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B1_SSIZE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B1_SSIZE1 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B1_SSIZE2(pub u32);
impl FCFG_B1_SSIZE2 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B1_SSIZE2 {
    #[inline(always)]
    fn default() -> FCFG_B1_SSIZE2 {
        FCFG_B1_SSIZE2(0)
    }
}
impl core::fmt::Debug for FCFG_B1_SSIZE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B1_SSIZE2")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B1_SSIZE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B1_SSIZE2 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B1_SSIZE3(pub u32);
impl FCFG_B1_SSIZE3 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B1_SSIZE3 {
    #[inline(always)]
    fn default() -> FCFG_B1_SSIZE3 {
        FCFG_B1_SSIZE3(0)
    }
}
impl core::fmt::Debug for FCFG_B1_SSIZE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B1_SSIZE3")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B1_SSIZE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B1_SSIZE3 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B1_START(pub u32);
impl FCFG_B1_START {
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B1_START_ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B1_START_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B1_MUX_FACTOR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B1_MUX_FACTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B1_MAX_SECTOR(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B1_MAX_SECTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FCFG_B1_START {
    #[inline(always)]
    fn default() -> FCFG_B1_START {
        FCFG_B1_START(0)
    }
}
impl core::fmt::Debug for FCFG_B1_START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B1_START")
            .field("B1_START_ADDR", &self.B1_START_ADDR())
            .field("B1_MUX_FACTOR", &self.B1_MUX_FACTOR())
            .field("B1_MAX_SECTOR", &self.B1_MAX_SECTOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B1_START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B1_START {{ B1_START_ADDR: {=u32:?}, B1_MUX_FACTOR: {=u8:?}, B1_MAX_SECTOR: {=u8:?} }}",
            self.B1_START_ADDR(),
            self.B1_MUX_FACTOR(),
            self.B1_MAX_SECTOR()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B2_SSIZE0(pub u32);
impl FCFG_B2_SSIZE0 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B2_SECT_SIZE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B2_SECT_SIZE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B2_SSIZE0 {
    #[inline(always)]
    fn default() -> FCFG_B2_SSIZE0 {
        FCFG_B2_SSIZE0(0)
    }
}
impl core::fmt::Debug for FCFG_B2_SSIZE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B2_SSIZE0")
            .field("B2_SECT_SIZE", &self.B2_SECT_SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B2_SSIZE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B2_SSIZE0 {{ B2_SECT_SIZE: {=u32:?} }}",
            self.B2_SECT_SIZE()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B2_SSIZE1(pub u32);
impl FCFG_B2_SSIZE1 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B2_SSIZE1 {
    #[inline(always)]
    fn default() -> FCFG_B2_SSIZE1 {
        FCFG_B2_SSIZE1(0)
    }
}
impl core::fmt::Debug for FCFG_B2_SSIZE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B2_SSIZE1")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B2_SSIZE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B2_SSIZE1 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B2_SSIZE2(pub u32);
impl FCFG_B2_SSIZE2 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B2_SSIZE2 {
    #[inline(always)]
    fn default() -> FCFG_B2_SSIZE2 {
        FCFG_B2_SSIZE2(0)
    }
}
impl core::fmt::Debug for FCFG_B2_SSIZE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B2_SSIZE2")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B2_SSIZE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B2_SSIZE2 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B2_SSIZE3(pub u32);
impl FCFG_B2_SSIZE3 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B2_SSIZE3 {
    #[inline(always)]
    fn default() -> FCFG_B2_SSIZE3 {
        FCFG_B2_SSIZE3(0)
    }
}
impl core::fmt::Debug for FCFG_B2_SSIZE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B2_SSIZE3")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B2_SSIZE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B2_SSIZE3 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B2_START(pub u32);
impl FCFG_B2_START {
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B2_START_ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B2_START_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B2_MUX_FACTOR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B2_MUX_FACTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B2_MAX_SECTOR(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B2_MAX_SECTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FCFG_B2_START {
    #[inline(always)]
    fn default() -> FCFG_B2_START {
        FCFG_B2_START(0)
    }
}
impl core::fmt::Debug for FCFG_B2_START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B2_START")
            .field("B2_START_ADDR", &self.B2_START_ADDR())
            .field("B2_MUX_FACTOR", &self.B2_MUX_FACTOR())
            .field("B2_MAX_SECTOR", &self.B2_MAX_SECTOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B2_START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B2_START {{ B2_START_ADDR: {=u32:?}, B2_MUX_FACTOR: {=u8:?}, B2_MAX_SECTOR: {=u8:?} }}",
            self.B2_START_ADDR(),
            self.B2_MUX_FACTOR(),
            self.B2_MAX_SECTOR()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B3_SSIZE0(pub u32);
impl FCFG_B3_SSIZE0 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B3_SECT_SIZE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B3_SECT_SIZE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B3_SSIZE0 {
    #[inline(always)]
    fn default() -> FCFG_B3_SSIZE0 {
        FCFG_B3_SSIZE0(0)
    }
}
impl core::fmt::Debug for FCFG_B3_SSIZE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B3_SSIZE0")
            .field("B3_SECT_SIZE", &self.B3_SECT_SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B3_SSIZE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B3_SSIZE0 {{ B3_SECT_SIZE: {=u32:?} }}",
            self.B3_SECT_SIZE()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B3_SSIZE1(pub u32);
impl FCFG_B3_SSIZE1 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B3_SSIZE1 {
    #[inline(always)]
    fn default() -> FCFG_B3_SSIZE1 {
        FCFG_B3_SSIZE1(0)
    }
}
impl core::fmt::Debug for FCFG_B3_SSIZE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B3_SSIZE1")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B3_SSIZE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B3_SSIZE1 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B3_SSIZE2(pub u32);
impl FCFG_B3_SSIZE2 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B3_SSIZE2 {
    #[inline(always)]
    fn default() -> FCFG_B3_SSIZE2 {
        FCFG_B3_SSIZE2(0)
    }
}
impl core::fmt::Debug for FCFG_B3_SSIZE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B3_SSIZE2")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B3_SSIZE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B3_SSIZE2 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B3_SSIZE3(pub u32);
impl FCFG_B3_SSIZE3 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B3_SSIZE3 {
    #[inline(always)]
    fn default() -> FCFG_B3_SSIZE3 {
        FCFG_B3_SSIZE3(0)
    }
}
impl core::fmt::Debug for FCFG_B3_SSIZE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B3_SSIZE3")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B3_SSIZE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B3_SSIZE3 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B3_START(pub u32);
impl FCFG_B3_START {
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B3_START_ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B3_START_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B3_MUX_FACTOR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B3_MUX_FACTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B3_MAX_SECTOR(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B3_MAX_SECTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FCFG_B3_START {
    #[inline(always)]
    fn default() -> FCFG_B3_START {
        FCFG_B3_START(0)
    }
}
impl core::fmt::Debug for FCFG_B3_START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B3_START")
            .field("B3_START_ADDR", &self.B3_START_ADDR())
            .field("B3_MUX_FACTOR", &self.B3_MUX_FACTOR())
            .field("B3_MAX_SECTOR", &self.B3_MAX_SECTOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B3_START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B3_START {{ B3_START_ADDR: {=u32:?}, B3_MUX_FACTOR: {=u8:?}, B3_MAX_SECTOR: {=u8:?} }}",
            self.B3_START_ADDR(),
            self.B3_MUX_FACTOR(),
            self.B3_MAX_SECTOR()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B4_SSIZE0(pub u32);
impl FCFG_B4_SSIZE0 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B4_SECT_SIZE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B4_SECT_SIZE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B4_SSIZE0 {
    #[inline(always)]
    fn default() -> FCFG_B4_SSIZE0 {
        FCFG_B4_SSIZE0(0)
    }
}
impl core::fmt::Debug for FCFG_B4_SSIZE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B4_SSIZE0")
            .field("B4_SECT_SIZE", &self.B4_SECT_SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B4_SSIZE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B4_SSIZE0 {{ B4_SECT_SIZE: {=u32:?} }}",
            self.B4_SECT_SIZE()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B4_SSIZE1(pub u32);
impl FCFG_B4_SSIZE1 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B4_SSIZE1 {
    #[inline(always)]
    fn default() -> FCFG_B4_SSIZE1 {
        FCFG_B4_SSIZE1(0)
    }
}
impl core::fmt::Debug for FCFG_B4_SSIZE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B4_SSIZE1")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B4_SSIZE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B4_SSIZE1 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B4_SSIZE2(pub u32);
impl FCFG_B4_SSIZE2 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B4_SSIZE2 {
    #[inline(always)]
    fn default() -> FCFG_B4_SSIZE2 {
        FCFG_B4_SSIZE2(0)
    }
}
impl core::fmt::Debug for FCFG_B4_SSIZE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B4_SSIZE2")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B4_SSIZE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B4_SSIZE2 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B4_SSIZE3(pub u32);
impl FCFG_B4_SSIZE3 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B4_SSIZE3 {
    #[inline(always)]
    fn default() -> FCFG_B4_SSIZE3 {
        FCFG_B4_SSIZE3(0)
    }
}
impl core::fmt::Debug for FCFG_B4_SSIZE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B4_SSIZE3")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B4_SSIZE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B4_SSIZE3 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B4_START(pub u32);
impl FCFG_B4_START {
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B4_START_ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B4_START_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B4_MUX_FACTOR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B4_MUX_FACTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B4_MAX_SECTOR(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B4_MAX_SECTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FCFG_B4_START {
    #[inline(always)]
    fn default() -> FCFG_B4_START {
        FCFG_B4_START(0)
    }
}
impl core::fmt::Debug for FCFG_B4_START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B4_START")
            .field("B4_START_ADDR", &self.B4_START_ADDR())
            .field("B4_MUX_FACTOR", &self.B4_MUX_FACTOR())
            .field("B4_MAX_SECTOR", &self.B4_MAX_SECTOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B4_START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B4_START {{ B4_START_ADDR: {=u32:?}, B4_MUX_FACTOR: {=u8:?}, B4_MAX_SECTOR: {=u8:?} }}",
            self.B4_START_ADDR(),
            self.B4_MUX_FACTOR(),
            self.B4_MAX_SECTOR()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B5_SSIZE0(pub u32);
impl FCFG_B5_SSIZE0 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B5_SECT_SIZE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B5_SECT_SIZE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B5_SSIZE0 {
    #[inline(always)]
    fn default() -> FCFG_B5_SSIZE0 {
        FCFG_B5_SSIZE0(0)
    }
}
impl core::fmt::Debug for FCFG_B5_SSIZE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B5_SSIZE0")
            .field("B5_SECT_SIZE", &self.B5_SECT_SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B5_SSIZE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B5_SSIZE0 {{ B5_SECT_SIZE: {=u32:?} }}",
            self.B5_SECT_SIZE()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B5_SSIZE1(pub u32);
impl FCFG_B5_SSIZE1 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B5_SSIZE1 {
    #[inline(always)]
    fn default() -> FCFG_B5_SSIZE1 {
        FCFG_B5_SSIZE1(0)
    }
}
impl core::fmt::Debug for FCFG_B5_SSIZE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B5_SSIZE1")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B5_SSIZE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B5_SSIZE1 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B5_SSIZE2(pub u32);
impl FCFG_B5_SSIZE2 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B5_SSIZE2 {
    #[inline(always)]
    fn default() -> FCFG_B5_SSIZE2 {
        FCFG_B5_SSIZE2(0)
    }
}
impl core::fmt::Debug for FCFG_B5_SSIZE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B5_SSIZE2")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B5_SSIZE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B5_SSIZE2 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B5_SSIZE3(pub u32);
impl FCFG_B5_SSIZE3 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B5_SSIZE3 {
    #[inline(always)]
    fn default() -> FCFG_B5_SSIZE3 {
        FCFG_B5_SSIZE3(0)
    }
}
impl core::fmt::Debug for FCFG_B5_SSIZE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B5_SSIZE3")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B5_SSIZE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B5_SSIZE3 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B5_START(pub u32);
impl FCFG_B5_START {
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B5_START_ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B5_START_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B5_MUX_FACTOR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B5_MUX_FACTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B5_MAX_SECTOR(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B5_MAX_SECTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FCFG_B5_START {
    #[inline(always)]
    fn default() -> FCFG_B5_START {
        FCFG_B5_START(0)
    }
}
impl core::fmt::Debug for FCFG_B5_START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B5_START")
            .field("B5_START_ADDR", &self.B5_START_ADDR())
            .field("B5_MUX_FACTOR", &self.B5_MUX_FACTOR())
            .field("B5_MAX_SECTOR", &self.B5_MAX_SECTOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B5_START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B5_START {{ B5_START_ADDR: {=u32:?}, B5_MUX_FACTOR: {=u8:?}, B5_MAX_SECTOR: {=u8:?} }}",
            self.B5_START_ADDR(),
            self.B5_MUX_FACTOR(),
            self.B5_MAX_SECTOR()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B6_SSIZE0(pub u32);
impl FCFG_B6_SSIZE0 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B6_SECT_SIZE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B6_SECT_SIZE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B6_SSIZE0 {
    #[inline(always)]
    fn default() -> FCFG_B6_SSIZE0 {
        FCFG_B6_SSIZE0(0)
    }
}
impl core::fmt::Debug for FCFG_B6_SSIZE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B6_SSIZE0")
            .field("B6_SECT_SIZE", &self.B6_SECT_SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B6_SSIZE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B6_SSIZE0 {{ B6_SECT_SIZE: {=u32:?} }}",
            self.B6_SECT_SIZE()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B6_SSIZE1(pub u32);
impl FCFG_B6_SSIZE1 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B6_SSIZE1 {
    #[inline(always)]
    fn default() -> FCFG_B6_SSIZE1 {
        FCFG_B6_SSIZE1(0)
    }
}
impl core::fmt::Debug for FCFG_B6_SSIZE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B6_SSIZE1")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B6_SSIZE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B6_SSIZE1 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B6_SSIZE2(pub u32);
impl FCFG_B6_SSIZE2 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B6_SSIZE2 {
    #[inline(always)]
    fn default() -> FCFG_B6_SSIZE2 {
        FCFG_B6_SSIZE2(0)
    }
}
impl core::fmt::Debug for FCFG_B6_SSIZE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B6_SSIZE2")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B6_SSIZE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B6_SSIZE2 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B6_SSIZE3(pub u32);
impl FCFG_B6_SSIZE3 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B6_SSIZE3 {
    #[inline(always)]
    fn default() -> FCFG_B6_SSIZE3 {
        FCFG_B6_SSIZE3(0)
    }
}
impl core::fmt::Debug for FCFG_B6_SSIZE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B6_SSIZE3")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B6_SSIZE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B6_SSIZE3 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B6_START(pub u32);
impl FCFG_B6_START {
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B6_START_ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B6_START_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B6_MUX_FACTOR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B6_MUX_FACTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B6_MAX_SECTOR(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B6_MAX_SECTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FCFG_B6_START {
    #[inline(always)]
    fn default() -> FCFG_B6_START {
        FCFG_B6_START(0)
    }
}
impl core::fmt::Debug for FCFG_B6_START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B6_START")
            .field("B6_START_ADDR", &self.B6_START_ADDR())
            .field("B6_MUX_FACTOR", &self.B6_MUX_FACTOR())
            .field("B6_MAX_SECTOR", &self.B6_MAX_SECTOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B6_START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B6_START {{ B6_START_ADDR: {=u32:?}, B6_MUX_FACTOR: {=u8:?}, B6_MAX_SECTOR: {=u8:?} }}",
            self.B6_START_ADDR(),
            self.B6_MUX_FACTOR(),
            self.B6_MAX_SECTOR()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B7_SSIZE0(pub u32);
impl FCFG_B7_SSIZE0 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B7_SECT_SIZE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B7_SECT_SIZE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B7_SSIZE0 {
    #[inline(always)]
    fn default() -> FCFG_B7_SSIZE0 {
        FCFG_B7_SSIZE0(0)
    }
}
impl core::fmt::Debug for FCFG_B7_SSIZE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B7_SSIZE0")
            .field("B7_SECT_SIZE", &self.B7_SECT_SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B7_SSIZE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B7_SSIZE0 {{ B7_SECT_SIZE: {=u32:?} }}",
            self.B7_SECT_SIZE()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B7_SSIZE1(pub u32);
impl FCFG_B7_SSIZE1 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B7_SSIZE1 {
    #[inline(always)]
    fn default() -> FCFG_B7_SSIZE1 {
        FCFG_B7_SSIZE1(0)
    }
}
impl core::fmt::Debug for FCFG_B7_SSIZE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B7_SSIZE1")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B7_SSIZE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B7_SSIZE1 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B7_SSIZE2(pub u32);
impl FCFG_B7_SSIZE2 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B7_SSIZE2 {
    #[inline(always)]
    fn default() -> FCFG_B7_SSIZE2 {
        FCFG_B7_SSIZE2(0)
    }
}
impl core::fmt::Debug for FCFG_B7_SSIZE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B7_SSIZE2")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B7_SSIZE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B7_SSIZE2 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B7_SSIZE3(pub u32);
impl FCFG_B7_SSIZE3 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG_B7_SSIZE3 {
    #[inline(always)]
    fn default() -> FCFG_B7_SSIZE3 {
        FCFG_B7_SSIZE3(0)
    }
}
impl core::fmt::Debug for FCFG_B7_SSIZE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B7_SSIZE3")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B7_SSIZE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B7_SSIZE3 {{ RESERVED: {=u32:?} }}",
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_B7_START(pub u32);
impl FCFG_B7_START {
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B7_START_ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B7_START_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B7_MUX_FACTOR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B7_MUX_FACTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B7_MAX_SECTOR(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B7_MAX_SECTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FCFG_B7_START {
    #[inline(always)]
    fn default() -> FCFG_B7_START {
        FCFG_B7_START(0)
    }
}
impl core::fmt::Debug for FCFG_B7_START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_B7_START")
            .field("B7_START_ADDR", &self.B7_START_ADDR())
            .field("B7_MUX_FACTOR", &self.B7_MUX_FACTOR())
            .field("B7_MAX_SECTOR", &self.B7_MAX_SECTOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_B7_START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_B7_START {{ B7_START_ADDR: {=u32:?}, B7_MUX_FACTOR: {=u8:?}, B7_MAX_SECTOR: {=u8:?} }}",
            self.B7_START_ADDR(),
            self.B7_MUX_FACTOR(),
            self.B7_MAX_SECTOR()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_BANK(pub u32);
impl FCFG_BANK {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MAIN_NUM_BANK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MAIN_NUM_BANK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "15:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MAIN_BANK_WIDTH(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "15:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MAIN_BANK_WIDTH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EE_NUM_BANK(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EE_NUM_BANK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "31:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EE_BANK_WIDTH(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "31:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EE_BANK_WIDTH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for FCFG_BANK {
    #[inline(always)]
    fn default() -> FCFG_BANK {
        FCFG_BANK(0)
    }
}
impl core::fmt::Debug for FCFG_BANK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_BANK")
            .field("MAIN_NUM_BANK", &self.MAIN_NUM_BANK())
            .field("MAIN_BANK_WIDTH", &self.MAIN_BANK_WIDTH())
            .field("EE_NUM_BANK", &self.EE_NUM_BANK())
            .field("EE_BANK_WIDTH", &self.EE_BANK_WIDTH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_BANK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_BANK {{ MAIN_NUM_BANK: {=u8:?}, MAIN_BANK_WIDTH: {=u16:?}, EE_NUM_BANK: {=u8:?}, EE_BANK_WIDTH: {=u16:?} }}",
            self.MAIN_NUM_BANK(),
            self.MAIN_BANK_WIDTH(),
            self.EE_NUM_BANK(),
            self.EE_BANK_WIDTH()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_BNK_TYPE(pub u32);
impl FCFG_BNK_TYPE {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B0_TYPE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B0_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B1_TYPE(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B1_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B2_TYPE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B2_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B3_TYPE(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B3_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B4_TYPE(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B4_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B5_TYPE(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B5_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B6_TYPE(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B6_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn B7_TYPE(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_B7_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FCFG_BNK_TYPE {
    #[inline(always)]
    fn default() -> FCFG_BNK_TYPE {
        FCFG_BNK_TYPE(0)
    }
}
impl core::fmt::Debug for FCFG_BNK_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_BNK_TYPE")
            .field("B0_TYPE", &self.B0_TYPE())
            .field("B1_TYPE", &self.B1_TYPE())
            .field("B2_TYPE", &self.B2_TYPE())
            .field("B3_TYPE", &self.B3_TYPE())
            .field("B4_TYPE", &self.B4_TYPE())
            .field("B5_TYPE", &self.B5_TYPE())
            .field("B6_TYPE", &self.B6_TYPE())
            .field("B7_TYPE", &self.B7_TYPE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_BNK_TYPE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_BNK_TYPE {{ B0_TYPE: {=u8:?}, B1_TYPE: {=u8:?}, B2_TYPE: {=u8:?}, B3_TYPE: {=u8:?}, B4_TYPE: {=u8:?}, B5_TYPE: {=u8:?}, B6_TYPE: {=u8:?}, B7_TYPE: {=u8:?} }}",
            self.B0_TYPE(),
            self.B1_TYPE(),
            self.B2_TYPE(),
            self.B3_TYPE(),
            self.B4_TYPE(),
            self.B5_TYPE(),
            self.B6_TYPE(),
            self.B7_TYPE()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG_WRAPPER(pub u32);
impl FCFG_WRAPPER {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU_TYPE1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CPU_TYPE1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "5:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn UERR(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "5:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_UERR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "7:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTO_SUSP(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "7:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_AUTO_SUSP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ECCA(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ECCA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SIL3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SIL3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFLUSH(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFLUSH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ROM(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ROM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EE_IN_MAIN(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EE_IN_MAIN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CPU2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "20:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MEM_MAP(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MEM_MAP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "23:21\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED21(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "23:21\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED21(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FAMILY_TYPE(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FAMILY_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for FCFG_WRAPPER {
    #[inline(always)]
    fn default() -> FCFG_WRAPPER {
        FCFG_WRAPPER(0)
    }
}
impl core::fmt::Debug for FCFG_WRAPPER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG_WRAPPER")
            .field("CPU_TYPE1", &self.CPU_TYPE1())
            .field("UERR", &self.UERR())
            .field("AUTO_SUSP", &self.AUTO_SUSP())
            .field("ECCA", &self.ECCA())
            .field("SIL3", &self.SIL3())
            .field("IFLUSH", &self.IFLUSH())
            .field("ROM", &self.ROM())
            .field("EE_IN_MAIN", &self.EE_IN_MAIN())
            .field("CPU2", &self.CPU2())
            .field("MEM_MAP", &self.MEM_MAP())
            .field("RESERVED21", &self.RESERVED21())
            .field("FAMILY_TYPE", &self.FAMILY_TYPE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG_WRAPPER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCFG_WRAPPER {{ CPU_TYPE1: {=u8:?}, UERR: {=u8:?}, AUTO_SUSP: {=u8:?}, ECCA: {=bool:?}, SIL3: {=bool:?}, IFLUSH: {=bool:?}, ROM: {=bool:?}, EE_IN_MAIN: {=u8:?}, CPU2: {=u8:?}, MEM_MAP: {=bool:?}, RESERVED21: {=u8:?}, FAMILY_TYPE: {=u8:?} }}",
            self.CPU_TYPE1(),
            self.UERR(),
            self.AUTO_SUSP(),
            self.ECCA(),
            self.SIL3(),
            self.IFLUSH(),
            self.ROM(),
            self.EE_IN_MAIN(),
            self.CPU2(),
            self.MEM_MAP(),
            self.RESERVED21(),
            self.FAMILY_TYPE()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCLKTRIM(pub u32);
impl FCLKTRIM {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM_EN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIM_EN(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCLKTRIM {
    #[inline(always)]
    fn default() -> FCLKTRIM {
        FCLKTRIM(0)
    }
}
impl core::fmt::Debug for FCLKTRIM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCLKTRIM")
            .field("TRIM_EN", &self.TRIM_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCLKTRIM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCLKTRIM {{ TRIM_EN: {=u32:?} }}", self.TRIM_EN())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCOR_ERR_ADD(pub u32);
impl FCOR_ERR_ADD {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FCOR_ERR_ADD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FCOR_ERR_ADD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCOR_ERR_ADD {
    #[inline(always)]
    fn default() -> FCOR_ERR_ADD {
        FCOR_ERR_ADD(0)
    }
}
impl core::fmt::Debug for FCOR_ERR_ADD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCOR_ERR_ADD")
            .field("FCOR_ERR_ADD", &self.FCOR_ERR_ADD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCOR_ERR_ADD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCOR_ERR_ADD {{ FCOR_ERR_ADD: {=u32:?} }}",
            self.FCOR_ERR_ADD()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCOR_ERR_CNT(pub u32);
impl FCOR_ERR_CNT {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn COR_ERR_CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_COR_ERR_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCOR_ERR_CNT {
    #[inline(always)]
    fn default() -> FCOR_ERR_CNT {
        FCOR_ERR_CNT(0)
    }
}
impl core::fmt::Debug for FCOR_ERR_CNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCOR_ERR_CNT")
            .field("COR_ERR_CNT", &self.COR_ERR_CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCOR_ERR_CNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCOR_ERR_CNT {{ COR_ERR_CNT: {=u32:?} }}",
            self.COR_ERR_CNT()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCOR_ERR_POS(pub u32);
impl FCOR_ERR_POS {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SERR_POS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SERR_POS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCOR_ERR_POS {
    #[inline(always)]
    fn default() -> FCOR_ERR_POS {
        FCOR_ERR_POS(0)
    }
}
impl core::fmt::Debug for FCOR_ERR_POS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCOR_ERR_POS")
            .field("SERR_POS", &self.SERR_POS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCOR_ERR_POS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCOR_ERR_POS {{ SERR_POS: {=u32:?} }}", self.SERR_POS())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDIAGCTL(pub u32);
impl FDIAGCTL {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DIAGMODE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DIAGMODE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FDIAGCTL {
    #[inline(always)]
    fn default() -> FDIAGCTL {
        FDIAGCTL(0)
    }
}
impl core::fmt::Debug for FDIAGCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDIAGCTL")
            .field("DIAGMODE", &self.DIAGMODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDIAGCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDIAGCTL {{ DIAGMODE: {=u32:?} }}", self.DIAGMODE())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FEDACCTL1(pub u32);
impl FEDACCTL1 {
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EDACEN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EDACEN(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "24:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SUSP_IGNR(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SUSP_IGNR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "31:25\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED25(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "31:25\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for FEDACCTL1 {
    #[inline(always)]
    fn default() -> FEDACCTL1 {
        FEDACCTL1(0)
    }
}
impl core::fmt::Debug for FEDACCTL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FEDACCTL1")
            .field("EDACEN", &self.EDACEN())
            .field("SUSP_IGNR", &self.SUSP_IGNR())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FEDACCTL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FEDACCTL1 {{ EDACEN: {=u32:?}, SUSP_IGNR: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.EDACEN(),
            self.SUSP_IGNR(),
            self.RESERVED25()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FEDACCTL2(pub u32);
impl FEDACCTL2 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_THRESHOLD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SEC_THRESHOLD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FEDACCTL2 {
    #[inline(always)]
    fn default() -> FEDACCTL2 {
        FEDACCTL2(0)
    }
}
impl core::fmt::Debug for FEDACCTL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FEDACCTL2")
            .field("SEC_THRESHOLD", &self.SEC_THRESHOLD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FEDACCTL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FEDACCTL2 {{ SEC_THRESHOLD: {=u32:?} }}",
            self.SEC_THRESHOLD()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FEDACSDIS(pub u32);
impl FEDACSDIS {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SECTORID0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SECTORID0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FEDACSDIS {
    #[inline(always)]
    fn default() -> FEDACSDIS {
        FEDACSDIS(0)
    }
}
impl core::fmt::Debug for FEDACSDIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FEDACSDIS")
            .field("SECTORID0", &self.SECTORID0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FEDACSDIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FEDACSDIS {{ SECTORID0: {=u32:?} }}", self.SECTORID0())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FEDACSDIS2(pub u32);
impl FEDACSDIS2 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SECTORID2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SECTORID2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FEDACSDIS2 {
    #[inline(always)]
    fn default() -> FEDACSDIS2 {
        FEDACSDIS2(0)
    }
}
impl core::fmt::Debug for FEDACSDIS2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FEDACSDIS2")
            .field("SECTORID2", &self.SECTORID2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FEDACSDIS2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FEDACSDIS2 {{ SECTORID2: {=u32:?} }}", self.SECTORID2())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FEDACSTAT(pub u32);
impl FEDACSTAT {
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR_PRF_FLG(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ERR_PRF_FLG(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "24:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_DONE(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RVF_INT(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RVF_INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "31:26\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED26(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "31:26\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED26(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for FEDACSTAT {
    #[inline(always)]
    fn default() -> FEDACSTAT {
        FEDACSTAT(0)
    }
}
impl core::fmt::Debug for FEDACSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FEDACSTAT")
            .field("ERR_PRF_FLG", &self.ERR_PRF_FLG())
            .field("FSM_DONE", &self.FSM_DONE())
            .field("RVF_INT", &self.RVF_INT())
            .field("RESERVED26", &self.RESERVED26())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FEDACSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FEDACSTAT {{ ERR_PRF_FLG: {=u32:?}, FSM_DONE: {=bool:?}, RVF_INT: {=bool:?}, RESERVED26: {=u8:?} }}",
            self.ERR_PRF_FLG(),
            self.FSM_DONE(),
            self.RVF_INT(),
            self.RESERVED26()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FEFUSECTL(pub u32);
impl FEFUSECTL {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EFUSE_EN(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EFUSE_EN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EF_TEST(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EF_TEST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "7:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "7:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EF_CLRZ(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EF_CLRZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "15:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "15:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BP_SEL(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BP_SEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn WRITE_EN(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_WRITE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "23:18\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED18(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "23:18\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED18(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "26:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CHAIN_SEL(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "26:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CHAIN_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "31:27\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED27(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x1f;
        val as u8
    }
    #[doc = "31:27\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED27(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
    }
}
impl Default for FEFUSECTL {
    #[inline(always)]
    fn default() -> FEFUSECTL {
        FEFUSECTL(0)
    }
}
impl core::fmt::Debug for FEFUSECTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FEFUSECTL")
            .field("EFUSE_EN", &self.EFUSE_EN())
            .field("EF_TEST", &self.EF_TEST())
            .field("RESERVED5", &self.RESERVED5())
            .field("EF_CLRZ", &self.EF_CLRZ())
            .field("RESERVED9", &self.RESERVED9())
            .field("BP_SEL", &self.BP_SEL())
            .field("WRITE_EN", &self.WRITE_EN())
            .field("RESERVED18", &self.RESERVED18())
            .field("CHAIN_SEL", &self.CHAIN_SEL())
            .field("RESERVED27", &self.RESERVED27())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FEFUSECTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FEFUSECTL {{ EFUSE_EN: {=u8:?}, EF_TEST: {=bool:?}, RESERVED5: {=u8:?}, EF_CLRZ: {=bool:?}, RESERVED9: {=u8:?}, BP_SEL: {=bool:?}, WRITE_EN: {=bool:?}, RESERVED18: {=u8:?}, CHAIN_SEL: {=u8:?}, RESERVED27: {=u8:?} }}",
            self.EFUSE_EN(),
            self.EF_TEST(),
            self.RESERVED5(),
            self.EF_CLRZ(),
            self.RESERVED9(),
            self.BP_SEL(),
            self.WRITE_EN(),
            self.RESERVED18(),
            self.CHAIN_SEL(),
            self.RESERVED27()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FEFUSEDATA(pub u32);
impl FEFUSEDATA {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FEFUSEDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FEFUSEDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FEFUSEDATA {
    #[inline(always)]
    fn default() -> FEFUSEDATA {
        FEFUSEDATA(0)
    }
}
impl core::fmt::Debug for FEFUSEDATA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FEFUSEDATA")
            .field("FEFUSEDATA", &self.FEFUSEDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FEFUSEDATA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FEFUSEDATA {{ FEFUSEDATA: {=u32:?} }}",
            self.FEFUSEDATA()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FEFUSESTAT(pub u32);
impl FEFUSESTAT {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SHIFT_DONE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SHIFT_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for FEFUSESTAT {
    #[inline(always)]
    fn default() -> FEFUSESTAT {
        FEFUSESTAT(0)
    }
}
impl core::fmt::Debug for FEFUSESTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FEFUSESTAT")
            .field("SHIFT_DONE", &self.SHIFT_DONE())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FEFUSESTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FEFUSESTAT {{ SHIFT_DONE: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.SHIFT_DONE(),
            self.RESERVED1()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FEMU_ADDR(pub u32);
impl FEMU_ADDR {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EMU_ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EMU_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FEMU_ADDR {
    #[inline(always)]
    fn default() -> FEMU_ADDR {
        FEMU_ADDR(0)
    }
}
impl core::fmt::Debug for FEMU_ADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FEMU_ADDR")
            .field("EMU_ADDR", &self.EMU_ADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FEMU_ADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FEMU_ADDR {{ EMU_ADDR: {=u32:?} }}", self.EMU_ADDR())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FEMU_DLSW(pub u32);
impl FEMU_DLSW {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FEMU_DLSW(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FEMU_DLSW(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FEMU_DLSW {
    #[inline(always)]
    fn default() -> FEMU_DLSW {
        FEMU_DLSW(0)
    }
}
impl core::fmt::Debug for FEMU_DLSW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FEMU_DLSW")
            .field("FEMU_DLSW", &self.FEMU_DLSW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FEMU_DLSW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FEMU_DLSW {{ FEMU_DLSW: {=u32:?} }}", self.FEMU_DLSW())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FEMU_DMSW(pub u32);
impl FEMU_DMSW {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FEMU_DMSW(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FEMU_DMSW(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FEMU_DMSW {
    #[inline(always)]
    fn default() -> FEMU_DMSW {
        FEMU_DMSW(0)
    }
}
impl core::fmt::Debug for FEMU_DMSW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FEMU_DMSW")
            .field("FEMU_DMSW", &self.FEMU_DMSW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FEMU_DMSW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FEMU_DMSW {{ FEMU_DMSW: {=u32:?} }}", self.FEMU_DMSW())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FEMU_ECC(pub u32);
impl FEMU_ECC {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EMU_ECC(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EMU_ECC(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FEMU_ECC {
    #[inline(always)]
    fn default() -> FEMU_ECC {
        FEMU_ECC(0)
    }
}
impl core::fmt::Debug for FEMU_ECC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FEMU_ECC")
            .field("EMU_ECC", &self.EMU_ECC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FEMU_ECC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FEMU_ECC {{ EMU_ECC: {=u32:?} }}", self.EMU_ECC())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_SIZE(pub u32);
impl FLASH_SIZE {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SECTORS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SECTORS(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for FLASH_SIZE {
    #[inline(always)]
    fn default() -> FLASH_SIZE {
        FLASH_SIZE(0)
    }
}
impl core::fmt::Debug for FLASH_SIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_SIZE")
            .field("SECTORS", &self.SECTORS())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_SIZE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASH_SIZE {{ SECTORS: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.SECTORS(),
            self.RESERVED8()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLOCK(pub u32);
impl FLOCK {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ENCOM(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ENCOM(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FLOCK {
    #[inline(always)]
    fn default() -> FLOCK {
        FLOCK(0)
    }
}
impl core::fmt::Debug for FLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLOCK")
            .field("ENCOM", &self.ENCOM())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLOCK {{ ENCOM: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.ENCOM(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FMAC(pub u32);
impl FMAC {
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BANK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BANK(&mut self, val: u8) {
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
impl Default for FMAC {
    #[inline(always)]
    fn default() -> FMAC {
        FMAC(0)
    }
}
impl core::fmt::Debug for FMAC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMAC")
            .field("BANK", &self.BANK())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FMAC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FMAC {{ BANK: {=u8:?}, RESERVED3: {=u32:?} }}",
            self.BANK(),
            self.RESERVED3()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FMC_REV_ID(pub u32);
impl FMC_REV_ID {
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CONFIG_CRC(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CONFIG_CRC(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "31:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MOD_VERSION(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "31:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MOD_VERSION(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FMC_REV_ID {
    #[inline(always)]
    fn default() -> FMC_REV_ID {
        FMC_REV_ID(0)
    }
}
impl core::fmt::Debug for FMC_REV_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMC_REV_ID")
            .field("CONFIG_CRC", &self.CONFIG_CRC())
            .field("MOD_VERSION", &self.MOD_VERSION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FMC_REV_ID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FMC_REV_ID {{ CONFIG_CRC: {=u16:?}, MOD_VERSION: {=u32:?} }}",
            self.CONFIG_CRC(),
            self.MOD_VERSION()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FMSTAT(pub u32);
impl FMSTAT {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SLOCK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SLOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PSUSP(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PSUSP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ESUSP(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ESUSP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VOLSTAT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VOLSTAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CSTAT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CSTAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn INVDAT(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_INVDAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PGM(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PGM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ERS(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ERS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CV(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EV(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PCV(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PCV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PGV(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PGV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DBF(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DBF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ILA(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ILA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RVF(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RVF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RDVER(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RDVER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RVSUSP(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RVSUSP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "31:18\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED18(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x3fff;
        val as u16
    }
    #[doc = "31:18\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED18(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 18usize)) | (((val as u32) & 0x3fff) << 18usize);
    }
}
impl Default for FMSTAT {
    #[inline(always)]
    fn default() -> FMSTAT {
        FMSTAT(0)
    }
}
impl core::fmt::Debug for FMSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMSTAT")
            .field("SLOCK", &self.SLOCK())
            .field("PSUSP", &self.PSUSP())
            .field("ESUSP", &self.ESUSP())
            .field("VOLSTAT", &self.VOLSTAT())
            .field("CSTAT", &self.CSTAT())
            .field("INVDAT", &self.INVDAT())
            .field("PGM", &self.PGM())
            .field("ERS", &self.ERS())
            .field("BUSY", &self.BUSY())
            .field("CV", &self.CV())
            .field("EV", &self.EV())
            .field("PCV", &self.PCV())
            .field("PGV", &self.PGV())
            .field("DBF", &self.DBF())
            .field("ILA", &self.ILA())
            .field("RVF", &self.RVF())
            .field("RDVER", &self.RDVER())
            .field("RVSUSP", &self.RVSUSP())
            .field("RESERVED18", &self.RESERVED18())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FMSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FMSTAT {{ SLOCK: {=bool:?}, PSUSP: {=bool:?}, ESUSP: {=bool:?}, VOLSTAT: {=bool:?}, CSTAT: {=bool:?}, INVDAT: {=bool:?}, PGM: {=bool:?}, ERS: {=bool:?}, BUSY: {=bool:?}, CV: {=bool:?}, EV: {=bool:?}, PCV: {=bool:?}, PGV: {=bool:?}, DBF: {=bool:?}, ILA: {=bool:?}, RVF: {=bool:?}, RDVER: {=bool:?}, RVSUSP: {=bool:?}, RESERVED18: {=u16:?} }}",
            self.SLOCK(),
            self.PSUSP(),
            self.ESUSP(),
            self.VOLSTAT(),
            self.CSTAT(),
            self.INVDAT(),
            self.PGM(),
            self.ERS(),
            self.BUSY(),
            self.CV(),
            self.EV(),
            self.PCV(),
            self.PGV(),
            self.DBF(),
            self.ILA(),
            self.RVF(),
            self.RDVER(),
            self.RVSUSP(),
            self.RESERVED18()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FPAC1(pub u32);
impl FPAC1 {
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PUMPPWR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PUMPPWR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "3:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "3:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "15:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PUMPRESET_PW(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "15:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PUMPRESET_PW(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "27:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PSLEEPTDIS(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "27:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PSLEEPTDIS(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED28(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED28(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FPAC1 {
    #[inline(always)]
    fn default() -> FPAC1 {
        FPAC1(0)
    }
}
impl core::fmt::Debug for FPAC1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPAC1")
            .field("PUMPPWR", &self.PUMPPWR())
            .field("RESERVED1", &self.RESERVED1())
            .field("PUMPRESET_PW", &self.PUMPRESET_PW())
            .field("PSLEEPTDIS", &self.PSLEEPTDIS())
            .field("RESERVED28", &self.RESERVED28())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FPAC1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FPAC1 {{ PUMPPWR: {=u8:?}, RESERVED1: {=u8:?}, PUMPRESET_PW: {=u16:?}, PSLEEPTDIS: {=u16:?}, RESERVED28: {=u8:?} }}",
            self.PUMPPWR(),
            self.RESERVED1(),
            self.PUMPRESET_PW(),
            self.PSLEEPTDIS(),
            self.RESERVED28()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FPAC2(pub u32);
impl FPAC2 {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PAGP(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PAGP(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FPAC2 {
    #[inline(always)]
    fn default() -> FPAC2 {
        FPAC2(0)
    }
}
impl core::fmt::Debug for FPAC2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPAC2")
            .field("PAGP", &self.PAGP())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FPAC2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FPAC2 {{ PAGP: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.PAGP(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FPAR_OVR(pub u32);
impl FPAR_OVR {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DAT_INV_PAR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DAT_INV_PAR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FPAR_OVR {
    #[inline(always)]
    fn default() -> FPAR_OVR {
        FPAR_OVR(0)
    }
}
impl core::fmt::Debug for FPAR_OVR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPAR_OVR")
            .field("DAT_INV_PAR", &self.DAT_INV_PAR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FPAR_OVR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FPAR_OVR {{ DAT_INV_PAR: {=u32:?} }}",
            self.DAT_INV_PAR()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FPMTCTL(pub u32);
impl FPMTCTL {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR_INCR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ADDR_INCR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FPMTCTL {
    #[inline(always)]
    fn default() -> FPMTCTL {
        FPMTCTL(0)
    }
}
impl core::fmt::Debug for FPMTCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPMTCTL")
            .field("ADDR_INCR", &self.ADDR_INCR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FPMTCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FPMTCTL {{ ADDR_INCR: {=u32:?} }}", self.ADDR_INCR())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FPRIM_ADD_TAG(pub u32);
impl FPRIM_ADD_TAG {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIM_ADD_TAG(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PRIM_ADD_TAG(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FPRIM_ADD_TAG {
    #[inline(always)]
    fn default() -> FPRIM_ADD_TAG {
        FPRIM_ADD_TAG(0)
    }
}
impl core::fmt::Debug for FPRIM_ADD_TAG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPRIM_ADD_TAG")
            .field("PRIM_ADD_TAG", &self.PRIM_ADD_TAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FPRIM_ADD_TAG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FPRIM_ADD_TAG {{ PRIM_ADD_TAG: {=u32:?} }}",
            self.PRIM_ADD_TAG()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FPSTROBES(pub u32);
impl FPSTROBES {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn V5PWRDNZ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_V5PWRDNZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn V3PWRDNZ(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_V3PWRDNZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "7:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "7:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EXECUTEZ(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EXECUTEZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "31:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "31:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for FPSTROBES {
    #[inline(always)]
    fn default() -> FPSTROBES {
        FPSTROBES(0)
    }
}
impl core::fmt::Debug for FPSTROBES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FPSTROBES")
            .field("V5PWRDNZ", &self.V5PWRDNZ())
            .field("V3PWRDNZ", &self.V3PWRDNZ())
            .field("RESERVED2", &self.RESERVED2())
            .field("EXECUTEZ", &self.EXECUTEZ())
            .field("RESERVED9", &self.RESERVED9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FPSTROBES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FPSTROBES {{ V5PWRDNZ: {=bool:?}, V3PWRDNZ: {=bool:?}, RESERVED2: {=u8:?}, EXECUTEZ: {=bool:?}, RESERVED9: {=u32:?} }}",
            self.V5PWRDNZ(),
            self.V3PWRDNZ(),
            self.RESERVED2(),
            self.EXECUTEZ(),
            self.RESERVED9()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FRAW_DATAH(pub u32);
impl FRAW_DATAH {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FRAW_DATAH(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FRAW_DATAH(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FRAW_DATAH {
    #[inline(always)]
    fn default() -> FRAW_DATAH {
        FRAW_DATAH(0)
    }
}
impl core::fmt::Debug for FRAW_DATAH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAW_DATAH")
            .field("FRAW_DATAH", &self.FRAW_DATAH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FRAW_DATAH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FRAW_DATAH {{ FRAW_DATAH: {=u32:?} }}",
            self.FRAW_DATAH()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FRAW_DATAL(pub u32);
impl FRAW_DATAL {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FRAW_DATAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FRAW_DATAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FRAW_DATAL {
    #[inline(always)]
    fn default() -> FRAW_DATAL {
        FRAW_DATAL(0)
    }
}
impl core::fmt::Debug for FRAW_DATAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAW_DATAL")
            .field("FRAW_DATAL", &self.FRAW_DATAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FRAW_DATAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FRAW_DATAL {{ FRAW_DATAL: {=u32:?} }}",
            self.FRAW_DATAL()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FRAW_ECC(pub u32);
impl FRAW_ECC {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RAW_ECC(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RAW_ECC(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FRAW_ECC {
    #[inline(always)]
    fn default() -> FRAW_ECC {
        FRAW_ECC(0)
    }
}
impl core::fmt::Debug for FRAW_ECC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAW_ECC")
            .field("RAW_ECC", &self.RAW_ECC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FRAW_ECC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FRAW_ECC {{ RAW_ECC: {=u32:?} }}", self.RAW_ECC())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FRDCTL(pub u32);
impl FRDCTL {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RWAIT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RWAIT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "31:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "31:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FRDCTL {
    #[inline(always)]
    fn default() -> FRDCTL {
        FRDCTL(0)
    }
}
impl core::fmt::Debug for FRDCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRDCTL")
            .field("RM", &self.RM())
            .field("RWAIT", &self.RWAIT())
            .field("RESERVED12", &self.RESERVED12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FRDCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FRDCTL {{ RM: {=u8:?}, RWAIT: {=u8:?}, RESERVED12: {=u32:?} }}",
            self.RM(),
            self.RWAIT(),
            self.RESERVED12()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FREDU_ADD_TAG(pub u32);
impl FREDU_ADD_TAG {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn REDU_ADD_TAG(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_REDU_ADD_TAG(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FREDU_ADD_TAG {
    #[inline(always)]
    fn default() -> FREDU_ADD_TAG {
        FREDU_ADD_TAG(0)
    }
}
impl core::fmt::Debug for FREDU_ADD_TAG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FREDU_ADD_TAG")
            .field("REDU_ADD_TAG", &self.REDU_ADD_TAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FREDU_ADD_TAG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FREDU_ADD_TAG {{ REDU_ADD_TAG: {=u32:?} }}",
            self.REDU_ADD_TAG()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSEQPMP(pub u32);
impl FSEQPMP {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SEQ_PUMP(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SEQ_PUMP(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VIN_BY_PASS(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VIN_BY_PASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "11:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "11:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "14:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VIN_AT_X(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "14:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VIN_AT_X(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "15:15\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM_0P8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIM_0P8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "21:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM_1P7(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "21:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIM_1P7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "23:22\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED22(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "23:22\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED22(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM_3P4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIM_3P4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED28(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED28(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FSEQPMP {
    #[inline(always)]
    fn default() -> FSEQPMP {
        FSEQPMP(0)
    }
}
impl core::fmt::Debug for FSEQPMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSEQPMP")
            .field("SEQ_PUMP", &self.SEQ_PUMP())
            .field("VIN_BY_PASS", &self.VIN_BY_PASS())
            .field("RESERVED9", &self.RESERVED9())
            .field("VIN_AT_X", &self.VIN_AT_X())
            .field("RESERVED15", &self.RESERVED15())
            .field("TRIM_0P8", &self.TRIM_0P8())
            .field("TRIM_1P7", &self.TRIM_1P7())
            .field("RESERVED22", &self.RESERVED22())
            .field("TRIM_3P4", &self.TRIM_3P4())
            .field("RESERVED28", &self.RESERVED28())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSEQPMP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSEQPMP {{ SEQ_PUMP: {=u8:?}, VIN_BY_PASS: {=bool:?}, RESERVED9: {=u8:?}, VIN_AT_X: {=u8:?}, RESERVED15: {=bool:?}, TRIM_0P8: {=u8:?}, TRIM_1P7: {=u8:?}, RESERVED22: {=u8:?}, TRIM_3P4: {=u8:?}, RESERVED28: {=u8:?} }}",
            self.SEQ_PUMP(),
            self.VIN_BY_PASS(),
            self.RESERVED9(),
            self.VIN_AT_X(),
            self.RESERVED15(),
            self.TRIM_0P8(),
            self.TRIM_1P7(),
            self.RESERVED22(),
            self.TRIM_3P4(),
            self.RESERVED28()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_ACC_EP(pub u32);
impl FSM_ACC_EP {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ACC_EP(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ACC_EP(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FSM_ACC_EP {
    #[inline(always)]
    fn default() -> FSM_ACC_EP {
        FSM_ACC_EP(0)
    }
}
impl core::fmt::Debug for FSM_ACC_EP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_ACC_EP")
            .field("ACC_EP", &self.ACC_EP())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_ACC_EP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_ACC_EP {{ ACC_EP: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.ACC_EP(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_ACC_PP(pub u32);
impl FSM_ACC_PP {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_ACC_PP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_ACC_PP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FSM_ACC_PP {
    #[inline(always)]
    fn default() -> FSM_ACC_PP {
        FSM_ACC_PP(0)
    }
}
impl core::fmt::Debug for FSM_ACC_PP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_ACC_PP")
            .field("FSM_ACC_PP", &self.FSM_ACC_PP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_ACC_PP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_ACC_PP {{ FSM_ACC_PP: {=u32:?} }}",
            self.FSM_ACC_PP()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_ADDR(pub u32);
impl FSM_ADDR {
    #[doc = "27:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CUR_ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "27:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CUR_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "30:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BANK(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "30:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BANK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
    #[doc = "31:31\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for FSM_ADDR {
    #[inline(always)]
    fn default() -> FSM_ADDR {
        FSM_ADDR(0)
    }
}
impl core::fmt::Debug for FSM_ADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_ADDR")
            .field("CUR_ADDR", &self.CUR_ADDR())
            .field("BANK", &self.BANK())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_ADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_ADDR {{ CUR_ADDR: {=u32:?}, BANK: {=u8:?}, RESERVED31: {=bool:?} }}",
            self.CUR_ADDR(),
            self.BANK(),
            self.RESERVED31()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_BSLE0(pub u32);
impl FSM_BSLE0 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_BSLE0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_BSLE0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FSM_BSLE0 {
    #[inline(always)]
    fn default() -> FSM_BSLE0 {
        FSM_BSLE0(0)
    }
}
impl core::fmt::Debug for FSM_BSLE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_BSLE0")
            .field("FSM_BSLE0", &self.FSM_BSLE0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_BSLE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FSM_BSLE0 {{ FSM_BSLE0: {=u32:?} }}", self.FSM_BSLE0())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_BSLE1(pub u32);
impl FSM_BSLE1 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_BSL1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_BSL1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FSM_BSLE1 {
    #[inline(always)]
    fn default() -> FSM_BSLE1 {
        FSM_BSLE1(0)
    }
}
impl core::fmt::Debug for FSM_BSLE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_BSLE1")
            .field("FSM_BSL1", &self.FSM_BSL1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_BSLE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FSM_BSLE1 {{ FSM_BSL1: {=u32:?} }}", self.FSM_BSL1())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_BSLP0(pub u32);
impl FSM_BSLP0 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_BSLP0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_BSLP0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FSM_BSLP0 {
    #[inline(always)]
    fn default() -> FSM_BSLP0 {
        FSM_BSLP0(0)
    }
}
impl core::fmt::Debug for FSM_BSLP0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_BSLP0")
            .field("FSM_BSLP0", &self.FSM_BSLP0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_BSLP0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FSM_BSLP0 {{ FSM_BSLP0: {=u32:?} }}", self.FSM_BSLP0())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_BSLP1(pub u32);
impl FSM_BSLP1 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_BSL1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_BSL1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FSM_BSLP1 {
    #[inline(always)]
    fn default() -> FSM_BSLP1 {
        FSM_BSLP1(0)
    }
}
impl core::fmt::Debug for FSM_BSLP1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_BSLP1")
            .field("FSM_BSL1", &self.FSM_BSL1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_BSLP1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FSM_BSLP1 {{ FSM_BSL1: {=u32:?} }}", self.FSM_BSL1())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_CMD(pub u32);
impl FSM_CMD {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSMCMD(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSMCMD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "31:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "31:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for FSM_CMD {
    #[inline(always)]
    fn default() -> FSM_CMD {
        FSM_CMD(0)
    }
}
impl core::fmt::Debug for FSM_CMD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_CMD")
            .field("FSMCMD", &self.FSMCMD())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_CMD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_CMD {{ FSMCMD: {=u8:?}, RESERVED6: {=u32:?} }}",
            self.FSMCMD(),
            self.RESERVED6()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_CMP_VSU(pub u32);
impl FSM_CMP_VSU {
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ADD_EXZ(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ADD_EXZ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FSM_CMP_VSU {
    #[inline(always)]
    fn default() -> FSM_CMP_VSU {
        FSM_CMP_VSU(0)
    }
}
impl core::fmt::Debug for FSM_CMP_VSU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_CMP_VSU")
            .field("RESERVED0", &self.RESERVED0())
            .field("ADD_EXZ", &self.ADD_EXZ())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_CMP_VSU {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_CMP_VSU {{ RESERVED0: {=u16:?}, ADD_EXZ: {=u8:?}, RESERVED16: {=u16:?} }}",
            self.RESERVED0(),
            self.ADD_EXZ(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_EC_STEP_HEIGHT(pub u32);
impl FSM_EC_STEP_HEIGHT {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EC_STEP_HEIGHT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EC_STEP_HEIGHT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "31:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "31:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for FSM_EC_STEP_HEIGHT {
    #[inline(always)]
    fn default() -> FSM_EC_STEP_HEIGHT {
        FSM_EC_STEP_HEIGHT(0)
    }
}
impl core::fmt::Debug for FSM_EC_STEP_HEIGHT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_EC_STEP_HEIGHT")
            .field("EC_STEP_HEIGHT", &self.EC_STEP_HEIGHT())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_EC_STEP_HEIGHT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_EC_STEP_HEIGHT {{ EC_STEP_HEIGHT: {=u8:?}, RESERVED4: {=u32:?} }}",
            self.EC_STEP_HEIGHT(),
            self.RESERVED4()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_ERA(pub u32);
impl FSM_ERA {
    #[doc = "22:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ERA_ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "22:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ERA_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "25:23\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ERA_BANK(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x07;
        val as u8
    }
    #[doc = "25:23\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ERA_BANK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
    }
    #[doc = "31:26\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED26(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "31:26\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED26(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for FSM_ERA {
    #[inline(always)]
    fn default() -> FSM_ERA {
        FSM_ERA(0)
    }
}
impl core::fmt::Debug for FSM_ERA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_ERA")
            .field("ERA_ADDR", &self.ERA_ADDR())
            .field("ERA_BANK", &self.ERA_BANK())
            .field("RESERVED26", &self.RESERVED26())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_ERA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_ERA {{ ERA_ADDR: {=u32:?}, ERA_BANK: {=u8:?}, RESERVED26: {=u8:?} }}",
            self.ERA_ADDR(),
            self.ERA_BANK(),
            self.RESERVED26()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_ERA_OH(pub u32);
impl FSM_ERA_OH {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ERA_OH(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ERA_OH(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FSM_ERA_OH {
    #[inline(always)]
    fn default() -> FSM_ERA_OH {
        FSM_ERA_OH(0)
    }
}
impl core::fmt::Debug for FSM_ERA_OH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_ERA_OH")
            .field("ERA_OH", &self.ERA_OH())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_ERA_OH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_ERA_OH {{ ERA_OH: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.ERA_OH(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_ERA_PUL(pub u32);
impl FSM_ERA_PUL {
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MAX_ERA_PUL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MAX_ERA_PUL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MAX_EC_LEVEL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MAX_EC_LEVEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "31:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED20(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "31:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED20(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for FSM_ERA_PUL {
    #[inline(always)]
    fn default() -> FSM_ERA_PUL {
        FSM_ERA_PUL(0)
    }
}
impl core::fmt::Debug for FSM_ERA_PUL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_ERA_PUL")
            .field("MAX_ERA_PUL", &self.MAX_ERA_PUL())
            .field("RESERVED12", &self.RESERVED12())
            .field("MAX_EC_LEVEL", &self.MAX_EC_LEVEL())
            .field("RESERVED20", &self.RESERVED20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_ERA_PUL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_ERA_PUL {{ MAX_ERA_PUL: {=u16:?}, RESERVED12: {=u8:?}, MAX_EC_LEVEL: {=u8:?}, RESERVED20: {=u16:?} }}",
            self.MAX_ERA_PUL(),
            self.RESERVED12(),
            self.MAX_EC_LEVEL(),
            self.RESERVED20()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_ERA_PW(pub u32);
impl FSM_ERA_PW {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_ERA_PW(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_ERA_PW(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FSM_ERA_PW {
    #[inline(always)]
    fn default() -> FSM_ERA_PW {
        FSM_ERA_PW(0)
    }
}
impl core::fmt::Debug for FSM_ERA_PW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_ERA_PW")
            .field("FSM_ERA_PW", &self.FSM_ERA_PW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_ERA_PW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_ERA_PW {{ FSM_ERA_PW: {=u32:?} }}",
            self.FSM_ERA_PW()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_ERR_ADDR(pub u32);
impl FSM_ERR_ADDR {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_ERR_BANK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_ERR_BANK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "31:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_ERR_ADDR(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_ERR_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for FSM_ERR_ADDR {
    #[inline(always)]
    fn default() -> FSM_ERR_ADDR {
        FSM_ERR_ADDR(0)
    }
}
impl core::fmt::Debug for FSM_ERR_ADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_ERR_ADDR")
            .field("FSM_ERR_BANK", &self.FSM_ERR_BANK())
            .field("RESERVED4", &self.RESERVED4())
            .field("FSM_ERR_ADDR", &self.FSM_ERR_ADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_ERR_ADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_ERR_ADDR {{ FSM_ERR_BANK: {=u8:?}, RESERVED4: {=u8:?}, FSM_ERR_ADDR: {=u32:?} }}",
            self.FSM_ERR_BANK(),
            self.RESERVED4(),
            self.FSM_ERR_ADDR()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_EXECUTE(pub u32);
impl FSM_EXECUTE {
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSMEXECUTE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSMEXECUTE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "15:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u16 {
        let val = (self.0 >> 5usize) & 0x07ff;
        val as u16
    }
    #[doc = "15:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 5usize)) | (((val as u32) & 0x07ff) << 5usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SUSPEND_NOW(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SUSPEND_NOW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "31:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED20(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "31:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED20(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for FSM_EXECUTE {
    #[inline(always)]
    fn default() -> FSM_EXECUTE {
        FSM_EXECUTE(0)
    }
}
impl core::fmt::Debug for FSM_EXECUTE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_EXECUTE")
            .field("FSMEXECUTE", &self.FSMEXECUTE())
            .field("RESERVED5", &self.RESERVED5())
            .field("SUSPEND_NOW", &self.SUSPEND_NOW())
            .field("RESERVED20", &self.RESERVED20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_EXECUTE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_EXECUTE {{ FSMEXECUTE: {=u8:?}, RESERVED5: {=u16:?}, SUSPEND_NOW: {=u8:?}, RESERVED20: {=u16:?} }}",
            self.FSMEXECUTE(),
            self.RESERVED5(),
            self.SUSPEND_NOW(),
            self.RESERVED20()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_EX_VAL(pub u32);
impl FSM_EX_VAL {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EXE_VALD(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EXE_VALD(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn REP_VSU(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_REP_VSU(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FSM_EX_VAL {
    #[inline(always)]
    fn default() -> FSM_EX_VAL {
        FSM_EX_VAL(0)
    }
}
impl core::fmt::Debug for FSM_EX_VAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_EX_VAL")
            .field("EXE_VALD", &self.EXE_VALD())
            .field("REP_VSU", &self.REP_VSU())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_EX_VAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_EX_VAL {{ EXE_VALD: {=u8:?}, REP_VSU: {=u8:?}, RESERVED16: {=u16:?} }}",
            self.EXE_VALD(),
            self.REP_VSU(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_FLES(pub u32);
impl FSM_FLES {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BLK_OTP(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BLK_OTP(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BLK_TIOTP(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BLK_TIOTP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "31:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "31:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FSM_FLES {
    #[inline(always)]
    fn default() -> FSM_FLES {
        FSM_FLES(0)
    }
}
impl core::fmt::Debug for FSM_FLES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_FLES")
            .field("BLK_OTP", &self.BLK_OTP())
            .field("BLK_TIOTP", &self.BLK_TIOTP())
            .field("RESERVED12", &self.RESERVED12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_FLES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_FLES {{ BLK_OTP: {=u8:?}, BLK_TIOTP: {=u8:?}, RESERVED12: {=u32:?} }}",
            self.BLK_OTP(),
            self.BLK_TIOTP(),
            self.RESERVED12()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_GLBCTL(pub u32);
impl FSM_GLBCTL {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKSEL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CLKSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for FSM_GLBCTL {
    #[inline(always)]
    fn default() -> FSM_GLBCTL {
        FSM_GLBCTL(0)
    }
}
impl core::fmt::Debug for FSM_GLBCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_GLBCTL")
            .field("CLKSEL", &self.CLKSEL())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_GLBCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_GLBCTL {{ CLKSEL: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.CLKSEL(),
            self.RESERVED1()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_MODE(pub u32);
impl FSM_MODE {
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CMD(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CMD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "5:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "5:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SAV_ERA_MODE(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SAV_ERA_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "11:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SAV_PGM_CMD(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "11:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SAV_PGM_CMD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "13:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SUBMODE(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "13:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SUBMODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "15:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ERA_SUBMODE(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "15:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ERA_SUBMODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "17:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PGM_SUBMODE(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "17:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PGM_SUBMODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "19:18\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RDV_SUBMODE(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "19:18\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RDV_SUBMODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "31:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED20(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "31:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED20(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for FSM_MODE {
    #[inline(always)]
    fn default() -> FSM_MODE {
        FSM_MODE(0)
    }
}
impl core::fmt::Debug for FSM_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_MODE")
            .field("CMD", &self.CMD())
            .field("MODE", &self.MODE())
            .field("SAV_ERA_MODE", &self.SAV_ERA_MODE())
            .field("SAV_PGM_CMD", &self.SAV_PGM_CMD())
            .field("SUBMODE", &self.SUBMODE())
            .field("ERA_SUBMODE", &self.ERA_SUBMODE())
            .field("PGM_SUBMODE", &self.PGM_SUBMODE())
            .field("RDV_SUBMODE", &self.RDV_SUBMODE())
            .field("RESERVED20", &self.RESERVED20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_MODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_MODE {{ CMD: {=u8:?}, MODE: {=u8:?}, SAV_ERA_MODE: {=u8:?}, SAV_PGM_CMD: {=u8:?}, SUBMODE: {=u8:?}, ERA_SUBMODE: {=u8:?}, PGM_SUBMODE: {=u8:?}, RDV_SUBMODE: {=u8:?}, RESERVED20: {=u16:?} }}",
            self.CMD(),
            self.MODE(),
            self.SAV_ERA_MODE(),
            self.SAV_PGM_CMD(),
            self.SUBMODE(),
            self.ERA_SUBMODE(),
            self.PGM_SUBMODE(),
            self.RDV_SUBMODE(),
            self.RESERVED20()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_PE_OSU(pub u32);
impl FSM_PE_OSU {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ERA_OSU(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ERA_OSU(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PGM_OSU(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PGM_OSU(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FSM_PE_OSU {
    #[inline(always)]
    fn default() -> FSM_PE_OSU {
        FSM_PE_OSU(0)
    }
}
impl core::fmt::Debug for FSM_PE_OSU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_PE_OSU")
            .field("ERA_OSU", &self.ERA_OSU())
            .field("PGM_OSU", &self.PGM_OSU())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_PE_OSU {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_PE_OSU {{ ERA_OSU: {=u8:?}, PGM_OSU: {=u8:?}, RESERVED16: {=u16:?} }}",
            self.ERA_OSU(),
            self.PGM_OSU(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_PE_VH(pub u32);
impl FSM_PE_VH {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ERA_VH(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ERA_VH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PGM_VH(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PGM_VH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FSM_PE_VH {
    #[inline(always)]
    fn default() -> FSM_PE_VH {
        FSM_PE_VH(0)
    }
}
impl core::fmt::Debug for FSM_PE_VH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_PE_VH")
            .field("ERA_VH", &self.ERA_VH())
            .field("PGM_VH", &self.PGM_VH())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_PE_VH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_PE_VH {{ ERA_VH: {=u8:?}, PGM_VH: {=u8:?}, RESERVED16: {=u16:?} }}",
            self.ERA_VH(),
            self.PGM_VH(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_PE_VSU(pub u32);
impl FSM_PE_VSU {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ERA_VSU(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ERA_VSU(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PGM_VSU(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PGM_VSU(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FSM_PE_VSU {
    #[inline(always)]
    fn default() -> FSM_PE_VSU {
        FSM_PE_VSU(0)
    }
}
impl core::fmt::Debug for FSM_PE_VSU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_PE_VSU")
            .field("ERA_VSU", &self.ERA_VSU())
            .field("PGM_VSU", &self.PGM_VSU())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_PE_VSU {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_PE_VSU {{ ERA_VSU: {=u8:?}, PGM_VSU: {=u8:?}, RESERVED16: {=u16:?} }}",
            self.ERA_VSU(),
            self.PGM_VSU(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_PGM(pub u32);
impl FSM_PGM {
    #[doc = "22:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PGM_ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "22:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PGM_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "25:23\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PGM_BANK(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x07;
        val as u8
    }
    #[doc = "25:23\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PGM_BANK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
    }
    #[doc = "31:26\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED26(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "31:26\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED26(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for FSM_PGM {
    #[inline(always)]
    fn default() -> FSM_PGM {
        FSM_PGM(0)
    }
}
impl core::fmt::Debug for FSM_PGM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_PGM")
            .field("PGM_ADDR", &self.PGM_ADDR())
            .field("PGM_BANK", &self.PGM_BANK())
            .field("RESERVED26", &self.RESERVED26())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_PGM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_PGM {{ PGM_ADDR: {=u32:?}, PGM_BANK: {=u8:?}, RESERVED26: {=u8:?} }}",
            self.PGM_ADDR(),
            self.PGM_BANK(),
            self.RESERVED26()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_PGM_MAXPUL(pub u32);
impl FSM_PGM_MAXPUL {
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_PGM_MAXPUL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_PGM_MAXPUL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "31:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "31:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FSM_PGM_MAXPUL {
    #[inline(always)]
    fn default() -> FSM_PGM_MAXPUL {
        FSM_PGM_MAXPUL(0)
    }
}
impl core::fmt::Debug for FSM_PGM_MAXPUL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_PGM_MAXPUL")
            .field("FSM_PGM_MAXPUL", &self.FSM_PGM_MAXPUL())
            .field("RESERVED12", &self.RESERVED12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_PGM_MAXPUL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_PGM_MAXPUL {{ FSM_PGM_MAXPUL: {=u16:?}, RESERVED12: {=u32:?} }}",
            self.FSM_PGM_MAXPUL(),
            self.RESERVED12()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_PRG_PUL(pub u32);
impl FSM_PRG_PUL {
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MAX_PRG_PUL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MAX_PRG_PUL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BEG_EC_LEVEL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BEG_EC_LEVEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "31:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED20(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "31:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED20(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for FSM_PRG_PUL {
    #[inline(always)]
    fn default() -> FSM_PRG_PUL {
        FSM_PRG_PUL(0)
    }
}
impl core::fmt::Debug for FSM_PRG_PUL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_PRG_PUL")
            .field("MAX_PRG_PUL", &self.MAX_PRG_PUL())
            .field("RESERVED12", &self.RESERVED12())
            .field("BEG_EC_LEVEL", &self.BEG_EC_LEVEL())
            .field("RESERVED20", &self.RESERVED20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_PRG_PUL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_PRG_PUL {{ MAX_PRG_PUL: {=u16:?}, RESERVED12: {=u8:?}, BEG_EC_LEVEL: {=u8:?}, RESERVED20: {=u16:?} }}",
            self.MAX_PRG_PUL(),
            self.RESERVED12(),
            self.BEG_EC_LEVEL(),
            self.RESERVED20()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_PRG_PW(pub u32);
impl FSM_PRG_PW {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PROG_PUL_WIDTH(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PROG_PUL_WIDTH(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FSM_PRG_PW {
    #[inline(always)]
    fn default() -> FSM_PRG_PW {
        FSM_PRG_PW(0)
    }
}
impl core::fmt::Debug for FSM_PRG_PW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_PRG_PW")
            .field("PROG_PUL_WIDTH", &self.PROG_PUL_WIDTH())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_PRG_PW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_PRG_PW {{ PROG_PUL_WIDTH: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.PROG_PUL_WIDTH(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_PUL_CNTR(pub u32);
impl FSM_PUL_CNTR {
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PUL_CNTR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PUL_CNTR(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "24:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CUR_EC_LEVEL(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "24:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CUR_EC_LEVEL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "31:25\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED25(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "31:25\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for FSM_PUL_CNTR {
    #[inline(always)]
    fn default() -> FSM_PUL_CNTR {
        FSM_PUL_CNTR(0)
    }
}
impl core::fmt::Debug for FSM_PUL_CNTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_PUL_CNTR")
            .field("PUL_CNTR", &self.PUL_CNTR())
            .field("RESERVED12", &self.RESERVED12())
            .field("CUR_EC_LEVEL", &self.CUR_EC_LEVEL())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_PUL_CNTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_PUL_CNTR {{ PUL_CNTR: {=u16:?}, RESERVED12: {=u8:?}, CUR_EC_LEVEL: {=u16:?}, RESERVED25: {=u8:?} }}",
            self.PUL_CNTR(),
            self.RESERVED12(),
            self.CUR_EC_LEVEL(),
            self.RESERVED25()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_P_OH(pub u32);
impl FSM_P_OH {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PGM_OH(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PGM_OH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FSM_P_OH {
    #[inline(always)]
    fn default() -> FSM_P_OH {
        FSM_P_OH(0)
    }
}
impl core::fmt::Debug for FSM_P_OH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_P_OH")
            .field("RESERVED0", &self.RESERVED0())
            .field("PGM_OH", &self.PGM_OH())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_P_OH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_P_OH {{ RESERVED0: {=u8:?}, PGM_OH: {=u8:?}, RESERVED16: {=u16:?} }}",
            self.RESERVED0(),
            self.PGM_OH(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_RD_H(pub u32);
impl FSM_RD_H {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RD_H(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RD_H(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for FSM_RD_H {
    #[inline(always)]
    fn default() -> FSM_RD_H {
        FSM_RD_H(0)
    }
}
impl core::fmt::Debug for FSM_RD_H {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_RD_H")
            .field("RD_H", &self.RD_H())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_RD_H {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_RD_H {{ RD_H: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.RD_H(),
            self.RESERVED8()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_SAV_ERA_PUL(pub u32);
impl FSM_SAV_ERA_PUL {
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SAV_ERA_PUL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SAV_ERA_PUL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "31:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "31:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FSM_SAV_ERA_PUL {
    #[inline(always)]
    fn default() -> FSM_SAV_ERA_PUL {
        FSM_SAV_ERA_PUL(0)
    }
}
impl core::fmt::Debug for FSM_SAV_ERA_PUL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_SAV_ERA_PUL")
            .field("SAV_ERA_PUL", &self.SAV_ERA_PUL())
            .field("RESERVED12", &self.RESERVED12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_SAV_ERA_PUL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_SAV_ERA_PUL {{ SAV_ERA_PUL: {=u16:?}, RESERVED12: {=u32:?} }}",
            self.SAV_ERA_PUL(),
            self.RESERVED12()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_SAV_PPUL(pub u32);
impl FSM_SAV_PPUL {
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SAV_P_PUL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SAV_P_PUL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "31:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "31:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FSM_SAV_PPUL {
    #[inline(always)]
    fn default() -> FSM_SAV_PPUL {
        FSM_SAV_PPUL(0)
    }
}
impl core::fmt::Debug for FSM_SAV_PPUL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_SAV_PPUL")
            .field("SAV_P_PUL", &self.SAV_P_PUL())
            .field("RESERVED12", &self.RESERVED12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_SAV_PPUL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_SAV_PPUL {{ SAV_P_PUL: {=u16:?}, RESERVED12: {=u32:?} }}",
            self.SAV_P_PUL(),
            self.RESERVED12()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_SECTOR(pub u32);
impl FSM_SECTOR {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_OUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SEC_OUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SECTOR(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SECTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_SECTOR_EXTENSION(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_SECTOR_EXTENSION(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SECT_ERASED(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SECT_ERASED(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FSM_SECTOR {
    #[inline(always)]
    fn default() -> FSM_SECTOR {
        FSM_SECTOR(0)
    }
}
impl core::fmt::Debug for FSM_SECTOR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_SECTOR")
            .field("SEC_OUT", &self.SEC_OUT())
            .field("SECTOR", &self.SECTOR())
            .field("FSM_SECTOR_EXTENSION", &self.FSM_SECTOR_EXTENSION())
            .field("SECT_ERASED", &self.SECT_ERASED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_SECTOR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_SECTOR {{ SEC_OUT: {=u8:?}, SECTOR: {=u8:?}, FSM_SECTOR_EXTENSION: {=u8:?}, SECT_ERASED: {=u16:?} }}",
            self.SEC_OUT(),
            self.SECTOR(),
            self.FSM_SECTOR_EXTENSION(),
            self.SECT_ERASED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_SECTOR1(pub u32);
impl FSM_SECTOR1 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_SECTOR1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_SECTOR1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FSM_SECTOR1 {
    #[inline(always)]
    fn default() -> FSM_SECTOR1 {
        FSM_SECTOR1(0)
    }
}
impl core::fmt::Debug for FSM_SECTOR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_SECTOR1")
            .field("FSM_SECTOR1", &self.FSM_SECTOR1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_SECTOR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_SECTOR1 {{ FSM_SECTOR1: {=u32:?} }}",
            self.FSM_SECTOR1()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_SECTOR2(pub u32);
impl FSM_SECTOR2 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_SECTOR2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_SECTOR2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FSM_SECTOR2 {
    #[inline(always)]
    fn default() -> FSM_SECTOR2 {
        FSM_SECTOR2(0)
    }
}
impl core::fmt::Debug for FSM_SECTOR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_SECTOR2")
            .field("FSM_SECTOR2", &self.FSM_SECTOR2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_SECTOR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_SECTOR2 {{ FSM_SECTOR2: {=u32:?} }}",
            self.FSM_SECTOR2()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_STAT(pub u32);
impl FSM_STAT {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn INV_DAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_INV_DAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn OVR_PUL_CNT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_OVR_PUL_CNT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn NON_OP(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_NON_OP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
impl Default for FSM_STAT {
    #[inline(always)]
    fn default() -> FSM_STAT {
        FSM_STAT(0)
    }
}
impl core::fmt::Debug for FSM_STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_STAT")
            .field("INV_DAT", &self.INV_DAT())
            .field("OVR_PUL_CNT", &self.OVR_PUL_CNT())
            .field("NON_OP", &self.NON_OP())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_STAT {{ INV_DAT: {=bool:?}, OVR_PUL_CNT: {=bool:?}, NON_OP: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.INV_DAT(),
            self.OVR_PUL_CNT(),
            self.NON_OP(),
            self.RESERVED3()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_STATE(pub u32);
impl FSM_STATE {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "6:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn OTP_ACT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_OTP_ACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TIOTP_ACT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TIOTP_ACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_ACT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_ACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EXECUTEZ(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EXECUTEZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CTRLENZ(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CTRLENZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "31:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "31:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FSM_STATE {
    #[inline(always)]
    fn default() -> FSM_STATE {
        FSM_STATE(0)
    }
}
impl core::fmt::Debug for FSM_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_STATE")
            .field("RESERVED0", &self.RESERVED0())
            .field("OTP_ACT", &self.OTP_ACT())
            .field("TIOTP_ACT", &self.TIOTP_ACT())
            .field("FSM_ACT", &self.FSM_ACT())
            .field("RESERVED9", &self.RESERVED9())
            .field("EXECUTEZ", &self.EXECUTEZ())
            .field("CTRLENZ", &self.CTRLENZ())
            .field("RESERVED12", &self.RESERVED12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_STATE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_STATE {{ RESERVED0: {=u8:?}, OTP_ACT: {=bool:?}, TIOTP_ACT: {=bool:?}, FSM_ACT: {=bool:?}, RESERVED9: {=bool:?}, EXECUTEZ: {=bool:?}, CTRLENZ: {=bool:?}, RESERVED12: {=u32:?} }}",
            self.RESERVED0(),
            self.OTP_ACT(),
            self.TIOTP_ACT(),
            self.FSM_ACT(),
            self.RESERVED9(),
            self.EXECUTEZ(),
            self.CTRLENZ(),
            self.RESERVED12()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_STEP_SIZE(pub u32);
impl FSM_STEP_SIZE {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "24:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EC_STEP_SIZE(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "24:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EC_STEP_SIZE(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "31:25\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED25(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "31:25\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for FSM_STEP_SIZE {
    #[inline(always)]
    fn default() -> FSM_STEP_SIZE {
        FSM_STEP_SIZE(0)
    }
}
impl core::fmt::Debug for FSM_STEP_SIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_STEP_SIZE")
            .field("RESERVED0", &self.RESERVED0())
            .field("EC_STEP_SIZE", &self.EC_STEP_SIZE())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_STEP_SIZE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_STEP_SIZE {{ RESERVED0: {=u16:?}, EC_STEP_SIZE: {=u16:?}, RESERVED25: {=u8:?} }}",
            self.RESERVED0(),
            self.EC_STEP_SIZE(),
            self.RESERVED25()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_ST_MACHINE(pub u32);
impl FSM_ST_MACHINE {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn OVERRIDE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn INV_DATA(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_INV_DATA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CMD_EN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CMD_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_TST_EN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DIS_TST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PREC_STOP_EN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PREC_STOP_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PGM_SEC_COF_EN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PGM_SEC_COF_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "10:7\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DBG_SHORT_ROW(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x0f;
        val as u8
    }
    #[doc = "10:7\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DBG_SHORT_ROW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DO_REDU_COL(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DO_REDU_COL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "13:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "13:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "14:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ONE_TIME_GOOD(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ONE_TIME_GOOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RV_INT_EN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RV_INT_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RV_RES(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RV_RES(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RV_SEC_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RV_SEC_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RANDOM(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RANDOM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CMPV_ALLOWED(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CMPV_ALLOWED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ALL_BANKS(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ALL_BANKS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_INT_EN(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_INT_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DO_PRECOND(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DO_PRECOND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED24(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED24(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for FSM_ST_MACHINE {
    #[inline(always)]
    fn default() -> FSM_ST_MACHINE {
        FSM_ST_MACHINE(0)
    }
}
impl core::fmt::Debug for FSM_ST_MACHINE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_ST_MACHINE")
            .field("OVERRIDE", &self.OVERRIDE())
            .field("INV_DATA", &self.INV_DATA())
            .field("CMD_EN", &self.CMD_EN())
            .field("DIS_TST_EN", &self.DIS_TST_EN())
            .field("PREC_STOP_EN", &self.PREC_STOP_EN())
            .field("PGM_SEC_COF_EN", &self.PGM_SEC_COF_EN())
            .field("RESERVED6", &self.RESERVED6())
            .field("DBG_SHORT_ROW", &self.DBG_SHORT_ROW())
            .field("DO_REDU_COL", &self.DO_REDU_COL())
            .field("RESERVED12", &self.RESERVED12())
            .field("ONE_TIME_GOOD", &self.ONE_TIME_GOOD())
            .field("RESERVED15", &self.RESERVED15())
            .field("RV_INT_EN", &self.RV_INT_EN())
            .field("RV_RES", &self.RV_RES())
            .field("RV_SEC_EN", &self.RV_SEC_EN())
            .field("RANDOM", &self.RANDOM())
            .field("CMPV_ALLOWED", &self.CMPV_ALLOWED())
            .field("ALL_BANKS", &self.ALL_BANKS())
            .field("FSM_INT_EN", &self.FSM_INT_EN())
            .field("DO_PRECOND", &self.DO_PRECOND())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_ST_MACHINE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_ST_MACHINE {{ OVERRIDE: {=bool:?}, INV_DATA: {=bool:?}, CMD_EN: {=bool:?}, DIS_TST_EN: {=bool:?}, PREC_STOP_EN: {=bool:?}, PGM_SEC_COF_EN: {=bool:?}, RESERVED6: {=bool:?}, DBG_SHORT_ROW: {=u8:?}, DO_REDU_COL: {=bool:?}, RESERVED12: {=u8:?}, ONE_TIME_GOOD: {=bool:?}, RESERVED15: {=bool:?}, RV_INT_EN: {=bool:?}, RV_RES: {=bool:?}, RV_SEC_EN: {=bool:?}, RANDOM: {=bool:?}, CMPV_ALLOWED: {=bool:?}, ALL_BANKS: {=bool:?}, FSM_INT_EN: {=bool:?}, DO_PRECOND: {=bool:?}, RESERVED24: {=u8:?} }}",
            self.OVERRIDE(),
            self.INV_DATA(),
            self.CMD_EN(),
            self.DIS_TST_EN(),
            self.PREC_STOP_EN(),
            self.PGM_SEC_COF_EN(),
            self.RESERVED6(),
            self.DBG_SHORT_ROW(),
            self.DO_REDU_COL(),
            self.RESERVED12(),
            self.ONE_TIME_GOOD(),
            self.RESERVED15(),
            self.RV_INT_EN(),
            self.RV_RES(),
            self.RV_SEC_EN(),
            self.RANDOM(),
            self.CMPV_ALLOWED(),
            self.ALL_BANKS(),
            self.FSM_INT_EN(),
            self.DO_PRECOND(),
            self.RESERVED24()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_TIMER(pub u32);
impl FSM_TIMER {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FSM_TIMER(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FSM_TIMER(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FSM_TIMER {
    #[inline(always)]
    fn default() -> FSM_TIMER {
        FSM_TIMER(0)
    }
}
impl core::fmt::Debug for FSM_TIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_TIMER")
            .field("FSM_TIMER", &self.FSM_TIMER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_TIMER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FSM_TIMER {{ FSM_TIMER: {=u32:?} }}", self.FSM_TIMER())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_VSTAT(pub u32);
impl FSM_VSTAT {
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VSTAT_CNT(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VSTAT_CNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FSM_VSTAT {
    #[inline(always)]
    fn default() -> FSM_VSTAT {
        FSM_VSTAT(0)
    }
}
impl core::fmt::Debug for FSM_VSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_VSTAT")
            .field("RESERVED0", &self.RESERVED0())
            .field("VSTAT_CNT", &self.VSTAT_CNT())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_VSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_VSTAT {{ RESERVED0: {=u16:?}, VSTAT_CNT: {=u8:?}, RESERVED16: {=u16:?} }}",
            self.RESERVED0(),
            self.VSTAT_CNT(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSM_WR_ENA(pub u32);
impl FSM_WR_ENA {
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn WR_ENA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_WR_ENA(&mut self, val: u8) {
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
impl Default for FSM_WR_ENA {
    #[inline(always)]
    fn default() -> FSM_WR_ENA {
        FSM_WR_ENA(0)
    }
}
impl core::fmt::Debug for FSM_WR_ENA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_WR_ENA")
            .field("WR_ENA", &self.WR_ENA())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSM_WR_ENA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSM_WR_ENA {{ WR_ENA: {=u8:?}, RESERVED3: {=u32:?} }}",
            self.WR_ENA(),
            self.RESERVED3()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSPRD(pub u32);
impl FSPRD {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RM0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RM0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RM1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RM1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "7:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "7:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RMBSEM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RMBSEM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_PREEMPT(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DIS_PREEMPT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FSPRD {
    #[inline(always)]
    fn default() -> FSPRD {
        FSPRD(0)
    }
}
impl core::fmt::Debug for FSPRD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSPRD")
            .field("RM0", &self.RM0())
            .field("RM1", &self.RM1())
            .field("RESERVED2", &self.RESERVED2())
            .field("RMBSEM", &self.RMBSEM())
            .field("DIS_PREEMPT", &self.DIS_PREEMPT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSPRD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSPRD {{ RM0: {=bool:?}, RM1: {=bool:?}, RESERVED2: {=u8:?}, RMBSEM: {=u8:?}, DIS_PREEMPT: {=u16:?} }}",
            self.RM0(),
            self.RM1(),
            self.RESERVED2(),
            self.RMBSEM(),
            self.DIS_PREEMPT()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FSWSTAT(pub u32);
impl FSWSTAT {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SAFELV(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SAFELV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for FSWSTAT {
    #[inline(always)]
    fn default() -> FSWSTAT {
        FSWSTAT(0)
    }
}
impl core::fmt::Debug for FSWSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSWSTAT")
            .field("SAFELV", &self.SAFELV())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FSWSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FSWSTAT {{ SAFELV: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.SAFELV(),
            self.RESERVED1()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FTCR(pub u32);
impl FTCR {
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TCR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TCR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED7(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for FTCR {
    #[inline(always)]
    fn default() -> FTCR {
        FTCR(0)
    }
}
impl core::fmt::Debug for FTCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTCR")
            .field("TCR", &self.TCR())
            .field("RESERVED7", &self.RESERVED7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FTCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FTCR {{ TCR: {=u8:?}, RESERVED7: {=u32:?} }}",
            self.TCR(),
            self.RESERVED7()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FTCTL(pub u32);
impl FTCTL {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TEST_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TEST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "15:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "15:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn WDATA_BLK_CLR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_WDATA_BLK_CLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for FTCTL {
    #[inline(always)]
    fn default() -> FTCTL {
        FTCTL(0)
    }
}
impl core::fmt::Debug for FTCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTCTL")
            .field("RESERVED0", &self.RESERVED0())
            .field("TEST_EN", &self.TEST_EN())
            .field("RESERVED2", &self.RESERVED2())
            .field("WDATA_BLK_CLR", &self.WDATA_BLK_CLR())
            .field("RESERVED17", &self.RESERVED17())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FTCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FTCTL {{ RESERVED0: {=bool:?}, TEST_EN: {=bool:?}, RESERVED2: {=u16:?}, WDATA_BLK_CLR: {=bool:?}, RESERVED17: {=u16:?} }}",
            self.RESERVED0(),
            self.TEST_EN(),
            self.RESERVED2(),
            self.WDATA_BLK_CLR(),
            self.RESERVED17()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FUNC_ERR_ADD(pub u32);
impl FUNC_ERR_ADD {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC_ERR_ADD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FUNC_ERR_ADD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FUNC_ERR_ADD {
    #[inline(always)]
    fn default() -> FUNC_ERR_ADD {
        FUNC_ERR_ADD(0)
    }
}
impl core::fmt::Debug for FUNC_ERR_ADD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_ERR_ADD")
            .field("FUNC_ERR_ADD", &self.FUNC_ERR_ADD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FUNC_ERR_ADD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FUNC_ERR_ADD {{ FUNC_ERR_ADD: {=u32:?} }}",
            self.FUNC_ERR_ADD()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FVHVCT1(pub u32);
impl FVHVCT1 {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VHVCT_PV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VHVCT_PV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM13_PV(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIM13_PV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VHVCT_E(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VHVCT_E(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM13_E(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIM13_E(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED24(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED24(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for FVHVCT1 {
    #[inline(always)]
    fn default() -> FVHVCT1 {
        FVHVCT1(0)
    }
}
impl core::fmt::Debug for FVHVCT1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FVHVCT1")
            .field("VHVCT_PV", &self.VHVCT_PV())
            .field("TRIM13_PV", &self.TRIM13_PV())
            .field("RESERVED8", &self.RESERVED8())
            .field("VHVCT_E", &self.VHVCT_E())
            .field("TRIM13_E", &self.TRIM13_E())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FVHVCT1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FVHVCT1 {{ VHVCT_PV: {=u8:?}, TRIM13_PV: {=u8:?}, RESERVED8: {=u8:?}, VHVCT_E: {=u8:?}, TRIM13_E: {=u8:?}, RESERVED24: {=u8:?} }}",
            self.VHVCT_PV(),
            self.TRIM13_PV(),
            self.RESERVED8(),
            self.VHVCT_E(),
            self.TRIM13_E(),
            self.RESERVED24()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FVHVCT2(pub u32);
impl FVHVCT2 {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VHVCT_P(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VHVCT_P(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM13_P(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIM13_P(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED24(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED24(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for FVHVCT2 {
    #[inline(always)]
    fn default() -> FVHVCT2 {
        FVHVCT2(0)
    }
}
impl core::fmt::Debug for FVHVCT2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FVHVCT2")
            .field("RESERVED0", &self.RESERVED0())
            .field("VHVCT_P", &self.VHVCT_P())
            .field("TRIM13_P", &self.TRIM13_P())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FVHVCT2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FVHVCT2 {{ RESERVED0: {=u16:?}, VHVCT_P: {=u8:?}, TRIM13_P: {=u8:?}, RESERVED24: {=u8:?} }}",
            self.RESERVED0(),
            self.VHVCT_P(),
            self.TRIM13_P(),
            self.RESERVED24()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FVHVCT3(pub u32);
impl FVHVCT3 {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VHVCT_READ(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VHVCT_READ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "15:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "15:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn WCT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_WCT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "31:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED20(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "31:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED20(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for FVHVCT3 {
    #[inline(always)]
    fn default() -> FVHVCT3 {
        FVHVCT3(0)
    }
}
impl core::fmt::Debug for FVHVCT3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FVHVCT3")
            .field("VHVCT_READ", &self.VHVCT_READ())
            .field("RESERVED4", &self.RESERVED4())
            .field("WCT", &self.WCT())
            .field("RESERVED20", &self.RESERVED20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FVHVCT3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FVHVCT3 {{ VHVCT_READ: {=u8:?}, RESERVED4: {=u16:?}, WCT: {=u8:?}, RESERVED20: {=u16:?} }}",
            self.VHVCT_READ(),
            self.RESERVED4(),
            self.WCT(),
            self.RESERVED20()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FVNVCT(pub u32);
impl FVNVCT {
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VIN_CT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VIN_CT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "7:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "7:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "12:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VCG2P5CT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "12:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VCG2P5CT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "31:13\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED13(&self) -> u32 {
        let val = (self.0 >> 13usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "31:13\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED13(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 13usize)) | (((val as u32) & 0x0007_ffff) << 13usize);
    }
}
impl Default for FVNVCT {
    #[inline(always)]
    fn default() -> FVNVCT {
        FVNVCT(0)
    }
}
impl core::fmt::Debug for FVNVCT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FVNVCT")
            .field("VIN_CT", &self.VIN_CT())
            .field("RESERVED5", &self.RESERVED5())
            .field("VCG2P5CT", &self.VCG2P5CT())
            .field("RESERVED13", &self.RESERVED13())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FVNVCT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FVNVCT {{ VIN_CT: {=u8:?}, RESERVED5: {=u8:?}, VCG2P5CT: {=u8:?}, RESERVED13: {=u32:?} }}",
            self.VIN_CT(),
            self.RESERVED5(),
            self.VCG2P5CT(),
            self.RESERVED13()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FVREADCT(pub u32);
impl FVREADCT {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VREADCT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VREADCT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "31:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "31:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for FVREADCT {
    #[inline(always)]
    fn default() -> FVREADCT {
        FVREADCT(0)
    }
}
impl core::fmt::Debug for FVREADCT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FVREADCT")
            .field("VREADCT", &self.VREADCT())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FVREADCT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FVREADCT {{ VREADCT: {=u8:?}, RESERVED4: {=u32:?} }}",
            self.VREADCT(),
            self.RESERVED4()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FVSLP(pub u32);
impl FVSLP {
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VSL_P(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VSL_P(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FVSLP {
    #[inline(always)]
    fn default() -> FVSLP {
        FVSLP(0)
    }
}
impl core::fmt::Debug for FVSLP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FVSLP")
            .field("RESERVED0", &self.RESERVED0())
            .field("VSL_P", &self.VSL_P())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FVSLP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FVSLP {{ RESERVED0: {=u16:?}, VSL_P: {=u8:?}, RESERVED16: {=u16:?} }}",
            self.RESERVED0(),
            self.VSL_P(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FVWLCT(pub u32);
impl FVWLCT {
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VWLCT_P(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VWLCT_P(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "31:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "31:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for FVWLCT {
    #[inline(always)]
    fn default() -> FVWLCT {
        FVWLCT(0)
    }
}
impl core::fmt::Debug for FVWLCT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FVWLCT")
            .field("VWLCT_P", &self.VWLCT_P())
            .field("RESERVED5", &self.RESERVED5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FVWLCT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FVWLCT {{ VWLCT_P: {=u8:?}, RESERVED5: {=u32:?} }}",
            self.VWLCT_P(),
            self.RESERVED5()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FWFLAG(pub u32);
impl FWFLAG {
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FWFLAG(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FWFLAG(&mut self, val: u8) {
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
impl Default for FWFLAG {
    #[inline(always)]
    fn default() -> FWFLAG {
        FWFLAG(0)
    }
}
impl core::fmt::Debug for FWFLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FWFLAG")
            .field("FWFLAG", &self.FWFLAG())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FWFLAG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FWFLAG {{ FWFLAG: {=u8:?}, RESERVED3: {=u32:?} }}",
            self.FWFLAG(),
            self.RESERVED3()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FWLOCK(pub u32);
impl FWLOCK {
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FWLOCK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FWLOCK(&mut self, val: u8) {
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
impl Default for FWLOCK {
    #[inline(always)]
    fn default() -> FWLOCK {
        FWLOCK(0)
    }
}
impl core::fmt::Debug for FWLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FWLOCK")
            .field("FWLOCK", &self.FWLOCK())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FWLOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FWLOCK {{ FWLOCK: {=u8:?}, RESERVED3: {=u32:?} }}",
            self.FWLOCK(),
            self.RESERVED3()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FWPWRITE0(pub u32);
impl FWPWRITE0 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FWPWRITE0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FWPWRITE0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FWPWRITE0 {
    #[inline(always)]
    fn default() -> FWPWRITE0 {
        FWPWRITE0(0)
    }
}
impl core::fmt::Debug for FWPWRITE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FWPWRITE0")
            .field("FWPWRITE0", &self.FWPWRITE0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FWPWRITE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FWPWRITE0 {{ FWPWRITE0: {=u32:?} }}", self.FWPWRITE0())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FWPWRITE1(pub u32);
impl FWPWRITE1 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FWPWRITE1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FWPWRITE1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FWPWRITE1 {
    #[inline(always)]
    fn default() -> FWPWRITE1 {
        FWPWRITE1(0)
    }
}
impl core::fmt::Debug for FWPWRITE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FWPWRITE1")
            .field("FWPWRITE1", &self.FWPWRITE1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FWPWRITE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FWPWRITE1 {{ FWPWRITE1: {=u32:?} }}", self.FWPWRITE1())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FWPWRITE2(pub u32);
impl FWPWRITE2 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FWPWRITE2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FWPWRITE2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FWPWRITE2 {
    #[inline(always)]
    fn default() -> FWPWRITE2 {
        FWPWRITE2(0)
    }
}
impl core::fmt::Debug for FWPWRITE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FWPWRITE2")
            .field("FWPWRITE2", &self.FWPWRITE2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FWPWRITE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FWPWRITE2 {{ FWPWRITE2: {=u32:?} }}", self.FWPWRITE2())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FWPWRITE3(pub u32);
impl FWPWRITE3 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FWPWRITE3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FWPWRITE3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FWPWRITE3 {
    #[inline(always)]
    fn default() -> FWPWRITE3 {
        FWPWRITE3(0)
    }
}
impl core::fmt::Debug for FWPWRITE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FWPWRITE3")
            .field("FWPWRITE3", &self.FWPWRITE3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FWPWRITE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FWPWRITE3 {{ FWPWRITE3: {=u32:?} }}", self.FWPWRITE3())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FWPWRITE4(pub u32);
impl FWPWRITE4 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FWPWRITE4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FWPWRITE4(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FWPWRITE4 {
    #[inline(always)]
    fn default() -> FWPWRITE4 {
        FWPWRITE4(0)
    }
}
impl core::fmt::Debug for FWPWRITE4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FWPWRITE4")
            .field("FWPWRITE4", &self.FWPWRITE4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FWPWRITE4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FWPWRITE4 {{ FWPWRITE4: {=u32:?} }}", self.FWPWRITE4())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FWPWRITE5(pub u32);
impl FWPWRITE5 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FWPWRITE5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FWPWRITE5(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FWPWRITE5 {
    #[inline(always)]
    fn default() -> FWPWRITE5 {
        FWPWRITE5(0)
    }
}
impl core::fmt::Debug for FWPWRITE5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FWPWRITE5")
            .field("FWPWRITE5", &self.FWPWRITE5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FWPWRITE5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FWPWRITE5 {{ FWPWRITE5: {=u32:?} }}", self.FWPWRITE5())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FWPWRITE6(pub u32);
impl FWPWRITE6 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FWPWRITE6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FWPWRITE6(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FWPWRITE6 {
    #[inline(always)]
    fn default() -> FWPWRITE6 {
        FWPWRITE6(0)
    }
}
impl core::fmt::Debug for FWPWRITE6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FWPWRITE6")
            .field("FWPWRITE6", &self.FWPWRITE6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FWPWRITE6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FWPWRITE6 {{ FWPWRITE6: {=u32:?} }}", self.FWPWRITE6())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FWPWRITE7(pub u32);
impl FWPWRITE7 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FWPWRITE7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FWPWRITE7(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FWPWRITE7 {
    #[inline(always)]
    fn default() -> FWPWRITE7 {
        FWPWRITE7(0)
    }
}
impl core::fmt::Debug for FWPWRITE7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FWPWRITE7")
            .field("FWPWRITE7", &self.FWPWRITE7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FWPWRITE7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FWPWRITE7 {{ FWPWRITE7: {=u32:?} }}", self.FWPWRITE7())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FWPWRITE_ECC(pub u32);
impl FWPWRITE_ECC {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ECCBYTES31_24(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ECCBYTES31_24(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ECCBYTES23_16(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ECCBYTES23_16(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ECCBYTES15_08(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ECCBYTES15_08(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ECCBYTES07_00(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ECCBYTES07_00(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for FWPWRITE_ECC {
    #[inline(always)]
    fn default() -> FWPWRITE_ECC {
        FWPWRITE_ECC(0)
    }
}
impl core::fmt::Debug for FWPWRITE_ECC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FWPWRITE_ECC")
            .field("ECCBYTES31_24", &self.ECCBYTES31_24())
            .field("ECCBYTES23_16", &self.ECCBYTES23_16())
            .field("ECCBYTES15_08", &self.ECCBYTES15_08())
            .field("ECCBYTES07_00", &self.ECCBYTES07_00())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FWPWRITE_ECC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FWPWRITE_ECC {{ ECCBYTES31_24: {=u8:?}, ECCBYTES23_16: {=u8:?}, ECCBYTES15_08: {=u8:?}, ECCBYTES07_00: {=u8:?} }}",
            self.ECCBYTES31_24(),
            self.ECCBYTES23_16(),
            self.ECCBYTES15_08(),
            self.ECCBYTES07_00()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PBISTCTL(pub u32);
impl PBISTCTL {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PBIST_KEY(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PBIST_KEY(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PBISTCTL {
    #[inline(always)]
    fn default() -> PBISTCTL {
        PBISTCTL(0)
    }
}
impl core::fmt::Debug for PBISTCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PBISTCTL")
            .field("PBIST_KEY", &self.PBIST_KEY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PBISTCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PBISTCTL {{ PBIST_KEY: {=u32:?} }}", self.PBIST_KEY())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ROM_TEST(pub u32);
impl ROM_TEST {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ROM_KEY(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ROM_KEY(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ROM_TEST {
    #[inline(always)]
    fn default() -> ROM_TEST {
        ROM_TEST(0)
    }
}
impl core::fmt::Debug for ROM_TEST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_TEST")
            .field("ROM_KEY", &self.ROM_KEY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ROM_TEST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ROM_TEST {{ ROM_KEY: {=u32:?} }}", self.ROM_KEY())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SELFTESTCYC(pub u32);
impl SELFTESTCYC {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CYCLES(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CYCLES(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SELFTESTCYC {
    #[inline(always)]
    fn default() -> SELFTESTCYC {
        SELFTESTCYC(0)
    }
}
impl core::fmt::Debug for SELFTESTCYC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SELFTESTCYC")
            .field("CYCLES", &self.CYCLES())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SELFTESTCYC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SELFTESTCYC {{ CYCLES: {=u32:?} }}", self.CYCLES())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SELFTESTSIGN(pub u32);
impl SELFTESTSIGN {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SIGNATURE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SIGNATURE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SELFTESTSIGN {
    #[inline(always)]
    fn default() -> SELFTESTSIGN {
        SELFTESTSIGN(0)
    }
}
impl core::fmt::Debug for SELFTESTSIGN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SELFTESTSIGN")
            .field("SIGNATURE", &self.SIGNATURE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SELFTESTSIGN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SELFTESTSIGN {{ SIGNATURE: {=u32:?} }}",
            self.SIGNATURE()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SINGLEBIT(pub u32);
impl SINGLEBIT {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FROM0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FROM0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FROMN(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FROMN(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for SINGLEBIT {
    #[inline(always)]
    fn default() -> SINGLEBIT {
        SINGLEBIT(0)
    }
}
impl core::fmt::Debug for SINGLEBIT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SINGLEBIT")
            .field("FROM0", &self.FROM0())
            .field("FROMN", &self.FROMN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SINGLEBIT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SINGLEBIT {{ FROM0: {=bool:?}, FROMN: {=u32:?} }}",
            self.FROM0(),
            self.FROMN()
        )
    }
}
#[doc = "FMC and Efuse Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STAT(pub u32);
impl STAT {
    #[doc = "0:0\\] Power state of the flash sub-system. 0 : Active 1 : Low power."]
    #[must_use]
    #[inline(always)]
    pub const fn POWER_MODE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Power state of the flash sub-system. 0 : Active 1 : Low power."]
    #[inline(always)]
    pub const fn set_POWER_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Fast version of the FMC FMSTAT.BUSY bit. This flag is valid immediately after the operation setting it (FMSTAT.BUSY is delayed some cycles) 0 : Not busy 1 : Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Fast version of the FMC FMSTAT.BUSY bit. This flag is valid immediately after the operation setting it (FMSTAT.BUSY is delayed some cycles) 0 : Not busy 1 : Busy."]
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Status indicator of flash sample and hold sequencing logic. This bit will go to 1 some delay after CFG.DIS_IDLE is set to 1. 0: Not disabled 1: Sample and hold disabled and stable."]
    #[must_use]
    #[inline(always)]
    pub const fn SAMHOLD_DIS(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Status indicator of flash sample and hold sequencing logic. This bit will go to 1 some delay after CFG.DIS_IDLE is set to 1. 0: Not disabled 1: Sample and hold disabled and stable."]
    #[inline(always)]
    pub const fn set_SAMHOLD_DIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "7:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "7:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
    }
    #[doc = "12:8\\] Same as EFUSEERROR.CODE."]
    #[must_use]
    #[inline(always)]
    pub const fn EFUSE_ERRCODE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "12:8\\] Same as EFUSEERROR.CODE."]
    #[inline(always)]
    pub const fn set_EFUSE_ERRCODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "13:13\\] Efuse scanning resulted in scan chain CRC error. 0 : No CRC error 1 : CRC Error."]
    #[must_use]
    #[inline(always)]
    pub const fn EFUSE_CRC_ERROR(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Efuse scanning resulted in scan chain CRC error. 0 : No CRC error 1 : CRC Error."]
    #[inline(always)]
    pub const fn set_EFUSE_CRC_ERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error."]
    #[must_use]
    #[inline(always)]
    pub const fn EFUSE_TIMEOUT(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Efuse scanning resulted in timeout error. 0 : No Timeout error 1 : Timeout Error."]
    #[inline(always)]
    pub const fn set_EFUSE_TIMEOUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank."]
    #[must_use]
    #[inline(always)]
    pub const fn EFUSE_BLANK(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Efuse scanning detected if fuse ROM is blank: 0 : Not blank 1 : Blank."]
    #[inline(always)]
    pub const fn set_EFUSE_BLANK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
            .field("POWER_MODE", &self.POWER_MODE())
            .field("BUSY", &self.BUSY())
            .field("SAMHOLD_DIS", &self.SAMHOLD_DIS())
            .field("RESERVED3", &self.RESERVED3())
            .field("EFUSE_ERRCODE", &self.EFUSE_ERRCODE())
            .field("EFUSE_CRC_ERROR", &self.EFUSE_CRC_ERROR())
            .field("EFUSE_TIMEOUT", &self.EFUSE_TIMEOUT())
            .field("EFUSE_BLANK", &self.EFUSE_BLANK())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STAT {{ POWER_MODE: {=bool:?}, BUSY: {=bool:?}, SAMHOLD_DIS: {=bool:?}, RESERVED3: {=u8:?}, EFUSE_ERRCODE: {=u8:?}, EFUSE_CRC_ERROR: {=bool:?}, EFUSE_TIMEOUT: {=bool:?}, EFUSE_BLANK: {=bool:?}, RESERVED16: {=u16:?} }}",
            self.POWER_MODE(),
            self.BUSY(),
            self.SAMHOLD_DIS(),
            self.RESERVED3(),
            self.EFUSE_ERRCODE(),
            self.EFUSE_CRC_ERROR(),
            self.EFUSE_TIMEOUT(),
            self.EFUSE_BLANK(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYSCODE_START(pub u32);
impl SYSCODE_START {
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSCODE_START(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SYSCODE_START(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "31:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "31:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for SYSCODE_START {
    #[inline(always)]
    fn default() -> SYSCODE_START {
        SYSCODE_START(0)
    }
}
impl core::fmt::Debug for SYSCODE_START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCODE_START")
            .field("SYSCODE_START", &self.SYSCODE_START())
            .field("RESERVED5", &self.RESERVED5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SYSCODE_START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SYSCODE_START {{ SYSCODE_START: {=u8:?}, RESERVED5: {=u32:?} }}",
            self.SYSCODE_START(),
            self.RESERVED5()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TWOBIT(pub u32);
impl TWOBIT {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FROM0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FROM0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FROMN(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FROMN(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for TWOBIT {
    #[inline(always)]
    fn default() -> TWOBIT {
        TWOBIT(0)
    }
}
impl core::fmt::Debug for TWOBIT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWOBIT")
            .field("FROM0", &self.FROM0())
            .field("FROMN", &self.FROMN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TWOBIT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TWOBIT {{ FROM0: {=bool:?}, FROMN: {=u32:?} }}",
            self.FROM0(),
            self.FROMN()
        )
    }
}

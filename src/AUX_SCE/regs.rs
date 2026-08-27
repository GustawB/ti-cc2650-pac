#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUSTAT(pub u32);
impl CPUSTAT {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn Z_FLAG(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_Z_FLAG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn N_FLAG(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_N_FLAG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn C_FLAG(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_C_FLAG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn V_FLAG(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_V_FLAG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SELF_STOP(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SELF_STOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn WEV(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_WEV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEEP(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SLEEP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BUS_ERROR(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BUS_ERROR(&mut self, val: bool) {
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
impl Default for CPUSTAT {
    #[inline(always)]
    fn default() -> CPUSTAT {
        CPUSTAT(0)
    }
}
impl core::fmt::Debug for CPUSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUSTAT")
            .field("Z_FLAG", &self.Z_FLAG())
            .field("N_FLAG", &self.N_FLAG())
            .field("C_FLAG", &self.C_FLAG())
            .field("V_FLAG", &self.V_FLAG())
            .field("RESERVED4", &self.RESERVED4())
            .field("SELF_STOP", &self.SELF_STOP())
            .field("WEV", &self.WEV())
            .field("SLEEP", &self.SLEEP())
            .field("BUS_ERROR", &self.BUS_ERROR())
            .field("RESERVED12", &self.RESERVED12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUSTAT {{ Z_FLAG: {=bool:?}, N_FLAG: {=bool:?}, C_FLAG: {=bool:?}, V_FLAG: {=bool:?}, RESERVED4: {=u8:?}, SELF_STOP: {=bool:?}, WEV: {=bool:?}, SLEEP: {=bool:?}, BUS_ERROR: {=bool:?}, RESERVED12: {=u32:?} }}",
            self.Z_FLAG(),
            self.N_FLAG(),
            self.C_FLAG(),
            self.V_FLAG(),
            self.RESERVED4(),
            self.SELF_STOP(),
            self.WEV(),
            self.SLEEP(),
            self.BUS_ERROR(),
            self.RESERVED12()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTL(pub u32);
impl CTL {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SUSPEND(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SUSPEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SINGLE_STEP(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SINGLE_STEP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESTART(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCE_WU_HIGH(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FORCE_WU_HIGH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCE_WU_LOW(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FORCE_WU_LOW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DBG_FREEZE_EN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DBG_FREEZE_EN(&mut self, val: bool) {
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
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET_VECTOR(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESET_VECTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
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
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCE_EV_HIGH(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FORCE_EV_HIGH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCE_EV_LOW(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FORCE_EV_LOW(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
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
            .field("CLK_EN", &self.CLK_EN())
            .field("SUSPEND", &self.SUSPEND())
            .field("SINGLE_STEP", &self.SINGLE_STEP())
            .field("RESTART", &self.RESTART())
            .field("FORCE_WU_HIGH", &self.FORCE_WU_HIGH())
            .field("FORCE_WU_LOW", &self.FORCE_WU_LOW())
            .field("DBG_FREEZE_EN", &self.DBG_FREEZE_EN())
            .field("RESERVED7", &self.RESERVED7())
            .field("RESET_VECTOR", &self.RESET_VECTOR())
            .field("RESERVED12", &self.RESERVED12())
            .field("FORCE_EV_HIGH", &self.FORCE_EV_HIGH())
            .field("FORCE_EV_LOW", &self.FORCE_EV_LOW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTL {{ CLK_EN: {=bool:?}, SUSPEND: {=bool:?}, SINGLE_STEP: {=bool:?}, RESTART: {=bool:?}, FORCE_WU_HIGH: {=bool:?}, FORCE_WU_LOW: {=bool:?}, DBG_FREEZE_EN: {=bool:?}, RESERVED7: {=bool:?}, RESET_VECTOR: {=u8:?}, RESERVED12: {=u8:?}, FORCE_EV_HIGH: {=u8:?}, FORCE_EV_LOW: {=u8:?} }}",
            self.CLK_EN(),
            self.SUSPEND(),
            self.SINGLE_STEP(),
            self.RESTART(),
            self.FORCE_WU_HIGH(),
            self.FORCE_WU_LOW(),
            self.DBG_FREEZE_EN(),
            self.RESERVED7(),
            self.RESET_VECTOR(),
            self.RESERVED12(),
            self.FORCE_EV_HIGH(),
            self.FORCE_EV_LOW()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FETCHSTAT(pub u32);
impl FETCHSTAT {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PC(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PC(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn OPCODE(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_OPCODE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FETCHSTAT {
    #[inline(always)]
    fn default() -> FETCHSTAT {
        FETCHSTAT(0)
    }
}
impl core::fmt::Debug for FETCHSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FETCHSTAT")
            .field("PC", &self.PC())
            .field("OPCODE", &self.OPCODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FETCHSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FETCHSTAT {{ PC: {=u16:?}, OPCODE: {=u16:?} }}",
            self.PC(),
            self.OPCODE()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LOOPADDR(pub u32);
impl LOOPADDR {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn START(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_START(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn STOP(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_STOP(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for LOOPADDR {
    #[inline(always)]
    fn default() -> LOOPADDR {
        LOOPADDR(0)
    }
}
impl core::fmt::Debug for LOOPADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOOPADDR")
            .field("START", &self.START())
            .field("STOP", &self.STOP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LOOPADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LOOPADDR {{ START: {=u16:?}, STOP: {=u16:?} }}",
            self.START(),
            self.STOP()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LOOPCNT(pub u32);
impl LOOPCNT {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ITER_LEFT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ITER_LEFT(&mut self, val: u8) {
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
impl Default for LOOPCNT {
    #[inline(always)]
    fn default() -> LOOPCNT {
        LOOPCNT(0)
    }
}
impl core::fmt::Debug for LOOPCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOOPCNT")
            .field("ITER_LEFT", &self.ITER_LEFT())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LOOPCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LOOPCNT {{ ITER_LEFT: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.ITER_LEFT(),
            self.RESERVED8()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct REG1_0(pub u32);
impl REG1_0 {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn REG0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_REG0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn REG1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_REG1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for REG1_0 {
    #[inline(always)]
    fn default() -> REG1_0 {
        REG1_0(0)
    }
}
impl core::fmt::Debug for REG1_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG1_0")
            .field("REG0", &self.REG0())
            .field("REG1", &self.REG1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for REG1_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "REG1_0 {{ REG0: {=u16:?}, REG1: {=u16:?} }}",
            self.REG0(),
            self.REG1()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct REG3_2(pub u32);
impl REG3_2 {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn REG2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_REG2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn REG3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_REG3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for REG3_2 {
    #[inline(always)]
    fn default() -> REG3_2 {
        REG3_2(0)
    }
}
impl core::fmt::Debug for REG3_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG3_2")
            .field("REG2", &self.REG2())
            .field("REG3", &self.REG3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for REG3_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "REG3_2 {{ REG2: {=u16:?}, REG3: {=u16:?} }}",
            self.REG2(),
            self.REG3()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct REG5_4(pub u32);
impl REG5_4 {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn REG4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_REG4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn REG5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_REG5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for REG5_4 {
    #[inline(always)]
    fn default() -> REG5_4 {
        REG5_4(0)
    }
}
impl core::fmt::Debug for REG5_4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG5_4")
            .field("REG4", &self.REG4())
            .field("REG5", &self.REG5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for REG5_4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "REG5_4 {{ REG4: {=u16:?}, REG5: {=u16:?} }}",
            self.REG4(),
            self.REG5()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct REG7_6(pub u32);
impl REG7_6 {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn REG6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_REG6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn REG7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_REG7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for REG7_6 {
    #[inline(always)]
    fn default() -> REG7_6 {
        REG7_6(0)
    }
}
impl core::fmt::Debug for REG7_6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG7_6")
            .field("REG6", &self.REG6())
            .field("REG7", &self.REG7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for REG7_6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "REG7_6 {{ REG6: {=u16:?}, REG7: {=u16:?} }}",
            self.REG6(),
            self.REG7()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WUSTAT(pub u32);
impl WUSTAT {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EV_SIGNALS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EV_SIGNALS(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_SIGNAL(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_WU_SIGNAL(&mut self, val: bool) {
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
    #[doc = "17:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EXC_VECTOR(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "17:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EXC_VECTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
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
impl Default for WUSTAT {
    #[inline(always)]
    fn default() -> WUSTAT {
        WUSTAT(0)
    }
}
impl core::fmt::Debug for WUSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUSTAT")
            .field("EV_SIGNALS", &self.EV_SIGNALS())
            .field("WU_SIGNAL", &self.WU_SIGNAL())
            .field("RESERVED9", &self.RESERVED9())
            .field("EXC_VECTOR", &self.EXC_VECTOR())
            .field("RESERVED18", &self.RESERVED18())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WUSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WUSTAT {{ EV_SIGNALS: {=u8:?}, WU_SIGNAL: {=bool:?}, RESERVED9: {=u8:?}, EXC_VECTOR: {=u8:?}, RESERVED18: {=u16:?} }}",
            self.EV_SIGNALS(),
            self.WU_SIGNAL(),
            self.RESERVED9(),
            self.EXC_VECTOR(),
            self.RESERVED18()
        )
    }
}

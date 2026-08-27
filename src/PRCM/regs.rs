#[doc = "Load PRCM Settings To CLKCTRL Power Domain."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLKLOADCTL(pub u32);
impl CLKLOADCTL {
    #[doc = "0:0\\] 0: No action 1: Load settings to CLKCTRL. Bit is HW cleared. Multiple changes to settings may be done before LOAD is written once so all changes takes place at the same time. LOAD can also be done after single setting updates. Registers that needs to be followed by LOAD before settings being applied are: - RFCCLKG - VIMSCLKG - SECDMACLKGR - SECDMACLKGS - SECDMACLKGDS - GPIOCLKGR - GPIOCLKGS - GPIOCLKGDS - GPTCLKGR - GPTCLKGS - GPTCLKGDS - GPTCLKDIV - I2CCLKGR - I2CCLKGS - I2CCLKGDS - SSICLKGR - SSICLKGS - SSICLKGDS - UARTCLKGR - UARTCLKGS - UARTCLKGDS - I2SCLKGR - I2SCLKGS - I2SCLKGDS - I2SBCLKSEL - I2SCLKCTL - I2SMCLKDIV - I2SBCLKDIV - I2SWCLKDIV."]
    #[must_use]
    #[inline(always)]
    pub const fn LOAD(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: No action 1: Load settings to CLKCTRL. Bit is HW cleared. Multiple changes to settings may be done before LOAD is written once so all changes takes place at the same time. LOAD can also be done after single setting updates. Registers that needs to be followed by LOAD before settings being applied are: - RFCCLKG - VIMSCLKG - SECDMACLKGR - SECDMACLKGS - SECDMACLKGDS - GPIOCLKGR - GPIOCLKGS - GPIOCLKGDS - GPTCLKGR - GPTCLKGS - GPTCLKGDS - GPTCLKDIV - I2CCLKGR - I2CCLKGS - I2CCLKGDS - SSICLKGR - SSICLKGS - SSICLKGDS - UARTCLKGR - UARTCLKGS - UARTCLKGDS - I2SCLKGR - I2SCLKGS - I2SCLKGDS - I2SBCLKSEL - I2SCLKCTL - I2SMCLKDIV - I2SBCLKDIV - I2SWCLKDIV."]
    #[inline(always)]
    pub const fn set_LOAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Status of LOAD. Will be cleared to 0 when any of the registers requiring a LOAD is written to, and be set to 1 when a LOAD is done. Note that writing no change to a register will result in the LOAD_DONE being cleared. 0 : One or more registers have been write accessed after last LOAD 1 : No registers are write accessed after last LOAD."]
    #[must_use]
    #[inline(always)]
    pub const fn LOAD_DONE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Status of LOAD. Will be cleared to 0 when any of the registers requiring a LOAD is written to, and be set to 1 when a LOAD is done. Note that writing no change to a register will result in the LOAD_DONE being cleared. 0 : One or more registers have been write accessed after last LOAD 1 : No registers are write accessed after last LOAD."]
    #[inline(always)]
    pub const fn set_LOAD_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
impl Default for CLKLOADCTL {
    #[inline(always)]
    fn default() -> CLKLOADCTL {
        CLKLOADCTL(0)
    }
}
impl core::fmt::Debug for CLKLOADCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKLOADCTL")
            .field("LOAD", &self.LOAD())
            .field("LOAD_DONE", &self.LOAD_DONE())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLKLOADCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CLKLOADCTL {{ LOAD: {=bool:?}, LOAD_DONE: {=bool:?}, RESERVED2: {=u32:?} }}",
            self.LOAD(),
            self.LOAD_DONE(),
            self.RESERVED2()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUCLKDIV(pub u32);
impl CPUCLKDIV {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RATIO(&self) -> super::vals::CPUCLKDIV_RATIO {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CPUCLKDIV_RATIO::from_bits(val as u8)
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RATIO(&mut self, val: super::vals::CPUCLKDIV_RATIO) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
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
impl Default for CPUCLKDIV {
    #[inline(always)]
    fn default() -> CPUCLKDIV {
        CPUCLKDIV(0)
    }
}
impl core::fmt::Debug for CPUCLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUCLKDIV")
            .field("RATIO", &self.RATIO())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUCLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUCLKDIV {{ RATIO: {:?}, RESERVED1: {=u32:?} }}",
            self.RATIO(),
            self.RESERVED1()
        )
    }
}
#[doc = "GPIO Clock Gate For Deep Sleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIOCLKGDS(pub u32);
impl GPIOCLKGDS {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: bool) {
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
impl Default for GPIOCLKGDS {
    #[inline(always)]
    fn default() -> GPIOCLKGDS {
        GPIOCLKGDS(0)
    }
}
impl core::fmt::Debug for GPIOCLKGDS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOCLKGDS")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIOCLKGDS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIOCLKGDS {{ CLK_EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "GPIO Clock Gate For Run Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIOCLKGR(pub u32);
impl GPIOCLKGR {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: bool) {
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
impl Default for GPIOCLKGR {
    #[inline(always)]
    fn default() -> GPIOCLKGR {
        GPIOCLKGR(0)
    }
}
impl core::fmt::Debug for GPIOCLKGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOCLKGR")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIOCLKGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIOCLKGR {{ CLK_EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "GPIO Clock Gate For Sleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIOCLKGS(pub u32);
impl GPIOCLKGS {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: bool) {
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
impl Default for GPIOCLKGS {
    #[inline(always)]
    fn default() -> GPIOCLKGS {
        GPIOCLKGS(0)
    }
}
impl core::fmt::Debug for GPIOCLKGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOCLKGS")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIOCLKGS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIOCLKGS {{ CLK_EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "GPT Scalar."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPTCLKDIV(pub u32);
impl GPTCLKDIV {
    #[doc = "3:0\\] Scalar used for GPTs. The division rate will be constant and ungated for Run / Sleep / DeepSleep mode. For changes to take effect, CLKLOADCTL.LOAD needs to be written Other values are not supported."]
    #[must_use]
    #[inline(always)]
    pub const fn RATIO(&self) -> super::vals::GPTCLKDIV_RATIO {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::GPTCLKDIV_RATIO::from_bits(val as u8)
    }
    #[doc = "3:0\\] Scalar used for GPTs. The division rate will be constant and ungated for Run / Sleep / DeepSleep mode. For changes to take effect, CLKLOADCTL.LOAD needs to be written Other values are not supported."]
    #[inline(always)]
    pub const fn set_RATIO(&mut self, val: super::vals::GPTCLKDIV_RATIO) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
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
impl Default for GPTCLKDIV {
    #[inline(always)]
    fn default() -> GPTCLKDIV {
        GPTCLKDIV(0)
    }
}
impl core::fmt::Debug for GPTCLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPTCLKDIV")
            .field("RATIO", &self.RATIO())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPTCLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPTCLKDIV {{ RATIO: {:?}, RESERVED4: {=u32:?} }}",
            self.RATIO(),
            self.RESERVED4()
        )
    }
}
#[doc = "GPT Clock Gate For Deep Sleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPTCLKGDS(pub u32);
impl GPTCLKGDS {
    #[doc = "3:0\\] Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> super::vals::GPTCLKGDS_CLK_EN {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::GPTCLKGDS_CLK_EN::from_bits(val as u8)
    }
    #[doc = "3:0\\] Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: super::vals::GPTCLKGDS_CLK_EN) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
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
impl Default for GPTCLKGDS {
    #[inline(always)]
    fn default() -> GPTCLKGDS {
        GPTCLKGDS(0)
    }
}
impl core::fmt::Debug for GPTCLKGDS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPTCLKGDS")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPTCLKGDS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPTCLKGDS {{ CLK_EN: {:?}, RESERVED4: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED4()
        )
    }
}
#[doc = "GPT Clock Gate For Run Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPTCLKGR(pub u32);
impl GPTCLKGR {
    #[doc = "3:0\\] Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> super::vals::GPTCLKGR_CLK_EN {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::GPTCLKGR_CLK_EN::from_bits(val as u8)
    }
    #[doc = "3:0\\] Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: super::vals::GPTCLKGR_CLK_EN) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
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
impl Default for GPTCLKGR {
    #[inline(always)]
    fn default() -> GPTCLKGR {
        GPTCLKGR(0)
    }
}
impl core::fmt::Debug for GPTCLKGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPTCLKGR")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPTCLKGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPTCLKGR {{ CLK_EN: {:?}, RESERVED4: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED4()
        )
    }
}
#[doc = "GPT Clock Gate For Sleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPTCLKGS(pub u32);
impl GPTCLKGS {
    #[doc = "3:0\\] Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> super::vals::GPTCLKGS_CLK_EN {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::GPTCLKGS_CLK_EN::from_bits(val as u8)
    }
    #[doc = "3:0\\] Each bit below has the following meaning: 0: Disable clock 1: Enable clock ENUMs can be combined For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: super::vals::GPTCLKGS_CLK_EN) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
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
impl Default for GPTCLKGS {
    #[inline(always)]
    fn default() -> GPTCLKGS {
        GPTCLKGS(0)
    }
}
impl core::fmt::Debug for GPTCLKGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPTCLKGS")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPTCLKGS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPTCLKGS {{ CLK_EN: {:?}, RESERVED4: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED4()
        )
    }
}
#[doc = "I2C Clock Gate For Deep Sleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CCLKGDS(pub u32);
impl I2CCLKGDS {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: bool) {
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
impl Default for I2CCLKGDS {
    #[inline(always)]
    fn default() -> I2CCLKGDS {
        I2CCLKGDS(0)
    }
}
impl core::fmt::Debug for I2CCLKGDS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2CCLKGDS")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2CCLKGDS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2CCLKGDS {{ CLK_EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "I2C Clock Gate For Run Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CCLKGR(pub u32);
impl I2CCLKGR {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: bool) {
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
impl Default for I2CCLKGR {
    #[inline(always)]
    fn default() -> I2CCLKGR {
        I2CCLKGR(0)
    }
}
impl core::fmt::Debug for I2CCLKGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2CCLKGR")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2CCLKGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2CCLKGR {{ CLK_EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "I2C Clock Gate For Sleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2CCLKGS(pub u32);
impl I2CCLKGS {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: bool) {
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
impl Default for I2CCLKGS {
    #[inline(always)]
    fn default() -> I2CCLKGS {
        I2CCLKGS(0)
    }
}
impl core::fmt::Debug for I2CCLKGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2CCLKGS")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2CCLKGS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2CCLKGS {{ CLK_EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "BCLK Division Ratio."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2SBCLKDIV(pub u32);
impl I2SBCLKDIV {
    #[doc = "9:0\\] An unsigned factor of the division ratio used to generate I2S BCLK \\[2-1024\\]: BCLK = MCUCLK/BDIV\\[Hz\\] MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC A value of 0 is interpreted as 1024. A value of 1 is invalid. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 0, the low phase of the clock is one MCUCLK period longer than the high phase. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 1 , the high phase of the clock is one MCUCLK period longer than the low phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn BDIV(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "9:0\\] An unsigned factor of the division ratio used to generate I2S BCLK \\[2-1024\\]: BCLK = MCUCLK/BDIV\\[Hz\\] MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC A value of 0 is interpreted as 1024. A value of 1 is invalid. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 0, the low phase of the clock is one MCUCLK period longer than the high phase. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 1 , the high phase of the clock is one MCUCLK period longer than the low phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_BDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "31:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED10(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "31:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED10(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for I2SBCLKDIV {
    #[inline(always)]
    fn default() -> I2SBCLKDIV {
        I2SBCLKDIV(0)
    }
}
impl core::fmt::Debug for I2SBCLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SBCLKDIV")
            .field("BDIV", &self.BDIV())
            .field("RESERVED10", &self.RESERVED10())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2SBCLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2SBCLKDIV {{ BDIV: {=u16:?}, RESERVED10: {=u32:?} }}",
            self.BDIV(),
            self.RESERVED10()
        )
    }
}
#[doc = "I2S Clock Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2SBCLKSEL(pub u32);
impl I2SBCLKSEL {
    #[doc = "0:0\\] BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn SRC(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_SRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_SPARE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for I2SBCLKSEL {
    #[inline(always)]
    fn default() -> I2SBCLKSEL {
        I2SBCLKSEL(0)
    }
}
impl core::fmt::Debug for I2SBCLKSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SBCLKSEL")
            .field("SRC", &self.SRC())
            .field("SPARE", &self.SPARE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2SBCLKSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2SBCLKSEL {{ SRC: {=bool:?}, SPARE: {=u32:?} }}",
            self.SRC(),
            self.SPARE()
        )
    }
}
#[doc = "I2S Clock Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2SCLKCTL(pub u32);
impl I2SCLKCTL {
    #[doc = "0:0\\] 0: MCLK, BCLK and WCLK will be static low 1: Enables the generation of MCLK, BCLK and WCLK For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: MCLK, BCLK and WCLK will be static low 1: Enables the generation of MCLK, BCLK and WCLK For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "2:1\\] Decides how the WCLK division ratio is calculated and used to generate different duty cycles (See I2SWCLKDIV.WDIV). 0: Single phase 1: Dual phase 2: User Defined 3: Reserved/Undefined For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn WCLK_PHASE(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "2:1\\] Decides how the WCLK division ratio is calculated and used to generate different duty cycles (See I2SWCLKDIV.WDIV). 0: Single phase 1: Dual phase 2: User Defined 3: Reserved/Undefined For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_WCLK_PHASE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "3:3\\] On the I2S serial interface, data and WCLK is sampled and clocked out on opposite edges of BCLK. 0 - data and WCLK are sampled on the negative edge and clocked out on the positive edge. 1 - data and WCLK are sampled on the positive edge and clocked out on the negative edge. For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn SMPL_ON_POSEDGE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] On the I2S serial interface, data and WCLK is sampled and clocked out on opposite edges of BCLK. 0 - data and WCLK are sampled on the negative edge and clocked out on the positive edge. 1 - data and WCLK are sampled on the positive edge and clocked out on the negative edge. For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_SMPL_ON_POSEDGE(&mut self, val: bool) {
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
impl Default for I2SCLKCTL {
    #[inline(always)]
    fn default() -> I2SCLKCTL {
        I2SCLKCTL(0)
    }
}
impl core::fmt::Debug for I2SCLKCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SCLKCTL")
            .field("EN", &self.EN())
            .field("WCLK_PHASE", &self.WCLK_PHASE())
            .field("SMPL_ON_POSEDGE", &self.SMPL_ON_POSEDGE())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2SCLKCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2SCLKCTL {{ EN: {=bool:?}, WCLK_PHASE: {=u8:?}, SMPL_ON_POSEDGE: {=bool:?}, RESERVED4: {=u32:?} }}",
            self.EN(),
            self.WCLK_PHASE(),
            self.SMPL_ON_POSEDGE(),
            self.RESERVED4()
        )
    }
}
#[doc = "I2S Clock Gate For Deep Sleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2SCLKGDS(pub u32);
impl I2SCLKGDS {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for I2SCLKGDS {
    #[inline(always)]
    fn default() -> I2SCLKGDS {
        I2SCLKGDS(0)
    }
}
impl core::fmt::Debug for I2SCLKGDS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SCLKGDS")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2SCLKGDS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2SCLKGDS {{ CLK_EN: {=bool:?}, RESERVED: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED()
        )
    }
}
#[doc = "I2S Clock Gate For Run Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2SCLKGR(pub u32);
impl I2SCLKGR {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for I2SCLKGR {
    #[inline(always)]
    fn default() -> I2SCLKGR {
        I2SCLKGR(0)
    }
}
impl core::fmt::Debug for I2SCLKGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SCLKGR")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2SCLKGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2SCLKGR {{ CLK_EN: {=bool:?}, RESERVED: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED()
        )
    }
}
#[doc = "I2S Clock Gate For Sleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2SCLKGS(pub u32);
impl I2SCLKGS {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for I2SCLKGS {
    #[inline(always)]
    fn default() -> I2SCLKGS {
        I2SCLKGS(0)
    }
}
impl core::fmt::Debug for I2SCLKGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SCLKGS")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2SCLKGS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2SCLKGS {{ CLK_EN: {=bool:?}, RESERVED: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED()
        )
    }
}
#[doc = "MCLK Division Ratio."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2SMCLKDIV(pub u32);
impl I2SMCLKDIV {
    #[doc = "9:0\\] An unsigned factor of the division ratio used to generate MCLK \\[2-1024\\]: MCLK = MCUCLK/MDIV\\[Hz\\] MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC A value of 0 is interpreted as 1024. A value of 1 is invalid. If MDIV is odd the low phase of the clock is one MCUCLK period longer than the high phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn MDIV(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "9:0\\] An unsigned factor of the division ratio used to generate MCLK \\[2-1024\\]: MCLK = MCUCLK/MDIV\\[Hz\\] MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC A value of 0 is interpreted as 1024. A value of 1 is invalid. If MDIV is odd the low phase of the clock is one MCUCLK period longer than the high phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_MDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "31:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED10(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "31:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED10(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for I2SMCLKDIV {
    #[inline(always)]
    fn default() -> I2SMCLKDIV {
        I2SMCLKDIV(0)
    }
}
impl core::fmt::Debug for I2SMCLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SMCLKDIV")
            .field("MDIV", &self.MDIV())
            .field("RESERVED10", &self.RESERVED10())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2SMCLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2SMCLKDIV {{ MDIV: {=u16:?}, RESERVED10: {=u32:?} }}",
            self.MDIV(),
            self.RESERVED10()
        )
    }
}
#[doc = "WCLK Division Ratio."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2SWCLKDIV(pub u32);
impl I2SWCLKDIV {
    #[doc = "15:0\\] If I2SCLKCTL.WCLK_PHASE = 0, Single phase. WCLK is high one BCLK period and low WDIV\\[9:0\\] (unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(WDIV\\[9:0\\] + 1) \\[Hz\\] MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC If I2SCLKCTL.WCLK_PHASE = 1, Dual phase. Each phase on WCLK (50% duty cycle) is WDIV\\[9:0\\] (unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(2*WDIV\\[9:0\\]) \\[Hz\\] If I2SCLKCTL.WCLK_PHASE = 2, User defined. WCLK is high WDIV\\[7:0\\] (unsigned, \\[1-255\\]) BCLK periods and low WDIV\\[15:8\\] (unsigned, \\[1-255\\]) BCLK periods. WCLK = MCUCLK / (BDIV*(WDIV\\[7:0\\] + WDIV\\[15:8\\]) \\[Hz\\] For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn WDIV(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] If I2SCLKCTL.WCLK_PHASE = 0, Single phase. WCLK is high one BCLK period and low WDIV\\[9:0\\] (unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(WDIV\\[9:0\\] + 1) \\[Hz\\] MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC If I2SCLKCTL.WCLK_PHASE = 1, Dual phase. Each phase on WCLK (50% duty cycle) is WDIV\\[9:0\\] (unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(2*WDIV\\[9:0\\]) \\[Hz\\] If I2SCLKCTL.WCLK_PHASE = 2, User defined. WCLK is high WDIV\\[7:0\\] (unsigned, \\[1-255\\]) BCLK periods and low WDIV\\[15:8\\] (unsigned, \\[1-255\\]) BCLK periods. WCLK = MCUCLK / (BDIV*(WDIV\\[7:0\\] + WDIV\\[15:8\\]) \\[Hz\\] For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_WDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for I2SWCLKDIV {
    #[inline(always)]
    fn default() -> I2SWCLKDIV {
        I2SWCLKDIV(0)
    }
}
impl core::fmt::Debug for I2SWCLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SWCLKDIV")
            .field("WDIV", &self.WDIV())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2SWCLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2SWCLKDIV {{ WDIV: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.WDIV(),
            self.RESERVED16()
        )
    }
}
#[doc = "Infrastructure Clock Division Factor For DeepSleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INFRCLKDIVDS(pub u32);
impl INFRCLKDIVDS {
    #[doc = "1:0\\] Division rate for clocks driving modules in the MCU_AON domain when system CPU is in seepsleep mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[must_use]
    #[inline(always)]
    pub const fn RATIO(&self) -> super::vals::INFRCLKDIVDS_RATIO {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::INFRCLKDIVDS_RATIO::from_bits(val as u8)
    }
    #[doc = "1:0\\] Division rate for clocks driving modules in the MCU_AON domain when system CPU is in seepsleep mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[inline(always)]
    pub const fn set_RATIO(&mut self, val: super::vals::INFRCLKDIVDS_RATIO) {
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
impl Default for INFRCLKDIVDS {
    #[inline(always)]
    fn default() -> INFRCLKDIVDS {
        INFRCLKDIVDS(0)
    }
}
impl core::fmt::Debug for INFRCLKDIVDS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFRCLKDIVDS")
            .field("RATIO", &self.RATIO())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INFRCLKDIVDS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INFRCLKDIVDS {{ RATIO: {:?}, RESERVED2: {=u32:?} }}",
            self.RATIO(),
            self.RESERVED2()
        )
    }
}
#[doc = "Infrastructure Clock Division Factor For Run Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INFRCLKDIVR(pub u32);
impl INFRCLKDIVR {
    #[doc = "1:0\\] Division rate for clocks driving modules in the MCU_AON domain when system CPU is in run mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[must_use]
    #[inline(always)]
    pub const fn RATIO(&self) -> super::vals::INFRCLKDIVR_RATIO {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::INFRCLKDIVR_RATIO::from_bits(val as u8)
    }
    #[doc = "1:0\\] Division rate for clocks driving modules in the MCU_AON domain when system CPU is in run mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[inline(always)]
    pub const fn set_RATIO(&mut self, val: super::vals::INFRCLKDIVR_RATIO) {
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
impl Default for INFRCLKDIVR {
    #[inline(always)]
    fn default() -> INFRCLKDIVR {
        INFRCLKDIVR(0)
    }
}
impl core::fmt::Debug for INFRCLKDIVR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFRCLKDIVR")
            .field("RATIO", &self.RATIO())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INFRCLKDIVR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INFRCLKDIVR {{ RATIO: {:?}, RESERVED2: {=u32:?} }}",
            self.RATIO(),
            self.RESERVED2()
        )
    }
}
#[doc = "Infrastructure Clock Division Factor For Sleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INFRCLKDIVS(pub u32);
impl INFRCLKDIVS {
    #[doc = "1:0\\] Division rate for clocks driving modules in the MCU_AON domain when system CPU is in sleep mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[must_use]
    #[inline(always)]
    pub const fn RATIO(&self) -> super::vals::INFRCLKDIVS_RATIO {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::INFRCLKDIVS_RATIO::from_bits(val as u8)
    }
    #[doc = "1:0\\] Division rate for clocks driving modules in the MCU_AON domain when system CPU is in sleep mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[inline(always)]
    pub const fn set_RATIO(&mut self, val: super::vals::INFRCLKDIVS_RATIO) {
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
impl Default for INFRCLKDIVS {
    #[inline(always)]
    fn default() -> INFRCLKDIVS {
        INFRCLKDIVS(0)
    }
}
impl core::fmt::Debug for INFRCLKDIVS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFRCLKDIVS")
            .field("RATIO", &self.RATIO())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INFRCLKDIVS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INFRCLKDIVS {{ RATIO: {:?}, RESERVED2: {=u32:?} }}",
            self.RATIO(),
            self.RESERVED2()
        )
    }
}
#[doc = "Power Domain Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDCTL0(pub u32);
impl PDCTL0 {
    #[doc = "0:0\\] 0: RFC power domain powered off if also PDCTL1.RFC_ON = 0 1: RFC power domain powered on."]
    #[must_use]
    #[inline(always)]
    pub const fn RFC_ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: RFC power domain powered off if also PDCTL1.RFC_ON = 0 1: RFC power domain powered on."]
    #[inline(always)]
    pub const fn set_RFC_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] SERIAL Power domain. 0: SERIAL power domain is powered down 1: SERIAL power domain is powered up."]
    #[must_use]
    #[inline(always)]
    pub const fn SERIAL_ON(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] SERIAL Power domain. 0: SERIAL power domain is powered down 1: SERIAL power domain is powered up."]
    #[inline(always)]
    pub const fn set_SERIAL_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] PERIPH Power domain. 0: PERIPH power domain is powered down 1: PERIPH power domain is powered up."]
    #[must_use]
    #[inline(always)]
    pub const fn PERIPH_ON(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] PERIPH Power domain. 0: PERIPH power domain is powered down 1: PERIPH power domain is powered up."]
    #[inline(always)]
    pub const fn set_PERIPH_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
impl Default for PDCTL0 {
    #[inline(always)]
    fn default() -> PDCTL0 {
        PDCTL0(0)
    }
}
impl core::fmt::Debug for PDCTL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCTL0")
            .field("RFC_ON", &self.RFC_ON())
            .field("SERIAL_ON", &self.SERIAL_ON())
            .field("PERIPH_ON", &self.PERIPH_ON())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDCTL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDCTL0 {{ RFC_ON: {=bool:?}, SERIAL_ON: {=bool:?}, PERIPH_ON: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.RFC_ON(),
            self.SERIAL_ON(),
            self.PERIPH_ON(),
            self.RESERVED3()
        )
    }
}
#[doc = "PERIPH Power Domain Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDCTL0PERIPH(pub u32);
impl PDCTL0PERIPH {
    #[doc = "0:0\\] Alias for PDCTL0.PERIPH_ON."]
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Alias for PDCTL0.PERIPH_ON."]
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
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
impl Default for PDCTL0PERIPH {
    #[inline(always)]
    fn default() -> PDCTL0PERIPH {
        PDCTL0PERIPH(0)
    }
}
impl core::fmt::Debug for PDCTL0PERIPH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCTL0PERIPH")
            .field("ON", &self.ON())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDCTL0PERIPH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDCTL0PERIPH {{ ON: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.ON(),
            self.RESERVED1()
        )
    }
}
#[doc = "RFC Power Domain Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDCTL0RFC(pub u32);
impl PDCTL0RFC {
    #[doc = "0:0\\] Alias for PDCTL0.RFC_ON."]
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Alias for PDCTL0.RFC_ON."]
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
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
impl Default for PDCTL0RFC {
    #[inline(always)]
    fn default() -> PDCTL0RFC {
        PDCTL0RFC(0)
    }
}
impl core::fmt::Debug for PDCTL0RFC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCTL0RFC")
            .field("ON", &self.ON())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDCTL0RFC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDCTL0RFC {{ ON: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.ON(),
            self.RESERVED1()
        )
    }
}
#[doc = "SERIAL Power Domain Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDCTL0SERIAL(pub u32);
impl PDCTL0SERIAL {
    #[doc = "0:0\\] Alias for PDCTL0.SERIAL_ON."]
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Alias for PDCTL0.SERIAL_ON."]
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
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
impl Default for PDCTL0SERIAL {
    #[inline(always)]
    fn default() -> PDCTL0SERIAL {
        PDCTL0SERIAL(0)
    }
}
impl core::fmt::Debug for PDCTL0SERIAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCTL0SERIAL")
            .field("ON", &self.ON())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDCTL0SERIAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDCTL0SERIAL {{ ON: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.ON(),
            self.RESERVED1()
        )
    }
}
#[doc = "Power Domain Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDCTL1(pub u32);
impl PDCTL1 {
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
    #[doc = "1:1\\] 0: Causes a power down of the CPU power domain when system CPU indicates it is idle. 1: Initiates power-on of the CPU power domain. This bit is automatically set by a WIC power-on event."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU_ON(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 0: Causes a power down of the CPU power domain when system CPU indicates it is idle. 1: Initiates power-on of the CPU power domain. This bit is automatically set by a WIC power-on event."]
    #[inline(always)]
    pub const fn set_CPU_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] 0: RFC power domain powered off if also PDCTL0.RFC_ON = 0 1: RFC power domain powered on Bit shall be used by RFC in autonomus mode but there is no HW restrictions fom system CPU to access the bit."]
    #[must_use]
    #[inline(always)]
    pub const fn RFC_ON(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] 0: RFC power domain powered off if also PDCTL0.RFC_ON = 0 1: RFC power domain powered on Bit shall be used by RFC in autonomus mode but there is no HW restrictions fom system CPU to access the bit."]
    #[inline(always)]
    pub const fn set_RFC_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] 0: VIMS power domain is only powered when CPU power domain is powered. 1: VIMS power domain is powered whenever the BUS power domain is powered."]
    #[must_use]
    #[inline(always)]
    pub const fn VIMS_MODE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] 0: VIMS power domain is only powered when CPU power domain is powered. 1: VIMS power domain is powered whenever the BUS power domain is powered."]
    #[inline(always)]
    pub const fn set_VIMS_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for PDCTL1 {
    #[inline(always)]
    fn default() -> PDCTL1 {
        PDCTL1(0)
    }
}
impl core::fmt::Debug for PDCTL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCTL1")
            .field("RESERVED0", &self.RESERVED0())
            .field("CPU_ON", &self.CPU_ON())
            .field("RFC_ON", &self.RFC_ON())
            .field("VIMS_MODE", &self.VIMS_MODE())
            .field("RESERVED4", &self.RESERVED4())
            .field("RESERVED5", &self.RESERVED5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDCTL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDCTL1 {{ RESERVED0: {=bool:?}, CPU_ON: {=bool:?}, RFC_ON: {=bool:?}, VIMS_MODE: {=bool:?}, RESERVED4: {=bool:?}, RESERVED5: {=u32:?} }}",
            self.RESERVED0(),
            self.CPU_ON(),
            self.RFC_ON(),
            self.VIMS_MODE(),
            self.RESERVED4(),
            self.RESERVED5()
        )
    }
}
#[doc = "CPU Power Domain Direct Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDCTL1CPU(pub u32);
impl PDCTL1CPU {
    #[doc = "0:0\\] This is an alias for PDCTL1.CPU_ON."]
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This is an alias for PDCTL1.CPU_ON."]
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
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
impl Default for PDCTL1CPU {
    #[inline(always)]
    fn default() -> PDCTL1CPU {
        PDCTL1CPU(0)
    }
}
impl core::fmt::Debug for PDCTL1CPU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCTL1CPU")
            .field("ON", &self.ON())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDCTL1CPU {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDCTL1CPU {{ ON: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.ON(),
            self.RESERVED1()
        )
    }
}
#[doc = "RFC Power Domain Direct Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDCTL1RFC(pub u32);
impl PDCTL1RFC {
    #[doc = "0:0\\] This is an alias for PDCTL1.RFC_ON."]
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This is an alias for PDCTL1.RFC_ON."]
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
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
impl Default for PDCTL1RFC {
    #[inline(always)]
    fn default() -> PDCTL1RFC {
        PDCTL1RFC(0)
    }
}
impl core::fmt::Debug for PDCTL1RFC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCTL1RFC")
            .field("ON", &self.ON())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDCTL1RFC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDCTL1RFC {{ ON: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.ON(),
            self.RESERVED1()
        )
    }
}
#[doc = "VIMS Mode Direct Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDCTL1VIMS(pub u32);
impl PDCTL1VIMS {
    #[doc = "0:0\\] This is an alias for PDCTL1.VIMS_MODE."]
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This is an alias for PDCTL1.VIMS_MODE."]
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
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
impl Default for PDCTL1VIMS {
    #[inline(always)]
    fn default() -> PDCTL1VIMS {
        PDCTL1VIMS(0)
    }
}
impl core::fmt::Debug for PDCTL1VIMS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCTL1VIMS")
            .field("ON", &self.ON())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDCTL1VIMS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDCTL1VIMS {{ ON: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.ON(),
            self.RESERVED1()
        )
    }
}
#[doc = "Power Domain Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDSTAT0(pub u32);
impl PDSTAT0 {
    #[doc = "0:0\\] RFC Power domain 0: Domain may be powered down 1: Domain powered up (guaranteed)."]
    #[must_use]
    #[inline(always)]
    pub const fn RFC_ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] RFC Power domain 0: Domain may be powered down 1: Domain powered up (guaranteed)."]
    #[inline(always)]
    pub const fn set_RFC_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] SERIAL Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)."]
    #[must_use]
    #[inline(always)]
    pub const fn SERIAL_ON(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] SERIAL Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)."]
    #[inline(always)]
    pub const fn set_SERIAL_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] PERIPH Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)."]
    #[must_use]
    #[inline(always)]
    pub const fn PERIPH_ON(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] PERIPH Power domain. 0: Domain may be powered down 1: Domain powered up (guaranteed)."]
    #[inline(always)]
    pub const fn set_PERIPH_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
impl Default for PDSTAT0 {
    #[inline(always)]
    fn default() -> PDSTAT0 {
        PDSTAT0(0)
    }
}
impl core::fmt::Debug for PDSTAT0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDSTAT0")
            .field("RFC_ON", &self.RFC_ON())
            .field("SERIAL_ON", &self.SERIAL_ON())
            .field("PERIPH_ON", &self.PERIPH_ON())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDSTAT0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDSTAT0 {{ RFC_ON: {=bool:?}, SERIAL_ON: {=bool:?}, PERIPH_ON: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.RFC_ON(),
            self.SERIAL_ON(),
            self.PERIPH_ON(),
            self.RESERVED3()
        )
    }
}
#[doc = "PERIPH Power Domain Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDSTAT0PERIPH(pub u32);
impl PDSTAT0PERIPH {
    #[doc = "0:0\\] Alias for PDSTAT0.PERIPH_ON."]
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Alias for PDSTAT0.PERIPH_ON."]
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
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
impl Default for PDSTAT0PERIPH {
    #[inline(always)]
    fn default() -> PDSTAT0PERIPH {
        PDSTAT0PERIPH(0)
    }
}
impl core::fmt::Debug for PDSTAT0PERIPH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDSTAT0PERIPH")
            .field("ON", &self.ON())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDSTAT0PERIPH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDSTAT0PERIPH {{ ON: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.ON(),
            self.RESERVED1()
        )
    }
}
#[doc = "RFC Power Domain Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDSTAT0RFC(pub u32);
impl PDSTAT0RFC {
    #[doc = "0:0\\] Alias for PDSTAT0.RFC_ON."]
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Alias for PDSTAT0.RFC_ON."]
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
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
impl Default for PDSTAT0RFC {
    #[inline(always)]
    fn default() -> PDSTAT0RFC {
        PDSTAT0RFC(0)
    }
}
impl core::fmt::Debug for PDSTAT0RFC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDSTAT0RFC")
            .field("ON", &self.ON())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDSTAT0RFC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDSTAT0RFC {{ ON: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.ON(),
            self.RESERVED1()
        )
    }
}
#[doc = "SERIAL Power Domain Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDSTAT0SERIAL(pub u32);
impl PDSTAT0SERIAL {
    #[doc = "0:0\\] Alias for PDSTAT0.SERIAL_ON."]
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Alias for PDSTAT0.SERIAL_ON."]
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
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
impl Default for PDSTAT0SERIAL {
    #[inline(always)]
    fn default() -> PDSTAT0SERIAL {
        PDSTAT0SERIAL(0)
    }
}
impl core::fmt::Debug for PDSTAT0SERIAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDSTAT0SERIAL")
            .field("ON", &self.ON())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDSTAT0SERIAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDSTAT0SERIAL {{ ON: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.ON(),
            self.RESERVED1()
        )
    }
}
#[doc = "Power Manager Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDSTAT1(pub u32);
impl PDSTAT1 {
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
    #[doc = "1:1\\] 0: CPU and BUS domain not accessible 1: CPU and BUS domains are both currently accessible."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU_ON(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 0: CPU and BUS domain not accessible 1: CPU and BUS domains are both currently accessible."]
    #[inline(always)]
    pub const fn set_CPU_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] 0: RFC domain not accessible 1: RFC domain is currently accessible."]
    #[must_use]
    #[inline(always)]
    pub const fn RFC_ON(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] 0: RFC domain not accessible 1: RFC domain is currently accessible."]
    #[inline(always)]
    pub const fn set_RFC_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] 0: VIMS domain not accessible 1: VIMS domain is currently accessible."]
    #[must_use]
    #[inline(always)]
    pub const fn VIMS_MODE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] 0: VIMS domain not accessible 1: VIMS domain is currently accessible."]
    #[inline(always)]
    pub const fn set_VIMS_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] 0: BUS domain not accessible 1: BUS domain is currently accessible."]
    #[must_use]
    #[inline(always)]
    pub const fn BUS_ON(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] 0: BUS domain not accessible 1: BUS domain is currently accessible."]
    #[inline(always)]
    pub const fn set_BUS_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for PDSTAT1 {
    #[inline(always)]
    fn default() -> PDSTAT1 {
        PDSTAT1(0)
    }
}
impl core::fmt::Debug for PDSTAT1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDSTAT1")
            .field("RESERVED0", &self.RESERVED0())
            .field("CPU_ON", &self.CPU_ON())
            .field("RFC_ON", &self.RFC_ON())
            .field("VIMS_MODE", &self.VIMS_MODE())
            .field("BUS_ON", &self.BUS_ON())
            .field("RESERVED5", &self.RESERVED5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDSTAT1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDSTAT1 {{ RESERVED0: {=bool:?}, CPU_ON: {=bool:?}, RFC_ON: {=bool:?}, VIMS_MODE: {=bool:?}, BUS_ON: {=bool:?}, RESERVED5: {=u32:?} }}",
            self.RESERVED0(),
            self.CPU_ON(),
            self.RFC_ON(),
            self.VIMS_MODE(),
            self.BUS_ON(),
            self.RESERVED5()
        )
    }
}
#[doc = "BUS Power Domain Direct Read Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDSTAT1BUS(pub u32);
impl PDSTAT1BUS {
    #[doc = "0:0\\] This is an alias for PDSTAT1.BUS_ON."]
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This is an alias for PDSTAT1.BUS_ON."]
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
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
impl Default for PDSTAT1BUS {
    #[inline(always)]
    fn default() -> PDSTAT1BUS {
        PDSTAT1BUS(0)
    }
}
impl core::fmt::Debug for PDSTAT1BUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDSTAT1BUS")
            .field("ON", &self.ON())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDSTAT1BUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDSTAT1BUS {{ ON: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.ON(),
            self.RESERVED1()
        )
    }
}
#[doc = "CPU Power Domain Direct Read Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDSTAT1CPU(pub u32);
impl PDSTAT1CPU {
    #[doc = "0:0\\] This is an alias for PDSTAT1.CPU_ON."]
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This is an alias for PDSTAT1.CPU_ON."]
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
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
impl Default for PDSTAT1CPU {
    #[inline(always)]
    fn default() -> PDSTAT1CPU {
        PDSTAT1CPU(0)
    }
}
impl core::fmt::Debug for PDSTAT1CPU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDSTAT1CPU")
            .field("ON", &self.ON())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDSTAT1CPU {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDSTAT1CPU {{ ON: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.ON(),
            self.RESERVED1()
        )
    }
}
#[doc = "RFC Power Domain Direct Read Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDSTAT1RFC(pub u32);
impl PDSTAT1RFC {
    #[doc = "0:0\\] This is an alias for PDSTAT1.RFC_ON."]
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This is an alias for PDSTAT1.RFC_ON."]
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
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
impl Default for PDSTAT1RFC {
    #[inline(always)]
    fn default() -> PDSTAT1RFC {
        PDSTAT1RFC(0)
    }
}
impl core::fmt::Debug for PDSTAT1RFC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDSTAT1RFC")
            .field("ON", &self.ON())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDSTAT1RFC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDSTAT1RFC {{ ON: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.ON(),
            self.RESERVED1()
        )
    }
}
#[doc = "VIMS Mode Direct Read Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PDSTAT1VIMS(pub u32);
impl PDSTAT1VIMS {
    #[doc = "0:0\\] This is an alias for PDSTAT1.VIMS_MODE."]
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This is an alias for PDSTAT1.VIMS_MODE."]
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
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
impl Default for PDSTAT1VIMS {
    #[inline(always)]
    fn default() -> PDSTAT1VIMS {
        PDSTAT1VIMS(0)
    }
}
impl core::fmt::Debug for PDSTAT1VIMS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDSTAT1VIMS")
            .field("ON", &self.ON())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PDSTAT1VIMS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PDSTAT1VIMS {{ ON: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.ON(),
            self.RESERVED1()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PERBUSDMACLKDIV(pub u32);
impl PERBUSDMACLKDIV {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SPARE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PERBUSDMACLKDIV {
    #[inline(always)]
    fn default() -> PERBUSDMACLKDIV {
        PERBUSDMACLKDIV(0)
    }
}
impl core::fmt::Debug for PERBUSDMACLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERBUSDMACLKDIV")
            .field("SPARE", &self.SPARE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PERBUSDMACLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PERBUSDMACLKDIV {{ SPARE: {=u32:?} }}", self.SPARE())
    }
}
#[doc = "Power Profiler Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWRPROFSTAT(pub u32);
impl PWRPROFSTAT {
    #[doc = "7:0\\] SW can use these bits to timestamp the application. These bits are also available through the testtap and can thus be used by the emulator to profile in real time."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] SW can use these bits to timestamp the application. These bits are also available through the testtap and can thus be used by the emulator to profile in real time."]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for PWRPROFSTAT {
    #[inline(always)]
    fn default() -> PWRPROFSTAT {
        PWRPROFSTAT(0)
    }
}
impl core::fmt::Debug for PWRPROFSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRPROFSTAT")
            .field("VALUE", &self.VALUE())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWRPROFSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWRPROFSTAT {{ VALUE: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.VALUE(),
            self.RESERVED8()
        )
    }
}
#[doc = "Memory Retention Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RAMRETEN(pub u32);
impl RAMRETEN {
    #[doc = "1:0\\] 0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions."]
    #[must_use]
    #[inline(always)]
    pub const fn VIMS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\] 0: Memory retention disabled 1: Memory retention enabled Bit 0: VIMS_TRAM Bit 1: VIMS_CRAM Legal modes depend on settings in VIMS:CTL.MODE 00: VIMS:CTL.MODE must be OFF before DEEPSLEEP is asserted - must be set to CACHE or SPLIT mode after waking up again 01: VIMS:CTL.MODE must be GPRAM before DEEPSLEEP is asserted. Must remain in GPRAM mode after wake up, alternatively select OFF mode first and then CACHE or SPILT mode. 10: Illegal mode 11: No restrictions."]
    #[inline(always)]
    pub const fn set_VIMS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "2:2\\] 0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM."]
    #[must_use]
    #[inline(always)]
    pub const fn RFC(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] 0: Retention for RFC SRAM disabled 1: Retention for RFC SRAM enabled Memories controlled: CPERAM MCERAM RFERAM."]
    #[inline(always)]
    pub const fn set_RFC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
impl Default for RAMRETEN {
    #[inline(always)]
    fn default() -> RAMRETEN {
        RAMRETEN(0)
    }
}
impl core::fmt::Debug for RAMRETEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAMRETEN")
            .field("VIMS", &self.VIMS())
            .field("RFC", &self.RFC())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RAMRETEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RAMRETEN {{ VIMS: {=u8:?}, RFC: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.VIMS(),
            self.RFC(),
            self.RESERVED3()
        )
    }
}
#[doc = "Control To RFC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCBITS(pub u32);
impl RFCBITS {
    #[doc = "31:0\\] Control bits for RFC. The RF core CPE processor will automatically check this register when it boots, and it can be used to immediately instruct CPE to perform some tasks at its start-up. The supported functionality is ROM-defined and may vary. See the technical reference manual for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn READ(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Control bits for RFC. The RF core CPE processor will automatically check this register when it boots, and it can be used to immediately instruct CPE to perform some tasks at its start-up. The supported functionality is ROM-defined and may vary. See the technical reference manual for more details."]
    #[inline(always)]
    pub const fn set_READ(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RFCBITS {
    #[inline(always)]
    fn default() -> RFCBITS {
        RFCBITS(0)
    }
}
impl core::fmt::Debug for RFCBITS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCBITS")
            .field("READ", &self.READ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCBITS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RFCBITS {{ READ: {=u32:?} }}", self.READ())
    }
}
#[doc = "RFC Clock Gate."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCCLKG(pub u32);
impl RFCCLKG {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock if RFC power domain is on For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock if RFC power domain is on For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: bool) {
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
impl Default for RFCCLKG {
    #[inline(always)]
    fn default() -> RFCCLKG {
        RFCCLKG(0)
    }
}
impl core::fmt::Debug for RFCCLKG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCCLKG")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCCLKG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCCLKG {{ CLK_EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "Allowed RFC Modes."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCMODEHWOPT(pub u32);
impl RFCMODEHWOPT {
    #[doc = "7:0\\] Permitted RFC modes. More than one mode can be permitted."]
    #[must_use]
    #[inline(always)]
    pub const fn AVAIL(&self) -> super::vals::AVAIL {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::AVAIL::from_bits(val as u8)
    }
    #[doc = "7:0\\] Permitted RFC modes. More than one mode can be permitted."]
    #[inline(always)]
    pub const fn set_AVAIL(&mut self, val: super::vals::AVAIL) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for RFCMODEHWOPT {
    #[inline(always)]
    fn default() -> RFCMODEHWOPT {
        RFCMODEHWOPT(0)
    }
}
impl core::fmt::Debug for RFCMODEHWOPT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCMODEHWOPT")
            .field("AVAIL", &self.AVAIL())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCMODEHWOPT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCMODEHWOPT {{ AVAIL: {:?}, RESERVED8: {=u32:?} }}",
            self.AVAIL(),
            self.RESERVED8()
        )
    }
}
#[doc = "Selected RFC Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCMODESEL(pub u32);
impl RFCMODESEL {
    #[doc = "2:0\\] Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writeable. See the technical reference manual for details."]
    #[must_use]
    #[inline(always)]
    pub const fn CURR(&self) -> super::vals::CURR {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::CURR::from_bits(val as u8)
    }
    #[doc = "2:0\\] Selects the set of commands that the RFC will accept. Only modes permitted by RFCMODEHWOPT.AVAIL are writeable. See the technical reference manual for details."]
    #[inline(always)]
    pub const fn set_CURR(&mut self, val: super::vals::CURR) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
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
impl Default for RFCMODESEL {
    #[inline(always)]
    fn default() -> RFCMODESEL {
        RFCMODESEL(0)
    }
}
impl core::fmt::Debug for RFCMODESEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCMODESEL")
            .field("CURR", &self.CURR())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCMODESEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCMODESEL {{ CURR: {:?}, RESERVED3: {=u32:?} }}",
            self.CURR(),
            self.RESERVED3()
        )
    }
}
#[doc = "TRNG, CRYPTO And UDMA Clock Gate For Deep Sleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SECDMACLKGDS(pub u32);
impl SECDMACLKGDS {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CRYPTO_CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CRYPTO_CLK_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn TRNG_CLK_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_TRNG_CLK_EN(&mut self, val: bool) {
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
    #[doc = "8:8\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_CLK_EN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_DMA_CLK_EN(&mut self, val: bool) {
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
impl Default for SECDMACLKGDS {
    #[inline(always)]
    fn default() -> SECDMACLKGDS {
        SECDMACLKGDS(0)
    }
}
impl core::fmt::Debug for SECDMACLKGDS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECDMACLKGDS")
            .field("CRYPTO_CLK_EN", &self.CRYPTO_CLK_EN())
            .field("TRNG_CLK_EN", &self.TRNG_CLK_EN())
            .field("RESERVED2", &self.RESERVED2())
            .field("DMA_CLK_EN", &self.DMA_CLK_EN())
            .field("RESERVED9", &self.RESERVED9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SECDMACLKGDS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SECDMACLKGDS {{ CRYPTO_CLK_EN: {=bool:?}, TRNG_CLK_EN: {=bool:?}, RESERVED2: {=u8:?}, DMA_CLK_EN: {=bool:?}, RESERVED9: {=u32:?} }}",
            self.CRYPTO_CLK_EN(),
            self.TRNG_CLK_EN(),
            self.RESERVED2(),
            self.DMA_CLK_EN(),
            self.RESERVED9()
        )
    }
}
#[doc = "TRNG, CRYPTO And UDMA Clock Gate For Run Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SECDMACLKGR(pub u32);
impl SECDMACLKGR {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CRYPTO_CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CRYPTO_CLK_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn TRNG_CLK_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_TRNG_CLK_EN(&mut self, val: bool) {
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
    #[doc = "8:8\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_CLK_EN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_DMA_CLK_EN(&mut self, val: bool) {
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
impl Default for SECDMACLKGR {
    #[inline(always)]
    fn default() -> SECDMACLKGR {
        SECDMACLKGR(0)
    }
}
impl core::fmt::Debug for SECDMACLKGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECDMACLKGR")
            .field("CRYPTO_CLK_EN", &self.CRYPTO_CLK_EN())
            .field("TRNG_CLK_EN", &self.TRNG_CLK_EN())
            .field("RESERVED2", &self.RESERVED2())
            .field("DMA_CLK_EN", &self.DMA_CLK_EN())
            .field("RESERVED9", &self.RESERVED9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SECDMACLKGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SECDMACLKGR {{ CRYPTO_CLK_EN: {=bool:?}, TRNG_CLK_EN: {=bool:?}, RESERVED2: {=u8:?}, DMA_CLK_EN: {=bool:?}, RESERVED9: {=u32:?} }}",
            self.CRYPTO_CLK_EN(),
            self.TRNG_CLK_EN(),
            self.RESERVED2(),
            self.DMA_CLK_EN(),
            self.RESERVED9()
        )
    }
}
#[doc = "TRNG, CRYPTO And UDMA Clock Gate For Sleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SECDMACLKGS(pub u32);
impl SECDMACLKGS {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CRYPTO_CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CRYPTO_CLK_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn TRNG_CLK_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_TRNG_CLK_EN(&mut self, val: bool) {
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
    #[doc = "8:8\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_CLK_EN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_DMA_CLK_EN(&mut self, val: bool) {
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
impl Default for SECDMACLKGS {
    #[inline(always)]
    fn default() -> SECDMACLKGS {
        SECDMACLKGS(0)
    }
}
impl core::fmt::Debug for SECDMACLKGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECDMACLKGS")
            .field("CRYPTO_CLK_EN", &self.CRYPTO_CLK_EN())
            .field("TRNG_CLK_EN", &self.TRNG_CLK_EN())
            .field("RESERVED2", &self.RESERVED2())
            .field("DMA_CLK_EN", &self.DMA_CLK_EN())
            .field("RESERVED9", &self.RESERVED9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SECDMACLKGS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SECDMACLKGS {{ CRYPTO_CLK_EN: {=bool:?}, TRNG_CLK_EN: {=bool:?}, RESERVED2: {=u8:?}, DMA_CLK_EN: {=bool:?}, RESERVED9: {=u32:?} }}",
            self.CRYPTO_CLK_EN(),
            self.TRNG_CLK_EN(),
            self.RESERVED2(),
            self.DMA_CLK_EN(),
            self.RESERVED9()
        )
    }
}
#[doc = "SSI Clock Gate For Deep Sleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSICLKGDS(pub u32);
impl SSICLKGDS {
    #[doc = "1:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> super::vals::SSICLKGDS_CLK_EN {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SSICLKGDS_CLK_EN::from_bits(val as u8)
    }
    #[doc = "1:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: super::vals::SSICLKGDS_CLK_EN) {
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
impl Default for SSICLKGDS {
    #[inline(always)]
    fn default() -> SSICLKGDS {
        SSICLKGDS(0)
    }
}
impl core::fmt::Debug for SSICLKGDS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSICLKGDS")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSICLKGDS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SSICLKGDS {{ CLK_EN: {:?}, RESERVED2: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED2()
        )
    }
}
#[doc = "SSI Clock Gate For Run Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSICLKGR(pub u32);
impl SSICLKGR {
    #[doc = "1:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> super::vals::SSICLKGR_CLK_EN {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SSICLKGR_CLK_EN::from_bits(val as u8)
    }
    #[doc = "1:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: super::vals::SSICLKGR_CLK_EN) {
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
impl Default for SSICLKGR {
    #[inline(always)]
    fn default() -> SSICLKGR {
        SSICLKGR(0)
    }
}
impl core::fmt::Debug for SSICLKGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSICLKGR")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSICLKGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SSICLKGR {{ CLK_EN: {:?}, RESERVED2: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED2()
        )
    }
}
#[doc = "SSI Clock Gate For Sleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSICLKGS(pub u32);
impl SSICLKGS {
    #[doc = "1:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> super::vals::SSICLKGS_CLK_EN {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SSICLKGS_CLK_EN::from_bits(val as u8)
    }
    #[doc = "1:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: super::vals::SSICLKGS_CLK_EN) {
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
impl Default for SSICLKGS {
    #[inline(always)]
    fn default() -> SSICLKGS {
        SSICLKGS(0)
    }
}
impl core::fmt::Debug for SSICLKGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSICLKGS")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSICLKGS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SSICLKGS {{ CLK_EN: {:?}, RESERVED2: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED2()
        )
    }
}
#[doc = "SW Initiated Resets."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SWRESET(pub u32);
impl SWRESET {
    #[doc = "1:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MCU(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MCU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
impl Default for SWRESET {
    #[inline(always)]
    fn default() -> SWRESET {
        SWRESET(0)
    }
}
impl core::fmt::Debug for SWRESET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWRESET")
            .field("RESERVED0", &self.RESERVED0())
            .field("MCU", &self.MCU())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SWRESET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SWRESET {{ RESERVED0: {=u8:?}, MCU: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.RESERVED0(),
            self.MCU(),
            self.RESERVED3()
        )
    }
}
#[doc = "UART Clock Gate For Deep Sleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UARTCLKGDS(pub u32);
impl UARTCLKGDS {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: bool) {
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
impl Default for UARTCLKGDS {
    #[inline(always)]
    fn default() -> UARTCLKGDS {
        UARTCLKGDS(0)
    }
}
impl core::fmt::Debug for UARTCLKGDS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UARTCLKGDS")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UARTCLKGDS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UARTCLKGDS {{ CLK_EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "UART Clock Gate For Run Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UARTCLKGR(pub u32);
impl UARTCLKGR {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: bool) {
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
impl Default for UARTCLKGR {
    #[inline(always)]
    fn default() -> UARTCLKGR {
        UARTCLKGR(0)
    }
}
impl core::fmt::Debug for UARTCLKGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UARTCLKGR")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UARTCLKGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UARTCLKGR {{ CLK_EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "UART Clock Gate For Sleep Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UARTCLKGS(pub u32);
impl UARTCLKGS {
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: bool) {
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
impl Default for UARTCLKGS {
    #[inline(always)]
    fn default() -> UARTCLKGS {
        UARTCLKGS(0)
    }
}
impl core::fmt::Debug for UARTCLKGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UARTCLKGS")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UARTCLKGS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UARTCLKGS {{ CLK_EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "MCU Voltage Domain Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VDCTL(pub u32);
impl VDCTL {
    #[doc = "0:0\\] Request WUC to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep."]
    #[must_use]
    #[inline(always)]
    pub const fn ULDO(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Request WUC to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep."]
    #[inline(always)]
    pub const fn set_ULDO(&mut self, val: bool) {
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
    #[doc = "2:2\\] Request WUC to power down the MCU voltage domain 0: No request 1: Assert request when possible. An asserted power down request will result in a boot of the MCU system when powered up again. The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep."]
    #[must_use]
    #[inline(always)]
    pub const fn MCU_VD(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Request WUC to power down the MCU voltage domain 0: No request 1: Assert request when possible. An asserted power down request will result in a boot of the MCU system when powered up again. The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep."]
    #[inline(always)]
    pub const fn set_MCU_VD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
impl Default for VDCTL {
    #[inline(always)]
    fn default() -> VDCTL {
        VDCTL(0)
    }
}
impl core::fmt::Debug for VDCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDCTL")
            .field("ULDO", &self.ULDO())
            .field("RESERVED1", &self.RESERVED1())
            .field("MCU_VD", &self.MCU_VD())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VDCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VDCTL {{ ULDO: {=bool:?}, RESERVED1: {=bool:?}, MCU_VD: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.ULDO(),
            self.RESERVED1(),
            self.MCU_VD(),
            self.RESERVED3()
        )
    }
}
#[doc = "VIMS Clock Gate."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VIMSCLKG(pub u32);
impl VIMSCLKG {
    #[doc = "1:0\\] 00: Disable clock 01: Disable clock when system CPU is in DeepSleep 11: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_EN(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\] 00: Disable clock 01: Disable clock when system CPU is in DeepSleep 11: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written."]
    #[inline(always)]
    pub const fn set_CLK_EN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
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
impl Default for VIMSCLKG {
    #[inline(always)]
    fn default() -> VIMSCLKG {
        VIMSCLKG(0)
    }
}
impl core::fmt::Debug for VIMSCLKG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VIMSCLKG")
            .field("CLK_EN", &self.CLK_EN())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VIMSCLKG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VIMSCLKG {{ CLK_EN: {=u8:?}, RESERVED2: {=u32:?} }}",
            self.CLK_EN(),
            self.RESERVED2()
        )
    }
}
#[doc = "WARM Reset Control And Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WARMRESET(pub u32);
impl WARMRESET {
    #[doc = "0:0\\] 0: No registered event 1: A WDT event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[must_use]
    #[inline(always)]
    pub const fn WDT_STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: No registered event 1: A WDT event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[inline(always)]
    pub const fn set_WDT_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 0: No registred event 1: A system CPU LOCKUP event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCKUP_STAT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 0: No registred event 1: A system CPU LOCKUP event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[inline(always)]
    pub const fn set_LOCKUP_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] 0: No action 1: A warm system reset event triggered by the below listed sources will result in an emulated pin reset. Warm reset sources included: ICEPick sysreset System CPU reset request, CPU_SCS:AIRCR.SYSRESETREQ System CPU Lockup WDT timeout An active ICEPick block system reset will gate all sources except ICEPick sysreset SW can read AON_SYSCTL:RESETCTL.RESET_SRC to find the source of the last reset resulting in a full power up sequence. WARMRESET in this register is set in the scenario that WR_TO_PINRESET=1 and one of the above listed sources is triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn WR_TO_PINRESET(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] 0: No action 1: A warm system reset event triggered by the below listed sources will result in an emulated pin reset. Warm reset sources included: ICEPick sysreset System CPU reset request, CPU_SCS:AIRCR.SYSRESETREQ System CPU Lockup WDT timeout An active ICEPick block system reset will gate all sources except ICEPick sysreset SW can read AON_SYSCTL:RESETCTL.RESET_SRC to find the source of the last reset resulting in a full power up sequence. WARMRESET in this register is set in the scenario that WR_TO_PINRESET=1 and one of the above listed sources is triggered."]
    #[inline(always)]
    pub const fn set_WR_TO_PINRESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
impl Default for WARMRESET {
    #[inline(always)]
    fn default() -> WARMRESET {
        WARMRESET(0)
    }
}
impl core::fmt::Debug for WARMRESET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WARMRESET")
            .field("WDT_STAT", &self.WDT_STAT())
            .field("LOCKUP_STAT", &self.LOCKUP_STAT())
            .field("WR_TO_PINRESET", &self.WR_TO_PINRESET())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WARMRESET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WARMRESET {{ WDT_STAT: {=bool:?}, LOCKUP_STAT: {=bool:?}, WR_TO_PINRESET: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.WDT_STAT(),
            self.LOCKUP_STAT(),
            self.WR_TO_PINRESET(),
            self.RESERVED3()
        )
    }
}

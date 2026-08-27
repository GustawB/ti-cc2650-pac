#[doc = "Power Management This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWRCTL(pub u32);
impl PWRCTL {
    #[doc = "0:0\\] Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDC_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE."]
    #[inline(always)]
    pub const fn set_DCDC_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Status of source for VDDRsupply: 0: DCDC/GLDO are generating VDDR 1: DCDC/GLDO are bypassed, external regulator supplies VDDR."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_REG_MODE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Status of source for VDDRsupply: 0: DCDC/GLDO are generating VDDR 1: DCDC/GLDO are bypassed, external regulator supplies VDDR."]
    #[inline(always)]
    pub const fn set_EXT_REG_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDRin active mode. 1: Use DCDC for regulation of VDDRin active mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDC_ACTIVE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDRin active mode. 1: Use DCDC for regulation of VDDRin active mode."]
    #[inline(always)]
    pub const fn set_DCDC_ACTIVE(&mut self, val: bool) {
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
impl Default for PWRCTL {
    #[inline(always)]
    fn default() -> PWRCTL {
        PWRCTL(0)
    }
}
impl core::fmt::Debug for PWRCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRCTL")
            .field("DCDC_EN", &self.DCDC_EN())
            .field("EXT_REG_MODE", &self.EXT_REG_MODE())
            .field("DCDC_ACTIVE", &self.DCDC_ACTIVE())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWRCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWRCTL {{ DCDC_EN: {=bool:?}, EXT_REG_MODE: {=bool:?}, DCDC_ACTIVE: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.DCDC_EN(),
            self.EXT_REG_MODE(),
            self.DCDC_ACTIVE(),
            self.RESERVED3()
        )
    }
}
#[doc = "Reset Management This register contains bitfields releated to system reset such as reset source and reset request and control of brown out resets."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESETCTL(pub u32);
impl RESETCTL {
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
    #[doc = "3:1\\] Shows the source of the last system reset: Occurrence of one of the reset sources may trigger several other reset sources as essential parts of the system are undergoing reset. This field will report the root cause of the reset (not the other resets that are consequence of the system reset). To support this feature the actual register is not captured before the reset source being released. If a new reset source is triggered, in a window of four 32 kHz periods after the previous has been released, this register may indicate Power on reset as source."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET_SRC(&self) -> super::vals::RESET_SRC {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::RESET_SRC::from_bits(val as u8)
    }
    #[doc = "3:1\\] Shows the source of the last system reset: Occurrence of one of the reset sources may trigger several other reset sources as essential parts of the system are undergoing reset. This field will report the root cause of the reset (not the other resets that are consequence of the system reset). To support this feature the actual register is not captured before the reset source being released. If a new reset source is triggered, in a window of four 32 kHz periods after the previous has been released, this register may indicate Power on reset as source."]
    #[inline(always)]
    pub const fn set_RESET_SRC(&mut self, val: super::vals::RESET_SRC) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "4:4\\] Controls reset generation in case SCLK_LF is lost. (provided that clock loss detection is enabled by DDI_0_OSC:CTL0.CLK_LOSS_EN) Note: Clock loss reset generation must be disabled before SCLK_LF clock source is changed in DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL and remain disabled untill the change is confirmed in DDI_0_OSC:STAT0.SCLK_LF_SRC. Failure to do so may result in a spurious system reset. Clock loss reset generation can be disabled through this bitfield or by clearing DDI_0_OSC:CTL0.CLK_LOSS_EN 0: Clock loss is ignored 1: Clock loss generates system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_LOSS_EN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Controls reset generation in case SCLK_LF is lost. (provided that clock loss detection is enabled by DDI_0_OSC:CTL0.CLK_LOSS_EN) Note: Clock loss reset generation must be disabled before SCLK_LF clock source is changed in DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL and remain disabled untill the change is confirmed in DDI_0_OSC:STAT0.SCLK_LF_SRC. Failure to do so may result in a spurious system reset. Clock loss reset generation can be disabled through this bitfield or by clearing DDI_0_OSC:CTL0.CLK_LOSS_EN 0: Clock loss is ignored 1: Clock loss generates system reset."]
    #[inline(always)]
    pub const fn set_CLK_LOSS_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Controls reset generation in case VDD is lost 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN_OVR=1 1: Brown out detect of VDD generates system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn VDD_LOSS_EN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Controls reset generation in case VDD is lost 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN_OVR=1 1: Brown out detect of VDD generates system reset."]
    #[inline(always)]
    pub const fn set_VDD_LOSS_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Controls reset generation in case VDDR is lost 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN_OVR=1 1: Brown out detect of VDDR generates system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_LOSS_EN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Controls reset generation in case VDDR is lost 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN_OVR=1 1: Brown out detect of VDDR generates system reset."]
    #[inline(always)]
    pub const fn set_VDDR_LOSS_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Controls reset generation in case VDDS is lost 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN_OVR=1 1: Brown out detect of VDDS generates system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDS_LOSS_EN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Controls reset generation in case VDDS is lost 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN_OVR=1 1: Brown out detect of VDDS generates system reset."]
    #[inline(always)]
    pub const fn set_VDDS_LOSS_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Override of VDD_LOSS_EN 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN=1 1: Brown out detect of VDD generates system reset (regardless of VDD_LOSS_EN) This bit can be locked."]
    #[must_use]
    #[inline(always)]
    pub const fn VDD_LOSS_EN_OVR(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Override of VDD_LOSS_EN 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN=1 1: Brown out detect of VDD generates system reset (regardless of VDD_LOSS_EN) This bit can be locked."]
    #[inline(always)]
    pub const fn set_VDD_LOSS_EN_OVR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Override of VDDR_LOSS_EN 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN=1 1: Brown out detect of VDDR generates system reset (regardless of VDDR_LOSS_EN) This bit can be locked."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_LOSS_EN_OVR(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Override of VDDR_LOSS_EN 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN=1 1: Brown out detect of VDDR generates system reset (regardless of VDDR_LOSS_EN) This bit can be locked."]
    #[inline(always)]
    pub const fn set_VDDR_LOSS_EN_OVR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Override of VDDS_LOSS_EN 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN=1 1: Brown out detect of VDDS generates system reset (regardless of VDDS_LOSS_EN) This bit can be locked."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDS_LOSS_EN_OVR(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Override of VDDS_LOSS_EN 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN=1 1: Brown out detect of VDDS generates system reset (regardless of VDDS_LOSS_EN) This bit can be locked."]
    #[inline(always)]
    pub const fn set_VDDS_LOSS_EN_OVR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_DET_0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BOOT_DET_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_DET_1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BOOT_DET_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] A wakeup from SHUTDOWN on an IO event has occurred Please refer to \\[IOC:IOCFGn,.WU_CFG\\] for configuring the IO's as wakeup sources. 0: The wakeup did not occur from SHUTDOWN on an IO event 1: A wakeup from SHUTDOWN occurred from an IO event The case where WU_FROM_SD is asserted but this bitfield is not asserted will only occur in a debug session. The boot code will not proceed with wakeup from SHUTDOWN procedure until this bitfield is asserted as well. Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_WU_FROM_SD(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] A wakeup from SHUTDOWN on an IO event has occurred Please refer to \\[IOC:IOCFGn,.WU_CFG\\] for configuring the IO's as wakeup sources. 0: The wakeup did not occur from SHUTDOWN on an IO event 1: A wakeup from SHUTDOWN occurred from an IO event The case where WU_FROM_SD is asserted but this bitfield is not asserted will only occur in a debug session. The boot code will not proceed with wakeup from SHUTDOWN procedure until this bitfield is asserted as well. Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset."]
    #[inline(always)]
    pub const fn set_GPIO_WU_FROM_SD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] A Wakeup from SHUTDOWN on an IO event has occurred, or a wakeup from SHUTDOWN has occurred as a result of the debugger being attached.. (TCK pin being forced low) Please refer to \\[IOC:IOCFGn,.WU_CFG\\] for configuring the IO's as wakeup sources. 0: Wakeup occurred from cold reset or brown out as seen in RESET_SRC 1: A wakeup has occurred from SHUTDOWN Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_FROM_SD(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] A Wakeup from SHUTDOWN on an IO event has occurred, or a wakeup from SHUTDOWN has occurred as a result of the debugger being attached.. (TCK pin being forced low) Please refer to \\[IOC:IOCFGn,.WU_CFG\\] for configuring the IO's as wakeup sources. 0: Wakeup occurred from cold reset or brown out as seen in RESET_SRC 1: A wakeup has occurred from SHUTDOWN Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset."]
    #[inline(always)]
    pub const fn set_WU_FROM_SD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_DET_0_SET(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BOOT_DET_0_SET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_DET_1_SET(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BOOT_DET_1_SET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "23:18\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED18(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "23:18\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED18(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "24:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_DET_0_CLR(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BOOT_DET_0_CLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_DET_1_CLR(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BOOT_DET_1_CLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "30:26\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED26(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "30:26\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED26(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "31:31\\] Cold reset register. Writing 1 to this bitfield will reset the entire chip and cause boot code to run again. 0: No effect 1: Generate system reset. Appears as SYSRESET in RESET_SRC."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSRESET(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Cold reset register. Writing 1 to this bitfield will reset the entire chip and cause boot code to run again. 0: No effect 1: Generate system reset. Appears as SYSRESET in RESET_SRC."]
    #[inline(always)]
    pub const fn set_SYSRESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RESETCTL {
    #[inline(always)]
    fn default() -> RESETCTL {
        RESETCTL(0)
    }
}
impl core::fmt::Debug for RESETCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESETCTL")
            .field("RESERVED0", &self.RESERVED0())
            .field("RESET_SRC", &self.RESET_SRC())
            .field("CLK_LOSS_EN", &self.CLK_LOSS_EN())
            .field("VDD_LOSS_EN", &self.VDD_LOSS_EN())
            .field("VDDR_LOSS_EN", &self.VDDR_LOSS_EN())
            .field("VDDS_LOSS_EN", &self.VDDS_LOSS_EN())
            .field("RESERVED8", &self.RESERVED8())
            .field("VDD_LOSS_EN_OVR", &self.VDD_LOSS_EN_OVR())
            .field("VDDR_LOSS_EN_OVR", &self.VDDR_LOSS_EN_OVR())
            .field("VDDS_LOSS_EN_OVR", &self.VDDS_LOSS_EN_OVR())
            .field("BOOT_DET_0", &self.BOOT_DET_0())
            .field("BOOT_DET_1", &self.BOOT_DET_1())
            .field("GPIO_WU_FROM_SD", &self.GPIO_WU_FROM_SD())
            .field("WU_FROM_SD", &self.WU_FROM_SD())
            .field("BOOT_DET_0_SET", &self.BOOT_DET_0_SET())
            .field("BOOT_DET_1_SET", &self.BOOT_DET_1_SET())
            .field("RESERVED18", &self.RESERVED18())
            .field("BOOT_DET_0_CLR", &self.BOOT_DET_0_CLR())
            .field("BOOT_DET_1_CLR", &self.BOOT_DET_1_CLR())
            .field("RESERVED26", &self.RESERVED26())
            .field("SYSRESET", &self.SYSRESET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESETCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RESETCTL {{ RESERVED0: {=bool:?}, RESET_SRC: {:?}, CLK_LOSS_EN: {=bool:?}, VDD_LOSS_EN: {=bool:?}, VDDR_LOSS_EN: {=bool:?}, VDDS_LOSS_EN: {=bool:?}, RESERVED8: {=bool:?}, VDD_LOSS_EN_OVR: {=bool:?}, VDDR_LOSS_EN_OVR: {=bool:?}, VDDS_LOSS_EN_OVR: {=bool:?}, BOOT_DET_0: {=bool:?}, BOOT_DET_1: {=bool:?}, GPIO_WU_FROM_SD: {=bool:?}, WU_FROM_SD: {=bool:?}, BOOT_DET_0_SET: {=bool:?}, BOOT_DET_1_SET: {=bool:?}, RESERVED18: {=u8:?}, BOOT_DET_0_CLR: {=bool:?}, BOOT_DET_1_CLR: {=bool:?}, RESERVED26: {=u8:?}, SYSRESET: {=bool:?} }}",
            self.RESERVED0(),
            self.RESET_SRC(),
            self.CLK_LOSS_EN(),
            self.VDD_LOSS_EN(),
            self.VDDR_LOSS_EN(),
            self.VDDS_LOSS_EN(),
            self.RESERVED8(),
            self.VDD_LOSS_EN_OVR(),
            self.VDDR_LOSS_EN_OVR(),
            self.VDDS_LOSS_EN_OVR(),
            self.BOOT_DET_0(),
            self.BOOT_DET_1(),
            self.GPIO_WU_FROM_SD(),
            self.WU_FROM_SD(),
            self.BOOT_DET_0_SET(),
            self.BOOT_DET_1_SET(),
            self.RESERVED18(),
            self.BOOT_DET_0_CLR(),
            self.BOOT_DET_1_CLR(),
            self.RESERVED26(),
            self.SYSRESET()
        )
    }
}
#[doc = "Sleep Mode This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SLEEPCTL(pub u32);
impl SLEEPCTL {
    #[doc = "0:0\\] Controls the I/O pad sleep mode. The boot code will set this bitfield automatically unless waking up from a SHUTDOWN ( RESETCTL.WU_FROM_SD is set ). 0: I/O pad sleep mode is enabled, ie all pads are latched and can not toggle. 1: I/O pad sleep mode is disabled Application software may want to reconfigure the state for all IO's before setting this bitfield upon waking up from a SHUTDOWN."]
    #[must_use]
    #[inline(always)]
    pub const fn IO_PAD_SLEEP_DIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Controls the I/O pad sleep mode. The boot code will set this bitfield automatically unless waking up from a SHUTDOWN ( RESETCTL.WU_FROM_SD is set ). 0: I/O pad sleep mode is enabled, ie all pads are latched and can not toggle. 1: I/O pad sleep mode is disabled Application software may want to reconfigure the state for all IO's before setting this bitfield upon waking up from a SHUTDOWN."]
    #[inline(always)]
    pub const fn set_IO_PAD_SLEEP_DIS(&mut self, val: bool) {
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
impl Default for SLEEPCTL {
    #[inline(always)]
    fn default() -> SLEEPCTL {
        SLEEPCTL(0)
    }
}
impl core::fmt::Debug for SLEEPCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLEEPCTL")
            .field("IO_PAD_SLEEP_DIS", &self.IO_PAD_SLEEP_DIS())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SLEEPCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SLEEPCTL {{ IO_PAD_SLEEP_DIS: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.IO_PAD_SLEEP_DIS(),
            self.RESERVED1()
        )
    }
}

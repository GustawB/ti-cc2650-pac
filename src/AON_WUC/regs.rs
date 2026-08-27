#[doc = "AUX Configuration This register contains power management related signals for the AUX domain."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUXCFG(pub u32);
impl AUXCFG {
    #[doc = "0:0\\] This bit controls retention mode for the AUX_RAM:BANK0: 0: Retention is disabled 1: Retention is enabled NB: If retention is disabled, the AUX_RAM will be powered off when it would otherwise be put in retention mode."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_RET_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This bit controls retention mode for the AUX_RAM:BANK0: 0: Retention is disabled 1: Retention is enabled NB: If retention is disabled, the AUX_RAM will be powered off when it would otherwise be put in retention mode."]
    #[inline(always)]
    pub const fn set_RAM_RET_EN(&mut self, val: bool) {
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
impl Default for AUXCFG {
    #[inline(always)]
    fn default() -> AUXCFG {
        AUXCFG(0)
    }
}
impl core::fmt::Debug for AUXCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUXCFG")
            .field("RAM_RET_EN", &self.RAM_RET_EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUXCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUXCFG {{ RAM_RET_EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.RAM_RET_EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUXCLK(pub u32);
impl AUXCLK {
    #[doc = "2:0\\] Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless."]
    #[must_use]
    #[inline(always)]
    pub const fn SRC(&self) -> super::vals::SRC {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SRC::from_bits(val as u8)
    }
    #[doc = "2:0\\] Selects the clock source for AUX: NB: Switching the clock source is guaranteed to be glitchless."]
    #[inline(always)]
    pub const fn set_SRC(&mut self, val: super::vals::SRC) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
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
    #[doc = "10:8\\] Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLK_HF_DIV(&self) -> super::vals::SCLK_HF_DIV {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::SCLK_HF_DIV::from_bits(val as u8)
    }
    #[doc = "10:8\\] Select the AUX clock divider for SCLK_HF NB: It is not supported to change the AUX clock divider while SCLK_HF is active source for AUX."]
    #[inline(always)]
    pub const fn set_SCLK_HF_DIV(&mut self, val: super::vals::SCLK_HF_DIV) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "12:11\\] When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode."]
    #[must_use]
    #[inline(always)]
    pub const fn PWR_DWN_SRC(&self) -> super::vals::AUXCLK_PWR_DWN_SRC {
        let val = (self.0 >> 11usize) & 0x03;
        super::vals::AUXCLK_PWR_DWN_SRC::from_bits(val as u8)
    }
    #[doc = "12:11\\] When AUX requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when AUX system is back in active mode."]
    #[inline(always)]
    pub const fn set_PWR_DWN_SRC(&mut self, val: super::vals::AUXCLK_PWR_DWN_SRC) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
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
impl Default for AUXCLK {
    #[inline(always)]
    fn default() -> AUXCLK {
        AUXCLK(0)
    }
}
impl core::fmt::Debug for AUXCLK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUXCLK")
            .field("SRC", &self.SRC())
            .field("RESERVED3", &self.RESERVED3())
            .field("SCLK_HF_DIV", &self.SCLK_HF_DIV())
            .field("PWR_DWN_SRC", &self.PWR_DWN_SRC())
            .field("RESERVED13", &self.RESERVED13())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUXCLK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUXCLK {{ SRC: {:?}, RESERVED3: {=u8:?}, SCLK_HF_DIV: {:?}, PWR_DWN_SRC: {:?}, RESERVED13: {=u32:?} }}",
            self.SRC(),
            self.RESERVED3(),
            self.SCLK_HF_DIV(),
            self.PWR_DWN_SRC(),
            self.RESERVED13()
        )
    }
}
#[doc = "AUX Control This register contains events and control signals for the AUX domain."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUXCTL(pub u32);
impl AUXCTL {
    #[doc = "0:0\\] Forces the AUX domain into active mode, overriding the requests from AUX_WUC:PWROFFREQ, AUX_WUC:PWRDWNREQ and AUX_WUC:MCUBUSCTL. Note that an ongoing AUX_WUC:PWROFFREQ will complete before this bit will set the AUX domain into active mode. MCU must set this bit in order to access the AUX peripherals. The AUX domain status can be read from PWRSTAT.AUX_PD_ON 0: AUX is allowed to Power Off, Power Down or Disconnect. 1: AUX Power OFF, Power Down or Disconnect requests will be overruled."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_FORCE_ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Forces the AUX domain into active mode, overriding the requests from AUX_WUC:PWROFFREQ, AUX_WUC:PWRDWNREQ and AUX_WUC:MCUBUSCTL. Note that an ongoing AUX_WUC:PWROFFREQ will complete before this bit will set the AUX domain into active mode. MCU must set this bit in order to access the AUX peripherals. The AUX domain status can be read from PWRSTAT.AUX_PD_ON 0: AUX is allowed to Power Off, Power Down or Disconnect. 1: AUX Power OFF, Power Down or Disconnect requests will be overruled."]
    #[inline(always)]
    pub const fn set_AUX_FORCE_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Writing 1 sets the software event to the AUX domain, which can be read through AUX_WUC:WUEVFLAGS.AON_SW. This event is normally cleared by AUX_SCE through the AUX_WUC:WUEVCLR.AON_SW. It can also be cleared by writing 0 to this register. Reading 0 means that there is no outstanding software event for AUX. Note that it can take up to 1,5 SCLK_LF clock cycles to clear the event from AUX."]
    #[must_use]
    #[inline(always)]
    pub const fn SWEV(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Writing 1 sets the software event to the AUX domain, which can be read through AUX_WUC:WUEVFLAGS.AON_SW. This event is normally cleared by AUX_SCE through the AUX_WUC:WUEVCLR.AON_SW. It can also be cleared by writing 0 to this register. Reading 0 means that there is no outstanding software event for AUX. Note that it can take up to 1,5 SCLK_LF clock cycles to clear the event from AUX."]
    #[inline(always)]
    pub const fn set_SWEV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Enables (1) or disables (0) AUX_SCE execution. AUX_SCE execution will begin when AUX Domain is powered and either this or AUX_SCE:CTL.CLK_EN is set. Setting this bit will assure that AUX_SCE execution starts as soon as AUX power domain is woken up. ( AUX_SCE:CTL.CLK_EN will be reset to 0 if AUX power domain has been off) 0: AUX_SCE execution will be disabled if AUX_SCE:CTL.CLK_EN is 0 1: AUX_SCE execution is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn SCE_RUN_EN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Enables (1) or disables (0) AUX_SCE execution. AUX_SCE execution will begin when AUX Domain is powered and either this or AUX_SCE:CTL.CLK_EN is set. Setting this bit will assure that AUX_SCE execution starts as soon as AUX power domain is woken up. ( AUX_SCE:CTL.CLK_EN will be reset to 0 if AUX power domain has been off) 0: AUX_SCE execution will be disabled if AUX_SCE:CTL.CLK_EN is 0 1: AUX_SCE execution is enabled."]
    #[inline(always)]
    pub const fn set_SCE_RUN_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "30:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "30:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 3usize)) | (((val as u32) & 0x0fff_ffff) << 3usize);
    }
    #[doc = "31:31\\] Reset request for AUX. Writing 1 to this register will assert reset to AUX. The reset will be held until the bit is cleared again. 0: AUX reset pin will be deasserted 1: AUX reset pin will be asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET_REQ(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Reset request for AUX. Writing 1 to this register will assert reset to AUX. The reset will be held until the bit is cleared again. 0: AUX reset pin will be deasserted 1: AUX reset pin will be asserted."]
    #[inline(always)]
    pub const fn set_RESET_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for AUXCTL {
    #[inline(always)]
    fn default() -> AUXCTL {
        AUXCTL(0)
    }
}
impl core::fmt::Debug for AUXCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUXCTL")
            .field("AUX_FORCE_ON", &self.AUX_FORCE_ON())
            .field("SWEV", &self.SWEV())
            .field("SCE_RUN_EN", &self.SCE_RUN_EN())
            .field("RESERVED3", &self.RESERVED3())
            .field("RESET_REQ", &self.RESET_REQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUXCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUXCTL {{ AUX_FORCE_ON: {=bool:?}, SWEV: {=bool:?}, SCE_RUN_EN: {=bool:?}, RESERVED3: {=u32:?}, RESET_REQ: {=bool:?} }}",
            self.AUX_FORCE_ON(),
            self.SWEV(),
            self.SCE_RUN_EN(),
            self.RESERVED3(),
            self.RESET_REQ()
        )
    }
}
#[doc = "Control 0 This register contains various chip level control and debug bitfields."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTL0(pub u32);
impl CTL0 {
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
    pub const fn MCU_SRAM_ERASE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MCU_SRAM_ERASE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_SRAM_ERASE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_AUX_SRAM_ERASE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "7:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "7:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "8:8\\] Controls whether MCU and AUX requesting to be powered off will enable a transition to powerdown: 0: Enabled 1: Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn PWR_DWN_DIS(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Controls whether MCU and AUX requesting to be powered off will enable a transition to powerdown: 0: Enabled 1: Disabled."]
    #[inline(always)]
    pub const fn set_PWR_DWN_DIS(&mut self, val: bool) {
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
impl Default for CTL0 {
    #[inline(always)]
    fn default() -> CTL0 {
        CTL0(0)
    }
}
impl core::fmt::Debug for CTL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL0")
            .field("RESERVED0", &self.RESERVED0())
            .field("MCU_SRAM_ERASE", &self.MCU_SRAM_ERASE())
            .field("AUX_SRAM_ERASE", &self.AUX_SRAM_ERASE())
            .field("RESERVED4", &self.RESERVED4())
            .field("PWR_DWN_DIS", &self.PWR_DWN_DIS())
            .field("RESERVED9", &self.RESERVED9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTL0 {{ RESERVED0: {=u8:?}, MCU_SRAM_ERASE: {=bool:?}, AUX_SRAM_ERASE: {=bool:?}, RESERVED4: {=u8:?}, PWR_DWN_DIS: {=bool:?}, RESERVED9: {=u32:?} }}",
            self.RESERVED0(),
            self.MCU_SRAM_ERASE(),
            self.AUX_SRAM_ERASE(),
            self.RESERVED4(),
            self.PWR_DWN_DIS(),
            self.RESERVED9()
        )
    }
}
#[doc = "Control 1 This register contains various chip level control and debug bitfields."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTL1(pub u32);
impl CTL1 {
    #[doc = "0:0\\] Indicates type of last MCU Voltage Domain reset: 0: Last MCU reset was not a warm reset 1: Last MCU reset was a warm reset (requested from MCU or JTAG as indicated in MCU_RESET_SRC) This bit can only be cleared by writing a 1 to it."]
    #[must_use]
    #[inline(always)]
    pub const fn MCU_WARM_RESET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Indicates type of last MCU Voltage Domain reset: 0: Last MCU reset was not a warm reset 1: Last MCU reset was a warm reset (requested from MCU or JTAG as indicated in MCU_RESET_SRC) This bit can only be cleared by writing a 1 to it."]
    #[inline(always)]
    pub const fn set_MCU_WARM_RESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Indicates source of last MCU Voltage Domain warm reset request: 0: MCU SW reset 1: JTAG reset This bit can only be cleared by writing a 1 to it."]
    #[must_use]
    #[inline(always)]
    pub const fn MCU_RESET_SRC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Indicates source of last MCU Voltage Domain warm reset request: 0: MCU SW reset 1: JTAG reset This bit can only be cleared by writing a 1 to it."]
    #[inline(always)]
    pub const fn set_MCU_RESET_SRC(&mut self, val: bool) {
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
impl Default for CTL1 {
    #[inline(always)]
    fn default() -> CTL1 {
        CTL1(0)
    }
}
impl core::fmt::Debug for CTL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL1")
            .field("MCU_WARM_RESET", &self.MCU_WARM_RESET())
            .field("MCU_RESET_SRC", &self.MCU_RESET_SRC())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTL1 {{ MCU_WARM_RESET: {=bool:?}, MCU_RESET_SRC: {=bool:?}, RESERVED2: {=u32:?} }}",
            self.MCU_WARM_RESET(),
            self.MCU_RESET_SRC(),
            self.RESERVED2()
        )
    }
}
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JTAGCFG(pub u32);
impl JTAGCFG {
    #[doc = "7:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "8:8\\] Controls JTAG PowerDomain power state: 0: Controlled exclusively by debug subsystem. (JTAG Powerdomain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. NB: The reset value causes JTAG Power Domain to be powered on by default. Software must clear this bit to turn off the JTAG Power Domain."]
    #[must_use]
    #[inline(always)]
    pub const fn JTAG_PD_FORCE_ON(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Controls JTAG PowerDomain power state: 0: Controlled exclusively by debug subsystem. (JTAG Powerdomain will be powered off unless a debugger is attached) 1: JTAG Power Domain is forced on, independent of debug subsystem. NB: The reset value causes JTAG Power Domain to be powered on by default. Software must clear this bit to turn off the JTAG Power Domain."]
    #[inline(always)]
    pub const fn set_JTAG_PD_FORCE_ON(&mut self, val: bool) {
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
impl Default for JTAGCFG {
    #[inline(always)]
    fn default() -> JTAGCFG {
        JTAGCFG(0)
    }
}
impl core::fmt::Debug for JTAGCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JTAGCFG")
            .field("RESERVED0", &self.RESERVED0())
            .field("JTAG_PD_FORCE_ON", &self.JTAG_PD_FORCE_ON())
            .field("RESERVED9", &self.RESERVED9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JTAGCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JTAGCFG {{ RESERVED0: {=u8:?}, JTAG_PD_FORCE_ON: {=bool:?}, RESERVED9: {=u32:?} }}",
            self.RESERVED0(),
            self.JTAG_PD_FORCE_ON(),
            self.RESERVED9()
        )
    }
}
#[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JTAGUSERCODE(pub u32);
impl JTAGUSERCODE {
    #[doc = "31:0\\] 32-bit JTAG USERCODE register feeding main JTAG TAP NB: This field can be locked."]
    #[must_use]
    #[inline(always)]
    pub const fn USER_CODE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] 32-bit JTAG USERCODE register feeding main JTAG TAP NB: This field can be locked."]
    #[inline(always)]
    pub const fn set_USER_CODE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for JTAGUSERCODE {
    #[inline(always)]
    fn default() -> JTAGUSERCODE {
        JTAGUSERCODE(0)
    }
}
impl core::fmt::Debug for JTAGUSERCODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JTAGUSERCODE")
            .field("USER_CODE", &self.USER_CODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JTAGUSERCODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JTAGUSERCODE {{ USER_CODE: {=u32:?} }}",
            self.USER_CODE()
        )
    }
}
#[doc = "MCU Configuration This register contains power management related bitfields for the MCU domain."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MCUCFG(pub u32);
impl MCUCFG {
    #[doc = "3:0\\] MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off."]
    #[must_use]
    #[inline(always)]
    pub const fn SRAM_RET_EN(&self) -> super::vals::SRAM_RET_EN {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::SRAM_RET_EN::from_bits(val as u8)
    }
    #[doc = "3:0\\] MCU SRAM is partitioned into 4 banks . This register controls which of the banks that has retention during MCU power off."]
    #[inline(always)]
    pub const fn set_SRAM_RET_EN(&mut self, val: super::vals::SRAM_RET_EN) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "15:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "15:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FIXED_WU_EN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FIXED_WU_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VIRT_OFF(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VIRT_OFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "31:18\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED18(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x3fff;
        val as u16
    }
    #[doc = "31:18\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED18(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 18usize)) | (((val as u32) & 0x3fff) << 18usize);
    }
}
impl Default for MCUCFG {
    #[inline(always)]
    fn default() -> MCUCFG {
        MCUCFG(0)
    }
}
impl core::fmt::Debug for MCUCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCUCFG")
            .field("SRAM_RET_EN", &self.SRAM_RET_EN())
            .field("RESERVED4", &self.RESERVED4())
            .field("FIXED_WU_EN", &self.FIXED_WU_EN())
            .field("VIRT_OFF", &self.VIRT_OFF())
            .field("RESERVED18", &self.RESERVED18())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MCUCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MCUCFG {{ SRAM_RET_EN: {:?}, RESERVED4: {=u16:?}, FIXED_WU_EN: {=bool:?}, VIRT_OFF: {=bool:?}, RESERVED18: {=u16:?} }}",
            self.SRAM_RET_EN(),
            self.RESERVED4(),
            self.FIXED_WU_EN(),
            self.VIRT_OFF(),
            self.RESERVED18()
        )
    }
}
#[doc = "MCU Clock Management This register contains bitfields related to the MCU clock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MCUCLK(pub u32);
impl MCUCLK {
    #[doc = "1:0\\] Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
    #[must_use]
    #[inline(always)]
    pub const fn PWR_DWN_SRC(&self) -> super::vals::MCUCLK_PWR_DWN_SRC {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MCUCLK_PWR_DWN_SRC::from_bits(val as u8)
    }
    #[doc = "1:0\\] Controls the clock source for the entire MCU domain while MCU is requesting powerdown. When MCU requests powerdown with SCLK_HF as source, then WUC will switch over to this clock source during powerdown, and automatically switch back to SCLK_HF when MCU is no longer requesting powerdown and system is back in active mode."]
    #[inline(always)]
    pub const fn set_PWR_DWN_SRC(&mut self, val: super::vals::MCUCLK_PWR_DWN_SRC) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "2:2\\] MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSC_HF_CAL_DONE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] MCU bootcode will set this bit when RCOSC_HF is calibrated. The FLASH can not be used until this bit is set. 1: RCOSC_HF is calibrated to 48 MHz, allowing FLASH to power up. 0: RCOSC_HF is not yet calibrated, ie FLASH must not assume that the SCLK_HF is safe."]
    #[inline(always)]
    pub const fn set_RCOSC_HF_CAL_DONE(&mut self, val: bool) {
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
impl Default for MCUCLK {
    #[inline(always)]
    fn default() -> MCUCLK {
        MCUCLK(0)
    }
}
impl core::fmt::Debug for MCUCLK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCUCLK")
            .field("PWR_DWN_SRC", &self.PWR_DWN_SRC())
            .field("RCOSC_HF_CAL_DONE", &self.RCOSC_HF_CAL_DONE())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MCUCLK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MCUCLK {{ PWR_DWN_SRC: {:?}, RCOSC_HF_CAL_DONE: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.PWR_DWN_SRC(),
            self.RCOSC_HF_CAL_DONE(),
            self.RESERVED3()
        )
    }
}
#[doc = "Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OSCCFG(pub u32);
impl OSCCFG {
    #[doc = "2:0\\] Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the exponent Note: Oscillator amplitude calibration is turned of when both PER_M and this bitfield are set to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn PER_E(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the exponent Note: Oscillator amplitude calibration is turned of when both PER_M and this bitfield are set to 0."]
    #[inline(always)]
    pub const fn set_PER_E(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "7:3\\] Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the mantissa Note: Oscillator amplitude calibration is turned of when both this bitfield and PER_E are set to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn PER_M(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "7:3\\] Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the mantissa Note: Oscillator amplitude calibration is turned of when both this bitfield and PER_E are set to 0."]
    #[inline(always)]
    pub const fn set_PER_M(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
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
impl Default for OSCCFG {
    #[inline(always)]
    fn default() -> OSCCFG {
        OSCCFG(0)
    }
}
impl core::fmt::Debug for OSCCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSCCFG")
            .field("PER_E", &self.PER_E())
            .field("PER_M", &self.PER_M())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OSCCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OSCCFG {{ PER_E: {=u8:?}, PER_M: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.PER_E(),
            self.PER_M(),
            self.RESERVED8()
        )
    }
}
#[doc = "Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWRSTAT(pub u32);
impl PWRSTAT {
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
    #[doc = "1:1\\] Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_RESET_DONE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Indicates Reset Done from AUX: 0: AUX is being reset 1: AUX reset is released."]
    #[inline(always)]
    pub const fn set_AUX_RESET_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Indicates that AUX Bus is connected: 0: AUX bus is not connected 1: AUX bus is connected ( idle_ack = 0 )."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_BUS_CONNECTED(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Indicates that AUX Bus is connected: 0: AUX bus is not connected 1: AUX bus is connected ( idle_ack = 0 )."]
    #[inline(always)]
    pub const fn set_AUX_BUS_CONNECTED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Indicates MCU power state: 0: MCU Power sequencing is not yet finalized and MCU_AONIF registers may not be reliable 1: MCU Power sequencing is finalized and all MCU_AONIF registers are reliable."]
    #[must_use]
    #[inline(always)]
    pub const fn MCU_PD_ON(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Indicates MCU power state: 0: MCU Power sequencing is not yet finalized and MCU_AONIF registers may not be reliable 1: MCU Power sequencing is finalized and all MCU_AONIF registers are reliable."]
    #[inline(always)]
    pub const fn set_MCU_PD_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Indicates AUX power state: 0: AUX is not ready for use ( may be powered off or in power state transition ) 1: AUX is powered on, connected to bus and ready for use,."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_PD_ON(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Indicates AUX power state: 0: AUX is not ready for use ( may be powered off or in power state transition ) 1: AUX is powered on, connected to bus and ready for use,."]
    #[inline(always)]
    pub const fn set_AUX_PD_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on."]
    #[must_use]
    #[inline(always)]
    pub const fn JTAG_PD_ON(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Indicates JTAG power state: 0: JTAG is powered off 1: JTAG is powered on."]
    #[inline(always)]
    pub const fn set_JTAG_PD_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "8:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED7(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "8:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "9:9\\] Indicates the AUX powerdown state when AUX domain is powered up. 0: Active mode 1: AUX Powerdown request has been granted."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_PWR_DWN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Indicates the AUX powerdown state when AUX domain is powered up. 0: Active mode 1: AUX Powerdown request has been granted."]
    #[inline(always)]
    pub const fn set_AUX_PWR_DWN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
impl Default for PWRSTAT {
    #[inline(always)]
    fn default() -> PWRSTAT {
        PWRSTAT(0)
    }
}
impl core::fmt::Debug for PWRSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRSTAT")
            .field("RESERVED0", &self.RESERVED0())
            .field("AUX_RESET_DONE", &self.AUX_RESET_DONE())
            .field("AUX_BUS_CONNECTED", &self.AUX_BUS_CONNECTED())
            .field("RESERVED3", &self.RESERVED3())
            .field("MCU_PD_ON", &self.MCU_PD_ON())
            .field("AUX_PD_ON", &self.AUX_PD_ON())
            .field("JTAG_PD_ON", &self.JTAG_PD_ON())
            .field("RESERVED7", &self.RESERVED7())
            .field("AUX_PWR_DWN", &self.AUX_PWR_DWN())
            .field("RESERVED10", &self.RESERVED10())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWRSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWRSTAT {{ RESERVED0: {=bool:?}, AUX_RESET_DONE: {=bool:?}, AUX_BUS_CONNECTED: {=bool:?}, RESERVED3: {=bool:?}, MCU_PD_ON: {=bool:?}, AUX_PD_ON: {=bool:?}, JTAG_PD_ON: {=bool:?}, RESERVED7: {=u8:?}, AUX_PWR_DWN: {=bool:?}, RESERVED10: {=u32:?} }}",
            self.RESERVED0(),
            self.AUX_RESET_DONE(),
            self.AUX_BUS_CONNECTED(),
            self.RESERVED3(),
            self.MCU_PD_ON(),
            self.AUX_PD_ON(),
            self.JTAG_PD_ON(),
            self.RESERVED7(),
            self.AUX_PWR_DWN(),
            self.RESERVED10()
        )
    }
}
#[doc = "Recharge Controller Configuration This register sets all relevant patameters for controlling the recharge algorithm."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RECHARGECFG(pub u32);
impl RECHARGECFG {
    #[doc = "2:0\\] Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Exponent of the Period. PERIOD=(PER_M*16+15)*2^PER_E."]
    #[must_use]
    #[inline(always)]
    pub const fn PER_E(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Exponent of the Period. PERIOD=(PER_M*16+15)*2^PER_E."]
    #[inline(always)]
    pub const fn set_PER_E(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "7:3\\] Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Mantissa of the Period. PERIOD=(PER_M*16+15)*2^PER_E."]
    #[must_use]
    #[inline(always)]
    pub const fn PER_M(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "7:3\\] Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Mantissa of the Period. PERIOD=(PER_M*16+15)*2^PER_E."]
    #[inline(always)]
    pub const fn set_PER_M(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
    }
    #[doc = "10:8\\] This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the exponent MAXCYCLES."]
    #[must_use]
    #[inline(always)]
    pub const fn MAX_PER_E(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "10:8\\] This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the exponent MAXCYCLES."]
    #[inline(always)]
    pub const fn set_MAX_PER_E(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "15:11\\] This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the mantissa of MAXCYCLES."]
    #[must_use]
    #[inline(always)]
    pub const fn MAX_PER_M(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "15:11\\] This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the mantissa of MAXCYCLES."]
    #[inline(always)]
    pub const fn set_MAX_PER_M(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
    #[doc = "19:16\\] Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C1 is 1 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1."]
    #[must_use]
    #[inline(always)]
    pub const fn C1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C1 is 1 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1."]
    #[inline(always)]
    pub const fn set_C1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "23:20\\] Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C2 is 2 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1."]
    #[must_use]
    #[inline(always)]
    pub const fn C2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C2 is 2 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1."]
    #[inline(always)]
    pub const fn set_C2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "30:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED24(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "30:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED24(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "31:31\\] Enable adaptive recharge Note: Recharge can be turned completely of by setting MAX_PER_E=7 and MAX_PER_M=31 and this bitfield to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ADAPTIVE_EN(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Enable adaptive recharge Note: Recharge can be turned completely of by setting MAX_PER_E=7 and MAX_PER_M=31 and this bitfield to 0."]
    #[inline(always)]
    pub const fn set_ADAPTIVE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RECHARGECFG {
    #[inline(always)]
    fn default() -> RECHARGECFG {
        RECHARGECFG(0)
    }
}
impl core::fmt::Debug for RECHARGECFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RECHARGECFG")
            .field("PER_E", &self.PER_E())
            .field("PER_M", &self.PER_M())
            .field("MAX_PER_E", &self.MAX_PER_E())
            .field("MAX_PER_M", &self.MAX_PER_M())
            .field("C1", &self.C1())
            .field("C2", &self.C2())
            .field("RESERVED24", &self.RESERVED24())
            .field("ADAPTIVE_EN", &self.ADAPTIVE_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RECHARGECFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RECHARGECFG {{ PER_E: {=u8:?}, PER_M: {=u8:?}, MAX_PER_E: {=u8:?}, MAX_PER_M: {=u8:?}, C1: {=u8:?}, C2: {=u8:?}, RESERVED24: {=u8:?}, ADAPTIVE_EN: {=bool:?} }}",
            self.PER_E(),
            self.PER_M(),
            self.MAX_PER_E(),
            self.MAX_PER_M(),
            self.C1(),
            self.C2(),
            self.RESERVED24(),
            self.ADAPTIVE_EN()
        )
    }
}
#[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RECHARGESTAT(pub u32);
impl RECHARGESTAT {
    #[doc = "15:0\\] The maximum value of recharge period seen with VDDR>threshold. The VDDR voltage is compared against the threshold voltage at just before each recharge. If VDDR is above threshold, MAX_USED_PER is updated with max ( current recharge peride; MAX_USED_PER ) This way MAX_USED_PER can track the recharge period where VDDR is decharged to the threshold value. We can therefore use the value as an indication of the leakage current during recharge. This bitfield is cleared to 0 when writing this register."]
    #[must_use]
    #[inline(always)]
    pub const fn MAX_USED_PER(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] The maximum value of recharge period seen with VDDR>threshold. The VDDR voltage is compared against the threshold voltage at just before each recharge. If VDDR is above threshold, MAX_USED_PER is updated with max ( current recharge peride; MAX_USED_PER ) This way MAX_USED_PER can track the recharge period where VDDR is decharged to the threshold value. We can therefore use the value as an indication of the leakage current during recharge. This bitfield is cleared to 0 when writing this register."]
    #[inline(always)]
    pub const fn set_MAX_USED_PER(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "19:16\\] The last 4 VDDR samples, bit 0 being the newest. The register is being updated in every recharge period with a shift left, and bit 0 is updated with the last VDDR sample, ie a 1 is shiftet in in case VDDR > VDDR_threshold just before recharge starts. Otherwise a 0 will be shifted in."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_SMPLS(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] The last 4 VDDR samples, bit 0 being the newest. The register is being updated in every recharge period with a shift left, and bit 0 is updated with the last VDDR sample, ie a 1 is shiftet in in case VDDR > VDDR_threshold just before recharge starts. Otherwise a 0 will be shifted in."]
    #[inline(always)]
    pub const fn set_VDDR_SMPLS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "31:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED20(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "31:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED20(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for RECHARGESTAT {
    #[inline(always)]
    fn default() -> RECHARGESTAT {
        RECHARGESTAT(0)
    }
}
impl core::fmt::Debug for RECHARGESTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RECHARGESTAT")
            .field("MAX_USED_PER", &self.MAX_USED_PER())
            .field("VDDR_SMPLS", &self.VDDR_SMPLS())
            .field("RESERVED20", &self.RESERVED20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RECHARGESTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RECHARGESTAT {{ MAX_USED_PER: {=u16:?}, VDDR_SMPLS: {=u8:?}, RESERVED20: {=u16:?} }}",
            self.MAX_USED_PER(),
            self.VDDR_SMPLS(),
            self.RESERVED20()
        )
    }
}
#[doc = "Shutdown Control This register contains bitfields required for entering shutdown mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SHUTDOWN(pub u32);
impl SHUTDOWN {
    #[doc = "0:0\\] Writing a 1 to this bit forces a shutdown request to be registered and all I/O values to be latched - in the PAD ring, possibly enabling I/O wakeup. Writing 0 will cancel a registered shutdown request and open th I/O latches residing in the PAD ring. A registered shutdown request takes effect the next time power down conditions exists. At this time, the will not enter Powerdown mode, but instead it will turn off all internal powersupplies, effectively putting the device into Shutdown mode."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Writing a 1 to this bit forces a shutdown request to be registered and all I/O values to be latched - in the PAD ring, possibly enabling I/O wakeup. Writing 0 will cancel a registered shutdown request and open th I/O latches residing in the PAD ring. A registered shutdown request takes effect the next time power down conditions exists. At this time, the will not enter Powerdown mode, but instead it will turn off all internal powersupplies, effectively putting the device into Shutdown mode."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
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
impl Default for SHUTDOWN {
    #[inline(always)]
    fn default() -> SHUTDOWN {
        SHUTDOWN(0)
    }
}
impl core::fmt::Debug for SHUTDOWN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHUTDOWN")
            .field("EN", &self.EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SHUTDOWN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SHUTDOWN {{ EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.EN(),
            self.RESERVED1()
        )
    }
}

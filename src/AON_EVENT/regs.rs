#[doc = "Wake-up Selector For AUX This register contains pointers to 3 events which are routed to AON_WUC as wakeup sources for AUX. AON_WUC will start a wakeup sequence for the AUX domain when either of the 3 selected events are asserted. A wakeup sequence will guarantee that the AUX power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for AUX. Note: It is recommended ( or required when AON_WUC:AUXCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before AUX is requesting powerdown. ( AUX_WUC:PWRDWNREQ.REQ is asserted\\] ) as it will speed up the wakeup procedure."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUXWUSEL(pub u32);
impl AUXWUSEL {
    #[doc = "5:0\\] AUX Wakeup Source #0 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:."]
    #[must_use]
    #[inline(always)]
    pub const fn WU0_EV(&self) -> super::vals::AUXWUSEL_WU0_EV {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::AUXWUSEL_WU0_EV::from_bits(val as u8)
    }
    #[doc = "5:0\\] AUX Wakeup Source #0 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:."]
    #[inline(always)]
    pub const fn set_WU0_EV(&mut self, val: super::vals::AUXWUSEL_WU0_EV) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "13:8\\] AUX Wakeup Source #1 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:."]
    #[must_use]
    #[inline(always)]
    pub const fn WU1_EV(&self) -> super::vals::AUXWUSEL_WU1_EV {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::AUXWUSEL_WU1_EV::from_bits(val as u8)
    }
    #[doc = "13:8\\] AUX Wakeup Source #1 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:."]
    #[inline(always)]
    pub const fn set_WU1_EV(&mut self, val: super::vals::AUXWUSEL_WU1_EV) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "15:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED14(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "15:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "21:16\\] AUX Wakeup Source #2 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:."]
    #[must_use]
    #[inline(always)]
    pub const fn WU2_EV(&self) -> super::vals::AUXWUSEL_WU2_EV {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::AUXWUSEL_WU2_EV::from_bits(val as u8)
    }
    #[doc = "21:16\\] AUX Wakeup Source #2 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:."]
    #[inline(always)]
    pub const fn set_WU2_EV(&mut self, val: super::vals::AUXWUSEL_WU2_EV) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "31:22\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED22(&self) -> u16 {
        let val = (self.0 >> 22usize) & 0x03ff;
        val as u16
    }
    #[doc = "31:22\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED22(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 22usize)) | (((val as u32) & 0x03ff) << 22usize);
    }
}
impl Default for AUXWUSEL {
    #[inline(always)]
    fn default() -> AUXWUSEL {
        AUXWUSEL(0)
    }
}
impl core::fmt::Debug for AUXWUSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUXWUSEL")
            .field("WU0_EV", &self.WU0_EV())
            .field("RESERVED6", &self.RESERVED6())
            .field("WU1_EV", &self.WU1_EV())
            .field("RESERVED14", &self.RESERVED14())
            .field("WU2_EV", &self.WU2_EV())
            .field("RESERVED22", &self.RESERVED22())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUXWUSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUXWUSEL {{ WU0_EV: {:?}, RESERVED6: {=u8:?}, WU1_EV: {:?}, RESERVED14: {=u8:?}, WU2_EV: {:?}, RESERVED22: {=u16:?} }}",
            self.WU0_EV(),
            self.RESERVED6(),
            self.WU1_EV(),
            self.RESERVED14(),
            self.WU2_EV(),
            self.RESERVED22()
        )
    }
}
#[doc = "Event Selector For MCU Event Fabric This register contains pointers for 3 AON events that are routed to the MCU Event Fabric EVENT."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVTOMCUSEL(pub u32);
impl EVTOMCUSEL {
    #[doc = "5:0\\] Event selector for AON_PROG0 event. AON Event Source id# selecting event routed to EVENT as AON_PROG0 event."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_PROG0_EV(&self) -> super::vals::AON_PROG0_EV {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::AON_PROG0_EV::from_bits(val as u8)
    }
    #[doc = "5:0\\] Event selector for AON_PROG0 event. AON Event Source id# selecting event routed to EVENT as AON_PROG0 event."]
    #[inline(always)]
    pub const fn set_AON_PROG0_EV(&mut self, val: super::vals::AON_PROG0_EV) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "13:8\\] Event selector for AON_PROG1 event. AON Event Source id# selecting event routed to EVENT as AON_PROG1 event."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_PROG1_EV(&self) -> super::vals::AON_PROG1_EV {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::AON_PROG1_EV::from_bits(val as u8)
    }
    #[doc = "13:8\\] Event selector for AON_PROG1 event. AON Event Source id# selecting event routed to EVENT as AON_PROG1 event."]
    #[inline(always)]
    pub const fn set_AON_PROG1_EV(&mut self, val: super::vals::AON_PROG1_EV) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "15:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED14(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "15:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "21:16\\] Event selector for AON_PROG2 event. AON Event Source id# selecting event routed to EVENT as AON_PROG2 event."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_PROG2_EV(&self) -> super::vals::AON_PROG2_EV {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::AON_PROG2_EV::from_bits(val as u8)
    }
    #[doc = "21:16\\] Event selector for AON_PROG2 event. AON Event Source id# selecting event routed to EVENT as AON_PROG2 event."]
    #[inline(always)]
    pub const fn set_AON_PROG2_EV(&mut self, val: super::vals::AON_PROG2_EV) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "31:22\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED22(&self) -> u16 {
        let val = (self.0 >> 22usize) & 0x03ff;
        val as u16
    }
    #[doc = "31:22\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED22(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 22usize)) | (((val as u32) & 0x03ff) << 22usize);
    }
}
impl Default for EVTOMCUSEL {
    #[inline(always)]
    fn default() -> EVTOMCUSEL {
        EVTOMCUSEL(0)
    }
}
impl core::fmt::Debug for EVTOMCUSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVTOMCUSEL")
            .field("AON_PROG0_EV", &self.AON_PROG0_EV())
            .field("RESERVED6", &self.RESERVED6())
            .field("AON_PROG1_EV", &self.AON_PROG1_EV())
            .field("RESERVED14", &self.RESERVED14())
            .field("AON_PROG2_EV", &self.AON_PROG2_EV())
            .field("RESERVED22", &self.RESERVED22())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EVTOMCUSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EVTOMCUSEL {{ AON_PROG0_EV: {:?}, RESERVED6: {=u8:?}, AON_PROG1_EV: {:?}, RESERVED14: {=u8:?}, AON_PROG2_EV: {:?}, RESERVED22: {=u16:?} }}",
            self.AON_PROG0_EV(),
            self.RESERVED6(),
            self.AON_PROG1_EV(),
            self.RESERVED14(),
            self.AON_PROG2_EV(),
            self.RESERVED22()
        )
    }
}
#[doc = "Wake-up Selector For MCU This register contains pointers to 4 events which are routed to AON_WUC as wakeup sources for MCU. AON_WUC will start a wakeup sequence for the MCU domain when either of the 4 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is recommended ( or required when AON_WUC:MCUCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before MCU is requesting powerdown. ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO ) as it will speed up the wakeup procedure."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MCUWUSEL(pub u32);
impl MCUWUSEL {
    #[doc = "5:0\\] MCU Wakeup Source #0 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:."]
    #[must_use]
    #[inline(always)]
    pub const fn WU0_EV(&self) -> super::vals::MCUWUSEL_WU0_EV {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::MCUWUSEL_WU0_EV::from_bits(val as u8)
    }
    #[doc = "5:0\\] MCU Wakeup Source #0 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:."]
    #[inline(always)]
    pub const fn set_WU0_EV(&mut self, val: super::vals::MCUWUSEL_WU0_EV) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "13:8\\] MCU Wakeup Source #1 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:."]
    #[must_use]
    #[inline(always)]
    pub const fn WU1_EV(&self) -> super::vals::MCUWUSEL_WU1_EV {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::MCUWUSEL_WU1_EV::from_bits(val as u8)
    }
    #[doc = "13:8\\] MCU Wakeup Source #1 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:."]
    #[inline(always)]
    pub const fn set_WU1_EV(&mut self, val: super::vals::MCUWUSEL_WU1_EV) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "15:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED14(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "15:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "21:16\\] MCU Wakeup Source #2 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:."]
    #[must_use]
    #[inline(always)]
    pub const fn WU2_EV(&self) -> super::vals::MCUWUSEL_WU2_EV {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::MCUWUSEL_WU2_EV::from_bits(val as u8)
    }
    #[doc = "21:16\\] MCU Wakeup Source #2 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:."]
    #[inline(always)]
    pub const fn set_WU2_EV(&mut self, val: super::vals::MCUWUSEL_WU2_EV) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
    }
    #[doc = "23:22\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED22(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "23:22\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED22(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "29:24\\] MCU Wakeup Source #3 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:."]
    #[must_use]
    #[inline(always)]
    pub const fn WU3_EV(&self) -> super::vals::WU3_EV {
        let val = (self.0 >> 24usize) & 0x3f;
        super::vals::WU3_EV::from_bits(val as u8)
    }
    #[doc = "29:24\\] MCU Wakeup Source #3 AON Event Source selecting 1 of 4 events routed to AON_WUC for waking up the MCU domain from Power Off or Power Down. Note:."]
    #[inline(always)]
    pub const fn set_WU3_EV(&mut self, val: super::vals::WU3_EV) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val.to_bits() as u32) & 0x3f) << 24usize);
    }
    #[doc = "31:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED30(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "31:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED30(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for MCUWUSEL {
    #[inline(always)]
    fn default() -> MCUWUSEL {
        MCUWUSEL(0)
    }
}
impl core::fmt::Debug for MCUWUSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCUWUSEL")
            .field("WU0_EV", &self.WU0_EV())
            .field("RESERVED6", &self.RESERVED6())
            .field("WU1_EV", &self.WU1_EV())
            .field("RESERVED14", &self.RESERVED14())
            .field("WU2_EV", &self.WU2_EV())
            .field("RESERVED22", &self.RESERVED22())
            .field("WU3_EV", &self.WU3_EV())
            .field("RESERVED30", &self.RESERVED30())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MCUWUSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MCUWUSEL {{ WU0_EV: {:?}, RESERVED6: {=u8:?}, WU1_EV: {:?}, RESERVED14: {=u8:?}, WU2_EV: {:?}, RESERVED22: {=u8:?}, WU3_EV: {:?}, RESERVED30: {=u8:?} }}",
            self.WU0_EV(),
            self.RESERVED6(),
            self.WU1_EV(),
            self.RESERVED14(),
            self.WU2_EV(),
            self.RESERVED22(),
            self.WU3_EV(),
            self.RESERVED30()
        )
    }
}
#[doc = "RTC Capture Event Selector For AON_RTC This register contains a pointer to select an AON event for RTC capture. Please refer to AON_RTC:CH1CAPT."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTCSEL(pub u32);
impl RTCSEL {
    #[doc = "5:0\\] AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_CH1_CAPT_EV(&self) -> super::vals::RTC_CH1_CAPT_EV {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::RTC_CH1_CAPT_EV::from_bits(val as u8)
    }
    #[doc = "5:0\\] AON Event Source id# for RTCSEL event which is fed to AON_RTC. Please refer to AON_RTC:CH1CAPT."]
    #[inline(always)]
    pub const fn set_RTC_CH1_CAPT_EV(&mut self, val: super::vals::RTC_CH1_CAPT_EV) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
impl Default for RTCSEL {
    #[inline(always)]
    fn default() -> RTCSEL {
        RTCSEL(0)
    }
}
impl core::fmt::Debug for RTCSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCSEL")
            .field("RTC_CH1_CAPT_EV", &self.RTC_CH1_CAPT_EV())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RTCSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RTCSEL {{ RTC_CH1_CAPT_EV: {:?}, RESERVED6: {=u32:?} }}",
            self.RTC_CH1_CAPT_EV(),
            self.RESERVED6()
        )
    }
}

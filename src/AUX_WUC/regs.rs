#[doc = "ADC Clock Control Controls the ADC internal clock Note that the ADC command and data interface requires MODCLKEN0.ANAIF or MODCLKEN1.ANAIF also to be set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADCCLKCTL(pub u32);
impl ADCCLKCTL {
    #[doc = "0:0\\] Enables(1) or disables (0) the ADC internal clock. This bit must not be modified unless ACK matches the current value."]
    #[must_use]
    #[inline(always)]
    pub const fn REQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Enables(1) or disables (0) the ADC internal clock. This bit must not be modified unless ACK matches the current value."]
    #[inline(always)]
    pub const fn set_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Acknowledges the last value written to REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn ACK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Acknowledges the last value written to REQ."]
    #[inline(always)]
    pub const fn set_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for ADCCLKCTL {
    #[inline(always)]
    fn default() -> ADCCLKCTL {
        ADCCLKCTL(0)
    }
}
impl core::fmt::Debug for ADCCLKCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCCLKCTL")
            .field("REQ", &self.REQ())
            .field("ACK", &self.ACK())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADCCLKCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADCCLKCTL {{ REQ: {=bool:?}, ACK: {=bool:?}, RESERVED: {=u32:?} }}",
            self.REQ(),
            self.ACK(),
            self.RESERVED()
        )
    }
}
#[doc = "AON Domain Control Status Status of AUX domain control from AON_WUC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AONCTLSTAT(pub u32);
impl AONCTLSTAT {
    #[doc = "0:0\\] Status of AON_WUC:AUX_CTL.SCE_RUN_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn SCE_RUN_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Status of AON_WUC:AUX_CTL.SCE_RUN_EN."]
    #[inline(always)]
    pub const fn set_SCE_RUN_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Status of AON_WUC:AUX_CTL.AUX_FORCE_ON."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_FORCE_ON(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Status of AON_WUC:AUX_CTL.AUX_FORCE_ON."]
    #[inline(always)]
    pub const fn set_AUX_FORCE_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for AONCTLSTAT {
    #[inline(always)]
    fn default() -> AONCTLSTAT {
        AONCTLSTAT(0)
    }
}
impl core::fmt::Debug for AONCTLSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AONCTLSTAT")
            .field("SCE_RUN_EN", &self.SCE_RUN_EN())
            .field("AUX_FORCE_ON", &self.AUX_FORCE_ON())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AONCTLSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AONCTLSTAT {{ SCE_RUN_EN: {=bool:?}, AUX_FORCE_ON: {=bool:?}, RESERVED: {=u32:?} }}",
            self.SCE_RUN_EN(),
            self.AUX_FORCE_ON(),
            self.RESERVED()
        )
    }
}
#[doc = "AUX Input Output Latch Controls latching of signals between AUX_AIODIO0/AUX_AIODIO1 and AON_IOC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUXIOLATCH(pub u32);
impl AUXIOLATCH {
    #[doc = "0:0\\] Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> super::vals::EN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EN::from_bits(val as u8)
    }
    #[doc = "0:0\\] Opens (1) or closes (0) the AUX_AIODIO0/AUX_AIODIO1 signal latching. At startup, set EN = TRANSP before configuring AUX_AIODIO0/AUX_AIODIO1 and subsequently selecting AUX mode in the AON_IOC. When powering off the AUX domain (using PWROFFREQ.REQ), set EN = STATIC in advance preserve the current state (mode and output value) of the I/O pins."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: super::vals::EN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
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
impl Default for AUXIOLATCH {
    #[inline(always)]
    fn default() -> AUXIOLATCH {
        AUXIOLATCH(0)
    }
}
impl core::fmt::Debug for AUXIOLATCH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUXIOLATCH")
            .field("EN", &self.EN())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUXIOLATCH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUXIOLATCH {{ EN: {:?}, RESERVED: {=u32:?} }}",
            self.EN(),
            self.RESERVED()
        )
    }
}
#[doc = "Low Frequency Clock Acknowledgment."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLKLFACK(pub u32);
impl CLKLFACK {
    #[doc = "0:0\\] Acknowledgment of CLKLFREQ.REQ 0: Acknowledgement that clock frequency is controlled by AON_WUC:AUXCLK and the system state 1: Acknowledgement that the low frequency clock SCLK_LF is the clock source for AUX."]
    #[must_use]
    #[inline(always)]
    pub const fn ACK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Acknowledgment of CLKLFREQ.REQ 0: Acknowledgement that clock frequency is controlled by AON_WUC:AUXCLK and the system state 1: Acknowledgement that the low frequency clock SCLK_LF is the clock source for AUX."]
    #[inline(always)]
    pub const fn set_ACK(&mut self, val: bool) {
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
impl Default for CLKLFACK {
    #[inline(always)]
    fn default() -> CLKLFACK {
        CLKLFACK(0)
    }
}
impl core::fmt::Debug for CLKLFACK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKLFACK")
            .field("ACK", &self.ACK())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLKLFACK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CLKLFACK {{ ACK: {=bool:?}, RESERVED: {=u32:?} }}",
            self.ACK(),
            self.RESERVED()
        )
    }
}
#[doc = "Low Frequency Clock Request."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLKLFREQ(pub u32);
impl CLKLFREQ {
    #[doc = "0:0\\] Low frequency request 0: Request clock frequency to be controlled by AON_WUC:AUXCLK and the system state 1: Request low frequency clock SCLK_LF as the clock source for AUX This bit must not be modified unless CLKLFACK.ACK matches the current value."]
    #[must_use]
    #[inline(always)]
    pub const fn REQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Low frequency request 0: Request clock frequency to be controlled by AON_WUC:AUXCLK and the system state 1: Request low frequency clock SCLK_LF as the clock source for AUX This bit must not be modified unless CLKLFACK.ACK matches the current value."]
    #[inline(always)]
    pub const fn set_REQ(&mut self, val: bool) {
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
impl Default for CLKLFREQ {
    #[inline(always)]
    fn default() -> CLKLFREQ {
        CLKLFREQ(0)
    }
}
impl core::fmt::Debug for CLKLFREQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKLFREQ")
            .field("REQ", &self.REQ())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLKLFREQ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CLKLFREQ {{ REQ: {=bool:?}, RESERVED: {=u32:?} }}",
            self.REQ(),
            self.RESERVED()
        )
    }
}
#[doc = "MCU Bus Control Controls the connection between the AUX domain bus and the MCU domain bus. The buses must be disconnected to allow power-down or power-off of the AUX domain."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MCUBUSCTL(pub u32);
impl MCUBUSCTL {
    #[doc = "0:0\\] Requests the AUX domain bus to be disconnected from the MCU domain bus. The request has no effect when AON_WUC:AUX_CTL.AUX_FORCE_ON is set. The disconnection status can be monitored through MCUBUSSTAT. Note however that this register cannot be read by the system CPU while disconnected. It is recommended that this bit is set and remains set after initial power-up, and that the system CPU uses AON_WUC:AUX_CTL.AUX_FORCE_ON to connect/disconnect the bus."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCONNECT_REQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Requests the AUX domain bus to be disconnected from the MCU domain bus. The request has no effect when AON_WUC:AUX_CTL.AUX_FORCE_ON is set. The disconnection status can be monitored through MCUBUSSTAT. Note however that this register cannot be read by the system CPU while disconnected. It is recommended that this bit is set and remains set after initial power-up, and that the system CPU uses AON_WUC:AUX_CTL.AUX_FORCE_ON to connect/disconnect the bus."]
    #[inline(always)]
    pub const fn set_DISCONNECT_REQ(&mut self, val: bool) {
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
impl Default for MCUBUSCTL {
    #[inline(always)]
    fn default() -> MCUBUSCTL {
        MCUBUSCTL(0)
    }
}
impl core::fmt::Debug for MCUBUSCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCUBUSCTL")
            .field("DISCONNECT_REQ", &self.DISCONNECT_REQ())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MCUBUSCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MCUBUSCTL {{ DISCONNECT_REQ: {=bool:?}, RESERVED: {=u32:?} }}",
            self.DISCONNECT_REQ(),
            self.RESERVED()
        )
    }
}
#[doc = "MCU Bus Status Indicates the connection state of the AUX domain and MCU domain buses. Note that this register cannot be read from the MCU domain while disconnected, and is therefore only useful for the AUX_SCE."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MCUBUSSTAT(pub u32);
impl MCUBUSSTAT {
    #[doc = "0:0\\] Acknowledges reception of the bus disconnection request, by matching the value of MCUBUSCTL.DISCONNECT_REQ. Note that if AON_WUC:AUXCTL.AUX_FORCE_ON = 1 a reconnect to the MCU domain bus will be made regardless of the state of MCUBUSCTL.DISCONNECT_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCONNECT_ACK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Acknowledges reception of the bus disconnection request, by matching the value of MCUBUSCTL.DISCONNECT_REQ. Note that if AON_WUC:AUXCTL.AUX_FORCE_ON = 1 a reconnect to the MCU domain bus will be made regardless of the state of MCUBUSCTL.DISCONNECT_REQ."]
    #[inline(always)]
    pub const fn set_DISCONNECT_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Indicates whether the AUX domain and MCU domain buses are currently disconnected (1) or connected (0)."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCONNECTED(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Indicates whether the AUX domain and MCU domain buses are currently disconnected (1) or connected (0)."]
    #[inline(always)]
    pub const fn set_DISCONNECTED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for MCUBUSSTAT {
    #[inline(always)]
    fn default() -> MCUBUSSTAT {
        MCUBUSSTAT(0)
    }
}
impl core::fmt::Debug for MCUBUSSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCUBUSSTAT")
            .field("DISCONNECT_ACK", &self.DISCONNECT_ACK())
            .field("DISCONNECTED", &self.DISCONNECTED())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MCUBUSSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MCUBUSSTAT {{ DISCONNECT_ACK: {=bool:?}, DISCONNECTED: {=bool:?}, RESERVED: {=u32:?} }}",
            self.DISCONNECT_ACK(),
            self.DISCONNECTED(),
            self.RESERVED()
        )
    }
}
#[doc = "Module Clock Enable Clock enable for each module in the AUX domain For use by the system CPU The settings in this register are OR'ed with the corresponding settings in MODCLKEN1. This allows the system CPU and AUX_SCE to request clocks independently. Settings take effect immediately."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MODCLKEN0(pub u32);
impl MODCLKEN0 {
    #[doc = "0:0\\] Enables (1) or disables (0) clock for AUX_SMPH."]
    #[must_use]
    #[inline(always)]
    pub const fn SMPH(&self) -> super::vals::MODCLKEN0_SMPH {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MODCLKEN0_SMPH::from_bits(val as u8)
    }
    #[doc = "0:0\\] Enables (1) or disables (0) clock for AUX_SMPH."]
    #[inline(always)]
    pub const fn set_SMPH(&mut self, val: super::vals::MODCLKEN0_SMPH) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Enables (1) or disables (0) clock for AUX_AIODIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn AIODIO0(&self) -> super::vals::MODCLKEN0_AIODIO0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::MODCLKEN0_AIODIO0::from_bits(val as u8)
    }
    #[doc = "1:1\\] Enables (1) or disables (0) clock for AUX_AIODIO0."]
    #[inline(always)]
    pub const fn set_AIODIO0(&mut self, val: super::vals::MODCLKEN0_AIODIO0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Enables (1) or disables (0) clock for AUX_AIODIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn AIODIO1(&self) -> super::vals::MODCLKEN0_AIODIO1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::MODCLKEN0_AIODIO1::from_bits(val as u8)
    }
    #[doc = "2:2\\] Enables (1) or disables (0) clock for AUX_AIODIO1."]
    #[inline(always)]
    pub const fn set_AIODIO1(&mut self, val: super::vals::MODCLKEN0_AIODIO1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Enables (1) or disables (0) clock for AUX_TIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER(&self) -> super::vals::MODCLKEN0_TIMER {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::MODCLKEN0_TIMER::from_bits(val as u8)
    }
    #[doc = "3:3\\] Enables (1) or disables (0) clock for AUX_TIMER."]
    #[inline(always)]
    pub const fn set_TIMER(&mut self, val: super::vals::MODCLKEN0_TIMER) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Enables (1) or disables (0) clock for AUX_ANAIF. Note that the ADC internal clock must be requested separately using ADCCLKCTL."]
    #[must_use]
    #[inline(always)]
    pub const fn ANAIF(&self) -> super::vals::MODCLKEN0_ANAIF {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::MODCLKEN0_ANAIF::from_bits(val as u8)
    }
    #[doc = "4:4\\] Enables (1) or disables (0) clock for AUX_ANAIF. Note that the ADC internal clock must be requested separately using ADCCLKCTL."]
    #[inline(always)]
    pub const fn set_ANAIF(&mut self, val: super::vals::MODCLKEN0_ANAIF) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Enables (1) or disables (0) clock for AUX_TDCIF. Note that the TDC counter and reference clock sources must be requested separately using TDCCLKCTL and REFCLKCTL, respectively."]
    #[must_use]
    #[inline(always)]
    pub const fn TDC(&self) -> super::vals::TDC {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::TDC::from_bits(val as u8)
    }
    #[doc = "5:5\\] Enables (1) or disables (0) clock for AUX_TDCIF. Note that the TDC counter and reference clock sources must be requested separately using TDCCLKCTL and REFCLKCTL, respectively."]
    #[inline(always)]
    pub const fn set_TDC(&mut self, val: super::vals::TDC) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Enables (1) or disables (0) clock for AUX_DDI0_OSC."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_DDI0_OSC(&self) -> super::vals::MODCLKEN0_AUX_DDI0_OSC {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::MODCLKEN0_AUX_DDI0_OSC::from_bits(val as u8)
    }
    #[doc = "6:6\\] Enables (1) or disables (0) clock for AUX_DDI0_OSC."]
    #[inline(always)]
    pub const fn set_AUX_DDI0_OSC(&mut self, val: super::vals::MODCLKEN0_AUX_DDI0_OSC) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Enables (1) or disables (0) clock for AUX_ADI4."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_ADI4(&self) -> super::vals::MODCLKEN0_AUX_ADI4 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::MODCLKEN0_AUX_ADI4::from_bits(val as u8)
    }
    #[doc = "7:7\\] Enables (1) or disables (0) clock for AUX_ADI4."]
    #[inline(always)]
    pub const fn set_AUX_ADI4(&mut self, val: super::vals::MODCLKEN0_AUX_ADI4) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for MODCLKEN0 {
    #[inline(always)]
    fn default() -> MODCLKEN0 {
        MODCLKEN0(0)
    }
}
impl core::fmt::Debug for MODCLKEN0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODCLKEN0")
            .field("SMPH", &self.SMPH())
            .field("AIODIO0", &self.AIODIO0())
            .field("AIODIO1", &self.AIODIO1())
            .field("TIMER", &self.TIMER())
            .field("ANAIF", &self.ANAIF())
            .field("TDC", &self.TDC())
            .field("AUX_DDI0_OSC", &self.AUX_DDI0_OSC())
            .field("AUX_ADI4", &self.AUX_ADI4())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MODCLKEN0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MODCLKEN0 {{ SMPH: {:?}, AIODIO0: {:?}, AIODIO1: {:?}, TIMER: {:?}, ANAIF: {:?}, TDC: {:?}, AUX_DDI0_OSC: {:?}, AUX_ADI4: {:?}, RESERVED: {=u32:?} }}",
            self.SMPH(),
            self.AIODIO0(),
            self.AIODIO1(),
            self.TIMER(),
            self.ANAIF(),
            self.TDC(),
            self.AUX_DDI0_OSC(),
            self.AUX_ADI4(),
            self.RESERVED()
        )
    }
}
#[doc = "Module Clock Enable 1 Clock enable for each module in the AUX domain, for use by the AUX_SCE. Settings take effect immediately. The settings in this register are OR'ed with the corresponding settings in MODCLKEN0. This allows system CPU and AUX_SCE to request clocks independently."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MODCLKEN1(pub u32);
impl MODCLKEN1 {
    #[doc = "0:0\\] Enables (1) or disables (0) clock for AUX_SMPH."]
    #[must_use]
    #[inline(always)]
    pub const fn SMPH(&self) -> super::vals::MODCLKEN1_SMPH {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MODCLKEN1_SMPH::from_bits(val as u8)
    }
    #[doc = "0:0\\] Enables (1) or disables (0) clock for AUX_SMPH."]
    #[inline(always)]
    pub const fn set_SMPH(&mut self, val: super::vals::MODCLKEN1_SMPH) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Enables (1) or disables (0) clock for AUX_AIODIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn AIODIO0(&self) -> super::vals::MODCLKEN1_AIODIO0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::MODCLKEN1_AIODIO0::from_bits(val as u8)
    }
    #[doc = "1:1\\] Enables (1) or disables (0) clock for AUX_AIODIO0."]
    #[inline(always)]
    pub const fn set_AIODIO0(&mut self, val: super::vals::MODCLKEN1_AIODIO0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Enables (1) or disables (0) clock for AUX_AIODIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn AIODIO1(&self) -> super::vals::MODCLKEN1_AIODIO1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::MODCLKEN1_AIODIO1::from_bits(val as u8)
    }
    #[doc = "2:2\\] Enables (1) or disables (0) clock for AUX_AIODIO1."]
    #[inline(always)]
    pub const fn set_AIODIO1(&mut self, val: super::vals::MODCLKEN1_AIODIO1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Enables (1) or disables (0) clock for AUX_TIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER(&self) -> super::vals::MODCLKEN1_TIMER {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::MODCLKEN1_TIMER::from_bits(val as u8)
    }
    #[doc = "3:3\\] Enables (1) or disables (0) clock for AUX_TIMER."]
    #[inline(always)]
    pub const fn set_TIMER(&mut self, val: super::vals::MODCLKEN1_TIMER) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Enables (1) or disables (0) clock for AUX_ANAIF."]
    #[must_use]
    #[inline(always)]
    pub const fn ANAIF(&self) -> super::vals::MODCLKEN1_ANAIF {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::MODCLKEN1_ANAIF::from_bits(val as u8)
    }
    #[doc = "4:4\\] Enables (1) or disables (0) clock for AUX_ANAIF."]
    #[inline(always)]
    pub const fn set_ANAIF(&mut self, val: super::vals::MODCLKEN1_ANAIF) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn TDC(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_TDC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Enables (1) or disables (0) clock for AUX_DDI0_OSC."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_DDI0_OSC(&self) -> super::vals::MODCLKEN1_AUX_DDI0_OSC {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::MODCLKEN1_AUX_DDI0_OSC::from_bits(val as u8)
    }
    #[doc = "6:6\\] Enables (1) or disables (0) clock for AUX_DDI0_OSC."]
    #[inline(always)]
    pub const fn set_AUX_DDI0_OSC(&mut self, val: super::vals::MODCLKEN1_AUX_DDI0_OSC) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Enables (1) or disables (0) clock for AUX_ADI4."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_ADI4(&self) -> super::vals::MODCLKEN1_AUX_ADI4 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::MODCLKEN1_AUX_ADI4::from_bits(val as u8)
    }
    #[doc = "7:7\\] Enables (1) or disables (0) clock for AUX_ADI4."]
    #[inline(always)]
    pub const fn set_AUX_ADI4(&mut self, val: super::vals::MODCLKEN1_AUX_ADI4) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for MODCLKEN1 {
    #[inline(always)]
    fn default() -> MODCLKEN1 {
        MODCLKEN1(0)
    }
}
impl core::fmt::Debug for MODCLKEN1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODCLKEN1")
            .field("SMPH", &self.SMPH())
            .field("AIODIO0", &self.AIODIO0())
            .field("AIODIO1", &self.AIODIO1())
            .field("TIMER", &self.TIMER())
            .field("ANAIF", &self.ANAIF())
            .field("TDC", &self.TDC())
            .field("AUX_DDI0_OSC", &self.AUX_DDI0_OSC())
            .field("AUX_ADI4", &self.AUX_ADI4())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MODCLKEN1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MODCLKEN1 {{ SMPH: {:?}, AIODIO0: {:?}, AIODIO1: {:?}, TIMER: {:?}, ANAIF: {:?}, TDC: {=bool:?}, AUX_DDI0_OSC: {:?}, AUX_ADI4: {:?}, RESERVED: {=u32:?} }}",
            self.SMPH(),
            self.AIODIO0(),
            self.AIODIO1(),
            self.TIMER(),
            self.ANAIF(),
            self.TDC(),
            self.AUX_DDI0_OSC(),
            self.AUX_ADI4(),
            self.RESERVED()
        )
    }
}
#[doc = "Power Down Acknowledgment."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWRDWNACK(pub u32);
impl PWRDWNACK {
    #[doc = "0:0\\] Power down acknowledgment. Indicates whether the power down request given by PWRDWNREQ.REQ is captured by the AON domain or not 0: AUX can assume that the system is in active mode 1: The request for power down is acknowledged and the AUX must act like the system is in power down mode and power supply is limited The system CPU cannot use this bit since the bus bridge between MCU domain and AUX domain is always disconnected when this bit is set. For AUX_SCE use only."]
    #[must_use]
    #[inline(always)]
    pub const fn ACK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Power down acknowledgment. Indicates whether the power down request given by PWRDWNREQ.REQ is captured by the AON domain or not 0: AUX can assume that the system is in active mode 1: The request for power down is acknowledged and the AUX must act like the system is in power down mode and power supply is limited The system CPU cannot use this bit since the bus bridge between MCU domain and AUX domain is always disconnected when this bit is set. For AUX_SCE use only."]
    #[inline(always)]
    pub const fn set_ACK(&mut self, val: bool) {
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
impl Default for PWRDWNACK {
    #[inline(always)]
    fn default() -> PWRDWNACK {
        PWRDWNACK(0)
    }
}
impl core::fmt::Debug for PWRDWNACK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRDWNACK")
            .field("ACK", &self.ACK())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWRDWNACK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWRDWNACK {{ ACK: {=bool:?}, RESERVED: {=u32:?} }}",
            self.ACK(),
            self.RESERVED()
        )
    }
}
#[doc = "Power Down Request Request from AUX for system to enter power down. When system is in power down there is limited current supply available and the clock source is set by AON_WUC:AUXCLK.PWR_DWN_SRC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWRDWNREQ(pub u32);
impl PWRDWNREQ {
    #[doc = "0:0\\] Power down request 0: Request for system to be in active mode 1: Request for system to be in power down mode When REQ is 1 one shall assume that the system is in power down, and that current supply is limited. When setting REQ = 0, one shall assume that the system is in power down until PWRDWNACK.ACK = 0."]
    #[must_use]
    #[inline(always)]
    pub const fn REQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Power down request 0: Request for system to be in active mode 1: Request for system to be in power down mode When REQ is 1 one shall assume that the system is in power down, and that current supply is limited. When setting REQ = 0, one shall assume that the system is in power down until PWRDWNACK.ACK = 0."]
    #[inline(always)]
    pub const fn set_REQ(&mut self, val: bool) {
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
impl Default for PWRDWNREQ {
    #[inline(always)]
    fn default() -> PWRDWNREQ {
        PWRDWNREQ(0)
    }
}
impl core::fmt::Debug for PWRDWNREQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRDWNREQ")
            .field("REQ", &self.REQ())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWRDWNREQ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWRDWNREQ {{ REQ: {=bool:?}, RESERVED: {=u32:?} }}",
            self.REQ(),
            self.RESERVED()
        )
    }
}
#[doc = "Power Off Request Requests power off request for the AUX domain. When powered off, the power supply and clock is disabled. This may only be used when taking the entire device into shutdown mode (i.e. with full device reset when resuming operation). Power off is prevented if AON_WUC:AUXCTL.AUX_FORCE_ON has been set, or if MCUBUSCTL.DISCONNECT_REQ has been cleared."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWROFFREQ(pub u32);
impl PWROFFREQ {
    #[doc = "0:0\\] Power off request 0: No action 1: Request to power down AUX. Once set, this bit shall not be cleared. The bit will be reset again when AUX is powered up again. The request will only happen if AONCTLSTAT.AUX_FORCE_ON = 0 and MCUBUSSTAT.DISCONNECTED=1."]
    #[must_use]
    #[inline(always)]
    pub const fn REQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Power off request 0: No action 1: Request to power down AUX. Once set, this bit shall not be cleared. The bit will be reset again when AUX is powered up again. The request will only happen if AONCTLSTAT.AUX_FORCE_ON = 0 and MCUBUSSTAT.DISCONNECTED=1."]
    #[inline(always)]
    pub const fn set_REQ(&mut self, val: bool) {
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
impl Default for PWROFFREQ {
    #[inline(always)]
    fn default() -> PWROFFREQ {
        PWROFFREQ(0)
    }
}
impl core::fmt::Debug for PWROFFREQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWROFFREQ")
            .field("REQ", &self.REQ())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWROFFREQ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWROFFREQ {{ REQ: {=bool:?}, RESERVED: {=u32:?} }}",
            self.REQ(),
            self.RESERVED()
        )
    }
}
#[doc = "Reference Clock Control Controls the TDC reference clock source, which is to be compared against the TDC counter clock. The source of this clock is controlled by OSC_DIG:CTL0.ACLK_REF_SRC_SEL."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct REFCLKCTL(pub u32);
impl REFCLKCTL {
    #[doc = "0:0\\] Enables(1) or disables (0) the TDC reference clock source. This bit must not be modified unless ACK matches the current value."]
    #[must_use]
    #[inline(always)]
    pub const fn REQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Enables(1) or disables (0) the TDC reference clock source. This bit must not be modified unless ACK matches the current value."]
    #[inline(always)]
    pub const fn set_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Acknowledges the last value written to REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn ACK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Acknowledges the last value written to REQ."]
    #[inline(always)]
    pub const fn set_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for REFCLKCTL {
    #[inline(always)]
    fn default() -> REFCLKCTL {
        REFCLKCTL(0)
    }
}
impl core::fmt::Debug for REFCLKCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REFCLKCTL")
            .field("REQ", &self.REQ())
            .field("ACK", &self.ACK())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for REFCLKCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "REFCLKCTL {{ REQ: {=bool:?}, ACK: {=bool:?}, RESERVED: {=u32:?} }}",
            self.REQ(),
            self.ACK(),
            self.RESERVED()
        )
    }
}
#[doc = "Real Time Counter Sub Second Increment 0 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 15:0. After setting INC15_0 and RTCSUBSECINC1.INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTCSUBSECINC0(pub u32);
impl RTCSUBSECINC0 {
    #[doc = "15:0\\] Bits 15:0 of the RTC sub-second increment value."]
    #[must_use]
    #[inline(always)]
    pub const fn INC15_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Bits 15:0 of the RTC sub-second increment value."]
    #[inline(always)]
    pub const fn set_INC15_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for RTCSUBSECINC0 {
    #[inline(always)]
    fn default() -> RTCSUBSECINC0 {
        RTCSUBSECINC0(0)
    }
}
impl core::fmt::Debug for RTCSUBSECINC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCSUBSECINC0")
            .field("INC15_0", &self.INC15_0())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RTCSUBSECINC0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RTCSUBSECINC0 {{ INC15_0: {=u16:?}, RESERVED: {=u16:?} }}",
            self.INC15_0(),
            self.RESERVED()
        )
    }
}
#[doc = "Real Time Counter Sub Second Increment 1 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 23:16. After setting RTCSUBSECINC0.INC15_0 and INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTCSUBSECINC1(pub u32);
impl RTCSUBSECINC1 {
    #[doc = "7:0\\] Bits 23:16 of the RTC sub-second increment value."]
    #[must_use]
    #[inline(always)]
    pub const fn INC23_16(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Bits 23:16 of the RTC sub-second increment value."]
    #[inline(always)]
    pub const fn set_INC23_16(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for RTCSUBSECINC1 {
    #[inline(always)]
    fn default() -> RTCSUBSECINC1 {
        RTCSUBSECINC1(0)
    }
}
impl core::fmt::Debug for RTCSUBSECINC1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCSUBSECINC1")
            .field("INC23_16", &self.INC23_16())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RTCSUBSECINC1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RTCSUBSECINC1 {{ INC23_16: {=u8:?}, RESERVED: {=u32:?} }}",
            self.INC23_16(),
            self.RESERVED()
        )
    }
}
#[doc = "Real Time Counter Sub Second Increment Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTCSUBSECINCCTL(pub u32);
impl RTCSUBSECINCCTL {
    #[doc = "0:0\\] Signal that a new real time counter sub second increment value is available 0: New sub second increment is not available 1: New sub second increment is available This bit must not be modified unless UPD_ACK matches the current value."]
    #[must_use]
    #[inline(always)]
    pub const fn UPD_REQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Signal that a new real time counter sub second increment value is available 0: New sub second increment is not available 1: New sub second increment is available This bit must not be modified unless UPD_ACK matches the current value."]
    #[inline(always)]
    pub const fn set_UPD_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Acknowledgment of the UPD_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn UPD_ACK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Acknowledgment of the UPD_REQ."]
    #[inline(always)]
    pub const fn set_UPD_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for RTCSUBSECINCCTL {
    #[inline(always)]
    fn default() -> RTCSUBSECINCCTL {
        RTCSUBSECINCCTL(0)
    }
}
impl core::fmt::Debug for RTCSUBSECINCCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCSUBSECINCCTL")
            .field("UPD_REQ", &self.UPD_REQ())
            .field("UPD_ACK", &self.UPD_ACK())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RTCSUBSECINCCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RTCSUBSECINCCTL {{ UPD_REQ: {=bool:?}, UPD_ACK: {=bool:?}, RESERVED: {=u32:?} }}",
            self.UPD_REQ(),
            self.UPD_ACK(),
            self.RESERVED()
        )
    }
}
#[doc = "TDC Clock Control Controls the TDC counter clock source, which steps the TDC counter value The source of this clock is controlled by OSC_DIG:CTL0.ACLK_TDC_SRC_SEL."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TDCCLKCTL(pub u32);
impl TDCCLKCTL {
    #[doc = "0:0\\] Enables(1) or disables (0) the TDC counter clock source. This bit must not be modified unless ACK matches the current value."]
    #[must_use]
    #[inline(always)]
    pub const fn REQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Enables(1) or disables (0) the TDC counter clock source. This bit must not be modified unless ACK matches the current value."]
    #[inline(always)]
    pub const fn set_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Acknowledges the last value written to REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn ACK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Acknowledges the last value written to REQ."]
    #[inline(always)]
    pub const fn set_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for TDCCLKCTL {
    #[inline(always)]
    fn default() -> TDCCLKCTL {
        TDCCLKCTL(0)
    }
}
impl core::fmt::Debug for TDCCLKCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDCCLKCTL")
            .field("REQ", &self.REQ())
            .field("ACK", &self.ACK())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TDCCLKCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TDCCLKCTL {{ REQ: {=bool:?}, ACK: {=bool:?}, RESERVED: {=u32:?} }}",
            self.REQ(),
            self.ACK(),
            self.RESERVED()
        )
    }
}
#[doc = "Wake-up Event Clear Clears wake-up events from the AON domain."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WUEVCLR(pub u32);
impl WUEVCLR {
    #[doc = "0:0\\] Set to clear the WUEVFLAGS.AON_PROG_WU wake-up event. Note only if an IO event is selected as wake-up event, is it possible to use this field to clear the source. Other sources cannot be cleared using this field. The IO pin needs to be assigned to AUX in the IOC and the input enable for the pin needs to be set in AIODIO0 or AIODIO1 for this clearing to take effect. This bit must remain set until WUEVFLAGS.AON_PROG_WU returns to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_PROG_WU(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Set to clear the WUEVFLAGS.AON_PROG_WU wake-up event. Note only if an IO event is selected as wake-up event, is it possible to use this field to clear the source. Other sources cannot be cleared using this field. The IO pin needs to be assigned to AUX in the IOC and the input enable for the pin needs to be set in AIODIO0 or AIODIO1 for this clearing to take effect. This bit must remain set until WUEVFLAGS.AON_PROG_WU returns to 0."]
    #[inline(always)]
    pub const fn set_AON_PROG_WU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Set to clear the WUEVFLAGS.AON_SW wake-up event. This bit must remain set until WUEVFLAGS.AON_SW returns to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_SW(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Set to clear the WUEVFLAGS.AON_SW wake-up event. This bit must remain set until WUEVFLAGS.AON_SW returns to 0."]
    #[inline(always)]
    pub const fn set_AON_SW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Set to clear the WUEVFLAGS.AON_RTC_CH2 wake-up event. Note that if RTC channel 2 is also set as source for AON_PROG_WU this field can also clear WUEVFLAGS.AON_PROG_WU This bit must remain set until WUEVFLAGS.AON_RTC_CH2 returns to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_RTC_CH2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Set to clear the WUEVFLAGS.AON_RTC_CH2 wake-up event. Note that if RTC channel 2 is also set as source for AON_PROG_WU this field can also clear WUEVFLAGS.AON_PROG_WU This bit must remain set until WUEVFLAGS.AON_RTC_CH2 returns to 0."]
    #[inline(always)]
    pub const fn set_AON_RTC_CH2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for WUEVCLR {
    #[inline(always)]
    fn default() -> WUEVCLR {
        WUEVCLR(0)
    }
}
impl core::fmt::Debug for WUEVCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUEVCLR")
            .field("AON_PROG_WU", &self.AON_PROG_WU())
            .field("AON_SW", &self.AON_SW())
            .field("AON_RTC_CH2", &self.AON_RTC_CH2())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WUEVCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WUEVCLR {{ AON_PROG_WU: {=bool:?}, AON_SW: {=bool:?}, AON_RTC_CH2: {=bool:?}, RESERVED: {=u32:?} }}",
            self.AON_PROG_WU(),
            self.AON_SW(),
            self.AON_RTC_CH2(),
            self.RESERVED()
        )
    }
}
#[doc = "Wake-up Event Flags Status of wake-up events from the AON domain The event flags are cleared by setting the corresponding bits in WUEVCLR."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WUEVFLAGS(pub u32);
impl WUEVFLAGS {
    #[doc = "0:0\\] Indicates pending event triggered by the sources selected in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV and AON_EVENT:AUXWUSEL.WU2_EV."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_PROG_WU(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Indicates pending event triggered by the sources selected in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV and AON_EVENT:AUXWUSEL.WU2_EV."]
    #[inline(always)]
    pub const fn set_AON_PROG_WU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Indicates pending event triggered by system CPU writing a 1 to AON_WUC:AUXCTL.SWEV."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_SW(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Indicates pending event triggered by system CPU writing a 1 to AON_WUC:AUXCTL.SWEV."]
    #[inline(always)]
    pub const fn set_AON_SW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Indicates pending event from AON_RTC_CH2 compare. Note that this flag will be set whenever the AON_RTC_CH2 event happens, but that does not mean that this event is a wake-up event. To make the AON_RTC_CH2 a wake-up event for the AUX domain configure it as a wake-up event in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV or AON_EVENT:AUXWUSEL.WU2_EV."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_RTC_CH2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Indicates pending event from AON_RTC_CH2 compare. Note that this flag will be set whenever the AON_RTC_CH2 event happens, but that does not mean that this event is a wake-up event. To make the AON_RTC_CH2 a wake-up event for the AUX domain configure it as a wake-up event in AON_EVENT:AUXWUSEL.WU0_EV, AON_EVENT:AUXWUSEL.WU1_EV or AON_EVENT:AUXWUSEL.WU2_EV."]
    #[inline(always)]
    pub const fn set_AON_RTC_CH2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for WUEVFLAGS {
    #[inline(always)]
    fn default() -> WUEVFLAGS {
        WUEVFLAGS(0)
    }
}
impl core::fmt::Debug for WUEVFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUEVFLAGS")
            .field("AON_PROG_WU", &self.AON_PROG_WU())
            .field("AON_SW", &self.AON_SW())
            .field("AON_RTC_CH2", &self.AON_RTC_CH2())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WUEVFLAGS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WUEVFLAGS {{ AON_PROG_WU: {=bool:?}, AON_SW: {=bool:?}, AON_RTC_CH2: {=bool:?}, RESERVED: {=u32:?} }}",
            self.AON_PROG_WU(),
            self.AON_SW(),
            self.AON_RTC_CH2(),
            self.RESERVED()
        )
    }
}

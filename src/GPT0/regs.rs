#[doc = "Combined CCP Output This register is used to logically AND CCP output pairs for each timer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANDCCP(pub u32);
impl ANDCCP {
    #[doc = "0:0\\] Enables AND operation of the CCP outputs for timers A and B. 0 : PWM outputs of Timer A and Timer B are the internal generated PWM signals of the respective timers. 1 : PWM output of Timer A is ANDed version of Timer A and Timer B PWM signals and Timer B PWM ouput is Timer B PWM signal only."]
    #[must_use]
    #[inline(always)]
    pub const fn CCP_AND_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Enables AND operation of the CCP outputs for timers A and B. 0 : PWM outputs of Timer A and Timer B are the internal generated PWM signals of the respective timers. 1 : PWM output of Timer A is ANDed version of Timer A and Timer B PWM signals and Timer B PWM ouput is Timer B PWM signal only."]
    #[inline(always)]
    pub const fn set_CCP_AND_EN(&mut self, val: bool) {
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
impl Default for ANDCCP {
    #[inline(always)]
    fn default() -> ANDCCP {
        ANDCCP(0)
    }
}
impl core::fmt::Debug for ANDCCP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANDCCP")
            .field("CCP_AND_EN", &self.CCP_AND_EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANDCCP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANDCCP {{ CCP_AND_EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.CCP_AND_EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG(pub u32);
impl CFG {
    #[doc = "2:0\\] GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG(&self) -> super::vals::CFG {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::CFG::from_bits(val as u8)
    }
    #[doc = "2:0\\] GPT Configuration 0x2- 0x3 - Reserved 0x5- 0x7 - Reserved."]
    #[inline(always)]
    pub const fn set_CFG(&mut self, val: super::vals::CFG) {
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
impl Default for CFG {
    #[inline(always)]
    fn default() -> CFG {
        CFG(0)
    }
}
impl core::fmt::Debug for CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("CFG", &self.CFG())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFG {{ CFG: {:?}, RESERVED3: {=u32:?} }}",
            self.CFG(),
            self.RESERVED3()
        )
    }
}
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTL(pub u32);
impl CTL {
    #[doc = "0:0\\] GPT Timer A Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TAEN(&self) -> super::vals::TAEN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TAEN::from_bits(val as u8)
    }
    #[doc = "0:0\\] GPT Timer A Enable."]
    #[inline(always)]
    pub const fn set_TAEN(&mut self, val: super::vals::TAEN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] GPT Timer A Stall Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TASTALL(&self) -> super::vals::TASTALL {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::TASTALL::from_bits(val as u8)
    }
    #[doc = "1:1\\] GPT Timer A Stall Enable."]
    #[inline(always)]
    pub const fn set_TASTALL(&mut self, val: super::vals::TASTALL) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "3:2\\] GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[must_use]
    #[inline(always)]
    pub const fn TAEVENT(&self) -> super::vals::TAEVENT {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::TAEVENT::from_bits(val as u8)
    }
    #[doc = "3:2\\] GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline(always)]
    pub const fn set_TAEVENT(&mut self, val: super::vals::TAEVENT) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "5:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "5:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "6:6\\] GPT Timer A PWM Output Level."]
    #[must_use]
    #[inline(always)]
    pub const fn TAPWML(&self) -> super::vals::TAPWML {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::TAPWML::from_bits(val as u8)
    }
    #[doc = "6:6\\] GPT Timer A PWM Output Level."]
    #[inline(always)]
    pub const fn set_TAPWML(&mut self, val: super::vals::TAPWML) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] GPT Timer B Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TBEN(&self) -> super::vals::TBEN {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::TBEN::from_bits(val as u8)
    }
    #[doc = "8:8\\] GPT Timer B Enable."]
    #[inline(always)]
    pub const fn set_TBEN(&mut self, val: super::vals::TBEN) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] GPT Timer B Stall Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TBSTALL(&self) -> super::vals::TBSTALL {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::TBSTALL::from_bits(val as u8)
    }
    #[doc = "9:9\\] GPT Timer B Stall Enable."]
    #[inline(always)]
    pub const fn set_TBSTALL(&mut self, val: super::vals::TBSTALL) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "11:10\\] GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[must_use]
    #[inline(always)]
    pub const fn TBEVENT(&self) -> super::vals::TBEVENT {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::TBEVENT::from_bits(val as u8)
    }
    #[doc = "11:10\\] GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline(always)]
    pub const fn set_TBEVENT(&mut self, val: super::vals::TBEVENT) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "13:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "13:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "14:14\\] GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted."]
    #[must_use]
    #[inline(always)]
    pub const fn TBPWML(&self) -> super::vals::TBPWML {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::TBPWML::from_bits(val as u8)
    }
    #[doc = "14:14\\] GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted."]
    #[inline(always)]
    pub const fn set_TBPWML(&mut self, val: super::vals::TBPWML) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "31:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> u32 {
        let val = (self.0 >> 15usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "31:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 15usize)) | (((val as u32) & 0x0001_ffff) << 15usize);
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
            .field("TAEN", &self.TAEN())
            .field("TASTALL", &self.TASTALL())
            .field("TAEVENT", &self.TAEVENT())
            .field("RESERVED4", &self.RESERVED4())
            .field("TAPWML", &self.TAPWML())
            .field("RESERVED7", &self.RESERVED7())
            .field("TBEN", &self.TBEN())
            .field("TBSTALL", &self.TBSTALL())
            .field("TBEVENT", &self.TBEVENT())
            .field("RESERVED12", &self.RESERVED12())
            .field("TBPWML", &self.TBPWML())
            .field("RESERVED15", &self.RESERVED15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTL {{ TAEN: {:?}, TASTALL: {:?}, TAEVENT: {:?}, RESERVED4: {=u8:?}, TAPWML: {:?}, RESERVED7: {=bool:?}, TBEN: {:?}, TBSTALL: {:?}, TBEVENT: {:?}, RESERVED12: {=u8:?}, TBPWML: {:?}, RESERVED15: {=u32:?} }}",
            self.TAEN(),
            self.TASTALL(),
            self.TAEVENT(),
            self.RESERVED4(),
            self.TAPWML(),
            self.RESERVED7(),
            self.TBEN(),
            self.TBSTALL(),
            self.TBEVENT(),
            self.RESERVED12(),
            self.TBPWML(),
            self.RESERVED15()
        )
    }
}
#[doc = "DMA Event This register allows software to enable/disable GPT DMA trigger events."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMAEV(pub u32);
impl DMAEV {
    #[doc = "0:0\\] GPT Timer A Time-Out DMA Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TATODMAEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] GPT Timer A Time-Out DMA Trigger Enable."]
    #[inline(always)]
    pub const fn set_TATODMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] GPT Timer A Capture Match DMA Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CAMDMAEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] GPT Timer A Capture Match DMA Trigger Enable."]
    #[inline(always)]
    pub const fn set_CAMDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] GPT Timer A Capture Event DMA Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CAEDMAEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] GPT Timer A Capture Event DMA Trigger Enable."]
    #[inline(always)]
    pub const fn set_CAEDMAEN(&mut self, val: bool) {
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
    #[doc = "4:4\\] GPT Timer A Match DMA Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TAMDMAEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] GPT Timer A Match DMA Trigger Enable."]
    #[inline(always)]
    pub const fn set_TAMDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "7:5\\] Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "7:5\\] Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "8:8\\] GPT Timer B Time-Out DMA Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TBTODMAEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] GPT Timer B Time-Out DMA Trigger Enable."]
    #[inline(always)]
    pub const fn set_TBTODMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] GPT Timer B Capture Match DMA Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CBMDMAEN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] GPT Timer B Capture Match DMA Trigger Enable."]
    #[inline(always)]
    pub const fn set_CBMDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] GPT Timer B Capture Event DMA Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CBEDMAEN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] GPT Timer B Capture Event DMA Trigger Enable."]
    #[inline(always)]
    pub const fn set_CBEDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] GPT Timer B Match DMA Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TBMDMAEN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] GPT Timer B Match DMA Trigger Enable."]
    #[inline(always)]
    pub const fn set_TBMDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "31:12\\] Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "31:12\\] Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for DMAEV {
    #[inline(always)]
    fn default() -> DMAEV {
        DMAEV(0)
    }
}
impl core::fmt::Debug for DMAEV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAEV")
            .field("TATODMAEN", &self.TATODMAEN())
            .field("CAMDMAEN", &self.CAMDMAEN())
            .field("CAEDMAEN", &self.CAEDMAEN())
            .field("RESERVED3", &self.RESERVED3())
            .field("TAMDMAEN", &self.TAMDMAEN())
            .field("RESERVED5", &self.RESERVED5())
            .field("TBTODMAEN", &self.TBTODMAEN())
            .field("CBMDMAEN", &self.CBMDMAEN())
            .field("CBEDMAEN", &self.CBEDMAEN())
            .field("TBMDMAEN", &self.TBMDMAEN())
            .field("RESERVED12", &self.RESERVED12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMAEV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMAEV {{ TATODMAEN: {=bool:?}, CAMDMAEN: {=bool:?}, CAEDMAEN: {=bool:?}, RESERVED3: {=bool:?}, TAMDMAEN: {=bool:?}, RESERVED5: {=u8:?}, TBTODMAEN: {=bool:?}, CBMDMAEN: {=bool:?}, CBEDMAEN: {=bool:?}, TBMDMAEN: {=bool:?}, RESERVED12: {=u32:?} }}",
            self.TATODMAEN(),
            self.CAMDMAEN(),
            self.CAEDMAEN(),
            self.RESERVED3(),
            self.TAMDMAEN(),
            self.RESERVED5(),
            self.TBTODMAEN(),
            self.CBMDMAEN(),
            self.CBEDMAEN(),
            self.TBMDMAEN(),
            self.RESERVED12()
        )
    }
}
#[doc = "Interrupt Clear This register is used to clear status bits in the RIS and MIS registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ICLR(pub u32);
impl ICLR {
    #[doc = "0:0\\] 0: Do nothing. 1: Clear RIS.TATORIS and MIS.TATOMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn TATOCINT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Do nothing. 1: Clear RIS.TATORIS and MIS.TATOMIS."]
    #[inline(always)]
    pub const fn set_TATOCINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 0: Do nothing. 1: Clear RIS.CAMRIS and MIS.CAMMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn CAMCINT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 0: Do nothing. 1: Clear RIS.CAMRIS and MIS.CAMMIS."]
    #[inline(always)]
    pub const fn set_CAMCINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] 0: Do nothing. 1: Clear RIS.CAERIS and MIS.CAEMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn CAECINT(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] 0: Do nothing. 1: Clear RIS.CAERIS and MIS.CAEMIS."]
    #[inline(always)]
    pub const fn set_CAECINT(&mut self, val: bool) {
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
    #[doc = "4:4\\] 0: Do nothing. 1: Clear RIS.TAMRIS and MIS.TAMMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn TAMCINT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] 0: Do nothing. 1: Clear RIS.TAMRIS and MIS.TAMMIS."]
    #[inline(always)]
    pub const fn set_TAMCINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] 0: Do nothing. 1: Clear RIS.DMAARIS and MIS.DMAAMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn DMAAINT(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] 0: Do nothing. 1: Clear RIS.DMAARIS and MIS.DMAAMIS."]
    #[inline(always)]
    pub const fn set_DMAAINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
    #[doc = "8:8\\] 0: Do nothing. 1: Clear RIS.TBTORIS and MIS.TBTOMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn TBTOCINT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] 0: Do nothing. 1: Clear RIS.TBTORIS and MIS.TBTOMIS."]
    #[inline(always)]
    pub const fn set_TBTOCINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] 0: Do nothing. 1: Clear RIS.CBMRIS and MIS.CBMMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn CBMCINT(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] 0: Do nothing. 1: Clear RIS.CBMRIS and MIS.CBMMIS."]
    #[inline(always)]
    pub const fn set_CBMCINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] 0: Do nothing. 1: Clear RIS.CBERIS and MIS.CBEMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn CBECINT(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] 0: Do nothing. 1: Clear RIS.CBERIS and MIS.CBEMIS."]
    #[inline(always)]
    pub const fn set_CBECINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] 0: Do nothing. 1: Clear RIS.TBMRIS and MIS.TBMMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn TBMCINT(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] 0: Do nothing. 1: Clear RIS.TBMRIS and MIS.TBMMIS."]
    #[inline(always)]
    pub const fn set_TBMCINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] 0: Do nothing. 1: Clear RIS.DMABRIS and MIS.DMABMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn DMABINT(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] 0: Do nothing. 1: Clear RIS.DMABRIS and MIS.DMABMIS."]
    #[inline(always)]
    pub const fn set_DMABINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
impl Default for ICLR {
    #[inline(always)]
    fn default() -> ICLR {
        ICLR(0)
    }
}
impl core::fmt::Debug for ICLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICLR")
            .field("TATOCINT", &self.TATOCINT())
            .field("CAMCINT", &self.CAMCINT())
            .field("CAECINT", &self.CAECINT())
            .field("RESERVED3", &self.RESERVED3())
            .field("TAMCINT", &self.TAMCINT())
            .field("DMAAINT", &self.DMAAINT())
            .field("RESERVED6", &self.RESERVED6())
            .field("TBTOCINT", &self.TBTOCINT())
            .field("CBMCINT", &self.CBMCINT())
            .field("CBECINT", &self.CBECINT())
            .field("TBMCINT", &self.TBMCINT())
            .field("RESERVED12", &self.RESERVED12())
            .field("DMABINT", &self.DMABINT())
            .field("RESERVED14", &self.RESERVED14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ICLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ICLR {{ TATOCINT: {=bool:?}, CAMCINT: {=bool:?}, CAECINT: {=bool:?}, RESERVED3: {=bool:?}, TAMCINT: {=bool:?}, DMAAINT: {=bool:?}, RESERVED6: {=u8:?}, TBTOCINT: {=bool:?}, CBMCINT: {=bool:?}, CBECINT: {=bool:?}, TBMCINT: {=bool:?}, RESERVED12: {=bool:?}, DMABINT: {=bool:?}, RESERVED14: {=u32:?} }}",
            self.TATOCINT(),
            self.CAMCINT(),
            self.CAECINT(),
            self.RESERVED3(),
            self.TAMCINT(),
            self.DMAAINT(),
            self.RESERVED6(),
            self.TBTOCINT(),
            self.CBMCINT(),
            self.CBECINT(),
            self.TBMCINT(),
            self.RESERVED12(),
            self.DMABINT(),
            self.RESERVED14()
        )
    }
}
#[doc = "Interrupt Mask This register is used to enable the interrupts. Associated registers: RIS, MIS, ICLR."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IMR(pub u32);
impl IMR {
    #[doc = "0:0\\] Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn TATOIM(&self) -> super::vals::TATOIM {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TATOIM::from_bits(val as u8)
    }
    #[doc = "0:0\\] Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS."]
    #[inline(always)]
    pub const fn set_TATOIM(&mut self, val: super::vals::TATOIM) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn CAMIM(&self) -> super::vals::CAMIM {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::CAMIM::from_bits(val as u8)
    }
    #[doc = "1:1\\] Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS."]
    #[inline(always)]
    pub const fn set_CAMIM(&mut self, val: super::vals::CAMIM) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn CAEIM(&self) -> super::vals::CAEIM {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CAEIM::from_bits(val as u8)
    }
    #[doc = "2:2\\] Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS."]
    #[inline(always)]
    pub const fn set_CAEIM(&mut self, val: super::vals::CAEIM) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
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
    #[doc = "4:4\\] Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn TAMIM(&self) -> super::vals::TAMIM {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::TAMIM::from_bits(val as u8)
    }
    #[doc = "4:4\\] Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS."]
    #[inline(always)]
    pub const fn set_TAMIM(&mut self, val: super::vals::TAMIM) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn DMAAIM(&self) -> super::vals::DMAAIM {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::DMAAIM::from_bits(val as u8)
    }
    #[doc = "5:5\\] Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS."]
    #[inline(always)]
    pub const fn set_DMAAIM(&mut self, val: super::vals::DMAAIM) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
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
    #[doc = "8:8\\] Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn TBTOIM(&self) -> super::vals::TBTOIM {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::TBTOIM::from_bits(val as u8)
    }
    #[doc = "8:8\\] Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS."]
    #[inline(always)]
    pub const fn set_TBTOIM(&mut self, val: super::vals::TBTOIM) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn CBMIM(&self) -> super::vals::CBMIM {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::CBMIM::from_bits(val as u8)
    }
    #[doc = "9:9\\] Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS."]
    #[inline(always)]
    pub const fn set_CBMIM(&mut self, val: super::vals::CBMIM) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn CBEIM(&self) -> super::vals::CBEIM {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::CBEIM::from_bits(val as u8)
    }
    #[doc = "10:10\\] Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS."]
    #[inline(always)]
    pub const fn set_CBEIM(&mut self, val: super::vals::CBEIM) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn TBMIM(&self) -> super::vals::TBMIM {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::TBMIM::from_bits(val as u8)
    }
    #[doc = "11:11\\] Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS."]
    #[inline(always)]
    pub const fn set_TBMIM(&mut self, val: super::vals::TBMIM) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn DMABIM(&self) -> super::vals::DMABIM {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::DMABIM::from_bits(val as u8)
    }
    #[doc = "13:13\\] Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS."]
    #[inline(always)]
    pub const fn set_DMABIM(&mut self, val: super::vals::DMABIM) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
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
impl Default for IMR {
    #[inline(always)]
    fn default() -> IMR {
        IMR(0)
    }
}
impl core::fmt::Debug for IMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMR")
            .field("TATOIM", &self.TATOIM())
            .field("CAMIM", &self.CAMIM())
            .field("CAEIM", &self.CAEIM())
            .field("RESERVED3", &self.RESERVED3())
            .field("TAMIM", &self.TAMIM())
            .field("DMAAIM", &self.DMAAIM())
            .field("RESERVED6", &self.RESERVED6())
            .field("TBTOIM", &self.TBTOIM())
            .field("CBMIM", &self.CBMIM())
            .field("CBEIM", &self.CBEIM())
            .field("TBMIM", &self.TBMIM())
            .field("RESERVED12", &self.RESERVED12())
            .field("DMABIM", &self.DMABIM())
            .field("RESERVED14", &self.RESERVED14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IMR {{ TATOIM: {:?}, CAMIM: {:?}, CAEIM: {:?}, RESERVED3: {=bool:?}, TAMIM: {:?}, DMAAIM: {:?}, RESERVED6: {=u8:?}, TBTOIM: {:?}, CBMIM: {:?}, CBEIM: {:?}, TBMIM: {:?}, RESERVED12: {=bool:?}, DMABIM: {:?}, RESERVED14: {=u32:?} }}",
            self.TATOIM(),
            self.CAMIM(),
            self.CAEIM(),
            self.RESERVED3(),
            self.TAMIM(),
            self.DMAAIM(),
            self.RESERVED6(),
            self.TBTOIM(),
            self.CBMIM(),
            self.CBEIM(),
            self.TBMIM(),
            self.RESERVED12(),
            self.DMABIM(),
            self.RESERVED14()
        )
    }
}
#[doc = "Masked Interrupt Status Values are result of bitwise AND operation between RIS and IMR Assosciated clear register: ICLR."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MIS(pub u32);
impl MIS {
    #[doc = "0:0\\] 0: No interrupt or interrupt not enabled 1: RIS.TATORIS = 1 && IMR.TATOIM = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn TATOMIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: No interrupt or interrupt not enabled 1: RIS.TATORIS = 1 && IMR.TATOIM = 1."]
    #[inline(always)]
    pub const fn set_TATOMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 0: No interrupt or interrupt not enabled 1: RIS.CAMRIS = 1 && IMR.CAMIM = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn CAMMIS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 0: No interrupt or interrupt not enabled 1: RIS.CAMRIS = 1 && IMR.CAMIM = 1."]
    #[inline(always)]
    pub const fn set_CAMMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] 0: No interrupt or interrupt not enabled 1: RIS.CAERIS = 1 && IMR.CAEIM = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn CAEMIS(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] 0: No interrupt or interrupt not enabled 1: RIS.CAERIS = 1 && IMR.CAEIM = 1."]
    #[inline(always)]
    pub const fn set_CAEMIS(&mut self, val: bool) {
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
    #[doc = "4:4\\] 0: No interrupt or interrupt not enabled 1: RIS.TAMRIS = 1 && IMR.TAMIM = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn TAMMIS(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] 0: No interrupt or interrupt not enabled 1: RIS.TAMRIS = 1 && IMR.TAMIM = 1."]
    #[inline(always)]
    pub const fn set_TAMMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] 0: No interrupt or interrupt not enabled 1: RIS.DMAARIS = 1 && IMR.DMAAIM = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn DMAAMIS(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] 0: No interrupt or interrupt not enabled 1: RIS.DMAARIS = 1 && IMR.DMAAIM = 1."]
    #[inline(always)]
    pub const fn set_DMAAMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
    #[doc = "8:8\\] 0: No interrupt or interrupt not enabled 1: RIS.TBTORIS = 1 && IMR.TBTOIM = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn TBTOMIS(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] 0: No interrupt or interrupt not enabled 1: RIS.TBTORIS = 1 && IMR.TBTOIM = 1."]
    #[inline(always)]
    pub const fn set_TBTOMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] 0: No interrupt or interrupt not enabled 1: RIS.CBMRIS = 1 && IMR.CBMIM = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn CBMMIS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] 0: No interrupt or interrupt not enabled 1: RIS.CBMRIS = 1 && IMR.CBMIM = 1."]
    #[inline(always)]
    pub const fn set_CBMMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] 0: No interrupt or interrupt not enabled 1: RIS.CBERIS = 1 && IMR.CBEIM = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn CBEMIS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] 0: No interrupt or interrupt not enabled 1: RIS.CBERIS = 1 && IMR.CBEIM = 1."]
    #[inline(always)]
    pub const fn set_CBEMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] 0: No interrupt or interrupt not enabled 1: RIS.TBMRIS = 1 && IMR.TBMIM = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn TBMMIS(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] 0: No interrupt or interrupt not enabled 1: RIS.TBMRIS = 1 && IMR.TBMIM = 1."]
    #[inline(always)]
    pub const fn set_TBMMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] 0: No interrupt or interrupt not enabled 1: RIS.DMABRIS = 1 && IMR.DMABIM = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn DMABMIS(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] 0: No interrupt or interrupt not enabled 1: RIS.DMABRIS = 1 && IMR.DMABIM = 1."]
    #[inline(always)]
    pub const fn set_DMABMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
impl Default for MIS {
    #[inline(always)]
    fn default() -> MIS {
        MIS(0)
    }
}
impl core::fmt::Debug for MIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIS")
            .field("TATOMIS", &self.TATOMIS())
            .field("CAMMIS", &self.CAMMIS())
            .field("CAEMIS", &self.CAEMIS())
            .field("RESERVED3", &self.RESERVED3())
            .field("TAMMIS", &self.TAMMIS())
            .field("DMAAMIS", &self.DMAAMIS())
            .field("RESERVED6", &self.RESERVED6())
            .field("TBTOMIS", &self.TBTOMIS())
            .field("CBMMIS", &self.CBMMIS())
            .field("CBEMIS", &self.CBEMIS())
            .field("TBMMIS", &self.TBMMIS())
            .field("RESERVED12", &self.RESERVED12())
            .field("DMABMIS", &self.DMABMIS())
            .field("RESERVED14", &self.RESERVED14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MIS {{ TATOMIS: {=bool:?}, CAMMIS: {=bool:?}, CAEMIS: {=bool:?}, RESERVED3: {=bool:?}, TAMMIS: {=bool:?}, DMAAMIS: {=bool:?}, RESERVED6: {=u8:?}, TBTOMIS: {=bool:?}, CBMMIS: {=bool:?}, CBEMIS: {=bool:?}, TBMMIS: {=bool:?}, RESERVED12: {=bool:?}, DMABMIS: {=bool:?}, RESERVED14: {=u32:?} }}",
            self.TATOMIS(),
            self.CAMMIS(),
            self.CAEMIS(),
            self.RESERVED3(),
            self.TAMMIS(),
            self.DMAAMIS(),
            self.RESERVED6(),
            self.TBTOMIS(),
            self.CBMMIS(),
            self.CBEMIS(),
            self.TBMMIS(),
            self.RESERVED12(),
            self.DMABMIS(),
            self.RESERVED14()
        )
    }
}
#[doc = "Raw Interrupt Status Associated registers: IMR, MIS, ICLR."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RIS(pub u32);
impl RIS {
    #[doc = "0:0\\] GPT Timer A Time-out Raw Interrupt 0: Timer A has not timed out 1: Timer A has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TAILR, depending on the count direction."]
    #[must_use]
    #[inline(always)]
    pub const fn TATORIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] GPT Timer A Time-out Raw Interrupt 0: Timer A has not timed out 1: Timer A has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TAILR, depending on the count direction."]
    #[inline(always)]
    pub const fn set_TATORIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] GPT Timer A Capture Mode Match Raw Interrupt 0: The capture mode match for Timer A has not occurred. 1: A capture mode match has occurred for Timer A. This interrupt asserts when the values in the TAR and TAPR match the values in the TAMATCHR and TAPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CAMCINT bit."]
    #[must_use]
    #[inline(always)]
    pub const fn CAMRIS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] GPT Timer A Capture Mode Match Raw Interrupt 0: The capture mode match for Timer A has not occurred. 1: A capture mode match has occurred for Timer A. This interrupt asserts when the values in the TAR and TAPR match the values in the TAMATCHR and TAPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CAMCINT bit."]
    #[inline(always)]
    pub const fn set_CAMRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] GPT Timer A Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode."]
    #[must_use]
    #[inline(always)]
    pub const fn CAERIS(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] GPT Timer A Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode."]
    #[inline(always)]
    pub const fn set_CAERIS(&mut self, val: bool) {
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
    #[doc = "4:4\\] GPT Timer A Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TAMR.TAMIE is set, and the match values in TAMATCHR and optionally TAPMR have been reached when configured in one-shot or periodic mode."]
    #[must_use]
    #[inline(always)]
    pub const fn TAMRIS(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] GPT Timer A Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TAMR.TAMIE is set, and the match values in TAMATCHR and optionally TAPMR have been reached when configured in one-shot or periodic mode."]
    #[inline(always)]
    pub const fn set_TAMRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] GPT Timer A DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed."]
    #[must_use]
    #[inline(always)]
    pub const fn DMAARIS(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] GPT Timer A DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed."]
    #[inline(always)]
    pub const fn set_DMAARIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
    #[doc = "8:8\\] GPT Timer B Time-out Raw Interrupt 0: Timer B has not timed out 1: Timer B has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TBILR, depending on the count direction."]
    #[must_use]
    #[inline(always)]
    pub const fn TBTORIS(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] GPT Timer B Time-out Raw Interrupt 0: Timer B has not timed out 1: Timer B has timed out. This interrupt is asserted when a one-shot or periodic mode timer reaches its count limit. The count limit is 0 or the value loaded into TBILR, depending on the count direction."]
    #[inline(always)]
    pub const fn set_TBTORIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] GPT Timer B Capture Mode Match Raw Interrupt 0: The capture mode match for Timer B has not occurred. 1: A capture mode match has occurred for Timer B. This interrupt asserts when the values in the TBR and TBPR match the values in the TBMATCHR and TBPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CBMCINT bit."]
    #[must_use]
    #[inline(always)]
    pub const fn CBMRIS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] GPT Timer B Capture Mode Match Raw Interrupt 0: The capture mode match for Timer B has not occurred. 1: A capture mode match has occurred for Timer B. This interrupt asserts when the values in the TBR and TBPR match the values in the TBMATCHR and TBPMR when configured in Input Edge-Time mode. This bit is cleared by writing a 1 to the ICLR.CBMCINT bit."]
    #[inline(always)]
    pub const fn set_CBMRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] GPT Timer B Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode."]
    #[must_use]
    #[inline(always)]
    pub const fn CBERIS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] GPT Timer B Capture Mode Event Raw Interrupt 0: The event has not occured. 1: The event has occured. This interrupt asserts when the subtimer is configured in Input Edge-Time mode."]
    #[inline(always)]
    pub const fn set_CBERIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] GPT Timer B Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TBMR.TBMIE is set, and the match values in TBMATCHR and optionally TBPMR have been reached when configured in one-shot or periodic mode."]
    #[must_use]
    #[inline(always)]
    pub const fn TBMRIS(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] GPT Timer B Match Raw Interrupt 0: The match value has not been reached 1: The match value is reached. TBMR.TBMIE is set, and the match values in TBMATCHR and optionally TBPMR have been reached when configured in one-shot or periodic mode."]
    #[inline(always)]
    pub const fn set_TBMRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] GPT Timer B DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed."]
    #[must_use]
    #[inline(always)]
    pub const fn DMABRIS(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] GPT Timer B DMA Done Raw Interrupt Status 0: Transfer has not completed 1: Transfer has completed."]
    #[inline(always)]
    pub const fn set_DMABRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
impl Default for RIS {
    #[inline(always)]
    fn default() -> RIS {
        RIS(0)
    }
}
impl core::fmt::Debug for RIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("TATORIS", &self.TATORIS())
            .field("CAMRIS", &self.CAMRIS())
            .field("CAERIS", &self.CAERIS())
            .field("RESERVED3", &self.RESERVED3())
            .field("TAMRIS", &self.TAMRIS())
            .field("DMAARIS", &self.DMAARIS())
            .field("RESERVED6", &self.RESERVED6())
            .field("TBTORIS", &self.TBTORIS())
            .field("CBMRIS", &self.CBMRIS())
            .field("CBERIS", &self.CBERIS())
            .field("TBMRIS", &self.TBMRIS())
            .field("RESERVED12", &self.RESERVED12())
            .field("DMABRIS", &self.DMABRIS())
            .field("RESERVED14", &self.RESERVED14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RIS {{ TATORIS: {=bool:?}, CAMRIS: {=bool:?}, CAERIS: {=bool:?}, RESERVED3: {=bool:?}, TAMRIS: {=bool:?}, DMAARIS: {=bool:?}, RESERVED6: {=u8:?}, TBTORIS: {=bool:?}, CBMRIS: {=bool:?}, CBERIS: {=bool:?}, TBMRIS: {=bool:?}, RESERVED12: {=bool:?}, DMABRIS: {=bool:?}, RESERVED14: {=u32:?} }}",
            self.TATORIS(),
            self.CAMRIS(),
            self.CAERIS(),
            self.RESERVED3(),
            self.TAMRIS(),
            self.DMAARIS(),
            self.RESERVED6(),
            self.TBTORIS(),
            self.CBMRIS(),
            self.CBERIS(),
            self.TBMRIS(),
            self.RESERVED12(),
            self.DMABRIS(),
            self.RESERVED14()
        )
    }
}
#[doc = "Synch Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYNC(pub u32);
impl SYNC {
    #[doc = "1:0\\] Synchronize GPT Timer 0."]
    #[must_use]
    #[inline(always)]
    pub const fn SYNC0(&self) -> super::vals::SYNC0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SYNC0::from_bits(val as u8)
    }
    #[doc = "1:0\\] Synchronize GPT Timer 0."]
    #[inline(always)]
    pub const fn set_SYNC0(&mut self, val: super::vals::SYNC0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "3:2\\] Synchronize GPT Timer 1."]
    #[must_use]
    #[inline(always)]
    pub const fn SYNC1(&self) -> super::vals::SYNC1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SYNC1::from_bits(val as u8)
    }
    #[doc = "3:2\\] Synchronize GPT Timer 1."]
    #[inline(always)]
    pub const fn set_SYNC1(&mut self, val: super::vals::SYNC1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "5:4\\] Synchronize GPT Timer 2."]
    #[must_use]
    #[inline(always)]
    pub const fn SYNC2(&self) -> super::vals::SYNC2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SYNC2::from_bits(val as u8)
    }
    #[doc = "5:4\\] Synchronize GPT Timer 2."]
    #[inline(always)]
    pub const fn set_SYNC2(&mut self, val: super::vals::SYNC2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "7:6\\] Synchronize GPT Timer 3."]
    #[must_use]
    #[inline(always)]
    pub const fn SYNC3(&self) -> super::vals::SYNC3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::SYNC3::from_bits(val as u8)
    }
    #[doc = "7:6\\] Synchronize GPT Timer 3."]
    #[inline(always)]
    pub const fn set_SYNC3(&mut self, val: super::vals::SYNC3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
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
impl Default for SYNC {
    #[inline(always)]
    fn default() -> SYNC {
        SYNC(0)
    }
}
impl core::fmt::Debug for SYNC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNC")
            .field("SYNC0", &self.SYNC0())
            .field("SYNC1", &self.SYNC1())
            .field("SYNC2", &self.SYNC2())
            .field("SYNC3", &self.SYNC3())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SYNC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SYNC {{ SYNC0: {:?}, SYNC1: {:?}, SYNC2: {:?}, SYNC3: {:?}, RESERVED8: {=u32:?} }}",
            self.SYNC0(),
            self.SYNC1(),
            self.SYNC2(),
            self.SYNC3(),
            self.RESERVED8()
        )
    }
}
#[doc = "Timer A Interval Load Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TAILR(pub u32);
impl TAILR {
    #[doc = "31:0\\] GPT Timer A Interval Load Register Writing this field loads the counter for Timer A. A read returns the current value of TAILR."]
    #[must_use]
    #[inline(always)]
    pub const fn TAILR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] GPT Timer A Interval Load Register Writing this field loads the counter for Timer A. A read returns the current value of TAILR."]
    #[inline(always)]
    pub const fn set_TAILR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TAILR {
    #[inline(always)]
    fn default() -> TAILR {
        TAILR(0)
    }
}
impl core::fmt::Debug for TAILR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAILR")
            .field("TAILR", &self.TAILR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TAILR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TAILR {{ TAILR: {=u32:?} }}", self.TAILR())
    }
}
#[doc = "Timer A Match Register Interrupts can be generated when the timer value is equal to the value in this register in one-shot or periodic mode. In Edge-Count mode, this register along with TAILR, determines how many edge events are counted. The total number of edge events counted is equal to the value in TAILR minus this value. Note that in edge-count mode, when executing an up-count, the value of TAPR and TAILR must be greater than the value of TAPMR and this register. In PWM mode, this value along with TAILR, determines the duty cycle of the output PWM signal. When a 16/32-bit GPT is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register. (The upper 16-bits correspond to the contents TBMATCHR). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR. Note : This register is updated internally (takes effect) based on TAMR.TAMRSU."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TAMATCHR(pub u32);
impl TAMATCHR {
    #[doc = "31:0\\] GPT Timer A Match Register."]
    #[must_use]
    #[inline(always)]
    pub const fn TAMATCHR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] GPT Timer A Match Register."]
    #[inline(always)]
    pub const fn set_TAMATCHR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TAMATCHR {
    #[inline(always)]
    fn default() -> TAMATCHR {
        TAMATCHR(0)
    }
}
impl core::fmt::Debug for TAMATCHR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAMATCHR")
            .field("TAMATCHR", &self.TAMATCHR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TAMATCHR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TAMATCHR {{ TAMATCHR: {=u32:?} }}", self.TAMATCHR())
    }
}
#[doc = "Timer A Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TAMR(pub u32);
impl TAMR {
    #[doc = "1:0\\] GPT Timer A Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register."]
    #[must_use]
    #[inline(always)]
    pub const fn TAMR(&self) -> super::vals::TAMR {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::TAMR::from_bits(val as u8)
    }
    #[doc = "1:0\\] GPT Timer A Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register."]
    #[inline(always)]
    pub const fn set_TAMR(&mut self, val: super::vals::TAMR) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "2:2\\] GPT Timer A Capture Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn TACM(&self) -> super::vals::TACM {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::TACM::from_bits(val as u8)
    }
    #[doc = "2:2\\] GPT Timer A Capture Mode."]
    #[inline(always)]
    pub const fn set_TACM(&mut self, val: super::vals::TACM) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] GPT Timer A Alternate Mode Note: To enable PWM mode, you must also clear TACM and then configure TAMR field to 0x2."]
    #[must_use]
    #[inline(always)]
    pub const fn TAAMS(&self) -> super::vals::TAAMS {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::TAAMS::from_bits(val as u8)
    }
    #[doc = "3:3\\] GPT Timer A Alternate Mode Note: To enable PWM mode, you must also clear TACM and then configure TAMR field to 0x2."]
    #[inline(always)]
    pub const fn set_TAAMS(&mut self, val: super::vals::TAAMS) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] GPT Timer A Count Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn TACDIR(&self) -> super::vals::TACDIR {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::TACDIR::from_bits(val as u8)
    }
    #[doc = "4:4\\] GPT Timer A Count Direction."]
    #[inline(always)]
    pub const fn set_TACDIR(&mut self, val: super::vals::TACDIR) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] GPT Timer A Match Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TAMIE(&self) -> super::vals::TAMIE {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::TAMIE::from_bits(val as u8)
    }
    #[doc = "5:5\\] GPT Timer A Match Interrupt Enable."]
    #[inline(always)]
    pub const fn set_TAMIE(&mut self, val: super::vals::TAMIE) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] GPT Timer A Wait-On-Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn TAWOT(&self) -> super::vals::TAWOT {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::TAWOT::from_bits(val as u8)
    }
    #[doc = "6:6\\] GPT Timer A Wait-On-Trigger."]
    #[inline(always)]
    pub const fn set_TAWOT(&mut self, val: super::vals::TAWOT) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] GPT Timer A Snap-Shot Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn TASNAPS(&self) -> super::vals::TASNAPS {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::TASNAPS::from_bits(val as u8)
    }
    #[doc = "7:7\\] GPT Timer A Snap-Shot Mode."]
    #[inline(always)]
    pub const fn set_TASNAPS(&mut self, val: super::vals::TASNAPS) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] GPT Timer A PWM Interval Load Write."]
    #[must_use]
    #[inline(always)]
    pub const fn TAILD(&self) -> super::vals::TAILD {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::TAILD::from_bits(val as u8)
    }
    #[doc = "8:8\\] GPT Timer A PWM Interval Load Write."]
    #[inline(always)]
    pub const fn set_TAILD(&mut self, val: super::vals::TAILD) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] GPTM Timer A PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TAEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TAOTE bit and the DMAEV.CAEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[must_use]
    #[inline(always)]
    pub const fn TAPWMIE(&self) -> super::vals::TAPWMIE {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::TAPWMIE::from_bits(val as u8)
    }
    #[doc = "9:9\\] GPTM Timer A PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TAEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TAOTE bit and the DMAEV.CAEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub const fn set_TAPWMIE(&mut self, val: super::vals::TAPWMIE) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Timer A Match Register Update mode This bit defines when the TAMATCHR and TAPR registers are updated. If the timer is disabled (CTL.TAEN = 0) when this bit is set, TAMATCHR and TAPR are updated when the timer is enabled. If the timer is stalled (CTL.TASTALL = 1) when this bit is set, TAMATCHR and TAPR are updated according to the configuration of this bit."]
    #[must_use]
    #[inline(always)]
    pub const fn TAMRSU(&self) -> super::vals::TAMRSU {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::TAMRSU::from_bits(val as u8)
    }
    #[doc = "10:10\\] Timer A Match Register Update mode This bit defines when the TAMATCHR and TAPR registers are updated. If the timer is disabled (CTL.TAEN = 0) when this bit is set, TAMATCHR and TAPR are updated when the timer is enabled. If the timer is stalled (CTL.TASTALL = 1) when this bit is set, TAMATCHR and TAPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub const fn set_TAMRSU(&mut self, val: super::vals::TAMRSU) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] GPTM Timer A PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TAILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TAILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[must_use]
    #[inline(always)]
    pub const fn TAPLO(&self) -> super::vals::TAPLO {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::TAPLO::from_bits(val as u8)
    }
    #[doc = "11:11\\] GPTM Timer A PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TAILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TAILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub const fn set_TAPLO(&mut self, val: super::vals::TAPLO) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] One-Shot/Periodic Interrupt Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn TACINTD(&self) -> super::vals::TACINTD {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::TACINTD::from_bits(val as u8)
    }
    #[doc = "12:12\\] One-Shot/Periodic Interrupt Disable."]
    #[inline(always)]
    pub const fn set_TACINTD(&mut self, val: super::vals::TACINTD) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "15:13\\] Timer Compare Action Select."]
    #[must_use]
    #[inline(always)]
    pub const fn TCACT(&self) -> super::vals::TAMR_TCACT {
        let val = (self.0 >> 13usize) & 0x07;
        super::vals::TAMR_TCACT::from_bits(val as u8)
    }
    #[doc = "15:13\\] Timer Compare Action Select."]
    #[inline(always)]
    pub const fn set_TCACT(&mut self, val: super::vals::TAMR_TCACT) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
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
impl Default for TAMR {
    #[inline(always)]
    fn default() -> TAMR {
        TAMR(0)
    }
}
impl core::fmt::Debug for TAMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAMR")
            .field("TAMR", &self.TAMR())
            .field("TACM", &self.TACM())
            .field("TAAMS", &self.TAAMS())
            .field("TACDIR", &self.TACDIR())
            .field("TAMIE", &self.TAMIE())
            .field("TAWOT", &self.TAWOT())
            .field("TASNAPS", &self.TASNAPS())
            .field("TAILD", &self.TAILD())
            .field("TAPWMIE", &self.TAPWMIE())
            .field("TAMRSU", &self.TAMRSU())
            .field("TAPLO", &self.TAPLO())
            .field("TACINTD", &self.TACINTD())
            .field("TCACT", &self.TCACT())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TAMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TAMR {{ TAMR: {:?}, TACM: {:?}, TAAMS: {:?}, TACDIR: {:?}, TAMIE: {:?}, TAWOT: {:?}, TASNAPS: {:?}, TAILD: {:?}, TAPWMIE: {:?}, TAMRSU: {:?}, TAPLO: {:?}, TACINTD: {:?}, TCACT: {:?}, RESERVED16: {=u16:?} }}",
            self.TAMR(),
            self.TACM(),
            self.TAAMS(),
            self.TACDIR(),
            self.TAMIE(),
            self.TAWOT(),
            self.TASNAPS(),
            self.TAILD(),
            self.TAPWMIE(),
            self.TAMRSU(),
            self.TAPLO(),
            self.TACINTD(),
            self.TCACT(),
            self.RESERVED16()
        )
    }
}
#[doc = "Timer A Pre-scale Match This register allows software to extend the range of the TAMATCHR when used individually."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TAPMR(pub u32);
impl TAPMR {
    #[doc = "7:0\\] GPT Timer A Pre-scale Match. In 16 bit mode this field holds bits 23 to 16."]
    #[must_use]
    #[inline(always)]
    pub const fn TAPSMR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] GPT Timer A Pre-scale Match. In 16 bit mode this field holds bits 23 to 16."]
    #[inline(always)]
    pub const fn set_TAPSMR(&mut self, val: u8) {
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
impl Default for TAPMR {
    #[inline(always)]
    fn default() -> TAPMR {
        TAPMR(0)
    }
}
impl core::fmt::Debug for TAPMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAPMR")
            .field("TAPSMR", &self.TAPSMR())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TAPMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TAPMR {{ TAPSMR: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.TAPSMR(),
            self.RESERVED8()
        )
    }
}
#[doc = "Timer A Pre-scale This register allows software to extend the range of the timers when they are used individually. When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter. When acting as a true prescaler, the prescaler counts down to 0 before the value in TAR and TAV registers are incremented. In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TAPR(pub u32);
impl TAPR {
    #[doc = "7:0\\] Timer A Pre-scale. Prescaler ratio in one-shot and periodic count mode is TAPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256."]
    #[must_use]
    #[inline(always)]
    pub const fn TAPSR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Timer A Pre-scale. Prescaler ratio in one-shot and periodic count mode is TAPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256."]
    #[inline(always)]
    pub const fn set_TAPSR(&mut self, val: u8) {
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
impl Default for TAPR {
    #[inline(always)]
    fn default() -> TAPR {
        TAPR(0)
    }
}
impl core::fmt::Debug for TAPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAPR")
            .field("TAPSR", &self.TAPSR())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TAPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TAPR {{ TAPSR: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.TAPSR(),
            self.RESERVED8()
        )
    }
}
#[doc = "Timer A Pre-scale Snap-shot Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAPR register either on the next cycle or on the next timeout. This register shows the current value of the Timer A pre-scaler in the 16-bit mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TAPS(pub u32);
impl TAPS {
    #[doc = "7:0\\] GPT Timer A Pre-scaler."]
    #[must_use]
    #[inline(always)]
    pub const fn PSS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] GPT Timer A Pre-scaler."]
    #[inline(always)]
    pub const fn set_PSS(&mut self, val: u8) {
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
impl Default for TAPS {
    #[inline(always)]
    fn default() -> TAPS {
        TAPS(0)
    }
}
impl core::fmt::Debug for TAPS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAPS")
            .field("PSS", &self.PSS())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TAPS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TAPS {{ PSS: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.PSS(),
            self.RESERVED8()
        )
    }
}
#[doc = "Timer A Pre-scale Value This register shows the current value of the Timer A free running pre-scaler in the 16-bit mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TAPV(pub u32);
impl TAPV {
    #[doc = "7:0\\] GPT Timer A Pre-scaler Value."]
    #[must_use]
    #[inline(always)]
    pub const fn PSV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] GPT Timer A Pre-scaler Value."]
    #[inline(always)]
    pub const fn set_PSV(&mut self, val: u8) {
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
impl Default for TAPV {
    #[inline(always)]
    fn default() -> TAPV {
        TAPV(0)
    }
}
impl core::fmt::Debug for TAPV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAPV")
            .field("PSV", &self.PSV())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TAPV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TAPV {{ PSV: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.PSV(),
            self.RESERVED8()
        )
    }
}
#[doc = "Timer A Register This register shows the current value of the Timer A counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place. When a GPT is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the Timer B (TBR) register). In the16-bit Input Edge Count, Input Edge Time, and PWM modes, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \\[23:16\\] in the TAV register. To read the value of the prescalar in periodic snapshot mode, read the Timer A Prescale Snapshot (TAPS) register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TAR(pub u32);
impl TAR {
    #[doc = "31:0\\] GPT Timer A Register Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer A Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
    #[must_use]
    #[inline(always)]
    pub const fn TAR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] GPT Timer A Register Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer A Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
    #[inline(always)]
    pub const fn set_TAR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TAR {
    #[inline(always)]
    fn default() -> TAR {
        TAR(0)
    }
}
impl core::fmt::Debug for TAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAR").field("TAR", &self.TAR()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TAR {{ TAR: {=u32:?} }}", self.TAR())
    }
}
#[doc = "Timer A Value When read, this register shows the current, free-running value of Timer A in all modes. Softwarecan use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a 16/32-bit GPTM is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TAV(pub u32);
impl TAV {
    #[doc = "31:0\\] GPT Timer A Register A read returns the current, free-running value of Timer A in all modes. When written, the value written into this register is loaded into the TAR register on the next clock cycle. Note: In 16-bit mode, only the lower 16-bits of this register can be written with a new value. Writes to the prescaler bits have no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn TAV(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] GPT Timer A Register A read returns the current, free-running value of Timer A in all modes. When written, the value written into this register is loaded into the TAR register on the next clock cycle. Note: In 16-bit mode, only the lower 16-bits of this register can be written with a new value. Writes to the prescaler bits have no effect."]
    #[inline(always)]
    pub const fn set_TAV(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TAV {
    #[inline(always)]
    fn default() -> TAV {
        TAV(0)
    }
}
impl core::fmt::Debug for TAV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAV").field("TAV", &self.TAV()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TAV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TAV {{ TAV: {=u32:?} }}", self.TAV())
    }
}
#[doc = "Timer B Interval Load Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TBILR(pub u32);
impl TBILR {
    #[doc = "31:0\\] GPT Timer B Interval Load Register Writing this field loads the counter for Timer B. A read returns the current value of TBILR."]
    #[must_use]
    #[inline(always)]
    pub const fn TBILR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] GPT Timer B Interval Load Register Writing this field loads the counter for Timer B. A read returns the current value of TBILR."]
    #[inline(always)]
    pub const fn set_TBILR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TBILR {
    #[inline(always)]
    fn default() -> TBILR {
        TBILR(0)
    }
}
impl core::fmt::Debug for TBILR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBILR")
            .field("TBILR", &self.TBILR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TBILR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TBILR {{ TBILR: {=u32:?} }}", self.TBILR())
    }
}
#[doc = "Timer B Match Register When a GPT is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of TAMATCHR. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits 15:0 are used for the match value. Bits 31:16 are reserved in both cases. Note : This register is updated internally (takes effect) based on TBMR.TBMRSU."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TBMATCHR(pub u32);
impl TBMATCHR {
    #[doc = "15:0\\] GPT Timer B Match Register."]
    #[must_use]
    #[inline(always)]
    pub const fn TBMATCHR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] GPT Timer B Match Register."]
    #[inline(always)]
    pub const fn set_TBMATCHR(&mut self, val: u16) {
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
impl Default for TBMATCHR {
    #[inline(always)]
    fn default() -> TBMATCHR {
        TBMATCHR(0)
    }
}
impl core::fmt::Debug for TBMATCHR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBMATCHR")
            .field("TBMATCHR", &self.TBMATCHR())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TBMATCHR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TBMATCHR {{ TBMATCHR: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.TBMATCHR(),
            self.RESERVED16()
        )
    }
}
#[doc = "Timer B Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TBMR(pub u32);
impl TBMR {
    #[doc = "1:0\\] GPT Timer B Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register."]
    #[must_use]
    #[inline(always)]
    pub const fn TBMR(&self) -> super::vals::TBMR {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::TBMR::from_bits(val as u8)
    }
    #[doc = "1:0\\] GPT Timer B Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register."]
    #[inline(always)]
    pub const fn set_TBMR(&mut self, val: super::vals::TBMR) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "2:2\\] GPT Timer B Capture Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn TBCM(&self) -> super::vals::TBCM {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::TBCM::from_bits(val as u8)
    }
    #[doc = "2:2\\] GPT Timer B Capture Mode."]
    #[inline(always)]
    pub const fn set_TBCM(&mut self, val: super::vals::TBCM) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] GPT Timer B Alternate Mode Note: To enable PWM mode, you must also clear TBCM bit and configure TBMR field to 0x2."]
    #[must_use]
    #[inline(always)]
    pub const fn TBAMS(&self) -> super::vals::TBAMS {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::TBAMS::from_bits(val as u8)
    }
    #[doc = "3:3\\] GPT Timer B Alternate Mode Note: To enable PWM mode, you must also clear TBCM bit and configure TBMR field to 0x2."]
    #[inline(always)]
    pub const fn set_TBAMS(&mut self, val: super::vals::TBAMS) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] GPT Timer B Count Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn TBCDIR(&self) -> super::vals::TBCDIR {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::TBCDIR::from_bits(val as u8)
    }
    #[doc = "4:4\\] GPT Timer B Count Direction."]
    #[inline(always)]
    pub const fn set_TBCDIR(&mut self, val: super::vals::TBCDIR) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] GPT Timer B Match Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TBMIE(&self) -> super::vals::TBMIE {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::TBMIE::from_bits(val as u8)
    }
    #[doc = "5:5\\] GPT Timer B Match Interrupt Enable."]
    #[inline(always)]
    pub const fn set_TBMIE(&mut self, val: super::vals::TBMIE) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] GPT Timer B Wait-On-Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn TBWOT(&self) -> super::vals::TBWOT {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::TBWOT::from_bits(val as u8)
    }
    #[doc = "6:6\\] GPT Timer B Wait-On-Trigger."]
    #[inline(always)]
    pub const fn set_TBWOT(&mut self, val: super::vals::TBWOT) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] GPT Timer B Snap-Shot Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn TBSNAPS(&self) -> super::vals::TBSNAPS {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::TBSNAPS::from_bits(val as u8)
    }
    #[doc = "7:7\\] GPT Timer B Snap-Shot Mode."]
    #[inline(always)]
    pub const fn set_TBSNAPS(&mut self, val: super::vals::TBSNAPS) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] GPT Timer B PWM Interval Load Write."]
    #[must_use]
    #[inline(always)]
    pub const fn TBILD(&self) -> super::vals::TBILD {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::TBILD::from_bits(val as u8)
    }
    #[doc = "8:8\\] GPT Timer B PWM Interval Load Write."]
    #[inline(always)]
    pub const fn set_TBILD(&mut self, val: super::vals::TBILD) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] GPTM Timer B PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TBEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TBOTE bit and the DMAEV.CBEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[must_use]
    #[inline(always)]
    pub const fn TBPWMIE(&self) -> super::vals::TBPWMIE {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::TBPWMIE::from_bits(val as u8)
    }
    #[doc = "9:9\\] GPTM Timer B PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TBEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TBOTE bit and the DMAEV.CBEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub const fn set_TBPWMIE(&mut self, val: super::vals::TBPWMIE) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Timer B Match Register Update mode This bit defines when the TBMATCHR and TBPR registers are updated If the timer is disabled (CTL.TBEN is clear) when this bit is set, TBMATCHR and TBPR are updated when the timer is enabled. If the timer is stalled (CTL.TBSTALL is set) when this bit is set, TBMATCHR and TBPR are updated according to the configuration of this bit."]
    #[must_use]
    #[inline(always)]
    pub const fn TBMRSU(&self) -> super::vals::TBMRSU {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::TBMRSU::from_bits(val as u8)
    }
    #[doc = "10:10\\] Timer B Match Register Update mode This bit defines when the TBMATCHR and TBPR registers are updated If the timer is disabled (CTL.TBEN is clear) when this bit is set, TBMATCHR and TBPR are updated when the timer is enabled. If the timer is stalled (CTL.TBSTALL is set) when this bit is set, TBMATCHR and TBPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub const fn set_TBMRSU(&mut self, val: super::vals::TBMRSU) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] GPTM Timer B PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TBILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TBILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[must_use]
    #[inline(always)]
    pub const fn TBPLO(&self) -> super::vals::TBPLO {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::TBPLO::from_bits(val as u8)
    }
    #[doc = "11:11\\] GPTM Timer B PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TBILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TBILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub const fn set_TBPLO(&mut self, val: super::vals::TBPLO) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] One-Shot/Periodic Interrupt Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn TBCINTD(&self) -> super::vals::TBCINTD {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::TBCINTD::from_bits(val as u8)
    }
    #[doc = "12:12\\] One-Shot/Periodic Interrupt Mode."]
    #[inline(always)]
    pub const fn set_TBCINTD(&mut self, val: super::vals::TBCINTD) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "15:13\\] Timer Compare Action Select."]
    #[must_use]
    #[inline(always)]
    pub const fn TCACT(&self) -> super::vals::TBMR_TCACT {
        let val = (self.0 >> 13usize) & 0x07;
        super::vals::TBMR_TCACT::from_bits(val as u8)
    }
    #[doc = "15:13\\] Timer Compare Action Select."]
    #[inline(always)]
    pub const fn set_TCACT(&mut self, val: super::vals::TBMR_TCACT) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
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
impl Default for TBMR {
    #[inline(always)]
    fn default() -> TBMR {
        TBMR(0)
    }
}
impl core::fmt::Debug for TBMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBMR")
            .field("TBMR", &self.TBMR())
            .field("TBCM", &self.TBCM())
            .field("TBAMS", &self.TBAMS())
            .field("TBCDIR", &self.TBCDIR())
            .field("TBMIE", &self.TBMIE())
            .field("TBWOT", &self.TBWOT())
            .field("TBSNAPS", &self.TBSNAPS())
            .field("TBILD", &self.TBILD())
            .field("TBPWMIE", &self.TBPWMIE())
            .field("TBMRSU", &self.TBMRSU())
            .field("TBPLO", &self.TBPLO())
            .field("TBCINTD", &self.TBCINTD())
            .field("TCACT", &self.TCACT())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TBMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TBMR {{ TBMR: {:?}, TBCM: {:?}, TBAMS: {:?}, TBCDIR: {:?}, TBMIE: {:?}, TBWOT: {:?}, TBSNAPS: {:?}, TBILD: {:?}, TBPWMIE: {:?}, TBMRSU: {:?}, TBPLO: {:?}, TBCINTD: {:?}, TCACT: {:?}, RESERVED16: {=u16:?} }}",
            self.TBMR(),
            self.TBCM(),
            self.TBAMS(),
            self.TBCDIR(),
            self.TBMIE(),
            self.TBWOT(),
            self.TBSNAPS(),
            self.TBILD(),
            self.TBPWMIE(),
            self.TBMRSU(),
            self.TBPLO(),
            self.TBCINTD(),
            self.TCACT(),
            self.RESERVED16()
        )
    }
}
#[doc = "Timer B Pre-scale Match This register allows software to extend the range of the TBMATCHR when used individually."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TBPMR(pub u32);
impl TBPMR {
    #[doc = "7:0\\] GPT Timer B Pre-scale Match Register. In 16 bit mode this field holds bits 23 to 16."]
    #[must_use]
    #[inline(always)]
    pub const fn TBPSMR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] GPT Timer B Pre-scale Match Register. In 16 bit mode this field holds bits 23 to 16."]
    #[inline(always)]
    pub const fn set_TBPSMR(&mut self, val: u8) {
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
impl Default for TBPMR {
    #[inline(always)]
    fn default() -> TBPMR {
        TBPMR(0)
    }
}
impl core::fmt::Debug for TBPMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBPMR")
            .field("TBPSMR", &self.TBPSMR())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TBPMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TBPMR {{ TBPSMR: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.TBPSMR(),
            self.RESERVED8()
        )
    }
}
#[doc = "Timer B Pre-scale This register allows software to extend the range of the timers when they are used individually. When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter. When acting as a true prescaler, the prescaler counts down to 0 before the value in TBR and TBV registers are incremented. In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TBPR(pub u32);
impl TBPR {
    #[doc = "7:0\\] Timer B Pre-scale. Prescale ratio in one-shot and periodic count mode is TBPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256."]
    #[must_use]
    #[inline(always)]
    pub const fn TBPSR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Timer B Pre-scale. Prescale ratio in one-shot and periodic count mode is TBPSR + 1, that is: 0: Prescaler ratio = 1 1: Prescaler ratio = 2 2: Prescaler ratio = 3 ... 255: Prescaler ratio = 256."]
    #[inline(always)]
    pub const fn set_TBPSR(&mut self, val: u8) {
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
impl Default for TBPR {
    #[inline(always)]
    fn default() -> TBPR {
        TBPR(0)
    }
}
impl core::fmt::Debug for TBPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBPR")
            .field("TBPSR", &self.TBPSR())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TBPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TBPR {{ TBPSR: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.TBPSR(),
            self.RESERVED8()
        )
    }
}
#[doc = "Timer B Pre-scale Snap-shot Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBPR register either on the next cycle or on the next timeout. This register shows the current value of the Timer B pre-scaler in the 16-bit mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TBPS(pub u32);
impl TBPS {
    #[doc = "7:0\\] GPT Timer B Pre-scaler."]
    #[must_use]
    #[inline(always)]
    pub const fn PSS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] GPT Timer B Pre-scaler."]
    #[inline(always)]
    pub const fn set_PSS(&mut self, val: u8) {
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
impl Default for TBPS {
    #[inline(always)]
    fn default() -> TBPS {
        TBPS(0)
    }
}
impl core::fmt::Debug for TBPS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBPS")
            .field("PSS", &self.PSS())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TBPS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TBPS {{ PSS: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.PSS(),
            self.RESERVED8()
        )
    }
}
#[doc = "Timer B Pre-scale Value This register shows the current value of the Timer B free running pre-scaler in the 16-bit mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TBPV(pub u32);
impl TBPV {
    #[doc = "7:0\\] GPT Timer B Pre-scaler Value."]
    #[must_use]
    #[inline(always)]
    pub const fn PSV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] GPT Timer B Pre-scaler Value."]
    #[inline(always)]
    pub const fn set_PSV(&mut self, val: u8) {
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
impl Default for TBPV {
    #[inline(always)]
    fn default() -> TBPV {
        TBPV(0)
    }
}
impl core::fmt::Debug for TBPV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBPV")
            .field("PSV", &self.PSV())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TBPV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TBPV {{ PSV: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.PSV(),
            self.RESERVED8()
        )
    }
}
#[doc = "Timer B Register This register shows the current value of the Timer B counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler in Input Edge Count, Input Edge Time, and PWM modes, which is the upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \\[23:16\\] in the TBV register. To read the value of the prescalar in periodic snapshot mode, read the Timer B Prescale Snapshot (TBPS) register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TBR(pub u32);
impl TBR {
    #[doc = "31:0\\] GPT Timer B Register Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer B Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
    #[must_use]
    #[inline(always)]
    pub const fn TBR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] GPT Timer B Register Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer B Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
    #[inline(always)]
    pub const fn set_TBR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TBR {
    #[inline(always)]
    fn default() -> TBR {
        TBR(0)
    }
}
impl core::fmt::Debug for TBR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBR").field("TBR", &self.TBR()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TBR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TBR {{ TBR: {=u32:?} }}", self.TBR())
    }
}
#[doc = "Timer B Value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a 16/32-bit GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TBV(pub u32);
impl TBV {
    #[doc = "31:0\\] GPT Timer B Register A read returns the current, free-running value of Timer B in all modes. When written, the value written into this register is loaded into the TBR register on the next clock cycle. Note: In 16-bit mode, only the lower 16-bits of this register can be written with a new value. Writes to the prescaler bits have no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn TBV(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] GPT Timer B Register A read returns the current, free-running value of Timer B in all modes. When written, the value written into this register is loaded into the TBR register on the next clock cycle. Note: In 16-bit mode, only the lower 16-bits of this register can be written with a new value. Writes to the prescaler bits have no effect."]
    #[inline(always)]
    pub const fn set_TBV(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TBV {
    #[inline(always)]
    fn default() -> TBV {
        TBV(0)
    }
}
impl core::fmt::Debug for TBV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBV").field("TBV", &self.TBV()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TBV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TBV {{ TBV: {=u32:?} }}", self.TBV())
    }
}
#[doc = "Peripheral Version This register provides information regarding the GPT version."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VERSION(pub u32);
impl VERSION {
    #[doc = "31:0\\] Timer Revision."]
    #[must_use]
    #[inline(always)]
    pub const fn VERSION(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Timer Revision."]
    #[inline(always)]
    pub const fn set_VERSION(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for VERSION {
    #[inline(always)]
    fn default() -> VERSION {
        VERSION(0)
    }
}
impl core::fmt::Debug for VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERSION")
            .field("VERSION", &self.VERSION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VERSION {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "VERSION {{ VERSION: {=u32:?} }}", self.VERSION())
    }
}

#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTL(pub u32);
impl CTL {
    #[doc = "1:0\\] TDC commands."]
    #[must_use]
    #[inline(always)]
    pub const fn CMD(&self) -> super::vals::CMD {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CMD::from_bits(val as u8)
    }
    #[doc = "1:0\\] TDC commands."]
    #[inline(always)]
    pub const fn set_CMD(&mut self, val: super::vals::CMD) {
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
impl Default for CTL {
    #[inline(always)]
    fn default() -> CTL {
        CTL(0)
    }
}
impl core::fmt::Debug for CTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL")
            .field("CMD", &self.CMD())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTL {{ CMD: {:?}, RESERVED2: {=u32:?} }}",
            self.CMD(),
            self.RESERVED2()
        )
    }
}
#[doc = "Prescaler Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRECNT(pub u32);
impl PRECNT {
    #[doc = "15:0\\] Prescaler counter value. Write a value to CNT to capture the value of the 16-bit prescaler counter into CNT. Read CNT to get the captured value. The read value gets 1 LSB uncertainty if the event source level rises when you release the reset. You must capture the prescaler counter value when the event source level is stable, either high or low: - Disable AUX I/O input buffer to clamp AUXIO event low. - Disable COMPA to clamp AUX_COMPA event low. The read value can in general get 1 LSB uncertainty when you gate the event source asynchronously. Please note the following: - The prescaler counter is reset to 2 by PRECTL.RESET_N. - The captured value is 2 when the number of rising edges on prescaler input is less than 3. Otherwise, captured value equals number of event pulses - 1."]
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Prescaler counter value. Write a value to CNT to capture the value of the 16-bit prescaler counter into CNT. Read CNT to get the captured value. The read value gets 1 LSB uncertainty if the event source level rises when you release the reset. You must capture the prescaler counter value when the event source level is stable, either high or low: - Disable AUX I/O input buffer to clamp AUXIO event low. - Disable COMPA to clamp AUX_COMPA event low. The read value can in general get 1 LSB uncertainty when you gate the event source asynchronously. Please note the following: - The prescaler counter is reset to 2 by PRECTL.RESET_N. - The captured value is 2 when the number of rising edges on prescaler input is less than 3. Otherwise, captured value equals number of event pulses - 1."]
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u16) {
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
impl Default for PRECNT {
    #[inline(always)]
    fn default() -> PRECNT {
        PRECNT(0)
    }
}
impl core::fmt::Debug for PRECNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRECNT")
            .field("CNT", &self.CNT())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRECNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRECNT {{ CNT: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.CNT(),
            self.RESERVED16()
        )
    }
}
#[doc = "Prescaler Control The prescaler can be used to count events that are faster than the AUX clock frequency. It can be used to: - count pulses on a specified event from the asynchronous event bus. - prescale a specified event from the asynchronous event bus. To use the prescaler output as an event source in TDC measurements you must set both TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to AUX_TDC_PRE. It is recommended to use the prescaler when the signal frequency to measure exceeds 1/10th of the AUX clock frequency."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRECTL(pub u32);
impl PRECTL {
    #[doc = "4:0\\] Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn SRC(&self) -> super::vals::SRC {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::SRC::from_bits(val as u8)
    }
    #[doc = "4:0\\] Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
    #[inline(always)]
    pub const fn set_SRC(&mut self, val: super::vals::SRC) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "5:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn RATIO(&self) -> super::vals::RATIO {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::RATIO::from_bits(val as u8)
    }
    #[doc = "6:6\\] Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler."]
    #[inline(always)]
    pub const fn set_RATIO(&mut self, val: super::vals::RATIO) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Prescaler reset. 0: Reset prescaler. 1: Release reset of prescaler. AUX_TDC_PRE event becomes 0 when you reset the prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET_N(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Prescaler reset. 0: Reset prescaler. 1: Release reset of prescaler. AUX_TDC_PRE event becomes 0 when you reset the prescaler."]
    #[inline(always)]
    pub const fn set_RESET_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
impl Default for PRECTL {
    #[inline(always)]
    fn default() -> PRECTL {
        PRECTL(0)
    }
}
impl core::fmt::Debug for PRECTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRECTL")
            .field("SRC", &self.SRC())
            .field("RESERVED5", &self.RESERVED5())
            .field("RATIO", &self.RATIO())
            .field("RESET_N", &self.RESET_N())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRECTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRECTL {{ SRC: {:?}, RESERVED5: {=bool:?}, RATIO: {:?}, RESET_N: {=bool:?}, RESERVED8: {=u32:?} }}",
            self.SRC(),
            self.RESERVED5(),
            self.RATIO(),
            self.RESET_N(),
            self.RESERVED8()
        )
    }
}
#[doc = "Result Result of last TDC conversion."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESULT(pub u32);
impl RESULT {
    #[doc = "24:0\\] TDC conversion result. The result of the TDC conversion is given in number of clock edges of the clock source selected in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. Both rising and falling edges are counted. If TDC counter saturates, VALUE is slightly higher than SATCFG.LIMIT, as it takes a non-zero time to stop the measurement. Hence, the maximum value of this field becomes slightly higher than 2^24 if you configure SATCFG.LIMIT to R24."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "24:0\\] TDC conversion result. The result of the TDC conversion is given in number of clock edges of the clock source selected in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. Both rising and falling edges are counted. If TDC counter saturates, VALUE is slightly higher than SATCFG.LIMIT, as it takes a non-zero time to stop the measurement. Hence, the maximum value of this field becomes slightly higher than 2^24 if you configure SATCFG.LIMIT to R24."]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED25(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for RESULT {
    #[inline(always)]
    fn default() -> RESULT {
        RESULT(0)
    }
}
impl core::fmt::Debug for RESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESULT")
            .field("VALUE", &self.VALUE())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESULT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RESULT {{ VALUE: {=u32:?}, RESERVED25: {=u8:?} }}",
            self.VALUE(),
            self.RESERVED25()
        )
    }
}
#[doc = "Saturation Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SATCFG(pub u32);
impl SATCFG {
    #[doc = "3:0\\] Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported."]
    #[must_use]
    #[inline(always)]
    pub const fn LIMIT(&self) -> super::vals::LIMIT {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::LIMIT::from_bits(val as u8)
    }
    #[doc = "3:0\\] Saturation limit. The flag STAT.SAT is set when the TDC counter saturates. Values not enumerated are not supported."]
    #[inline(always)]
    pub const fn set_LIMIT(&mut self, val: super::vals::LIMIT) {
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
impl Default for SATCFG {
    #[inline(always)]
    fn default() -> SATCFG {
        SATCFG(0)
    }
}
impl core::fmt::Debug for SATCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SATCFG")
            .field("LIMIT", &self.LIMIT())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SATCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SATCFG {{ LIMIT: {:?}, RESERVED4: {=u32:?} }}",
            self.LIMIT(),
            self.RESERVED4()
        )
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STAT(pub u32);
impl STAT {
    #[doc = "5:0\\] TDC state machine status."]
    #[must_use]
    #[inline(always)]
    pub const fn STATE(&self) -> super::vals::STATE {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::STATE::from_bits(val as u8)
    }
    #[doc = "5:0\\] TDC state machine status."]
    #[inline(always)]
    pub const fn set_STATE(&mut self, val: super::vals::STATE) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "6:6\\] TDC measurement complete flag. 0: TDC measurement has not yet completed. 1: TDC measurement has completed. This field clears when a new TDC measurement starts or when you write CLR_RESULT to CTL.CMD."]
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] TDC measurement complete flag. 0: TDC measurement has not yet completed. 1: TDC measurement has completed. This field clears when a new TDC measurement starts or when you write CLR_RESULT to CTL.CMD."]
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] TDC measurement saturation flag. 0: Conversion has not saturated. 1: Conversion stopped due to saturation. This field is cleared when a new measurement is started or when CLR_RESULT is written to CTL.CMD."]
    #[must_use]
    #[inline(always)]
    pub const fn SAT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] TDC measurement saturation flag. 0: Conversion has not saturated. 1: Conversion stopped due to saturation. This field is cleared when a new measurement is started or when CLR_RESULT is written to CTL.CMD."]
    #[inline(always)]
    pub const fn set_SAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
impl Default for STAT {
    #[inline(always)]
    fn default() -> STAT {
        STAT(0)
    }
}
impl core::fmt::Debug for STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("STATE", &self.STATE())
            .field("DONE", &self.DONE())
            .field("SAT", &self.SAT())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STAT {{ STATE: {:?}, DONE: {=bool:?}, SAT: {=bool:?}, RESERVED8: {=u32:?} }}",
            self.STATE(),
            self.DONE(),
            self.SAT(),
            self.RESERVED8()
        )
    }
}
#[doc = "Trigger Counter Stop-counter control and status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TRIGCNT(pub u32);
impl TRIGCNT {
    #[doc = "15:0\\] Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. Read CNT to get the remaining number of stop events to ignore during a TDC measurement. Write CNT to update the remaining number of stop events to ignore during a TDC measurement. The TDC measurement ignores updates of CNT if there are no more stop events left to ignore. When AUX_TDC:TRIGCNTCFG.EN is 1, TRIGCNTLOAD.CNT is loaded into CNT at the start of the measurement."]
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. Read CNT to get the remaining number of stop events to ignore during a TDC measurement. Write CNT to update the remaining number of stop events to ignore during a TDC measurement. The TDC measurement ignores updates of CNT if there are no more stop events left to ignore. When AUX_TDC:TRIGCNTCFG.EN is 1, TRIGCNTLOAD.CNT is loaded into CNT at the start of the measurement."]
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u16) {
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
impl Default for TRIGCNT {
    #[inline(always)]
    fn default() -> TRIGCNT {
        TRIGCNT(0)
    }
}
impl core::fmt::Debug for TRIGCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIGCNT")
            .field("CNT", &self.CNT())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TRIGCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TRIGCNT {{ CNT: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.CNT(),
            self.RESERVED16()
        )
    }
}
#[doc = "Trigger Counter Configuration Stop-counter configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TRIGCNTCFG(pub u32);
impl TRIGCNTCFG {
    #[doc = "0:0\\] Enable stop-counter. 0: Disable stop-counter. 1: Enable stop-counter. Change only while STAT.STATE is IDLE."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Enable stop-counter. 0: Disable stop-counter. 1: Enable stop-counter. Change only while STAT.STATE is IDLE."]
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
impl Default for TRIGCNTCFG {
    #[inline(always)]
    fn default() -> TRIGCNTCFG {
        TRIGCNTCFG(0)
    }
}
impl core::fmt::Debug for TRIGCNTCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIGCNTCFG")
            .field("EN", &self.EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TRIGCNTCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TRIGCNTCFG {{ EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "Trigger Counter Load Stop-counter load."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TRIGCNTLOAD(pub u32);
impl TRIGCNTLOAD {
    #[doc = "15:0\\] Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. To measure frequency of an event source: - Set start event equal to stop event. - Set CNT to number of periods to measure. Both 0 and 1 values measures a single event source period. To measure pulse width of an event source: - Set start event source equal to stop event source. - Select different polarity for start and stop event. - Set CNT to 0. To measure time from the start event to the Nth stop event when N > 1: - Select different start and stop event source. - Set CNT to (N-1). See the Technical Reference Manual for event timing requirements. When AUX_TDC:TRIGCNTCFG.EN is 1, CNT is loaded into TRIGCNT.CNT at the start of the measurement."]
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. To measure frequency of an event source: - Set start event equal to stop event. - Set CNT to number of periods to measure. Both 0 and 1 values measures a single event source period. To measure pulse width of an event source: - Set start event source equal to stop event source. - Select different polarity for start and stop event. - Set CNT to 0. To measure time from the start event to the Nth stop event when N > 1: - Select different start and stop event source. - Set CNT to (N-1). See the Technical Reference Manual for event timing requirements. When AUX_TDC:TRIGCNTCFG.EN is 1, CNT is loaded into TRIGCNT.CNT at the start of the measurement."]
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u16) {
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
impl Default for TRIGCNTLOAD {
    #[inline(always)]
    fn default() -> TRIGCNTLOAD {
        TRIGCNTLOAD(0)
    }
}
impl core::fmt::Debug for TRIGCNTLOAD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIGCNTLOAD")
            .field("CNT", &self.CNT())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TRIGCNTLOAD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TRIGCNTLOAD {{ CNT: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.CNT(),
            self.RESERVED16()
        )
    }
}
#[doc = "Trigger Source Select source and polarity for TDC start and stop events. See the Technical Reference Manual for event timing requirements."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TRIGSRC(pub u32);
impl TRIGSRC {
    #[doc = "4:0\\] Select start source from the asynchronous AUX event bus. Change only while STAT.STATE is IDLE."]
    #[must_use]
    #[inline(always)]
    pub const fn START_SRC(&self) -> super::vals::START_SRC {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::START_SRC::from_bits(val as u8)
    }
    #[doc = "4:0\\] Select start source from the asynchronous AUX event bus. Change only while STAT.STATE is IDLE."]
    #[inline(always)]
    pub const fn set_START_SRC(&mut self, val: super::vals::START_SRC) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "5:5\\] Polarity of start source. Change only while STAT.STATE is IDLE."]
    #[must_use]
    #[inline(always)]
    pub const fn START_POL(&self) -> super::vals::START_POL {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::START_POL::from_bits(val as u8)
    }
    #[doc = "5:5\\] Polarity of start source. Change only while STAT.STATE is IDLE."]
    #[inline(always)]
    pub const fn set_START_POL(&mut self, val: super::vals::START_POL) {
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
    #[doc = "12:8\\] Select stop source from the asynchronous AUX event bus. Change only while STAT.STATE is IDLE."]
    #[must_use]
    #[inline(always)]
    pub const fn STOP_SRC(&self) -> super::vals::STOP_SRC {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::STOP_SRC::from_bits(val as u8)
    }
    #[doc = "12:8\\] Select stop source from the asynchronous AUX event bus. Change only while STAT.STATE is IDLE."]
    #[inline(always)]
    pub const fn set_STOP_SRC(&mut self, val: super::vals::STOP_SRC) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "13:13\\] Polarity of stop source. Change only while STAT.STATE is IDLE."]
    #[must_use]
    #[inline(always)]
    pub const fn STOP_POL(&self) -> super::vals::STOP_POL {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::STOP_POL::from_bits(val as u8)
    }
    #[doc = "13:13\\] Polarity of stop source. Change only while STAT.STATE is IDLE."]
    #[inline(always)]
    pub const fn set_STOP_POL(&mut self, val: super::vals::STOP_POL) {
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
impl Default for TRIGSRC {
    #[inline(always)]
    fn default() -> TRIGSRC {
        TRIGSRC(0)
    }
}
impl core::fmt::Debug for TRIGSRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIGSRC")
            .field("START_SRC", &self.START_SRC())
            .field("START_POL", &self.START_POL())
            .field("RESERVED6", &self.RESERVED6())
            .field("STOP_SRC", &self.STOP_SRC())
            .field("STOP_POL", &self.STOP_POL())
            .field("RESERVED14", &self.RESERVED14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TRIGSRC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TRIGSRC {{ START_SRC: {:?}, START_POL: {:?}, RESERVED6: {=u8:?}, STOP_SRC: {:?}, STOP_POL: {:?}, RESERVED14: {=u32:?} }}",
            self.START_SRC(),
            self.START_POL(),
            self.RESERVED6(),
            self.STOP_SRC(),
            self.STOP_POL(),
            self.RESERVED14()
        )
    }
}

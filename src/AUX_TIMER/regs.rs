#[doc = "Timer 0 Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T0CFG(pub u32);
impl T0CFG {
    #[doc = "0:0\\] Timer 0 reload mode."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOAD(&self) -> super::vals::T0CFG_RELOAD {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::T0CFG_RELOAD::from_bits(val as u8)
    }
    #[doc = "0:0\\] Timer 0 reload mode."]
    #[inline(always)]
    pub const fn set_RELOAD(&mut self, val: super::vals::T0CFG_RELOAD) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Timer 0 mode. Configure source for Timer 0 prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::T0CFG_MODE {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::T0CFG_MODE::from_bits(val as u8)
    }
    #[doc = "1:1\\] Timer 0 mode. Configure source for Timer 0 prescaler."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::T0CFG_MODE) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "7:4\\] Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[must_use]
    #[inline(always)]
    pub const fn PRE(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "7:4\\] Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[inline(always)]
    pub const fn set_PRE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "12:8\\] Select Timer 0 tick source from the synchronous event bus."]
    #[must_use]
    #[inline(always)]
    pub const fn TICK_SRC(&self) -> super::vals::T0CFG_TICK_SRC {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::T0CFG_TICK_SRC::from_bits(val as u8)
    }
    #[doc = "12:8\\] Select Timer 0 tick source from the synchronous event bus."]
    #[inline(always)]
    pub const fn set_TICK_SRC(&mut self, val: super::vals::T0CFG_TICK_SRC) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "13:13\\] Tick source polarity for Timer 0."]
    #[must_use]
    #[inline(always)]
    pub const fn TICK_SRC_POL(&self) -> super::vals::T0CFG_TICK_SRC_POL {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::T0CFG_TICK_SRC_POL::from_bits(val as u8)
    }
    #[doc = "13:13\\] Tick source polarity for Timer 0."]
    #[inline(always)]
    pub const fn set_TICK_SRC_POL(&mut self, val: super::vals::T0CFG_TICK_SRC_POL) {
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
impl Default for T0CFG {
    #[inline(always)]
    fn default() -> T0CFG {
        T0CFG(0)
    }
}
impl core::fmt::Debug for T0CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0CFG")
            .field("RELOAD", &self.RELOAD())
            .field("MODE", &self.MODE())
            .field("RESERVED2", &self.RESERVED2())
            .field("PRE", &self.PRE())
            .field("TICK_SRC", &self.TICK_SRC())
            .field("TICK_SRC_POL", &self.TICK_SRC_POL())
            .field("RESERVED14", &self.RESERVED14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for T0CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "T0CFG {{ RELOAD: {:?}, MODE: {:?}, RESERVED2: {=u8:?}, PRE: {=u8:?}, TICK_SRC: {:?}, TICK_SRC_POL: {:?}, RESERVED14: {=u32:?} }}",
            self.RELOAD(),
            self.MODE(),
            self.RESERVED2(),
            self.PRE(),
            self.TICK_SRC(),
            self.TICK_SRC_POL(),
            self.RESERVED14()
        )
    }
}
#[doc = "Timer 0 Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T0CTL(pub u32);
impl T0CTL {
    #[doc = "0:0\\] Timer 0 enable. 0: Disable Timer 0. 1: Enable Timer 0. The counter restarts from 0 when you enable Timer 0."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Timer 0 enable. 0: Disable Timer 0. 1: Enable Timer 0. The counter restarts from 0 when you enable Timer 0."]
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
impl Default for T0CTL {
    #[inline(always)]
    fn default() -> T0CTL {
        T0CTL(0)
    }
}
impl core::fmt::Debug for T0CTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0CTL")
            .field("EN", &self.EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for T0CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "T0CTL {{ EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "Timer 0 Target."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T0TARGET(pub u32);
impl T0TARGET {
    #[doc = "15:0\\] Timer 0 target value. Manual Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER0_EV pulses high for 1 AUX clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 0 counts to 1. AUX_TIMER0_EV pulses high for 1 AUX clock period. Continuous Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER0_EV pulses high for 1 AUX clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 0 counter value remains 0. AUX_TIMER0_EV goes high and remains high 1 AUX clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Timer 0 target value. Manual Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER0_EV pulses high for 1 AUX clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 0 counts to 1. AUX_TIMER0_EV pulses high for 1 AUX clock period. Continuous Reload Mode: - Timer 0 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER0_EV pulses high for 1 AUX clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 0 counter value remains 0. AUX_TIMER0_EV goes high and remains high 1 AUX clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u16) {
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
impl Default for T0TARGET {
    #[inline(always)]
    fn default() -> T0TARGET {
        T0TARGET(0)
    }
}
impl core::fmt::Debug for T0TARGET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0TARGET")
            .field("VALUE", &self.VALUE())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for T0TARGET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "T0TARGET {{ VALUE: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.VALUE(),
            self.RESERVED16()
        )
    }
}
#[doc = "Timer 1 Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T1CFG(pub u32);
impl T1CFG {
    #[doc = "0:0\\] Timer 1 reload mode."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOAD(&self) -> super::vals::T1CFG_RELOAD {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::T1CFG_RELOAD::from_bits(val as u8)
    }
    #[doc = "0:0\\] Timer 1 reload mode."]
    #[inline(always)]
    pub const fn set_RELOAD(&mut self, val: super::vals::T1CFG_RELOAD) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Timer 1 mode. Configure source for Timer 1 prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::T1CFG_MODE {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::T1CFG_MODE::from_bits(val as u8)
    }
    #[doc = "1:1\\] Timer 1 mode. Configure source for Timer 1 prescaler."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::T1CFG_MODE) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "7:4\\] Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[must_use]
    #[inline(always)]
    pub const fn PRE(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "7:4\\] Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[inline(always)]
    pub const fn set_PRE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "12:8\\] Select Timer 1 tick source from the synchronous event bus."]
    #[must_use]
    #[inline(always)]
    pub const fn TICK_SRC(&self) -> super::vals::T1CFG_TICK_SRC {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::T1CFG_TICK_SRC::from_bits(val as u8)
    }
    #[doc = "12:8\\] Select Timer 1 tick source from the synchronous event bus."]
    #[inline(always)]
    pub const fn set_TICK_SRC(&mut self, val: super::vals::T1CFG_TICK_SRC) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "13:13\\] Tick source polarity for Timer 1."]
    #[must_use]
    #[inline(always)]
    pub const fn TICK_SRC_POL(&self) -> super::vals::T1CFG_TICK_SRC_POL {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::T1CFG_TICK_SRC_POL::from_bits(val as u8)
    }
    #[doc = "13:13\\] Tick source polarity for Timer 1."]
    #[inline(always)]
    pub const fn set_TICK_SRC_POL(&mut self, val: super::vals::T1CFG_TICK_SRC_POL) {
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
impl Default for T1CFG {
    #[inline(always)]
    fn default() -> T1CFG {
        T1CFG(0)
    }
}
impl core::fmt::Debug for T1CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T1CFG")
            .field("RELOAD", &self.RELOAD())
            .field("MODE", &self.MODE())
            .field("RESERVED2", &self.RESERVED2())
            .field("PRE", &self.PRE())
            .field("TICK_SRC", &self.TICK_SRC())
            .field("TICK_SRC_POL", &self.TICK_SRC_POL())
            .field("RESERVED14", &self.RESERVED14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for T1CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "T1CFG {{ RELOAD: {:?}, MODE: {:?}, RESERVED2: {=u8:?}, PRE: {=u8:?}, TICK_SRC: {:?}, TICK_SRC_POL: {:?}, RESERVED14: {=u32:?} }}",
            self.RELOAD(),
            self.MODE(),
            self.RESERVED2(),
            self.PRE(),
            self.TICK_SRC(),
            self.TICK_SRC_POL(),
            self.RESERVED14()
        )
    }
}
#[doc = "Timer 1 Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T1CTL(pub u32);
impl T1CTL {
    #[doc = "0:0\\] Timer 1 enable. 0: Disable Timer 1. 1: Enable Timer 1. The counter restarts from 0 when you enable Timer 1."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Timer 1 enable. 0: Disable Timer 1. 1: Enable Timer 1. The counter restarts from 0 when you enable Timer 1."]
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
impl Default for T1CTL {
    #[inline(always)]
    fn default() -> T1CTL {
        T1CTL(0)
    }
}
impl core::fmt::Debug for T1CTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T1CTL")
            .field("EN", &self.EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for T1CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "T1CTL {{ EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "Timer 1 Target Timer 1 counter target value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T1TARGET(pub u32);
impl T1TARGET {
    #[doc = "7:0\\] Timer 1 target value. Manual Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER1_EV pulses high for 1 AUX clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 1 counts to 1. AUX_TIMER1_EV pulses high for 1 AUX clock period. Continuous Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER1_EV pulses high for 1 AUX clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 1 counter value remains 0. AUX_TIMER1_EV goes high and remains high 1 AUX clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Timer 1 target value. Manual Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than VALUE. - AUX_TIMER1_EV pulses high for 1 AUX clock period when the counter value is equal to or greater than VALUE. Note: When VALUE is 0, Timer 1 counts to 1. AUX_TIMER1_EV pulses high for 1 AUX clock period. Continuous Reload Mode: - Timer 1 increments until the counter value becomes equal to or greater than ( VALUE - 1), then restarts from 0. - AUX_TIMER1_EV pulses high for 1 AUX clock period when the counter value is 0, except for when you enable the timer. Note: When VALUE is less than 2, Timer 1 counter value remains 0. AUX_TIMER1_EV goes high and remains high 1 AUX clock period after you enable the timer. It is allowed to update the VALUE while the timer runs."]
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
impl Default for T1TARGET {
    #[inline(always)]
    fn default() -> T1TARGET {
        T1TARGET(0)
    }
}
impl core::fmt::Debug for T1TARGET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T1TARGET")
            .field("VALUE", &self.VALUE())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for T1TARGET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "T1TARGET {{ VALUE: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.VALUE(),
            self.RESERVED8()
        )
    }
}

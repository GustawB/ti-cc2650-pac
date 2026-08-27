#[doc = "This component control the Real Time Clock residing in AON Note: This module is only supporting 32 bit ReadWrite access."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AON_RTC {
    ptr: *mut u8,
}
unsafe impl Send for AON_RTC {}
unsafe impl Sync for AON_RTC {}
impl AON_RTC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control This register contains various bitfields for configuration of RTC."]
    #[inline(always)]
    pub const fn CTL(self) -> crate::common::Reg<regs::CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Event Flags, RTC Status This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield."]
    #[inline(always)]
    pub const fn EVFLAGS(self) -> crate::common::Reg<regs::EVFLAGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Second Counter Value, Integer Part."]
    #[inline(always)]
    pub const fn SEC(self) -> crate::common::Reg<regs::SEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Second Counter Value, Fractional Part."]
    #[inline(always)]
    pub const fn SUBSEC(self) -> crate::common::Reg<regs::SUBSEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Subseconds Increment Value added to SUBSEC.VALUE on every SCLK_LFclock cycle."]
    #[inline(always)]
    pub const fn SUBSECINC(self) -> crate::common::Reg<regs::SUBSECINC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Channel Configuration."]
    #[inline(always)]
    pub const fn CHCTL(self) -> crate::common::Reg<regs::CHCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Channel 0 Compare Value."]
    #[inline(always)]
    pub const fn CH0CMP(self) -> crate::common::Reg<regs::CH0CMP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Channel 1 Compare Value."]
    #[inline(always)]
    pub const fn CH1CMP(self) -> crate::common::Reg<regs::CH1CMP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Channel 2 Compare Value."]
    #[inline(always)]
    pub const fn CH2CMP(self) -> crate::common::Reg<regs::CH2CMP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\] event."]
    #[inline(always)]
    pub const fn CH2CMPINC(self) -> crate::common::Reg<regs::CH2CMPINC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Channel 1 Capture Value If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL."]
    #[inline(always)]
    pub const fn CH1CAPT(self) -> crate::common::Reg<regs::CH1CAPT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "AON Synchronization This register is used for synchronizing between MCU and entire AON domain."]
    #[inline(always)]
    pub const fn SYNC(self) -> crate::common::Reg<regs::SYNC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
}
pub mod regs;
pub mod vals;

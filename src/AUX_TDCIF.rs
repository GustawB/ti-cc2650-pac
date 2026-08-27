#[doc = "AUX Time To Digital Converter."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_TDCIF {
    ptr: *mut u8,
}
unsafe impl Send for AUX_TDCIF {}
unsafe impl Sync for AUX_TDCIF {}
impl AUX_TDCIF {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn CTL(self) -> crate::common::Reg<regs::CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn STAT(self) -> crate::common::Reg<regs::STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Result Result of last TDC conversion."]
    #[inline(always)]
    pub const fn RESULT(self) -> crate::common::Reg<regs::RESULT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Saturation Configuration."]
    #[inline(always)]
    pub const fn SATCFG(self) -> crate::common::Reg<regs::SATCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Trigger Source Select source and polarity for TDC start and stop events. See the Technical Reference Manual for event timing requirements."]
    #[inline(always)]
    pub const fn TRIGSRC(self) -> crate::common::Reg<regs::TRIGSRC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Trigger Counter Stop-counter control and status."]
    #[inline(always)]
    pub const fn TRIGCNT(self) -> crate::common::Reg<regs::TRIGCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Trigger Counter Load Stop-counter load."]
    #[inline(always)]
    pub const fn TRIGCNTLOAD(self) -> crate::common::Reg<regs::TRIGCNTLOAD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Trigger Counter Configuration Stop-counter configuration."]
    #[inline(always)]
    pub const fn TRIGCNTCFG(self) -> crate::common::Reg<regs::TRIGCNTCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Prescaler Control The prescaler can be used to count events that are faster than the AUX clock frequency. It can be used to: - count pulses on a specified event from the asynchronous event bus. - prescale a specified event from the asynchronous event bus. To use the prescaler output as an event source in TDC measurements you must set both TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to AUX_TDC_PRE. It is recommended to use the prescaler when the signal frequency to measure exceeds 1/10th of the AUX clock frequency."]
    #[inline(always)]
    pub const fn PRECTL(self) -> crate::common::Reg<regs::PRECTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Prescaler Counter."]
    #[inline(always)]
    pub const fn PRECNT(self) -> crate::common::Reg<regs::PRECNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
}
pub mod regs;
pub mod vals;

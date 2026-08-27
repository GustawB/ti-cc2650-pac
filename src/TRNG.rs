#[doc = "True Random Number Generator."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TRNG {
    ptr: *mut u8,
}
unsafe impl Send for TRNG {}
unsafe impl Sync for TRNG {}
impl TRNG {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Random Number Lower Word Readout Value."]
    #[inline(always)]
    pub const fn OUT0(self) -> crate::common::Reg<regs::OUT0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Random Number Upper Word Readout Value."]
    #[inline(always)]
    pub const fn OUT1(self) -> crate::common::Reg<regs::OUT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt Status."]
    #[inline(always)]
    pub const fn IRQFLAGSTAT(self) -> crate::common::Reg<regs::IRQFLAGSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Interrupt Mask."]
    #[inline(always)]
    pub const fn IRQFLAGMASK(self) -> crate::common::Reg<regs::IRQFLAGMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Flag Clear."]
    #[inline(always)]
    pub const fn IRQFLAGCLR(self) -> crate::common::Reg<regs::IRQFLAGCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn CTL(self) -> crate::common::Reg<regs::CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Configuration 0."]
    #[inline(always)]
    pub const fn CFG0(self) -> crate::common::Reg<regs::CFG0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Alarm Control."]
    #[inline(always)]
    pub const fn ALARMCNT(self) -> crate::common::Reg<regs::ALARMCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "FRO Enable."]
    #[inline(always)]
    pub const fn FROEN(self) -> crate::common::Reg<regs::FROEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "FRO De-tune Bit."]
    #[inline(always)]
    pub const fn FRODETUNE(self) -> crate::common::Reg<regs::FRODETUNE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Alarm Event."]
    #[inline(always)]
    pub const fn ALARMMASK(self) -> crate::common::Reg<regs::ALARMMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Alarm Shutdown."]
    #[inline(always)]
    pub const fn ALARMSTOP(self) -> crate::common::Reg<regs::ALARMSTOP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "LFSR Readout Value."]
    #[inline(always)]
    pub const fn LFSR0(self) -> crate::common::Reg<regs::LFSR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "LFSR Readout Value."]
    #[inline(always)]
    pub const fn LFSR1(self) -> crate::common::Reg<regs::LFSR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "LFSR Readout Value."]
    #[inline(always)]
    pub const fn LFSR2(self) -> crate::common::Reg<regs::LFSR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "TRNG Engine Options Information."]
    #[inline(always)]
    pub const fn HWOPT(self) -> crate::common::Reg<regs::HWOPT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "HW Version 0 EIP Number And Core Revision."]
    #[inline(always)]
    pub const fn HWVER0(self) -> crate::common::Reg<regs::HWVER0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Interrupt Status After Masking."]
    #[inline(always)]
    pub const fn IRQSTATMASK(self) -> crate::common::Reg<regs::IRQSTATMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1fd8usize) as _) }
    }
    #[doc = "HW Version 1 TRNG Revision Number."]
    #[inline(always)]
    pub const fn HWVER1(self) -> crate::common::Reg<regs::HWVER1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1fe0usize) as _) }
    }
    #[doc = "Interrupt Set."]
    #[inline(always)]
    pub const fn IRQSET(self) -> crate::common::Reg<regs::IRQSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1fecusize) as _) }
    }
    #[doc = "SW Reset Control."]
    #[inline(always)]
    pub const fn SWRESET(self) -> crate::common::Reg<regs::SWRESET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ff0usize) as _) }
    }
    #[doc = "Interrupt Status."]
    #[inline(always)]
    pub const fn IRQSTAT(self) -> crate::common::Reg<regs::IRQSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ff8usize) as _) }
    }
}
pub mod regs;

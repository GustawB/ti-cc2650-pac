#[doc = "Watchdog Timer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WDT {
    ptr: *mut u8,
}
unsafe impl Send for WDT {}
unsafe impl Sync for WDT {}
impl WDT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration."]
    #[inline(always)]
    pub const fn LOAD(self) -> crate::common::Reg<regs::LOAD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Current Count Value."]
    #[inline(always)]
    pub const fn VALUE(self) -> crate::common::Reg<regs::VALUE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn CTL(self) -> crate::common::Reg<regs::CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Interrupt Clear."]
    #[inline(always)]
    pub const fn ICR(self) -> crate::common::Reg<regs::ICR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Raw Interrupt Status."]
    #[inline(always)]
    pub const fn RIS(self) -> crate::common::Reg<regs::RIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Masked Interrupt Status."]
    #[inline(always)]
    pub const fn MIS(self) -> crate::common::Reg<regs::MIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Test Mode."]
    #[inline(always)]
    pub const fn TEST(self) -> crate::common::Reg<regs::TEST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "Interrupt Cause Test Mode."]
    #[inline(always)]
    pub const fn INT_CAUS(self) -> crate::common::Reg<regs::INT_CAUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn LOCK(self) -> crate::common::Reg<regs::LOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
}
pub mod regs;
pub mod vals;

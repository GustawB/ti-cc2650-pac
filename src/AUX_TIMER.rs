#[doc = "AUX Timer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_TIMER {
    ptr: *mut u8,
}
unsafe impl Send for AUX_TIMER {}
unsafe impl Sync for AUX_TIMER {}
impl AUX_TIMER {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Timer 0 Configuration."]
    #[inline(always)]
    pub const fn T0CFG(self) -> crate::common::Reg<regs::T0CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Timer 1 Configuration."]
    #[inline(always)]
    pub const fn T1CFG(self) -> crate::common::Reg<regs::T1CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Timer 0 Control."]
    #[inline(always)]
    pub const fn T0CTL(self) -> crate::common::Reg<regs::T0CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Timer 0 Target."]
    #[inline(always)]
    pub const fn T0TARGET(self) -> crate::common::Reg<regs::T0TARGET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Timer 1 Target Timer 1 counter target value."]
    #[inline(always)]
    pub const fn T1TARGET(self) -> crate::common::Reg<regs::T1TARGET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Timer 1 Control."]
    #[inline(always)]
    pub const fn T1CTL(self) -> crate::common::Reg<regs::T1CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs;
pub mod vals;

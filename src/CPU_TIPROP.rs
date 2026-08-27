#[doc = "Cortex-M's TI proprietary registers."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPU_TIPROP {
    ptr: *mut u8,
}
unsafe impl Send for CPU_TIPROP {}
unsafe impl Sync for CPU_TIPROP {}
impl CPU_TIPROP {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn RESERVED000(self) -> crate::common::Reg<regs::RESERVED000, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn TRACECLKMUX(self) -> crate::common::Reg<regs::TRACECLKMUX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn DYN_CG(self) -> crate::common::Reg<regs::DYN_CG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;

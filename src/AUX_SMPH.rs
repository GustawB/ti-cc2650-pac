#[doc = "AUX Semaphore Controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_SMPH {
    ptr: *mut u8,
}
unsafe impl Send for AUX_SMPH {}
unsafe impl Sync for AUX_SMPH {}
impl AUX_SMPH {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Semaphore 0."]
    #[inline(always)]
    pub const fn SMPH0(self) -> crate::common::Reg<regs::SMPH0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Semaphore 1."]
    #[inline(always)]
    pub const fn SMPH1(self) -> crate::common::Reg<regs::SMPH1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Semaphore 2."]
    #[inline(always)]
    pub const fn SMPH2(self) -> crate::common::Reg<regs::SMPH2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Semaphore 3."]
    #[inline(always)]
    pub const fn SMPH3(self) -> crate::common::Reg<regs::SMPH3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Semaphore 4."]
    #[inline(always)]
    pub const fn SMPH4(self) -> crate::common::Reg<regs::SMPH4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Semaphore 5."]
    #[inline(always)]
    pub const fn SMPH5(self) -> crate::common::Reg<regs::SMPH5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Semaphore 6."]
    #[inline(always)]
    pub const fn SMPH6(self) -> crate::common::Reg<regs::SMPH6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Semaphore 7."]
    #[inline(always)]
    pub const fn SMPH7(self) -> crate::common::Reg<regs::SMPH7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Auto Take Sticky Request for Single Semaphore."]
    #[inline(always)]
    pub const fn AUTOTAKE(self) -> crate::common::Reg<regs::AUTOTAKE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
pub mod regs;

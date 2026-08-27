#[doc = "RF Core Radio Timer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFC_RAT {
    ptr: *mut u8,
}
unsafe impl Send for RFC_RAT {}
unsafe impl Sync for RFC_RAT {}
impl RFC_RAT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Radio Timer Counter Value."]
    #[inline(always)]
    pub const fn RATCNT(self) -> crate::common::Reg<regs::RATCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Timer Channel 0 Capture/Compare Register."]
    #[inline(always)]
    pub const fn RATCH0VAL(self) -> crate::common::Reg<regs::RATCH0VAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Timer Channel 1 Capture/Compare Register."]
    #[inline(always)]
    pub const fn RATCH1VAL(self) -> crate::common::Reg<regs::RATCH1VAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Timer Channel 2 Capture/Compare Register."]
    #[inline(always)]
    pub const fn RATCH2VAL(self) -> crate::common::Reg<regs::RATCH2VAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Timer Channel 3 Capture/Compare Register."]
    #[inline(always)]
    pub const fn RATCH3VAL(self) -> crate::common::Reg<regs::RATCH3VAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Timer Channel 4 Capture/Compare Register."]
    #[inline(always)]
    pub const fn RATCH4VAL(self) -> crate::common::Reg<regs::RATCH4VAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Timer Channel 5 Capture/Compare Register."]
    #[inline(always)]
    pub const fn RATCH5VAL(self) -> crate::common::Reg<regs::RATCH5VAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Timer Channel 6 Capture/Compare Register."]
    #[inline(always)]
    pub const fn RATCH6VAL(self) -> crate::common::Reg<regs::RATCH6VAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Timer Channel 7 Capture/Compare Register."]
    #[inline(always)]
    pub const fn RATCH7VAL(self) -> crate::common::Reg<regs::RATCH7VAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
}
pub mod regs;

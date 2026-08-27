#[doc = "RF Core Power Management."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFC_PWR {
    ptr: *mut u8,
}
unsafe impl Send for RFC_PWR {}
unsafe impl Sync for RFC_PWR {}
impl RFC_PWR {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RF Core Power Management and Clock Enable."]
    #[inline(always)]
    pub const fn PWMCLKEN(self) -> crate::common::Reg<regs::PWMCLKEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs;

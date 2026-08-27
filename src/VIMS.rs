#[doc = "Versatile Instruction Memory System Controls memory access to the Flash and encapsulates the following instruction memories: - Boot ROM - Cache / GPRAM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VIMS {
    ptr: *mut u8,
}
unsafe impl Send for VIMS {}
unsafe impl Sync for VIMS {}
impl VIMS {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Status Displays current VIMS mode and line buffer status."]
    #[inline(always)]
    pub const fn STAT(self) -> crate::common::Reg<regs::STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control Configure VIMS mode and line buffer settings."]
    #[inline(always)]
    pub const fn CTL(self) -> crate::common::Reg<regs::CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs;
pub mod vals;

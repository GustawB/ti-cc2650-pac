#[doc = "Always On (AON) IO Controller - controls IO operation when the MCU IO Controller (IOC) is powered off and resides in the AON domain. Note: This module only supports 32 bit Read/Write access from MCU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AON_IOC {
    ptr: *mut u8,
}
unsafe impl Send for AON_IOC {}
unsafe impl Sync for AON_IOC {}
impl AON_IOC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn IOSTRMIN(self) -> crate::common::Reg<regs::IOSTRMIN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn IOSTRMED(self) -> crate::common::Reg<regs::IOSTRMED, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn IOSTRMAX(self) -> crate::common::Reg<regs::IOSTRMAX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "IO Latch Control Controls transparency of all latches holding I/O or configuration state from the MCU IOC."]
    #[inline(always)]
    pub const fn IOCLATCH(self) -> crate::common::Reg<regs::IOCLATCH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SCLK_LF External Output Control."]
    #[inline(always)]
    pub const fn CLK32KCTL(self) -> crate::common::Reg<regs::CLK32KCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
pub mod regs;
pub mod vals;

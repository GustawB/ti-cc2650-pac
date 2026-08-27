#[doc = "This component controls AON_SYSCTL, which is the device's system controller. Note: This module is only supporting 32 bit ReadWrite access from MCU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AON_SYSCTL {
    ptr: *mut u8,
}
unsafe impl Send for AON_SYSCTL {}
unsafe impl Sync for AON_SYSCTL {}
impl AON_SYSCTL {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Power Management This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
    #[inline(always)]
    pub const fn PWRCTL(self) -> crate::common::Reg<regs::PWRCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Reset Management This register contains bitfields releated to system reset such as reset source and reset request and control of brown out resets."]
    #[inline(always)]
    pub const fn RESETCTL(self) -> crate::common::Reg<regs::RESETCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Sleep Mode This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN."]
    #[inline(always)]
    pub const fn SLEEPCTL(self) -> crate::common::Reg<regs::SLEEPCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs;
pub mod vals;

#[doc = "RF Core Doorbell."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFC_DBELL {
    ptr: *mut u8,
}
unsafe impl Send for RFC_DBELL {}
unsafe impl Sync for RFC_DBELL {}
impl RFC_DBELL {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Doorbell Command Register."]
    #[inline(always)]
    pub const fn CMDR(self) -> crate::common::Reg<regs::CMDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Doorbell Command Status Register."]
    #[inline(always)]
    pub const fn CMDSTA(self) -> crate::common::Reg<regs::CMDSTA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt Flags From RF Hardware Modules."]
    #[inline(always)]
    pub const fn RFHWIFG(self) -> crate::common::Reg<regs::RFHWIFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Interrupt Enable For RF Hardware Modules."]
    #[inline(always)]
    pub const fn RFHWIEN(self) -> crate::common::Reg<regs::RFHWIEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Flags For Command and Packet Engine Generated Interrupts."]
    #[inline(always)]
    pub const fn RFCPEIFG(self) -> crate::common::Reg<regs::RFCPEIFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt Enable For Command and Packet Engine Generated Interrupts."]
    #[inline(always)]
    pub const fn RFCPEIEN(self) -> crate::common::Reg<regs::RFCPEIEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt Vector Selection For Command and Packet Engine Generated Interrupts."]
    #[inline(always)]
    pub const fn RFCPEISL(self) -> crate::common::Reg<regs::RFCPEISL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Doorbell Command Acknowledgement Interrupt Flag."]
    #[inline(always)]
    pub const fn RFACKIFG(self) -> crate::common::Reg<regs::RFACKIFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "RF Core General Purpose Output Control."]
    #[inline(always)]
    pub const fn SYSGPOCTL(self) -> crate::common::Reg<regs::SYSGPOCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
pub mod regs;
pub mod vals;

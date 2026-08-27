#[doc = "ARM Micro Direct Memory Access Controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UDMA0 {
    ptr: *mut u8,
}
unsafe impl Send for UDMA0 {}
unsafe impl Sync for UDMA0 {}
impl UDMA0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Configuration."]
    #[inline(always)]
    pub const fn CFG(self) -> crate::common::Reg<regs::CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Channel Control Data Base Pointer."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Channel Alternate Control Data Base Pointer."]
    #[inline(always)]
    pub const fn ALTCTRL(self) -> crate::common::Reg<regs::ALTCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Channel Wait On Request Status."]
    #[inline(always)]
    pub const fn WAITONREQ(self) -> crate::common::Reg<regs::WAITONREQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Channel Software Request."]
    #[inline(always)]
    pub const fn SOFTREQ(self) -> crate::common::Reg<regs::SOFTREQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Channel Set UseBurst."]
    #[inline(always)]
    pub const fn SETBURST(self) -> crate::common::Reg<regs::SETBURST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Channel Clear UseBurst."]
    #[inline(always)]
    pub const fn CLEARBURST(self) -> crate::common::Reg<regs::CLEARBURST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Channel Set Request Mask."]
    #[inline(always)]
    pub const fn SETREQMASK(self) -> crate::common::Reg<regs::SETREQMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Clear Channel Request Mask."]
    #[inline(always)]
    pub const fn CLEARREQMASK(self) -> crate::common::Reg<regs::CLEARREQMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Set Channel Enable."]
    #[inline(always)]
    pub const fn SETCHANNELEN(self) -> crate::common::Reg<regs::SETCHANNELEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Clear Channel Enable."]
    #[inline(always)]
    pub const fn CLEARCHANNELEN(
        self,
    ) -> crate::common::Reg<regs::CLEARCHANNELEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Channel Set Primary-Alternate."]
    #[inline(always)]
    pub const fn SETCHNLPRIALT(self) -> crate::common::Reg<regs::SETCHNLPRIALT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Channel Clear Primary-Alternate."]
    #[inline(always)]
    pub const fn CLEARCHNLPRIALT(
        self,
    ) -> crate::common::Reg<regs::CLEARCHNLPRIALT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Set Channel Priority."]
    #[inline(always)]
    pub const fn SETCHNLPRIORITY(
        self,
    ) -> crate::common::Reg<regs::SETCHNLPRIORITY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Clear Channel Priority."]
    #[inline(always)]
    pub const fn CLEARCHNLPRIORITY(
        self,
    ) -> crate::common::Reg<regs::CLEARCHNLPRIORITY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Error Status and Clear."]
    #[inline(always)]
    pub const fn ERROR(self) -> crate::common::Reg<regs::ERROR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Channel Request Done."]
    #[inline(always)]
    pub const fn REQDONE(self) -> crate::common::Reg<regs::REQDONE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "Channel Request Done Mask."]
    #[inline(always)]
    pub const fn DONEMASK(self) -> crate::common::Reg<regs::DONEMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
}
pub mod regs;

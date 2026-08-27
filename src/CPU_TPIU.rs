#[doc = "Cortex-M3's Trace Port Interface Unit (TPIU)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPU_TPIU {
    ptr: *mut u8,
}
unsafe impl Send for CPU_TPIU {}
unsafe impl Sync for CPU_TPIU {}
impl CPU_TPIU {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Supported Sync Port Sizes This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture."]
    #[inline(always)]
    pub const fn SSPSR(self) -> crate::common::Reg<regs::SSPSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Current Sync Port Size This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit."]
    #[inline(always)]
    pub const fn CSPSR(self) -> crate::common::Reg<regs::CSPSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Async Clock Prescaler This register scales the baud rate of the asynchronous output."]
    #[inline(always)]
    pub const fn ACPR(self) -> crate::common::Reg<regs::ACPR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Selected Pin Protocol This register selects the protocol to be used for trace output. Note: If this register is changed while trace data is being output, data corruption occurs."]
    #[inline(always)]
    pub const fn SPPR(self) -> crate::common::Reg<regs::SPPR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Formatter and Flush Status."]
    #[inline(always)]
    pub const fn FFSR(self) -> crate::common::Reg<regs::FFSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Formatter and Flush Control When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption. Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value."]
    #[inline(always)]
    pub const fn FFCR(self) -> crate::common::Reg<regs::FFCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Formatter Synchronization Counter."]
    #[inline(always)]
    pub const fn FSCR(self) -> crate::common::Reg<regs::FSCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "Claim Tag Mask."]
    #[inline(always)]
    pub const fn CLAIMMASK(self) -> crate::common::Reg<regs::CLAIMMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa0usize) as _) }
    }
    #[doc = "Claim Tag Set."]
    #[inline(always)]
    pub const fn CLAIMSET(self) -> crate::common::Reg<regs::CLAIMSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa0usize) as _) }
    }
    #[doc = "Claim Tag Clear."]
    #[inline(always)]
    pub const fn CLAIMCLR(self) -> crate::common::Reg<regs::CLAIMCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa4usize) as _) }
    }
    #[doc = "Current Claim Tag."]
    #[inline(always)]
    pub const fn CLAIMTAG(self) -> crate::common::Reg<regs::CLAIMTAG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa4usize) as _) }
    }
    #[doc = "Device ID."]
    #[inline(always)]
    pub const fn DEVID(self) -> crate::common::Reg<regs::DEVID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc8usize) as _) }
    }
}
pub mod regs;
pub mod vals;

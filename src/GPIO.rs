#[doc = "MCU GPIO - I/F for controlling and reading IO status and IO event status."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO {
    ptr: *mut u8,
}
unsafe impl Send for GPIO {}
unsafe impl Sync for GPIO {}
impl GPIO {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Data Out 0 to 3 Alias register for byte access to each bit in DOUT31_0."]
    #[inline(always)]
    pub const fn DOUT3_0(self) -> crate::common::Reg<regs::DOUT3_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Data Out 4 to 7 Alias register for byte access to each bit in DOUT31_0."]
    #[inline(always)]
    pub const fn DOUT7_4(self) -> crate::common::Reg<regs::DOUT7_4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Data Out 8 to 11 Alias register for byte access to each bit in DOUT31_0."]
    #[inline(always)]
    pub const fn DOUT11_8(self) -> crate::common::Reg<regs::DOUT11_8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Data Out 12 to 15 Alias register for byte access to each bit in DOUT31_0."]
    #[inline(always)]
    pub const fn DOUT15_12(self) -> crate::common::Reg<regs::DOUT15_12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Data Out 16 to 19 Alias register for byte access to each bit in DOUT31_0."]
    #[inline(always)]
    pub const fn DOUT19_16(self) -> crate::common::Reg<regs::DOUT19_16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Data Out 20 to 23 Alias register for byte access to each bit in DOUT31_0."]
    #[inline(always)]
    pub const fn DOUT23_20(self) -> crate::common::Reg<regs::DOUT23_20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Data Out 24 to 27 Alias register for byte access to each bit in DOUT31_0."]
    #[inline(always)]
    pub const fn DOUT27_24(self) -> crate::common::Reg<regs::DOUT27_24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Data Out 28 to 31 Alias register for byte access to each bit in DOUT31_0."]
    #[inline(always)]
    pub const fn DOUT31_28(self) -> crate::common::Reg<regs::DOUT31_28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Data Output for DIO 0 to 31."]
    #[inline(always)]
    pub const fn DOUT31_0(self) -> crate::common::Reg<regs::DOUT31_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT31_0 register."]
    #[inline(always)]
    pub const fn DOUTSET31_0(self) -> crate::common::Reg<regs::DOUTSET31_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT31_0 register."]
    #[inline(always)]
    pub const fn DOUTCLR31_0(self) -> crate::common::Reg<regs::DOUTCLR31_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
    #[inline(always)]
    pub const fn DOUTTGL31_0(self) -> crate::common::Reg<regs::DOUTTGL31_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Data Input from DIO 0 to 31."]
    #[inline(always)]
    pub const fn DIN31_0(self) -> crate::common::Reg<regs::DIN31_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Data Output Enable for DIO 0 to 31."]
    #[inline(always)]
    pub const fn DOE31_0(self) -> crate::common::Reg<regs::DOE31_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Event Register for DIO 0 to 31 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
    #[inline(always)]
    pub const fn EVFLAGS31_0(self) -> crate::common::Reg<regs::EVFLAGS31_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
}
pub mod regs;

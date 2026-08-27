#[doc = "I2S Audio DMA module supporting formats I2S, LJF, RJF and DSP."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2S0 {
    ptr: *mut u8,
}
unsafe impl Send for I2S0 {}
unsafe impl Sync for I2S0 {}
impl I2S0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "WCLK Source Selection."]
    #[inline(always)]
    pub const fn AIFWCLKSRC(self) -> crate::common::Reg<regs::AIFWCLKSRC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DMA Buffer Size Configuration."]
    #[inline(always)]
    pub const fn AIFDMACFG(self) -> crate::common::Reg<regs::AIFDMACFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Pin Direction."]
    #[inline(always)]
    pub const fn AIFDIRCFG(self) -> crate::common::Reg<regs::AIFDIRCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Serial Interface Format Configuration."]
    #[inline(always)]
    pub const fn AIFFMTCFG(self) -> crate::common::Reg<regs::AIFFMTCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Word Selection Bit Mask for Pin 0."]
    #[inline(always)]
    pub const fn AIFWMASK0(self) -> crate::common::Reg<regs::AIFWMASK0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Word Selection Bit Mask for Pin 1."]
    #[inline(always)]
    pub const fn AIFWMASK1(self) -> crate::common::Reg<regs::AIFWMASK1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn AIFWMASK2(self) -> crate::common::Reg<regs::AIFWMASK2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Audio Interface PWM Debug Value."]
    #[inline(always)]
    pub const fn AIFPWMVALUE(self) -> crate::common::Reg<regs::AIFPWMVALUE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "DMA Input Buffer Next Pointer."]
    #[inline(always)]
    pub const fn AIFINPTRNEXT(self) -> crate::common::Reg<regs::AIFINPTRNEXT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "DMA Input Buffer Current Pointer."]
    #[inline(always)]
    pub const fn AIFINPTR(self) -> crate::common::Reg<regs::AIFINPTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "DMA Output Buffer Next Pointer."]
    #[inline(always)]
    pub const fn AIFOUTPTRNEXT(self) -> crate::common::Reg<regs::AIFOUTPTRNEXT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "DMA Output Buffer Current Pointer."]
    #[inline(always)]
    pub const fn AIFOUTPTR(self) -> crate::common::Reg<regs::AIFOUTPTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Samplestamp Generator Control Register."]
    #[inline(always)]
    pub const fn STMPCTL(self) -> crate::common::Reg<regs::STMPCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Captured XOSC Counter Value, Capture Channel 0."]
    #[inline(always)]
    pub const fn STMPXCNTCAPT0(self) -> crate::common::Reg<regs::STMPXCNTCAPT0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "XOSC Period Value."]
    #[inline(always)]
    pub const fn STMPXPER(self) -> crate::common::Reg<regs::STMPXPER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Captured WCLK Counter Value, Capture Channel 0."]
    #[inline(always)]
    pub const fn STMPWCNTCAPT0(self) -> crate::common::Reg<regs::STMPWCNTCAPT0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "WCLK Counter Period Value."]
    #[inline(always)]
    pub const fn STMPWPER(self) -> crate::common::Reg<regs::STMPWPER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "WCLK Counter Trigger Value for Input Pins."]
    #[inline(always)]
    pub const fn STMPINTRIG(self) -> crate::common::Reg<regs::STMPINTRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "WCLK Counter Trigger Value for Output Pins."]
    #[inline(always)]
    pub const fn STMPOUTTRIG(self) -> crate::common::Reg<regs::STMPOUTTRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "WCLK Counter Set Operation."]
    #[inline(always)]
    pub const fn STMPWSET(self) -> crate::common::Reg<regs::STMPWSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "WCLK Counter Add Operation."]
    #[inline(always)]
    pub const fn STMPWADD(self) -> crate::common::Reg<regs::STMPWADD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "XOSC Minimum Period Value Minimum Value of STMPXPER."]
    #[inline(always)]
    pub const fn STMPXPERMIN(self) -> crate::common::Reg<regs::STMPXPERMIN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Current Value of WCNT."]
    #[inline(always)]
    pub const fn STMPWCNT(self) -> crate::common::Reg<regs::STMPWCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Current Value of XCNT."]
    #[inline(always)]
    pub const fn STMPXCNT(self) -> crate::common::Reg<regs::STMPXCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn STMPXCNTCAPT1(self) -> crate::common::Reg<regs::STMPXCNTCAPT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn STMPWCNTCAPT1(self) -> crate::common::Reg<regs::STMPWCNTCAPT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Interrupt Mask Register Selects mask states of the flags in IRQFLAGS that contribute to the I2S_IRQ event."]
    #[inline(always)]
    pub const fn IRQMASK(self) -> crate::common::Reg<regs::IRQMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Raw Interrupt Status Register."]
    #[inline(always)]
    pub const fn IRQFLAGS(self) -> crate::common::Reg<regs::IRQFLAGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Interrupt Set Register."]
    #[inline(always)]
    pub const fn IRQSET(self) -> crate::common::Reg<regs::IRQSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Interrupt Clear Register."]
    #[inline(always)]
    pub const fn IRQCLR(self) -> crate::common::Reg<regs::IRQCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
}
pub mod regs;
pub mod vals;

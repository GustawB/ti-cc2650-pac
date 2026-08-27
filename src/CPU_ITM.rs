#[doc = "Cortex-M's Instrumentation Trace Macrocell (ITM)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPU_ITM {
    ptr: *mut u8,
}
unsafe impl Send for CPU_ITM {}
unsafe impl Sync for CPU_ITM {}
impl CPU_ITM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Stimulus Port 0."]
    #[inline(always)]
    pub const fn STIM0(self) -> crate::common::Reg<regs::STIM0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Stimulus Port 1."]
    #[inline(always)]
    pub const fn STIM1(self) -> crate::common::Reg<regs::STIM1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Stimulus Port 2."]
    #[inline(always)]
    pub const fn STIM2(self) -> crate::common::Reg<regs::STIM2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Stimulus Port 3."]
    #[inline(always)]
    pub const fn STIM3(self) -> crate::common::Reg<regs::STIM3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Stimulus Port 4."]
    #[inline(always)]
    pub const fn STIM4(self) -> crate::common::Reg<regs::STIM4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Stimulus Port 5."]
    #[inline(always)]
    pub const fn STIM5(self) -> crate::common::Reg<regs::STIM5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Stimulus Port 6."]
    #[inline(always)]
    pub const fn STIM6(self) -> crate::common::Reg<regs::STIM6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Stimulus Port 7."]
    #[inline(always)]
    pub const fn STIM7(self) -> crate::common::Reg<regs::STIM7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Stimulus Port 8."]
    #[inline(always)]
    pub const fn STIM8(self) -> crate::common::Reg<regs::STIM8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Stimulus Port 9."]
    #[inline(always)]
    pub const fn STIM9(self) -> crate::common::Reg<regs::STIM9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Stimulus Port 10."]
    #[inline(always)]
    pub const fn STIM10(self) -> crate::common::Reg<regs::STIM10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Stimulus Port 11."]
    #[inline(always)]
    pub const fn STIM11(self) -> crate::common::Reg<regs::STIM11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Stimulus Port 12."]
    #[inline(always)]
    pub const fn STIM12(self) -> crate::common::Reg<regs::STIM12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Stimulus Port 13."]
    #[inline(always)]
    pub const fn STIM13(self) -> crate::common::Reg<regs::STIM13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Stimulus Port 14."]
    #[inline(always)]
    pub const fn STIM14(self) -> crate::common::Reg<regs::STIM14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Stimulus Port 15."]
    #[inline(always)]
    pub const fn STIM15(self) -> crate::common::Reg<regs::STIM15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Stimulus Port 16."]
    #[inline(always)]
    pub const fn STIM16(self) -> crate::common::Reg<regs::STIM16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Stimulus Port 17."]
    #[inline(always)]
    pub const fn STIM17(self) -> crate::common::Reg<regs::STIM17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Stimulus Port 18."]
    #[inline(always)]
    pub const fn STIM18(self) -> crate::common::Reg<regs::STIM18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Stimulus Port 19."]
    #[inline(always)]
    pub const fn STIM19(self) -> crate::common::Reg<regs::STIM19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Stimulus Port 20."]
    #[inline(always)]
    pub const fn STIM20(self) -> crate::common::Reg<regs::STIM20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Stimulus Port 21."]
    #[inline(always)]
    pub const fn STIM21(self) -> crate::common::Reg<regs::STIM21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Stimulus Port 22."]
    #[inline(always)]
    pub const fn STIM22(self) -> crate::common::Reg<regs::STIM22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Stimulus Port 23."]
    #[inline(always)]
    pub const fn STIM23(self) -> crate::common::Reg<regs::STIM23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Stimulus Port 24."]
    #[inline(always)]
    pub const fn STIM24(self) -> crate::common::Reg<regs::STIM24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Stimulus Port 25."]
    #[inline(always)]
    pub const fn STIM25(self) -> crate::common::Reg<regs::STIM25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Stimulus Port 26."]
    #[inline(always)]
    pub const fn STIM26(self) -> crate::common::Reg<regs::STIM26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Stimulus Port 27."]
    #[inline(always)]
    pub const fn STIM27(self) -> crate::common::Reg<regs::STIM27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Stimulus Port 28."]
    #[inline(always)]
    pub const fn STIM28(self) -> crate::common::Reg<regs::STIM28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Stimulus Port 29."]
    #[inline(always)]
    pub const fn STIM29(self) -> crate::common::Reg<regs::STIM29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Stimulus Port 30."]
    #[inline(always)]
    pub const fn STIM30(self) -> crate::common::Reg<regs::STIM30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Stimulus Port 31."]
    #[inline(always)]
    pub const fn STIM31(self) -> crate::common::Reg<regs::STIM31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Trace Enable Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required."]
    #[inline(always)]
    pub const fn TER(self) -> crate::common::Reg<regs::TER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize) as _) }
    }
    #[doc = "Trace Privilege This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode."]
    #[inline(always)]
    pub const fn TPR(self) -> crate::common::Reg<regs::TPR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e40usize) as _) }
    }
    #[doc = "Trace Control Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set."]
    #[inline(always)]
    pub const fn TCR(self) -> crate::common::Reg<regs::TCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e80usize) as _) }
    }
    #[doc = "Lock Access This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR."]
    #[inline(always)]
    pub const fn LAR(self) -> crate::common::Reg<regs::LAR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb0usize) as _) }
    }
    #[doc = "Lock Status Use this register to enable write accesses to the Control Register."]
    #[inline(always)]
    pub const fn LSR(self) -> crate::common::Reg<regs::LSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb4usize) as _) }
    }
}
pub mod regs;
pub mod vals;

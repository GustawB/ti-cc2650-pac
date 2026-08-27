#[doc = "MCU Semaphore Module This module provides 32 binary semaphores. The state of a binary semaphore is either taken or available. A semaphore does not implement any ownership attribute. Still, a semaphore can be used to handle mutual exclusion scenarios."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMPH {
    ptr: *mut u8,
}
unsafe impl Send for SMPH {}
unsafe impl Sync for SMPH {}
impl SMPH {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MCU SEMAPHORE 0."]
    #[inline(always)]
    pub const fn SMPH0(self) -> crate::common::Reg<regs::SMPH0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 1."]
    #[inline(always)]
    pub const fn SMPH1(self) -> crate::common::Reg<regs::SMPH1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 2."]
    #[inline(always)]
    pub const fn SMPH2(self) -> crate::common::Reg<regs::SMPH2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 3."]
    #[inline(always)]
    pub const fn SMPH3(self) -> crate::common::Reg<regs::SMPH3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "MCU SEMAPHORE 4."]
    #[inline(always)]
    pub const fn SMPH4(self) -> crate::common::Reg<regs::SMPH4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 5."]
    #[inline(always)]
    pub const fn SMPH5(self) -> crate::common::Reg<regs::SMPH5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 6."]
    #[inline(always)]
    pub const fn SMPH6(self) -> crate::common::Reg<regs::SMPH6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 7."]
    #[inline(always)]
    pub const fn SMPH7(self) -> crate::common::Reg<regs::SMPH7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "MCU SEMAPHORE 8."]
    #[inline(always)]
    pub const fn SMPH8(self) -> crate::common::Reg<regs::SMPH8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 9."]
    #[inline(always)]
    pub const fn SMPH9(self) -> crate::common::Reg<regs::SMPH9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 10."]
    #[inline(always)]
    pub const fn SMPH10(self) -> crate::common::Reg<regs::SMPH10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 11."]
    #[inline(always)]
    pub const fn SMPH11(self) -> crate::common::Reg<regs::SMPH11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "MCU SEMAPHORE 12."]
    #[inline(always)]
    pub const fn SMPH12(self) -> crate::common::Reg<regs::SMPH12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 13."]
    #[inline(always)]
    pub const fn SMPH13(self) -> crate::common::Reg<regs::SMPH13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 14."]
    #[inline(always)]
    pub const fn SMPH14(self) -> crate::common::Reg<regs::SMPH14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 15."]
    #[inline(always)]
    pub const fn SMPH15(self) -> crate::common::Reg<regs::SMPH15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "MCU SEMAPHORE 16."]
    #[inline(always)]
    pub const fn SMPH16(self) -> crate::common::Reg<regs::SMPH16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 17."]
    #[inline(always)]
    pub const fn SMPH17(self) -> crate::common::Reg<regs::SMPH17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 18."]
    #[inline(always)]
    pub const fn SMPH18(self) -> crate::common::Reg<regs::SMPH18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 19."]
    #[inline(always)]
    pub const fn SMPH19(self) -> crate::common::Reg<regs::SMPH19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "MCU SEMAPHORE 20."]
    #[inline(always)]
    pub const fn SMPH20(self) -> crate::common::Reg<regs::SMPH20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 21."]
    #[inline(always)]
    pub const fn SMPH21(self) -> crate::common::Reg<regs::SMPH21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 22."]
    #[inline(always)]
    pub const fn SMPH22(self) -> crate::common::Reg<regs::SMPH22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 23."]
    #[inline(always)]
    pub const fn SMPH23(self) -> crate::common::Reg<regs::SMPH23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "MCU SEMAPHORE 24."]
    #[inline(always)]
    pub const fn SMPH24(self) -> crate::common::Reg<regs::SMPH24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 25."]
    #[inline(always)]
    pub const fn SMPH25(self) -> crate::common::Reg<regs::SMPH25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 26."]
    #[inline(always)]
    pub const fn SMPH26(self) -> crate::common::Reg<regs::SMPH26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 27."]
    #[inline(always)]
    pub const fn SMPH27(self) -> crate::common::Reg<regs::SMPH27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "MCU SEMAPHORE 28."]
    #[inline(always)]
    pub const fn SMPH28(self) -> crate::common::Reg<regs::SMPH28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 29."]
    #[inline(always)]
    pub const fn SMPH29(self) -> crate::common::Reg<regs::SMPH29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 30."]
    #[inline(always)]
    pub const fn SMPH30(self) -> crate::common::Reg<regs::SMPH30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 31."]
    #[inline(always)]
    pub const fn SMPH31(self) -> crate::common::Reg<regs::SMPH31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "MCU SEMAPHORE 0 ALIAS."]
    #[inline(always)]
    pub const fn PEEK0(self) -> crate::common::Reg<regs::PEEK0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 1 ALIAS."]
    #[inline(always)]
    pub const fn PEEK1(self) -> crate::common::Reg<regs::PEEK1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 2 ALIAS."]
    #[inline(always)]
    pub const fn PEEK2(self) -> crate::common::Reg<regs::PEEK2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0808usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 3 ALIAS."]
    #[inline(always)]
    pub const fn PEEK3(self) -> crate::common::Reg<regs::PEEK3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
    #[doc = "MCU SEMAPHORE 4 ALIAS."]
    #[inline(always)]
    pub const fn PEEK4(self) -> crate::common::Reg<regs::PEEK4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0810usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 5 ALIAS."]
    #[inline(always)]
    pub const fn PEEK5(self) -> crate::common::Reg<regs::PEEK5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0814usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 6 ALIAS."]
    #[inline(always)]
    pub const fn PEEK6(self) -> crate::common::Reg<regs::PEEK6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0818usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 7 ALIAS."]
    #[inline(always)]
    pub const fn PEEK7(self) -> crate::common::Reg<regs::PEEK7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x081cusize) as _) }
    }
    #[doc = "MCU SEMAPHORE 8 ALIAS."]
    #[inline(always)]
    pub const fn PEEK8(self) -> crate::common::Reg<regs::PEEK8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0820usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 9 ALIAS."]
    #[inline(always)]
    pub const fn PEEK9(self) -> crate::common::Reg<regs::PEEK9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0824usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 10 ALIAS."]
    #[inline(always)]
    pub const fn PEEK10(self) -> crate::common::Reg<regs::PEEK10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0828usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 11 ALIAS."]
    #[inline(always)]
    pub const fn PEEK11(self) -> crate::common::Reg<regs::PEEK11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x082cusize) as _) }
    }
    #[doc = "MCU SEMAPHORE 12 ALIAS."]
    #[inline(always)]
    pub const fn PEEK12(self) -> crate::common::Reg<regs::PEEK12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0830usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 13 ALIAS."]
    #[inline(always)]
    pub const fn PEEK13(self) -> crate::common::Reg<regs::PEEK13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0834usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 14 ALIAS."]
    #[inline(always)]
    pub const fn PEEK14(self) -> crate::common::Reg<regs::PEEK14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0838usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 15 ALIAS."]
    #[inline(always)]
    pub const fn PEEK15(self) -> crate::common::Reg<regs::PEEK15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x083cusize) as _) }
    }
    #[doc = "MCU SEMAPHORE 16 ALIAS."]
    #[inline(always)]
    pub const fn PEEK16(self) -> crate::common::Reg<regs::PEEK16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0840usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 17 ALIAS."]
    #[inline(always)]
    pub const fn PEEK17(self) -> crate::common::Reg<regs::PEEK17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0844usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 18 ALIAS."]
    #[inline(always)]
    pub const fn PEEK18(self) -> crate::common::Reg<regs::PEEK18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0848usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 19 ALIAS."]
    #[inline(always)]
    pub const fn PEEK19(self) -> crate::common::Reg<regs::PEEK19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x084cusize) as _) }
    }
    #[doc = "MCU SEMAPHORE 20 ALIAS."]
    #[inline(always)]
    pub const fn PEEK20(self) -> crate::common::Reg<regs::PEEK20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0850usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 21 ALIAS."]
    #[inline(always)]
    pub const fn PEEK21(self) -> crate::common::Reg<regs::PEEK21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0854usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 22 ALIAS."]
    #[inline(always)]
    pub const fn PEEK22(self) -> crate::common::Reg<regs::PEEK22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0858usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 23 ALIAS."]
    #[inline(always)]
    pub const fn PEEK23(self) -> crate::common::Reg<regs::PEEK23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x085cusize) as _) }
    }
    #[doc = "MCU SEMAPHORE 24 ALIAS."]
    #[inline(always)]
    pub const fn PEEK24(self) -> crate::common::Reg<regs::PEEK24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0860usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 25 ALIAS."]
    #[inline(always)]
    pub const fn PEEK25(self) -> crate::common::Reg<regs::PEEK25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0864usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 26 ALIAS."]
    #[inline(always)]
    pub const fn PEEK26(self) -> crate::common::Reg<regs::PEEK26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0868usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 27 ALIAS."]
    #[inline(always)]
    pub const fn PEEK27(self) -> crate::common::Reg<regs::PEEK27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x086cusize) as _) }
    }
    #[doc = "MCU SEMAPHORE 28 ALIAS."]
    #[inline(always)]
    pub const fn PEEK28(self) -> crate::common::Reg<regs::PEEK28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0870usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 29 ALIAS."]
    #[inline(always)]
    pub const fn PEEK29(self) -> crate::common::Reg<regs::PEEK29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0874usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 30 ALIAS."]
    #[inline(always)]
    pub const fn PEEK30(self) -> crate::common::Reg<regs::PEEK30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0878usize) as _) }
    }
    #[doc = "MCU SEMAPHORE 31 ALIAS."]
    #[inline(always)]
    pub const fn PEEK31(self) -> crate::common::Reg<regs::PEEK31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x087cusize) as _) }
    }
}
pub mod regs;

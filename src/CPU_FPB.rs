#[doc = "Cortex-M's Flash Patch and Breakpoint (FPB)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPU_FPB {
    ptr: *mut u8,
}
unsafe impl Send for CPU_FPB {}
unsafe impl Sync for CPU_FPB {}
impl CPU_FPB {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control This register is used to enable the flash patch block."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Remap This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators."]
    #[inline(always)]
    pub const fn REMAP(self) -> crate::common::Reg<regs::REMAP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Comparator 0."]
    #[inline(always)]
    pub const fn COMP0(self) -> crate::common::Reg<regs::COMP0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Comparator 1."]
    #[inline(always)]
    pub const fn COMP1(self) -> crate::common::Reg<regs::COMP1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Comparator 2."]
    #[inline(always)]
    pub const fn COMP2(self) -> crate::common::Reg<regs::COMP2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Comparator 3."]
    #[inline(always)]
    pub const fn COMP3(self) -> crate::common::Reg<regs::COMP3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Comparator 4."]
    #[inline(always)]
    pub const fn COMP4(self) -> crate::common::Reg<regs::COMP4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Comparator 5."]
    #[inline(always)]
    pub const fn COMP5(self) -> crate::common::Reg<regs::COMP5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Comparator 6."]
    #[inline(always)]
    pub const fn COMP6(self) -> crate::common::Reg<regs::COMP6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Comparator 7."]
    #[inline(always)]
    pub const fn COMP7(self) -> crate::common::Reg<regs::COMP7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
}
pub mod regs;

#[doc = "IO Controller (IOC) - configures all the DIOs and resides in the MCU domain."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOC {
    ptr: *mut u8,
}
unsafe impl Send for IOC {}
unsafe impl Sync for IOC {}
impl IOC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration of DIO0."]
    #[inline(always)]
    pub const fn IOCFG0(self) -> crate::common::Reg<regs::IOCFG0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Configuration of DIO1."]
    #[inline(always)]
    pub const fn IOCFG1(self) -> crate::common::Reg<regs::IOCFG1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Configuration of DIO2."]
    #[inline(always)]
    pub const fn IOCFG2(self) -> crate::common::Reg<regs::IOCFG2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Configuration of DIO3."]
    #[inline(always)]
    pub const fn IOCFG3(self) -> crate::common::Reg<regs::IOCFG3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Configuration of DIO4."]
    #[inline(always)]
    pub const fn IOCFG4(self) -> crate::common::Reg<regs::IOCFG4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Configuration of DIO5."]
    #[inline(always)]
    pub const fn IOCFG5(self) -> crate::common::Reg<regs::IOCFG5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Configuration of DIO6."]
    #[inline(always)]
    pub const fn IOCFG6(self) -> crate::common::Reg<regs::IOCFG6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Configuration of DIO7."]
    #[inline(always)]
    pub const fn IOCFG7(self) -> crate::common::Reg<regs::IOCFG7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Configuration of DIO8."]
    #[inline(always)]
    pub const fn IOCFG8(self) -> crate::common::Reg<regs::IOCFG8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Configuration of DIO9."]
    #[inline(always)]
    pub const fn IOCFG9(self) -> crate::common::Reg<regs::IOCFG9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Configuration of DIO10."]
    #[inline(always)]
    pub const fn IOCFG10(self) -> crate::common::Reg<regs::IOCFG10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Configuration of DIO11."]
    #[inline(always)]
    pub const fn IOCFG11(self) -> crate::common::Reg<regs::IOCFG11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Configuration of DIO12."]
    #[inline(always)]
    pub const fn IOCFG12(self) -> crate::common::Reg<regs::IOCFG12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Configuration of DIO13."]
    #[inline(always)]
    pub const fn IOCFG13(self) -> crate::common::Reg<regs::IOCFG13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Configuration of DIO14."]
    #[inline(always)]
    pub const fn IOCFG14(self) -> crate::common::Reg<regs::IOCFG14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Configuration of DIO15."]
    #[inline(always)]
    pub const fn IOCFG15(self) -> crate::common::Reg<regs::IOCFG15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Configuration of DIO16."]
    #[inline(always)]
    pub const fn IOCFG16(self) -> crate::common::Reg<regs::IOCFG16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Configuration of DIO17."]
    #[inline(always)]
    pub const fn IOCFG17(self) -> crate::common::Reg<regs::IOCFG17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Configuration of DIO18."]
    #[inline(always)]
    pub const fn IOCFG18(self) -> crate::common::Reg<regs::IOCFG18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Configuration of DIO19."]
    #[inline(always)]
    pub const fn IOCFG19(self) -> crate::common::Reg<regs::IOCFG19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Configuration of DIO20."]
    #[inline(always)]
    pub const fn IOCFG20(self) -> crate::common::Reg<regs::IOCFG20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Configuration of DIO21."]
    #[inline(always)]
    pub const fn IOCFG21(self) -> crate::common::Reg<regs::IOCFG21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Configuration of DIO22."]
    #[inline(always)]
    pub const fn IOCFG22(self) -> crate::common::Reg<regs::IOCFG22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Configuration of DIO23."]
    #[inline(always)]
    pub const fn IOCFG23(self) -> crate::common::Reg<regs::IOCFG23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Configuration of DIO24."]
    #[inline(always)]
    pub const fn IOCFG24(self) -> crate::common::Reg<regs::IOCFG24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Configuration of DIO25."]
    #[inline(always)]
    pub const fn IOCFG25(self) -> crate::common::Reg<regs::IOCFG25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Configuration of DIO26."]
    #[inline(always)]
    pub const fn IOCFG26(self) -> crate::common::Reg<regs::IOCFG26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Configuration of DIO27."]
    #[inline(always)]
    pub const fn IOCFG27(self) -> crate::common::Reg<regs::IOCFG27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Configuration of DIO28."]
    #[inline(always)]
    pub const fn IOCFG28(self) -> crate::common::Reg<regs::IOCFG28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Configuration of DIO29."]
    #[inline(always)]
    pub const fn IOCFG29(self) -> crate::common::Reg<regs::IOCFG29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Configuration of DIO30."]
    #[inline(always)]
    pub const fn IOCFG30(self) -> crate::common::Reg<regs::IOCFG30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Configuration of DIO31."]
    #[inline(always)]
    pub const fn IOCFG31(self) -> crate::common::Reg<regs::IOCFG31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
}
pub mod regs;
pub mod vals;

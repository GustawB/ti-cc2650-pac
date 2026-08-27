#[doc = "Power, Reset and Clock Management."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRCM {
    ptr: *mut u8,
}
unsafe impl Send for PRCM {}
unsafe impl Sync for PRCM {}
impl PRCM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Infrastructure Clock Division Factor For Run Mode."]
    #[inline(always)]
    pub const fn INFRCLKDIVR(self) -> crate::common::Reg<regs::INFRCLKDIVR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Infrastructure Clock Division Factor For Sleep Mode."]
    #[inline(always)]
    pub const fn INFRCLKDIVS(self) -> crate::common::Reg<regs::INFRCLKDIVS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Infrastructure Clock Division Factor For DeepSleep Mode."]
    #[inline(always)]
    pub const fn INFRCLKDIVDS(self) -> crate::common::Reg<regs::INFRCLKDIVDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "MCU Voltage Domain Control."]
    #[inline(always)]
    pub const fn VDCTL(self) -> crate::common::Reg<regs::VDCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Load PRCM Settings To CLKCTRL Power Domain."]
    #[inline(always)]
    pub const fn CLKLOADCTL(self) -> crate::common::Reg<regs::CLKLOADCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "RFC Clock Gate."]
    #[inline(always)]
    pub const fn RFCCLKG(self) -> crate::common::Reg<regs::RFCCLKG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "VIMS Clock Gate."]
    #[inline(always)]
    pub const fn VIMSCLKG(self) -> crate::common::Reg<regs::VIMSCLKG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "TRNG, CRYPTO And UDMA Clock Gate For Run Mode."]
    #[inline(always)]
    pub const fn SECDMACLKGR(self) -> crate::common::Reg<regs::SECDMACLKGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "TRNG, CRYPTO And UDMA Clock Gate For Sleep Mode."]
    #[inline(always)]
    pub const fn SECDMACLKGS(self) -> crate::common::Reg<regs::SECDMACLKGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "TRNG, CRYPTO And UDMA Clock Gate For Deep Sleep Mode."]
    #[inline(always)]
    pub const fn SECDMACLKGDS(self) -> crate::common::Reg<regs::SECDMACLKGDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "GPIO Clock Gate For Run Mode."]
    #[inline(always)]
    pub const fn GPIOCLKGR(self) -> crate::common::Reg<regs::GPIOCLKGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "GPIO Clock Gate For Sleep Mode."]
    #[inline(always)]
    pub const fn GPIOCLKGS(self) -> crate::common::Reg<regs::GPIOCLKGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "GPIO Clock Gate For Deep Sleep Mode."]
    #[inline(always)]
    pub const fn GPIOCLKGDS(self) -> crate::common::Reg<regs::GPIOCLKGDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "GPT Clock Gate For Run Mode."]
    #[inline(always)]
    pub const fn GPTCLKGR(self) -> crate::common::Reg<regs::GPTCLKGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "GPT Clock Gate For Sleep Mode."]
    #[inline(always)]
    pub const fn GPTCLKGS(self) -> crate::common::Reg<regs::GPTCLKGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "GPT Clock Gate For Deep Sleep Mode."]
    #[inline(always)]
    pub const fn GPTCLKGDS(self) -> crate::common::Reg<regs::GPTCLKGDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "I2C Clock Gate For Run Mode."]
    #[inline(always)]
    pub const fn I2CCLKGR(self) -> crate::common::Reg<regs::I2CCLKGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "I2C Clock Gate For Sleep Mode."]
    #[inline(always)]
    pub const fn I2CCLKGS(self) -> crate::common::Reg<regs::I2CCLKGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "I2C Clock Gate For Deep Sleep Mode."]
    #[inline(always)]
    pub const fn I2CCLKGDS(self) -> crate::common::Reg<regs::I2CCLKGDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "UART Clock Gate For Run Mode."]
    #[inline(always)]
    pub const fn UARTCLKGR(self) -> crate::common::Reg<regs::UARTCLKGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "UART Clock Gate For Sleep Mode."]
    #[inline(always)]
    pub const fn UARTCLKGS(self) -> crate::common::Reg<regs::UARTCLKGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "UART Clock Gate For Deep Sleep Mode."]
    #[inline(always)]
    pub const fn UARTCLKGDS(self) -> crate::common::Reg<regs::UARTCLKGDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "SSI Clock Gate For Run Mode."]
    #[inline(always)]
    pub const fn SSICLKGR(self) -> crate::common::Reg<regs::SSICLKGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "SSI Clock Gate For Sleep Mode."]
    #[inline(always)]
    pub const fn SSICLKGS(self) -> crate::common::Reg<regs::SSICLKGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "SSI Clock Gate For Deep Sleep Mode."]
    #[inline(always)]
    pub const fn SSICLKGDS(self) -> crate::common::Reg<regs::SSICLKGDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "I2S Clock Gate For Run Mode."]
    #[inline(always)]
    pub const fn I2SCLKGR(self) -> crate::common::Reg<regs::I2SCLKGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "I2S Clock Gate For Sleep Mode."]
    #[inline(always)]
    pub const fn I2SCLKGS(self) -> crate::common::Reg<regs::I2SCLKGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "I2S Clock Gate For Deep Sleep Mode."]
    #[inline(always)]
    pub const fn I2SCLKGDS(self) -> crate::common::Reg<regs::I2SCLKGDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CPUCLKDIV(self) -> crate::common::Reg<regs::CPUCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn PERBUSDMACLKDIV(
        self,
    ) -> crate::common::Reg<regs::PERBUSDMACLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "I2S Clock Control."]
    #[inline(always)]
    pub const fn I2SBCLKSEL(self) -> crate::common::Reg<regs::I2SBCLKSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "GPT Scalar."]
    #[inline(always)]
    pub const fn GPTCLKDIV(self) -> crate::common::Reg<regs::GPTCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "I2S Clock Control."]
    #[inline(always)]
    pub const fn I2SCLKCTL(self) -> crate::common::Reg<regs::I2SCLKCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "MCLK Division Ratio."]
    #[inline(always)]
    pub const fn I2SMCLKDIV(self) -> crate::common::Reg<regs::I2SMCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "BCLK Division Ratio."]
    #[inline(always)]
    pub const fn I2SBCLKDIV(self) -> crate::common::Reg<regs::I2SBCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "WCLK Division Ratio."]
    #[inline(always)]
    pub const fn I2SWCLKDIV(self) -> crate::common::Reg<regs::I2SWCLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "SW Initiated Resets."]
    #[inline(always)]
    pub const fn SWRESET(self) -> crate::common::Reg<regs::SWRESET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "WARM Reset Control And Status."]
    #[inline(always)]
    pub const fn WARMRESET(self) -> crate::common::Reg<regs::WARMRESET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Power Domain Control."]
    #[inline(always)]
    pub const fn PDCTL0(self) -> crate::common::Reg<regs::PDCTL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "RFC Power Domain Control."]
    #[inline(always)]
    pub const fn PDCTL0RFC(self) -> crate::common::Reg<regs::PDCTL0RFC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "SERIAL Power Domain Control."]
    #[inline(always)]
    pub const fn PDCTL0SERIAL(self) -> crate::common::Reg<regs::PDCTL0SERIAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "PERIPH Power Domain Control."]
    #[inline(always)]
    pub const fn PDCTL0PERIPH(self) -> crate::common::Reg<regs::PDCTL0PERIPH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Power Domain Status."]
    #[inline(always)]
    pub const fn PDSTAT0(self) -> crate::common::Reg<regs::PDSTAT0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "RFC Power Domain Status."]
    #[inline(always)]
    pub const fn PDSTAT0RFC(self) -> crate::common::Reg<regs::PDSTAT0RFC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "SERIAL Power Domain Status."]
    #[inline(always)]
    pub const fn PDSTAT0SERIAL(self) -> crate::common::Reg<regs::PDSTAT0SERIAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "PERIPH Power Domain Status."]
    #[inline(always)]
    pub const fn PDSTAT0PERIPH(self) -> crate::common::Reg<regs::PDSTAT0PERIPH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Power Domain Control."]
    #[inline(always)]
    pub const fn PDCTL1(self) -> crate::common::Reg<regs::PDCTL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "CPU Power Domain Direct Control."]
    #[inline(always)]
    pub const fn PDCTL1CPU(self) -> crate::common::Reg<regs::PDCTL1CPU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "RFC Power Domain Direct Control."]
    #[inline(always)]
    pub const fn PDCTL1RFC(self) -> crate::common::Reg<regs::PDCTL1RFC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "VIMS Mode Direct Control."]
    #[inline(always)]
    pub const fn PDCTL1VIMS(self) -> crate::common::Reg<regs::PDCTL1VIMS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Power Manager Status."]
    #[inline(always)]
    pub const fn PDSTAT1(self) -> crate::common::Reg<regs::PDSTAT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "BUS Power Domain Direct Read Status."]
    #[inline(always)]
    pub const fn PDSTAT1BUS(self) -> crate::common::Reg<regs::PDSTAT1BUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "RFC Power Domain Direct Read Status."]
    #[inline(always)]
    pub const fn PDSTAT1RFC(self) -> crate::common::Reg<regs::PDSTAT1RFC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "CPU Power Domain Direct Read Status."]
    #[inline(always)]
    pub const fn PDSTAT1CPU(self) -> crate::common::Reg<regs::PDSTAT1CPU, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "VIMS Mode Direct Read Status."]
    #[inline(always)]
    pub const fn PDSTAT1VIMS(self) -> crate::common::Reg<regs::PDSTAT1VIMS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Control To RFC."]
    #[inline(always)]
    pub const fn RFCBITS(self) -> crate::common::Reg<regs::RFCBITS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "Selected RFC Mode."]
    #[inline(always)]
    pub const fn RFCMODESEL(self) -> crate::common::Reg<regs::RFCMODESEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "Allowed RFC Modes."]
    #[inline(always)]
    pub const fn RFCMODEHWOPT(self) -> crate::common::Reg<regs::RFCMODEHWOPT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "Power Profiler Register."]
    #[inline(always)]
    pub const fn PWRPROFSTAT(self) -> crate::common::Reg<regs::PWRPROFSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "Memory Retention Control."]
    #[inline(always)]
    pub const fn RAMRETEN(self) -> crate::common::Reg<regs::RAMRETEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
}
pub mod regs;
pub mod vals;

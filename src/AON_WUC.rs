#[doc = "This component control the Wakeup controller residing in the AON domain. Note: This module is only supporting 32 bit ReadWrite access from MCU."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AON_WUC {
    ptr: *mut u8,
}
unsafe impl Send for AON_WUC {}
unsafe impl Sync for AON_WUC {}
impl AON_WUC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MCU Clock Management This register contains bitfields related to the MCU clock."]
    #[inline(always)]
    pub const fn MCUCLK(self) -> crate::common::Reg<regs::MCUCLK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
    #[inline(always)]
    pub const fn AUXCLK(self) -> crate::common::Reg<regs::AUXCLK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MCU Configuration This register contains power management related bitfields for the MCU domain."]
    #[inline(always)]
    pub const fn MCUCFG(self) -> crate::common::Reg<regs::MCUCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "AUX Configuration This register contains power management related signals for the AUX domain."]
    #[inline(always)]
    pub const fn AUXCFG(self) -> crate::common::Reg<regs::AUXCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "AUX Control This register contains events and control signals for the AUX domain."]
    #[inline(always)]
    pub const fn AUXCTL(self) -> crate::common::Reg<regs::AUXCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up."]
    #[inline(always)]
    pub const fn PWRSTAT(self) -> crate::common::Reg<regs::PWRSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Shutdown Control This register contains bitfields required for entering shutdown mode."]
    #[inline(always)]
    pub const fn SHUTDOWN(self) -> crate::common::Reg<regs::SHUTDOWN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Control 0 This register contains various chip level control and debug bitfields."]
    #[inline(always)]
    pub const fn CTL0(self) -> crate::common::Reg<regs::CTL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Control 1 This register contains various chip level control and debug bitfields."]
    #[inline(always)]
    pub const fn CTL1(self) -> crate::common::Reg<regs::CTL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Recharge Controller Configuration This register sets all relevant patameters for controlling the recharge algorithm."]
    #[inline(always)]
    pub const fn RECHARGECFG(self) -> crate::common::Reg<regs::RECHARGECFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
    #[inline(always)]
    pub const fn RECHARGESTAT(self) -> crate::common::Reg<regs::RECHARGESTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
    #[inline(always)]
    pub const fn OSCCFG(self) -> crate::common::Reg<regs::OSCCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP."]
    #[inline(always)]
    pub const fn JTAGCFG(self) -> crate::common::Reg<regs::JTAGCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
    #[inline(always)]
    pub const fn JTAGUSERCODE(self) -> crate::common::Reg<regs::JTAGUSERCODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
}
pub mod regs;
pub mod vals;

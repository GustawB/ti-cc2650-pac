#[doc = "AUX Wake-up controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_WUC {
    ptr: *mut u8,
}
unsafe impl Send for AUX_WUC {}
unsafe impl Sync for AUX_WUC {}
impl AUX_WUC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Module Clock Enable Clock enable for each module in the AUX domain For use by the system CPU The settings in this register are OR'ed with the corresponding settings in MODCLKEN1. This allows the system CPU and AUX_SCE to request clocks independently. Settings take effect immediately."]
    #[inline(always)]
    pub const fn MODCLKEN0(self) -> crate::common::Reg<regs::MODCLKEN0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Power Off Request Requests power off request for the AUX domain. When powered off, the power supply and clock is disabled. This may only be used when taking the entire device into shutdown mode (i.e. with full device reset when resuming operation). Power off is prevented if AON_WUC:AUXCTL.AUX_FORCE_ON has been set, or if MCUBUSCTL.DISCONNECT_REQ has been cleared."]
    #[inline(always)]
    pub const fn PWROFFREQ(self) -> crate::common::Reg<regs::PWROFFREQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Power Down Request Request from AUX for system to enter power down. When system is in power down there is limited current supply available and the clock source is set by AON_WUC:AUXCLK.PWR_DWN_SRC."]
    #[inline(always)]
    pub const fn PWRDWNREQ(self) -> crate::common::Reg<regs::PWRDWNREQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Power Down Acknowledgment."]
    #[inline(always)]
    pub const fn PWRDWNACK(self) -> crate::common::Reg<regs::PWRDWNACK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Low Frequency Clock Request."]
    #[inline(always)]
    pub const fn CLKLFREQ(self) -> crate::common::Reg<regs::CLKLFREQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Low Frequency Clock Acknowledgment."]
    #[inline(always)]
    pub const fn CLKLFACK(self) -> crate::common::Reg<regs::CLKLFACK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Wake-up Event Flags Status of wake-up events from the AON domain The event flags are cleared by setting the corresponding bits in WUEVCLR."]
    #[inline(always)]
    pub const fn WUEVFLAGS(self) -> crate::common::Reg<regs::WUEVFLAGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Wake-up Event Clear Clears wake-up events from the AON domain."]
    #[inline(always)]
    pub const fn WUEVCLR(self) -> crate::common::Reg<regs::WUEVCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "ADC Clock Control Controls the ADC internal clock Note that the ADC command and data interface requires MODCLKEN0.ANAIF or MODCLKEN1.ANAIF also to be set."]
    #[inline(always)]
    pub const fn ADCCLKCTL(self) -> crate::common::Reg<regs::ADCCLKCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "TDC Clock Control Controls the TDC counter clock source, which steps the TDC counter value The source of this clock is controlled by OSC_DIG:CTL0.ACLK_TDC_SRC_SEL."]
    #[inline(always)]
    pub const fn TDCCLKCTL(self) -> crate::common::Reg<regs::TDCCLKCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Reference Clock Control Controls the TDC reference clock source, which is to be compared against the TDC counter clock. The source of this clock is controlled by OSC_DIG:CTL0.ACLK_REF_SRC_SEL."]
    #[inline(always)]
    pub const fn REFCLKCTL(self) -> crate::common::Reg<regs::REFCLKCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Real Time Counter Sub Second Increment 0 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 15:0. After setting INC15_0 and RTCSUBSECINC1.INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
    #[inline(always)]
    pub const fn RTCSUBSECINC0(self) -> crate::common::Reg<regs::RTCSUBSECINC0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Real Time Counter Sub Second Increment 1 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 23:16. After setting RTCSUBSECINC0.INC15_0 and INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
    #[inline(always)]
    pub const fn RTCSUBSECINC1(self) -> crate::common::Reg<regs::RTCSUBSECINC1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Real Time Counter Sub Second Increment Control."]
    #[inline(always)]
    pub const fn RTCSUBSECINCCTL(
        self,
    ) -> crate::common::Reg<regs::RTCSUBSECINCCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "MCU Bus Control Controls the connection between the AUX domain bus and the MCU domain bus. The buses must be disconnected to allow power-down or power-off of the AUX domain."]
    #[inline(always)]
    pub const fn MCUBUSCTL(self) -> crate::common::Reg<regs::MCUBUSCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "MCU Bus Status Indicates the connection state of the AUX domain and MCU domain buses. Note that this register cannot be read from the MCU domain while disconnected, and is therefore only useful for the AUX_SCE."]
    #[inline(always)]
    pub const fn MCUBUSSTAT(self) -> crate::common::Reg<regs::MCUBUSSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "AON Domain Control Status Status of AUX domain control from AON_WUC."]
    #[inline(always)]
    pub const fn AONCTLSTAT(self) -> crate::common::Reg<regs::AONCTLSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "AUX Input Output Latch Controls latching of signals between AUX_AIODIO0/AUX_AIODIO1 and AON_IOC."]
    #[inline(always)]
    pub const fn AUXIOLATCH(self) -> crate::common::Reg<regs::AUXIOLATCH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Module Clock Enable 1 Clock enable for each module in the AUX domain, for use by the AUX_SCE. Settings take effect immediately. The settings in this register are OR'ed with the corresponding settings in MODCLKEN0. This allows system CPU and AUX_SCE to request clocks independently."]
    #[inline(always)]
    pub const fn MODCLKEN1(self) -> crate::common::Reg<regs::MODCLKEN1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
}
pub mod regs;
pub mod vals;

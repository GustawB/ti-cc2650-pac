#[doc = "This is the DDI for the digital block that controls all the analog clock oscillators (OSC_DIG) and performs qualification of the clocks generated."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_DDI0_OSC {
    ptr: *mut u8,
}
unsafe impl Send for AUX_DDI0_OSC {}
unsafe impl Sync for AUX_DDI0_OSC {}
impl AUX_DDI0_OSC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control 0 Controls clock source selects."]
    #[inline(always)]
    pub const fn CTL0(self) -> crate::common::Reg<regs::CTL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control 1 This register contains OSC_DIG configuration."]
    #[inline(always)]
    pub const fn CTL1(self) -> crate::common::Reg<regs::CTL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RADC External Configuration."]
    #[inline(always)]
    pub const fn RADCEXTCFG(self) -> crate::common::Reg<regs::RADCEXTCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Amplitude Compensation Control."]
    #[inline(always)]
    pub const fn AMPCOMPCTL(self) -> crate::common::Reg<regs::AMPCOMPCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Amplitude Compensation Threshold 1 This register contains threshold values for amplitude compensation algorithm."]
    #[inline(always)]
    pub const fn AMPCOMPTH1(self) -> crate::common::Reg<regs::AMPCOMPTH1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Amplitude Compensation Threshold 2 This register contains threshold values for amplitude compensation algorithm."]
    #[inline(always)]
    pub const fn AMPCOMPTH2(self) -> crate::common::Reg<regs::AMPCOMPTH2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Analog Bypass Values 1."]
    #[inline(always)]
    pub const fn ANABYPASSVAL1(self) -> crate::common::Reg<regs::ANABYPASSVAL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ANABYPASSVAL2(self) -> crate::common::Reg<regs::ANABYPASSVAL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Analog Test Control."]
    #[inline(always)]
    pub const fn ATESTCTL(self) -> crate::common::Reg<regs::ATESTCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "ADC Doubler Nanoamp Control."]
    #[inline(always)]
    pub const fn ADCDOUBLERNANOAMPCTL(
        self,
    ) -> crate::common::Reg<regs::ADCDOUBLERNANOAMPCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "XOSCHF Control."]
    #[inline(always)]
    pub const fn XOSCHFCTL(self) -> crate::common::Reg<regs::XOSCHFCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Low Frequency Oscillator Control."]
    #[inline(always)]
    pub const fn LFOSCCTL(self) -> crate::common::Reg<regs::LFOSCCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "RCOSCHF Control."]
    #[inline(always)]
    pub const fn RCOSCHFCTL(self) -> crate::common::Reg<regs::RCOSCHFCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Status 0 This register contains status signals from OSC_DIG."]
    #[inline(always)]
    pub const fn STAT0(self) -> crate::common::Reg<regs::STAT0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Status 1 This register contains status signals from OSC_DIG."]
    #[inline(always)]
    pub const fn STAT1(self) -> crate::common::Reg<regs::STAT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Status 2 This register contains status signals from AMPCOMP FSM."]
    #[inline(always)]
    pub const fn STAT2(self) -> crate::common::Reg<regs::STAT2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
}
pub mod regs;
pub mod vals;

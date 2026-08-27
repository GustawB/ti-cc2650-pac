#[doc = "AUX Analog Peripheral Control Module."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_ANAIF {
    ptr: *mut u8,
}
unsafe impl Send for AUX_ANAIF {}
unsafe impl Sync for AUX_ANAIF {}
impl AUX_ANAIF {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion."]
    #[inline(always)]
    pub const fn ADCCTL(self) -> crate::common::Reg<regs::ADCCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "ADC FIFO Status FIFO can hold up to four ADC samples."]
    #[inline(always)]
    pub const fn ADCFIFOSTAT(self) -> crate::common::Reg<regs::ADCFIFOSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "ADC FIFO."]
    #[inline(always)]
    pub const fn ADCFIFO(self) -> crate::common::Reg<regs::ADCFIFO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "ADC Trigger."]
    #[inline(always)]
    pub const fn ADCTRIG(self) -> crate::common::Reg<regs::ADCTRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Current Source Control."]
    #[inline(always)]
    pub const fn ISRCCTL(self) -> crate::common::Reg<regs::ISRCCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
pub mod regs;
pub mod vals;

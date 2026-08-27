#[doc = "Configuration registers controlling analog peripherals of AUX. Registers Fields should be considered static unless otherwise noted (as dynamic)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_ADI4 {
    ptr: *mut u8,
}
unsafe impl Send for AUX_ADI4 {}
unsafe impl Sync for AUX_ADI4 {}
impl AUX_ADI4 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn MUX0(self) -> crate::common::Reg<regs::MUX0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn MUX1(self) -> crate::common::Reg<regs::MUX1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn MUX2(self) -> crate::common::Reg<regs::MUX2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn MUX3(self) -> crate::common::Reg<regs::MUX3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "Current Source Strength and trim control for current source. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ISRC(self) -> crate::common::Reg<regs::ISRC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Comparator Control COMPA and COMPB comparators. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn COMP(self) -> crate::common::Reg<regs::COMP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn MUX4(self) -> crate::common::Reg<regs::MUX4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "ADC Control 0 ADC Sample Control. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ADC0(self) -> crate::common::Reg<regs::ADC0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "ADC Control 1 ADC Comparator Control. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ADC1(self) -> crate::common::Reg<regs::ADC1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "ADC Reference 0 Control reference used by the ADC. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ADCREF0(self) -> crate::common::Reg<regs::ADCREF0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "ADC Reference 1 Control reference used by the ADC. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ADCREF1(self) -> crate::common::Reg<regs::ADCREF1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
}
pub mod regs;
pub mod vals;

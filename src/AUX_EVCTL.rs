#[doc = "AUX Event Controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_EVCTL {
    ptr: *mut u8,
}
unsafe impl Send for AUX_EVCTL {}
unsafe impl Sync for AUX_EVCTL {}
impl AUX_EVCTL {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Vector Configuration 0 AUX_SCE wakeup vector 0 and 1 configuration."]
    #[inline(always)]
    pub const fn VECCFG0(self) -> crate::common::Reg<regs::VECCFG0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Vector Configuration 1 AUX_SCE event vectors 2 and 3 configuration."]
    #[inline(always)]
    pub const fn VECCFG1(self) -> crate::common::Reg<regs::VECCFG1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Sensor Controller Engine Wait Event Selection Configuration of this register controls bit index 7 in AUX_SCE:WUSTAT.EV_SIGNALS. This bit can be used by AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions."]
    #[inline(always)]
    pub const fn SCEWEVSEL(self) -> crate::common::Reg<regs::SCEWEVSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Events To AON Flags This register contains a collection of event flags routed to AON_EVENT. To clear an event flag, write to EVTOAONFLAGSCLR or write 0 to event flag in this register."]
    #[inline(always)]
    pub const fn EVTOAONFLAGS(self) -> crate::common::Reg<regs::EVTOAONFLAGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS."]
    #[inline(always)]
    pub const fn EVTOAONPOL(self) -> crate::common::Reg<regs::EVTOAONPOL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Direct Memory Access Control."]
    #[inline(always)]
    pub const fn DMACTL(self) -> crate::common::Reg<regs::DMACTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Software Event Set Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR. Use of these event flags is software-defined."]
    #[inline(always)]
    pub const fn SWEVSET(self) -> crate::common::Reg<regs::SWEVSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Event Status 0 Register holds events 0 thru 15 of the 32-bit event bus that is synchronous to AUX clock. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC."]
    #[inline(always)]
    pub const fn EVSTAT0(self) -> crate::common::Reg<regs::EVSTAT0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Event Status 1 Current event source levels, 31:16."]
    #[inline(always)]
    pub const fn EVSTAT1(self) -> crate::common::Reg<regs::EVSTAT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS."]
    #[inline(always)]
    pub const fn EVTOMCUPOL(self) -> crate::common::Reg<regs::EVTOMCUPOL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag."]
    #[inline(always)]
    pub const fn EVTOMCUFLAGS(self) -> crate::common::Reg<regs::EVTOMCUFLAGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set."]
    #[inline(always)]
    pub const fn COMBEVTOMCUMASK(
        self,
    ) -> crate::common::Reg<regs::COMBEVTOMCUMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Vector Flags If a vector flag becomes 1 and AUX_SCE sleeps, AUX_SCE will wake up and execute the corresponding vector. The vector with the lowest index will execute first if multiple vectors flags are set. AUX_SCE must return to sleep to execute the next vector. During execution of a vector, AUX_SCE must clear the vector flag that triggered execution. Write 1 to bit index n in VECFLAGSCLR to clear vector flag n."]
    #[inline(always)]
    pub const fn VECFLAGS(self) -> crate::common::Reg<regs::VECFLAGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
    #[inline(always)]
    pub const fn EVTOMCUFLAGSCLR(
        self,
    ) -> crate::common::Reg<regs::EVTOMCUFLAGSCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
    #[inline(always)]
    pub const fn EVTOAONFLAGSCLR(
        self,
    ) -> crate::common::Reg<regs::EVTOAONFLAGSCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Vector Flags Clear Strobes for clearing flags in VECFLAGS."]
    #[inline(always)]
    pub const fn VECFLAGSCLR(self) -> crate::common::Reg<regs::VECFLAGSCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
}
pub mod regs;
pub mod vals;

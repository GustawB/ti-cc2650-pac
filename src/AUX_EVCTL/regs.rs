#[doc = "Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMBEVTOMCUMASK(pub u32);
impl COMBEVTOMCUMASK {
    #[doc = "0:0\\] EVTOMCUFLAGS.AON_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_WU_EV(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] EVTOMCUFLAGS.AON_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub const fn set_AON_WU_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] EVTOMCUFLAGS.AUX_COMPA contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPA(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] EVTOMCUFLAGS.AUX_COMPA contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub const fn set_AUX_COMPA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] EVTOMCUFLAGS.AUX_COMPB contribution to the AUX_COMB event. 0: Exclude 1: Include."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPB(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] EVTOMCUFLAGS.AUX_COMPB contribution to the AUX_COMB event. 0: Exclude 1: Include."]
    #[inline(always)]
    pub const fn set_AUX_COMPB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] EVTOMCUFLAGS.TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[must_use]
    #[inline(always)]
    pub const fn TDC_DONE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] EVTOMCUFLAGS.TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub const fn set_TDC_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] EVTOMCUFLAGS.TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER0_EV(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] EVTOMCUFLAGS.TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub const fn set_TIMER0_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] EVTOMCUFLAGS.TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER1_EV(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] EVTOMCUFLAGS.TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub const fn set_TIMER1_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[must_use]
    #[inline(always)]
    pub const fn SMPH_AUTOTAKE_DONE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub const fn set_SMPH_AUTOTAKE_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] EVTOMCUFLAGS.ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_DONE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] EVTOMCUFLAGS.ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub const fn set_ADC_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_FIFO_ALMOST_FULL(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub const fn set_ADC_FIFO_ALMOST_FULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[must_use]
    #[inline(always)]
    pub const fn OBSMUX0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub const fn set_OBSMUX0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] EVTOMCUFLAGS.ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_IRQ(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] EVTOMCUFLAGS.ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub const fn set_ADC_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "31:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED11(&self) -> u32 {
        let val = (self.0 >> 11usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "31:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED11(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 11usize)) | (((val as u32) & 0x001f_ffff) << 11usize);
    }
}
impl Default for COMBEVTOMCUMASK {
    #[inline(always)]
    fn default() -> COMBEVTOMCUMASK {
        COMBEVTOMCUMASK(0)
    }
}
impl core::fmt::Debug for COMBEVTOMCUMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMBEVTOMCUMASK")
            .field("AON_WU_EV", &self.AON_WU_EV())
            .field("AUX_COMPA", &self.AUX_COMPA())
            .field("AUX_COMPB", &self.AUX_COMPB())
            .field("TDC_DONE", &self.TDC_DONE())
            .field("TIMER0_EV", &self.TIMER0_EV())
            .field("TIMER1_EV", &self.TIMER1_EV())
            .field("SMPH_AUTOTAKE_DONE", &self.SMPH_AUTOTAKE_DONE())
            .field("ADC_DONE", &self.ADC_DONE())
            .field("ADC_FIFO_ALMOST_FULL", &self.ADC_FIFO_ALMOST_FULL())
            .field("OBSMUX0", &self.OBSMUX0())
            .field("ADC_IRQ", &self.ADC_IRQ())
            .field("RESERVED11", &self.RESERVED11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMBEVTOMCUMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COMBEVTOMCUMASK {{ AON_WU_EV: {=bool:?}, AUX_COMPA: {=bool:?}, AUX_COMPB: {=bool:?}, TDC_DONE: {=bool:?}, TIMER0_EV: {=bool:?}, TIMER1_EV: {=bool:?}, SMPH_AUTOTAKE_DONE: {=bool:?}, ADC_DONE: {=bool:?}, ADC_FIFO_ALMOST_FULL: {=bool:?}, OBSMUX0: {=bool:?}, ADC_IRQ: {=bool:?}, RESERVED11: {=u32:?} }}",
            self.AON_WU_EV(),
            self.AUX_COMPA(),
            self.AUX_COMPB(),
            self.TDC_DONE(),
            self.TIMER0_EV(),
            self.TIMER1_EV(),
            self.SMPH_AUTOTAKE_DONE(),
            self.ADC_DONE(),
            self.ADC_FIFO_ALMOST_FULL(),
            self.OBSMUX0(),
            self.ADC_IRQ(),
            self.RESERVED11()
        )
    }
}
#[doc = "Direct Memory Access Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACTL(pub u32);
impl DMACTL {
    #[doc = "0:0\\] Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> super::vals::SEL {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SEL::from_bits(val as u8)
    }
    #[doc = "0:0\\] Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data."]
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: super::vals::SEL) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] uDMA ADC interface enable. 0: Disable UDMA0 interface to ADC. 1: Enable UDMA0 interface to ADC."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] uDMA ADC interface enable. 0: Disable UDMA0 interface to ADC. 1: Enable UDMA0 interface to ADC."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] UDMA0 Request mode."]
    #[must_use]
    #[inline(always)]
    pub const fn REQ_MODE(&self) -> super::vals::REQ_MODE {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::REQ_MODE::from_bits(val as u8)
    }
    #[doc = "2:2\\] UDMA0 Request mode."]
    #[inline(always)]
    pub const fn set_REQ_MODE(&mut self, val: super::vals::REQ_MODE) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for DMACTL {
    #[inline(always)]
    fn default() -> DMACTL {
        DMACTL(0)
    }
}
impl core::fmt::Debug for DMACTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACTL")
            .field("SEL", &self.SEL())
            .field("EN", &self.EN())
            .field("REQ_MODE", &self.REQ_MODE())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACTL {{ SEL: {:?}, EN: {=bool:?}, REQ_MODE: {:?}, RESERVED3: {=u32:?} }}",
            self.SEL(),
            self.EN(),
            self.REQ_MODE(),
            self.RESERVED3()
        )
    }
}
#[doc = "Event Status 0 Register holds events 0 thru 15 of the 32-bit event bus that is synchronous to AUX clock. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVSTAT0(pub u32);
impl EVSTAT0 {
    #[doc = "0:0\\] AON_RTC:EVFLAGS.CH2."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_RTC_CH2(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] AON_RTC:EVFLAGS.CH2."]
    #[inline(always)]
    pub const fn set_AON_RTC_CH2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Comparator A output."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPA(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Comparator A output."]
    #[inline(always)]
    pub const fn set_AUX_COMPA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Comparator B output."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPB(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Comparator B output."]
    #[inline(always)]
    pub const fn set_AUX_COMPB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] AUX_TDC:STAT.DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn TDC_DONE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] AUX_TDC:STAT.DONE."]
    #[inline(always)]
    pub const fn set_TDC_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] AUX_TIMER0_EV event, see AUX_TIMER:T0TARGET for description."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER0_EV(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] AUX_TIMER0_EV event, see AUX_TIMER:T0TARGET for description."]
    #[inline(always)]
    pub const fn set_TIMER0_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] AUX_TIMER1_EV event, see AUX_TIMER:T1TARGET for description."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER1_EV(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] AUX_TIMER1_EV event, see AUX_TIMER:T1TARGET for description."]
    #[inline(always)]
    pub const fn set_TIMER1_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
    #[must_use]
    #[inline(always)]
    pub const fn SMPH_AUTOTAKE_DONE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
    #[inline(always)]
    pub const fn set_SMPH_AUTOTAKE_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] AUX_ANAIF ADC conversion done event."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_DONE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] AUX_ANAIF ADC conversion done event."]
    #[inline(always)]
    pub const fn set_ADC_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_FIFO_ALMOST_FULL(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL."]
    #[inline(always)]
    pub const fn set_ADC_FIFO_ALMOST_FULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
    #[must_use]
    #[inline(always)]
    pub const fn OBSMUX0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Observation input 0 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL0 and can be overridden by IOC:OBSAUXOUTPUT.SEL_MISC."]
    #[inline(always)]
    pub const fn set_OBSMUX0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
    #[must_use]
    #[inline(always)]
    pub const fn OBSMUX1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Observation input 1 from IOC. This event is configured by IOC:OBSAUXOUTPUT.SEL1."]
    #[inline(always)]
    pub const fn set_OBSMUX1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] AON_WUC:AUXCTL.SWEV."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_SW(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] AON_WUC:AUXCTL.SWEV."]
    #[inline(always)]
    pub const fn set_AON_SW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] AON_EVENT:AUXWUSEL.WU2_EV OR AON_EVENT:AUXWUSEL.WU1_EV OR AON_EVENT:AUXWUSEL.WU0_EV."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_PROG_WU(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] AON_EVENT:AUXWUSEL.WU2_EV OR AON_EVENT:AUXWUSEL.WU1_EV OR AON_EVENT:AUXWUSEL.WU0_EV."]
    #[inline(always)]
    pub const fn set_AON_PROG_WU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
    #[inline(always)]
    pub const fn set_AUXIO0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
    #[inline(always)]
    pub const fn set_AUXIO1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
    #[inline(always)]
    pub const fn set_AUXIO2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for EVSTAT0 {
    #[inline(always)]
    fn default() -> EVSTAT0 {
        EVSTAT0(0)
    }
}
impl core::fmt::Debug for EVSTAT0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVSTAT0")
            .field("AON_RTC_CH2", &self.AON_RTC_CH2())
            .field("AUX_COMPA", &self.AUX_COMPA())
            .field("AUX_COMPB", &self.AUX_COMPB())
            .field("TDC_DONE", &self.TDC_DONE())
            .field("TIMER0_EV", &self.TIMER0_EV())
            .field("TIMER1_EV", &self.TIMER1_EV())
            .field("SMPH_AUTOTAKE_DONE", &self.SMPH_AUTOTAKE_DONE())
            .field("ADC_DONE", &self.ADC_DONE())
            .field("ADC_FIFO_ALMOST_FULL", &self.ADC_FIFO_ALMOST_FULL())
            .field("OBSMUX0", &self.OBSMUX0())
            .field("OBSMUX1", &self.OBSMUX1())
            .field("AON_SW", &self.AON_SW())
            .field("AON_PROG_WU", &self.AON_PROG_WU())
            .field("AUXIO0", &self.AUXIO0())
            .field("AUXIO1", &self.AUXIO1())
            .field("AUXIO2", &self.AUXIO2())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EVSTAT0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EVSTAT0 {{ AON_RTC_CH2: {=bool:?}, AUX_COMPA: {=bool:?}, AUX_COMPB: {=bool:?}, TDC_DONE: {=bool:?}, TIMER0_EV: {=bool:?}, TIMER1_EV: {=bool:?}, SMPH_AUTOTAKE_DONE: {=bool:?}, ADC_DONE: {=bool:?}, ADC_FIFO_ALMOST_FULL: {=bool:?}, OBSMUX0: {=bool:?}, OBSMUX1: {=bool:?}, AON_SW: {=bool:?}, AON_PROG_WU: {=bool:?}, AUXIO0: {=bool:?}, AUXIO1: {=bool:?}, AUXIO2: {=bool:?}, RESERVED: {=u16:?} }}",
            self.AON_RTC_CH2(),
            self.AUX_COMPA(),
            self.AUX_COMPB(),
            self.TDC_DONE(),
            self.TIMER0_EV(),
            self.TIMER1_EV(),
            self.SMPH_AUTOTAKE_DONE(),
            self.ADC_DONE(),
            self.ADC_FIFO_ALMOST_FULL(),
            self.OBSMUX0(),
            self.OBSMUX1(),
            self.AON_SW(),
            self.AON_PROG_WU(),
            self.AUXIO0(),
            self.AUXIO1(),
            self.AUXIO2(),
            self.RESERVED()
        )
    }
}
#[doc = "Event Status 1 Current event source levels, 31:16."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVSTAT1(pub u32);
impl EVSTAT1 {
    #[doc = "0:0\\] AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO3(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
    #[inline(always)]
    pub const fn set_AUXIO3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO4(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
    #[inline(always)]
    pub const fn set_AUXIO4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO5(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
    #[inline(always)]
    pub const fn set_AUXIO5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO6(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
    #[inline(always)]
    pub const fn set_AUXIO6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO7(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
    #[inline(always)]
    pub const fn set_AUXIO7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO8(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
    #[inline(always)]
    pub const fn set_AUXIO8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO9(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
    #[inline(always)]
    pub const fn set_AUXIO9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO10(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
    #[inline(always)]
    pub const fn set_AUXIO10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO11(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
    #[inline(always)]
    pub const fn set_AUXIO11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO12(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
    #[inline(always)]
    pub const fn set_AUXIO12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO13(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
    #[inline(always)]
    pub const fn set_AUXIO13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO14(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
    #[inline(always)]
    pub const fn set_AUXIO14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
    #[must_use]
    #[inline(always)]
    pub const fn AUXIO15(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
    #[inline(always)]
    pub const fn set_AUXIO15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_WUC:REFCLKCTL.REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn ACLK_REF(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_WUC:REFCLKCTL.REQ."]
    #[inline(always)]
    pub const fn set_ACLK_REF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Event from EVENT configured by EVENT:AUXSEL0."]
    #[must_use]
    #[inline(always)]
    pub const fn MCU_EV(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Event from EVENT configured by EVENT:AUXSEL0."]
    #[inline(always)]
    pub const fn set_MCU_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_IRQ(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline(always)]
    pub const fn set_ADC_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for EVSTAT1 {
    #[inline(always)]
    fn default() -> EVSTAT1 {
        EVSTAT1(0)
    }
}
impl core::fmt::Debug for EVSTAT1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVSTAT1")
            .field("AUXIO3", &self.AUXIO3())
            .field("AUXIO4", &self.AUXIO4())
            .field("AUXIO5", &self.AUXIO5())
            .field("AUXIO6", &self.AUXIO6())
            .field("AUXIO7", &self.AUXIO7())
            .field("AUXIO8", &self.AUXIO8())
            .field("AUXIO9", &self.AUXIO9())
            .field("AUXIO10", &self.AUXIO10())
            .field("AUXIO11", &self.AUXIO11())
            .field("AUXIO12", &self.AUXIO12())
            .field("AUXIO13", &self.AUXIO13())
            .field("AUXIO14", &self.AUXIO14())
            .field("AUXIO15", &self.AUXIO15())
            .field("ACLK_REF", &self.ACLK_REF())
            .field("MCU_EV", &self.MCU_EV())
            .field("ADC_IRQ", &self.ADC_IRQ())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EVSTAT1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EVSTAT1 {{ AUXIO3: {=bool:?}, AUXIO4: {=bool:?}, AUXIO5: {=bool:?}, AUXIO6: {=bool:?}, AUXIO7: {=bool:?}, AUXIO8: {=bool:?}, AUXIO9: {=bool:?}, AUXIO10: {=bool:?}, AUXIO11: {=bool:?}, AUXIO12: {=bool:?}, AUXIO13: {=bool:?}, AUXIO14: {=bool:?}, AUXIO15: {=bool:?}, ACLK_REF: {=bool:?}, MCU_EV: {=bool:?}, ADC_IRQ: {=bool:?}, RESERVED16: {=u16:?} }}",
            self.AUXIO3(),
            self.AUXIO4(),
            self.AUXIO5(),
            self.AUXIO6(),
            self.AUXIO7(),
            self.AUXIO8(),
            self.AUXIO9(),
            self.AUXIO10(),
            self.AUXIO11(),
            self.AUXIO12(),
            self.AUXIO13(),
            self.AUXIO14(),
            self.AUXIO15(),
            self.ACLK_REF(),
            self.MCU_EV(),
            self.ADC_IRQ(),
            self.RESERVED16()
        )
    }
}
#[doc = "Events To AON Flags This register contains a collection of event flags routed to AON_EVENT. To clear an event flag, write to EVTOAONFLAGSCLR or write 0 to event flag in this register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVTOAONFLAGS(pub u32);
impl EVTOAONFLAGS {
    #[doc = "0:0\\] This event flag is set when software writes a 1 to SWEVSET.SWEV0."]
    #[must_use]
    #[inline(always)]
    pub const fn SWEV0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This event flag is set when software writes a 1 to SWEVSET.SWEV0."]
    #[inline(always)]
    pub const fn set_SWEV0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] This event flag is set when software writes a 1 to SWEVSET.SWEV1."]
    #[must_use]
    #[inline(always)]
    pub const fn SWEV1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] This event flag is set when software writes a 1 to SWEVSET.SWEV1."]
    #[inline(always)]
    pub const fn set_SWEV1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] This event flag is set when software writes a 1 to SWEVSET.SWEV2."]
    #[must_use]
    #[inline(always)]
    pub const fn SWEV2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] This event flag is set when software writes a 1 to SWEVSET.SWEV2."]
    #[inline(always)]
    pub const fn set_SWEV2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] This event flag is set when edge selected by EVTOAONPOL.AUX_COMPA occurs on EVSTAT0.AUX_COMPA."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPA(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] This event flag is set when edge selected by EVTOAONPOL.AUX_COMPA occurs on EVSTAT0.AUX_COMPA."]
    #[inline(always)]
    pub const fn set_AUX_COMPA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] This event flag is set when edge selected by EVTOAONPOL.AUX_COMPB occurs on EVSTAT0.AUX_COMPB."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPB(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] This event flag is set when edge selected by EVTOAONPOL.AUX_COMPB occurs on EVSTAT0.AUX_COMPB."]
    #[inline(always)]
    pub const fn set_AUX_COMPB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] This event flag is set when level selected by EVTOAONPOL.ADC_DONE occurs on EVSTAT0.ADC_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_DONE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] This event flag is set when level selected by EVTOAONPOL.ADC_DONE occurs on EVSTAT0.ADC_DONE."]
    #[inline(always)]
    pub const fn set_ADC_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] This event flag is set when level selected by EVTOAONPOL.TDC_DONE occurs on EVSTAT0.TDC_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn TDC_DONE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] This event flag is set when level selected by EVTOAONPOL.TDC_DONE occurs on EVSTAT0.TDC_DONE."]
    #[inline(always)]
    pub const fn set_TDC_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] This event flag is set when level selected by EVTOAONPOL.TIMER0_EV occurs on EVSTAT0.TIMER0_EV."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER0_EV(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] This event flag is set when level selected by EVTOAONPOL.TIMER0_EV occurs on EVSTAT0.TIMER0_EV."]
    #[inline(always)]
    pub const fn set_TIMER0_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] This event flag is set when level selected by EVTOAONPOL.TIMER1_EV occurs on EVSTAT0.TIMER1_EV."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER1_EV(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] This event flag is set when level selected by EVTOAONPOL.TIMER1_EV occurs on EVSTAT0.TIMER1_EV."]
    #[inline(always)]
    pub const fn set_TIMER1_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "31:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "31:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for EVTOAONFLAGS {
    #[inline(always)]
    fn default() -> EVTOAONFLAGS {
        EVTOAONFLAGS(0)
    }
}
impl core::fmt::Debug for EVTOAONFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVTOAONFLAGS")
            .field("SWEV0", &self.SWEV0())
            .field("SWEV1", &self.SWEV1())
            .field("SWEV2", &self.SWEV2())
            .field("AUX_COMPA", &self.AUX_COMPA())
            .field("AUX_COMPB", &self.AUX_COMPB())
            .field("ADC_DONE", &self.ADC_DONE())
            .field("TDC_DONE", &self.TDC_DONE())
            .field("TIMER0_EV", &self.TIMER0_EV())
            .field("TIMER1_EV", &self.TIMER1_EV())
            .field("RESERVED9", &self.RESERVED9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EVTOAONFLAGS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EVTOAONFLAGS {{ SWEV0: {=bool:?}, SWEV1: {=bool:?}, SWEV2: {=bool:?}, AUX_COMPA: {=bool:?}, AUX_COMPB: {=bool:?}, ADC_DONE: {=bool:?}, TDC_DONE: {=bool:?}, TIMER0_EV: {=bool:?}, TIMER1_EV: {=bool:?}, RESERVED9: {=u32:?} }}",
            self.SWEV0(),
            self.SWEV1(),
            self.SWEV2(),
            self.AUX_COMPA(),
            self.AUX_COMPB(),
            self.ADC_DONE(),
            self.TDC_DONE(),
            self.TIMER0_EV(),
            self.TIMER1_EV(),
            self.RESERVED9()
        )
    }
}
#[doc = "Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVTOAONFLAGSCLR(pub u32);
impl EVTOAONFLAGSCLR {
    #[doc = "0:0\\] Write 1 to clear EVTOAONFLAGS.SWEV0. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn SWEV0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Write 1 to clear EVTOAONFLAGS.SWEV0. Read value is 0."]
    #[inline(always)]
    pub const fn set_SWEV0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Write 1 to clear EVTOAONFLAGS.SWEV1. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn SWEV1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Write 1 to clear EVTOAONFLAGS.SWEV1. Read value is 0."]
    #[inline(always)]
    pub const fn set_SWEV1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Write 1 to clear EVTOAONFLAGS.SWEV2. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn SWEV2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Write 1 to clear EVTOAONFLAGS.SWEV2. Read value is 0."]
    #[inline(always)]
    pub const fn set_SWEV2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Write 1 to clear EVTOAONFLAGS.AUX_COMPA. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPA(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Write 1 to clear EVTOAONFLAGS.AUX_COMPA. Read value is 0."]
    #[inline(always)]
    pub const fn set_AUX_COMPA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Write 1 to clear EVTOAONFLAGS.AUX_COMPB. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPB(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Write 1 to clear EVTOAONFLAGS.AUX_COMPB. Read value is 0."]
    #[inline(always)]
    pub const fn set_AUX_COMPB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Write 1 to clear EVTOAONFLAGS.ADC_DONE. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_DONE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Write 1 to clear EVTOAONFLAGS.ADC_DONE. Read value is 0."]
    #[inline(always)]
    pub const fn set_ADC_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Write 1 to clear EVTOAONFLAGS.TDC_DONE. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn TDC_DONE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Write 1 to clear EVTOAONFLAGS.TDC_DONE. Read value is 0."]
    #[inline(always)]
    pub const fn set_TDC_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Write 1 to clear EVTOAONFLAGS.TIMER0_EV. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER0_EV(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Write 1 to clear EVTOAONFLAGS.TIMER0_EV. Read value is 0."]
    #[inline(always)]
    pub const fn set_TIMER0_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Write 1 to clear EVTOAONFLAGS.TIMER1_EV. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER1_EV(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Write 1 to clear EVTOAONFLAGS.TIMER1_EV. Read value is 0."]
    #[inline(always)]
    pub const fn set_TIMER1_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "31:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "31:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for EVTOAONFLAGSCLR {
    #[inline(always)]
    fn default() -> EVTOAONFLAGSCLR {
        EVTOAONFLAGSCLR(0)
    }
}
impl core::fmt::Debug for EVTOAONFLAGSCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVTOAONFLAGSCLR")
            .field("SWEV0", &self.SWEV0())
            .field("SWEV1", &self.SWEV1())
            .field("SWEV2", &self.SWEV2())
            .field("AUX_COMPA", &self.AUX_COMPA())
            .field("AUX_COMPB", &self.AUX_COMPB())
            .field("ADC_DONE", &self.ADC_DONE())
            .field("TDC_DONE", &self.TDC_DONE())
            .field("TIMER0_EV", &self.TIMER0_EV())
            .field("TIMER1_EV", &self.TIMER1_EV())
            .field("RESERVED9", &self.RESERVED9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EVTOAONFLAGSCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EVTOAONFLAGSCLR {{ SWEV0: {=bool:?}, SWEV1: {=bool:?}, SWEV2: {=bool:?}, AUX_COMPA: {=bool:?}, AUX_COMPB: {=bool:?}, ADC_DONE: {=bool:?}, TDC_DONE: {=bool:?}, TIMER0_EV: {=bool:?}, TIMER1_EV: {=bool:?}, RESERVED9: {=u32:?} }}",
            self.SWEV0(),
            self.SWEV1(),
            self.SWEV2(),
            self.AUX_COMPA(),
            self.AUX_COMPB(),
            self.ADC_DONE(),
            self.TDC_DONE(),
            self.TIMER0_EV(),
            self.TIMER1_EV(),
            self.RESERVED9()
        )
    }
}
#[doc = "Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVTOAONPOL(pub u32);
impl EVTOAONPOL {
    #[doc = "2:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "3:3\\] Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPA(&self) -> super::vals::EVTOAONPOL_AUX_COMPA {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::EVTOAONPOL_AUX_COMPA::from_bits(val as u8)
    }
    #[doc = "3:3\\] Select the edge of EVSTAT0.AUX_COMPA that sets EVTOAONFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub const fn set_AUX_COMPA(&mut self, val: super::vals::EVTOAONPOL_AUX_COMPA) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPB(&self) -> super::vals::EVTOAONPOL_AUX_COMPB {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::EVTOAONPOL_AUX_COMPB::from_bits(val as u8)
    }
    #[doc = "4:4\\] Select the edge of EVSTAT0.AUX_COMPB that sets EVTOAONFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub const fn set_AUX_COMPB(&mut self, val: super::vals::EVTOAONPOL_AUX_COMPB) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_DONE(&self) -> super::vals::EVTOAONPOL_ADC_DONE {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::EVTOAONPOL_ADC_DONE::from_bits(val as u8)
    }
    #[doc = "5:5\\] Select the level of EVSTAT0.ADC_DONE that sets EVTOAONFLAGS.ADC_DONE."]
    #[inline(always)]
    pub const fn set_ADC_DONE(&mut self, val: super::vals::EVTOAONPOL_ADC_DONE) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn TDC_DONE(&self) -> super::vals::EVTOAONPOL_TDC_DONE {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::EVTOAONPOL_TDC_DONE::from_bits(val as u8)
    }
    #[doc = "6:6\\] Select level of EVSTAT0.TDC_DONE that sets EVTOAONFLAGS.TDC_DONE."]
    #[inline(always)]
    pub const fn set_TDC_DONE(&mut self, val: super::vals::EVTOAONPOL_TDC_DONE) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER0_EV(&self) -> super::vals::EVTOAONPOL_TIMER0_EV {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::EVTOAONPOL_TIMER0_EV::from_bits(val as u8)
    }
    #[doc = "7:7\\] Select the level of EVSTAT0.TIMER0_EV that sets EVTOAONFLAGS.TIMER0_EV."]
    #[inline(always)]
    pub const fn set_TIMER0_EV(&mut self, val: super::vals::EVTOAONPOL_TIMER0_EV) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER1_EV(&self) -> super::vals::EVTOAONPOL_TIMER1_EV {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::EVTOAONPOL_TIMER1_EV::from_bits(val as u8)
    }
    #[doc = "8:8\\] Select the level of EVSTAT0.TIMER1_EV that sets EVTOAONFLAGS.TIMER1_EV."]
    #[inline(always)]
    pub const fn set_TIMER1_EV(&mut self, val: super::vals::EVTOAONPOL_TIMER1_EV) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "31:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "31:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for EVTOAONPOL {
    #[inline(always)]
    fn default() -> EVTOAONPOL {
        EVTOAONPOL(0)
    }
}
impl core::fmt::Debug for EVTOAONPOL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVTOAONPOL")
            .field("RESERVED2", &self.RESERVED2())
            .field("AUX_COMPA", &self.AUX_COMPA())
            .field("AUX_COMPB", &self.AUX_COMPB())
            .field("ADC_DONE", &self.ADC_DONE())
            .field("TDC_DONE", &self.TDC_DONE())
            .field("TIMER0_EV", &self.TIMER0_EV())
            .field("TIMER1_EV", &self.TIMER1_EV())
            .field("RESERVED9", &self.RESERVED9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EVTOAONPOL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EVTOAONPOL {{ RESERVED2: {=u8:?}, AUX_COMPA: {:?}, AUX_COMPB: {:?}, ADC_DONE: {:?}, TDC_DONE: {:?}, TIMER0_EV: {:?}, TIMER1_EV: {:?}, RESERVED9: {=u32:?} }}",
            self.RESERVED2(),
            self.AUX_COMPA(),
            self.AUX_COMPB(),
            self.ADC_DONE(),
            self.TDC_DONE(),
            self.TIMER0_EV(),
            self.TIMER1_EV(),
            self.RESERVED9()
        )
    }
}
#[doc = "Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVTOMCUFLAGS(pub u32);
impl EVTOMCUFLAGS {
    #[doc = "0:0\\] This event flag is set when level selected by EVTOMCUPOL.AON_WU_EV occurs on the reduction-OR of the AUX_EVCTL:EVSTAT0.RTC_CH2_EV, AUX_EVCTL:EVSTAT0.AON_SW, and AUX_EVCTL:EVSTAT0.AON_PROG_WU events."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_WU_EV(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This event flag is set when level selected by EVTOMCUPOL.AON_WU_EV occurs on the reduction-OR of the AUX_EVCTL:EVSTAT0.RTC_CH2_EV, AUX_EVCTL:EVSTAT0.AON_SW, and AUX_EVCTL:EVSTAT0.AON_PROG_WU events."]
    #[inline(always)]
    pub const fn set_AON_WU_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] This event flag is set when edge selected by EVTOMCUPOL.AUX_COMPA occurs on EVSTAT0.AUX_COMPA."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPA(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] This event flag is set when edge selected by EVTOMCUPOL.AUX_COMPA occurs on EVSTAT0.AUX_COMPA."]
    #[inline(always)]
    pub const fn set_AUX_COMPA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] This event flag is set when edge selected by EVTOMCUPOL.AUX_COMPB occurs on EVSTAT0.AUX_COMPB."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPB(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] This event flag is set when edge selected by EVTOMCUPOL.AUX_COMPB occurs on EVSTAT0.AUX_COMPB."]
    #[inline(always)]
    pub const fn set_AUX_COMPB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] This event flag is set when level selected by EVTOMCUPOL.TDC_DONE occurs on EVSTAT0.TDC_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn TDC_DONE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] This event flag is set when level selected by EVTOMCUPOL.TDC_DONE occurs on EVSTAT0.TDC_DONE."]
    #[inline(always)]
    pub const fn set_TDC_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] This event flag is set when level selected by EVTOMCUPOL.TIMER0_EV occurs on EVSTAT0.TIMER0_EV."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER0_EV(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] This event flag is set when level selected by EVTOMCUPOL.TIMER0_EV occurs on EVSTAT0.TIMER0_EV."]
    #[inline(always)]
    pub const fn set_TIMER0_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] This event flag is set when level selected by EVTOMCUPOL.TIMER1_EV occurs on EVSTAT0.TIMER1_EV."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER1_EV(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] This event flag is set when level selected by EVTOMCUPOL.TIMER1_EV occurs on EVSTAT0.TIMER1_EV."]
    #[inline(always)]
    pub const fn set_TIMER1_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] This event flag is set when level selected by EVTOMCUPOL.SMPH_AUTOTAKE_DONE occurs on EVSTAT0.SMPH_AUTOTAKE_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn SMPH_AUTOTAKE_DONE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] This event flag is set when level selected by EVTOMCUPOL.SMPH_AUTOTAKE_DONE occurs on EVSTAT0.SMPH_AUTOTAKE_DONE."]
    #[inline(always)]
    pub const fn set_SMPH_AUTOTAKE_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] This event flag is set when level selected by EVTOMCUPOL.ADC_DONE occurs on EVSTAT0.ADC_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_DONE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] This event flag is set when level selected by EVTOMCUPOL.ADC_DONE occurs on EVSTAT0.ADC_DONE."]
    #[inline(always)]
    pub const fn set_ADC_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] This event flag is set when level selected by EVTOMCUPOL.ADC_FIFO_ALMOST_FULL occurs on EVSTAT0.ADC_FIFO_ALMOST_FULL."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_FIFO_ALMOST_FULL(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] This event flag is set when level selected by EVTOMCUPOL.ADC_FIFO_ALMOST_FULL occurs on EVSTAT0.ADC_FIFO_ALMOST_FULL."]
    #[inline(always)]
    pub const fn set_ADC_FIFO_ALMOST_FULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] This event flag is set when level selected by EVTOMCUPOL.MCU_OBSMUX0 occurs on EVSTAT0.MCU_OBSMUX0."]
    #[must_use]
    #[inline(always)]
    pub const fn OBSMUX0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] This event flag is set when level selected by EVTOMCUPOL.MCU_OBSMUX0 occurs on EVSTAT0.MCU_OBSMUX0."]
    #[inline(always)]
    pub const fn set_OBSMUX0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] This event flag is set when level selected by EVTOMCUPOL.ADC_IRQ occurs on EVSTAT0.ADC_IRQ."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_IRQ(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] This event flag is set when level selected by EVTOMCUPOL.ADC_IRQ occurs on EVSTAT0.ADC_IRQ."]
    #[inline(always)]
    pub const fn set_ADC_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "31:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED11(&self) -> u32 {
        let val = (self.0 >> 11usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "31:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED11(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 11usize)) | (((val as u32) & 0x001f_ffff) << 11usize);
    }
}
impl Default for EVTOMCUFLAGS {
    #[inline(always)]
    fn default() -> EVTOMCUFLAGS {
        EVTOMCUFLAGS(0)
    }
}
impl core::fmt::Debug for EVTOMCUFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVTOMCUFLAGS")
            .field("AON_WU_EV", &self.AON_WU_EV())
            .field("AUX_COMPA", &self.AUX_COMPA())
            .field("AUX_COMPB", &self.AUX_COMPB())
            .field("TDC_DONE", &self.TDC_DONE())
            .field("TIMER0_EV", &self.TIMER0_EV())
            .field("TIMER1_EV", &self.TIMER1_EV())
            .field("SMPH_AUTOTAKE_DONE", &self.SMPH_AUTOTAKE_DONE())
            .field("ADC_DONE", &self.ADC_DONE())
            .field("ADC_FIFO_ALMOST_FULL", &self.ADC_FIFO_ALMOST_FULL())
            .field("OBSMUX0", &self.OBSMUX0())
            .field("ADC_IRQ", &self.ADC_IRQ())
            .field("RESERVED11", &self.RESERVED11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EVTOMCUFLAGS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EVTOMCUFLAGS {{ AON_WU_EV: {=bool:?}, AUX_COMPA: {=bool:?}, AUX_COMPB: {=bool:?}, TDC_DONE: {=bool:?}, TIMER0_EV: {=bool:?}, TIMER1_EV: {=bool:?}, SMPH_AUTOTAKE_DONE: {=bool:?}, ADC_DONE: {=bool:?}, ADC_FIFO_ALMOST_FULL: {=bool:?}, OBSMUX0: {=bool:?}, ADC_IRQ: {=bool:?}, RESERVED11: {=u32:?} }}",
            self.AON_WU_EV(),
            self.AUX_COMPA(),
            self.AUX_COMPB(),
            self.TDC_DONE(),
            self.TIMER0_EV(),
            self.TIMER1_EV(),
            self.SMPH_AUTOTAKE_DONE(),
            self.ADC_DONE(),
            self.ADC_FIFO_ALMOST_FULL(),
            self.OBSMUX0(),
            self.ADC_IRQ(),
            self.RESERVED11()
        )
    }
}
#[doc = "Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVTOMCUFLAGSCLR(pub u32);
impl EVTOMCUFLAGSCLR {
    #[doc = "0:0\\] Write 1 to clear EVTOMCUFLAGS.AON_WU_EV. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_WU_EV(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Write 1 to clear EVTOMCUFLAGS.AON_WU_EV. Read value is 0."]
    #[inline(always)]
    pub const fn set_AON_WU_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Write 1 to clear EVTOMCUFLAGS.AUX_COMPA. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPA(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Write 1 to clear EVTOMCUFLAGS.AUX_COMPA. Read value is 0."]
    #[inline(always)]
    pub const fn set_AUX_COMPA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Write 1 to clear EVTOMCUFLAGS.AUX_COMPB. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPB(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Write 1 to clear EVTOMCUFLAGS.AUX_COMPB. Read value is 0."]
    #[inline(always)]
    pub const fn set_AUX_COMPB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Write 1 to clear EVTOMCUFLAGS.TDC_DONE. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn TDC_DONE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Write 1 to clear EVTOMCUFLAGS.TDC_DONE. Read value is 0."]
    #[inline(always)]
    pub const fn set_TDC_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Write 1 to clear EVTOMCUFLAGS.TIMER0_EV. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER0_EV(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Write 1 to clear EVTOMCUFLAGS.TIMER0_EV. Read value is 0."]
    #[inline(always)]
    pub const fn set_TIMER0_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Write 1 to clear EVTOMCUFLAGS.TIMER1_EV. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER1_EV(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Write 1 to clear EVTOMCUFLAGS.TIMER1_EV. Read value is 0."]
    #[inline(always)]
    pub const fn set_TIMER1_EV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Write 1 to clear EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn SMPH_AUTOTAKE_DONE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Write 1 to clear EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE. Read value is 0."]
    #[inline(always)]
    pub const fn set_SMPH_AUTOTAKE_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Write 1 to clear EVTOMCUFLAGS.ADC_DONE. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_DONE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Write 1 to clear EVTOMCUFLAGS.ADC_DONE. Read value is 0."]
    #[inline(always)]
    pub const fn set_ADC_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Write 1 to clear EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_FIFO_ALMOST_FULL(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Write 1 to clear EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL. Read value is 0."]
    #[inline(always)]
    pub const fn set_ADC_FIFO_ALMOST_FULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Write 1 to clear EVTOMCUFLAGS.MCU_OBSMUX0. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn OBSMUX0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Write 1 to clear EVTOMCUFLAGS.MCU_OBSMUX0. Read value is 0."]
    #[inline(always)]
    pub const fn set_OBSMUX0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Write 1 to clear EVTOMCUFLAGS.ADC_IRQ. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_IRQ(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Write 1 to clear EVTOMCUFLAGS.ADC_IRQ. Read value is 0."]
    #[inline(always)]
    pub const fn set_ADC_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "31:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED11(&self) -> u32 {
        let val = (self.0 >> 11usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "31:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED11(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 11usize)) | (((val as u32) & 0x001f_ffff) << 11usize);
    }
}
impl Default for EVTOMCUFLAGSCLR {
    #[inline(always)]
    fn default() -> EVTOMCUFLAGSCLR {
        EVTOMCUFLAGSCLR(0)
    }
}
impl core::fmt::Debug for EVTOMCUFLAGSCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVTOMCUFLAGSCLR")
            .field("AON_WU_EV", &self.AON_WU_EV())
            .field("AUX_COMPA", &self.AUX_COMPA())
            .field("AUX_COMPB", &self.AUX_COMPB())
            .field("TDC_DONE", &self.TDC_DONE())
            .field("TIMER0_EV", &self.TIMER0_EV())
            .field("TIMER1_EV", &self.TIMER1_EV())
            .field("SMPH_AUTOTAKE_DONE", &self.SMPH_AUTOTAKE_DONE())
            .field("ADC_DONE", &self.ADC_DONE())
            .field("ADC_FIFO_ALMOST_FULL", &self.ADC_FIFO_ALMOST_FULL())
            .field("OBSMUX0", &self.OBSMUX0())
            .field("ADC_IRQ", &self.ADC_IRQ())
            .field("RESERVED11", &self.RESERVED11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EVTOMCUFLAGSCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EVTOMCUFLAGSCLR {{ AON_WU_EV: {=bool:?}, AUX_COMPA: {=bool:?}, AUX_COMPB: {=bool:?}, TDC_DONE: {=bool:?}, TIMER0_EV: {=bool:?}, TIMER1_EV: {=bool:?}, SMPH_AUTOTAKE_DONE: {=bool:?}, ADC_DONE: {=bool:?}, ADC_FIFO_ALMOST_FULL: {=bool:?}, OBSMUX0: {=bool:?}, ADC_IRQ: {=bool:?}, RESERVED11: {=u32:?} }}",
            self.AON_WU_EV(),
            self.AUX_COMPA(),
            self.AUX_COMPB(),
            self.TDC_DONE(),
            self.TIMER0_EV(),
            self.TIMER1_EV(),
            self.SMPH_AUTOTAKE_DONE(),
            self.ADC_DONE(),
            self.ADC_FIFO_ALMOST_FULL(),
            self.OBSMUX0(),
            self.ADC_IRQ(),
            self.RESERVED11()
        )
    }
}
#[doc = "Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVTOMCUPOL(pub u32);
impl EVTOMCUPOL {
    #[doc = "0:0\\] Select the event source level that sets EVTOMCUFLAGS.AON_WU_EV."]
    #[must_use]
    #[inline(always)]
    pub const fn AON_WU_EV(&self) -> super::vals::AON_WU_EV {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::AON_WU_EV::from_bits(val as u8)
    }
    #[doc = "0:0\\] Select the event source level that sets EVTOMCUFLAGS.AON_WU_EV."]
    #[inline(always)]
    pub const fn set_AON_WU_EV(&mut self, val: super::vals::AON_WU_EV) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Select the event source level that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPA(&self) -> super::vals::EVTOMCUPOL_AUX_COMPA {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::EVTOMCUPOL_AUX_COMPA::from_bits(val as u8)
    }
    #[doc = "1:1\\] Select the event source level that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[inline(always)]
    pub const fn set_AUX_COMPA(&mut self, val: super::vals::EVTOMCUPOL_AUX_COMPA) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Select the event source level that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_COMPB(&self) -> super::vals::EVTOMCUPOL_AUX_COMPB {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::EVTOMCUPOL_AUX_COMPB::from_bits(val as u8)
    }
    #[doc = "2:2\\] Select the event source level that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[inline(always)]
    pub const fn set_AUX_COMPB(&mut self, val: super::vals::EVTOMCUPOL_AUX_COMPB) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Select the event source level that sets EVTOMCUFLAGS.TDC_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn TDC_DONE(&self) -> super::vals::EVTOMCUPOL_TDC_DONE {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::EVTOMCUPOL_TDC_DONE::from_bits(val as u8)
    }
    #[doc = "3:3\\] Select the event source level that sets EVTOMCUFLAGS.TDC_DONE."]
    #[inline(always)]
    pub const fn set_TDC_DONE(&mut self, val: super::vals::EVTOMCUPOL_TDC_DONE) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Select the event source level that sets EVTOMCUFLAGS.TIMER0_EV."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER0_EV(&self) -> super::vals::EVTOMCUPOL_TIMER0_EV {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::EVTOMCUPOL_TIMER0_EV::from_bits(val as u8)
    }
    #[doc = "4:4\\] Select the event source level that sets EVTOMCUFLAGS.TIMER0_EV."]
    #[inline(always)]
    pub const fn set_TIMER0_EV(&mut self, val: super::vals::EVTOMCUPOL_TIMER0_EV) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Select the event source level that sets EVTOMCUFLAGS.TIMER1_EV."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER1_EV(&self) -> super::vals::EVTOMCUPOL_TIMER1_EV {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::EVTOMCUPOL_TIMER1_EV::from_bits(val as u8)
    }
    #[doc = "5:5\\] Select the event source level that sets EVTOMCUFLAGS.TIMER1_EV."]
    #[inline(always)]
    pub const fn set_TIMER1_EV(&mut self, val: super::vals::EVTOMCUPOL_TIMER1_EV) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Select the event source level that sets EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn SMPH_AUTOTAKE_DONE(&self) -> super::vals::SMPH_AUTOTAKE_DONE {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SMPH_AUTOTAKE_DONE::from_bits(val as u8)
    }
    #[doc = "6:6\\] Select the event source level that sets EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE."]
    #[inline(always)]
    pub const fn set_SMPH_AUTOTAKE_DONE(&mut self, val: super::vals::SMPH_AUTOTAKE_DONE) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Select the event source level that sets EVTOMCUFLAGS.ADC_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_DONE(&self) -> super::vals::EVTOMCUPOL_ADC_DONE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::EVTOMCUPOL_ADC_DONE::from_bits(val as u8)
    }
    #[doc = "7:7\\] Select the event source level that sets EVTOMCUFLAGS.ADC_DONE."]
    #[inline(always)]
    pub const fn set_ADC_DONE(&mut self, val: super::vals::EVTOMCUPOL_ADC_DONE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Select the event source level that sets EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_FIFO_ALMOST_FULL(&self) -> super::vals::ADC_FIFO_ALMOST_FULL {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::ADC_FIFO_ALMOST_FULL::from_bits(val as u8)
    }
    #[doc = "8:8\\] Select the event source level that sets EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    #[inline(always)]
    pub const fn set_ADC_FIFO_ALMOST_FULL(&mut self, val: super::vals::ADC_FIFO_ALMOST_FULL) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Select the event source level that sets EVTOMCUFLAGS.OBSMUX0."]
    #[must_use]
    #[inline(always)]
    pub const fn OBSMUX0(&self) -> super::vals::OBSMUX0 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::OBSMUX0::from_bits(val as u8)
    }
    #[doc = "9:9\\] Select the event source level that sets EVTOMCUFLAGS.OBSMUX0."]
    #[inline(always)]
    pub const fn set_OBSMUX0(&mut self, val: super::vals::OBSMUX0) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Select the event source level that sets EVTOMCUFLAGS.ADC_IRQ."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_IRQ(&self) -> super::vals::ADC_IRQ {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::ADC_IRQ::from_bits(val as u8)
    }
    #[doc = "10:10\\] Select the event source level that sets EVTOMCUFLAGS.ADC_IRQ."]
    #[inline(always)]
    pub const fn set_ADC_IRQ(&mut self, val: super::vals::ADC_IRQ) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "31:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED11(&self) -> u32 {
        let val = (self.0 >> 11usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "31:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED11(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 11usize)) | (((val as u32) & 0x001f_ffff) << 11usize);
    }
}
impl Default for EVTOMCUPOL {
    #[inline(always)]
    fn default() -> EVTOMCUPOL {
        EVTOMCUPOL(0)
    }
}
impl core::fmt::Debug for EVTOMCUPOL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVTOMCUPOL")
            .field("AON_WU_EV", &self.AON_WU_EV())
            .field("AUX_COMPA", &self.AUX_COMPA())
            .field("AUX_COMPB", &self.AUX_COMPB())
            .field("TDC_DONE", &self.TDC_DONE())
            .field("TIMER0_EV", &self.TIMER0_EV())
            .field("TIMER1_EV", &self.TIMER1_EV())
            .field("SMPH_AUTOTAKE_DONE", &self.SMPH_AUTOTAKE_DONE())
            .field("ADC_DONE", &self.ADC_DONE())
            .field("ADC_FIFO_ALMOST_FULL", &self.ADC_FIFO_ALMOST_FULL())
            .field("OBSMUX0", &self.OBSMUX0())
            .field("ADC_IRQ", &self.ADC_IRQ())
            .field("RESERVED11", &self.RESERVED11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EVTOMCUPOL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EVTOMCUPOL {{ AON_WU_EV: {:?}, AUX_COMPA: {:?}, AUX_COMPB: {:?}, TDC_DONE: {:?}, TIMER0_EV: {:?}, TIMER1_EV: {:?}, SMPH_AUTOTAKE_DONE: {:?}, ADC_DONE: {:?}, ADC_FIFO_ALMOST_FULL: {:?}, OBSMUX0: {:?}, ADC_IRQ: {:?}, RESERVED11: {=u32:?} }}",
            self.AON_WU_EV(),
            self.AUX_COMPA(),
            self.AUX_COMPB(),
            self.TDC_DONE(),
            self.TIMER0_EV(),
            self.TIMER1_EV(),
            self.SMPH_AUTOTAKE_DONE(),
            self.ADC_DONE(),
            self.ADC_FIFO_ALMOST_FULL(),
            self.OBSMUX0(),
            self.ADC_IRQ(),
            self.RESERVED11()
        )
    }
}
#[doc = "Sensor Controller Engine Wait Event Selection Configuration of this register controls bit index 7 in AUX_SCE:WUSTAT.EV_SIGNALS. This bit can be used by AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCEWEVSEL(pub u32);
impl SCEWEVSEL {
    #[doc = "4:0\\] Select event source to connect to AUX_SCE:WUSTAT.EV_SIGNALS bit 7."]
    #[must_use]
    #[inline(always)]
    pub const fn WEV7_EV(&self) -> super::vals::WEV7_EV {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::WEV7_EV::from_bits(val as u8)
    }
    #[doc = "4:0\\] Select event source to connect to AUX_SCE:WUSTAT.EV_SIGNALS bit 7."]
    #[inline(always)]
    pub const fn set_WEV7_EV(&mut self, val: super::vals::WEV7_EV) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for SCEWEVSEL {
    #[inline(always)]
    fn default() -> SCEWEVSEL {
        SCEWEVSEL(0)
    }
}
impl core::fmt::Debug for SCEWEVSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCEWEVSEL")
            .field("WEV7_EV", &self.WEV7_EV())
            .field("RESERVED5", &self.RESERVED5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SCEWEVSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SCEWEVSEL {{ WEV7_EV: {:?}, RESERVED5: {=u32:?} }}",
            self.WEV7_EV(),
            self.RESERVED5()
        )
    }
}
#[doc = "Software Event Set Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR. Use of these event flags is software-defined."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SWEVSET(pub u32);
impl SWEVSET {
    #[doc = "0:0\\] Software event flag 0. 0: No effect. 1: Set software event flag 0."]
    #[must_use]
    #[inline(always)]
    pub const fn SWEV0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Software event flag 0. 0: No effect. 1: Set software event flag 0."]
    #[inline(always)]
    pub const fn set_SWEV0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Software event flag 1. 0: No effect. 1: Set software event flag 1."]
    #[must_use]
    #[inline(always)]
    pub const fn SWEV1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Software event flag 1. 0: No effect. 1: Set software event flag 1."]
    #[inline(always)]
    pub const fn set_SWEV1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Software event flag 2. 0: No effect. 1: Set software event flag 2."]
    #[must_use]
    #[inline(always)]
    pub const fn SWEV2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Software event flag 2. 0: No effect. 1: Set software event flag 2."]
    #[inline(always)]
    pub const fn set_SWEV2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for SWEVSET {
    #[inline(always)]
    fn default() -> SWEVSET {
        SWEVSET(0)
    }
}
impl core::fmt::Debug for SWEVSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWEVSET")
            .field("SWEV0", &self.SWEV0())
            .field("SWEV1", &self.SWEV1())
            .field("SWEV2", &self.SWEV2())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SWEVSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SWEVSET {{ SWEV0: {=bool:?}, SWEV1: {=bool:?}, SWEV2: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.SWEV0(),
            self.SWEV1(),
            self.SWEV2(),
            self.RESERVED3()
        )
    }
}
#[doc = "Vector Configuration 0 AUX_SCE wakeup vector 0 and 1 configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VECCFG0(pub u32);
impl VECCFG0 {
    #[doc = "4:0\\] Select vector 0 trigger source event."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC0_EV(&self) -> super::vals::VEC0_EV {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::VEC0_EV::from_bits(val as u8)
    }
    #[doc = "4:0\\] Select vector 0 trigger source event."]
    #[inline(always)]
    pub const fn set_VEC0_EV(&mut self, val: super::vals::VEC0_EV) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "5:5\\] Vector 0 trigger enable. When enabled, VEC0_EV event with VEC0_POL polarity triggers a jump to vector # 0 when AUX_SCE sleeps."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC0_EN(&self) -> super::vals::VEC0_EN {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::VEC0_EN::from_bits(val as u8)
    }
    #[doc = "5:5\\] Vector 0 trigger enable. When enabled, VEC0_EV event with VEC0_POL polarity triggers a jump to vector # 0 when AUX_SCE sleeps."]
    #[inline(always)]
    pub const fn set_VEC0_EN(&mut self, val: super::vals::VEC0_EN) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Vector 0 trigger event polarity. To manually trigger vector 0 execution: - AUX_SCE must sleep. - Set VEC0_EV to a known static value. - Toggle VEC0_POL twice."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC0_POL(&self) -> super::vals::VEC0_POL {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::VEC0_POL::from_bits(val as u8)
    }
    #[doc = "6:6\\] Vector 0 trigger event polarity. To manually trigger vector 0 execution: - AUX_SCE must sleep. - Set VEC0_EV to a known static value. - Toggle VEC0_POL twice."]
    #[inline(always)]
    pub const fn set_VEC0_POL(&mut self, val: super::vals::VEC0_POL) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "12:8\\] Select vector 1 trigger source event."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC1_EV(&self) -> super::vals::VEC1_EV {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::VEC1_EV::from_bits(val as u8)
    }
    #[doc = "12:8\\] Select vector 1 trigger source event."]
    #[inline(always)]
    pub const fn set_VEC1_EV(&mut self, val: super::vals::VEC1_EV) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "13:13\\] Vector 1 trigger enable. When enabled, VEC1_EV event with VEC1_POL polarity triggers a jump to vector # 1 when AUX_SCE sleeps. Lower vectors (0) have priority."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC1_EN(&self) -> super::vals::VEC1_EN {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::VEC1_EN::from_bits(val as u8)
    }
    #[doc = "13:13\\] Vector 1 trigger enable. When enabled, VEC1_EV event with VEC1_POL polarity triggers a jump to vector # 1 when AUX_SCE sleeps. Lower vectors (0) have priority."]
    #[inline(always)]
    pub const fn set_VEC1_EN(&mut self, val: super::vals::VEC1_EN) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Vector 1 trigger event polarity. To manually trigger vector 1 execution: - AUX_SCE must sleep. - Set VEC1_EV to a known static value. - Toggle VEC1_POL twice."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC1_POL(&self) -> super::vals::VEC1_POL {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::VEC1_POL::from_bits(val as u8)
    }
    #[doc = "14:14\\] Vector 1 trigger event polarity. To manually trigger vector 1 execution: - AUX_SCE must sleep. - Set VEC1_EV to a known static value. - Toggle VEC1_POL twice."]
    #[inline(always)]
    pub const fn set_VEC1_POL(&mut self, val: super::vals::VEC1_POL) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "31:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> u32 {
        let val = (self.0 >> 15usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "31:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 15usize)) | (((val as u32) & 0x0001_ffff) << 15usize);
    }
}
impl Default for VECCFG0 {
    #[inline(always)]
    fn default() -> VECCFG0 {
        VECCFG0(0)
    }
}
impl core::fmt::Debug for VECCFG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VECCFG0")
            .field("VEC0_EV", &self.VEC0_EV())
            .field("VEC0_EN", &self.VEC0_EN())
            .field("VEC0_POL", &self.VEC0_POL())
            .field("RESERVED7", &self.RESERVED7())
            .field("VEC1_EV", &self.VEC1_EV())
            .field("VEC1_EN", &self.VEC1_EN())
            .field("VEC1_POL", &self.VEC1_POL())
            .field("RESERVED15", &self.RESERVED15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VECCFG0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VECCFG0 {{ VEC0_EV: {:?}, VEC0_EN: {:?}, VEC0_POL: {:?}, RESERVED7: {=bool:?}, VEC1_EV: {:?}, VEC1_EN: {:?}, VEC1_POL: {:?}, RESERVED15: {=u32:?} }}",
            self.VEC0_EV(),
            self.VEC0_EN(),
            self.VEC0_POL(),
            self.RESERVED7(),
            self.VEC1_EV(),
            self.VEC1_EN(),
            self.VEC1_POL(),
            self.RESERVED15()
        )
    }
}
#[doc = "Vector Configuration 1 AUX_SCE event vectors 2 and 3 configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VECCFG1(pub u32);
impl VECCFG1 {
    #[doc = "4:0\\] Select vector 2 trigger source event."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC2_EV(&self) -> super::vals::VEC2_EV {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::VEC2_EV::from_bits(val as u8)
    }
    #[doc = "4:0\\] Select vector 2 trigger source event."]
    #[inline(always)]
    pub const fn set_VEC2_EV(&mut self, val: super::vals::VEC2_EV) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "5:5\\] Vector 2 trigger enable. When enabled, VEC2_EV event with VEC2_POL polarity triggers a jump to vector # 2 when AUX_SCE sleeps. Lower vectors (0 and 1) have priority."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC2_EN(&self) -> super::vals::VEC2_EN {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::VEC2_EN::from_bits(val as u8)
    }
    #[doc = "5:5\\] Vector 2 trigger enable. When enabled, VEC2_EV event with VEC2_POL polarity triggers a jump to vector # 2 when AUX_SCE sleeps. Lower vectors (0 and 1) have priority."]
    #[inline(always)]
    pub const fn set_VEC2_EN(&mut self, val: super::vals::VEC2_EN) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Vector 2 trigger event polarity. To manually trigger vector 2 execution: - AUX_SCE must sleep. - Set VEC2_EV to a known static value. - Toggle VEC2_POL twice."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC2_POL(&self) -> super::vals::VEC2_POL {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::VEC2_POL::from_bits(val as u8)
    }
    #[doc = "6:6\\] Vector 2 trigger event polarity. To manually trigger vector 2 execution: - AUX_SCE must sleep. - Set VEC2_EV to a known static value. - Toggle VEC2_POL twice."]
    #[inline(always)]
    pub const fn set_VEC2_POL(&mut self, val: super::vals::VEC2_POL) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "12:8\\] Select vector 3 trigger source event."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC3_EV(&self) -> super::vals::VEC3_EV {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::VEC3_EV::from_bits(val as u8)
    }
    #[doc = "12:8\\] Select vector 3 trigger source event."]
    #[inline(always)]
    pub const fn set_VEC3_EV(&mut self, val: super::vals::VEC3_EV) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "13:13\\] Vector 3 trigger enable. When enabled, VEC3_EV event with VEC3_POL polarity triggers a jump to vector # 3 when AUX_SCE sleeps. Lower vectors (0, 1, and 2) have priority."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC3_EN(&self) -> super::vals::VEC3_EN {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::VEC3_EN::from_bits(val as u8)
    }
    #[doc = "13:13\\] Vector 3 trigger enable. When enabled, VEC3_EV event with VEC3_POL polarity triggers a jump to vector # 3 when AUX_SCE sleeps. Lower vectors (0, 1, and 2) have priority."]
    #[inline(always)]
    pub const fn set_VEC3_EN(&mut self, val: super::vals::VEC3_EN) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Vector 3 trigger event polarity. To manually trigger vector 3 execution: - AUX_SCE must sleep. - Set VEC3_EV to a known static value. - Toggle VEC3_POL twice."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC3_POL(&self) -> super::vals::VEC3_POL {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::VEC3_POL::from_bits(val as u8)
    }
    #[doc = "14:14\\] Vector 3 trigger event polarity. To manually trigger vector 3 execution: - AUX_SCE must sleep. - Set VEC3_EV to a known static value. - Toggle VEC3_POL twice."]
    #[inline(always)]
    pub const fn set_VEC3_POL(&mut self, val: super::vals::VEC3_POL) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "31:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> u32 {
        let val = (self.0 >> 15usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "31:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 15usize)) | (((val as u32) & 0x0001_ffff) << 15usize);
    }
}
impl Default for VECCFG1 {
    #[inline(always)]
    fn default() -> VECCFG1 {
        VECCFG1(0)
    }
}
impl core::fmt::Debug for VECCFG1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VECCFG1")
            .field("VEC2_EV", &self.VEC2_EV())
            .field("VEC2_EN", &self.VEC2_EN())
            .field("VEC2_POL", &self.VEC2_POL())
            .field("RESERVED7", &self.RESERVED7())
            .field("VEC3_EV", &self.VEC3_EV())
            .field("VEC3_EN", &self.VEC3_EN())
            .field("VEC3_POL", &self.VEC3_POL())
            .field("RESERVED15", &self.RESERVED15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VECCFG1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VECCFG1 {{ VEC2_EV: {:?}, VEC2_EN: {:?}, VEC2_POL: {:?}, RESERVED7: {=bool:?}, VEC3_EV: {:?}, VEC3_EN: {:?}, VEC3_POL: {:?}, RESERVED15: {=u32:?} }}",
            self.VEC2_EV(),
            self.VEC2_EN(),
            self.VEC2_POL(),
            self.RESERVED7(),
            self.VEC3_EV(),
            self.VEC3_EN(),
            self.VEC3_POL(),
            self.RESERVED15()
        )
    }
}
#[doc = "Vector Flags If a vector flag becomes 1 and AUX_SCE sleeps, AUX_SCE will wake up and execute the corresponding vector. The vector with the lowest index will execute first if multiple vectors flags are set. AUX_SCE must return to sleep to execute the next vector. During execution of a vector, AUX_SCE must clear the vector flag that triggered execution. Write 1 to bit index n in VECFLAGSCLR to clear vector flag n."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VECFLAGS(pub u32);
impl VECFLAGS {
    #[doc = "0:0\\] Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
    #[inline(always)]
    pub const fn set_VEC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
    #[inline(always)]
    pub const fn set_VEC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
    #[inline(always)]
    pub const fn set_VEC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
    #[inline(always)]
    pub const fn set_VEC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for VECFLAGS {
    #[inline(always)]
    fn default() -> VECFLAGS {
        VECFLAGS(0)
    }
}
impl core::fmt::Debug for VECFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VECFLAGS")
            .field("VEC0", &self.VEC0())
            .field("VEC1", &self.VEC1())
            .field("VEC2", &self.VEC2())
            .field("VEC3", &self.VEC3())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VECFLAGS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VECFLAGS {{ VEC0: {=bool:?}, VEC1: {=bool:?}, VEC2: {=bool:?}, VEC3: {=bool:?}, RESERVED4: {=u32:?} }}",
            self.VEC0(),
            self.VEC1(),
            self.VEC2(),
            self.VEC3(),
            self.RESERVED4()
        )
    }
}
#[doc = "Vector Flags Clear Strobes for clearing flags in VECFLAGS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VECFLAGSCLR(pub u32);
impl VECFLAGSCLR {
    #[doc = "0:0\\] Clear vector flag 0. 0: No effect. 1: Clear VECFLAGS.VEC0. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Clear vector flag 0. 0: No effect. 1: Clear VECFLAGS.VEC0. Read value is 0."]
    #[inline(always)]
    pub const fn set_VEC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Clear vector flag 1. 0: No effect. 1: Clear VECFLAGS.VEC1. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Clear vector flag 1. 0: No effect. 1: Clear VECFLAGS.VEC1. Read value is 0."]
    #[inline(always)]
    pub const fn set_VEC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Clear vector flag 2. 0: No effect. 1: Clear VECFLAGS.VEC2. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Clear vector flag 2. 0: No effect. 1: Clear VECFLAGS.VEC2. Read value is 0."]
    #[inline(always)]
    pub const fn set_VEC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Clear vector flag 3. 0: No effect. 1: Clear VECFLAGS.VEC3. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn VEC3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Clear vector flag 3. 0: No effect. 1: Clear VECFLAGS.VEC3. Read value is 0."]
    #[inline(always)]
    pub const fn set_VEC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for VECFLAGSCLR {
    #[inline(always)]
    fn default() -> VECFLAGSCLR {
        VECFLAGSCLR(0)
    }
}
impl core::fmt::Debug for VECFLAGSCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VECFLAGSCLR")
            .field("VEC0", &self.VEC0())
            .field("VEC1", &self.VEC1())
            .field("VEC2", &self.VEC2())
            .field("VEC3", &self.VEC3())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VECFLAGSCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VECFLAGSCLR {{ VEC0: {=bool:?}, VEC1: {=bool:?}, VEC2: {=bool:?}, VEC3: {=bool:?}, RESERVED4: {=u32:?} }}",
            self.VEC0(),
            self.VEC1(),
            self.VEC2(),
            self.VEC3(),
            self.RESERVED4()
        )
    }
}

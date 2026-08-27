#[doc = "Comparator 0 This register is used to write the reference value for comparator 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP0(pub u32);
impl COMP0 {
    #[doc = "31:0\\] Reference value to compare against PC or the data address as given by FUNCTION0. Comparator 0 can also compare against the value of the PC Sampler Counter (CYCCNT)."]
    #[must_use]
    #[inline(always)]
    pub const fn COMP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Reference value to compare against PC or the data address as given by FUNCTION0. Comparator 0 can also compare against the value of the PC Sampler Counter (CYCCNT)."]
    #[inline(always)]
    pub const fn set_COMP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for COMP0 {
    #[inline(always)]
    fn default() -> COMP0 {
        COMP0(0)
    }
}
impl core::fmt::Debug for COMP0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP0").field("COMP", &self.COMP()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "COMP0 {{ COMP: {=u32:?} }}", self.COMP())
    }
}
#[doc = "Comparator 1 This register is used to write the reference value for comparator 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP1(pub u32);
impl COMP1 {
    #[doc = "31:0\\] Reference value to compare against PC or the data address as given by FUNCTION1. Comparator 1 can also compare data values. So this register can contain reference values for data matching."]
    #[must_use]
    #[inline(always)]
    pub const fn COMP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Reference value to compare against PC or the data address as given by FUNCTION1. Comparator 1 can also compare data values. So this register can contain reference values for data matching."]
    #[inline(always)]
    pub const fn set_COMP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for COMP1 {
    #[inline(always)]
    fn default() -> COMP1 {
        COMP1(0)
    }
}
impl core::fmt::Debug for COMP1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP1").field("COMP", &self.COMP()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "COMP1 {{ COMP: {=u32:?} }}", self.COMP())
    }
}
#[doc = "Comparator 2 This register is used to write the reference value for comparator 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP2(pub u32);
impl COMP2 {
    #[doc = "31:0\\] Reference value to compare against PC or the data address as given by FUNCTION2."]
    #[must_use]
    #[inline(always)]
    pub const fn COMP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Reference value to compare against PC or the data address as given by FUNCTION2."]
    #[inline(always)]
    pub const fn set_COMP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for COMP2 {
    #[inline(always)]
    fn default() -> COMP2 {
        COMP2(0)
    }
}
impl core::fmt::Debug for COMP2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP2").field("COMP", &self.COMP()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "COMP2 {{ COMP: {=u32:?} }}", self.COMP())
    }
}
#[doc = "Comparator 3 This register is used to write the reference value for comparator 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP3(pub u32);
impl COMP3 {
    #[doc = "31:0\\] Reference value to compare against PC or the data address as given by FUNCTION3."]
    #[must_use]
    #[inline(always)]
    pub const fn COMP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Reference value to compare against PC or the data address as given by FUNCTION3."]
    #[inline(always)]
    pub const fn set_COMP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for COMP3 {
    #[inline(always)]
    fn default() -> COMP3 {
        COMP3(0)
    }
}
impl core::fmt::Debug for COMP3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP3").field("COMP", &self.COMP()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "COMP3 {{ COMP: {=u32:?} }}", self.COMP())
    }
}
#[doc = "CPI Count This register is used to count the total number of instruction cycles beyond the first cycle."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPICNT(pub u32);
impl CPICNT {
    #[doc = "7:0\\] Current CPI counter value. Increments on the additional cycles (the first cycle is not counted) required to execute all instructions except those recorded by LSUCNT. This counter also increments on all instruction fetch stalls. If CTRL.CPIEVTENA is set, an event is emitted when the counter overflows. This counter initializes to 0 when it is enabled using CTRL.CPIEVTENA."]
    #[must_use]
    #[inline(always)]
    pub const fn CPICNT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Current CPI counter value. Increments on the additional cycles (the first cycle is not counted) required to execute all instructions except those recorded by LSUCNT. This counter also increments on all instruction fetch stalls. If CTRL.CPIEVTENA is set, an event is emitted when the counter overflows. This counter initializes to 0 when it is enabled using CTRL.CPIEVTENA."]
    #[inline(always)]
    pub const fn set_CPICNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for CPICNT {
    #[inline(always)]
    fn default() -> CPICNT {
        CPICNT(0)
    }
}
impl core::fmt::Debug for CPICNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPICNT")
            .field("CPICNT", &self.CPICNT())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPICNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPICNT {{ CPICNT: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.CPICNT(),
            self.RESERVED8()
        )
    }
}
#[doc = "Control Use the DWT Control Register to enable the DWT unit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "0:0\\] Enable CYCCNT, allowing it to increment and generate synchronization and count events. If NOCYCCNT = 1, this bit reads zero and ignore writes."]
    #[must_use]
    #[inline(always)]
    pub const fn CYCCNTENA(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Enable CYCCNT, allowing it to increment and generate synchronization and count events. If NOCYCCNT = 1, this bit reads zero and ignore writes."]
    #[inline(always)]
    pub const fn set_CYCCNTENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "4:1\\] Reload value for post-scalar counter POSTCNT. When 0, events are triggered on each tap change (a power of 2). If this field has a non-0 value, it forms a count-down value, to be reloaded into POSTCNT each time it reaches 0. For example, a value 1 in this register means an event is formed every other tap change."]
    #[must_use]
    #[inline(always)]
    pub const fn POSTPRESET(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "4:1\\] Reload value for post-scalar counter POSTCNT. When 0, events are triggered on each tap change (a power of 2). If this field has a non-0 value, it forms a count-down value, to be reloaded into POSTCNT each time it reaches 0. For example, a value 1 in this register means an event is formed every other tap change."]
    #[inline(always)]
    pub const fn set_POSTPRESET(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "8:5\\] Post-scalar counter for CYCTAP. When the selected tapped bit changes from 0 to 1 or 1 to 0, the post scalar counter is down-counted when not 0. If 0, it triggers an event for PCSAMPLEENA or CYCEVTENA use. It also reloads with the value from POSTPRESET."]
    #[must_use]
    #[inline(always)]
    pub const fn POSTCNT(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x0f;
        val as u8
    }
    #[doc = "8:5\\] Post-scalar counter for CYCTAP. When the selected tapped bit changes from 0 to 1 or 1 to 0, the post scalar counter is down-counted when not 0. If 0, it triggers an event for PCSAMPLEENA or CYCEVTENA use. It also reloads with the value from POSTPRESET."]
    #[inline(always)]
    pub const fn set_POSTCNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
    }
    #[doc = "9:9\\] Selects a tap on CYCCNT. These are spaced at bits \\[6\\] and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA)."]
    #[must_use]
    #[inline(always)]
    pub const fn CYCTAP(&self) -> super::vals::CYCTAP {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::CYCTAP::from_bits(val as u8)
    }
    #[doc = "9:9\\] Selects a tap on CYCCNT. These are spaced at bits \\[6\\] and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA)."]
    #[inline(always)]
    pub const fn set_CYCTAP(&mut self, val: super::vals::CYCTAP) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "11:10\\] Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0)."]
    #[must_use]
    #[inline(always)]
    pub const fn SYNCTAP(&self) -> super::vals::SYNCTAP {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::SYNCTAP::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0)."]
    #[inline(always)]
    pub const fn set_SYNCTAP(&mut self, val: super::vals::SYNCTAP) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] Enables PC Sampling event. A PC sample event is emitted when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. Enabling this bit overrides CYCEVTENA. 0: PC Sampling event disabled. 1: Sampling event enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn PCSAMPLEENA(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Enables PC Sampling event. A PC sample event is emitted when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. Enabling this bit overrides CYCEVTENA. 0: PC Sampling event disabled. 1: Sampling event enabled."]
    #[inline(always)]
    pub const fn set_PCSAMPLEENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "15:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED13(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "15:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "16:16\\] Enables Interrupt event tracing. 0: Interrupt event trace disabled. 1: Interrupt event trace enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn EXCTRCENA(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Enables Interrupt event tracing. 0: Interrupt event trace disabled. 1: Interrupt event trace enabled."]
    #[inline(always)]
    pub const fn set_EXCTRCENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Enables CPI count event. Emits an event when CPICNT overflows (every 256 cycles of multi-cycle instructions). 0: CPI counter events disabled. 1: CPI counter events enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CPIEVTENA(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Enables CPI count event. Emits an event when CPICNT overflows (every 256 cycles of multi-cycle instructions). 0: CPI counter events disabled. 1: CPI counter events enabled."]
    #[inline(always)]
    pub const fn set_CPIEVTENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Enables Interrupt overhead event. Emits an event when EXCCNT overflows (every 256 cycles of interrupt overhead). 0x0: Interrupt overhead event disabled. 0x1: Interrupt overhead event enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn EXCEVTENA(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Enables Interrupt overhead event. Emits an event when EXCCNT overflows (every 256 cycles of interrupt overhead). 0x0: Interrupt overhead event disabled. 0x1: Interrupt overhead event enabled."]
    #[inline(always)]
    pub const fn set_EXCEVTENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Enables Sleep count event. Emits an event when SLEEPCNT overflows (every 256 cycles that the processor is sleeping). 0: Sleep count events disabled. 1: Sleep count events enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEEPEVTENA(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Enables Sleep count event. Emits an event when SLEEPCNT overflows (every 256 cycles that the processor is sleeping). 0: Sleep count events disabled. 1: Sleep count events enabled."]
    #[inline(always)]
    pub const fn set_SLEEPEVTENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Enables LSU count event. Emits an event when LSUCNT overflows (every 256 cycles of LSU operation). LSU counts include all LSU costs after the initial cycle for the instruction. 0: LSU count events disabled. 1: LSU count events enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn LSUEVTENA(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Enables LSU count event. Emits an event when LSUCNT overflows (every 256 cycles of LSU operation). LSU counts include all LSU costs after the initial cycle for the instruction. 0: LSU count events disabled. 1: LSU count events enabled."]
    #[inline(always)]
    pub const fn set_LSUEVTENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Enables Folded instruction count event. Emits an event when FOLDCNT overflows (every 256 cycles of folded instructions). A folded instruction is one that does not incur even one cycle to execute. For example, an IT instruction is folded away and so does not use up one cycle. 0: Folded instruction count events disabled. 1: Folded instruction count events enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn FOLDEVTENA(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Enables Folded instruction count event. Emits an event when FOLDCNT overflows (every 256 cycles of folded instructions). A folded instruction is one that does not incur even one cycle to execute. For example, an IT instruction is folded away and so does not use up one cycle. 0: Folded instruction count events disabled. 1: Folded instruction count events enabled."]
    #[inline(always)]
    pub const fn set_FOLDEVTENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Enables Cycle count event. Emits an event when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. This event is only emitted if PCSAMPLEENA is disabled. PCSAMPLEENA overrides the setting of this bit. 0: Cycle count events disabled 1: Cycle count events enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CYCEVTENA(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Enables Cycle count event. Emits an event when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. This event is only emitted if PCSAMPLEENA is disabled. PCSAMPLEENA overrides the setting of this bit. 0: Cycle count events disabled 1: Cycle count events enabled."]
    #[inline(always)]
    pub const fn set_CYCEVTENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] When set, FOLDCNT, LSUCNT, SLEEPCNT, EXCCNT, and CPICNT are not supported."]
    #[must_use]
    #[inline(always)]
    pub const fn NOPRFCNT(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] When set, FOLDCNT, LSUCNT, SLEEPCNT, EXCCNT, and CPICNT are not supported."]
    #[inline(always)]
    pub const fn set_NOPRFCNT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] When set, CYCCNT is not supported."]
    #[must_use]
    #[inline(always)]
    pub const fn NOCYCCNT(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] When set, CYCCNT is not supported."]
    #[inline(always)]
    pub const fn set_NOCYCCNT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "31:26\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED26(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "31:26\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED26(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for CTRL {
    #[inline(always)]
    fn default() -> CTRL {
        CTRL(0)
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("CYCCNTENA", &self.CYCCNTENA())
            .field("POSTPRESET", &self.POSTPRESET())
            .field("POSTCNT", &self.POSTCNT())
            .field("CYCTAP", &self.CYCTAP())
            .field("SYNCTAP", &self.SYNCTAP())
            .field("PCSAMPLEENA", &self.PCSAMPLEENA())
            .field("RESERVED13", &self.RESERVED13())
            .field("EXCTRCENA", &self.EXCTRCENA())
            .field("CPIEVTENA", &self.CPIEVTENA())
            .field("EXCEVTENA", &self.EXCEVTENA())
            .field("SLEEPEVTENA", &self.SLEEPEVTENA())
            .field("LSUEVTENA", &self.LSUEVTENA())
            .field("FOLDEVTENA", &self.FOLDEVTENA())
            .field("CYCEVTENA", &self.CYCEVTENA())
            .field("RESERVED23", &self.RESERVED23())
            .field("NOPRFCNT", &self.NOPRFCNT())
            .field("NOCYCCNT", &self.NOCYCCNT())
            .field("RESERVED26", &self.RESERVED26())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ CYCCNTENA: {=bool:?}, POSTPRESET: {=u8:?}, POSTCNT: {=u8:?}, CYCTAP: {:?}, SYNCTAP: {:?}, PCSAMPLEENA: {=bool:?}, RESERVED13: {=u8:?}, EXCTRCENA: {=bool:?}, CPIEVTENA: {=bool:?}, EXCEVTENA: {=bool:?}, SLEEPEVTENA: {=bool:?}, LSUEVTENA: {=bool:?}, FOLDEVTENA: {=bool:?}, CYCEVTENA: {=bool:?}, RESERVED23: {=bool:?}, NOPRFCNT: {=bool:?}, NOCYCCNT: {=bool:?}, RESERVED26: {=u8:?} }}",
            self.CYCCNTENA(),
            self.POSTPRESET(),
            self.POSTCNT(),
            self.CYCTAP(),
            self.SYNCTAP(),
            self.PCSAMPLEENA(),
            self.RESERVED13(),
            self.EXCTRCENA(),
            self.CPIEVTENA(),
            self.EXCEVTENA(),
            self.SLEEPEVTENA(),
            self.LSUEVTENA(),
            self.FOLDEVTENA(),
            self.CYCEVTENA(),
            self.RESERVED23(),
            self.NOPRFCNT(),
            self.NOCYCCNT(),
            self.RESERVED26()
        )
    }
}
#[doc = "Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CYCCNT(pub u32);
impl CYCCNT {
    #[doc = "31:0\\] Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
    #[must_use]
    #[inline(always)]
    pub const fn CYCCNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
    #[inline(always)]
    pub const fn set_CYCCNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CYCCNT {
    #[inline(always)]
    fn default() -> CYCCNT {
        CYCCNT(0)
    }
}
impl core::fmt::Debug for CYCCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CYCCNT")
            .field("CYCCNT", &self.CYCCNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CYCCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CYCCNT {{ CYCCNT: {=u32:?} }}", self.CYCCNT())
    }
}
#[doc = "Exception Overhead Count This register is used to count the total cycles spent in interrupt processing."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EXCCNT(pub u32);
impl EXCCNT {
    #[doc = "7:0\\] Current interrupt overhead counter value. Counts the total cycles spent in interrupt processing (for example entry stacking, return unstacking, pre-emption). An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.EXCEVTENA."]
    #[must_use]
    #[inline(always)]
    pub const fn EXCCNT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Current interrupt overhead counter value. Counts the total cycles spent in interrupt processing (for example entry stacking, return unstacking, pre-emption). An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.EXCEVTENA."]
    #[inline(always)]
    pub const fn set_EXCCNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for EXCCNT {
    #[inline(always)]
    fn default() -> EXCCNT {
        EXCCNT(0)
    }
}
impl core::fmt::Debug for EXCCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXCCNT")
            .field("EXCCNT", &self.EXCCNT())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EXCCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EXCCNT {{ EXCCNT: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.EXCCNT(),
            self.RESERVED8()
        )
    }
}
#[doc = "Fold Count This register is used to count the total number of folded instructions. The counter increments on each instruction which takes 0 cycles."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FOLDCNT(pub u32);
impl FOLDCNT {
    #[doc = "7:0\\] This counts the total number folded instructions. This counter initializes to 0 when it is enabled using CTRL.FOLDEVTENA."]
    #[must_use]
    #[inline(always)]
    pub const fn FOLDCNT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] This counts the total number folded instructions. This counter initializes to 0 when it is enabled using CTRL.FOLDEVTENA."]
    #[inline(always)]
    pub const fn set_FOLDCNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for FOLDCNT {
    #[inline(always)]
    fn default() -> FOLDCNT {
        FOLDCNT(0)
    }
}
impl core::fmt::Debug for FOLDCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FOLDCNT")
            .field("FOLDCNT", &self.FOLDCNT())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FOLDCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FOLDCNT {{ FOLDCNT: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.FOLDCNT(),
            self.RESERVED8()
        )
    }
}
#[doc = "Function 0 Use the DWT Function Registers 0 to control the operation of the comparator 0. This comparator can: 1. Match against either the PC or the data address. This is controlled by CYCMATCH. This function is only available for comparator 0 (COMP0). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FUNCTION0(pub u32);
impl FUNCTION0 {
    #[doc = "3:0\\] Function settings. 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNCTION(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Function settings. 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[inline(always)]
    pub const fn set_FUNCTION(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[must_use]
    #[inline(always)]
    pub const fn EMITRANGE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[inline(always)]
    pub const fn set_EMITRANGE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] This bit is only available in comparator 0. When set, COMP0 will compare against the cycle counter (CYCCNT)."]
    #[must_use]
    #[inline(always)]
    pub const fn CYCMATCH(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] This bit is only available in comparator 0. When set, COMP0 will compare against the cycle counter (CYCCNT)."]
    #[inline(always)]
    pub const fn set_CYCMATCH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "23:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0xffff;
        val as u16
    }
    #[doc = "23:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 8usize)) | (((val as u32) & 0xffff) << 8usize);
    }
    #[doc = "24:24\\] This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHED(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[inline(always)]
    pub const fn set_MATCHED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED25(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for FUNCTION0 {
    #[inline(always)]
    fn default() -> FUNCTION0 {
        FUNCTION0(0)
    }
}
impl core::fmt::Debug for FUNCTION0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNCTION0")
            .field("FUNCTION", &self.FUNCTION())
            .field("RESERVED4", &self.RESERVED4())
            .field("EMITRANGE", &self.EMITRANGE())
            .field("RESERVED6", &self.RESERVED6())
            .field("CYCMATCH", &self.CYCMATCH())
            .field("RESERVED8", &self.RESERVED8())
            .field("MATCHED", &self.MATCHED())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FUNCTION0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FUNCTION0 {{ FUNCTION: {=u8:?}, RESERVED4: {=bool:?}, EMITRANGE: {=bool:?}, RESERVED6: {=bool:?}, CYCMATCH: {=bool:?}, RESERVED8: {=u16:?}, MATCHED: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.FUNCTION(),
            self.RESERVED4(),
            self.EMITRANGE(),
            self.RESERVED6(),
            self.CYCMATCH(),
            self.RESERVED8(),
            self.MATCHED(),
            self.RESERVED25()
        )
    }
}
#[doc = "Function 1 Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can: 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FUNCTION1(pub u32);
impl FUNCTION1 {
    #[doc = "3:0\\] Function settings: 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: FUNCTION is overridden for comparators given by DATAVADDR0 and DATAVADDR1 if DATAVMATCH is also set. The comparators given by DATAVADDR0 and DATAVADDR1 can then only perform address comparator matches for comparator 1 data matches. Note 4: If the data matching functionality is not included during implementation it is not possible to set DATAVADDR0, DATAVADDR1, or DATAVMATCH. This means that the data matching functionality is not available in the implementation. Test the availability of data matching by writing and reading DATAVMATCH. If it is not settable then data matching is unavailable. Note 5: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNCTION(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Function settings: 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: FUNCTION is overridden for comparators given by DATAVADDR0 and DATAVADDR1 if DATAVMATCH is also set. The comparators given by DATAVADDR0 and DATAVADDR1 can then only perform address comparator matches for comparator 1 data matches. Note 4: If the data matching functionality is not included during implementation it is not possible to set DATAVADDR0, DATAVADDR1, or DATAVMATCH. This means that the data matching functionality is not available in the implementation. Test the availability of data matching by writing and reading DATAVMATCH. If it is not settable then data matching is unavailable. Note 5: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[inline(always)]
    pub const fn set_FUNCTION(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[must_use]
    #[inline(always)]
    pub const fn EMITRANGE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[inline(always)]
    pub const fn set_EMITRANGE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "8:8\\] Data match feature: 0: Perform address comparison 1: Perform data value compare. The comparators given by DATAVADDR0 and DATAVADDR1 provide the address for the data comparison. The FUNCTION setting for the comparators given by DATAVADDR0 and DATAVADDR1 are overridden and those comparators only provide the address match for the data comparison. This bit is only available in comparator 1."]
    #[must_use]
    #[inline(always)]
    pub const fn DATAVMATCH(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Data match feature: 0: Perform address comparison 1: Perform data value compare. The comparators given by DATAVADDR0 and DATAVADDR1 provide the address for the data comparison. The FUNCTION setting for the comparators given by DATAVADDR0 and DATAVADDR1 are overridden and those comparators only provide the address match for the data comparison. This bit is only available in comparator 1."]
    #[inline(always)]
    pub const fn set_DATAVMATCH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Read only bit-field only supported in comparator 1. 0: DATAVADDR1 not supported 1: DATAVADDR1 supported (enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn LNK1ENA(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Read only bit-field only supported in comparator 1. 0: DATAVADDR1 not supported 1: DATAVADDR1 supported (enabled)."]
    #[inline(always)]
    pub const fn set_LNK1ENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "11:10\\] Defines the size of the data in the COMP1 register that is to be matched: 0x0: Byte 0x1: Halfword 0x2: Word 0x3: Unpredictable."]
    #[must_use]
    #[inline(always)]
    pub const fn DATAVSIZE(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "11:10\\] Defines the size of the data in the COMP1 register that is to be matched: 0x0: Byte 0x1: Halfword 0x2: Word 0x3: Unpredictable."]
    #[inline(always)]
    pub const fn set_DATAVSIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "15:12\\] Identity of a linked address comparator for data value matching when DATAVMATCH == 1."]
    #[must_use]
    #[inline(always)]
    pub const fn DATAVADDR0(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Identity of a linked address comparator for data value matching when DATAVMATCH == 1."]
    #[inline(always)]
    pub const fn set_DATAVADDR0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "19:16\\] Identity of a second linked address comparator for data value matching when DATAVMATCH == 1 and LNK1ENA == 1."]
    #[must_use]
    #[inline(always)]
    pub const fn DATAVADDR1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Identity of a second linked address comparator for data value matching when DATAVMATCH == 1 and LNK1ENA == 1."]
    #[inline(always)]
    pub const fn set_DATAVADDR1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "23:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED20(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED20(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "24:24\\] This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHED(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[inline(always)]
    pub const fn set_MATCHED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED25(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for FUNCTION1 {
    #[inline(always)]
    fn default() -> FUNCTION1 {
        FUNCTION1(0)
    }
}
impl core::fmt::Debug for FUNCTION1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNCTION1")
            .field("FUNCTION", &self.FUNCTION())
            .field("RESERVED4", &self.RESERVED4())
            .field("EMITRANGE", &self.EMITRANGE())
            .field("RESERVED6", &self.RESERVED6())
            .field("DATAVMATCH", &self.DATAVMATCH())
            .field("LNK1ENA", &self.LNK1ENA())
            .field("DATAVSIZE", &self.DATAVSIZE())
            .field("DATAVADDR0", &self.DATAVADDR0())
            .field("DATAVADDR1", &self.DATAVADDR1())
            .field("RESERVED20", &self.RESERVED20())
            .field("MATCHED", &self.MATCHED())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FUNCTION1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FUNCTION1 {{ FUNCTION: {=u8:?}, RESERVED4: {=bool:?}, EMITRANGE: {=bool:?}, RESERVED6: {=u8:?}, DATAVMATCH: {=bool:?}, LNK1ENA: {=bool:?}, DATAVSIZE: {=u8:?}, DATAVADDR0: {=u8:?}, DATAVADDR1: {=u8:?}, RESERVED20: {=u8:?}, MATCHED: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.FUNCTION(),
            self.RESERVED4(),
            self.EMITRANGE(),
            self.RESERVED6(),
            self.DATAVMATCH(),
            self.LNK1ENA(),
            self.DATAVSIZE(),
            self.DATAVADDR0(),
            self.DATAVADDR1(),
            self.RESERVED20(),
            self.MATCHED(),
            self.RESERVED25()
        )
    }
}
#[doc = "Function 2 Use the DWT Function Registers 2 to control the operation of the comparator 2. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FUNCTION2(pub u32);
impl FUNCTION2 {
    #[doc = "3:0\\] Function settings. 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNCTION(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Function settings. 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[inline(always)]
    pub const fn set_FUNCTION(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[must_use]
    #[inline(always)]
    pub const fn EMITRANGE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[inline(always)]
    pub const fn set_EMITRANGE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "23:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "23:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 6usize)) | (((val as u32) & 0x0003_ffff) << 6usize);
    }
    #[doc = "24:24\\] This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHED(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[inline(always)]
    pub const fn set_MATCHED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED25(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for FUNCTION2 {
    #[inline(always)]
    fn default() -> FUNCTION2 {
        FUNCTION2(0)
    }
}
impl core::fmt::Debug for FUNCTION2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNCTION2")
            .field("FUNCTION", &self.FUNCTION())
            .field("RESERVED4", &self.RESERVED4())
            .field("EMITRANGE", &self.EMITRANGE())
            .field("RESERVED6", &self.RESERVED6())
            .field("MATCHED", &self.MATCHED())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FUNCTION2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FUNCTION2 {{ FUNCTION: {=u8:?}, RESERVED4: {=bool:?}, EMITRANGE: {=bool:?}, RESERVED6: {=u32:?}, MATCHED: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.FUNCTION(),
            self.RESERVED4(),
            self.EMITRANGE(),
            self.RESERVED6(),
            self.MATCHED(),
            self.RESERVED25()
        )
    }
}
#[doc = "Function 3 Use the DWT Function Registers 3 to control the operation of the comparator 3. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FUNCTION3(pub u32);
impl FUNCTION3 {
    #[doc = "3:0\\] Function settings. 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNCTION(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Function settings. 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[inline(always)]
    pub const fn set_FUNCTION(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[must_use]
    #[inline(always)]
    pub const fn EMITRANGE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[inline(always)]
    pub const fn set_EMITRANGE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "23:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "23:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 6usize)) | (((val as u32) & 0x0003_ffff) << 6usize);
    }
    #[doc = "24:24\\] This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHED(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[inline(always)]
    pub const fn set_MATCHED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED25(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for FUNCTION3 {
    #[inline(always)]
    fn default() -> FUNCTION3 {
        FUNCTION3(0)
    }
}
impl core::fmt::Debug for FUNCTION3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNCTION3")
            .field("FUNCTION", &self.FUNCTION())
            .field("RESERVED4", &self.RESERVED4())
            .field("EMITRANGE", &self.EMITRANGE())
            .field("RESERVED6", &self.RESERVED6())
            .field("MATCHED", &self.MATCHED())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FUNCTION3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FUNCTION3 {{ FUNCTION: {=u8:?}, RESERVED4: {=bool:?}, EMITRANGE: {=bool:?}, RESERVED6: {=u32:?}, MATCHED: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.FUNCTION(),
            self.RESERVED4(),
            self.EMITRANGE(),
            self.RESERVED6(),
            self.MATCHED(),
            self.RESERVED25()
        )
    }
}
#[doc = "LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LSUCNT(pub u32);
impl LSUCNT {
    #[doc = "7:0\\] LSU counter. This counts the total number of cycles that the processor is processing an LSU operation. The initial execution cost of the instruction is not counted. For example, an LDR that takes two cycles to complete increments this counter one cycle. Equivalently, an LDR that stalls for two cycles (i.e. takes four cycles to execute), increments this counter three times. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.LSUEVTENA."]
    #[must_use]
    #[inline(always)]
    pub const fn LSUCNT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] LSU counter. This counts the total number of cycles that the processor is processing an LSU operation. The initial execution cost of the instruction is not counted. For example, an LDR that takes two cycles to complete increments this counter one cycle. Equivalently, an LDR that stalls for two cycles (i.e. takes four cycles to execute), increments this counter three times. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.LSUEVTENA."]
    #[inline(always)]
    pub const fn set_LSUCNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for LSUCNT {
    #[inline(always)]
    fn default() -> LSUCNT {
        LSUCNT(0)
    }
}
impl core::fmt::Debug for LSUCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LSUCNT")
            .field("LSUCNT", &self.LSUCNT())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LSUCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LSUCNT {{ LSUCNT: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.LSUCNT(),
            self.RESERVED8()
        )
    }
}
#[doc = "Mask 0 Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MASK0(pub u32);
impl MASK0 {
    #[doc = "3:0\\] Mask on data address when matching against COMP0. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP0. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP0 is 3, this matches a word access of 0, because 3 would be within the word."]
    #[must_use]
    #[inline(always)]
    pub const fn MASK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Mask on data address when matching against COMP0. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP0. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP0 is 3, this matches a word access of 0, because 3 would be within the word."]
    #[inline(always)]
    pub const fn set_MASK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
impl Default for MASK0 {
    #[inline(always)]
    fn default() -> MASK0 {
        MASK0(0)
    }
}
impl core::fmt::Debug for MASK0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK0")
            .field("MASK", &self.MASK())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MASK0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MASK0 {{ MASK: {=u8:?}, RESERVED4: {=u32:?} }}",
            self.MASK(),
            self.RESERVED4()
        )
    }
}
#[doc = "Mask 1 Use the DWT Mask Registers 1 to apply a mask to data addresses when matching against COMP1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MASK1(pub u32);
impl MASK1 {
    #[doc = "3:0\\] Mask on data address when matching against COMP1. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP1. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP1 is 3, this matches a word access of 0, because 3 would be within the word."]
    #[must_use]
    #[inline(always)]
    pub const fn MASK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Mask on data address when matching against COMP1. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP1. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP1 is 3, this matches a word access of 0, because 3 would be within the word."]
    #[inline(always)]
    pub const fn set_MASK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
impl Default for MASK1 {
    #[inline(always)]
    fn default() -> MASK1 {
        MASK1(0)
    }
}
impl core::fmt::Debug for MASK1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK1")
            .field("MASK", &self.MASK())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MASK1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MASK1 {{ MASK: {=u8:?}, RESERVED4: {=u32:?} }}",
            self.MASK(),
            self.RESERVED4()
        )
    }
}
#[doc = "Mask 2 Use the DWT Mask Registers 2 to apply a mask to data addresses when matching against COMP2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MASK2(pub u32);
impl MASK2 {
    #[doc = "3:0\\] Mask on data address when matching against COMP2. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP2. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP2 is 3, this matches a word access of 0, because 3 would be within the word."]
    #[must_use]
    #[inline(always)]
    pub const fn MASK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Mask on data address when matching against COMP2. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP2. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP2 is 3, this matches a word access of 0, because 3 would be within the word."]
    #[inline(always)]
    pub const fn set_MASK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
impl Default for MASK2 {
    #[inline(always)]
    fn default() -> MASK2 {
        MASK2(0)
    }
}
impl core::fmt::Debug for MASK2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK2")
            .field("MASK", &self.MASK())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MASK2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MASK2 {{ MASK: {=u8:?}, RESERVED4: {=u32:?} }}",
            self.MASK(),
            self.RESERVED4()
        )
    }
}
#[doc = "Mask 3 Use the DWT Mask Registers 3 to apply a mask to data addresses when matching against COMP3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MASK3(pub u32);
impl MASK3 {
    #[doc = "3:0\\] Mask on data address when matching against COMP3. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP3. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP3 is 3, this matches a word access of 0, because 3 would be within the word."]
    #[must_use]
    #[inline(always)]
    pub const fn MASK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Mask on data address when matching against COMP3. This is the size of the ignore mask. That is, DWT matching is performed as:(ADDR ANDed with (0xFFFF left bit-shifted by MASK)) == COMP3. However, the actual comparison is slightly more complex to enable matching an address wherever it appears on a bus. So, if COMP3 is 3, this matches a word access of 0, because 3 would be within the word."]
    #[inline(always)]
    pub const fn set_MASK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
impl Default for MASK3 {
    #[inline(always)]
    fn default() -> MASK3 {
        MASK3(0)
    }
}
impl core::fmt::Debug for MASK3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK3")
            .field("MASK", &self.MASK())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MASK3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MASK3 {{ MASK: {=u8:?}, RESERVED4: {=u32:?} }}",
            self.MASK(),
            self.RESERVED4()
        )
    }
}
#[doc = "Program Counter Sample This register is used to enable coarse-grained software profiling using a debug agent, without changing the currently executing code. If the core is not in debug state, the value returned is the instruction address of a recently executed instruction. If the core is in debug state, the value returned is 0xFFFFFFFF."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PCSR(pub u32);
impl PCSR {
    #[doc = "31:0\\] Execution instruction address sample, or 0xFFFFFFFF if the core is halted."]
    #[must_use]
    #[inline(always)]
    pub const fn EIASAMPLE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Execution instruction address sample, or 0xFFFFFFFF if the core is halted."]
    #[inline(always)]
    pub const fn set_EIASAMPLE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PCSR {
    #[inline(always)]
    fn default() -> PCSR {
        PCSR(0)
    }
}
impl core::fmt::Debug for PCSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCSR")
            .field("EIASAMPLE", &self.EIASAMPLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PCSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PCSR {{ EIASAMPLE: {=u32:?} }}", self.EIASAMPLE())
    }
}
#[doc = "Sleep Count This register is used to count the total number of cycles during which the processor is sleeping."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SLEEPCNT(pub u32);
impl SLEEPCNT {
    #[doc = "7:0\\] Sleep counter. Counts the number of cycles during which the processor is sleeping. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.SLEEPEVTENA. Note that the sleep counter is clocked using CPU's free-running clock. In some power modes the free-running clock to CPU is gated to minimize power consumption. This means that the sleep counter will be invalid in these power modes."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEEPCNT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Sleep counter. Counts the number of cycles during which the processor is sleeping. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.SLEEPEVTENA. Note that the sleep counter is clocked using CPU's free-running clock. In some power modes the free-running clock to CPU is gated to minimize power consumption. This means that the sleep counter will be invalid in these power modes."]
    #[inline(always)]
    pub const fn set_SLEEPCNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for SLEEPCNT {
    #[inline(always)]
    fn default() -> SLEEPCNT {
        SLEEPCNT(0)
    }
}
impl core::fmt::Debug for SLEEPCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLEEPCNT")
            .field("SLEEPCNT", &self.SLEEPCNT())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SLEEPCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SLEEPCNT {{ SLEEPCNT: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.SLEEPCNT(),
            self.RESERVED8()
        )
    }
}

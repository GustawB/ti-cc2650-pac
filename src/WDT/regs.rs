#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTL(pub u32);
impl CTL {
    #[doc = "0:0\\] WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset."]
    #[must_use]
    #[inline(always)]
    pub const fn INTEN(&self) -> super::vals::INTEN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::INTEN::from_bits(val as u8)
    }
    #[doc = "0:0\\] WDT Interrupt Enable 0: Interrupt event disabled. 1: Interrupt event enabled. Once set, this bit can only be cleared by a hardware reset."]
    #[inline(always)]
    pub const fn set_INTEN(&mut self, val: super::vals::INTEN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output."]
    #[must_use]
    #[inline(always)]
    pub const fn RESEN(&self) -> super::vals::RESEN {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RESEN::from_bits(val as u8)
    }
    #[doc = "1:1\\] WDT Reset Enable. Defines the function of the WDT reset source (see PRCM:WARMRESET.WDT_STAT if enabled) 0: Disabled. 1: Enable the Watchdog reset output."]
    #[inline(always)]
    pub const fn set_RESEN(&mut self, val: super::vals::RESEN) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn INTTYPE(&self) -> super::vals::INTTYPE {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::INTTYPE::from_bits(val as u8)
    }
    #[doc = "2:2\\] WDT Interrupt Type 0: WDT interrupt is a standard interrupt. 1: WDT interrupt is a non-maskable interrupt."]
    #[inline(always)]
    pub const fn set_INTTYPE(&mut self, val: super::vals::INTTYPE) {
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
impl Default for CTL {
    #[inline(always)]
    fn default() -> CTL {
        CTL(0)
    }
}
impl core::fmt::Debug for CTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL")
            .field("INTEN", &self.INTEN())
            .field("RESEN", &self.RESEN())
            .field("INTTYPE", &self.INTTYPE())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTL {{ INTEN: {:?}, RESEN: {:?}, INTTYPE: {:?}, RESERVED3: {=u32:?} }}",
            self.INTEN(),
            self.RESEN(),
            self.INTTYPE(),
            self.RESERVED3()
        )
    }
}
#[doc = "Interrupt Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ICR(pub u32);
impl ICR {
    #[doc = "31:0\\] This register is the interrupt clear register. A write of any value to this register clears the WDT interrupt and reloads the 32-bit counter from the LOAD register."]
    #[must_use]
    #[inline(always)]
    pub const fn WDTICR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] This register is the interrupt clear register. A write of any value to this register clears the WDT interrupt and reloads the 32-bit counter from the LOAD register."]
    #[inline(always)]
    pub const fn set_WDTICR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ICR {
    #[inline(always)]
    fn default() -> ICR {
        ICR(0)
    }
}
impl core::fmt::Debug for ICR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("WDTICR", &self.WDTICR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ICR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ICR {{ WDTICR: {=u32:?} }}", self.WDTICR())
    }
}
#[doc = "Interrupt Cause Test Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INT_CAUS(pub u32);
impl INT_CAUS {
    #[doc = "0:0\\] Replica of RIS.WDTRIS."]
    #[must_use]
    #[inline(always)]
    pub const fn CAUSE_INTR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Replica of RIS.WDTRIS."]
    #[inline(always)]
    pub const fn set_CAUSE_INTR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Indicates that the cause of an interrupt was a reset generated but blocked due to TEST.TEST_EN (only possible when TEST.TEST_EN is set)."]
    #[must_use]
    #[inline(always)]
    pub const fn CAUSE_RESET(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Indicates that the cause of an interrupt was a reset generated but blocked due to TEST.TEST_EN (only possible when TEST.TEST_EN is set)."]
    #[inline(always)]
    pub const fn set_CAUSE_RESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for INT_CAUS {
    #[inline(always)]
    fn default() -> INT_CAUS {
        INT_CAUS(0)
    }
}
impl core::fmt::Debug for INT_CAUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CAUS")
            .field("CAUSE_INTR", &self.CAUSE_INTR())
            .field("CAUSE_RESET", &self.CAUSE_RESET())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INT_CAUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INT_CAUS {{ CAUSE_INTR: {=bool:?}, CAUSE_RESET: {=bool:?}, RESERVED2: {=u32:?} }}",
            self.CAUSE_INTR(),
            self.CAUSE_RESET(),
            self.RESERVED2()
        )
    }
}
#[doc = "Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LOAD(pub u32);
impl LOAD {
    #[doc = "31:0\\] This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, an interrupt is immediately generated."]
    #[must_use]
    #[inline(always)]
    pub const fn WDTLOAD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] This register is the 32-bit interval value used by the 32-bit counter. When this register is written, the value is immediately loaded and the counter is restarted to count down from the new value. If this register is loaded with 0x0000.0000, an interrupt is immediately generated."]
    #[inline(always)]
    pub const fn set_WDTLOAD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LOAD {
    #[inline(always)]
    fn default() -> LOAD {
        LOAD(0)
    }
}
impl core::fmt::Debug for LOAD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOAD")
            .field("WDTLOAD", &self.WDTLOAD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LOAD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LOAD {{ WDTLOAD: {=u32:?} }}", self.WDTLOAD())
    }
}
#[doc = "Lock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LOCK(pub u32);
impl LOCK {
    #[doc = "31:0\\] WDT Lock: A write of the value 0x1ACC.E551 unlocks the watchdog registers for write access. A write of any other value reapplies the lock, preventing any register updates (NOTE: TEST.TEST_EN bit is not lockable). A read of this register returns the following values: 0x0000.0000: Unlocked 0x0000.0001: Locked."]
    #[must_use]
    #[inline(always)]
    pub const fn WDTLOCK(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] WDT Lock: A write of the value 0x1ACC.E551 unlocks the watchdog registers for write access. A write of any other value reapplies the lock, preventing any register updates (NOTE: TEST.TEST_EN bit is not lockable). A read of this register returns the following values: 0x0000.0000: Unlocked 0x0000.0001: Locked."]
    #[inline(always)]
    pub const fn set_WDTLOCK(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LOCK {
    #[inline(always)]
    fn default() -> LOCK {
        LOCK(0)
    }
}
impl core::fmt::Debug for LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCK")
            .field("WDTLOCK", &self.WDTLOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LOCK {{ WDTLOCK: {=u32:?} }}", self.WDTLOCK())
    }
}
#[doc = "Masked Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MIS(pub u32);
impl MIS {
    #[doc = "0:0\\] This register is the masked interrupt status register. The value of this register is the logical AND of the raw interrupt bit and the WDT interrupt enable bit CTL.INTEN. Value Description 0: The WDT has not timed out or is masked. 1: An unmasked WDT time-out event has occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn WDTMIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This register is the masked interrupt status register. The value of this register is the logical AND of the raw interrupt bit and the WDT interrupt enable bit CTL.INTEN. Value Description 0: The WDT has not timed out or is masked. 1: An unmasked WDT time-out event has occurred."]
    #[inline(always)]
    pub const fn set_WDTMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for MIS {
    #[inline(always)]
    fn default() -> MIS {
        MIS(0)
    }
}
impl core::fmt::Debug for MIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIS")
            .field("WDTMIS", &self.WDTMIS())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MIS {{ WDTMIS: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.WDTMIS(),
            self.RESERVED1()
        )
    }
}
#[doc = "Raw Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RIS(pub u32);
impl RIS {
    #[doc = "0:0\\] This register is the raw interrupt status register. WDT interrupt events can be monitored via this register if the controller interrupt is masked. Value Description 0: The WDT has not timed out 1: A WDT time-out event has occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn WDTRIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This register is the raw interrupt status register. WDT interrupt events can be monitored via this register if the controller interrupt is masked. Value Description 0: The WDT has not timed out 1: A WDT time-out event has occurred."]
    #[inline(always)]
    pub const fn set_WDTRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for RIS {
    #[inline(always)]
    fn default() -> RIS {
        RIS(0)
    }
}
impl core::fmt::Debug for RIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("WDTRIS", &self.WDTRIS())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RIS {{ WDTRIS: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.WDTRIS(),
            self.RESERVED1()
        )
    }
}
#[doc = "Test Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TEST(pub u32);
impl TEST {
    #[doc = "0:0\\] The test enable bit 0: Enable external reset 1: Disables the generation of an external reset. Instead bit 1 of the INT_CAUS register is set and an interrupt is generated."]
    #[must_use]
    #[inline(always)]
    pub const fn TEST_EN(&self) -> super::vals::TEST_EN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TEST_EN::from_bits(val as u8)
    }
    #[doc = "0:0\\] The test enable bit 0: Enable external reset 1: Disables the generation of an external reset. Instead bit 1 of the INT_CAUS register is set and an interrupt is generated."]
    #[inline(always)]
    pub const fn set_TEST_EN(&mut self, val: super::vals::TEST_EN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "8:8\\] WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
    #[must_use]
    #[inline(always)]
    pub const fn STALL(&self) -> super::vals::STALL {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::STALL::from_bits(val as u8)
    }
    #[doc = "8:8\\] WDT Stall Enable 0: The WDT timer continues counting if the CPU is stopped with a debugger. 1: If the CPU is stopped with a debugger, the WDT stops counting. Once the CPU is restarted, the WDT resumes counting."]
    #[inline(always)]
    pub const fn set_STALL(&mut self, val: super::vals::STALL) {
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
impl Default for TEST {
    #[inline(always)]
    fn default() -> TEST {
        TEST(0)
    }
}
impl core::fmt::Debug for TEST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST")
            .field("TEST_EN", &self.TEST_EN())
            .field("RESERVED1", &self.RESERVED1())
            .field("STALL", &self.STALL())
            .field("RESERVED9", &self.RESERVED9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TEST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TEST {{ TEST_EN: {:?}, RESERVED1: {=u8:?}, STALL: {:?}, RESERVED9: {=u32:?} }}",
            self.TEST_EN(),
            self.RESERVED1(),
            self.STALL(),
            self.RESERVED9()
        )
    }
}
#[doc = "Current Count Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VALUE(pub u32);
impl VALUE {
    #[doc = "31:0\\] This register contains the current count value of the timer."]
    #[must_use]
    #[inline(always)]
    pub const fn WDTVALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] This register contains the current count value of the timer."]
    #[inline(always)]
    pub const fn set_WDTVALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for VALUE {
    #[inline(always)]
    fn default() -> VALUE {
        VALUE(0)
    }
}
impl core::fmt::Debug for VALUE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VALUE")
            .field("WDTVALUE", &self.WDTVALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VALUE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "VALUE {{ WDTVALUE: {=u32:?} }}", self.WDTVALUE())
    }
}

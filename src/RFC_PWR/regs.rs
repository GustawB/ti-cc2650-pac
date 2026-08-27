#[doc = "RF Core Power Management and Clock Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWMCLKEN(pub u32);
impl PWMCLKEN {
    #[doc = "0:0\\] Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
    #[must_use]
    #[inline(always)]
    pub const fn RFC(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Enable essential clocks for the RF Core interface. This includes the interconnect, the radio doorbell DBELL command interface, the power management (PWR) clock control module, and bus clock (sclk) for the CPE. To remove possibility of locking yourself out from the RF Core, this bit can not be cleared. If you need to disable all clocks to the RF Core, see the PRCM:RFCCLKG.CLK_EN register."]
    #[inline(always)]
    pub const fn set_RFC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
    #[must_use]
    #[inline(always)]
    pub const fn CPE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Enable processor clock (hclk) to the Command and Packet Engine (CPE). As part of RF Core initialization, set this bit together with CPERAM bit to enable CPE to boot."]
    #[inline(always)]
    pub const fn set_CPE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
    #[must_use]
    #[inline(always)]
    pub const fn CPERAM(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Enable clock to the Command and Packet Engine (CPE) RAM module. As part of RF Core initialization, set this bit together with CPE bit to enable CPE to boot."]
    #[inline(always)]
    pub const fn set_CPERAM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Enable clock to the Modem (MDM) module."]
    #[must_use]
    #[inline(always)]
    pub const fn MDM(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Enable clock to the Modem (MDM) module."]
    #[inline(always)]
    pub const fn set_MDM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Enable clock to the Modem RAM module."]
    #[must_use]
    #[inline(always)]
    pub const fn MDMRAM(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Enable clock to the Modem RAM module."]
    #[inline(always)]
    pub const fn set_MDMRAM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Enable clock to the RF Engine (RFE) module."]
    #[must_use]
    #[inline(always)]
    pub const fn RFE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Enable clock to the RF Engine (RFE) module."]
    #[inline(always)]
    pub const fn set_RFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Enable clock to the RF Engine RAM module."]
    #[must_use]
    #[inline(always)]
    pub const fn RFERAM(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Enable clock to the RF Engine RAM module."]
    #[inline(always)]
    pub const fn set_RFERAM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Enable clock to the Radio Timer (RAT) module."]
    #[must_use]
    #[inline(always)]
    pub const fn RAT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Enable clock to the Radio Timer (RAT) module."]
    #[inline(always)]
    pub const fn set_RAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Enable clock to the Packet Handling Accelerator (PHA) module."]
    #[must_use]
    #[inline(always)]
    pub const fn PHA(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Enable clock to the Packet Handling Accelerator (PHA) module."]
    #[inline(always)]
    pub const fn set_PHA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
    #[must_use]
    #[inline(always)]
    pub const fn FSCA(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Enable clock to the Frequency Synthesizer Calibration Accelerator (FSCA) module."]
    #[inline(always)]
    pub const fn set_FSCA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Enable clock to the RF Core Tracer (RFCTRC) module."]
    #[must_use]
    #[inline(always)]
    pub const fn RFCTRC(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Enable clock to the RF Core Tracer (RFCTRC) module."]
    #[inline(always)]
    pub const fn set_RFCTRC(&mut self, val: bool) {
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
impl Default for PWMCLKEN {
    #[inline(always)]
    fn default() -> PWMCLKEN {
        PWMCLKEN(0)
    }
}
impl core::fmt::Debug for PWMCLKEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMCLKEN")
            .field("RFC", &self.RFC())
            .field("CPE", &self.CPE())
            .field("CPERAM", &self.CPERAM())
            .field("MDM", &self.MDM())
            .field("MDMRAM", &self.MDMRAM())
            .field("RFE", &self.RFE())
            .field("RFERAM", &self.RFERAM())
            .field("RAT", &self.RAT())
            .field("PHA", &self.PHA())
            .field("FSCA", &self.FSCA())
            .field("RFCTRC", &self.RFCTRC())
            .field("RESERVED11", &self.RESERVED11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWMCLKEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWMCLKEN {{ RFC: {=bool:?}, CPE: {=bool:?}, CPERAM: {=bool:?}, MDM: {=bool:?}, MDMRAM: {=bool:?}, RFE: {=bool:?}, RFERAM: {=bool:?}, RAT: {=bool:?}, PHA: {=bool:?}, FSCA: {=bool:?}, RFCTRC: {=bool:?}, RESERVED11: {=u32:?} }}",
            self.RFC(),
            self.CPE(),
            self.CPERAM(),
            self.MDM(),
            self.MDMRAM(),
            self.RFE(),
            self.RFERAM(),
            self.RAT(),
            self.PHA(),
            self.FSCA(),
            self.RFCTRC(),
            self.RESERVED11()
        )
    }
}

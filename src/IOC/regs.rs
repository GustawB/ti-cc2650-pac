#[doc = "Configuration of DIO0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG0(pub u32);
impl IOCFG0 {
    #[doc = "5:0\\] Selects usage for DIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG0_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG0_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO0."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG0_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG0_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG0_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG0_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG0_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG0_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG0_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG0_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG0_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG0_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG0_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG0_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG0_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG0_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG0_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG0_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG0 {
    #[inline(always)]
    fn default() -> IOCFG0 {
        IOCFG0(0)
    }
}
impl core::fmt::Debug for IOCFG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG0")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG0 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG1(pub u32);
impl IOCFG1 {
    #[doc = "5:0\\] Selects usage for DIO1."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG1_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG1_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO1."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG1_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG1_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG1_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG1_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG1_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG1_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG1_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG1_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG1_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG1_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG1_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG1_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG1_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG1_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG1_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG1_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG1 {
    #[inline(always)]
    fn default() -> IOCFG1 {
        IOCFG1(0)
    }
}
impl core::fmt::Debug for IOCFG1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG1")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG1 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO10."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG10(pub u32);
impl IOCFG10 {
    #[doc = "5:0\\] Selects usage for DIO10."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG10_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG10_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO10."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG10_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG10_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG10_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG10_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG10_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG10_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG10_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG10_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG10_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG10_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG10_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG10_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG10_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG10_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG10_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG10_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG10 {
    #[inline(always)]
    fn default() -> IOCFG10 {
        IOCFG10(0)
    }
}
impl core::fmt::Debug for IOCFG10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG10")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG10 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO11."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG11(pub u32);
impl IOCFG11 {
    #[doc = "5:0\\] Selects usage for DIO11."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG11_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG11_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO11."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG11_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG11_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG11_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG11_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG11_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG11_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG11_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG11_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG11_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG11_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG11_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG11_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG11_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG11_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG11_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG11_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG11 {
    #[inline(always)]
    fn default() -> IOCFG11 {
        IOCFG11(0)
    }
}
impl core::fmt::Debug for IOCFG11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG11")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG11 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO12."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG12(pub u32);
impl IOCFG12 {
    #[doc = "5:0\\] Selects usage for DIO12."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG12_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG12_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO12."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG12_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG12_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG12_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG12_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG12_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG12_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG12_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG12_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG12_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG12_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG12_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG12_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG12_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG12_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG12_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG12_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG12 {
    #[inline(always)]
    fn default() -> IOCFG12 {
        IOCFG12(0)
    }
}
impl core::fmt::Debug for IOCFG12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG12")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG12 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO13."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG13(pub u32);
impl IOCFG13 {
    #[doc = "5:0\\] Selects usage for DIO13."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG13_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG13_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO13."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG13_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG13_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG13_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG13_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG13_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG13_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG13_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG13_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG13_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG13_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG13_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG13_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG13_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG13_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG13_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG13_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG13 {
    #[inline(always)]
    fn default() -> IOCFG13 {
        IOCFG13(0)
    }
}
impl core::fmt::Debug for IOCFG13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG13")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG13 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO14."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG14(pub u32);
impl IOCFG14 {
    #[doc = "5:0\\] Selects usage for DIO14."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG14_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG14_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO14."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG14_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG14_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG14_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG14_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG14_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG14_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG14_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG14_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG14_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG14_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG14_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG14_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG14_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG14_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG14_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG14_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG14 {
    #[inline(always)]
    fn default() -> IOCFG14 {
        IOCFG14(0)
    }
}
impl core::fmt::Debug for IOCFG14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG14")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG14 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO15."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG15(pub u32);
impl IOCFG15 {
    #[doc = "5:0\\] Selects usage for DIO15."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG15_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG15_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO15."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG15_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG15_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG15_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG15_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG15_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG15_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG15_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG15_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG15_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG15_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG15_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG15_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG15_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG15_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG15_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG15_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG15 {
    #[inline(always)]
    fn default() -> IOCFG15 {
        IOCFG15(0)
    }
}
impl core::fmt::Debug for IOCFG15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG15")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG15 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO16."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG16(pub u32);
impl IOCFG16 {
    #[doc = "5:0\\] Selects usage for DIO16."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG16_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG16_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO16."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG16_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG16_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG16_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG16_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG16_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG16_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG16_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG16_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG16_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG16_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG16_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG16_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG16_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG16_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG16_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG16_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG16 {
    #[inline(always)]
    fn default() -> IOCFG16 {
        IOCFG16(0)
    }
}
impl core::fmt::Debug for IOCFG16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG16")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG16 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO17."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG17(pub u32);
impl IOCFG17 {
    #[doc = "5:0\\] Selects usage for DIO17."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG17_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG17_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO17."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG17_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG17_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG17_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG17_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG17_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG17_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG17_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG17_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG17_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG17_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG17_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG17_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG17_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG17_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG17_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG17_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG17 {
    #[inline(always)]
    fn default() -> IOCFG17 {
        IOCFG17(0)
    }
}
impl core::fmt::Debug for IOCFG17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG17")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG17 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO18."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG18(pub u32);
impl IOCFG18 {
    #[doc = "5:0\\] Selects usage for DIO18."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG18_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG18_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO18."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG18_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG18_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG18_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG18_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG18_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG18_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG18_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG18_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG18_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG18_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG18_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG18_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG18_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG18_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG18_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG18_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG18 {
    #[inline(always)]
    fn default() -> IOCFG18 {
        IOCFG18(0)
    }
}
impl core::fmt::Debug for IOCFG18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG18")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG18 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO19."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG19(pub u32);
impl IOCFG19 {
    #[doc = "5:0\\] Selects usage for DIO19."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG19_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG19_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO19."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG19_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG19_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG19_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG19_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG19_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG19_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG19_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG19_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG19_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG19_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG19_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG19_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG19_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG19_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG19_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG19_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG19 {
    #[inline(always)]
    fn default() -> IOCFG19 {
        IOCFG19(0)
    }
}
impl core::fmt::Debug for IOCFG19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG19")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG19 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG2(pub u32);
impl IOCFG2 {
    #[doc = "5:0\\] Selects usage for DIO2."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG2_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG2_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO2."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG2_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG2_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG2_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG2_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG2_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG2_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG2_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG2_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG2_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG2_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG2_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG2_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG2_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG2_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG2_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG2_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG2 {
    #[inline(always)]
    fn default() -> IOCFG2 {
        IOCFG2(0)
    }
}
impl core::fmt::Debug for IOCFG2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG2")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG2 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO20."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG20(pub u32);
impl IOCFG20 {
    #[doc = "5:0\\] Selects usage for DIO20."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG20_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG20_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO20."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG20_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG20_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG20_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG20_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG20_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG20_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG20_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG20_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG20_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG20_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG20_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG20_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG20_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG20_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG20_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG20_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG20 {
    #[inline(always)]
    fn default() -> IOCFG20 {
        IOCFG20(0)
    }
}
impl core::fmt::Debug for IOCFG20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG20")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG20 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO21."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG21(pub u32);
impl IOCFG21 {
    #[doc = "5:0\\] Selects usage for DIO21."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG21_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG21_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO21."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG21_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG21_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG21_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG21_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG21_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG21_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG21_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG21_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG21_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG21_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG21_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG21_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG21_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG21_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG21_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG21_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG21 {
    #[inline(always)]
    fn default() -> IOCFG21 {
        IOCFG21(0)
    }
}
impl core::fmt::Debug for IOCFG21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG21")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG21 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO22."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG22(pub u32);
impl IOCFG22 {
    #[doc = "5:0\\] Selects usage for DIO22."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG22_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG22_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO22."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG22_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG22_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG22_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG22_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG22_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG22_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG22_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG22_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG22_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG22_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG22_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG22_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG22_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG22_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG22_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG22_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG22 {
    #[inline(always)]
    fn default() -> IOCFG22 {
        IOCFG22(0)
    }
}
impl core::fmt::Debug for IOCFG22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG22")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG22 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO23."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG23(pub u32);
impl IOCFG23 {
    #[doc = "5:0\\] Selects usage for DIO23."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG23_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG23_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO23."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG23_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG23_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG23_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG23_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG23_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG23_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG23_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG23_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG23_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG23_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG23_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG23_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG23_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG23_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG23_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG23_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG23 {
    #[inline(always)]
    fn default() -> IOCFG23 {
        IOCFG23(0)
    }
}
impl core::fmt::Debug for IOCFG23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG23")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG23 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO24."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG24(pub u32);
impl IOCFG24 {
    #[doc = "5:0\\] Selects usage for DIO24."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG24_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG24_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO24."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG24_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG24_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG24_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG24_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG24_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG24_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG24_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG24_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG24_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG24_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG24_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG24_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG24_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG24_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG24_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG24_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG24 {
    #[inline(always)]
    fn default() -> IOCFG24 {
        IOCFG24(0)
    }
}
impl core::fmt::Debug for IOCFG24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG24")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG24 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO25."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG25(pub u32);
impl IOCFG25 {
    #[doc = "5:0\\] Selects usage for DIO25."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG25_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG25_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO25."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG25_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG25_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG25_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG25_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG25_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG25_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG25_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG25_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG25_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG25_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG25_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG25_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG25_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG25_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG25_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG25_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG25 {
    #[inline(always)]
    fn default() -> IOCFG25 {
        IOCFG25(0)
    }
}
impl core::fmt::Debug for IOCFG25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG25")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG25 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO26."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG26(pub u32);
impl IOCFG26 {
    #[doc = "5:0\\] Selects usage for DIO26."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG26_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG26_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO26."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG26_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG26_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG26_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG26_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG26_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG26_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG26_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG26_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG26_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG26_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG26_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG26_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG26_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG26_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG26_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG26_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG26 {
    #[inline(always)]
    fn default() -> IOCFG26 {
        IOCFG26(0)
    }
}
impl core::fmt::Debug for IOCFG26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG26")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG26 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO27."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG27(pub u32);
impl IOCFG27 {
    #[doc = "5:0\\] Selects usage for DIO27."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG27_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG27_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO27."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG27_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG27_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG27_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG27_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG27_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG27_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG27_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG27_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG27_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG27_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG27_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG27_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG27_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG27_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG27_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG27_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG27 {
    #[inline(always)]
    fn default() -> IOCFG27 {
        IOCFG27(0)
    }
}
impl core::fmt::Debug for IOCFG27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG27")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG27 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO28."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG28(pub u32);
impl IOCFG28 {
    #[doc = "5:0\\] Selects usage for DIO28."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG28_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG28_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO28."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG28_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG28_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG28_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG28_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG28_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG28_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG28_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG28_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG28_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG28_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG28_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG28_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG28_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG28_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG28_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG28_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG28 {
    #[inline(always)]
    fn default() -> IOCFG28 {
        IOCFG28(0)
    }
}
impl core::fmt::Debug for IOCFG28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG28")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG28 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO29."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG29(pub u32);
impl IOCFG29 {
    #[doc = "5:0\\] Selects usage for DIO29."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG29_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG29_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO29."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG29_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG29_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG29_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG29_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG29_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG29_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG29_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG29_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG29_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG29_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG29_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG29_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG29_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG29_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG29_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG29_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG29 {
    #[inline(always)]
    fn default() -> IOCFG29 {
        IOCFG29(0)
    }
}
impl core::fmt::Debug for IOCFG29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG29")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG29 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG3(pub u32);
impl IOCFG3 {
    #[doc = "5:0\\] Selects usage for DIO3."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG3_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG3_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO3."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG3_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG3_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG3_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG3_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG3_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG3_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG3_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG3_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG3_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG3_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG3_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG3_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG3_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG3_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG3_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG3_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG3 {
    #[inline(always)]
    fn default() -> IOCFG3 {
        IOCFG3(0)
    }
}
impl core::fmt::Debug for IOCFG3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG3")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG3 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO30."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG30(pub u32);
impl IOCFG30 {
    #[doc = "5:0\\] Selects usage for DIO30."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG30_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG30_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO30."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG30_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG30_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG30_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG30_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG30_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG30_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG30_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG30_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG30_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG30_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG30_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG30_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG30_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG30_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG30_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG30_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG30 {
    #[inline(always)]
    fn default() -> IOCFG30 {
        IOCFG30(0)
    }
}
impl core::fmt::Debug for IOCFG30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG30")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG30 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO31."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG31(pub u32);
impl IOCFG31 {
    #[doc = "5:0\\] Selects usage for DIO31."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG31_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG31_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO31."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG31_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG31_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG31_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG31_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG31_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG31_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG31_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG31_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG31_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG31_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG31_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG31_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG31_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG31_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG31_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG31_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG31 {
    #[inline(always)]
    fn default() -> IOCFG31 {
        IOCFG31(0)
    }
}
impl core::fmt::Debug for IOCFG31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG31")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG31 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG4(pub u32);
impl IOCFG4 {
    #[doc = "5:0\\] Selects usage for DIO4."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG4_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG4_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO4."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG4_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG4_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG4_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG4_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG4_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG4_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG4_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG4_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG4_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG4_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG4_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG4_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG4_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG4_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG4_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG4_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG4 {
    #[inline(always)]
    fn default() -> IOCFG4 {
        IOCFG4(0)
    }
}
impl core::fmt::Debug for IOCFG4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG4")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG4 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG5(pub u32);
impl IOCFG5 {
    #[doc = "5:0\\] Selects usage for DIO5."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG5_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG5_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO5."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG5_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG5_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG5_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG5_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG5_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG5_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG5_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG5_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG5_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG5_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG5_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG5_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG5_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG5_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG5_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG5_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG5 {
    #[inline(always)]
    fn default() -> IOCFG5 {
        IOCFG5(0)
    }
}
impl core::fmt::Debug for IOCFG5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG5")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG5 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG6(pub u32);
impl IOCFG6 {
    #[doc = "5:0\\] Selects usage for DIO6."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG6_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG6_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO6."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG6_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG6_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG6_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG6_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG6_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG6_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG6_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG6_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG6_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG6_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG6_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG6_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG6_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG6_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG6_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG6_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG6 {
    #[inline(always)]
    fn default() -> IOCFG6 {
        IOCFG6(0)
    }
}
impl core::fmt::Debug for IOCFG6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG6")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG6 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG7(pub u32);
impl IOCFG7 {
    #[doc = "5:0\\] Selects usage for DIO7."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG7_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG7_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO7."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG7_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG7_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG7_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG7_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG7_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG7_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG7_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG7_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG7_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG7_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG7_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG7_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG7_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG7_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG7_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG7_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG7 {
    #[inline(always)]
    fn default() -> IOCFG7 {
        IOCFG7(0)
    }
}
impl core::fmt::Debug for IOCFG7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG7")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG7 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO8."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG8(pub u32);
impl IOCFG8 {
    #[doc = "5:0\\] Selects usage for DIO8."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG8_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG8_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO8."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG8_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG8_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG8_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG8_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG8_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG8_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG8_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG8_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG8_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG8_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG8_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG8_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG8_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG8_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG8_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG8_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG8 {
    #[inline(always)]
    fn default() -> IOCFG8 {
        IOCFG8(0)
    }
}
impl core::fmt::Debug for IOCFG8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG8")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG8 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}
#[doc = "Configuration of DIO9."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCFG9(pub u32);
impl IOCFG9 {
    #[doc = "5:0\\] Selects usage for DIO9."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ID(&self) -> super::vals::IOCFG9_PORT_ID {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::IOCFG9_PORT_ID::from_bits(val as u8)
    }
    #[doc = "5:0\\] Selects usage for DIO9."]
    #[inline(always)]
    pub const fn set_PORT_ID(&mut self, val: super::vals::IOCFG9_PORT_ID) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[must_use]
    #[inline(always)]
    pub const fn IOSTR(&self) -> super::vals::IOCFG9_IOSTR {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IOCFG9_IOSTR::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR."]
    #[inline(always)]
    pub const fn set_IOSTR(&mut self, val: super::vals::IOCFG9_IOSTR) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn IOCURR(&self) -> super::vals::IOCFG9_IOCURR {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IOCFG9_IOCURR::from_bits(val as u8)
    }
    #[doc = "11:10\\] Selects IO current mode of this IO."]
    #[inline(always)]
    pub const fn set_IOCURR(&mut self, val: super::vals::IOCFG9_IOCURR) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW_RED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline(always)]
    pub const fn set_SLEW_RED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Pull control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULL_CTL(&self) -> super::vals::IOCFG9_PULL_CTL {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::IOCFG9_PULL_CTL::from_bits(val as u8)
    }
    #[doc = "14:13\\] Pull control."]
    #[inline(always)]
    pub const fn set_PULL_CTL(&mut self, val: super::vals::IOCFG9_PULL_CTL) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_DET(&self) -> super::vals::IOCFG9_EDGE_DET {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::IOCFG9_EDGE_DET::from_bits(val as u8)
    }
    #[doc = "17:16\\] Enable generation of edge detection events on this IO."]
    #[inline(always)]
    pub const fn set_EDGE_DET(&mut self, val: super::vals::IOCFG9_EDGE_DET) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE_IRQ_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)."]
    #[inline(always)]
    pub const fn set_EDGE_IRQ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMODE(&self) -> super::vals::IOCFG9_IOMODE {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::IOCFG9_IOMODE::from_bits(val as u8)
    }
    #[doc = "26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline(always)]
    pub const fn set_IOMODE(&mut self, val: super::vals::IOCFG9_IOMODE) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[must_use]
    #[inline(always)]
    pub const fn WU_CFG(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline(always)]
    pub const fn set_WU_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HYST_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable."]
    #[inline(always)]
    pub const fn set_HYST_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IOCFG9 {
    #[inline(always)]
    fn default() -> IOCFG9 {
        IOCFG9(0)
    }
}
impl core::fmt::Debug for IOCFG9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCFG9")
            .field("PORT_ID", &self.PORT_ID())
            .field("RESERVED6", &self.RESERVED6())
            .field("IOSTR", &self.IOSTR())
            .field("IOCURR", &self.IOCURR())
            .field("SLEW_RED", &self.SLEW_RED())
            .field("PULL_CTL", &self.PULL_CTL())
            .field("RESERVED15", &self.RESERVED15())
            .field("EDGE_DET", &self.EDGE_DET())
            .field("EDGE_IRQ_EN", &self.EDGE_IRQ_EN())
            .field("RESERVED19", &self.RESERVED19())
            .field("IOMODE", &self.IOMODE())
            .field("WU_CFG", &self.WU_CFG())
            .field("IE", &self.IE())
            .field("HYST_EN", &self.HYST_EN())
            .field("RESERVED31", &self.RESERVED31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCFG9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCFG9 {{ PORT_ID: {:?}, RESERVED6: {=u8:?}, IOSTR: {:?}, IOCURR: {:?}, SLEW_RED: {=bool:?}, PULL_CTL: {:?}, RESERVED15: {=bool:?}, EDGE_DET: {:?}, EDGE_IRQ_EN: {=bool:?}, RESERVED19: {=u8:?}, IOMODE: {:?}, WU_CFG: {=u8:?}, IE: {=bool:?}, HYST_EN: {=bool:?}, RESERVED31: {=bool:?} }}",
            self.PORT_ID(),
            self.RESERVED6(),
            self.IOSTR(),
            self.IOCURR(),
            self.SLEW_RED(),
            self.PULL_CTL(),
            self.RESERVED15(),
            self.EDGE_DET(),
            self.EDGE_IRQ_EN(),
            self.RESERVED19(),
            self.IOMODE(),
            self.WU_CFG(),
            self.IE(),
            self.HYST_EN(),
            self.RESERVED31()
        )
    }
}

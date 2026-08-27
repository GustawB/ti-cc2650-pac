#[doc = "Channel 0 Compare Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CH0CMP(pub u32);
impl CH0CMP {
    #[doc = "31:0\\] RTC Channel 0 compare value. Bit 31 to 16 represents seconds and bits 15 to 0 represents subseconds of the compare value. The compare value is compared against SEC.VALUE (15:0) and SUBSEC.VALUE (31:16) values of the Real Time Clock register. A Cannel 0 event is generated when {SEC.VALUE(15:0),SUBSEC.VALUE (31:16)} is reaching or exciting the compare value. Writing to this register can trigger an immediate*) event in case the new compare value matches a Real Time Clock value from 1 second in the past up till current Real Time Clock value. Example: To generate a compare 5.5 seconds RTC start,- set this value = 0x0005_8000 *) It can take up to 2 SCLK_LF clock cycles before event occurs due to synchronization."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] RTC Channel 0 compare value. Bit 31 to 16 represents seconds and bits 15 to 0 represents subseconds of the compare value. The compare value is compared against SEC.VALUE (15:0) and SUBSEC.VALUE (31:16) values of the Real Time Clock register. A Cannel 0 event is generated when {SEC.VALUE(15:0),SUBSEC.VALUE (31:16)} is reaching or exciting the compare value. Writing to this register can trigger an immediate*) event in case the new compare value matches a Real Time Clock value from 1 second in the past up till current Real Time Clock value. Example: To generate a compare 5.5 seconds RTC start,- set this value = 0x0005_8000 *) It can take up to 2 SCLK_LF clock cycles before event occurs due to synchronization."]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CH0CMP {
    #[inline(always)]
    fn default() -> CH0CMP {
        CH0CMP(0)
    }
}
impl core::fmt::Debug for CH0CMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH0CMP")
            .field("VALUE", &self.VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CH0CMP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CH0CMP {{ VALUE: {=u32:?} }}", self.VALUE())
    }
}
#[doc = "Channel 1 Capture Value If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CH1CAPT(pub u32);
impl CH1CAPT {
    #[doc = "15:0\\] Value of SUBSEC.VALUE bits 31:16 at capture time."]
    #[must_use]
    #[inline(always)]
    pub const fn SUBSEC(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Value of SUBSEC.VALUE bits 31:16 at capture time."]
    #[inline(always)]
    pub const fn set_SUBSEC(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Value of SEC.VALUE bits 15:0 at capture time."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Value of SEC.VALUE bits 15:0 at capture time."]
    #[inline(always)]
    pub const fn set_SEC(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CH1CAPT {
    #[inline(always)]
    fn default() -> CH1CAPT {
        CH1CAPT(0)
    }
}
impl core::fmt::Debug for CH1CAPT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH1CAPT")
            .field("SUBSEC", &self.SUBSEC())
            .field("SEC", &self.SEC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CH1CAPT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CH1CAPT {{ SUBSEC: {=u16:?}, SEC: {=u16:?} }}",
            self.SUBSEC(),
            self.SEC()
        )
    }
}
#[doc = "Channel 1 Compare Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CH1CMP(pub u32);
impl CH1CMP {
    #[doc = "31:0\\] RTC Channel 1 compare value. Bit 31 to 16 represents seconds and bits 15 to 0 represents subseconds of the compare value. The compare value is compared against SEC.VALUE (15:0) and SUBSEC.VALUE (31:16) values of the Real Time Clock register. A Cannel 0 event is generated when {SEC.VALUE(15:0),SUBSEC.VALUE (31:16)} is reaching or exciting the compare value. Writing to this register can trigger an immediate*) event in case the new compare value matches a Real Time Clock value from 1 second in the past up till current Real Time Clock value. Example: To generate a compare 5.5 seconds RTC start,- set this value = 0x0005_8000 *) It can take up to 2 SCLK_LF clock cycles before event occurs due to synchronization."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] RTC Channel 1 compare value. Bit 31 to 16 represents seconds and bits 15 to 0 represents subseconds of the compare value. The compare value is compared against SEC.VALUE (15:0) and SUBSEC.VALUE (31:16) values of the Real Time Clock register. A Cannel 0 event is generated when {SEC.VALUE(15:0),SUBSEC.VALUE (31:16)} is reaching or exciting the compare value. Writing to this register can trigger an immediate*) event in case the new compare value matches a Real Time Clock value from 1 second in the past up till current Real Time Clock value. Example: To generate a compare 5.5 seconds RTC start,- set this value = 0x0005_8000 *) It can take up to 2 SCLK_LF clock cycles before event occurs due to synchronization."]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CH1CMP {
    #[inline(always)]
    fn default() -> CH1CMP {
        CH1CMP(0)
    }
}
impl core::fmt::Debug for CH1CMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH1CMP")
            .field("VALUE", &self.VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CH1CMP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CH1CMP {{ VALUE: {=u32:?} }}", self.VALUE())
    }
}
#[doc = "Channel 2 Compare Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CH2CMP(pub u32);
impl CH2CMP {
    #[doc = "31:0\\] RTC Channel 2 compare value. Bit 31 to 16 represents seconds and bits 15 to 0 represents subseconds of the compare value. The compare value is compared against SEC.VALUE (15:0) and SUBSEC.VALUE (31:16) values of the Real Time Clock register. A Cannel 0 event is generated when {SEC.VALUE(15:0),SUBSEC.VALUE (31:16)} is reaching or exciting the compare value. Writing to this register can trigger an immediate*) event in case the new compare value matches a Real Time Clock value from 1 second in the past up till current Real Time Clock value. Example: To generate a compare 5.5 seconds RTC start,- set this value = 0x0005_8000 *) It can take up to 2 SCLK_LF clock cycles before event occurs due to synchronization."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] RTC Channel 2 compare value. Bit 31 to 16 represents seconds and bits 15 to 0 represents subseconds of the compare value. The compare value is compared against SEC.VALUE (15:0) and SUBSEC.VALUE (31:16) values of the Real Time Clock register. A Cannel 0 event is generated when {SEC.VALUE(15:0),SUBSEC.VALUE (31:16)} is reaching or exciting the compare value. Writing to this register can trigger an immediate*) event in case the new compare value matches a Real Time Clock value from 1 second in the past up till current Real Time Clock value. Example: To generate a compare 5.5 seconds RTC start,- set this value = 0x0005_8000 *) It can take up to 2 SCLK_LF clock cycles before event occurs due to synchronization."]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CH2CMP {
    #[inline(always)]
    fn default() -> CH2CMP {
        CH2CMP(0)
    }
}
impl core::fmt::Debug for CH2CMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH2CMP")
            .field("VALUE", &self.VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CH2CMP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CH2CMP {{ VALUE: {=u32:?} }}", self.VALUE())
    }
}
#[doc = "Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\] event."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CH2CMPINC(pub u32);
impl CH2CMPINC {
    #[doc = "31:0\\] If CHCTL.CH2_CONT_EN is set, this value is added to CH2CMP.VALUE on every channel 2 compare event."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] If CHCTL.CH2_CONT_EN is set, this value is added to CH2CMP.VALUE on every channel 2 compare event."]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CH2CMPINC {
    #[inline(always)]
    fn default() -> CH2CMPINC {
        CH2CMPINC(0)
    }
}
impl core::fmt::Debug for CH2CMPINC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH2CMPINC")
            .field("VALUE", &self.VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CH2CMPINC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CH2CMPINC {{ VALUE: {=u32:?} }}", self.VALUE())
    }
}
#[doc = "Channel Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CHCTL(pub u32);
impl CHCTL {
    #[doc = "0:0\\] RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn CH0_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] RTC Channel 0 Enable 0: Disable RTC Channel 0 1: Enable RTC Channel 0."]
    #[inline(always)]
    pub const fn set_CH0_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
    #[doc = "8:8\\] RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1."]
    #[must_use]
    #[inline(always)]
    pub const fn CH1_EN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] RTC Channel 1 Enable 0: Disable RTC Channel 1 1: Enable RTC Channel 1."]
    #[inline(always)]
    pub const fn set_CH1_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Set Channel 1 mode 0: Compare mode (default) 1: Capture mode."]
    #[must_use]
    #[inline(always)]
    pub const fn CH1_CAPT_EN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Set Channel 1 mode 0: Compare mode (default) 1: Capture mode."]
    #[inline(always)]
    pub const fn set_CH1_CAPT_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "15:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED10(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "15:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[doc = "16:16\\] RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2."]
    #[must_use]
    #[inline(always)]
    pub const fn CH2_EN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] RTC Channel 2 Enable 0: Disable RTC Channel 2 1: Enable RTC Channel 2."]
    #[inline(always)]
    pub const fn set_CH2_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Set to enable continuous operation of Channel 2."]
    #[must_use]
    #[inline(always)]
    pub const fn CH2_CONT_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Set to enable continuous operation of Channel 2."]
    #[inline(always)]
    pub const fn set_CH2_CONT_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "31:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x1fff;
        val as u16
    }
    #[doc = "31:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 19usize)) | (((val as u32) & 0x1fff) << 19usize);
    }
}
impl Default for CHCTL {
    #[inline(always)]
    fn default() -> CHCTL {
        CHCTL(0)
    }
}
impl core::fmt::Debug for CHCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHCTL")
            .field("CH0_EN", &self.CH0_EN())
            .field("RESERVED1", &self.RESERVED1())
            .field("CH1_EN", &self.CH1_EN())
            .field("CH1_CAPT_EN", &self.CH1_CAPT_EN())
            .field("RESERVED10", &self.RESERVED10())
            .field("CH2_EN", &self.CH2_EN())
            .field("RESERVED17", &self.RESERVED17())
            .field("CH2_CONT_EN", &self.CH2_CONT_EN())
            .field("RESERVED19", &self.RESERVED19())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CHCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CHCTL {{ CH0_EN: {=bool:?}, RESERVED1: {=u8:?}, CH1_EN: {=bool:?}, CH1_CAPT_EN: {=bool:?}, RESERVED10: {=u8:?}, CH2_EN: {=bool:?}, RESERVED17: {=bool:?}, CH2_CONT_EN: {=bool:?}, RESERVED19: {=u16:?} }}",
            self.CH0_EN(),
            self.RESERVED1(),
            self.CH1_EN(),
            self.CH1_CAPT_EN(),
            self.RESERVED10(),
            self.CH2_EN(),
            self.RESERVED17(),
            self.CH2_CONT_EN(),
            self.RESERVED19()
        )
    }
}
#[doc = "Control This register contains various bitfields for configuration of RTC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTL(pub u32);
impl CTL {
    #[doc = "0:0\\] Enable RTC counter 0: Halted (frozen) 1: Running."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Enable RTC counter 0: Halted (frozen) 1: Running."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] RTC_UPD is a 16 KHz signal used to sync up the radio timer. The 16 Khz is SCLK_LF divided by 2 0: RTC_UPD signal is forced to 0 1: RTC_UPD signal is toggling @16 kHz."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_UPD_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] RTC_UPD is a 16 KHz signal used to sync up the radio timer. The 16 Khz is SCLK_LF divided by 2 0: RTC_UPD signal is forced to 0 1: RTC_UPD signal is toggling @16 kHz."]
    #[inline(always)]
    pub const fn set_RTC_UPD_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] RTC_4KHZ is a 4 KHz reference output, tapped from SUBSEC.VALUE bit 19 which is used by AUX timer. 0: RTC_4KHZ signal is forced to 0 1: RTC_4KHZ is enabled ( provied that RTC is enabled EN)."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_4KHZ_EN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] RTC_4KHZ is a 4 KHz reference output, tapped from SUBSEC.VALUE bit 19 which is used by AUX timer. 0: RTC_4KHZ signal is forced to 0 1: RTC_4KHZ is enabled ( provied that RTC is enabled EN)."]
    #[inline(always)]
    pub const fn set_RTC_4KHZ_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "6:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "6:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "7:7\\] RTC Counter reset. Writing 1 to this bit will reset the RTC counter. This bit is cleared when reset takes effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] RTC Counter reset. Writing 1 to this bit will reset the RTC counter. This bit is cleared when reset takes effect."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "11:8\\] Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed."]
    #[must_use]
    #[inline(always)]
    pub const fn EV_DELAY(&self) -> super::vals::EV_DELAY {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::EV_DELAY::from_bits(val as u8)
    }
    #[doc = "11:8\\] Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed."]
    #[inline(always)]
    pub const fn set_EV_DELAY(&mut self, val: super::vals::EV_DELAY) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "15:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "18:16\\] Eventmask selecting which delayed events that form the combined event."]
    #[must_use]
    #[inline(always)]
    pub const fn COMB_EV_MASK(&self) -> super::vals::COMB_EV_MASK {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::COMB_EV_MASK::from_bits(val as u8)
    }
    #[doc = "18:16\\] Eventmask selecting which delayed events that form the combined event."]
    #[inline(always)]
    pub const fn set_COMB_EV_MASK(&mut self, val: super::vals::COMB_EV_MASK) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "31:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x1fff;
        val as u16
    }
    #[doc = "31:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 19usize)) | (((val as u32) & 0x1fff) << 19usize);
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
            .field("EN", &self.EN())
            .field("RTC_UPD_EN", &self.RTC_UPD_EN())
            .field("RTC_4KHZ_EN", &self.RTC_4KHZ_EN())
            .field("RESERVED3", &self.RESERVED3())
            .field("RESET", &self.RESET())
            .field("EV_DELAY", &self.EV_DELAY())
            .field("RESERVED12", &self.RESERVED12())
            .field("COMB_EV_MASK", &self.COMB_EV_MASK())
            .field("RESERVED19", &self.RESERVED19())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTL {{ EN: {=bool:?}, RTC_UPD_EN: {=bool:?}, RTC_4KHZ_EN: {=bool:?}, RESERVED3: {=u8:?}, RESET: {=bool:?}, EV_DELAY: {:?}, RESERVED12: {=u8:?}, COMB_EV_MASK: {:?}, RESERVED19: {=u16:?} }}",
            self.EN(),
            self.RTC_UPD_EN(),
            self.RTC_4KHZ_EN(),
            self.RESERVED3(),
            self.RESET(),
            self.EV_DELAY(),
            self.RESERVED12(),
            self.COMB_EV_MASK(),
            self.RESERVED19()
        )
    }
}
#[doc = "Event Flags, RTC Status This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVFLAGS(pub u32);
impl EVFLAGS {
    #[doc = "0:0\\] Channel 0 event flag, set when CHCTL.CH0_EN = 1 and the RTC value matches or passes the CH0CMP value. An event will be scheduled to occur as soon as possible when writing to CH0CMP provided that the channels is enabled and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance."]
    #[must_use]
    #[inline(always)]
    pub const fn CH0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Channel 0 event flag, set when CHCTL.CH0_EN = 1 and the RTC value matches or passes the CH0CMP value. An event will be scheduled to occur as soon as possible when writing to CH0CMP provided that the channels is enabled and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance."]
    #[inline(always)]
    pub const fn set_CH0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
    #[doc = "8:8\\] Channel 1 event flag, set when CHCTL.CH1_EN = 1 and one of the following: - CHCTL.CH1_CAPT_EN = 0 and the RTC value matches or passes the CH1CMP value. - CHCTL.CH1_CAPT_EN = 1 and capture occurs. An event will be scheduled to occur as soon as possible when writing to CH1CMP provided that the channel is enabled, in compare mode and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance."]
    #[must_use]
    #[inline(always)]
    pub const fn CH1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Channel 1 event flag, set when CHCTL.CH1_EN = 1 and one of the following: - CHCTL.CH1_CAPT_EN = 0 and the RTC value matches or passes the CH1CMP value. - CHCTL.CH1_CAPT_EN = 1 and capture occurs. An event will be scheduled to occur as soon as possible when writing to CH1CMP provided that the channel is enabled, in compare mode and the new value matches any time between next RTC value and 1 second in the past. Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance."]
    #[inline(always)]
    pub const fn set_CH1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "16:16\\] Channel 2 event flag, set when CHCTL.CH2_EN = 1 and the RTC value matches or passes the CH2CMP value. An event will be scheduled to occur as soon as possible when writing to CH2CMP provided that the channel is enabled and the new value matches any time between next RTC value and 1 second in the past Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance. AUX_SCE can read the flag through AUX_WUC:WUEVFLAGS.AON_RTC_CH2 and clear it using AUX_WUC:WUEVCLR.AON_RTC_CH2."]
    #[must_use]
    #[inline(always)]
    pub const fn CH2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Channel 2 event flag, set when CHCTL.CH2_EN = 1 and the RTC value matches or passes the CH2CMP value. An event will be scheduled to occur as soon as possible when writing to CH2CMP provided that the channel is enabled and the new value matches any time between next RTC value and 1 second in the past Writing 1 clears this flag. Note that a new event can not occur on this channel in first 2 SCLK_LF cycles after a clearance. AUX_SCE can read the flag through AUX_WUC:WUEVFLAGS.AON_RTC_CH2 and clear it using AUX_WUC:WUEVCLR.AON_RTC_CH2."]
    #[inline(always)]
    pub const fn set_CH2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "31:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "31:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for EVFLAGS {
    #[inline(always)]
    fn default() -> EVFLAGS {
        EVFLAGS(0)
    }
}
impl core::fmt::Debug for EVFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVFLAGS")
            .field("CH0", &self.CH0())
            .field("RESERVED1", &self.RESERVED1())
            .field("CH1", &self.CH1())
            .field("RESERVED9", &self.RESERVED9())
            .field("CH2", &self.CH2())
            .field("RESERVED17", &self.RESERVED17())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EVFLAGS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EVFLAGS {{ CH0: {=bool:?}, RESERVED1: {=u8:?}, CH1: {=bool:?}, RESERVED9: {=u8:?}, CH2: {=bool:?}, RESERVED17: {=u16:?} }}",
            self.CH0(),
            self.RESERVED1(),
            self.CH1(),
            self.RESERVED9(),
            self.CH2(),
            self.RESERVED17()
        )
    }
}
#[doc = "Second Counter Value, Integer Part."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEC(pub u32);
impl SEC {
    #[doc = "31:0\\] Unsigned integer representing Real Time Clock in seconds. When reading this register the content of SUBSEC.VALUE is simultaneously latched. A consistent reading of the combined Real Time Clock can be obtained by first reading this register, then reading SUBSEC register."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Unsigned integer representing Real Time Clock in seconds. When reading this register the content of SUBSEC.VALUE is simultaneously latched. A consistent reading of the combined Real Time Clock can be obtained by first reading this register, then reading SUBSEC register."]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SEC {
    #[inline(always)]
    fn default() -> SEC {
        SEC(0)
    }
}
impl core::fmt::Debug for SEC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC").field("VALUE", &self.VALUE()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SEC {{ VALUE: {=u32:?} }}", self.VALUE())
    }
}
#[doc = "Second Counter Value, Fractional Part."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SUBSEC(pub u32);
impl SUBSEC {
    #[doc = "31:0\\] Unsigned integer representing Real Time Clock in fractions of a second (VALUE/2^32 seconds) at the time when SEC register was read. Examples : - 0x0000_0000 = 0.0 sec - 0x4000_0000 = 0.25 sec - 0x8000_0000 = 0.5 sec - 0xC000_0000 = 0.75 sec."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Unsigned integer representing Real Time Clock in fractions of a second (VALUE/2^32 seconds) at the time when SEC register was read. Examples : - 0x0000_0000 = 0.0 sec - 0x4000_0000 = 0.25 sec - 0x8000_0000 = 0.5 sec - 0xC000_0000 = 0.75 sec."]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SUBSEC {
    #[inline(always)]
    fn default() -> SUBSEC {
        SUBSEC(0)
    }
}
impl core::fmt::Debug for SUBSEC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUBSEC")
            .field("VALUE", &self.VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SUBSEC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SUBSEC {{ VALUE: {=u32:?} }}", self.VALUE())
    }
}
#[doc = "Subseconds Increment Value added to SUBSEC.VALUE on every SCLK_LFclock cycle."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SUBSECINC(pub u32);
impl SUBSECINC {
    #[doc = "23:0\\] This value compensates for a SCLK_LF clock which has an offset from 32768 Hz. The compensation value can be found as 2^38 / freq, where freq is SCLK_LF clock frequency in Hertz This value is added to SUBSEC.VALUE on every cycle, and carry of this is added to SEC.VALUE. To perform the addition, bits \\[23:6\\] are aligned with SUBSEC.VALUE bits \\[17:0\\]. The remaining bits \\[5:0\\] are accumulated in a hidden 6-bit register that generates a carry into the above mentioned addition on overflow. The default value corresponds to incrementing by precisely 1/32768 of a second. NOTE: This register is read only. Modification of the register value must be done using registers AUX_WUC:RTCSUBSECINC1 , AUX_WUC:RTCSUBSECINC0 and AUX_WUC:RTCSUBSECINCCTL."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUEINC(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] This value compensates for a SCLK_LF clock which has an offset from 32768 Hz. The compensation value can be found as 2^38 / freq, where freq is SCLK_LF clock frequency in Hertz This value is added to SUBSEC.VALUE on every cycle, and carry of this is added to SEC.VALUE. To perform the addition, bits \\[23:6\\] are aligned with SUBSEC.VALUE bits \\[17:0\\]. The remaining bits \\[5:0\\] are accumulated in a hidden 6-bit register that generates a carry into the above mentioned addition on overflow. The default value corresponds to incrementing by precisely 1/32768 of a second. NOTE: This register is read only. Modification of the register value must be done using registers AUX_WUC:RTCSUBSECINC1 , AUX_WUC:RTCSUBSECINC0 and AUX_WUC:RTCSUBSECINCCTL."]
    #[inline(always)]
    pub const fn set_VALUEINC(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED24(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED24(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for SUBSECINC {
    #[inline(always)]
    fn default() -> SUBSECINC {
        SUBSECINC(0)
    }
}
impl core::fmt::Debug for SUBSECINC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUBSECINC")
            .field("VALUEINC", &self.VALUEINC())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SUBSECINC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SUBSECINC {{ VALUEINC: {=u32:?}, RESERVED24: {=u8:?} }}",
            self.VALUEINC(),
            self.RESERVED24()
        )
    }
}
#[doc = "AON Synchronization This register is used for synchronizing between MCU and entire AON domain."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYNC(pub u32);
impl SYNC {
    #[doc = "0:0\\] This register will always return 0,- however it will not return the value until there are no outstanding write requests between MCU and AON Note: Writing to this register prior to reading will force a wait until next SCLK_LF edge. This is recommended for syncing read registers from AON when waking up from sleep Failure to do so may result in reading AON values from prior to going to sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn WBUSY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This register will always return 0,- however it will not return the value until there are no outstanding write requests between MCU and AON Note: Writing to this register prior to reading will force a wait until next SCLK_LF edge. This is recommended for syncing read registers from AON when waking up from sleep Failure to do so may result in reading AON values from prior to going to sleep."]
    #[inline(always)]
    pub const fn set_WBUSY(&mut self, val: bool) {
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
impl Default for SYNC {
    #[inline(always)]
    fn default() -> SYNC {
        SYNC(0)
    }
}
impl core::fmt::Debug for SYNC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNC")
            .field("WBUSY", &self.WBUSY())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SYNC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SYNC {{ WBUSY: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.WBUSY(),
            self.RESERVED1()
        )
    }
}

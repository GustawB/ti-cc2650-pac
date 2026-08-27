#[doc = "ADC Control 0 ADC Sample Control. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC0(pub u8);
impl ADC0 {
    #[doc = "0:0\\] ADC Enable 0: Disable 1: Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] ADC Enable 0: Disable 1: Enable."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Reset ADC digital subchip, active low. ADC must be reset every time it is reconfigured. 0: Reset 1: Normal operation."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET_N(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Reset ADC digital subchip, active low. ADC must be reset every time it is reconfigured. 0: Reset 1: Normal operation."]
    #[inline(always)]
    pub const fn set_RESET_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "6:3\\] Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us."]
    #[must_use]
    #[inline(always)]
    pub const fn SMPL_CYCLE_EXP(&self) -> super::vals::SMPL_CYCLE_EXP {
        let val = (self.0 >> 3usize) & 0x0f;
        super::vals::SMPL_CYCLE_EXP::from_bits(val as u8)
    }
    #[doc = "6:3\\] Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us."]
    #[inline(always)]
    pub const fn set_SMPL_CYCLE_EXP(&mut self, val: super::vals::SMPL_CYCLE_EXP) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val.to_bits() as u8) & 0x0f) << 3usize);
    }
    #[doc = "7:7\\] ADC Sampling mode: 0: Synchronous mode 1: Asynchronous mode The ADC does a sample-and-hold before conversion. In synchronous mode the sampling starts when the ADC clock detects a rising edge on the trigger signal. Jitter/uncertainty will be inferred in the detection if the trigger signal originates from a domain that is asynchronous to the ADC clock. SMPL_CYCLE_EXP determines the the duration of sampling. Conversion starts immediately after sampling ends. In asynchronous mode the sampling is continuous when enabled. Sampling ends and conversion starts immediately with the rising edge of the trigger signal. Sampling restarts when the conversion has finished. Asynchronous mode is useful when it is important to avoid jitter in the sampling instant of an externally driven signal."]
    #[must_use]
    #[inline(always)]
    pub const fn SMPL_MODE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] ADC Sampling mode: 0: Synchronous mode 1: Asynchronous mode The ADC does a sample-and-hold before conversion. In synchronous mode the sampling starts when the ADC clock detects a rising edge on the trigger signal. Jitter/uncertainty will be inferred in the detection if the trigger signal originates from a domain that is asynchronous to the ADC clock. SMPL_CYCLE_EXP determines the the duration of sampling. Conversion starts immediately after sampling ends. In asynchronous mode the sampling is continuous when enabled. Sampling ends and conversion starts immediately with the rising edge of the trigger signal. Sampling restarts when the conversion has finished. Asynchronous mode is useful when it is important to avoid jitter in the sampling instant of an externally driven signal."]
    #[inline(always)]
    pub const fn set_SMPL_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for ADC0 {
    #[inline(always)]
    fn default() -> ADC0 {
        ADC0(0)
    }
}
impl core::fmt::Debug for ADC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC0")
            .field("EN", &self.EN())
            .field("RESET_N", &self.RESET_N())
            .field("RESERVED2", &self.RESERVED2())
            .field("SMPL_CYCLE_EXP", &self.SMPL_CYCLE_EXP())
            .field("SMPL_MODE", &self.SMPL_MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC0 {{ EN: {=bool:?}, RESET_N: {=bool:?}, RESERVED2: {=bool:?}, SMPL_CYCLE_EXP: {:?}, SMPL_MODE: {=bool:?} }}",
            self.EN(),
            self.RESET_N(),
            self.RESERVED2(),
            self.SMPL_CYCLE_EXP(),
            self.SMPL_MODE()
        )
    }
}
#[doc = "ADC Control 1 ADC Comparator Control. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC1(pub u8);
impl ADC1 {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SCALE_DIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SCALE_DIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
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
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for ADC1 {
    #[inline(always)]
    fn default() -> ADC1 {
        ADC1(0)
    }
}
impl core::fmt::Debug for ADC1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC1")
            .field("SCALE_DIS", &self.SCALE_DIS())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC1 {{ SCALE_DIS: {=bool:?}, RESERVED1: {=u8:?} }}",
            self.SCALE_DIS(),
            self.RESERVED1()
        )
    }
}
#[doc = "ADC Reference 0 Control reference used by the ADC. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADCREF0(pub u8);
impl ADCREF0 {
    #[doc = "0:0\\] ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] ADC reference module enable: 0: ADC reference module powered down 1: ADC reference module enabled."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "2:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "2:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u8) & 0x03) << 1usize);
    }
    #[doc = "3:3\\] ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS."]
    #[must_use]
    #[inline(always)]
    pub const fn SRC(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] ADC reference source: 0: Fixed reference = 4.3V 1: Relative reference = VDDS."]
    #[inline(always)]
    pub const fn set_SRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EXT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IOMUX(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IOMUX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Keep ADCREF powered up in IDLE state when ADC0.SMPL_MODE = 0. Set to 1 if ADC0.SMPL_CYCLE_EXP is less than 6 (21.3us sampling time)."]
    #[must_use]
    #[inline(always)]
    pub const fn REF_ON_IDLE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Keep ADCREF powered up in IDLE state when ADC0.SMPL_MODE = 0. Set to 1 if ADC0.SMPL_CYCLE_EXP is less than 6 (21.3us sampling time)."]
    #[inline(always)]
    pub const fn set_REF_ON_IDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
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
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for ADCREF0 {
    #[inline(always)]
    fn default() -> ADCREF0 {
        ADCREF0(0)
    }
}
impl core::fmt::Debug for ADCREF0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCREF0")
            .field("EN", &self.EN())
            .field("RESERVED1", &self.RESERVED1())
            .field("SRC", &self.SRC())
            .field("EXT", &self.EXT())
            .field("IOMUX", &self.IOMUX())
            .field("REF_ON_IDLE", &self.REF_ON_IDLE())
            .field("RESERVED7", &self.RESERVED7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADCREF0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADCREF0 {{ EN: {=bool:?}, RESERVED1: {=u8:?}, SRC: {=bool:?}, EXT: {=bool:?}, IOMUX: {=bool:?}, REF_ON_IDLE: {=bool:?}, RESERVED7: {=bool:?} }}",
            self.EN(),
            self.RESERVED1(),
            self.SRC(),
            self.EXT(),
            self.IOMUX(),
            self.REF_ON_IDLE(),
            self.RESERVED7()
        )
    }
}
#[doc = "ADC Reference 1 Control reference used by the ADC. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADCREF1(pub u8);
impl ADCREF1 {
    #[doc = "5:0\\] Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V."]
    #[must_use]
    #[inline(always)]
    pub const fn VTRIM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V."]
    #[inline(always)]
    pub const fn set_VTRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u8) & 0x3f) << 0usize);
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
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
    }
}
impl Default for ADCREF1 {
    #[inline(always)]
    fn default() -> ADCREF1 {
        ADCREF1(0)
    }
}
impl core::fmt::Debug for ADCREF1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCREF1")
            .field("VTRIM", &self.VTRIM())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADCREF1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADCREF1 {{ VTRIM: {=u8:?}, RESERVED6: {=u8:?} }}",
            self.VTRIM(),
            self.RESERVED6()
        )
    }
}
#[doc = "Comparator Control COMPA and COMPB comparators. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COMP(pub u8);
impl COMP {
    #[doc = "0:0\\] COMPA enable."]
    #[must_use]
    #[inline(always)]
    pub const fn COMPA_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] COMPA enable."]
    #[inline(always)]
    pub const fn set_COMPA_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] COMPB enable."]
    #[must_use]
    #[inline(always)]
    pub const fn COMPB_EN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] COMPB enable."]
    #[inline(always)]
    pub const fn set_COMPB_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "5:3\\] COMPB voltage reference trim temperature coded:."]
    #[must_use]
    #[inline(always)]
    pub const fn COMPB_TRIM(&self) -> super::vals::COMPB_TRIM {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::COMPB_TRIM::from_bits(val as u8)
    }
    #[doc = "5:3\\] COMPB voltage reference trim temperature coded:."]
    #[inline(always)]
    pub const fn set_COMPB_TRIM(&mut self, val: super::vals::COMPB_TRIM) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u8) & 0x07) << 3usize);
    }
    #[doc = "6:6\\] Enables 2uA IPTAT current from ISRC to COMPA reference node. Requires ISRC.EN = 1. Used with COMPA_REF_RES_EN to generate voltage reference for cap-sense."]
    #[must_use]
    #[inline(always)]
    pub const fn COMPA_REF_CURR_EN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Enables 2uA IPTAT current from ISRC to COMPA reference node. Requires ISRC.EN = 1. Used with COMPA_REF_RES_EN to generate voltage reference for cap-sense."]
    #[inline(always)]
    pub const fn set_COMPA_REF_CURR_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Enables 400kohm resistance from COMPA reference node to ground. Used with COMPA_REF_CURR_EN to generate voltage reference for cap-sense."]
    #[must_use]
    #[inline(always)]
    pub const fn COMPA_REF_RES_EN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Enables 400kohm resistance from COMPA reference node to ground. Used with COMPA_REF_CURR_EN to generate voltage reference for cap-sense."]
    #[inline(always)]
    pub const fn set_COMPA_REF_RES_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for COMP {
    #[inline(always)]
    fn default() -> COMP {
        COMP(0)
    }
}
impl core::fmt::Debug for COMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP")
            .field("COMPA_EN", &self.COMPA_EN())
            .field("RESERVED1", &self.RESERVED1())
            .field("COMPB_EN", &self.COMPB_EN())
            .field("COMPB_TRIM", &self.COMPB_TRIM())
            .field("COMPA_REF_CURR_EN", &self.COMPA_REF_CURR_EN())
            .field("COMPA_REF_RES_EN", &self.COMPA_REF_RES_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COMP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COMP {{ COMPA_EN: {=bool:?}, RESERVED1: {=bool:?}, COMPB_EN: {=bool:?}, COMPB_TRIM: {:?}, COMPA_REF_CURR_EN: {=bool:?}, COMPA_REF_RES_EN: {=bool:?} }}",
            self.COMPA_EN(),
            self.RESERVED1(),
            self.COMPB_EN(),
            self.COMPB_TRIM(),
            self.COMPA_REF_CURR_EN(),
            self.COMPA_REF_RES_EN()
        )
    }
}
#[doc = "Current Source Strength and trim control for current source. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ISRC(pub u8);
impl ISRC {
    #[doc = "0:0\\] Current source enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Current source enable."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "7:2\\] Adjust current from current source. Output currents may be combined to get desired total current."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM(&self) -> super::vals::TRIM {
        let val = (self.0 >> 2usize) & 0x3f;
        super::vals::TRIM::from_bits(val as u8)
    }
    #[doc = "7:2\\] Adjust current from current source. Output currents may be combined to get desired total current."]
    #[inline(always)]
    pub const fn set_TRIM(&mut self, val: super::vals::TRIM) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val.to_bits() as u8) & 0x3f) << 2usize);
    }
}
impl Default for ISRC {
    #[inline(always)]
    fn default() -> ISRC {
        ISRC(0)
    }
}
impl core::fmt::Debug for ISRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISRC")
            .field("EN", &self.EN())
            .field("RESERVED1", &self.RESERVED1())
            .field("TRIM", &self.TRIM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ISRC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ISRC {{ EN: {=bool:?}, RESERVED1: {=bool:?}, TRIM: {:?} }}",
            self.EN(),
            self.RESERVED1(),
            self.TRIM()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MUX0(pub u8);
impl MUX0 {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn COMPA_REF(&self) -> super::vals::MUX0_COMPA_REF {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::MUX0_COMPA_REF::from_bits(val as u8)
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_COMPA_REF(&mut self, val: super::vals::MUX0_COMPA_REF) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
    #[doc = "7:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "7:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
    }
}
impl Default for MUX0 {
    #[inline(always)]
    fn default() -> MUX0 {
        MUX0(0)
    }
}
impl core::fmt::Debug for MUX0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUX0")
            .field("COMPA_REF", &self.COMPA_REF())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MUX0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MUX0 {{ COMPA_REF: {:?}, RESERVED4: {=u8:?} }}",
            self.COMPA_REF(),
            self.RESERVED4()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MUX1(pub u8);
impl MUX1 {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn COMPA_IN(&self) -> super::vals::COMPA_IN {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::COMPA_IN::from_bits(val as u8)
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_COMPA_IN(&mut self, val: super::vals::COMPA_IN) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u8) & 0xff) << 0usize);
    }
}
impl Default for MUX1 {
    #[inline(always)]
    fn default() -> MUX1 {
        MUX1(0)
    }
}
impl core::fmt::Debug for MUX1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUX1")
            .field("COMPA_IN", &self.COMPA_IN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MUX1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MUX1 {{ COMPA_IN: {:?} }}", self.COMPA_IN())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MUX2(pub u8);
impl MUX2 {
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn COMPB_REF(&self) -> super::vals::COMPB_REF {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::COMPB_REF::from_bits(val as u8)
    }
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_COMPB_REF(&mut self, val: super::vals::COMPB_REF) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u8) & 0x07) << 0usize);
    }
    #[doc = "7:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCCOMPB_IN(&self) -> super::vals::MUX2_ADCCOMPB_IN {
        let val = (self.0 >> 3usize) & 0x1f;
        super::vals::MUX2_ADCCOMPB_IN::from_bits(val as u8)
    }
    #[doc = "7:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ADCCOMPB_IN(&mut self, val: super::vals::MUX2_ADCCOMPB_IN) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val.to_bits() as u8) & 0x1f) << 3usize);
    }
}
impl Default for MUX2 {
    #[inline(always)]
    fn default() -> MUX2 {
        MUX2(0)
    }
}
impl core::fmt::Debug for MUX2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUX2")
            .field("COMPB_REF", &self.COMPB_REF())
            .field("ADCCOMPB_IN", &self.ADCCOMPB_IN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MUX2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MUX2 {{ COMPB_REF: {:?}, ADCCOMPB_IN: {:?} }}",
            self.COMPB_REF(),
            self.ADCCOMPB_IN()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MUX3(pub u8);
impl MUX3 {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCCOMPB_IN(&self) -> super::vals::MUX3_ADCCOMPB_IN {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::MUX3_ADCCOMPB_IN::from_bits(val as u8)
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ADCCOMPB_IN(&mut self, val: super::vals::MUX3_ADCCOMPB_IN) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u8) & 0xff) << 0usize);
    }
}
impl Default for MUX3 {
    #[inline(always)]
    fn default() -> MUX3 {
        MUX3(0)
    }
}
impl core::fmt::Debug for MUX3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUX3")
            .field("ADCCOMPB_IN", &self.ADCCOMPB_IN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MUX3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MUX3 {{ ADCCOMPB_IN: {:?} }}", self.ADCCOMPB_IN())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MUX4(pub u8);
impl MUX4 {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn COMPA_REF(&self) -> super::vals::MUX4_COMPA_REF {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::MUX4_COMPA_REF::from_bits(val as u8)
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_COMPA_REF(&mut self, val: super::vals::MUX4_COMPA_REF) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u8) & 0xff) << 0usize);
    }
}
impl Default for MUX4 {
    #[inline(always)]
    fn default() -> MUX4 {
        MUX4(0)
    }
}
impl core::fmt::Debug for MUX4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUX4")
            .field("COMPA_REF", &self.COMPA_REF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MUX4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MUX4 {{ COMPA_REF: {:?} }}", self.COMPA_REF())
    }
}

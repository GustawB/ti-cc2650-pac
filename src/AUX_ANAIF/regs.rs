#[doc = "ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADCCTL(pub u32);
impl ADCCTL {
    #[doc = "1:0\\] ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
    #[must_use]
    #[inline(always)]
    pub const fn CMD(&self) -> super::vals::CMD {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CMD::from_bits(val as u8)
    }
    #[doc = "1:0\\] ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
    #[inline(always)]
    pub const fn set_CMD(&mut self, val: super::vals::CMD) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "7:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "7:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "12:8\\] Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT<n> if you want to trigger the ADC manually through ADCTRIG.START."]
    #[must_use]
    #[inline(always)]
    pub const fn START_SRC(&self) -> super::vals::START_SRC {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::START_SRC::from_bits(val as u8)
    }
    #[doc = "12:8\\] Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT<n> if you want to trigger the ADC manually through ADCTRIG.START."]
    #[inline(always)]
    pub const fn set_START_SRC(&mut self, val: super::vals::START_SRC) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
    #[doc = "13:13\\] Select active polarity for START_SRC event."]
    #[must_use]
    #[inline(always)]
    pub const fn START_POL(&self) -> super::vals::START_POL {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::START_POL::from_bits(val as u8)
    }
    #[doc = "13:13\\] Select active polarity for START_SRC event."]
    #[inline(always)]
    pub const fn set_START_POL(&mut self, val: super::vals::START_POL) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "31:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED14(&self) -> u32 {
        let val = (self.0 >> 14usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "31:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED14(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 14usize)) | (((val as u32) & 0x0003_ffff) << 14usize);
    }
}
impl Default for ADCCTL {
    #[inline(always)]
    fn default() -> ADCCTL {
        ADCCTL(0)
    }
}
impl core::fmt::Debug for ADCCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCCTL")
            .field("CMD", &self.CMD())
            .field("RESERVED2", &self.RESERVED2())
            .field("START_SRC", &self.START_SRC())
            .field("START_POL", &self.START_POL())
            .field("RESERVED14", &self.RESERVED14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADCCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADCCTL {{ CMD: {:?}, RESERVED2: {=u8:?}, START_SRC: {:?}, START_POL: {:?}, RESERVED14: {=u32:?} }}",
            self.CMD(),
            self.RESERVED2(),
            self.START_SRC(),
            self.START_POL(),
            self.RESERVED14()
        )
    }
}
#[doc = "ADC FIFO."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADCFIFO(pub u32);
impl ADCFIFO {
    #[doc = "11:0\\] FIFO data. Read: Get oldest ADC sample from FIFO. Write: Write dummy sample to FIFO. This is useful for code development when you do not have real ADC samples."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "11:0\\] FIFO data. Read: Get oldest ADC sample from FIFO. Write: Write dummy sample to FIFO. This is useful for code development when you do not have real ADC samples."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "31:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "31:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for ADCFIFO {
    #[inline(always)]
    fn default() -> ADCFIFO {
        ADCFIFO(0)
    }
}
impl core::fmt::Debug for ADCFIFO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCFIFO")
            .field("DATA", &self.DATA())
            .field("RESERVED12", &self.RESERVED12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADCFIFO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADCFIFO {{ DATA: {=u16:?}, RESERVED12: {=u32:?} }}",
            self.DATA(),
            self.RESERVED12()
        )
    }
}
#[doc = "ADC FIFO Status FIFO can hold up to four ADC samples."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADCFIFOSTAT(pub u32);
impl ADCFIFOSTAT {
    #[doc = "0:0\\] FIFO empty flag. 0: FIFO contains one or more samples. 1: FIFO is empty. When the flag is set, read returns the previous sample that was read and sets the UNDERFLOW flag."]
    #[must_use]
    #[inline(always)]
    pub const fn EMPTY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] FIFO empty flag. 0: FIFO contains one or more samples. 1: FIFO is empty. When the flag is set, read returns the previous sample that was read and sets the UNDERFLOW flag."]
    #[inline(always)]
    pub const fn set_EMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] FIFO almost full flag. 0: There are less than 3 samples in the FIFO, or the FIFO is full. The FULL flag is also asserted in the latter case. 1: There are 3 samples in the FIFO, there is room for one more sample."]
    #[must_use]
    #[inline(always)]
    pub const fn ALMOST_FULL(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] FIFO almost full flag. 0: There are less than 3 samples in the FIFO, or the FIFO is full. The FULL flag is also asserted in the latter case. 1: There are 3 samples in the FIFO, there is room for one more sample."]
    #[inline(always)]
    pub const fn set_ALMOST_FULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] FIFO full flag. 0: FIFO is not full, there is less than 4 samples in the FIFO. 1: FIFO is full, there are 4 samples in the FIFO. When the flag is set, it is not possible to add more samples to the ADC FIFO. An attempt to add samples sets the OVERFLOW flag."]
    #[must_use]
    #[inline(always)]
    pub const fn FULL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] FIFO full flag. 0: FIFO is not full, there is less than 4 samples in the FIFO. 1: FIFO is full, there are 4 samples in the FIFO. When the flag is set, it is not possible to add more samples to the ADC FIFO. An attempt to add samples sets the OVERFLOW flag."]
    #[inline(always)]
    pub const fn set_FULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] FIFO underflow flag. 0: FIFO has not underflowed. 1: FIFO has underflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO read pointer is static. Read returns the previous sample that was read. Flush FIFO to clear the flag."]
    #[must_use]
    #[inline(always)]
    pub const fn UNDERFLOW(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] FIFO underflow flag. 0: FIFO has not underflowed. 1: FIFO has underflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO read pointer is static. Read returns the previous sample that was read. Flush FIFO to clear the flag."]
    #[inline(always)]
    pub const fn set_UNDERFLOW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] FIFO overflow flag. 0: FIFO has not overflowed. 1: FIFO has overflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO write pointer is static. It is not possible to add more samples to the ADC FIFO. Flush FIFO to clear the flag."]
    #[must_use]
    #[inline(always)]
    pub const fn OVERFLOW(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] FIFO overflow flag. 0: FIFO has not overflowed. 1: FIFO has overflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO write pointer is static. It is not possible to add more samples to the ADC FIFO. Flush FIFO to clear the flag."]
    #[inline(always)]
    pub const fn set_OVERFLOW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
impl Default for ADCFIFOSTAT {
    #[inline(always)]
    fn default() -> ADCFIFOSTAT {
        ADCFIFOSTAT(0)
    }
}
impl core::fmt::Debug for ADCFIFOSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCFIFOSTAT")
            .field("EMPTY", &self.EMPTY())
            .field("ALMOST_FULL", &self.ALMOST_FULL())
            .field("FULL", &self.FULL())
            .field("UNDERFLOW", &self.UNDERFLOW())
            .field("OVERFLOW", &self.OVERFLOW())
            .field("RESERVED5", &self.RESERVED5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADCFIFOSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADCFIFOSTAT {{ EMPTY: {=bool:?}, ALMOST_FULL: {=bool:?}, FULL: {=bool:?}, UNDERFLOW: {=bool:?}, OVERFLOW: {=bool:?}, RESERVED5: {=u32:?} }}",
            self.EMPTY(),
            self.ALMOST_FULL(),
            self.FULL(),
            self.UNDERFLOW(),
            self.OVERFLOW(),
            self.RESERVED5()
        )
    }
}
#[doc = "ADC Trigger."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADCTRIG(pub u32);
impl ADCTRIG {
    #[doc = "0:0\\] Manual ADC trigger. 0: No effect. 1: Single ADC trigger. To manually trigger the ADC, you must set ADCCTL.START_SRC to NO_EVENT<n> to avoid conflict with event-driven ADC trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn START(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Manual ADC trigger. 0: No effect. 1: Single ADC trigger. To manually trigger the ADC, you must set ADCCTL.START_SRC to NO_EVENT<n> to avoid conflict with event-driven ADC trigger."]
    #[inline(always)]
    pub const fn set_START(&mut self, val: bool) {
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
impl Default for ADCTRIG {
    #[inline(always)]
    fn default() -> ADCTRIG {
        ADCTRIG(0)
    }
}
impl core::fmt::Debug for ADCTRIG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCTRIG")
            .field("START", &self.START())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADCTRIG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADCTRIG {{ START: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.START(),
            self.RESERVED1()
        )
    }
}
#[doc = "Current Source Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ISRCCTL(pub u32);
impl ISRCCTL {
    #[doc = "0:0\\] ISRC reset control. 0: ISRC drives 0 uA. 1: ISRC drives current ADI_4_AUX:ISRC.TRIM to COMPA_IN."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET_N(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] ISRC reset control. 0: ISRC drives 0 uA. 1: ISRC drives current ADI_4_AUX:ISRC.TRIM to COMPA_IN."]
    #[inline(always)]
    pub const fn set_RESET_N(&mut self, val: bool) {
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
impl Default for ISRCCTL {
    #[inline(always)]
    fn default() -> ISRCCTL {
        ISRCCTL(0)
    }
}
impl core::fmt::Debug for ISRCCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISRCCTL")
            .field("RESET_N", &self.RESET_N())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ISRCCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ISRCCTL {{ RESET_N: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.RESET_N(),
            self.RESERVED1()
        )
    }
}

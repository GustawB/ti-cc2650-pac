#[doc = "Alarm Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ALARMCNT(pub u32);
impl ALARMCNT {
    #[doc = "7:0\\] Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
    #[must_use]
    #[inline(always)]
    pub const fn ALARM_THR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
    #[inline(always)]
    pub const fn set_ALARM_THR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "20:16\\] Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
    #[must_use]
    #[inline(always)]
    pub const fn SHUTDOWN_THR(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "20:16\\] Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
    #[inline(always)]
    pub const fn set_SHUTDOWN_THR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "23:21\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED21(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "23:21\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED21(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "29:24\\] Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
    #[must_use]
    #[inline(always)]
    pub const fn SHUTDOWN_CNT(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "29:24\\] Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
    #[inline(always)]
    pub const fn set_SHUTDOWN_CNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "31:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED30(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "31:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED30(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ALARMCNT {
    #[inline(always)]
    fn default() -> ALARMCNT {
        ALARMCNT(0)
    }
}
impl core::fmt::Debug for ALARMCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALARMCNT")
            .field("ALARM_THR", &self.ALARM_THR())
            .field("RESERVED8", &self.RESERVED8())
            .field("SHUTDOWN_THR", &self.SHUTDOWN_THR())
            .field("RESERVED21", &self.RESERVED21())
            .field("SHUTDOWN_CNT", &self.SHUTDOWN_CNT())
            .field("RESERVED30", &self.RESERVED30())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ALARMCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ALARMCNT {{ ALARM_THR: {=u8:?}, RESERVED8: {=u8:?}, SHUTDOWN_THR: {=u8:?}, RESERVED21: {=u8:?}, SHUTDOWN_CNT: {=u8:?}, RESERVED30: {=u8:?} }}",
            self.ALARM_THR(),
            self.RESERVED8(),
            self.SHUTDOWN_THR(),
            self.RESERVED21(),
            self.SHUTDOWN_CNT(),
            self.RESERVED30()
        )
    }
}
#[doc = "Alarm Event."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ALARMMASK(pub u32);
impl ALARMMASK {
    #[doc = "23:0\\] Logging bits for the 'alarm events' of individual FROs. A '1' in bit \\[n\\] indicates FRO 'n' experienced an 'alarm event'."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO_MASK(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Logging bits for the 'alarm events' of individual FROs. A '1' in bit \\[n\\] indicates FRO 'n' experienced an 'alarm event'."]
    #[inline(always)]
    pub const fn set_FRO_MASK(&mut self, val: u32) {
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
impl Default for ALARMMASK {
    #[inline(always)]
    fn default() -> ALARMMASK {
        ALARMMASK(0)
    }
}
impl core::fmt::Debug for ALARMMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALARMMASK")
            .field("FRO_MASK", &self.FRO_MASK())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ALARMMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ALARMMASK {{ FRO_MASK: {=u32:?}, RESERVED24: {=u8:?} }}",
            self.FRO_MASK(),
            self.RESERVED24()
        )
    }
}
#[doc = "Alarm Shutdown."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ALARMSTOP(pub u32);
impl ALARMSTOP {
    #[doc = "23:0\\] Logging bits for the 'alarm events' of individual FROs. A '1' in bit \\[n\\] indicates FRO 'n' experienced more than one 'alarm event' in quick succession and has been turned off. A '1' in this field forces the corresponding bit in FROEN.FRO_MASK to '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO_FLAGS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Logging bits for the 'alarm events' of individual FROs. A '1' in bit \\[n\\] indicates FRO 'n' experienced more than one 'alarm event' in quick succession and has been turned off. A '1' in this field forces the corresponding bit in FROEN.FRO_MASK to '0'."]
    #[inline(always)]
    pub const fn set_FRO_FLAGS(&mut self, val: u32) {
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
impl Default for ALARMSTOP {
    #[inline(always)]
    fn default() -> ALARMSTOP {
        ALARMSTOP(0)
    }
}
impl core::fmt::Debug for ALARMSTOP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALARMSTOP")
            .field("FRO_FLAGS", &self.FRO_FLAGS())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ALARMSTOP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ALARMSTOP {{ FRO_FLAGS: {=u32:?}, RESERVED24: {=u8:?} }}",
            self.FRO_FLAGS(),
            self.RESERVED24()
        )
    }
}
#[doc = "Configuration 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG0(pub u32);
impl CFG0 {
    #[doc = "7:0\\] This field determines the minimum number of samples (between 2^6 and 2^14) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the value of this field is zero, the number of samples is fixed to the value determined by the MAX_REFILL_CYCLES field, otherwise the minimum number of samples equals the written value times 64 (which can be up to 2^14). To ensure same entropy in all generated random numbers the value 0 should be used. Then MAX_REFILL_CYCLES controls the minimum refill interval. The number of samples defined here cannot be higher than the number defined by the 'max_refill_cycles' field (i.e. that field takes precedence). No random value will be created if min refill > max refill. This field can only be modified while CTL.TRNG_EN = 0. 0x00: Minimum samples = MAX_REFILL_CYCLES (all numbers have same entropy) 0x01: 1*2^6 samples 0x02: 2*2^6 samples ... 0xFF: 255*2^6 samples."]
    #[must_use]
    #[inline(always)]
    pub const fn MIN_REFILL_CYCLES(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] This field determines the minimum number of samples (between 2^6 and 2^14) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the value of this field is zero, the number of samples is fixed to the value determined by the MAX_REFILL_CYCLES field, otherwise the minimum number of samples equals the written value times 64 (which can be up to 2^14). To ensure same entropy in all generated random numbers the value 0 should be used. Then MAX_REFILL_CYCLES controls the minimum refill interval. The number of samples defined here cannot be higher than the number defined by the 'max_refill_cycles' field (i.e. that field takes precedence). No random value will be created if min refill > max refill. This field can only be modified while CTL.TRNG_EN = 0. 0x00: Minimum samples = MAX_REFILL_CYCLES (all numbers have same entropy) 0x01: 1*2^6 samples 0x02: 2*2^6 samples ... 0xFF: 255*2^6 samples."]
    #[inline(always)]
    pub const fn set_MIN_REFILL_CYCLES(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "11:8\\] This field directly controls the number of clock cycles between samples taken from the FROs. Default value 0 indicates that samples are taken every clock cycle, maximum value 0xF takes one sample every 16 clock cycles. This field must be set to a value such that the slowest FRO (even under worst-case conditions) has a cycle time less than twice the sample period. This field can only be modified while CTL.TRNG_EN is '0'."]
    #[must_use]
    #[inline(always)]
    pub const fn SMPL_DIV(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "11:8\\] This field directly controls the number of clock cycles between samples taken from the FROs. Default value 0 indicates that samples are taken every clock cycle, maximum value 0xF takes one sample every 16 clock cycles. This field must be set to a value such that the slowest FRO (even under worst-case conditions) has a cycle time less than twice the sample period. This field can only be modified while CTL.TRNG_EN is '0'."]
    #[inline(always)]
    pub const fn set_SMPL_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
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
    #[doc = "31:16\\] This field determines the maximum number of samples (between 2^8 and 2^24) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while CTL.TRNG_EN is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn MAX_REFILL_CYCLES(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] This field determines the maximum number of samples (between 2^8 and 2^24) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while CTL.TRNG_EN is 0."]
    #[inline(always)]
    pub const fn set_MAX_REFILL_CYCLES(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CFG0 {
    #[inline(always)]
    fn default() -> CFG0 {
        CFG0(0)
    }
}
impl core::fmt::Debug for CFG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG0")
            .field("MIN_REFILL_CYCLES", &self.MIN_REFILL_CYCLES())
            .field("SMPL_DIV", &self.SMPL_DIV())
            .field("RESERVED12", &self.RESERVED12())
            .field("MAX_REFILL_CYCLES", &self.MAX_REFILL_CYCLES())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFG0 {{ MIN_REFILL_CYCLES: {=u8:?}, SMPL_DIV: {=u8:?}, RESERVED12: {=u8:?}, MAX_REFILL_CYCLES: {=u16:?} }}",
            self.MIN_REFILL_CYCLES(),
            self.SMPL_DIV(),
            self.RESERVED12(),
            self.MAX_REFILL_CYCLES()
        )
    }
}
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTL(pub u32);
impl CTL {
    #[doc = "0:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 1: Enables access to the TESTCNT and LFSR0/LFSR1/LFSR2 registers (the latter are automatically cleared before enabling access) and keeps IRQFLAGSTAT.NEED_CLOCK at '1'. This bit shall not be used unless you need to change the LFSR seed prior to creating a new random value. All other testing is done external to register control."]
    #[must_use]
    #[inline(always)]
    pub const fn TEST_MODE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 1: Enables access to the TESTCNT and LFSR0/LFSR1/LFSR2 registers (the latter are automatically cleared before enabling access) and keeps IRQFLAGSTAT.NEED_CLOCK at '1'. This bit shall not be used unless you need to change the LFSR seed prior to creating a new random value. All other testing is done external to register control."]
    #[inline(always)]
    pub const fn set_TEST_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] 1: Remove XNOR feedback from the main LFSR, converting it into a normal shift register for the XOR-ed outputs of the FROs (shifting data in on the LSB side). A '1' also forces the LFSR to sample continuously. This bit can only be set to '1' when TEST_MODE is also set to '1' and should not be used for other than test purposes."]
    #[must_use]
    #[inline(always)]
    pub const fn NO_LFSR_FB(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] 1: Remove XNOR feedback from the main LFSR, converting it into a normal shift register for the XOR-ed outputs of the FROs (shifting data in on the LSB side). A '1' also forces the LFSR to sample continuously. This bit can only be set to '1' when TEST_MODE is also set to '1' and should not be used for other than test purposes."]
    #[inline(always)]
    pub const fn set_NO_LFSR_FB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "9:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x7f;
        val as u8
    }
    #[doc = "9:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 3usize)) | (((val as u32) & 0x7f) << 3usize);
    }
    #[doc = "10:10\\] 0: Forces all TRNG logic back into the idle state immediately. 1: Starts TRNG, gathering entropy from the FROs for the number of samples determined by STARTUP_CYCLES."]
    #[must_use]
    #[inline(always)]
    pub const fn TRNG_EN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] 0: Forces all TRNG logic back into the idle state immediately. 1: Starts TRNG, gathering entropy from the FROs for the number of samples determined by STARTUP_CYCLES."]
    #[inline(always)]
    pub const fn set_TRNG_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "15:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED11(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "15:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
    #[doc = "31:16\\] This field determines the number of samples (between 2^8 and 2^24) taken to gather entropy from the FROs during startup. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while TRNG_EN is 0. If 1 an update will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn STARTUP_CYCLES(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] This field determines the number of samples (between 2^8 and 2^24) taken to gather entropy from the FROs during startup. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while TRNG_EN is 0. If 1 an update will be ignored."]
    #[inline(always)]
    pub const fn set_STARTUP_CYCLES(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
            .field("RESERVED0", &self.RESERVED0())
            .field("TEST_MODE", &self.TEST_MODE())
            .field("NO_LFSR_FB", &self.NO_LFSR_FB())
            .field("RESERVED3", &self.RESERVED3())
            .field("TRNG_EN", &self.TRNG_EN())
            .field("RESERVED11", &self.RESERVED11())
            .field("STARTUP_CYCLES", &self.STARTUP_CYCLES())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTL {{ RESERVED0: {=bool:?}, TEST_MODE: {=bool:?}, NO_LFSR_FB: {=bool:?}, RESERVED3: {=u8:?}, TRNG_EN: {=bool:?}, RESERVED11: {=u8:?}, STARTUP_CYCLES: {=u16:?} }}",
            self.RESERVED0(),
            self.TEST_MODE(),
            self.NO_LFSR_FB(),
            self.RESERVED3(),
            self.TRNG_EN(),
            self.RESERVED11(),
            self.STARTUP_CYCLES()
        )
    }
}
#[doc = "FRO De-tune Bit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FRODETUNE(pub u32);
impl FRODETUNE {
    #[doc = "23:0\\] De-tune bits for the individual FROs. A '1' in bit \\[n\\] lets FRO 'n' run approximately 5% faster. The value of one of these bits may only be changed while the corresponding FRO is turned off (by temporarily writing a '0' in the corresponding bit of the FROEN.FRO_MASK register)."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO_MASK(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] De-tune bits for the individual FROs. A '1' in bit \\[n\\] lets FRO 'n' run approximately 5% faster. The value of one of these bits may only be changed while the corresponding FRO is turned off (by temporarily writing a '0' in the corresponding bit of the FROEN.FRO_MASK register)."]
    #[inline(always)]
    pub const fn set_FRO_MASK(&mut self, val: u32) {
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
impl Default for FRODETUNE {
    #[inline(always)]
    fn default() -> FRODETUNE {
        FRODETUNE(0)
    }
}
impl core::fmt::Debug for FRODETUNE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRODETUNE")
            .field("FRO_MASK", &self.FRO_MASK())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FRODETUNE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FRODETUNE {{ FRO_MASK: {=u32:?}, RESERVED24: {=u8:?} }}",
            self.FRO_MASK(),
            self.RESERVED24()
        )
    }
}
#[doc = "FRO Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FROEN(pub u32);
impl FROEN {
    #[doc = "23:0\\] Enable bits for the individual FROs. A '1' in bit \\[n\\] enables FRO 'n'. Default state is all '1's to enable all FROs after power-up. Note that they are not actually started up before the CTL.TRNG_EN bit is set to '1'. Bits are automatically forced to '0' here (and cannot be written to '1') while the corresponding bit in ALARMSTOP.FRO_FLAGS has value '1'."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO_MASK(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Enable bits for the individual FROs. A '1' in bit \\[n\\] enables FRO 'n'. Default state is all '1's to enable all FROs after power-up. Note that they are not actually started up before the CTL.TRNG_EN bit is set to '1'. Bits are automatically forced to '0' here (and cannot be written to '1') while the corresponding bit in ALARMSTOP.FRO_FLAGS has value '1'."]
    #[inline(always)]
    pub const fn set_FRO_MASK(&mut self, val: u32) {
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
impl Default for FROEN {
    #[inline(always)]
    fn default() -> FROEN {
        FROEN(0)
    }
}
impl core::fmt::Debug for FROEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FROEN")
            .field("FRO_MASK", &self.FRO_MASK())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FROEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FROEN {{ FRO_MASK: {=u32:?}, RESERVED24: {=u8:?} }}",
            self.FRO_MASK(),
            self.RESERVED24()
        )
    }
}
#[doc = "TRNG Engine Options Information."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HWOPT(pub u32);
impl HWOPT {
    #[doc = "5:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "11:6\\] Number of FROs implemented in this TRNG, value 24 (decimal)."]
    #[must_use]
    #[inline(always)]
    pub const fn NR_OF_FROS(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "11:6\\] Number of FROs implemented in this TRNG, value 24 (decimal)."]
    #[inline(always)]
    pub const fn set_NR_OF_FROS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
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
impl Default for HWOPT {
    #[inline(always)]
    fn default() -> HWOPT {
        HWOPT(0)
    }
}
impl core::fmt::Debug for HWOPT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWOPT")
            .field("RESERVED0", &self.RESERVED0())
            .field("NR_OF_FROS", &self.NR_OF_FROS())
            .field("RESERVED12", &self.RESERVED12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HWOPT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HWOPT {{ RESERVED0: {=u8:?}, NR_OF_FROS: {=u8:?}, RESERVED12: {=u32:?} }}",
            self.RESERVED0(),
            self.NR_OF_FROS(),
            self.RESERVED12()
        )
    }
}
#[doc = "HW Version 0 EIP Number And Core Revision."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HWVER0(pub u32);
impl HWVER0 {
    #[doc = "7:0\\] 8 bits binary encoding of the module number. This TRNG gives 0x4B."]
    #[must_use]
    #[inline(always)]
    pub const fn EIP_NUM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] 8 bits binary encoding of the module number. This TRNG gives 0x4B."]
    #[inline(always)]
    pub const fn set_EIP_NUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Bit-by-bit logic complement of bits \\[7:0\\]. This TRNG gives 0xB4."]
    #[must_use]
    #[inline(always)]
    pub const fn EIP_NUM_COMPL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Bit-by-bit logic complement of bits \\[7:0\\]. This TRNG gives 0xB4."]
    #[inline(always)]
    pub const fn set_EIP_NUM_COMPL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "19:16\\] 4 bits binary encoding of the hardware patch level, initial release will carry value zero."]
    #[must_use]
    #[inline(always)]
    pub const fn HW_PATCH_LVL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] 4 bits binary encoding of the hardware patch level, initial release will carry value zero."]
    #[inline(always)]
    pub const fn set_HW_PATCH_LVL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "23:20\\] 4 bits binary encoding of the minor hardware revision number."]
    #[must_use]
    #[inline(always)]
    pub const fn HW_MINOR_VER(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] 4 bits binary encoding of the minor hardware revision number."]
    #[inline(always)]
    pub const fn set_HW_MINOR_VER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "27:24\\] 4 bits binary encoding of the major hardware revision number."]
    #[must_use]
    #[inline(always)]
    pub const fn HW_MAJOR_VER(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] 4 bits binary encoding of the major hardware revision number."]
    #[inline(always)]
    pub const fn set_HW_MAJOR_VER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED28(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED28(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for HWVER0 {
    #[inline(always)]
    fn default() -> HWVER0 {
        HWVER0(0)
    }
}
impl core::fmt::Debug for HWVER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWVER0")
            .field("EIP_NUM", &self.EIP_NUM())
            .field("EIP_NUM_COMPL", &self.EIP_NUM_COMPL())
            .field("HW_PATCH_LVL", &self.HW_PATCH_LVL())
            .field("HW_MINOR_VER", &self.HW_MINOR_VER())
            .field("HW_MAJOR_VER", &self.HW_MAJOR_VER())
            .field("RESERVED28", &self.RESERVED28())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HWVER0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HWVER0 {{ EIP_NUM: {=u8:?}, EIP_NUM_COMPL: {=u8:?}, HW_PATCH_LVL: {=u8:?}, HW_MINOR_VER: {=u8:?}, HW_MAJOR_VER: {=u8:?}, RESERVED28: {=u8:?} }}",
            self.EIP_NUM(),
            self.EIP_NUM_COMPL(),
            self.HW_PATCH_LVL(),
            self.HW_MINOR_VER(),
            self.HW_MAJOR_VER(),
            self.RESERVED28()
        )
    }
}
#[doc = "HW Version 1 TRNG Revision Number."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HWVER1(pub u32);
impl HWVER1 {
    #[doc = "7:0\\] The revision number of this module is Rev 2.0."]
    #[must_use]
    #[inline(always)]
    pub const fn REV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] The revision number of this module is Rev 2.0."]
    #[inline(always)]
    pub const fn set_REV(&mut self, val: u8) {
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
impl Default for HWVER1 {
    #[inline(always)]
    fn default() -> HWVER1 {
        HWVER1(0)
    }
}
impl core::fmt::Debug for HWVER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWVER1")
            .field("REV", &self.REV())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HWVER1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HWVER1 {{ REV: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.REV(),
            self.RESERVED8()
        )
    }
}
#[doc = "Interrupt Flag Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQFLAGCLR(pub u32);
impl IRQFLAGCLR {
    #[doc = "0:0\\] 1: Clear IRQFLAGSTAT.RDY."]
    #[must_use]
    #[inline(always)]
    pub const fn RDY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 1: Clear IRQFLAGSTAT.RDY."]
    #[inline(always)]
    pub const fn set_RDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 1: Clear IRQFLAGSTAT.SHUTDOWN_OVF."]
    #[must_use]
    #[inline(always)]
    pub const fn SHUTDOWN_OVF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 1: Clear IRQFLAGSTAT.SHUTDOWN_OVF."]
    #[inline(always)]
    pub const fn set_SHUTDOWN_OVF(&mut self, val: bool) {
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
impl Default for IRQFLAGCLR {
    #[inline(always)]
    fn default() -> IRQFLAGCLR {
        IRQFLAGCLR(0)
    }
}
impl core::fmt::Debug for IRQFLAGCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQFLAGCLR")
            .field("RDY", &self.RDY())
            .field("SHUTDOWN_OVF", &self.SHUTDOWN_OVF())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQFLAGCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQFLAGCLR {{ RDY: {=bool:?}, SHUTDOWN_OVF: {=bool:?}, RESERVED2: {=u32:?} }}",
            self.RDY(),
            self.SHUTDOWN_OVF(),
            self.RESERVED2()
        )
    }
}
#[doc = "Interrupt Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQFLAGMASK(pub u32);
impl IRQFLAGMASK {
    #[doc = "0:0\\] 1: Allow IRQFLAGSTAT.RDY to activate the interrupt from this module."]
    #[must_use]
    #[inline(always)]
    pub const fn RDY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 1: Allow IRQFLAGSTAT.RDY to activate the interrupt from this module."]
    #[inline(always)]
    pub const fn set_RDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 1: Allow IRQFLAGSTAT.SHUTDOWN_OVF to activate the interrupt from this module."]
    #[must_use]
    #[inline(always)]
    pub const fn SHUTDOWN_OVF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 1: Allow IRQFLAGSTAT.SHUTDOWN_OVF to activate the interrupt from this module."]
    #[inline(always)]
    pub const fn set_SHUTDOWN_OVF(&mut self, val: bool) {
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
impl Default for IRQFLAGMASK {
    #[inline(always)]
    fn default() -> IRQFLAGMASK {
        IRQFLAGMASK(0)
    }
}
impl core::fmt::Debug for IRQFLAGMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQFLAGMASK")
            .field("RDY", &self.RDY())
            .field("SHUTDOWN_OVF", &self.SHUTDOWN_OVF())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQFLAGMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQFLAGMASK {{ RDY: {=bool:?}, SHUTDOWN_OVF: {=bool:?}, RESERVED2: {=u32:?} }}",
            self.RDY(),
            self.SHUTDOWN_OVF(),
            self.RESERVED2()
        )
    }
}
#[doc = "Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQFLAGSTAT(pub u32);
impl IRQFLAGSTAT {
    #[doc = "0:0\\] 1: Data are available in OUT0 and OUT1. Acknowledging this state by writing '1' to IRQFLAGCLR.RDY clears this bit to '0'. If a new number is already available in the internal register of the TRNG, the number is directly clocked into the result register. In this case the status bit is asserted again, after one clock cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn RDY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 1: Data are available in OUT0 and OUT1. Acknowledging this state by writing '1' to IRQFLAGCLR.RDY clears this bit to '0'. If a new number is already available in the internal register of the TRNG, the number is directly clocked into the result register. In this case the status bit is asserted again, after one clock cycle."]
    #[inline(always)]
    pub const fn set_RDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 1: The number of FROs shut down (i.e. the number of '1' bits in the ALARMSTOP register) has exceeded the threshold set by ALARMCNT.SHUTDOWN_THR Writing '1' to IRQFLAGCLR.SHUTDOWN_OVF clears this bit to '0' again."]
    #[must_use]
    #[inline(always)]
    pub const fn SHUTDOWN_OVF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 1: The number of FROs shut down (i.e. the number of '1' bits in the ALARMSTOP register) has exceeded the threshold set by ALARMCNT.SHUTDOWN_THR Writing '1' to IRQFLAGCLR.SHUTDOWN_OVF clears this bit to '0' again."]
    #[inline(always)]
    pub const fn set_SHUTDOWN_OVF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "30:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "30:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 2usize)) | (((val as u32) & 0x1fff_ffff) << 2usize);
    }
    #[doc = "31:31\\] 1: Indicates that the TRNG is busy generating entropy or is in one of its test modes - clocks may not be turned off and the power supply voltage must be kept stable. 0: TRNG is idle and can be shut down."]
    #[must_use]
    #[inline(always)]
    pub const fn NEED_CLOCK(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] 1: Indicates that the TRNG is busy generating entropy or is in one of its test modes - clocks may not be turned off and the power supply voltage must be kept stable. 0: TRNG is idle and can be shut down."]
    #[inline(always)]
    pub const fn set_NEED_CLOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IRQFLAGSTAT {
    #[inline(always)]
    fn default() -> IRQFLAGSTAT {
        IRQFLAGSTAT(0)
    }
}
impl core::fmt::Debug for IRQFLAGSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQFLAGSTAT")
            .field("RDY", &self.RDY())
            .field("SHUTDOWN_OVF", &self.SHUTDOWN_OVF())
            .field("RESERVED2", &self.RESERVED2())
            .field("NEED_CLOCK", &self.NEED_CLOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQFLAGSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQFLAGSTAT {{ RDY: {=bool:?}, SHUTDOWN_OVF: {=bool:?}, RESERVED2: {=u32:?}, NEED_CLOCK: {=bool:?} }}",
            self.RDY(),
            self.SHUTDOWN_OVF(),
            self.RESERVED2(),
            self.NEED_CLOCK()
        )
    }
}
#[doc = "Interrupt Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSET(pub u32);
impl IRQSET {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RDY(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RDY(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IRQSET {
    #[inline(always)]
    fn default() -> IRQSET {
        IRQSET(0)
    }
}
impl core::fmt::Debug for IRQSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSET").field("RDY", &self.RDY()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IRQSET {{ RDY: {=u32:?} }}", self.RDY())
    }
}
#[doc = "Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSTAT(pub u32);
impl IRQSTAT {
    #[doc = "0:0\\] TRNG Interrupt status. OR'ed version of IRQFLAGSTAT.SHUTDOWN_OVF and IRQFLAGSTAT.RDY."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] TRNG Interrupt status. OR'ed version of IRQFLAGSTAT.SHUTDOWN_OVF and IRQFLAGSTAT.RDY."]
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
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
impl Default for IRQSTAT {
    #[inline(always)]
    fn default() -> IRQSTAT {
        IRQSTAT(0)
    }
}
impl core::fmt::Debug for IRQSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSTAT")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQSTAT {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "Interrupt Status After Masking."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSTATMASK(pub u32);
impl IRQSTATMASK {
    #[doc = "0:0\\] New random value available (result of IRQFLAGSTAT.RDY AND'ed with IRQFLAGMASK.RDY)."]
    #[must_use]
    #[inline(always)]
    pub const fn RDY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] New random value available (result of IRQFLAGSTAT.RDY AND'ed with IRQFLAGMASK.RDY)."]
    #[inline(always)]
    pub const fn set_RDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Shutdown Overflow (result of IRQFLAGSTAT.SHUTDOWN_OVF AND'ed with IRQFLAGMASK.SHUTDOWN_OVF)."]
    #[must_use]
    #[inline(always)]
    pub const fn SHUTDOWN_OVF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Shutdown Overflow (result of IRQFLAGSTAT.SHUTDOWN_OVF AND'ed with IRQFLAGMASK.SHUTDOWN_OVF)."]
    #[inline(always)]
    pub const fn set_SHUTDOWN_OVF(&mut self, val: bool) {
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
impl Default for IRQSTATMASK {
    #[inline(always)]
    fn default() -> IRQSTATMASK {
        IRQSTATMASK(0)
    }
}
impl core::fmt::Debug for IRQSTATMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSTATMASK")
            .field("RDY", &self.RDY())
            .field("SHUTDOWN_OVF", &self.SHUTDOWN_OVF())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSTATMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQSTATMASK {{ RDY: {=bool:?}, SHUTDOWN_OVF: {=bool:?}, RESERVED2: {=u32:?} }}",
            self.RDY(),
            self.SHUTDOWN_OVF(),
            self.RESERVED2()
        )
    }
}
#[doc = "LFSR Readout Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LFSR0(pub u32);
impl LFSR0 {
    #[doc = "31:0\\] Bits \\[31:0\\] of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn LFSR_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Bits \\[31:0\\] of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    pub const fn set_LFSR_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LFSR0 {
    #[inline(always)]
    fn default() -> LFSR0 {
        LFSR0(0)
    }
}
impl core::fmt::Debug for LFSR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFSR0")
            .field("LFSR_31_0", &self.LFSR_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LFSR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LFSR0 {{ LFSR_31_0: {=u32:?} }}", self.LFSR_31_0())
    }
}
#[doc = "LFSR Readout Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LFSR1(pub u32);
impl LFSR1 {
    #[doc = "31:0\\] Bits \\[63:32\\] of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn LFSR_63_32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Bits \\[63:32\\] of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    pub const fn set_LFSR_63_32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LFSR1 {
    #[inline(always)]
    fn default() -> LFSR1 {
        LFSR1(0)
    }
}
impl core::fmt::Debug for LFSR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFSR1")
            .field("LFSR_63_32", &self.LFSR_63_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LFSR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LFSR1 {{ LFSR_63_32: {=u32:?} }}", self.LFSR_63_32())
    }
}
#[doc = "LFSR Readout Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LFSR2(pub u32);
impl LFSR2 {
    #[doc = "16:0\\] Bits \\[80:64\\] of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn LFSR_80_64(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "16:0\\] Bits \\[80:64\\] of the main entropy accumulation LFSR. Register can only be accessed when CTL.TEST_MODE = 1. Register contents will be cleared to zero before access is enabled."]
    #[inline(always)]
    pub const fn set_LFSR_80_64(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
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
impl Default for LFSR2 {
    #[inline(always)]
    fn default() -> LFSR2 {
        LFSR2(0)
    }
}
impl core::fmt::Debug for LFSR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFSR2")
            .field("LFSR_80_64", &self.LFSR_80_64())
            .field("RESERVED17", &self.RESERVED17())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LFSR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LFSR2 {{ LFSR_80_64: {=u32:?}, RESERVED17: {=u16:?} }}",
            self.LFSR_80_64(),
            self.RESERVED17()
        )
    }
}
#[doc = "Random Number Lower Word Readout Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OUT0(pub u32);
impl OUT0 {
    #[doc = "31:0\\] LSW of 64- bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] LSW of 64- bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
    #[inline(always)]
    pub const fn set_VALUE_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for OUT0 {
    #[inline(always)]
    fn default() -> OUT0 {
        OUT0(0)
    }
}
impl core::fmt::Debug for OUT0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT0")
            .field("VALUE_31_0", &self.VALUE_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OUT0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OUT0 {{ VALUE_31_0: {=u32:?} }}", self.VALUE_31_0())
    }
}
#[doc = "Random Number Upper Word Readout Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OUT1(pub u32);
impl OUT1 {
    #[doc = "31:0\\] MSW of 64-bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE_63_32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] MSW of 64-bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
    #[inline(always)]
    pub const fn set_VALUE_63_32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for OUT1 {
    #[inline(always)]
    fn default() -> OUT1 {
        OUT1(0)
    }
}
impl core::fmt::Debug for OUT1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT1")
            .field("VALUE_63_32", &self.VALUE_63_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OUT1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OUT1 {{ VALUE_63_32: {=u32:?} }}", self.VALUE_63_32())
    }
}
#[doc = "SW Reset Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SWRESET(pub u32);
impl SWRESET {
    #[doc = "0:0\\] Write '1' to soft reset , reset will be low for 4-5 clock cycles. Poll to 0 for reset to be completed."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Write '1' to soft reset , reset will be low for 4-5 clock cycles. Poll to 0 for reset to be completed."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: bool) {
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
impl Default for SWRESET {
    #[inline(always)]
    fn default() -> SWRESET {
        SWRESET(0)
    }
}
impl core::fmt::Debug for SWRESET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWRESET")
            .field("RESET", &self.RESET())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SWRESET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SWRESET {{ RESET: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.RESET(),
            self.RESERVED1()
        )
    }
}

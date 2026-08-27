#[doc = "Pin Direction."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AIFDIRCFG(pub u32);
impl AIFDIRCFG {
    #[doc = "1:0\\] Configures the AD0 audio data pin usage: 0x3: Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn AD0(&self) -> super::vals::AD0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::AD0::from_bits(val as u8)
    }
    #[doc = "1:0\\] Configures the AD0 audio data pin usage: 0x3: Reserved."]
    #[inline(always)]
    pub const fn set_AD0(&mut self, val: super::vals::AD0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "5:4\\] Configures the AD1 audio data pin usage: 0x3: Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn AD1(&self) -> super::vals::AD1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::AD1::from_bits(val as u8)
    }
    #[doc = "5:4\\] Configures the AD1 audio data pin usage: 0x3: Reserved."]
    #[inline(always)]
    pub const fn set_AD1(&mut self, val: super::vals::AD1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for AIFDIRCFG {
    #[inline(always)]
    fn default() -> AIFDIRCFG {
        AIFDIRCFG(0)
    }
}
impl core::fmt::Debug for AIFDIRCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIFDIRCFG")
            .field("AD0", &self.AD0())
            .field("RESERVED2", &self.RESERVED2())
            .field("AD1", &self.AD1())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AIFDIRCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AIFDIRCFG {{ AD0: {:?}, RESERVED2: {=u8:?}, AD1: {:?}, RESERVED6: {=u32:?} }}",
            self.AD0(),
            self.RESERVED2(),
            self.AD1(),
            self.RESERVED6()
        )
    }
}
#[doc = "DMA Buffer Size Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AIFDMACFG(pub u32);
impl AIFDMACFG {
    #[doc = "7:0\\] Defines the length of the DMA buffer. Writing a non-zero value to this register field enables and initializes AIF. Note that before doing so, all other configuration must have been done, and AIFINPTRNEXT/AIFOUTPTRNEXT must have been loaded."]
    #[must_use]
    #[inline(always)]
    pub const fn END_FRAME_IDX(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Defines the length of the DMA buffer. Writing a non-zero value to this register field enables and initializes AIF. Note that before doing so, all other configuration must have been done, and AIFINPTRNEXT/AIFOUTPTRNEXT must have been loaded."]
    #[inline(always)]
    pub const fn set_END_FRAME_IDX(&mut self, val: u8) {
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
impl Default for AIFDMACFG {
    #[inline(always)]
    fn default() -> AIFDMACFG {
        AIFDMACFG(0)
    }
}
impl core::fmt::Debug for AIFDMACFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIFDMACFG")
            .field("END_FRAME_IDX", &self.END_FRAME_IDX())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AIFDMACFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AIFDMACFG {{ END_FRAME_IDX: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.END_FRAME_IDX(),
            self.RESERVED8()
        )
    }
}
#[doc = "Serial Interface Format Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AIFFMTCFG(pub u32);
impl AIFFMTCFG {
    #[doc = "4:0\\] Number of bits per word (8-24): In single-phase format, this is the exact number of bits per word. In dual-phase format, this is the maximum number of bits per word. Values below 8 and above 24 give undefined behavior. Data written to memory is always aligned to 16 or 24 bits as defined by MEM_LEN_24. Bit widths that differ from this alignment will either be truncated or zero padded."]
    #[must_use]
    #[inline(always)]
    pub const fn WORD_LEN(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "4:0\\] Number of bits per word (8-24): In single-phase format, this is the exact number of bits per word. In dual-phase format, this is the maximum number of bits per word. Values below 8 and above 24 give undefined behavior. Data written to memory is always aligned to 16 or 24 bits as defined by MEM_LEN_24. Bit widths that differ from this alignment will either be truncated or zero padded."]
    #[inline(always)]
    pub const fn set_WORD_LEN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "5:5\\] Selects dual- or single-phase format. 0: Single-phase: DSP format 1: Dual-phase: I2S, LJF and RJF formats."]
    #[must_use]
    #[inline(always)]
    pub const fn DUAL_PHASE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Selects dual- or single-phase format. 0: Single-phase: DSP format 1: Dual-phase: I2S, LJF and RJF formats."]
    #[inline(always)]
    pub const fn set_DUAL_PHASE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] On the serial audio interface, data (and wclk) is sampled and clocked out on opposite edges of BCLK."]
    #[must_use]
    #[inline(always)]
    pub const fn SMPL_EDGE(&self) -> super::vals::SMPL_EDGE {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SMPL_EDGE::from_bits(val as u8)
    }
    #[doc = "6:6\\] On the serial audio interface, data (and wclk) is sampled and clocked out on opposite edges of BCLK."]
    #[inline(always)]
    pub const fn set_SMPL_EDGE(&mut self, val: super::vals::SMPL_EDGE) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] The size of each word stored to or loaded from memory:."]
    #[must_use]
    #[inline(always)]
    pub const fn MEM_LEN_24(&self) -> super::vals::MEM_LEN_24 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::MEM_LEN_24::from_bits(val as u8)
    }
    #[doc = "7:7\\] The size of each word stored to or loaded from memory:."]
    #[inline(always)]
    pub const fn set_MEM_LEN_24(&mut self, val: super::vals::MEM_LEN_24) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "15:8\\] The number of BCLK periods between a WCLK edge and MSB of the first word in a phase: 0x00: LJF and DSP format 0x01: I2S and DSP format 0x02: RJF format ... 0xFF: RJF format Note: When 0, MSB of the next word will be output in the idle period between LSB of the previous word and the start of the next word. Otherwise logical 0 will be output until the data delay has expired."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA_DELAY(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] The number of BCLK periods between a WCLK edge and MSB of the first word in a phase: 0x00: LJF and DSP format 0x01: I2S and DSP format 0x02: RJF format ... 0xFF: RJF format Note: When 0, MSB of the next word will be output in the idle period between LSB of the previous word and the start of the next word. Otherwise logical 0 will be output until the data delay has expired."]
    #[inline(always)]
    pub const fn set_DATA_DELAY(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
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
impl Default for AIFFMTCFG {
    #[inline(always)]
    fn default() -> AIFFMTCFG {
        AIFFMTCFG(0)
    }
}
impl core::fmt::Debug for AIFFMTCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIFFMTCFG")
            .field("WORD_LEN", &self.WORD_LEN())
            .field("DUAL_PHASE", &self.DUAL_PHASE())
            .field("SMPL_EDGE", &self.SMPL_EDGE())
            .field("MEM_LEN_24", &self.MEM_LEN_24())
            .field("DATA_DELAY", &self.DATA_DELAY())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AIFFMTCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AIFFMTCFG {{ WORD_LEN: {=u8:?}, DUAL_PHASE: {=bool:?}, SMPL_EDGE: {:?}, MEM_LEN_24: {:?}, DATA_DELAY: {=u8:?}, RESERVED16: {=u16:?} }}",
            self.WORD_LEN(),
            self.DUAL_PHASE(),
            self.SMPL_EDGE(),
            self.MEM_LEN_24(),
            self.DATA_DELAY(),
            self.RESERVED16()
        )
    }
}
#[doc = "DMA Input Buffer Current Pointer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AIFINPTR(pub u32);
impl AIFINPTR {
    #[doc = "31:0\\] Value of the DMA input buffer pointer currently used by the DMA controller. Incremented by 1 (byte) or 2 (word) for each AHB access."]
    #[must_use]
    #[inline(always)]
    pub const fn PTR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Value of the DMA input buffer pointer currently used by the DMA controller. Incremented by 1 (byte) or 2 (word) for each AHB access."]
    #[inline(always)]
    pub const fn set_PTR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AIFINPTR {
    #[inline(always)]
    fn default() -> AIFINPTR {
        AIFINPTR(0)
    }
}
impl core::fmt::Debug for AIFINPTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIFINPTR")
            .field("PTR", &self.PTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AIFINPTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AIFINPTR {{ PTR: {=u32:?} }}", self.PTR())
    }
}
#[doc = "DMA Input Buffer Next Pointer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AIFINPTRNEXT(pub u32);
impl AIFINPTRNEXT {
    #[doc = "31:0\\] Pointer to the first byte in the next DMA input buffer. The read value equals the last written value until the currently used DMA input buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_IN. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all input pins will be disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn PTR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Pointer to the first byte in the next DMA input buffer. The read value equals the last written value until the currently used DMA input buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_IN. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all input pins will be disabled."]
    #[inline(always)]
    pub const fn set_PTR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AIFINPTRNEXT {
    #[inline(always)]
    fn default() -> AIFINPTRNEXT {
        AIFINPTRNEXT(0)
    }
}
impl core::fmt::Debug for AIFINPTRNEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIFINPTRNEXT")
            .field("PTR", &self.PTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AIFINPTRNEXT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AIFINPTRNEXT {{ PTR: {=u32:?} }}", self.PTR())
    }
}
#[doc = "DMA Output Buffer Current Pointer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AIFOUTPTR(pub u32);
impl AIFOUTPTR {
    #[doc = "31:0\\] Value of the DMA output buffer pointer currently used by the DMA controller Incremented by 1 (byte) or 2 (word) for each AHB access."]
    #[must_use]
    #[inline(always)]
    pub const fn PTR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Value of the DMA output buffer pointer currently used by the DMA controller Incremented by 1 (byte) or 2 (word) for each AHB access."]
    #[inline(always)]
    pub const fn set_PTR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AIFOUTPTR {
    #[inline(always)]
    fn default() -> AIFOUTPTR {
        AIFOUTPTR(0)
    }
}
impl core::fmt::Debug for AIFOUTPTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIFOUTPTR")
            .field("PTR", &self.PTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AIFOUTPTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AIFOUTPTR {{ PTR: {=u32:?} }}", self.PTR())
    }
}
#[doc = "DMA Output Buffer Next Pointer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AIFOUTPTRNEXT(pub u32);
impl AIFOUTPTRNEXT {
    #[doc = "31:0\\] Pointer to the first byte in the next DMA output buffer. The read value equals the last written value until the currently used DMA output buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_OUT. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. At this time, the first two samples will be fetched from memory. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all output pins will be disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn PTR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Pointer to the first byte in the next DMA output buffer. The read value equals the last written value until the currently used DMA output buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_OUT. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. At this time, the first two samples will be fetched from memory. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all output pins will be disabled."]
    #[inline(always)]
    pub const fn set_PTR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AIFOUTPTRNEXT {
    #[inline(always)]
    fn default() -> AIFOUTPTRNEXT {
        AIFOUTPTRNEXT(0)
    }
}
impl core::fmt::Debug for AIFOUTPTRNEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIFOUTPTRNEXT")
            .field("PTR", &self.PTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AIFOUTPTRNEXT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AIFOUTPTRNEXT {{ PTR: {=u32:?} }}", self.PTR())
    }
}
#[doc = "Audio Interface PWM Debug Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AIFPWMVALUE(pub u32);
impl AIFPWMVALUE {
    #[doc = "15:0\\] The value written to this register determines the width of the active high PWM pulse (pwm_debug), which starts together with MSB of the first output word in a DMA buffer: 0x0000: Constant low 0x0001: Width of the pulse (number of BCLK cycles, here 1). ... 0xFFFE: Width of the pulse (number of BCLK cycles, here 65534). 0xFFFF: Constant high."]
    #[must_use]
    #[inline(always)]
    pub const fn PULSE_WIDTH(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] The value written to this register determines the width of the active high PWM pulse (pwm_debug), which starts together with MSB of the first output word in a DMA buffer: 0x0000: Constant low 0x0001: Width of the pulse (number of BCLK cycles, here 1). ... 0xFFFE: Width of the pulse (number of BCLK cycles, here 65534). 0xFFFF: Constant high."]
    #[inline(always)]
    pub const fn set_PULSE_WIDTH(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for AIFPWMVALUE {
    #[inline(always)]
    fn default() -> AIFPWMVALUE {
        AIFPWMVALUE(0)
    }
}
impl core::fmt::Debug for AIFPWMVALUE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIFPWMVALUE")
            .field("PULSE_WIDTH", &self.PULSE_WIDTH())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AIFPWMVALUE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AIFPWMVALUE {{ PULSE_WIDTH: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.PULSE_WIDTH(),
            self.RESERVED16()
        )
    }
}
#[doc = "WCLK Source Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AIFWCLKSRC(pub u32);
impl AIFWCLKSRC {
    #[doc = "1:0\\] Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC."]
    #[must_use]
    #[inline(always)]
    pub const fn WCLK_SRC(&self) -> super::vals::WCLK_SRC {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::WCLK_SRC::from_bits(val as u8)
    }
    #[doc = "1:0\\] Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC."]
    #[inline(always)]
    pub const fn set_WCLK_SRC(&mut self, val: super::vals::WCLK_SRC) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "2:2\\] Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted."]
    #[must_use]
    #[inline(always)]
    pub const fn WCLK_INV(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted."]
    #[inline(always)]
    pub const fn set_WCLK_INV(&mut self, val: bool) {
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
impl Default for AIFWCLKSRC {
    #[inline(always)]
    fn default() -> AIFWCLKSRC {
        AIFWCLKSRC(0)
    }
}
impl core::fmt::Debug for AIFWCLKSRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIFWCLKSRC")
            .field("WCLK_SRC", &self.WCLK_SRC())
            .field("WCLK_INV", &self.WCLK_INV())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AIFWCLKSRC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AIFWCLKSRC {{ WCLK_SRC: {:?}, WCLK_INV: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.WCLK_SRC(),
            self.WCLK_INV(),
            self.RESERVED3()
        )
    }
}
#[doc = "Word Selection Bit Mask for Pin 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AIFWMASK0(pub u32);
impl AIFWMASK0 {
    #[doc = "7:0\\] Bit-mask indicating valid channels in a frame on AD0. In single-phase mode, each bit represents one channel, starting with LSB for the first word in the frame. A frame can contain up to 8 channels. Channels that are not included in the mask will not be sampled and stored in memory, and clocked out as '0'. In dual-phase mode, only the two LSBs are considered. For a stereo configuration, set both bits. For a mono configuration, set bit 0 only. In mono mode, only channel 0 will be sampled and stored to memory, and channel 0 will be repeated when clocked out. In mono mode, only channel 0 will be sampled and stored to memory, and channel 0 will be repeated in the second phase when clocked out. If all bits are zero, no input words will be stored to memory, and the output data lines will be constant '0'. This can be utilized when PWM debug output is desired without any actively used output pins."]
    #[must_use]
    #[inline(always)]
    pub const fn MASK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Bit-mask indicating valid channels in a frame on AD0. In single-phase mode, each bit represents one channel, starting with LSB for the first word in the frame. A frame can contain up to 8 channels. Channels that are not included in the mask will not be sampled and stored in memory, and clocked out as '0'. In dual-phase mode, only the two LSBs are considered. For a stereo configuration, set both bits. For a mono configuration, set bit 0 only. In mono mode, only channel 0 will be sampled and stored to memory, and channel 0 will be repeated when clocked out. In mono mode, only channel 0 will be sampled and stored to memory, and channel 0 will be repeated in the second phase when clocked out. If all bits are zero, no input words will be stored to memory, and the output data lines will be constant '0'. This can be utilized when PWM debug output is desired without any actively used output pins."]
    #[inline(always)]
    pub const fn set_MASK(&mut self, val: u8) {
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
impl Default for AIFWMASK0 {
    #[inline(always)]
    fn default() -> AIFWMASK0 {
        AIFWMASK0(0)
    }
}
impl core::fmt::Debug for AIFWMASK0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIFWMASK0")
            .field("MASK", &self.MASK())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AIFWMASK0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AIFWMASK0 {{ MASK: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.MASK(),
            self.RESERVED8()
        )
    }
}
#[doc = "Word Selection Bit Mask for Pin 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AIFWMASK1(pub u32);
impl AIFWMASK1 {
    #[doc = "7:0\\] Bit-mask indicating valid channels in a frame on AD1. In single-phase mode, each bit represents one channel, starting with LSB for the first word in the frame. A frame can contain up to 8 channels. Channels that are not included in the mask will not be sampled and stored in memory, and clocked out as '0'. In dual-phase mode, only the two LSBs are considered. For a stereo configuration, set both bits. For a mono configuration, set bit 0 only. In mono mode, only channel 0 will be sampled and stored to memory, and channel 0 will be repeated when clocked out. In mono mode, only channel 0 will be sampled and stored to memory, and channel 0 will be repeated in the second phase when clocked out. If all bits are zero, no input words will be stored to memory, and the output data lines will be constant '0'. This can be utilized when PWM debug output is desired without any actively used output pins."]
    #[must_use]
    #[inline(always)]
    pub const fn MASK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Bit-mask indicating valid channels in a frame on AD1. In single-phase mode, each bit represents one channel, starting with LSB for the first word in the frame. A frame can contain up to 8 channels. Channels that are not included in the mask will not be sampled and stored in memory, and clocked out as '0'. In dual-phase mode, only the two LSBs are considered. For a stereo configuration, set both bits. For a mono configuration, set bit 0 only. In mono mode, only channel 0 will be sampled and stored to memory, and channel 0 will be repeated when clocked out. In mono mode, only channel 0 will be sampled and stored to memory, and channel 0 will be repeated in the second phase when clocked out. If all bits are zero, no input words will be stored to memory, and the output data lines will be constant '0'. This can be utilized when PWM debug output is desired without any actively used output pins."]
    #[inline(always)]
    pub const fn set_MASK(&mut self, val: u8) {
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
impl Default for AIFWMASK1 {
    #[inline(always)]
    fn default() -> AIFWMASK1 {
        AIFWMASK1(0)
    }
}
impl core::fmt::Debug for AIFWMASK1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIFWMASK1")
            .field("MASK", &self.MASK())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AIFWMASK1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AIFWMASK1 {{ MASK: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.MASK(),
            self.RESERVED8()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AIFWMASK2(pub u32);
impl AIFWMASK2 {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AIFWMASK2 {
    #[inline(always)]
    fn default() -> AIFWMASK2 {
        AIFWMASK2(0)
    }
}
impl core::fmt::Debug for AIFWMASK2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIFWMASK2")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AIFWMASK2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AIFWMASK2 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Interrupt Clear Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQCLR(pub u32);
impl IRQCLR {
    #[doc = "0:0\\] 1: Clears the interrupt of IRQFLAGS.PTR_ERR (unless a set criteria was given at the same time in which the clear will be ignored)."]
    #[must_use]
    #[inline(always)]
    pub const fn PTR_ERR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 1: Clears the interrupt of IRQFLAGS.PTR_ERR (unless a set criteria was given at the same time in which the clear will be ignored)."]
    #[inline(always)]
    pub const fn set_PTR_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 1: Clears the interrupt of IRQFLAGS.WCLK_ERR (unless a set criteria was given at the same time in which the clear will be ignored)."]
    #[must_use]
    #[inline(always)]
    pub const fn WCLK_ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 1: Clears the interrupt of IRQFLAGS.WCLK_ERR (unless a set criteria was given at the same time in which the clear will be ignored)."]
    #[inline(always)]
    pub const fn set_WCLK_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] 1: Clears the interrupt of IRQFLAGS.BUS_ERR (unless a set criteria was given at the same time in which the clear will be ignored)."]
    #[must_use]
    #[inline(always)]
    pub const fn BUS_ERR(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] 1: Clears the interrupt of IRQFLAGS.BUS_ERR (unless a set criteria was given at the same time in which the clear will be ignored)."]
    #[inline(always)]
    pub const fn set_BUS_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] 1: Clears the interrupt of IRQFLAGS.WCLK_TIMEOUT (unless a set criteria was given at the same time in which the clear will be ignored)."]
    #[must_use]
    #[inline(always)]
    pub const fn WCLK_TIMEOUT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] 1: Clears the interrupt of IRQFLAGS.WCLK_TIMEOUT (unless a set criteria was given at the same time in which the clear will be ignored)."]
    #[inline(always)]
    pub const fn set_WCLK_TIMEOUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] 1: Clears the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a set criteria was given at the same time in which the clear will be ignored)."]
    #[must_use]
    #[inline(always)]
    pub const fn AIF_DMA_OUT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] 1: Clears the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a set criteria was given at the same time in which the clear will be ignored)."]
    #[inline(always)]
    pub const fn set_AIF_DMA_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] 1: Clears the interrupt of IRQFLAGS.AIF_DMA_IN (unless a set criteria was given at the same time in which the clear will be ignored)."]
    #[must_use]
    #[inline(always)]
    pub const fn AIF_DMA_IN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] 1: Clears the interrupt of IRQFLAGS.AIF_DMA_IN (unless a set criteria was given at the same time in which the clear will be ignored)."]
    #[inline(always)]
    pub const fn set_AIF_DMA_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for IRQCLR {
    #[inline(always)]
    fn default() -> IRQCLR {
        IRQCLR(0)
    }
}
impl core::fmt::Debug for IRQCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQCLR")
            .field("PTR_ERR", &self.PTR_ERR())
            .field("WCLK_ERR", &self.WCLK_ERR())
            .field("BUS_ERR", &self.BUS_ERR())
            .field("WCLK_TIMEOUT", &self.WCLK_TIMEOUT())
            .field("AIF_DMA_OUT", &self.AIF_DMA_OUT())
            .field("AIF_DMA_IN", &self.AIF_DMA_IN())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQCLR {{ PTR_ERR: {=bool:?}, WCLK_ERR: {=bool:?}, BUS_ERR: {=bool:?}, WCLK_TIMEOUT: {=bool:?}, AIF_DMA_OUT: {=bool:?}, AIF_DMA_IN: {=bool:?}, RESERVED6: {=u32:?} }}",
            self.PTR_ERR(),
            self.WCLK_ERR(),
            self.BUS_ERR(),
            self.WCLK_TIMEOUT(),
            self.AIF_DMA_OUT(),
            self.AIF_DMA_IN(),
            self.RESERVED6()
        )
    }
}
#[doc = "Raw Interrupt Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQFLAGS(pub u32);
impl IRQFLAGS {
    #[doc = "0:0\\] Set when AIFINPTRNEXT or AIFOUTPTRNEXT has not been loaded with the next block address in time. This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.PTR_ERR)."]
    #[must_use]
    #[inline(always)]
    pub const fn PTR_ERR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Set when AIFINPTRNEXT or AIFOUTPTRNEXT has not been loaded with the next block address in time. This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.PTR_ERR)."]
    #[inline(always)]
    pub const fn set_PTR_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Set when: - An unexpected WCLK edge occurs during the data delay period of a phase. Note unexpected WCLK edges during the word and idle periods of the phase are not detected. - In dual-phase mode, when two WCLK edges are less than 4 BCLK cycles apart. - In single-phase mode, when a WCLK pulse occurs before the last channel. This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.WCLK_ERR)."]
    #[must_use]
    #[inline(always)]
    pub const fn WCLK_ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Set when: - An unexpected WCLK edge occurs during the data delay period of a phase. Note unexpected WCLK edges during the word and idle periods of the phase are not detected. - In dual-phase mode, when two WCLK edges are less than 4 BCLK cycles apart. - In single-phase mode, when a WCLK pulse occurs before the last channel. This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.WCLK_ERR)."]
    #[inline(always)]
    pub const fn set_WCLK_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Set when a DMA operation is not completed in time (that is audio output buffer underflow, or audio input buffer overflow). This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.BUS_ERR). Note that DMA initiated transactions to illegal addresses will not trigger an interrupt. The response to such transactions is undefined."]
    #[must_use]
    #[inline(always)]
    pub const fn BUS_ERR(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Set when a DMA operation is not completed in time (that is audio output buffer underflow, or audio input buffer overflow). This error requires a complete restart since word synchronization has been lost. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.BUS_ERR). Note that DMA initiated transactions to illegal addresses will not trigger an interrupt. The response to such transactions is undefined."]
    #[inline(always)]
    pub const fn set_BUS_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Set when the sample stamp generator does not detect a positive WCLK edge for 65535 clk periods. This signalizes that the internal or external BCLK and WCLK generator source has been disabled. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.WCLK_TIMEOUT)."]
    #[must_use]
    #[inline(always)]
    pub const fn WCLK_TIMEOUT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Set when the sample stamp generator does not detect a positive WCLK edge for 65535 clk periods. This signalizes that the internal or external BCLK and WCLK generator source has been disabled. The bit is sticky and may only be cleared by software (by writing '1' to IRQCLR.WCLK_TIMEOUT)."]
    #[inline(always)]
    pub const fn set_WCLK_TIMEOUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Set when condition for this bit field event occurs (auto cleared when output pointer is updated - AIFOUTPTRNEXT), see description of AIFOUTPTRNEXT register for details."]
    #[must_use]
    #[inline(always)]
    pub const fn AIF_DMA_OUT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Set when condition for this bit field event occurs (auto cleared when output pointer is updated - AIFOUTPTRNEXT), see description of AIFOUTPTRNEXT register for details."]
    #[inline(always)]
    pub const fn set_AIF_DMA_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Set when condition for this bit field event occurs (auto cleared when input pointer is updated - AIFINPTRNEXT), see description of AIFINPTRNEXT register for details."]
    #[must_use]
    #[inline(always)]
    pub const fn AIF_DMA_IN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Set when condition for this bit field event occurs (auto cleared when input pointer is updated - AIFINPTRNEXT), see description of AIFINPTRNEXT register for details."]
    #[inline(always)]
    pub const fn set_AIF_DMA_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for IRQFLAGS {
    #[inline(always)]
    fn default() -> IRQFLAGS {
        IRQFLAGS(0)
    }
}
impl core::fmt::Debug for IRQFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQFLAGS")
            .field("PTR_ERR", &self.PTR_ERR())
            .field("WCLK_ERR", &self.WCLK_ERR())
            .field("BUS_ERR", &self.BUS_ERR())
            .field("WCLK_TIMEOUT", &self.WCLK_TIMEOUT())
            .field("AIF_DMA_OUT", &self.AIF_DMA_OUT())
            .field("AIF_DMA_IN", &self.AIF_DMA_IN())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQFLAGS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQFLAGS {{ PTR_ERR: {=bool:?}, WCLK_ERR: {=bool:?}, BUS_ERR: {=bool:?}, WCLK_TIMEOUT: {=bool:?}, AIF_DMA_OUT: {=bool:?}, AIF_DMA_IN: {=bool:?}, RESERVED6: {=u32:?} }}",
            self.PTR_ERR(),
            self.WCLK_ERR(),
            self.BUS_ERR(),
            self.WCLK_TIMEOUT(),
            self.AIF_DMA_OUT(),
            self.AIF_DMA_IN(),
            self.RESERVED6()
        )
    }
}
#[doc = "Interrupt Mask Register Selects mask states of the flags in IRQFLAGS that contribute to the I2S_IRQ event."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQMASK(pub u32);
impl IRQMASK {
    #[doc = "0:0\\] IRQFLAGS.PTR_ERR interrupt mask. 0: Disable 1: Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PTR_ERR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] IRQFLAGS.PTR_ERR interrupt mask. 0: Disable 1: Enable."]
    #[inline(always)]
    pub const fn set_PTR_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] IRQFLAGS.WCLK_ERR interrupt mask 0: Disable 1: Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn WCLK_ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] IRQFLAGS.WCLK_ERR interrupt mask 0: Disable 1: Enable."]
    #[inline(always)]
    pub const fn set_WCLK_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] IRQFLAGS.BUS_ERR interrupt mask 0: Disable 1: Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BUS_ERR(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] IRQFLAGS.BUS_ERR interrupt mask 0: Disable 1: Enable."]
    #[inline(always)]
    pub const fn set_BUS_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] IRQFLAGS.WCLK_TIMEOUT interrupt mask 0: Disable 1: Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn WCLK_TIMEOUT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] IRQFLAGS.WCLK_TIMEOUT interrupt mask 0: Disable 1: Enable."]
    #[inline(always)]
    pub const fn set_WCLK_TIMEOUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] IRQFLAGS.AIF_DMA_OUT interrupt mask 0: Disable 1: Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn AIF_DMA_OUT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] IRQFLAGS.AIF_DMA_OUT interrupt mask 0: Disable 1: Enable."]
    #[inline(always)]
    pub const fn set_AIF_DMA_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] IRQFLAGS.AIF_DMA_IN interrupt mask 0: Disable 1: Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn AIF_DMA_IN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] IRQFLAGS.AIF_DMA_IN interrupt mask 0: Disable 1: Enable."]
    #[inline(always)]
    pub const fn set_AIF_DMA_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for IRQMASK {
    #[inline(always)]
    fn default() -> IRQMASK {
        IRQMASK(0)
    }
}
impl core::fmt::Debug for IRQMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQMASK")
            .field("PTR_ERR", &self.PTR_ERR())
            .field("WCLK_ERR", &self.WCLK_ERR())
            .field("BUS_ERR", &self.BUS_ERR())
            .field("WCLK_TIMEOUT", &self.WCLK_TIMEOUT())
            .field("AIF_DMA_OUT", &self.AIF_DMA_OUT())
            .field("AIF_DMA_IN", &self.AIF_DMA_IN())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQMASK {{ PTR_ERR: {=bool:?}, WCLK_ERR: {=bool:?}, BUS_ERR: {=bool:?}, WCLK_TIMEOUT: {=bool:?}, AIF_DMA_OUT: {=bool:?}, AIF_DMA_IN: {=bool:?}, RESERVED6: {=u32:?} }}",
            self.PTR_ERR(),
            self.WCLK_ERR(),
            self.BUS_ERR(),
            self.WCLK_TIMEOUT(),
            self.AIF_DMA_OUT(),
            self.AIF_DMA_IN(),
            self.RESERVED6()
        )
    }
}
#[doc = "Interrupt Set Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSET(pub u32);
impl IRQSET {
    #[doc = "0:0\\] 1: Sets the interrupt of IRQFLAGS.PTR_ERR."]
    #[must_use]
    #[inline(always)]
    pub const fn PTR_ERR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 1: Sets the interrupt of IRQFLAGS.PTR_ERR."]
    #[inline(always)]
    pub const fn set_PTR_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 1: Sets the interrupt of IRQFLAGS.WCLK_ERR."]
    #[must_use]
    #[inline(always)]
    pub const fn WCLK_ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 1: Sets the interrupt of IRQFLAGS.WCLK_ERR."]
    #[inline(always)]
    pub const fn set_WCLK_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] 1: Sets the interrupt of IRQFLAGS.BUS_ERR."]
    #[must_use]
    #[inline(always)]
    pub const fn BUS_ERR(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] 1: Sets the interrupt of IRQFLAGS.BUS_ERR."]
    #[inline(always)]
    pub const fn set_BUS_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] 1: Sets the interrupt of IRQFLAGS.WCLK_TIMEOUT."]
    #[must_use]
    #[inline(always)]
    pub const fn WCLK_TIMEOUT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] 1: Sets the interrupt of IRQFLAGS.WCLK_TIMEOUT."]
    #[inline(always)]
    pub const fn set_WCLK_TIMEOUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] 1: Sets the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a auto clear criteria was given at the same time, in which the set will be ignored)."]
    #[must_use]
    #[inline(always)]
    pub const fn AIF_DMA_OUT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] 1: Sets the interrupt of IRQFLAGS.AIF_DMA_OUT (unless a auto clear criteria was given at the same time, in which the set will be ignored)."]
    #[inline(always)]
    pub const fn set_AIF_DMA_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] 1: Sets the interrupt of IRQFLAGS.AIF_DMA_IN (unless a auto clear criteria was given at the same time, in which the set will be ignored)."]
    #[must_use]
    #[inline(always)]
    pub const fn AIF_DMA_IN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] 1: Sets the interrupt of IRQFLAGS.AIF_DMA_IN (unless a auto clear criteria was given at the same time, in which the set will be ignored)."]
    #[inline(always)]
    pub const fn set_AIF_DMA_IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
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
        f.debug_struct("IRQSET")
            .field("PTR_ERR", &self.PTR_ERR())
            .field("WCLK_ERR", &self.WCLK_ERR())
            .field("BUS_ERR", &self.BUS_ERR())
            .field("WCLK_TIMEOUT", &self.WCLK_TIMEOUT())
            .field("AIF_DMA_OUT", &self.AIF_DMA_OUT())
            .field("AIF_DMA_IN", &self.AIF_DMA_IN())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQSET {{ PTR_ERR: {=bool:?}, WCLK_ERR: {=bool:?}, BUS_ERR: {=bool:?}, WCLK_TIMEOUT: {=bool:?}, AIF_DMA_OUT: {=bool:?}, AIF_DMA_IN: {=bool:?}, RESERVED6: {=u32:?} }}",
            self.PTR_ERR(),
            self.WCLK_ERR(),
            self.BUS_ERR(),
            self.WCLK_TIMEOUT(),
            self.AIF_DMA_OUT(),
            self.AIF_DMA_IN(),
            self.RESERVED6()
        )
    }
}
#[doc = "Samplestamp Generator Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STMPCTL(pub u32);
impl STMPCTL {
    #[doc = "0:0\\] Enables the samplestamp generator. The samplestamp generator must only be enabled after it has been properly configured. When cleared, all samplestamp generator counters and capture values are cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn STMP_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Enables the samplestamp generator. The samplestamp generator must only be enabled after it has been properly configured. When cleared, all samplestamp generator counters and capture values are cleared."]
    #[inline(always)]
    pub const fn set_STMP_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Low until the input pins are ready to be started by the samplestamp generator. When started (that is STMPINTRIG equals the WCLK counter) the bit goes back low."]
    #[must_use]
    #[inline(always)]
    pub const fn IN_RDY(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Low until the input pins are ready to be started by the samplestamp generator. When started (that is STMPINTRIG equals the WCLK counter) the bit goes back low."]
    #[inline(always)]
    pub const fn set_IN_RDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Low until the output pins are ready to be started by the samplestamp generator. When started (that is STMPOUTTRIG equals the WCLK counter) the bit goes back low."]
    #[must_use]
    #[inline(always)]
    pub const fn OUT_RDY(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Low until the output pins are ready to be started by the samplestamp generator. When started (that is STMPOUTTRIG equals the WCLK counter) the bit goes back low."]
    #[inline(always)]
    pub const fn set_OUT_RDY(&mut self, val: bool) {
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
impl Default for STMPCTL {
    #[inline(always)]
    fn default() -> STMPCTL {
        STMPCTL(0)
    }
}
impl core::fmt::Debug for STMPCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STMPCTL")
            .field("STMP_EN", &self.STMP_EN())
            .field("IN_RDY", &self.IN_RDY())
            .field("OUT_RDY", &self.OUT_RDY())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STMPCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STMPCTL {{ STMP_EN: {=bool:?}, IN_RDY: {=bool:?}, OUT_RDY: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.STMP_EN(),
            self.IN_RDY(),
            self.OUT_RDY(),
            self.RESERVED3()
        )
    }
}
#[doc = "WCLK Counter Trigger Value for Input Pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STMPINTRIG(pub u32);
impl STMPINTRIG {
    #[doc = "15:0\\] Compare value used to start the incoming audio streams. This bit field shall equal the WCLK counter value during the WCLK period in which the first input word(s) are sampled and stored to memory (that is the sample at the start of the very first DMA input buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as inputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and at least 32 BCLK cycle ticks have happened. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
    #[must_use]
    #[inline(always)]
    pub const fn IN_START_WCNT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Compare value used to start the incoming audio streams. This bit field shall equal the WCLK counter value during the WCLK period in which the first input word(s) are sampled and stored to memory (that is the sample at the start of the very first DMA input buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as inputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and at least 32 BCLK cycle ticks have happened. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
    #[inline(always)]
    pub const fn set_IN_START_WCNT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for STMPINTRIG {
    #[inline(always)]
    fn default() -> STMPINTRIG {
        STMPINTRIG(0)
    }
}
impl core::fmt::Debug for STMPINTRIG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STMPINTRIG")
            .field("IN_START_WCNT", &self.IN_START_WCNT())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STMPINTRIG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STMPINTRIG {{ IN_START_WCNT: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.IN_START_WCNT(),
            self.RESERVED16()
        )
    }
}
#[doc = "WCLK Counter Trigger Value for Output Pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STMPOUTTRIG(pub u32);
impl STMPOUTTRIG {
    #[doc = "15:0\\] Compare value used to start the outgoing audio streams. This bit field must equal the WCLK counter value during the WCLK period in which the first output word(s) read from memory are clocked out (that is the sample at the start of the very first DMA output buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as outputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and 32 BCLK cycle ticks have happened. - 2 samples have been preloaded from memory (examine the AIFOUTPTR register if necessary). Note: The memory read access is only performed when required, that is channels 0/1 must be selected in AIFWMASK0/AIFWMASK1. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
    #[must_use]
    #[inline(always)]
    pub const fn OUT_START_WCNT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Compare value used to start the outgoing audio streams. This bit field must equal the WCLK counter value during the WCLK period in which the first output word(s) read from memory are clocked out (that is the sample at the start of the very first DMA output buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as outputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and 32 BCLK cycle ticks have happened. - 2 samples have been preloaded from memory (examine the AIFOUTPTR register if necessary). Note: The memory read access is only performed when required, that is channels 0/1 must be selected in AIFWMASK0/AIFWMASK1. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
    #[inline(always)]
    pub const fn set_OUT_START_WCNT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for STMPOUTTRIG {
    #[inline(always)]
    fn default() -> STMPOUTTRIG {
        STMPOUTTRIG(0)
    }
}
impl core::fmt::Debug for STMPOUTTRIG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STMPOUTTRIG")
            .field("OUT_START_WCNT", &self.OUT_START_WCNT())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STMPOUTTRIG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STMPOUTTRIG {{ OUT_START_WCNT: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.OUT_START_WCNT(),
            self.RESERVED16()
        )
    }
}
#[doc = "WCLK Counter Add Operation."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STMPWADD(pub u32);
impl STMPWADD {
    #[doc = "15:0\\] WCLK counter modification: Adds the written value to the running WCLK counter. If a positive edge of WCLK occurs at the same time as the operation, this will be taken into account. To add a negative value, write \"STMPWPER.VALUE - value\"."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE_INC(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] WCLK counter modification: Adds the written value to the running WCLK counter. If a positive edge of WCLK occurs at the same time as the operation, this will be taken into account. To add a negative value, write \"STMPWPER.VALUE - value\"."]
    #[inline(always)]
    pub const fn set_VALUE_INC(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for STMPWADD {
    #[inline(always)]
    fn default() -> STMPWADD {
        STMPWADD(0)
    }
}
impl core::fmt::Debug for STMPWADD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STMPWADD")
            .field("VALUE_INC", &self.VALUE_INC())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STMPWADD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STMPWADD {{ VALUE_INC: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.VALUE_INC(),
            self.RESERVED16()
        )
    }
}
#[doc = "Current Value of WCNT."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STMPWCNT(pub u32);
impl STMPWCNT {
    #[doc = "15:0\\] Current value of the WCLK counter."]
    #[must_use]
    #[inline(always)]
    pub const fn CURR_VALUE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Current value of the WCLK counter."]
    #[inline(always)]
    pub const fn set_CURR_VALUE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for STMPWCNT {
    #[inline(always)]
    fn default() -> STMPWCNT {
        STMPWCNT(0)
    }
}
impl core::fmt::Debug for STMPWCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STMPWCNT")
            .field("CURR_VALUE", &self.CURR_VALUE())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STMPWCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STMPWCNT {{ CURR_VALUE: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.CURR_VALUE(),
            self.RESERVED16()
        )
    }
}
#[doc = "Captured WCLK Counter Value, Capture Channel 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STMPWCNTCAPT0(pub u32);
impl STMPWCNTCAPT0 {
    #[doc = "15:0\\] The value of the samplestamp WCLK counter (STMPWCNT.CURR_VALUE) last time an event was pulsed (event source selected in EVENT:I2SSTMPSEL0.EV for channel 0). This number corresponds to the number of positive WCLK edges since the samplestamp generator was enabled (not taking modification through STMPWADD/STMPWSET into account). The value is cleared when STMPCTL.STMP_EN = 0."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPT_VALUE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] The value of the samplestamp WCLK counter (STMPWCNT.CURR_VALUE) last time an event was pulsed (event source selected in EVENT:I2SSTMPSEL0.EV for channel 0). This number corresponds to the number of positive WCLK edges since the samplestamp generator was enabled (not taking modification through STMPWADD/STMPWSET into account). The value is cleared when STMPCTL.STMP_EN = 0."]
    #[inline(always)]
    pub const fn set_CAPT_VALUE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for STMPWCNTCAPT0 {
    #[inline(always)]
    fn default() -> STMPWCNTCAPT0 {
        STMPWCNTCAPT0(0)
    }
}
impl core::fmt::Debug for STMPWCNTCAPT0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STMPWCNTCAPT0")
            .field("CAPT_VALUE", &self.CAPT_VALUE())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STMPWCNTCAPT0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STMPWCNTCAPT0 {{ CAPT_VALUE: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.CAPT_VALUE(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STMPWCNTCAPT1(pub u32);
impl STMPWCNTCAPT1 {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPT_VALUE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CAPT_VALUE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for STMPWCNTCAPT1 {
    #[inline(always)]
    fn default() -> STMPWCNTCAPT1 {
        STMPWCNTCAPT1(0)
    }
}
impl core::fmt::Debug for STMPWCNTCAPT1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STMPWCNTCAPT1")
            .field("CAPT_VALUE", &self.CAPT_VALUE())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STMPWCNTCAPT1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STMPWCNTCAPT1 {{ CAPT_VALUE: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.CAPT_VALUE(),
            self.RESERVED16()
        )
    }
}
#[doc = "WCLK Counter Period Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STMPWPER(pub u32);
impl STMPWPER {
    #[doc = "15:0\\] Used to define when STMPWCNT is to be reset so number of WCLK edges are found for the size of the sample buffer. This is thus a modulo value for the WCLK counter. This number must correspond to the size of the sample buffer used by the system (that is the index of the last sample plus 1)."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Used to define when STMPWCNT is to be reset so number of WCLK edges are found for the size of the sample buffer. This is thus a modulo value for the WCLK counter. This number must correspond to the size of the sample buffer used by the system (that is the index of the last sample plus 1)."]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for STMPWPER {
    #[inline(always)]
    fn default() -> STMPWPER {
        STMPWPER(0)
    }
}
impl core::fmt::Debug for STMPWPER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STMPWPER")
            .field("VALUE", &self.VALUE())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STMPWPER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STMPWPER {{ VALUE: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.VALUE(),
            self.RESERVED16()
        )
    }
}
#[doc = "WCLK Counter Set Operation."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STMPWSET(pub u32);
impl STMPWSET {
    #[doc = "15:0\\] WCLK counter modification: Sets the running WCLK counter equal to the written value."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] WCLK counter modification: Sets the running WCLK counter equal to the written value."]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for STMPWSET {
    #[inline(always)]
    fn default() -> STMPWSET {
        STMPWSET(0)
    }
}
impl core::fmt::Debug for STMPWSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STMPWSET")
            .field("VALUE", &self.VALUE())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STMPWSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STMPWSET {{ VALUE: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.VALUE(),
            self.RESERVED16()
        )
    }
}
#[doc = "Current Value of XCNT."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STMPXCNT(pub u32);
impl STMPXCNT {
    #[doc = "15:0\\] Current value of the XOSC counter, latched when reading STMPWCNT."]
    #[must_use]
    #[inline(always)]
    pub const fn CURR_VALUE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Current value of the XOSC counter, latched when reading STMPWCNT."]
    #[inline(always)]
    pub const fn set_CURR_VALUE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for STMPXCNT {
    #[inline(always)]
    fn default() -> STMPXCNT {
        STMPXCNT(0)
    }
}
impl core::fmt::Debug for STMPXCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STMPXCNT")
            .field("CURR_VALUE", &self.CURR_VALUE())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STMPXCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STMPXCNT {{ CURR_VALUE: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.CURR_VALUE(),
            self.RESERVED16()
        )
    }
}
#[doc = "Captured XOSC Counter Value, Capture Channel 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STMPXCNTCAPT0(pub u32);
impl STMPXCNTCAPT0 {
    #[doc = "15:0\\] The value of the samplestamp XOSC counter (STMPXCNT.CURR_VALUE) last time an event was pulsed (event source selected in \\[EVENT.I2SSTMPSEL0.EV\\] for channel 0). This number corresponds to the number of 24 MHz clock cycles since the last positive edge of the selected WCLK. The value is cleared when STMPCTL.STMP_EN = 0. Note: Due to buffering and synchronization, WCLK is delayed by a small number of BCLK periods and clk periods. Note: When calculating the fractional part of the sample stamp, STMPXPER may be less than this bit field."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPT_VALUE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] The value of the samplestamp XOSC counter (STMPXCNT.CURR_VALUE) last time an event was pulsed (event source selected in \\[EVENT.I2SSTMPSEL0.EV\\] for channel 0). This number corresponds to the number of 24 MHz clock cycles since the last positive edge of the selected WCLK. The value is cleared when STMPCTL.STMP_EN = 0. Note: Due to buffering and synchronization, WCLK is delayed by a small number of BCLK periods and clk periods. Note: When calculating the fractional part of the sample stamp, STMPXPER may be less than this bit field."]
    #[inline(always)]
    pub const fn set_CAPT_VALUE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for STMPXCNTCAPT0 {
    #[inline(always)]
    fn default() -> STMPXCNTCAPT0 {
        STMPXCNTCAPT0(0)
    }
}
impl core::fmt::Debug for STMPXCNTCAPT0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STMPXCNTCAPT0")
            .field("CAPT_VALUE", &self.CAPT_VALUE())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STMPXCNTCAPT0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STMPXCNTCAPT0 {{ CAPT_VALUE: {=u16:?}, RESERVED: {=u16:?} }}",
            self.CAPT_VALUE(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STMPXCNTCAPT1(pub u32);
impl STMPXCNTCAPT1 {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPT_VALUE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CAPT_VALUE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for STMPXCNTCAPT1 {
    #[inline(always)]
    fn default() -> STMPXCNTCAPT1 {
        STMPXCNTCAPT1(0)
    }
}
impl core::fmt::Debug for STMPXCNTCAPT1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STMPXCNTCAPT1")
            .field("CAPT_VALUE", &self.CAPT_VALUE())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STMPXCNTCAPT1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STMPXCNTCAPT1 {{ CAPT_VALUE: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.CAPT_VALUE(),
            self.RESERVED16()
        )
    }
}
#[doc = "XOSC Period Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STMPXPER(pub u32);
impl STMPXPER {
    #[doc = "15:0\\] The number of 24 MHz clock cycles in the previous WCLK period (that is - the next value of the XOSC counter at the positive WCLK edge, had it not been reset to 0). The value is cleared when STMPCTL.STMP_EN = 0."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] The number of 24 MHz clock cycles in the previous WCLK period (that is - the next value of the XOSC counter at the positive WCLK edge, had it not been reset to 0). The value is cleared when STMPCTL.STMP_EN = 0."]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for STMPXPER {
    #[inline(always)]
    fn default() -> STMPXPER {
        STMPXPER(0)
    }
}
impl core::fmt::Debug for STMPXPER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STMPXPER")
            .field("VALUE", &self.VALUE())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STMPXPER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STMPXPER {{ VALUE: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.VALUE(),
            self.RESERVED16()
        )
    }
}
#[doc = "XOSC Minimum Period Value Minimum Value of STMPXPER."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STMPXPERMIN(pub u32);
impl STMPXPERMIN {
    #[doc = "15:0\\] Each time STMPXPER is updated, the value is also loaded into this register, provided that the value is smaller than the current value in this register. When written, the register is reset to 0xFFFF (65535), regardless of the value written. The minimum value can be used to detect extra WCLK pulses (this registers value will be significantly smaller than STMPXPER.VALUE)."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Each time STMPXPER is updated, the value is also loaded into this register, provided that the value is smaller than the current value in this register. When written, the register is reset to 0xFFFF (65535), regardless of the value written. The minimum value can be used to detect extra WCLK pulses (this registers value will be significantly smaller than STMPXPER.VALUE)."]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for STMPXPERMIN {
    #[inline(always)]
    fn default() -> STMPXPERMIN {
        STMPXPERMIN(0)
    }
}
impl core::fmt::Debug for STMPXPERMIN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STMPXPERMIN")
            .field("VALUE", &self.VALUE())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STMPXPERMIN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STMPXPERMIN {{ VALUE: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.VALUE(),
            self.RESERVED16()
        )
    }
}

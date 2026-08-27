#[doc = "General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIODIE(pub u32);
impl GPIODIE {
    #[doc = "7:0\\] Write 1 to bit index n in this bit vector to enable digital input buffer for AUXIO\\[8i+n\\]. Write 0 to bit index n in this bit vector to disable digital input buffer for AUXIO\\[8i+n\\]. You must enable the digital input buffer for AUXIO\\[8i+n\\] to read the pin value in GPIODIN. You must disable the digital input buffer for analog input or pins that float to avoid current leakage."]
    #[must_use]
    #[inline(always)]
    pub const fn IO7_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Write 1 to bit index n in this bit vector to enable digital input buffer for AUXIO\\[8i+n\\]. Write 0 to bit index n in this bit vector to disable digital input buffer for AUXIO\\[8i+n\\]. You must enable the digital input buffer for AUXIO\\[8i+n\\] to read the pin value in GPIODIN. You must disable the digital input buffer for analog input or pins that float to avoid current leakage."]
    #[inline(always)]
    pub const fn set_IO7_0(&mut self, val: u8) {
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
impl Default for GPIODIE {
    #[inline(always)]
    fn default() -> GPIODIE {
        GPIODIE(0)
    }
}
impl core::fmt::Debug for GPIODIE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIODIE")
            .field("IO7_0", &self.IO7_0())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIODIE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIODIE {{ IO7_0: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.IO7_0(),
            self.RESERVED8()
        )
    }
}
#[doc = "General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIODIN(pub u32);
impl GPIODIN {
    #[doc = "7:0\\] Bit n in this bit vector contains the value for AUXIO\\[8i+n\\] when GPIODIE bit n is set. Otherwise, bit n value is old."]
    #[must_use]
    #[inline(always)]
    pub const fn IO7_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Bit n in this bit vector contains the value for AUXIO\\[8i+n\\] when GPIODIE bit n is set. Otherwise, bit n value is old."]
    #[inline(always)]
    pub const fn set_IO7_0(&mut self, val: u8) {
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
impl Default for GPIODIN {
    #[inline(always)]
    fn default() -> GPIODIN {
        GPIODIN(0)
    }
}
impl core::fmt::Debug for GPIODIN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIODIN")
            .field("IO7_0", &self.IO7_0())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIODIN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIODIN {{ IO7_0: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.IO7_0(),
            self.RESERVED8()
        )
    }
}
#[doc = "General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIODOUT(pub u32);
impl GPIODOUT {
    #[doc = "7:0\\] Write 1 to bit index n in this bit vector to set AUXIO\\[8i+n\\]. Write 0 to bit index n in this bit vector to clear AUXIO\\[8i+n\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn IO7_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Write 1 to bit index n in this bit vector to set AUXIO\\[8i+n\\]. Write 0 to bit index n in this bit vector to clear AUXIO\\[8i+n\\]."]
    #[inline(always)]
    pub const fn set_IO7_0(&mut self, val: u8) {
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
impl Default for GPIODOUT {
    #[inline(always)]
    fn default() -> GPIODOUT {
        GPIODOUT(0)
    }
}
impl core::fmt::Debug for GPIODOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIODOUT")
            .field("IO7_0", &self.IO7_0())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIODOUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIODOUT {{ IO7_0: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.IO7_0(),
            self.RESERVED8()
        )
    }
}
#[doc = "General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIODOUTCLR(pub u32);
impl GPIODOUTCLR {
    #[doc = "7:0\\] Write 1 to bit index n in this bit vector to clear GPIODOUT bit n. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn IO7_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Write 1 to bit index n in this bit vector to clear GPIODOUT bit n. Read value is 0."]
    #[inline(always)]
    pub const fn set_IO7_0(&mut self, val: u8) {
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
impl Default for GPIODOUTCLR {
    #[inline(always)]
    fn default() -> GPIODOUTCLR {
        GPIODOUTCLR(0)
    }
}
impl core::fmt::Debug for GPIODOUTCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIODOUTCLR")
            .field("IO7_0", &self.IO7_0())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIODOUTCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIODOUTCLR {{ IO7_0: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.IO7_0(),
            self.RESERVED8()
        )
    }
}
#[doc = "General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIODOUTSET(pub u32);
impl GPIODOUTSET {
    #[doc = "7:0\\] Write 1 to bit index n in this bit vector to set GPIODOUT bit n. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn IO7_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Write 1 to bit index n in this bit vector to set GPIODOUT bit n. Read value is 0."]
    #[inline(always)]
    pub const fn set_IO7_0(&mut self, val: u8) {
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
impl Default for GPIODOUTSET {
    #[inline(always)]
    fn default() -> GPIODOUTSET {
        GPIODOUTSET(0)
    }
}
impl core::fmt::Debug for GPIODOUTSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIODOUTSET")
            .field("IO7_0", &self.IO7_0())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIODOUTSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIODOUTSET {{ IO7_0: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.IO7_0(),
            self.RESERVED8()
        )
    }
}
#[doc = "General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIODOUTTGL(pub u32);
impl GPIODOUTTGL {
    #[doc = "7:0\\] Write 1 to bit index n in this bit vector to toggle GPIODOUT bit n. Read value is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn IO7_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Write 1 to bit index n in this bit vector to toggle GPIODOUT bit n. Read value is 0."]
    #[inline(always)]
    pub const fn set_IO7_0(&mut self, val: u8) {
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
impl Default for GPIODOUTTGL {
    #[inline(always)]
    fn default() -> GPIODOUTTGL {
        GPIODOUTTGL(0)
    }
}
impl core::fmt::Debug for GPIODOUTTGL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIODOUTTGL")
            .field("IO7_0", &self.IO7_0())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIODOUTTGL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIODOUTTGL {{ IO7_0: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.IO7_0(),
            self.RESERVED8()
        )
    }
}
#[doc = "Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOMODE(pub u32);
impl IOMODE {
    #[doc = "1:0\\] Select mode for AUXIO\\[8i+0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn IO0(&self) -> super::vals::IO0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::IO0::from_bits(val as u8)
    }
    #[doc = "1:0\\] Select mode for AUXIO\\[8i+0\\]."]
    #[inline(always)]
    pub const fn set_IO0(&mut self, val: super::vals::IO0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "3:2\\] Select mode for AUXIO\\[8i+1\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn IO1(&self) -> super::vals::IO1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::IO1::from_bits(val as u8)
    }
    #[doc = "3:2\\] Select mode for AUXIO\\[8i+1\\]."]
    #[inline(always)]
    pub const fn set_IO1(&mut self, val: super::vals::IO1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "5:4\\] Select mode for AUXIO\\[8i+2\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn IO2(&self) -> super::vals::IO2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::IO2::from_bits(val as u8)
    }
    #[doc = "5:4\\] Select mode for AUXIO\\[8i+2\\]."]
    #[inline(always)]
    pub const fn set_IO2(&mut self, val: super::vals::IO2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "7:6\\] Select mode for AUXIO\\[8i+3\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn IO3(&self) -> super::vals::IO3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::IO3::from_bits(val as u8)
    }
    #[doc = "7:6\\] Select mode for AUXIO\\[8i+3\\]."]
    #[inline(always)]
    pub const fn set_IO3(&mut self, val: super::vals::IO3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "9:8\\] Select mode for AUXIO\\[8i+4\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn IO4(&self) -> super::vals::IO4 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::IO4::from_bits(val as u8)
    }
    #[doc = "9:8\\] Select mode for AUXIO\\[8i+4\\]."]
    #[inline(always)]
    pub const fn set_IO4(&mut self, val: super::vals::IO4) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "11:10\\] Select mode for AUXIO\\[8i+5\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn IO5(&self) -> super::vals::IO5 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::IO5::from_bits(val as u8)
    }
    #[doc = "11:10\\] Select mode for AUXIO\\[8i+5\\]."]
    #[inline(always)]
    pub const fn set_IO5(&mut self, val: super::vals::IO5) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "13:12\\] Select mode for AUXIO\\[8i+6\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn IO6(&self) -> super::vals::IO6 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::IO6::from_bits(val as u8)
    }
    #[doc = "13:12\\] Select mode for AUXIO\\[8i+6\\]."]
    #[inline(always)]
    pub const fn set_IO6(&mut self, val: super::vals::IO6) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "15:14\\] Select mode for AUXIO\\[8i+7\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn IO7(&self) -> super::vals::IO7 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::IO7::from_bits(val as u8)
    }
    #[doc = "15:14\\] Select mode for AUXIO\\[8i+7\\]."]
    #[inline(always)]
    pub const fn set_IO7(&mut self, val: super::vals::IO7) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
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
impl Default for IOMODE {
    #[inline(always)]
    fn default() -> IOMODE {
        IOMODE(0)
    }
}
impl core::fmt::Debug for IOMODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOMODE")
            .field("IO0", &self.IO0())
            .field("IO1", &self.IO1())
            .field("IO2", &self.IO2())
            .field("IO3", &self.IO3())
            .field("IO4", &self.IO4())
            .field("IO5", &self.IO5())
            .field("IO6", &self.IO6())
            .field("IO7", &self.IO7())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOMODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOMODE {{ IO0: {:?}, IO1: {:?}, IO2: {:?}, IO3: {:?}, IO4: {:?}, IO5: {:?}, IO6: {:?}, IO7: {:?}, RESERVED16: {=u16:?} }}",
            self.IO0(),
            self.IO1(),
            self.IO2(),
            self.IO3(),
            self.IO4(),
            self.IO5(),
            self.IO6(),
            self.IO7(),
            self.RESERVED16()
        )
    }
}

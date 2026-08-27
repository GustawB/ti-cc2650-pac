#[doc = "Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BAT(pub u32);
impl BAT {
    #[doc = "7:0\\] Fractional part, standard binary fractional encoding. 0x00: .0V ... 0x20: 1/8 = .125V 0x40: 1/4 = .25V 0x80: 1/2 = .5V ... 0xA0: 1/2 + 1/8 = .625V ... 0xFF: Max."]
    #[must_use]
    #[inline(always)]
    pub const fn FRAC(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Fractional part, standard binary fractional encoding. 0x00: .0V ... 0x20: 1/8 = .125V 0x40: 1/4 = .25V 0x80: 1/2 = .5V ... 0xA0: 1/2 + 1/8 = .625V ... 0xFF: Max."]
    #[inline(always)]
    pub const fn set_FRAC(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "10:8\\] Integer part: 0x0: 0V + fractional part ... 0x3: 3V + fractional part 0x4: 4V + fractional part."]
    #[must_use]
    #[inline(always)]
    pub const fn INT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "10:8\\] Integer part: 0x0: 0V + fractional part ... 0x3: 3V + fractional part 0x4: 4V + fractional part."]
    #[inline(always)]
    pub const fn set_INT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
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
impl Default for BAT {
    #[inline(always)]
    fn default() -> BAT {
        BAT(0)
    }
}
impl core::fmt::Debug for BAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BAT")
            .field("FRAC", &self.FRAC())
            .field("INT", &self.INT())
            .field("RESERVED11", &self.RESERVED11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BAT {{ FRAC: {=u8:?}, INT: {=u8:?}, RESERVED11: {=u32:?} }}",
            self.FRAC(),
            self.INT(),
            self.RESERVED11()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BATMONP0(pub u32);
impl BATMONP0 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "31:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "31:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for BATMONP0 {
    #[inline(always)]
    fn default() -> BATMONP0 {
        BATMONP0(0)
    }
}
impl core::fmt::Debug for BATMONP0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BATMONP0")
            .field("CFG", &self.CFG())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BATMONP0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BATMONP0 {{ CFG: {=u8:?}, RESERVED6: {=u32:?} }}",
            self.CFG(),
            self.RESERVED6()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BATMONP1(pub u32);
impl BATMONP1 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "31:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "31:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for BATMONP1 {
    #[inline(always)]
    fn default() -> BATMONP1 {
        BATMONP1(0)
    }
}
impl core::fmt::Debug for BATMONP1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BATMONP1")
            .field("CFG", &self.CFG())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BATMONP1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BATMONP1 {{ CFG: {=u8:?}, RESERVED6: {=u32:?} }}",
            self.CFG(),
            self.RESERVED6()
        )
    }
}
#[doc = "Battery Update Indicates BAT Updates."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BATUPD(pub u32);
impl BATUPD {
    #[doc = "0:0\\] 0: No update since last clear 1: New battery voltage is present. Write 1 to clear the status."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: No update since last clear 1: New battery voltage is present. Write 1 to clear the status."]
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
impl Default for BATUPD {
    #[inline(always)]
    fn default() -> BATUPD {
        BATUPD(0)
    }
}
impl core::fmt::Debug for BATUPD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BATUPD")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BATUPD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BATUPD {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTL(pub u32);
impl CTL {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MEAS_EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MEAS_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CALC_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CALC_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "31:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
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
            .field("MEAS_EN", &self.MEAS_EN())
            .field("CALC_EN", &self.CALC_EN())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTL {{ MEAS_EN: {=bool:?}, CALC_EN: {=bool:?}, RESERVED2: {=u32:?} }}",
            self.MEAS_EN(),
            self.CALC_EN(),
            self.RESERVED2()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASHPUMPP0(pub u32);
impl FLASHPUMPP0 {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn OVR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_OVR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LOWLIM(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LOWLIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "7:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HIGHLIM(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "7:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HIGHLIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FALLB(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FALLB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "31:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "31:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for FLASHPUMPP0 {
    #[inline(always)]
    fn default() -> FLASHPUMPP0 {
        FLASHPUMPP0(0)
    }
}
impl core::fmt::Debug for FLASHPUMPP0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASHPUMPP0")
            .field("CFG", &self.CFG())
            .field("OVR", &self.OVR())
            .field("LOWLIM", &self.LOWLIM())
            .field("HIGHLIM", &self.HIGHLIM())
            .field("FALLB", &self.FALLB())
            .field("RESERVED9", &self.RESERVED9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASHPUMPP0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASHPUMPP0 {{ CFG: {=u8:?}, OVR: {=bool:?}, LOWLIM: {=bool:?}, HIGHLIM: {=u8:?}, FALLB: {=bool:?}, RESERVED9: {=u32:?} }}",
            self.CFG(),
            self.OVR(),
            self.LOWLIM(),
            self.HIGHLIM(),
            self.FALLB(),
            self.RESERVED9()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOSTRP0(pub u32);
impl IOSTRP0 {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CFG1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "5:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "5:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CFG2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "31:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "31:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for IOSTRP0 {
    #[inline(always)]
    fn default() -> IOSTRP0 {
        IOSTRP0(0)
    }
}
impl core::fmt::Debug for IOSTRP0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOSTRP0")
            .field("CFG1", &self.CFG1())
            .field("CFG2", &self.CFG2())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOSTRP0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOSTRP0 {{ CFG1: {=u8:?}, CFG2: {=u8:?}, RESERVED6: {=u32:?} }}",
            self.CFG1(),
            self.CFG2(),
            self.RESERVED6()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MEASCFG(pub u32);
impl MEASCFG {
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PER(&self) -> super::vals::PER {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::PER::from_bits(val as u8)
    }
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PER(&mut self, val: super::vals::PER) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "31:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for MEASCFG {
    #[inline(always)]
    fn default() -> MEASCFG {
        MEASCFG(0)
    }
}
impl core::fmt::Debug for MEASCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEASCFG")
            .field("PER", &self.PER())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MEASCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MEASCFG {{ PER: {:?}, RESERVED2: {=u32:?} }}",
            self.PER(),
            self.RESERVED2()
        )
    }
}
#[doc = "Temperature Last Measured Temperature in Degrees Celsius This register may be read while TEMPUPD.STAT = 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TEMP(pub u32);
impl TEMP {
    #[doc = "7:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "16:8\\] Integer part (signed) of temperature value. Total value = INTEGER + FRACTIONAL 2's complement encoding 0x100: Min value 0x1D8: -40C 0x1FF: -1C 0x00: 0C 0x1B: 27C 0x55: 85C 0xFF: Max value."]
    #[must_use]
    #[inline(always)]
    pub const fn INT(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x01ff;
        val as u16
    }
    #[doc = "16:8\\] Integer part (signed) of temperature value. Total value = INTEGER + FRACTIONAL 2's complement encoding 0x100: Min value 0x1D8: -40C 0x1FF: -1C 0x00: 0C 0x1B: 27C 0x55: 85C 0xFF: Max value."]
    #[inline(always)]
    pub const fn set_INT(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 8usize)) | (((val as u32) & 0x01ff) << 8usize);
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
impl Default for TEMP {
    #[inline(always)]
    fn default() -> TEMP {
        TEMP(0)
    }
}
impl core::fmt::Debug for TEMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEMP")
            .field("RESERVED0", &self.RESERVED0())
            .field("INT", &self.INT())
            .field("RESERVED17", &self.RESERVED17())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TEMP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TEMP {{ RESERVED0: {=u8:?}, INT: {=u16:?}, RESERVED17: {=u16:?} }}",
            self.RESERVED0(),
            self.INT(),
            self.RESERVED17()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TEMPP0(pub u32);
impl TEMPP0 {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for TEMPP0 {
    #[inline(always)]
    fn default() -> TEMPP0 {
        TEMPP0(0)
    }
}
impl core::fmt::Debug for TEMPP0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEMPP0")
            .field("CFG", &self.CFG())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TEMPP0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TEMPP0 {{ CFG: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.CFG(),
            self.RESERVED8()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TEMPP1(pub u32);
impl TEMPP1 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "31:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "31:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for TEMPP1 {
    #[inline(always)]
    fn default() -> TEMPP1 {
        TEMPP1(0)
    }
}
impl core::fmt::Debug for TEMPP1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEMPP1")
            .field("CFG", &self.CFG())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TEMPP1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TEMPP1 {{ CFG: {=u8:?}, RESERVED6: {=u32:?} }}",
            self.CFG(),
            self.RESERVED6()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TEMPP2(pub u32);
impl TEMPP2 {
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "31:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "31:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for TEMPP2 {
    #[inline(always)]
    fn default() -> TEMPP2 {
        TEMPP2(0)
    }
}
impl core::fmt::Debug for TEMPP2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEMPP2")
            .field("CFG", &self.CFG())
            .field("RESERVED5", &self.RESERVED5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TEMPP2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TEMPP2 {{ CFG: {=u8:?}, RESERVED5: {=u32:?} }}",
            self.CFG(),
            self.RESERVED5()
        )
    }
}
#[doc = "Temperature Update Indicates TEMP Updates."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TEMPUPD(pub u32);
impl TEMPUPD {
    #[doc = "0:0\\] 0: No update since last clear 1: New temperature is present. Write 1 to clear the status."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: No update since last clear 1: New temperature is present. Write 1 to clear the status."]
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
impl Default for TEMPUPD {
    #[inline(always)]
    fn default() -> TEMPUPD {
        TEMPUPD(0)
    }
}
impl core::fmt::Debug for TEMPUPD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEMPUPD")
            .field("STAT", &self.STAT())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TEMPUPD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TEMPUPD {{ STAT: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.STAT(),
            self.RESERVED1()
        )
    }
}

#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DYN_CG(pub u32);
impl DYN_CG {
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DYN_CG(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DYN_CG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "31:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DYN_CG {
    #[inline(always)]
    fn default() -> DYN_CG {
        DYN_CG(0)
    }
}
impl core::fmt::Debug for DYN_CG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DYN_CG")
            .field("DYN_CG", &self.DYN_CG())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DYN_CG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DYN_CG {{ DYN_CG: {=u8:?}, RESERVED: {=u32:?} }}",
            self.DYN_CG(),
            self.RESERVED()
        )
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESERVED000(pub u32);
impl RESERVED000 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RESERVED000 {
    #[inline(always)]
    fn default() -> RESERVED000 {
        RESERVED000(0)
    }
}
impl core::fmt::Debug for RESERVED000 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVED000")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESERVED000 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RESERVED000 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TRACECLKMUX(pub u32);
impl TRACECLKMUX {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRACECLK_N_SWV(&self) -> super::vals::TRACECLK_N_SWV {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TRACECLK_N_SWV::from_bits(val as u8)
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRACECLK_N_SWV(&mut self, val: super::vals::TRACECLK_N_SWV) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for TRACECLKMUX {
    #[inline(always)]
    fn default() -> TRACECLKMUX {
        TRACECLKMUX(0)
    }
}
impl core::fmt::Debug for TRACECLKMUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRACECLKMUX")
            .field("TRACECLK_N_SWV", &self.TRACECLK_N_SWV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TRACECLKMUX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TRACECLKMUX {{ TRACECLK_N_SWV: {:?}, RESERVED: {=u32:?} }}",
            self.TRACECLK_N_SWV(),
            self.RESERVED()
        )
    }
}

#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AMPCOMP_CTRL1(pub u32);
impl AMPCOMP_CTRL1 {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IBIASCAP_HPTOLP_OL_CNT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IBIASCAP_HPTOLP_OL_CNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CAP_STEP(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CAP_STEP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_IBIAS_WAIT_CNT_FINAL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LPM_IBIAS_WAIT_CNT_FINAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IBIAS_INIT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IBIAS_INIT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IBIAS_OFFSET(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IBIAS_OFFSET(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "29:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "29:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "30:30\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn AMPCOMP_REQ_MODE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_AMPCOMP_REQ_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for AMPCOMP_CTRL1 {
    #[inline(always)]
    fn default() -> AMPCOMP_CTRL1 {
        AMPCOMP_CTRL1(0)
    }
}
impl core::fmt::Debug for AMPCOMP_CTRL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMPCOMP_CTRL1")
            .field("IBIASCAP_HPTOLP_OL_CNT", &self.IBIASCAP_HPTOLP_OL_CNT())
            .field("CAP_STEP", &self.CAP_STEP())
            .field("LPM_IBIAS_WAIT_CNT_FINAL", &self.LPM_IBIAS_WAIT_CNT_FINAL())
            .field("IBIAS_INIT", &self.IBIAS_INIT())
            .field("IBIAS_OFFSET", &self.IBIAS_OFFSET())
            .field("RESERVED0", &self.RESERVED0())
            .field("AMPCOMP_REQ_MODE", &self.AMPCOMP_REQ_MODE())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AMPCOMP_CTRL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AMPCOMP_CTRL1 {{ IBIASCAP_HPTOLP_OL_CNT: {=u8:?}, CAP_STEP: {=u8:?}, LPM_IBIAS_WAIT_CNT_FINAL: {=u8:?}, IBIAS_INIT: {=u8:?}, IBIAS_OFFSET: {=u8:?}, RESERVED0: {=u8:?}, AMPCOMP_REQ_MODE: {=bool:?}, RESERVED1: {=bool:?} }}",
            self.IBIASCAP_HPTOLP_OL_CNT(),
            self.CAP_STEP(),
            self.LPM_IBIAS_WAIT_CNT_FINAL(),
            self.IBIAS_INIT(),
            self.IBIAS_OFFSET(),
            self.RESERVED0(),
            self.AMPCOMP_REQ_MODE(),
            self.RESERVED1()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AMPCOMP_TH1(pub u32);
impl AMPCOMP_TH1 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPMRAMP1_TH(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPMRAMP1_TH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "9:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IBIASCAP_LPTOHP_OL_CNT(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x0f;
        val as u8
    }
    #[doc = "9:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IBIASCAP_LPTOHP_OL_CNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
    }
    #[doc = "15:10\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPMRAMP3_HTH(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "15:10\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPMRAMP3_HTH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[doc = "17:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "17:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "23:18\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPMRAMP3_LTH(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "23:18\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPMRAMP3_LTH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for AMPCOMP_TH1 {
    #[inline(always)]
    fn default() -> AMPCOMP_TH1 {
        AMPCOMP_TH1(0)
    }
}
impl core::fmt::Debug for AMPCOMP_TH1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMPCOMP_TH1")
            .field("HPMRAMP1_TH", &self.HPMRAMP1_TH())
            .field("IBIASCAP_LPTOHP_OL_CNT", &self.IBIASCAP_LPTOHP_OL_CNT())
            .field("HPMRAMP3_HTH", &self.HPMRAMP3_HTH())
            .field("RESERVED0", &self.RESERVED0())
            .field("HPMRAMP3_LTH", &self.HPMRAMP3_LTH())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AMPCOMP_TH1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AMPCOMP_TH1 {{ HPMRAMP1_TH: {=u8:?}, IBIASCAP_LPTOHP_OL_CNT: {=u8:?}, HPMRAMP3_HTH: {=u8:?}, RESERVED0: {=u8:?}, HPMRAMP3_LTH: {=u8:?}, RESERVED1: {=u8:?} }}",
            self.HPMRAMP1_TH(),
            self.IBIASCAP_LPTOHP_OL_CNT(),
            self.HPMRAMP3_HTH(),
            self.RESERVED0(),
            self.HPMRAMP3_LTH(),
            self.RESERVED1()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AMPCOMP_TH2(pub u32);
impl AMPCOMP_TH2 {
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "7:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_COMP_AMPTH_HPM(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "7:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ADC_COMP_AMPTH_HPM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "9:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "9:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "15:10\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_COMP_AMPTH_LPM(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "15:10\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ADC_COMP_AMPTH_LPM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[doc = "17:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "17:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "23:18\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LPMUPDATE_HTM(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "23:18\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LPMUPDATE_HTM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "25:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "25:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "31:26\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LPMUPDATE_LTH(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "31:26\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LPMUPDATE_LTH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for AMPCOMP_TH2 {
    #[inline(always)]
    fn default() -> AMPCOMP_TH2 {
        AMPCOMP_TH2(0)
    }
}
impl core::fmt::Debug for AMPCOMP_TH2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMPCOMP_TH2")
            .field("RESERVED0", &self.RESERVED0())
            .field("ADC_COMP_AMPTH_HPM", &self.ADC_COMP_AMPTH_HPM())
            .field("RESERVED1", &self.RESERVED1())
            .field("ADC_COMP_AMPTH_LPM", &self.ADC_COMP_AMPTH_LPM())
            .field("RESERVED2", &self.RESERVED2())
            .field("LPMUPDATE_HTM", &self.LPMUPDATE_HTM())
            .field("RESERVED3", &self.RESERVED3())
            .field("LPMUPDATE_LTH", &self.LPMUPDATE_LTH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AMPCOMP_TH2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AMPCOMP_TH2 {{ RESERVED0: {=u8:?}, ADC_COMP_AMPTH_HPM: {=u8:?}, RESERVED1: {=u8:?}, ADC_COMP_AMPTH_LPM: {=u8:?}, RESERVED2: {=u8:?}, LPMUPDATE_HTM: {=u8:?}, RESERVED3: {=u8:?}, LPMUPDATE_LTH: {=u8:?} }}",
            self.RESERVED0(),
            self.ADC_COMP_AMPTH_HPM(),
            self.RESERVED1(),
            self.ADC_COMP_AMPTH_LPM(),
            self.RESERVED2(),
            self.LPMUPDATE_HTM(),
            self.RESERVED3(),
            self.LPMUPDATE_LTH()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANA2_TRIM(pub u32);
impl ANA2_TRIM {
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDC_HIGH_EN_SEL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DCDC_HIGH_EN_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "5:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDC_LOW_EN_SEL(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "5:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DCDC_LOW_EN_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "7:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DEAD_TIME_TRIM(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "7:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DEAD_TIME_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "10:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDC_IPEAK(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "10:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DCDC_IPEAK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DITHER_EN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DITHER_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "21:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn NANOAMP_RES_TRIM(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "21:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_NANOAMP_RES_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "22:22\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ATESTLF_UDIGLDO_IBIAS_TRIM(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ATESTLF_UDIGLDO_IBIAS_TRIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "24:23\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SET_RCOSC_HF_FINE_RESISTOR(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x03;
        val as u8
    }
    #[doc = "24:23\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SET_RCOSC_HF_FINE_RESISTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
    }
    #[doc = "25:25\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "30:26\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSCHFCTRIMFRACT(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "30:26\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RCOSCHFCTRIMFRACT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "31:31\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSCHFCTRIMFRACT_EN(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RCOSCHFCTRIMFRACT_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ANA2_TRIM {
    #[inline(always)]
    fn default() -> ANA2_TRIM {
        ANA2_TRIM(0)
    }
}
impl core::fmt::Debug for ANA2_TRIM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANA2_TRIM")
            .field("DCDC_HIGH_EN_SEL", &self.DCDC_HIGH_EN_SEL())
            .field("DCDC_LOW_EN_SEL", &self.DCDC_LOW_EN_SEL())
            .field("DEAD_TIME_TRIM", &self.DEAD_TIME_TRIM())
            .field("DCDC_IPEAK", &self.DCDC_IPEAK())
            .field("DITHER_EN", &self.DITHER_EN())
            .field("RESERVED1", &self.RESERVED1())
            .field("NANOAMP_RES_TRIM", &self.NANOAMP_RES_TRIM())
            .field(
                "ATESTLF_UDIGLDO_IBIAS_TRIM",
                &self.ATESTLF_UDIGLDO_IBIAS_TRIM(),
            )
            .field(
                "SET_RCOSC_HF_FINE_RESISTOR",
                &self.SET_RCOSC_HF_FINE_RESISTOR(),
            )
            .field("RESERVED0", &self.RESERVED0())
            .field("RCOSCHFCTRIMFRACT", &self.RCOSCHFCTRIMFRACT())
            .field("RCOSCHFCTRIMFRACT_EN", &self.RCOSCHFCTRIMFRACT_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANA2_TRIM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANA2_TRIM {{ DCDC_HIGH_EN_SEL: {=u8:?}, DCDC_LOW_EN_SEL: {=u8:?}, DEAD_TIME_TRIM: {=u8:?}, DCDC_IPEAK: {=u8:?}, DITHER_EN: {=bool:?}, RESERVED1: {=u8:?}, NANOAMP_RES_TRIM: {=u8:?}, ATESTLF_UDIGLDO_IBIAS_TRIM: {=bool:?}, SET_RCOSC_HF_FINE_RESISTOR: {=u8:?}, RESERVED0: {=bool:?}, RCOSCHFCTRIMFRACT: {=u8:?}, RCOSCHFCTRIMFRACT_EN: {=bool:?} }}",
            self.DCDC_HIGH_EN_SEL(),
            self.DCDC_LOW_EN_SEL(),
            self.DEAD_TIME_TRIM(),
            self.DCDC_IPEAK(),
            self.DITHER_EN(),
            self.RESERVED1(),
            self.NANOAMP_RES_TRIM(),
            self.ATESTLF_UDIGLDO_IBIAS_TRIM(),
            self.SET_RCOSC_HF_FINE_RESISTOR(),
            self.RESERVED0(),
            self.RCOSCHFCTRIMFRACT(),
            self.RCOSCHFCTRIMFRACT_EN()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANABYPASS_VALUE2(pub u32);
impl ANABYPASS_VALUE2 {
    #[doc = "13:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_HF_IBIASTHERM(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "13:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_XOSC_HF_IBIASTHERM(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "31:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 14usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "31:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 14usize)) | (((val as u32) & 0x0003_ffff) << 14usize);
    }
}
impl Default for ANABYPASS_VALUE2 {
    #[inline(always)]
    fn default() -> ANABYPASS_VALUE2 {
        ANABYPASS_VALUE2(0)
    }
}
impl core::fmt::Debug for ANABYPASS_VALUE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANABYPASS_VALUE2")
            .field("XOSC_HF_IBIASTHERM", &self.XOSC_HF_IBIASTHERM())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANABYPASS_VALUE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANABYPASS_VALUE2 {{ XOSC_HF_IBIASTHERM: {=u16:?}, RESERVED: {=u32:?} }}",
            self.XOSC_HF_IBIASTHERM(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BAT_RC_LDO_TRIM(pub u32);
impl BAT_RC_LDO_TRIM {
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MEASUREPER(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MEASUREPER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "7:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "7:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSCHF_ITUNE_TRIM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RCOSCHF_ITUNE_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VTRIM_UDIG(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VTRIM_UDIG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VTRIM_BOD(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VTRIM_BOD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for BAT_RC_LDO_TRIM {
    #[inline(always)]
    fn default() -> BAT_RC_LDO_TRIM {
        BAT_RC_LDO_TRIM(0)
    }
}
impl core::fmt::Debug for BAT_RC_LDO_TRIM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BAT_RC_LDO_TRIM")
            .field("MEASUREPER", &self.MEASUREPER())
            .field("RESERVED1", &self.RESERVED1())
            .field("RCOSCHF_ITUNE_TRIM", &self.RCOSCHF_ITUNE_TRIM())
            .field("RESERVED2", &self.RESERVED2())
            .field("VTRIM_UDIG", &self.VTRIM_UDIG())
            .field("RESERVED3", &self.RESERVED3())
            .field("VTRIM_BOD", &self.VTRIM_BOD())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BAT_RC_LDO_TRIM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BAT_RC_LDO_TRIM {{ MEASUREPER: {=u8:?}, RESERVED1: {=u8:?}, RCOSCHF_ITUNE_TRIM: {=u8:?}, RESERVED2: {=u8:?}, VTRIM_UDIG: {=u8:?}, RESERVED3: {=u8:?}, VTRIM_BOD: {=u8:?}, RESERVED4: {=u8:?} }}",
            self.MEASUREPER(),
            self.RESERVED1(),
            self.RCOSCHF_ITUNE_TRIM(),
            self.RESERVED2(),
            self.VTRIM_UDIG(),
            self.RESERVED3(),
            self.VTRIM_BOD(),
            self.RESERVED4()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAP_TRIM(pub u32);
impl CAP_TRIM {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FLUX_CAP_0P4_TRIM(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FLUX_CAP_0P4_TRIM(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FLUX_CAP_0P28_TRIM(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FLUX_CAP_0P28_TRIM(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CAP_TRIM {
    #[inline(always)]
    fn default() -> CAP_TRIM {
        CAP_TRIM(0)
    }
}
impl core::fmt::Debug for CAP_TRIM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_TRIM")
            .field("FLUX_CAP_0P4_TRIM", &self.FLUX_CAP_0P4_TRIM())
            .field("FLUX_CAP_0P28_TRIM", &self.FLUX_CAP_0P28_TRIM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAP_TRIM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAP_TRIM {{ FLUX_CAP_0P4_TRIM: {=u16:?}, FLUX_CAP_0P28_TRIM: {=u16:?} }}",
            self.FLUX_CAP_0P4_TRIM(),
            self.FLUX_CAP_0P28_TRIM()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_IF_ADC(pub u32);
impl CONFIG_IF_ADC {
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFANALDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFANALDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "9:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFDIGLDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "9:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFDIGLDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "13:10\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn INT2ADJ(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[doc = "13:10\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_INT2ADJ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[doc = "15:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn AAFCAP(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "15:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_AAFCAP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FF1ADJ(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FF1ADJ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn INT3ADJ(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_INT3ADJ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FF3ADJ(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FF3ADJ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FF2ADJ(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FF2ADJ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for CONFIG_IF_ADC {
    #[inline(always)]
    fn default() -> CONFIG_IF_ADC {
        CONFIG_IF_ADC(0)
    }
}
impl core::fmt::Debug for CONFIG_IF_ADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_IF_ADC")
            .field("IFANALDO_TRIM_OUTPUT", &self.IFANALDO_TRIM_OUTPUT())
            .field("IFDIGLDO_TRIM_OUTPUT", &self.IFDIGLDO_TRIM_OUTPUT())
            .field("INT2ADJ", &self.INT2ADJ())
            .field("AAFCAP", &self.AAFCAP())
            .field("FF1ADJ", &self.FF1ADJ())
            .field("INT3ADJ", &self.INT3ADJ())
            .field("FF3ADJ", &self.FF3ADJ())
            .field("FF2ADJ", &self.FF2ADJ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_IF_ADC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_IF_ADC {{ IFANALDO_TRIM_OUTPUT: {=u8:?}, IFDIGLDO_TRIM_OUTPUT: {=u8:?}, INT2ADJ: {=u8:?}, AAFCAP: {=u8:?}, FF1ADJ: {=u8:?}, INT3ADJ: {=u8:?}, FF3ADJ: {=u8:?}, FF2ADJ: {=u8:?} }}",
            self.IFANALDO_TRIM_OUTPUT(),
            self.IFDIGLDO_TRIM_OUTPUT(),
            self.INT2ADJ(),
            self.AAFCAP(),
            self.FF1ADJ(),
            self.INT3ADJ(),
            self.FF3ADJ(),
            self.FF2ADJ()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_MISC_ADC(pub u32);
impl CONFIG_MISC_ADC {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DACTRIM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DACTRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn QUANTCTLTHRES(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_QUANTCTLTHRES(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "16:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RSSI_OFFSET(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0xff;
        val as u8
    }
    #[doc = "16:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RSSI_OFFSET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 9usize)) | (((val as u32) & 0xff) << 9usize);
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RSSITRIMCOMPLETE_N(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RSSITRIMCOMPLETE_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "31:18\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x3fff;
        val as u16
    }
    #[doc = "31:18\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 18usize)) | (((val as u32) & 0x3fff) << 18usize);
    }
}
impl Default for CONFIG_MISC_ADC {
    #[inline(always)]
    fn default() -> CONFIG_MISC_ADC {
        CONFIG_MISC_ADC(0)
    }
}
impl core::fmt::Debug for CONFIG_MISC_ADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_MISC_ADC")
            .field("DACTRIM", &self.DACTRIM())
            .field("QUANTCTLTHRES", &self.QUANTCTLTHRES())
            .field("RSSI_OFFSET", &self.RSSI_OFFSET())
            .field("RSSITRIMCOMPLETE_N", &self.RSSITRIMCOMPLETE_N())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_MISC_ADC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_MISC_ADC {{ DACTRIM: {=u8:?}, QUANTCTLTHRES: {=u8:?}, RSSI_OFFSET: {=u8:?}, RSSITRIMCOMPLETE_N: {=bool:?}, RESERVED: {=u16:?} }}",
            self.DACTRIM(),
            self.QUANTCTLTHRES(),
            self.RSSI_OFFSET(),
            self.RSSITRIMCOMPLETE_N(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_MISC_ADC_DIV10(pub u32);
impl CONFIG_MISC_ADC_DIV10 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DACTRIM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DACTRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn QUANTCTLTHRES(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_QUANTCTLTHRES(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "16:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RSSI_OFFSET(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0xff;
        val as u8
    }
    #[doc = "16:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RSSI_OFFSET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 9usize)) | (((val as u32) & 0xff) << 9usize);
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for CONFIG_MISC_ADC_DIV10 {
    #[inline(always)]
    fn default() -> CONFIG_MISC_ADC_DIV10 {
        CONFIG_MISC_ADC_DIV10(0)
    }
}
impl core::fmt::Debug for CONFIG_MISC_ADC_DIV10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_MISC_ADC_DIV10")
            .field("DACTRIM", &self.DACTRIM())
            .field("QUANTCTLTHRES", &self.QUANTCTLTHRES())
            .field("RSSI_OFFSET", &self.RSSI_OFFSET())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_MISC_ADC_DIV10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_MISC_ADC_DIV10 {{ DACTRIM: {=u8:?}, QUANTCTLTHRES: {=u8:?}, RSSI_OFFSET: {=u8:?}, RESERVED: {=u16:?} }}",
            self.DACTRIM(),
            self.QUANTCTLTHRES(),
            self.RSSI_OFFSET(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_MISC_ADC_DIV12(pub u32);
impl CONFIG_MISC_ADC_DIV12 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DACTRIM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DACTRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn QUANTCTLTHRES(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_QUANTCTLTHRES(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "16:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RSSI_OFFSET(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0xff;
        val as u8
    }
    #[doc = "16:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RSSI_OFFSET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 9usize)) | (((val as u32) & 0xff) << 9usize);
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for CONFIG_MISC_ADC_DIV12 {
    #[inline(always)]
    fn default() -> CONFIG_MISC_ADC_DIV12 {
        CONFIG_MISC_ADC_DIV12(0)
    }
}
impl core::fmt::Debug for CONFIG_MISC_ADC_DIV12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_MISC_ADC_DIV12")
            .field("DACTRIM", &self.DACTRIM())
            .field("QUANTCTLTHRES", &self.QUANTCTLTHRES())
            .field("RSSI_OFFSET", &self.RSSI_OFFSET())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_MISC_ADC_DIV12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_MISC_ADC_DIV12 {{ DACTRIM: {=u8:?}, QUANTCTLTHRES: {=u8:?}, RSSI_OFFSET: {=u8:?}, RESERVED: {=u16:?} }}",
            self.DACTRIM(),
            self.QUANTCTLTHRES(),
            self.RSSI_OFFSET(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_MISC_ADC_DIV15(pub u32);
impl CONFIG_MISC_ADC_DIV15 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DACTRIM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DACTRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn QUANTCTLTHRES(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_QUANTCTLTHRES(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "16:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RSSI_OFFSET(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0xff;
        val as u8
    }
    #[doc = "16:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RSSI_OFFSET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 9usize)) | (((val as u32) & 0xff) << 9usize);
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for CONFIG_MISC_ADC_DIV15 {
    #[inline(always)]
    fn default() -> CONFIG_MISC_ADC_DIV15 {
        CONFIG_MISC_ADC_DIV15(0)
    }
}
impl core::fmt::Debug for CONFIG_MISC_ADC_DIV15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_MISC_ADC_DIV15")
            .field("DACTRIM", &self.DACTRIM())
            .field("QUANTCTLTHRES", &self.QUANTCTLTHRES())
            .field("RSSI_OFFSET", &self.RSSI_OFFSET())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_MISC_ADC_DIV15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_MISC_ADC_DIV15 {{ DACTRIM: {=u8:?}, QUANTCTLTHRES: {=u8:?}, RSSI_OFFSET: {=u8:?}, RESERVED: {=u16:?} }}",
            self.DACTRIM(),
            self.QUANTCTLTHRES(),
            self.RSSI_OFFSET(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_MISC_ADC_DIV30(pub u32);
impl CONFIG_MISC_ADC_DIV30 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DACTRIM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DACTRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn QUANTCTLTHRES(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_QUANTCTLTHRES(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "16:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RSSI_OFFSET(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0xff;
        val as u8
    }
    #[doc = "16:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RSSI_OFFSET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 9usize)) | (((val as u32) & 0xff) << 9usize);
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for CONFIG_MISC_ADC_DIV30 {
    #[inline(always)]
    fn default() -> CONFIG_MISC_ADC_DIV30 {
        CONFIG_MISC_ADC_DIV30(0)
    }
}
impl core::fmt::Debug for CONFIG_MISC_ADC_DIV30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_MISC_ADC_DIV30")
            .field("DACTRIM", &self.DACTRIM())
            .field("QUANTCTLTHRES", &self.QUANTCTLTHRES())
            .field("RSSI_OFFSET", &self.RSSI_OFFSET())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_MISC_ADC_DIV30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_MISC_ADC_DIV30 {{ DACTRIM: {=u8:?}, QUANTCTLTHRES: {=u8:?}, RSSI_OFFSET: {=u8:?}, RESERVED: {=u16:?} }}",
            self.DACTRIM(),
            self.QUANTCTLTHRES(),
            self.RSSI_OFFSET(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_MISC_ADC_DIV5(pub u32);
impl CONFIG_MISC_ADC_DIV5 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DACTRIM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DACTRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn QUANTCTLTHRES(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_QUANTCTLTHRES(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "16:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RSSI_OFFSET(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0xff;
        val as u8
    }
    #[doc = "16:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RSSI_OFFSET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 9usize)) | (((val as u32) & 0xff) << 9usize);
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for CONFIG_MISC_ADC_DIV5 {
    #[inline(always)]
    fn default() -> CONFIG_MISC_ADC_DIV5 {
        CONFIG_MISC_ADC_DIV5(0)
    }
}
impl core::fmt::Debug for CONFIG_MISC_ADC_DIV5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_MISC_ADC_DIV5")
            .field("DACTRIM", &self.DACTRIM())
            .field("QUANTCTLTHRES", &self.QUANTCTLTHRES())
            .field("RSSI_OFFSET", &self.RSSI_OFFSET())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_MISC_ADC_DIV5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_MISC_ADC_DIV5 {{ DACTRIM: {=u8:?}, QUANTCTLTHRES: {=u8:?}, RSSI_OFFSET: {=u8:?}, RESERVED: {=u16:?} }}",
            self.DACTRIM(),
            self.QUANTCTLTHRES(),
            self.RSSI_OFFSET(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_MISC_ADC_DIV6(pub u32);
impl CONFIG_MISC_ADC_DIV6 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DACTRIM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DACTRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn QUANTCTLTHRES(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "8:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_QUANTCTLTHRES(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "16:9\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RSSI_OFFSET(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0xff;
        val as u8
    }
    #[doc = "16:9\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RSSI_OFFSET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 9usize)) | (((val as u32) & 0xff) << 9usize);
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "31:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for CONFIG_MISC_ADC_DIV6 {
    #[inline(always)]
    fn default() -> CONFIG_MISC_ADC_DIV6 {
        CONFIG_MISC_ADC_DIV6(0)
    }
}
impl core::fmt::Debug for CONFIG_MISC_ADC_DIV6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_MISC_ADC_DIV6")
            .field("DACTRIM", &self.DACTRIM())
            .field("QUANTCTLTHRES", &self.QUANTCTLTHRES())
            .field("RSSI_OFFSET", &self.RSSI_OFFSET())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_MISC_ADC_DIV6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_MISC_ADC_DIV6 {{ DACTRIM: {=u8:?}, QUANTCTLTHRES: {=u8:?}, RSSI_OFFSET: {=u8:?}, RESERVED: {=u16:?} }}",
            self.DACTRIM(),
            self.QUANTCTLTHRES(),
            self.RSSI_OFFSET(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_OSC_TOP(pub u32);
impl CONFIG_OSC_TOP {
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSCLF_RTUNE_TRIM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RCOSCLF_RTUNE_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "9:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSCLF_CTUNE_TRIM(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0xff;
        val as u8
    }
    #[doc = "9:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RCOSCLF_CTUNE_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 2usize)) | (((val as u32) & 0xff) << 2usize);
    }
    #[doc = "25:10\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_HF_COLUMN_Q12(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0xffff;
        val as u16
    }
    #[doc = "25:10\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_XOSC_HF_COLUMN_Q12(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 10usize)) | (((val as u32) & 0xffff) << 10usize);
    }
    #[doc = "29:26\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_HF_ROW_Q12(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x0f;
        val as u8
    }
    #[doc = "29:26\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_XOSC_HF_ROW_Q12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 26usize)) | (((val as u32) & 0x0f) << 26usize);
    }
    #[doc = "31:30\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "31:30\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for CONFIG_OSC_TOP {
    #[inline(always)]
    fn default() -> CONFIG_OSC_TOP {
        CONFIG_OSC_TOP(0)
    }
}
impl core::fmt::Debug for CONFIG_OSC_TOP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_OSC_TOP")
            .field("RCOSCLF_RTUNE_TRIM", &self.RCOSCLF_RTUNE_TRIM())
            .field("RCOSCLF_CTUNE_TRIM", &self.RCOSCLF_CTUNE_TRIM())
            .field("XOSC_HF_COLUMN_Q12", &self.XOSC_HF_COLUMN_Q12())
            .field("XOSC_HF_ROW_Q12", &self.XOSC_HF_ROW_Q12())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_OSC_TOP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_OSC_TOP {{ RCOSCLF_RTUNE_TRIM: {=u8:?}, RCOSCLF_CTUNE_TRIM: {=u8:?}, XOSC_HF_COLUMN_Q12: {=u16:?}, XOSC_HF_ROW_Q12: {=u8:?}, RESERVED: {=u8:?} }}",
            self.RCOSCLF_RTUNE_TRIM(),
            self.RCOSCLF_CTUNE_TRIM(),
            self.XOSC_HF_COLUMN_Q12(),
            self.XOSC_HF_ROW_Q12(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_RF_FRONTEND(pub u32);
impl CONFIG_RF_FRONTEND {
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RFLDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RFLDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "12:7\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x3f;
        val as u8
    }
    #[doc = "12:7\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 7usize)) | (((val as u32) & 0x3f) << 7usize);
    }
    #[doc = "13:13\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PATRIMCOMPLETE_N(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PATRIMCOMPLETE_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "18:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CTL_PA0_TRIM(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x1f;
        val as u8
    }
    #[doc = "18:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CTL_PA0_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 14usize)) | (((val as u32) & 0x1f) << 14usize);
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFAMP_TRIM(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFAMP_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LNA_IB(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LNA_IB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFAMP_IB(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFAMP_IB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for CONFIG_RF_FRONTEND {
    #[inline(always)]
    fn default() -> CONFIG_RF_FRONTEND {
        CONFIG_RF_FRONTEND(0)
    }
}
impl core::fmt::Debug for CONFIG_RF_FRONTEND {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_RF_FRONTEND")
            .field("RFLDO_TRIM_OUTPUT", &self.RFLDO_TRIM_OUTPUT())
            .field("RESERVED", &self.RESERVED())
            .field("PATRIMCOMPLETE_N", &self.PATRIMCOMPLETE_N())
            .field("CTL_PA0_TRIM", &self.CTL_PA0_TRIM())
            .field("IFAMP_TRIM", &self.IFAMP_TRIM())
            .field("LNA_IB", &self.LNA_IB())
            .field("IFAMP_IB", &self.IFAMP_IB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_RF_FRONTEND {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_RF_FRONTEND {{ RFLDO_TRIM_OUTPUT: {=u8:?}, RESERVED: {=u8:?}, PATRIMCOMPLETE_N: {=bool:?}, CTL_PA0_TRIM: {=u8:?}, IFAMP_TRIM: {=u8:?}, LNA_IB: {=u8:?}, IFAMP_IB: {=u8:?} }}",
            self.RFLDO_TRIM_OUTPUT(),
            self.RESERVED(),
            self.PATRIMCOMPLETE_N(),
            self.CTL_PA0_TRIM(),
            self.IFAMP_TRIM(),
            self.LNA_IB(),
            self.IFAMP_IB()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_RF_FRONTEND_DIV10(pub u32);
impl CONFIG_RF_FRONTEND_DIV10 {
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RFLDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RFLDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "13:7\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[doc = "13:7\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[doc = "18:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CTL_PA0_TRIM(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x1f;
        val as u8
    }
    #[doc = "18:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CTL_PA0_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 14usize)) | (((val as u32) & 0x1f) << 14usize);
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFAMP_TRIM(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFAMP_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LNA_IB(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LNA_IB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFAMP_IB(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFAMP_IB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for CONFIG_RF_FRONTEND_DIV10 {
    #[inline(always)]
    fn default() -> CONFIG_RF_FRONTEND_DIV10 {
        CONFIG_RF_FRONTEND_DIV10(0)
    }
}
impl core::fmt::Debug for CONFIG_RF_FRONTEND_DIV10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_RF_FRONTEND_DIV10")
            .field("RFLDO_TRIM_OUTPUT", &self.RFLDO_TRIM_OUTPUT())
            .field("RESERVED", &self.RESERVED())
            .field("CTL_PA0_TRIM", &self.CTL_PA0_TRIM())
            .field("IFAMP_TRIM", &self.IFAMP_TRIM())
            .field("LNA_IB", &self.LNA_IB())
            .field("IFAMP_IB", &self.IFAMP_IB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_RF_FRONTEND_DIV10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_RF_FRONTEND_DIV10 {{ RFLDO_TRIM_OUTPUT: {=u8:?}, RESERVED: {=u8:?}, CTL_PA0_TRIM: {=u8:?}, IFAMP_TRIM: {=u8:?}, LNA_IB: {=u8:?}, IFAMP_IB: {=u8:?} }}",
            self.RFLDO_TRIM_OUTPUT(),
            self.RESERVED(),
            self.CTL_PA0_TRIM(),
            self.IFAMP_TRIM(),
            self.LNA_IB(),
            self.IFAMP_IB()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_RF_FRONTEND_DIV12(pub u32);
impl CONFIG_RF_FRONTEND_DIV12 {
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RFLDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RFLDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "13:7\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[doc = "13:7\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[doc = "18:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CTL_PA0_TRIM(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x1f;
        val as u8
    }
    #[doc = "18:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CTL_PA0_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 14usize)) | (((val as u32) & 0x1f) << 14usize);
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFAMP_TRIM(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFAMP_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LNA_IB(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LNA_IB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFAMP_IB(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFAMP_IB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for CONFIG_RF_FRONTEND_DIV12 {
    #[inline(always)]
    fn default() -> CONFIG_RF_FRONTEND_DIV12 {
        CONFIG_RF_FRONTEND_DIV12(0)
    }
}
impl core::fmt::Debug for CONFIG_RF_FRONTEND_DIV12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_RF_FRONTEND_DIV12")
            .field("RFLDO_TRIM_OUTPUT", &self.RFLDO_TRIM_OUTPUT())
            .field("RESERVED", &self.RESERVED())
            .field("CTL_PA0_TRIM", &self.CTL_PA0_TRIM())
            .field("IFAMP_TRIM", &self.IFAMP_TRIM())
            .field("LNA_IB", &self.LNA_IB())
            .field("IFAMP_IB", &self.IFAMP_IB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_RF_FRONTEND_DIV12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_RF_FRONTEND_DIV12 {{ RFLDO_TRIM_OUTPUT: {=u8:?}, RESERVED: {=u8:?}, CTL_PA0_TRIM: {=u8:?}, IFAMP_TRIM: {=u8:?}, LNA_IB: {=u8:?}, IFAMP_IB: {=u8:?} }}",
            self.RFLDO_TRIM_OUTPUT(),
            self.RESERVED(),
            self.CTL_PA0_TRIM(),
            self.IFAMP_TRIM(),
            self.LNA_IB(),
            self.IFAMP_IB()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_RF_FRONTEND_DIV15(pub u32);
impl CONFIG_RF_FRONTEND_DIV15 {
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RFLDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RFLDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "13:7\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[doc = "13:7\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[doc = "18:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CTL_PA0_TRIM(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x1f;
        val as u8
    }
    #[doc = "18:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CTL_PA0_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 14usize)) | (((val as u32) & 0x1f) << 14usize);
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFAMP_TRIM(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFAMP_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LNA_IB(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LNA_IB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFAMP_IB(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFAMP_IB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for CONFIG_RF_FRONTEND_DIV15 {
    #[inline(always)]
    fn default() -> CONFIG_RF_FRONTEND_DIV15 {
        CONFIG_RF_FRONTEND_DIV15(0)
    }
}
impl core::fmt::Debug for CONFIG_RF_FRONTEND_DIV15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_RF_FRONTEND_DIV15")
            .field("RFLDO_TRIM_OUTPUT", &self.RFLDO_TRIM_OUTPUT())
            .field("RESERVED", &self.RESERVED())
            .field("CTL_PA0_TRIM", &self.CTL_PA0_TRIM())
            .field("IFAMP_TRIM", &self.IFAMP_TRIM())
            .field("LNA_IB", &self.LNA_IB())
            .field("IFAMP_IB", &self.IFAMP_IB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_RF_FRONTEND_DIV15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_RF_FRONTEND_DIV15 {{ RFLDO_TRIM_OUTPUT: {=u8:?}, RESERVED: {=u8:?}, CTL_PA0_TRIM: {=u8:?}, IFAMP_TRIM: {=u8:?}, LNA_IB: {=u8:?}, IFAMP_IB: {=u8:?} }}",
            self.RFLDO_TRIM_OUTPUT(),
            self.RESERVED(),
            self.CTL_PA0_TRIM(),
            self.IFAMP_TRIM(),
            self.LNA_IB(),
            self.IFAMP_IB()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_RF_FRONTEND_DIV30(pub u32);
impl CONFIG_RF_FRONTEND_DIV30 {
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RFLDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RFLDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "13:7\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[doc = "13:7\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[doc = "18:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CTL_PA0_TRIM(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x1f;
        val as u8
    }
    #[doc = "18:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CTL_PA0_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 14usize)) | (((val as u32) & 0x1f) << 14usize);
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFAMP_TRIM(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFAMP_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LNA_IB(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LNA_IB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFAMP_IB(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFAMP_IB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for CONFIG_RF_FRONTEND_DIV30 {
    #[inline(always)]
    fn default() -> CONFIG_RF_FRONTEND_DIV30 {
        CONFIG_RF_FRONTEND_DIV30(0)
    }
}
impl core::fmt::Debug for CONFIG_RF_FRONTEND_DIV30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_RF_FRONTEND_DIV30")
            .field("RFLDO_TRIM_OUTPUT", &self.RFLDO_TRIM_OUTPUT())
            .field("RESERVED", &self.RESERVED())
            .field("CTL_PA0_TRIM", &self.CTL_PA0_TRIM())
            .field("IFAMP_TRIM", &self.IFAMP_TRIM())
            .field("LNA_IB", &self.LNA_IB())
            .field("IFAMP_IB", &self.IFAMP_IB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_RF_FRONTEND_DIV30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_RF_FRONTEND_DIV30 {{ RFLDO_TRIM_OUTPUT: {=u8:?}, RESERVED: {=u8:?}, CTL_PA0_TRIM: {=u8:?}, IFAMP_TRIM: {=u8:?}, LNA_IB: {=u8:?}, IFAMP_IB: {=u8:?} }}",
            self.RFLDO_TRIM_OUTPUT(),
            self.RESERVED(),
            self.CTL_PA0_TRIM(),
            self.IFAMP_TRIM(),
            self.LNA_IB(),
            self.IFAMP_IB()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_RF_FRONTEND_DIV5(pub u32);
impl CONFIG_RF_FRONTEND_DIV5 {
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RFLDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RFLDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "13:7\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[doc = "13:7\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[doc = "18:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CTL_PA0_TRIM(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x1f;
        val as u8
    }
    #[doc = "18:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CTL_PA0_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 14usize)) | (((val as u32) & 0x1f) << 14usize);
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFAMP_TRIM(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFAMP_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LNA_IB(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LNA_IB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFAMP_IB(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFAMP_IB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for CONFIG_RF_FRONTEND_DIV5 {
    #[inline(always)]
    fn default() -> CONFIG_RF_FRONTEND_DIV5 {
        CONFIG_RF_FRONTEND_DIV5(0)
    }
}
impl core::fmt::Debug for CONFIG_RF_FRONTEND_DIV5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_RF_FRONTEND_DIV5")
            .field("RFLDO_TRIM_OUTPUT", &self.RFLDO_TRIM_OUTPUT())
            .field("RESERVED", &self.RESERVED())
            .field("CTL_PA0_TRIM", &self.CTL_PA0_TRIM())
            .field("IFAMP_TRIM", &self.IFAMP_TRIM())
            .field("LNA_IB", &self.LNA_IB())
            .field("IFAMP_IB", &self.IFAMP_IB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_RF_FRONTEND_DIV5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_RF_FRONTEND_DIV5 {{ RFLDO_TRIM_OUTPUT: {=u8:?}, RESERVED: {=u8:?}, CTL_PA0_TRIM: {=u8:?}, IFAMP_TRIM: {=u8:?}, LNA_IB: {=u8:?}, IFAMP_IB: {=u8:?} }}",
            self.RFLDO_TRIM_OUTPUT(),
            self.RESERVED(),
            self.CTL_PA0_TRIM(),
            self.IFAMP_TRIM(),
            self.LNA_IB(),
            self.IFAMP_IB()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_RF_FRONTEND_DIV6(pub u32);
impl CONFIG_RF_FRONTEND_DIV6 {
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RFLDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "6:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RFLDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "13:7\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[doc = "13:7\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[doc = "18:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CTL_PA0_TRIM(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x1f;
        val as u8
    }
    #[doc = "18:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CTL_PA0_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 14usize)) | (((val as u32) & 0x1f) << 14usize);
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFAMP_TRIM(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFAMP_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LNA_IB(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LNA_IB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IFAMP_IB(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IFAMP_IB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for CONFIG_RF_FRONTEND_DIV6 {
    #[inline(always)]
    fn default() -> CONFIG_RF_FRONTEND_DIV6 {
        CONFIG_RF_FRONTEND_DIV6(0)
    }
}
impl core::fmt::Debug for CONFIG_RF_FRONTEND_DIV6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_RF_FRONTEND_DIV6")
            .field("RFLDO_TRIM_OUTPUT", &self.RFLDO_TRIM_OUTPUT())
            .field("RESERVED", &self.RESERVED())
            .field("CTL_PA0_TRIM", &self.CTL_PA0_TRIM())
            .field("IFAMP_TRIM", &self.IFAMP_TRIM())
            .field("LNA_IB", &self.LNA_IB())
            .field("IFAMP_IB", &self.IFAMP_IB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_RF_FRONTEND_DIV6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_RF_FRONTEND_DIV6 {{ RFLDO_TRIM_OUTPUT: {=u8:?}, RESERVED: {=u8:?}, CTL_PA0_TRIM: {=u8:?}, IFAMP_TRIM: {=u8:?}, LNA_IB: {=u8:?}, IFAMP_IB: {=u8:?} }}",
            self.RFLDO_TRIM_OUTPUT(),
            self.RESERVED(),
            self.CTL_PA0_TRIM(),
            self.IFAMP_TRIM(),
            self.LNA_IB(),
            self.IFAMP_IB()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_SYNTH(pub u32);
impl CONFIG_SYNTH {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SLDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SLDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LDOVCO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LDOVCO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "27:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RFC_MDM_DEMIQMC0(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0xffff;
        val as u16
    }
    #[doc = "27:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RFC_MDM_DEMIQMC0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for CONFIG_SYNTH {
    #[inline(always)]
    fn default() -> CONFIG_SYNTH {
        CONFIG_SYNTH(0)
    }
}
impl core::fmt::Debug for CONFIG_SYNTH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_SYNTH")
            .field("SLDO_TRIM_OUTPUT", &self.SLDO_TRIM_OUTPUT())
            .field("LDOVCO_TRIM_OUTPUT", &self.LDOVCO_TRIM_OUTPUT())
            .field("RFC_MDM_DEMIQMC0", &self.RFC_MDM_DEMIQMC0())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_SYNTH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_SYNTH {{ SLDO_TRIM_OUTPUT: {=u8:?}, LDOVCO_TRIM_OUTPUT: {=u8:?}, RFC_MDM_DEMIQMC0: {=u16:?}, RESERVED: {=u8:?} }}",
            self.SLDO_TRIM_OUTPUT(),
            self.LDOVCO_TRIM_OUTPUT(),
            self.RFC_MDM_DEMIQMC0(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_SYNTH_DIV10(pub u32);
impl CONFIG_SYNTH_DIV10 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SLDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SLDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LDOVCO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LDOVCO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "27:12\\] Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn RFC_MDM_DEMIQMC0(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0xffff;
        val as u16
    }
    #[doc = "27:12\\] Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[inline(always)]
    pub const fn set_RFC_MDM_DEMIQMC0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for CONFIG_SYNTH_DIV10 {
    #[inline(always)]
    fn default() -> CONFIG_SYNTH_DIV10 {
        CONFIG_SYNTH_DIV10(0)
    }
}
impl core::fmt::Debug for CONFIG_SYNTH_DIV10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_SYNTH_DIV10")
            .field("SLDO_TRIM_OUTPUT", &self.SLDO_TRIM_OUTPUT())
            .field("LDOVCO_TRIM_OUTPUT", &self.LDOVCO_TRIM_OUTPUT())
            .field("RFC_MDM_DEMIQMC0", &self.RFC_MDM_DEMIQMC0())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_SYNTH_DIV10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_SYNTH_DIV10 {{ SLDO_TRIM_OUTPUT: {=u8:?}, LDOVCO_TRIM_OUTPUT: {=u8:?}, RFC_MDM_DEMIQMC0: {=u16:?}, RESERVED: {=u8:?} }}",
            self.SLDO_TRIM_OUTPUT(),
            self.LDOVCO_TRIM_OUTPUT(),
            self.RFC_MDM_DEMIQMC0(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_SYNTH_DIV12(pub u32);
impl CONFIG_SYNTH_DIV12 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SLDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SLDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LDOVCO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LDOVCO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "27:12\\] Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn RFC_MDM_DEMIQMC0(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0xffff;
        val as u16
    }
    #[doc = "27:12\\] Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[inline(always)]
    pub const fn set_RFC_MDM_DEMIQMC0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for CONFIG_SYNTH_DIV12 {
    #[inline(always)]
    fn default() -> CONFIG_SYNTH_DIV12 {
        CONFIG_SYNTH_DIV12(0)
    }
}
impl core::fmt::Debug for CONFIG_SYNTH_DIV12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_SYNTH_DIV12")
            .field("SLDO_TRIM_OUTPUT", &self.SLDO_TRIM_OUTPUT())
            .field("LDOVCO_TRIM_OUTPUT", &self.LDOVCO_TRIM_OUTPUT())
            .field("RFC_MDM_DEMIQMC0", &self.RFC_MDM_DEMIQMC0())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_SYNTH_DIV12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_SYNTH_DIV12 {{ SLDO_TRIM_OUTPUT: {=u8:?}, LDOVCO_TRIM_OUTPUT: {=u8:?}, RFC_MDM_DEMIQMC0: {=u16:?}, RESERVED: {=u8:?} }}",
            self.SLDO_TRIM_OUTPUT(),
            self.LDOVCO_TRIM_OUTPUT(),
            self.RFC_MDM_DEMIQMC0(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_SYNTH_DIV15(pub u32);
impl CONFIG_SYNTH_DIV15 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SLDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SLDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LDOVCO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LDOVCO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "27:12\\] Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn RFC_MDM_DEMIQMC0(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0xffff;
        val as u16
    }
    #[doc = "27:12\\] Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[inline(always)]
    pub const fn set_RFC_MDM_DEMIQMC0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for CONFIG_SYNTH_DIV15 {
    #[inline(always)]
    fn default() -> CONFIG_SYNTH_DIV15 {
        CONFIG_SYNTH_DIV15(0)
    }
}
impl core::fmt::Debug for CONFIG_SYNTH_DIV15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_SYNTH_DIV15")
            .field("SLDO_TRIM_OUTPUT", &self.SLDO_TRIM_OUTPUT())
            .field("LDOVCO_TRIM_OUTPUT", &self.LDOVCO_TRIM_OUTPUT())
            .field("RFC_MDM_DEMIQMC0", &self.RFC_MDM_DEMIQMC0())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_SYNTH_DIV15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_SYNTH_DIV15 {{ SLDO_TRIM_OUTPUT: {=u8:?}, LDOVCO_TRIM_OUTPUT: {=u8:?}, RFC_MDM_DEMIQMC0: {=u16:?}, RESERVED: {=u8:?} }}",
            self.SLDO_TRIM_OUTPUT(),
            self.LDOVCO_TRIM_OUTPUT(),
            self.RFC_MDM_DEMIQMC0(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_SYNTH_DIV30(pub u32);
impl CONFIG_SYNTH_DIV30 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SLDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SLDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LDOVCO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LDOVCO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "27:12\\] Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn RFC_MDM_DEMIQMC0(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0xffff;
        val as u16
    }
    #[doc = "27:12\\] Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[inline(always)]
    pub const fn set_RFC_MDM_DEMIQMC0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for CONFIG_SYNTH_DIV30 {
    #[inline(always)]
    fn default() -> CONFIG_SYNTH_DIV30 {
        CONFIG_SYNTH_DIV30(0)
    }
}
impl core::fmt::Debug for CONFIG_SYNTH_DIV30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_SYNTH_DIV30")
            .field("SLDO_TRIM_OUTPUT", &self.SLDO_TRIM_OUTPUT())
            .field("LDOVCO_TRIM_OUTPUT", &self.LDOVCO_TRIM_OUTPUT())
            .field("RFC_MDM_DEMIQMC0", &self.RFC_MDM_DEMIQMC0())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_SYNTH_DIV30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_SYNTH_DIV30 {{ SLDO_TRIM_OUTPUT: {=u8:?}, LDOVCO_TRIM_OUTPUT: {=u8:?}, RFC_MDM_DEMIQMC0: {=u16:?}, RESERVED: {=u8:?} }}",
            self.SLDO_TRIM_OUTPUT(),
            self.LDOVCO_TRIM_OUTPUT(),
            self.RFC_MDM_DEMIQMC0(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_SYNTH_DIV5(pub u32);
impl CONFIG_SYNTH_DIV5 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SLDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SLDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LDOVCO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LDOVCO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "27:12\\] Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn RFC_MDM_DEMIQMC0(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0xffff;
        val as u16
    }
    #[doc = "27:12\\] Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[inline(always)]
    pub const fn set_RFC_MDM_DEMIQMC0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for CONFIG_SYNTH_DIV5 {
    #[inline(always)]
    fn default() -> CONFIG_SYNTH_DIV5 {
        CONFIG_SYNTH_DIV5(0)
    }
}
impl core::fmt::Debug for CONFIG_SYNTH_DIV5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_SYNTH_DIV5")
            .field("SLDO_TRIM_OUTPUT", &self.SLDO_TRIM_OUTPUT())
            .field("LDOVCO_TRIM_OUTPUT", &self.LDOVCO_TRIM_OUTPUT())
            .field("RFC_MDM_DEMIQMC0", &self.RFC_MDM_DEMIQMC0())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_SYNTH_DIV5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_SYNTH_DIV5 {{ SLDO_TRIM_OUTPUT: {=u8:?}, LDOVCO_TRIM_OUTPUT: {=u8:?}, RFC_MDM_DEMIQMC0: {=u16:?}, RESERVED: {=u8:?} }}",
            self.SLDO_TRIM_OUTPUT(),
            self.LDOVCO_TRIM_OUTPUT(),
            self.RFC_MDM_DEMIQMC0(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIG_SYNTH_DIV6(pub u32);
impl CONFIG_SYNTH_DIV6 {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SLDO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SLDO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LDOVCO_TRIM_OUTPUT(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LDOVCO_TRIM_OUTPUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "27:12\\] Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn RFC_MDM_DEMIQMC0(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0xffff;
        val as u16
    }
    #[doc = "27:12\\] Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[inline(always)]
    pub const fn set_RFC_MDM_DEMIQMC0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for CONFIG_SYNTH_DIV6 {
    #[inline(always)]
    fn default() -> CONFIG_SYNTH_DIV6 {
        CONFIG_SYNTH_DIV6(0)
    }
}
impl core::fmt::Debug for CONFIG_SYNTH_DIV6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG_SYNTH_DIV6")
            .field("SLDO_TRIM_OUTPUT", &self.SLDO_TRIM_OUTPUT())
            .field("LDOVCO_TRIM_OUTPUT", &self.LDOVCO_TRIM_OUTPUT())
            .field("RFC_MDM_DEMIQMC0", &self.RFC_MDM_DEMIQMC0())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIG_SYNTH_DIV6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONFIG_SYNTH_DIV6 {{ SLDO_TRIM_OUTPUT: {=u8:?}, LDOVCO_TRIM_OUTPUT: {=u8:?}, RFC_MDM_DEMIQMC0: {=u16:?}, RESERVED: {=u8:?} }}",
            self.SLDO_TRIM_OUTPUT(),
            self.LDOVCO_TRIM_OUTPUT(),
            self.RFC_MDM_DEMIQMC0(),
            self.RESERVED()
        )
    }
}
#[doc = "Factory Configuration (FCFG1) Revision."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG1_REVISION(pub u32);
impl FCFG1_REVISION {
    #[doc = "31:0\\] The revision number of the FCFG1 layout. This value will be read by application SW in order to determine which FCFG1 parameters that have valid values. This revision number must be incremented by 1 before any devices are to be produced if the FCFG1 layout has changed since the previous production of devices. Value migth change without warning."]
    #[must_use]
    #[inline(always)]
    pub const fn REV(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] The revision number of the FCFG1 layout. This value will be read by application SW in order to determine which FCFG1 parameters that have valid values. This revision number must be incremented by 1 before any devices are to be produced if the FCFG1 layout has changed since the previous production of devices. Value migth change without warning."]
    #[inline(always)]
    pub const fn set_REV(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FCFG1_REVISION {
    #[inline(always)]
    fn default() -> FCFG1_REVISION {
        FCFG1_REVISION(0)
    }
}
impl core::fmt::Debug for FCFG1_REVISION {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCFG1_REVISION")
            .field("REV", &self.REV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCFG1_REVISION {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FCFG1_REVISION {{ REV: {=u32:?} }}", self.REV())
    }
}
#[doc = "FLASH_COORDINATE."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_COORDINATE(pub u32);
impl FLASH_COORDINATE {
    #[doc = "15:0\\] Y coordinate of this unit on the wafer."]
    #[must_use]
    #[inline(always)]
    pub const fn YCOORDINATE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Y coordinate of this unit on the wafer."]
    #[inline(always)]
    pub const fn set_YCOORDINATE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] X coordinate of this unit on the wafer."]
    #[must_use]
    #[inline(always)]
    pub const fn XCOORDINATE(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] X coordinate of this unit on the wafer."]
    #[inline(always)]
    pub const fn set_XCOORDINATE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FLASH_COORDINATE {
    #[inline(always)]
    fn default() -> FLASH_COORDINATE {
        FLASH_COORDINATE(0)
    }
}
impl core::fmt::Debug for FLASH_COORDINATE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_COORDINATE")
            .field("YCOORDINATE", &self.YCOORDINATE())
            .field("XCOORDINATE", &self.XCOORDINATE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_COORDINATE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASH_COORDINATE {{ YCOORDINATE: {=u16:?}, XCOORDINATE: {=u16:?} }}",
            self.YCOORDINATE(),
            self.XCOORDINATE()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_C_E_P_R(pub u32);
impl FLASH_C_E_P_R {
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CVSU(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CVSU(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn A_EXEZ_SETUP(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_A_EXEZ_SETUP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PV_ACCESS(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PV_ACCESS(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RVSU(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RVSU(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for FLASH_C_E_P_R {
    #[inline(always)]
    fn default() -> FLASH_C_E_P_R {
        FLASH_C_E_P_R(0)
    }
}
impl core::fmt::Debug for FLASH_C_E_P_R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_C_E_P_R")
            .field("CVSU", &self.CVSU())
            .field("A_EXEZ_SETUP", &self.A_EXEZ_SETUP())
            .field("PV_ACCESS", &self.PV_ACCESS())
            .field("RVSU", &self.RVSU())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_C_E_P_R {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASH_C_E_P_R {{ CVSU: {=u16:?}, A_EXEZ_SETUP: {=u8:?}, PV_ACCESS: {=u8:?}, RVSU: {=u8:?} }}",
            self.CVSU(),
            self.A_EXEZ_SETUP(),
            self.PV_ACCESS(),
            self.RVSU()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_EH_SEQ(pub u32);
impl FLASH_EH_SEQ {
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SM_FREQUENCY(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "11:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SM_FREQUENCY(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VSTAT(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VSTAT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SEQ(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SEQ(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EH(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for FLASH_EH_SEQ {
    #[inline(always)]
    fn default() -> FLASH_EH_SEQ {
        FLASH_EH_SEQ(0)
    }
}
impl core::fmt::Debug for FLASH_EH_SEQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_EH_SEQ")
            .field("SM_FREQUENCY", &self.SM_FREQUENCY())
            .field("VSTAT", &self.VSTAT())
            .field("SEQ", &self.SEQ())
            .field("EH", &self.EH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_EH_SEQ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASH_EH_SEQ {{ SM_FREQUENCY: {=u16:?}, VSTAT: {=u8:?}, SEQ: {=u8:?}, EH: {=u8:?} }}",
            self.SM_FREQUENCY(),
            self.VSTAT(),
            self.SEQ(),
            self.EH()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_ERA_PW(pub u32);
impl FLASH_ERA_PW {
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ERASE_PW(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ERASE_PW(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLASH_ERA_PW {
    #[inline(always)]
    fn default() -> FLASH_ERA_PW {
        FLASH_ERA_PW(0)
    }
}
impl core::fmt::Debug for FLASH_ERA_PW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_ERA_PW")
            .field("ERASE_PW", &self.ERASE_PW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_ERA_PW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLASH_ERA_PW {{ ERASE_PW: {=u32:?} }}", self.ERASE_PW())
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_E_P(pub u32);
impl FLASH_E_P {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EVSU(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EVSU(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PVSU(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PVSU(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ESU(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ESU(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PSU(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PSU(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for FLASH_E_P {
    #[inline(always)]
    fn default() -> FLASH_E_P {
        FLASH_E_P(0)
    }
}
impl core::fmt::Debug for FLASH_E_P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_E_P")
            .field("EVSU", &self.EVSU())
            .field("PVSU", &self.PVSU())
            .field("ESU", &self.ESU())
            .field("PSU", &self.PSU())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_E_P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASH_E_P {{ EVSU: {=u8:?}, PVSU: {=u8:?}, ESU: {=u8:?}, PSU: {=u8:?} }}",
            self.EVSU(),
            self.PVSU(),
            self.ESU(),
            self.PSU()
        )
    }
}
#[doc = "FLASH_NUMBER."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_NUMBER(pub u32);
impl FLASH_NUMBER {
    #[doc = "31:0\\] Number of the manufacturing lot that produced this unit."]
    #[must_use]
    #[inline(always)]
    pub const fn LOT_NUMBER(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Number of the manufacturing lot that produced this unit."]
    #[inline(always)]
    pub const fn set_LOT_NUMBER(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLASH_NUMBER {
    #[inline(always)]
    fn default() -> FLASH_NUMBER {
        FLASH_NUMBER(0)
    }
}
impl core::fmt::Debug for FLASH_NUMBER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_NUMBER")
            .field("LOT_NUMBER", &self.LOT_NUMBER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_NUMBER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASH_NUMBER {{ LOT_NUMBER: {=u32:?} }}",
            self.LOT_NUMBER()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_OTP_DATA3(pub u32);
impl FLASH_OTP_DATA3 {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_SYSCODE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_WAIT_SYSCODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASH_SIZE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FLASH_SIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "17:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM_1P7(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "17:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIM_1P7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "21:18\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MAX_EC_LEVEL(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "21:18\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MAX_EC_LEVEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "22:22\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DO_PRECOND(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DO_PRECOND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "31:23\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn EC_STEP_SIZE(&self) -> u16 {
        let val = (self.0 >> 23usize) & 0x01ff;
        val as u16
    }
    #[doc = "31:23\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_EC_STEP_SIZE(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 23usize)) | (((val as u32) & 0x01ff) << 23usize);
    }
}
impl Default for FLASH_OTP_DATA3 {
    #[inline(always)]
    fn default() -> FLASH_OTP_DATA3 {
        FLASH_OTP_DATA3(0)
    }
}
impl core::fmt::Debug for FLASH_OTP_DATA3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_OTP_DATA3")
            .field("WAIT_SYSCODE", &self.WAIT_SYSCODE())
            .field("FLASH_SIZE", &self.FLASH_SIZE())
            .field("TRIM_1P7", &self.TRIM_1P7())
            .field("MAX_EC_LEVEL", &self.MAX_EC_LEVEL())
            .field("DO_PRECOND", &self.DO_PRECOND())
            .field("EC_STEP_SIZE", &self.EC_STEP_SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_OTP_DATA3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASH_OTP_DATA3 {{ WAIT_SYSCODE: {=u8:?}, FLASH_SIZE: {=u8:?}, TRIM_1P7: {=u8:?}, MAX_EC_LEVEL: {=u8:?}, DO_PRECOND: {=bool:?}, EC_STEP_SIZE: {=u16:?} }}",
            self.WAIT_SYSCODE(),
            self.FLASH_SIZE(),
            self.TRIM_1P7(),
            self.MAX_EC_LEVEL(),
            self.DO_PRECOND(),
            self.EC_STEP_SIZE()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_OTP_DATA4(pub u32);
impl FLASH_OTP_DATA4 {
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VIN_AT_X_EXT_RD(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VIN_AT_X_EXT_RD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_IDLE_EXT_RD(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DIS_IDLE_EXT_RD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_STANDBY_EXT_RD(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DIS_STANDBY_EXT_RD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "6:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn STANDBY_PW_SEL_EXT_RD(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "6:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_STANDBY_PW_SEL_EXT_RD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "7:7\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn STANDBY_MODE_SEL_EXT_RD(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_STANDBY_MODE_SEL_EXT_RD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "10:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VIN_AT_X_INT_RD(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "10:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VIN_AT_X_INT_RD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_IDLE_INT_RD(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DIS_IDLE_INT_RD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_STANDBY_INT_RD(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DIS_STANDBY_INT_RD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn STANDBY_PW_SEL_INT_RD(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "14:13\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_STANDBY_PW_SEL_INT_RD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn STANDBY_MODE_SEL_INT_RD(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_STANDBY_MODE_SEL_INT_RD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "18:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VIN_AT_X_EXT_WRT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "18:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VIN_AT_X_EXT_WRT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "19:19\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_IDLE_EXT_WRT(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DIS_IDLE_EXT_WRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_STANDBY_EXT_WRT(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DIS_STANDBY_EXT_WRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "22:21\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn STANDBY_PW_SEL_EXT_WRT(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "22:21\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_STANDBY_PW_SEL_EXT_WRT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
    #[doc = "23:23\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn STANDBY_MODE_SEL_EXT_WRT(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_STANDBY_MODE_SEL_EXT_WRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "26:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VIN_AT_X_INT_WRT(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "26:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VIN_AT_X_INT_WRT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "27:27\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_IDLE_INT_WRT(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DIS_IDLE_INT_WRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_STANDBY_INT_WRT(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DIS_STANDBY_INT_WRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "30:29\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn STANDBY_PW_SEL_INT_WRT(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "30:29\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_STANDBY_PW_SEL_INT_WRT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
    #[doc = "31:31\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn STANDBY_MODE_SEL_INT_WRT(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_STANDBY_MODE_SEL_INT_WRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for FLASH_OTP_DATA4 {
    #[inline(always)]
    fn default() -> FLASH_OTP_DATA4 {
        FLASH_OTP_DATA4(0)
    }
}
impl core::fmt::Debug for FLASH_OTP_DATA4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_OTP_DATA4")
            .field("VIN_AT_X_EXT_RD", &self.VIN_AT_X_EXT_RD())
            .field("DIS_IDLE_EXT_RD", &self.DIS_IDLE_EXT_RD())
            .field("DIS_STANDBY_EXT_RD", &self.DIS_STANDBY_EXT_RD())
            .field("STANDBY_PW_SEL_EXT_RD", &self.STANDBY_PW_SEL_EXT_RD())
            .field("STANDBY_MODE_SEL_EXT_RD", &self.STANDBY_MODE_SEL_EXT_RD())
            .field("VIN_AT_X_INT_RD", &self.VIN_AT_X_INT_RD())
            .field("DIS_IDLE_INT_RD", &self.DIS_IDLE_INT_RD())
            .field("DIS_STANDBY_INT_RD", &self.DIS_STANDBY_INT_RD())
            .field("STANDBY_PW_SEL_INT_RD", &self.STANDBY_PW_SEL_INT_RD())
            .field("STANDBY_MODE_SEL_INT_RD", &self.STANDBY_MODE_SEL_INT_RD())
            .field("VIN_AT_X_EXT_WRT", &self.VIN_AT_X_EXT_WRT())
            .field("DIS_IDLE_EXT_WRT", &self.DIS_IDLE_EXT_WRT())
            .field("DIS_STANDBY_EXT_WRT", &self.DIS_STANDBY_EXT_WRT())
            .field("STANDBY_PW_SEL_EXT_WRT", &self.STANDBY_PW_SEL_EXT_WRT())
            .field("STANDBY_MODE_SEL_EXT_WRT", &self.STANDBY_MODE_SEL_EXT_WRT())
            .field("VIN_AT_X_INT_WRT", &self.VIN_AT_X_INT_WRT())
            .field("DIS_IDLE_INT_WRT", &self.DIS_IDLE_INT_WRT())
            .field("DIS_STANDBY_INT_WRT", &self.DIS_STANDBY_INT_WRT())
            .field("STANDBY_PW_SEL_INT_WRT", &self.STANDBY_PW_SEL_INT_WRT())
            .field("STANDBY_MODE_SEL_INT_WRT", &self.STANDBY_MODE_SEL_INT_WRT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_OTP_DATA4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASH_OTP_DATA4 {{ VIN_AT_X_EXT_RD: {=u8:?}, DIS_IDLE_EXT_RD: {=bool:?}, DIS_STANDBY_EXT_RD: {=bool:?}, STANDBY_PW_SEL_EXT_RD: {=u8:?}, STANDBY_MODE_SEL_EXT_RD: {=bool:?}, VIN_AT_X_INT_RD: {=u8:?}, DIS_IDLE_INT_RD: {=bool:?}, DIS_STANDBY_INT_RD: {=bool:?}, STANDBY_PW_SEL_INT_RD: {=u8:?}, STANDBY_MODE_SEL_INT_RD: {=bool:?}, VIN_AT_X_EXT_WRT: {=u8:?}, DIS_IDLE_EXT_WRT: {=bool:?}, DIS_STANDBY_EXT_WRT: {=bool:?}, STANDBY_PW_SEL_EXT_WRT: {=u8:?}, STANDBY_MODE_SEL_EXT_WRT: {=bool:?}, VIN_AT_X_INT_WRT: {=u8:?}, DIS_IDLE_INT_WRT: {=bool:?}, DIS_STANDBY_INT_WRT: {=bool:?}, STANDBY_PW_SEL_INT_WRT: {=u8:?}, STANDBY_MODE_SEL_INT_WRT: {=bool:?} }}",
            self.VIN_AT_X_EXT_RD(),
            self.DIS_IDLE_EXT_RD(),
            self.DIS_STANDBY_EXT_RD(),
            self.STANDBY_PW_SEL_EXT_RD(),
            self.STANDBY_MODE_SEL_EXT_RD(),
            self.VIN_AT_X_INT_RD(),
            self.DIS_IDLE_INT_RD(),
            self.DIS_STANDBY_INT_RD(),
            self.STANDBY_PW_SEL_INT_RD(),
            self.STANDBY_MODE_SEL_INT_RD(),
            self.VIN_AT_X_EXT_WRT(),
            self.DIS_IDLE_EXT_WRT(),
            self.DIS_STANDBY_EXT_WRT(),
            self.STANDBY_PW_SEL_EXT_WRT(),
            self.STANDBY_MODE_SEL_EXT_WRT(),
            self.VIN_AT_X_INT_WRT(),
            self.DIS_IDLE_INT_WRT(),
            self.DIS_STANDBY_INT_WRT(),
            self.STANDBY_PW_SEL_INT_WRT(),
            self.STANDBY_MODE_SEL_INT_WRT()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_PP(pub u32);
impl FLASH_PP {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MAX_PP(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MAX_PP(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PUMP_SU(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PUMP_SU(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for FLASH_PP {
    #[inline(always)]
    fn default() -> FLASH_PP {
        FLASH_PP(0)
    }
}
impl core::fmt::Debug for FLASH_PP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_PP")
            .field("MAX_PP", &self.MAX_PP())
            .field("RESERVED", &self.RESERVED())
            .field("PUMP_SU", &self.PUMP_SU())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_PP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASH_PP {{ MAX_PP: {=u16:?}, RESERVED: {=u8:?}, PUMP_SU: {=u8:?} }}",
            self.MAX_PP(),
            self.RESERVED(),
            self.PUMP_SU()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_PROG_EP(pub u32);
impl FLASH_PROG_EP {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PROGRAM_PW(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PROGRAM_PW(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MAX_EP(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MAX_EP(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FLASH_PROG_EP {
    #[inline(always)]
    fn default() -> FLASH_PROG_EP {
        FLASH_PROG_EP(0)
    }
}
impl core::fmt::Debug for FLASH_PROG_EP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_PROG_EP")
            .field("PROGRAM_PW", &self.PROGRAM_PW())
            .field("MAX_EP", &self.MAX_EP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_PROG_EP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASH_PROG_EP {{ PROGRAM_PW: {=u16:?}, MAX_EP: {=u16:?} }}",
            self.PROGRAM_PW(),
            self.MAX_EP()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_P_R_PV(pub u32);
impl FLASH_P_R_PV {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PVH2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PVH2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PVH(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PVH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RH(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PH(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for FLASH_P_R_PV {
    #[inline(always)]
    fn default() -> FLASH_P_R_PV {
        FLASH_P_R_PV(0)
    }
}
impl core::fmt::Debug for FLASH_P_R_PV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_P_R_PV")
            .field("PVH2", &self.PVH2())
            .field("PVH", &self.PVH())
            .field("RH", &self.RH())
            .field("PH", &self.PH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_P_R_PV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASH_P_R_PV {{ PVH2: {=u8:?}, PVH: {=u8:?}, RH: {=u8:?}, PH: {=u8:?} }}",
            self.PVH2(),
            self.PVH(),
            self.RH(),
            self.PH()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_V(pub u32);
impl FLASH_V {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn V_READ(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_V_READ(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VWL_P(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VWL_P(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VSL_P(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VSL_P(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for FLASH_V {
    #[inline(always)]
    fn default() -> FLASH_V {
        FLASH_V(0)
    }
}
impl core::fmt::Debug for FLASH_V {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_V")
            .field("RESERVED", &self.RESERVED())
            .field("V_READ", &self.V_READ())
            .field("VWL_P", &self.VWL_P())
            .field("VSL_P", &self.VSL_P())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_V {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASH_V {{ RESERVED: {=u8:?}, V_READ: {=u8:?}, VWL_P: {=u8:?}, VSL_P: {=u8:?} }}",
            self.RESERVED(),
            self.V_READ(),
            self.VWL_P(),
            self.VSL_P()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_VHV(pub u32);
impl FLASH_VHV {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VHV_E(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VHV_E(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "7:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM13_E(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIM13_E(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VHV_P(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VHV_P(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM13_P(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIM13_P(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FLASH_VHV {
    #[inline(always)]
    fn default() -> FLASH_VHV {
        FLASH_VHV(0)
    }
}
impl core::fmt::Debug for FLASH_VHV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_VHV")
            .field("VHV_E", &self.VHV_E())
            .field("RESERVED0", &self.RESERVED0())
            .field("TRIM13_E", &self.TRIM13_E())
            .field("RESERVED1", &self.RESERVED1())
            .field("VHV_P", &self.VHV_P())
            .field("RESERVED2", &self.RESERVED2())
            .field("TRIM13_P", &self.TRIM13_P())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_VHV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASH_VHV {{ VHV_E: {=u8:?}, RESERVED0: {=u8:?}, TRIM13_E: {=u8:?}, RESERVED1: {=u8:?}, VHV_P: {=u8:?}, RESERVED2: {=u8:?}, TRIM13_P: {=u8:?}, RESERVED3: {=u8:?} }}",
            self.VHV_E(),
            self.RESERVED0(),
            self.TRIM13_E(),
            self.RESERVED1(),
            self.VHV_P(),
            self.RESERVED2(),
            self.TRIM13_P(),
            self.RESERVED3()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_VHV_E(pub u32);
impl FLASH_VHV_E {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VHV_E_STEP_HIGHT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VHV_E_STEP_HIGHT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VHV_E_START(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VHV_E_START(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FLASH_VHV_E {
    #[inline(always)]
    fn default() -> FLASH_VHV_E {
        FLASH_VHV_E(0)
    }
}
impl core::fmt::Debug for FLASH_VHV_E {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_VHV_E")
            .field("VHV_E_STEP_HIGHT", &self.VHV_E_STEP_HIGHT())
            .field("VHV_E_START", &self.VHV_E_START())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_VHV_E {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASH_VHV_E {{ VHV_E_STEP_HIGHT: {=u16:?}, VHV_E_START: {=u16:?} }}",
            self.VHV_E_STEP_HIGHT(),
            self.VHV_E_START()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_VHV_PV(pub u32);
impl FLASH_VHV_PV {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VINH(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VINH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VCG2P5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VCG2P5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VHV_PV(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VHV_PV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM13_PV(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIM13_PV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FLASH_VHV_PV {
    #[inline(always)]
    fn default() -> FLASH_VHV_PV {
        FLASH_VHV_PV(0)
    }
}
impl core::fmt::Debug for FLASH_VHV_PV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_VHV_PV")
            .field("VINH", &self.VINH())
            .field("VCG2P5", &self.VCG2P5())
            .field("VHV_PV", &self.VHV_PV())
            .field("RESERVED0", &self.RESERVED0())
            .field("TRIM13_PV", &self.TRIM13_PV())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_VHV_PV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASH_VHV_PV {{ VINH: {=u8:?}, VCG2P5: {=u8:?}, VHV_PV: {=u8:?}, RESERVED0: {=u8:?}, TRIM13_PV: {=u8:?}, RESERVED1: {=u8:?} }}",
            self.VINH(),
            self.VCG2P5(),
            self.VHV_PV(),
            self.RESERVED0(),
            self.TRIM13_PV(),
            self.RESERVED1()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FREQ_OFFSET(pub u32);
impl FREQ_OFFSET {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPOSC_COMP_P2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPOSC_COMP_P2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPOSC_COMP_P1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPOSC_COMP_P1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPOSC_COMP_P0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPOSC_COMP_P0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FREQ_OFFSET {
    #[inline(always)]
    fn default() -> FREQ_OFFSET {
        FREQ_OFFSET(0)
    }
}
impl core::fmt::Debug for FREQ_OFFSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FREQ_OFFSET")
            .field("HPOSC_COMP_P2", &self.HPOSC_COMP_P2())
            .field("HPOSC_COMP_P1", &self.HPOSC_COMP_P1())
            .field("HPOSC_COMP_P0", &self.HPOSC_COMP_P0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FREQ_OFFSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FREQ_OFFSET {{ HPOSC_COMP_P2: {=u8:?}, HPOSC_COMP_P1: {=u8:?}, HPOSC_COMP_P0: {=u16:?} }}",
            self.HPOSC_COMP_P2(),
            self.HPOSC_COMP_P1(),
            self.HPOSC_COMP_P0()
        )
    }
}
#[doc = "IcePick Device Identification Reading this register and the USER_ID register is the only support way of identifying a device."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ICEPICK_DEVICE_ID(pub u32);
impl ICEPICK_DEVICE_ID {
    #[doc = "11:0\\] Manufacturer code. 0x02F: Texas Instruments."]
    #[must_use]
    #[inline(always)]
    pub const fn MANUFACTURER_ID(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "11:0\\] Manufacturer code. 0x02F: Texas Instruments."]
    #[inline(always)]
    pub const fn set_MANUFACTURER_ID(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "27:12\\] Field used to identify silicon die."]
    #[must_use]
    #[inline(always)]
    pub const fn WAFER_ID(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0xffff;
        val as u16
    }
    #[doc = "27:12\\] Field used to identify silicon die."]
    #[inline(always)]
    pub const fn set_WAFER_ID(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
    }
    #[doc = "31:28\\] Field used to distinguish revisions of the device."]
    #[must_use]
    #[inline(always)]
    pub const fn PG_REV(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Field used to distinguish revisions of the device."]
    #[inline(always)]
    pub const fn set_PG_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for ICEPICK_DEVICE_ID {
    #[inline(always)]
    fn default() -> ICEPICK_DEVICE_ID {
        ICEPICK_DEVICE_ID(0)
    }
}
impl core::fmt::Debug for ICEPICK_DEVICE_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICEPICK_DEVICE_ID")
            .field("MANUFACTURER_ID", &self.MANUFACTURER_ID())
            .field("WAFER_ID", &self.WAFER_ID())
            .field("PG_REV", &self.PG_REV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ICEPICK_DEVICE_ID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ICEPICK_DEVICE_ID {{ MANUFACTURER_ID: {=u16:?}, WAFER_ID: {=u16:?}, PG_REV: {=u8:?} }}",
            self.MANUFACTURER_ID(),
            self.WAFER_ID(),
            self.PG_REV()
        )
    }
}
#[doc = "IO Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOCONF(pub u32);
impl IOCONF {
    #[doc = "6:0\\] Number of available DIOs."]
    #[must_use]
    #[inline(always)]
    pub const fn GPIO_CNT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "6:0\\] Number of available DIOs."]
    #[inline(always)]
    pub const fn set_GPIO_CNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED7(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for IOCONF {
    #[inline(always)]
    fn default() -> IOCONF {
        IOCONF(0)
    }
}
impl core::fmt::Debug for IOCONF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOCONF")
            .field("GPIO_CNT", &self.GPIO_CNT())
            .field("RESERVED7", &self.RESERVED7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOCONF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOCONF {{ GPIO_CNT: {=u8:?}, RESERVED7: {=u32:?} }}",
            self.GPIO_CNT(),
            self.RESERVED7()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LDO_TRIM(pub u32);
impl LDO_TRIM {
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VTRIM_DELTA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VTRIM_DELTA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "7:3\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "7:3\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
    }
    #[doc = "10:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ITRIM_UDIGLDO(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "10:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ITRIM_UDIGLDO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "12:11\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ITRIM_DIGLDO_LOAD(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "12:11\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ITRIM_DIGLDO_LOAD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[doc = "15:13\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "15:13\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "18:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn GLDO_CURSRC(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "18:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_GLDO_CURSRC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "23:19\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "28:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_TRIM_SLEEP(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "28:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VDDR_TRIM_SLEEP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "31:29\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "31:29\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for LDO_TRIM {
    #[inline(always)]
    fn default() -> LDO_TRIM {
        LDO_TRIM(0)
    }
}
impl core::fmt::Debug for LDO_TRIM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LDO_TRIM")
            .field("VTRIM_DELTA", &self.VTRIM_DELTA())
            .field("RESERVED1", &self.RESERVED1())
            .field("ITRIM_UDIGLDO", &self.ITRIM_UDIGLDO())
            .field("ITRIM_DIGLDO_LOAD", &self.ITRIM_DIGLDO_LOAD())
            .field("RESERVED2", &self.RESERVED2())
            .field("GLDO_CURSRC", &self.GLDO_CURSRC())
            .field("RESERVED3", &self.RESERVED3())
            .field("VDDR_TRIM_SLEEP", &self.VDDR_TRIM_SLEEP())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LDO_TRIM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LDO_TRIM {{ VTRIM_DELTA: {=u8:?}, RESERVED1: {=u8:?}, ITRIM_UDIGLDO: {=u8:?}, ITRIM_DIGLDO_LOAD: {=u8:?}, RESERVED2: {=u8:?}, GLDO_CURSRC: {=u8:?}, RESERVED3: {=u8:?}, VDDR_TRIM_SLEEP: {=u8:?}, RESERVED4: {=u8:?} }}",
            self.VTRIM_DELTA(),
            self.RESERVED1(),
            self.ITRIM_UDIGLDO(),
            self.ITRIM_DIGLDO_LOAD(),
            self.RESERVED2(),
            self.GLDO_CURSRC(),
            self.RESERVED3(),
            self.VDDR_TRIM_SLEEP(),
            self.RESERVED4()
        )
    }
}
#[doc = "MAC IEEE 802.15.4 Address 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MAC_15_4_0(pub u32);
impl MAC_15_4_0 {
    #[doc = "31:0\\] The first 32-bits of the 64-bit MAC 15.4 address."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR_0_31(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] The first 32-bits of the 64-bit MAC 15.4 address."]
    #[inline(always)]
    pub const fn set_ADDR_0_31(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MAC_15_4_0 {
    #[inline(always)]
    fn default() -> MAC_15_4_0 {
        MAC_15_4_0(0)
    }
}
impl core::fmt::Debug for MAC_15_4_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC_15_4_0")
            .field("ADDR_0_31", &self.ADDR_0_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MAC_15_4_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MAC_15_4_0 {{ ADDR_0_31: {=u32:?} }}", self.ADDR_0_31())
    }
}
#[doc = "MAC IEEE 802.15.4 Address 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MAC_15_4_1(pub u32);
impl MAC_15_4_1 {
    #[doc = "31:0\\] The last 32-bits of the 64-bit MAC 15.4 address."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR_32_63(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] The last 32-bits of the 64-bit MAC 15.4 address."]
    #[inline(always)]
    pub const fn set_ADDR_32_63(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MAC_15_4_1 {
    #[inline(always)]
    fn default() -> MAC_15_4_1 {
        MAC_15_4_1(0)
    }
}
impl core::fmt::Debug for MAC_15_4_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC_15_4_1")
            .field("ADDR_32_63", &self.ADDR_32_63())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MAC_15_4_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MAC_15_4_1 {{ ADDR_32_63: {=u32:?} }}",
            self.ADDR_32_63()
        )
    }
}
#[doc = "MAC BLE Address 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MAC_BLE_0(pub u32);
impl MAC_BLE_0 {
    #[doc = "31:0\\] The first 32-bits of the 64-bit MAC BLE address."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR_0_31(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] The first 32-bits of the 64-bit MAC BLE address."]
    #[inline(always)]
    pub const fn set_ADDR_0_31(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MAC_BLE_0 {
    #[inline(always)]
    fn default() -> MAC_BLE_0 {
        MAC_BLE_0(0)
    }
}
impl core::fmt::Debug for MAC_BLE_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC_BLE_0")
            .field("ADDR_0_31", &self.ADDR_0_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MAC_BLE_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MAC_BLE_0 {{ ADDR_0_31: {=u32:?} }}", self.ADDR_0_31())
    }
}
#[doc = "MAC BLE Address 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MAC_BLE_1(pub u32);
impl MAC_BLE_1 {
    #[doc = "31:0\\] The last 32-bits of the 64-bit MAC BLE address."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR_32_63(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] The last 32-bits of the 64-bit MAC BLE address."]
    #[inline(always)]
    pub const fn set_ADDR_32_63(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MAC_BLE_1 {
    #[inline(always)]
    fn default() -> MAC_BLE_1 {
        MAC_BLE_1(0)
    }
}
impl core::fmt::Debug for MAC_BLE_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC_BLE_1")
            .field("ADDR_32_63", &self.ADDR_32_63())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MAC_BLE_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MAC_BLE_1 {{ ADDR_32_63: {=u32:?} }}", self.ADDR_32_63())
    }
}
#[doc = "Misc configurations."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MISC_CONF_1(pub u32);
impl MISC_CONF_1 {
    #[doc = "7:0\\] HW minor revision number (a value of 0xFF shall be treated equally to 0x00). Any test of this field by SW should be implemented as a 'greater or equal' comparison as signed integer. Value may change without warning."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVICE_MINOR_REV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] HW minor revision number (a value of 0xFF shall be treated equally to 0x00). Any test of this field by SW should be implemented as a 'greater or equal' comparison as signed integer. Value may change without warning."]
    #[inline(always)]
    pub const fn set_DEVICE_MINOR_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for MISC_CONF_1 {
    #[inline(always)]
    fn default() -> MISC_CONF_1 {
        MISC_CONF_1(0)
    }
}
impl core::fmt::Debug for MISC_CONF_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC_CONF_1")
            .field("DEVICE_MINOR_REV", &self.DEVICE_MINOR_REV())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MISC_CONF_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MISC_CONF_1 {{ DEVICE_MINOR_REV: {=u8:?}, RESERVED: {=u32:?} }}",
            self.DEVICE_MINOR_REV(),
            self.RESERVED()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MISC_CONF_2(pub u32);
impl MISC_CONF_2 {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPOSC_COMP_P3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPOSC_COMP_P3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for MISC_CONF_2 {
    #[inline(always)]
    fn default() -> MISC_CONF_2 {
        MISC_CONF_2(0)
    }
}
impl core::fmt::Debug for MISC_CONF_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC_CONF_2")
            .field("HPOSC_COMP_P3", &self.HPOSC_COMP_P3())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MISC_CONF_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MISC_CONF_2 {{ HPOSC_COMP_P3: {=u8:?}, RESERVED: {=u32:?} }}",
            self.HPOSC_COMP_P3(),
            self.RESERVED()
        )
    }
}
#[doc = "Misc OTP Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MISC_OTP_DATA(pub u32);
impl MISC_OTP_DATA {
    #[doc = "7:0\\] The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
    #[must_use]
    #[inline(always)]
    pub const fn TEST_PROGRAM_REV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
    #[inline(always)]
    pub const fn set_TEST_PROGRAM_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PO_TAIL_RES_TRIM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PO_TAIL_RES_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "14:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PER_E(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "14:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PER_E(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "19:15\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PER_M(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "19:15\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PER_M(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[doc = "27:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSC_HF_CRIM(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0xff;
        val as u8
    }
    #[doc = "27:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RCOSC_HF_CRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSC_HF_ITUNE(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RCOSC_HF_ITUNE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for MISC_OTP_DATA {
    #[inline(always)]
    fn default() -> MISC_OTP_DATA {
        MISC_OTP_DATA(0)
    }
}
impl core::fmt::Debug for MISC_OTP_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC_OTP_DATA")
            .field("TEST_PROGRAM_REV", &self.TEST_PROGRAM_REV())
            .field("PO_TAIL_RES_TRIM", &self.PO_TAIL_RES_TRIM())
            .field("PER_E", &self.PER_E())
            .field("PER_M", &self.PER_M())
            .field("RCOSC_HF_CRIM", &self.RCOSC_HF_CRIM())
            .field("RCOSC_HF_ITUNE", &self.RCOSC_HF_ITUNE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MISC_OTP_DATA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MISC_OTP_DATA {{ TEST_PROGRAM_REV: {=u8:?}, PO_TAIL_RES_TRIM: {=u8:?}, PER_E: {=u8:?}, PER_M: {=u8:?}, RCOSC_HF_CRIM: {=u8:?}, RCOSC_HF_ITUNE: {=u8:?} }}",
            self.TEST_PROGRAM_REV(),
            self.PO_TAIL_RES_TRIM(),
            self.PER_E(),
            self.PER_M(),
            self.RCOSC_HF_CRIM(),
            self.RCOSC_HF_ITUNE()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MISC_OTP_DATA_1(pub u32);
impl MISC_OTP_DATA_1 {
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IDAC_STEP(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IDAC_STEP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "9:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_IBIAS_WAIT_CNT(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x3f;
        val as u8
    }
    #[doc = "9:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LPM_IBIAS_WAIT_CNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
    }
    #[doc = "19:10\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPM_IBIAS_WAIT_CNT(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[doc = "19:10\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPM_IBIAS_WAIT_CNT(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
    #[doc = "21:20\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DBLR_LOOP_FILTER_RESET_VOLTAGE(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "21:20\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DBLR_LOOP_FILTER_RESET_VOLTAGE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "23:22\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LP_BUF_ITRIM(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "23:22\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LP_BUF_ITRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "26:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HP_BUF_ITRIM(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "26:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HP_BUF_ITRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "28:27\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PEAK_DET_ITRIM(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PEAK_DET_ITRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "31:29\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "31:29\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for MISC_OTP_DATA_1 {
    #[inline(always)]
    fn default() -> MISC_OTP_DATA_1 {
        MISC_OTP_DATA_1(0)
    }
}
impl core::fmt::Debug for MISC_OTP_DATA_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC_OTP_DATA_1")
            .field("IDAC_STEP", &self.IDAC_STEP())
            .field("LPM_IBIAS_WAIT_CNT", &self.LPM_IBIAS_WAIT_CNT())
            .field("HPM_IBIAS_WAIT_CNT", &self.HPM_IBIAS_WAIT_CNT())
            .field(
                "DBLR_LOOP_FILTER_RESET_VOLTAGE",
                &self.DBLR_LOOP_FILTER_RESET_VOLTAGE(),
            )
            .field("LP_BUF_ITRIM", &self.LP_BUF_ITRIM())
            .field("HP_BUF_ITRIM", &self.HP_BUF_ITRIM())
            .field("PEAK_DET_ITRIM", &self.PEAK_DET_ITRIM())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MISC_OTP_DATA_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MISC_OTP_DATA_1 {{ IDAC_STEP: {=u8:?}, LPM_IBIAS_WAIT_CNT: {=u8:?}, HPM_IBIAS_WAIT_CNT: {=u16:?}, DBLR_LOOP_FILTER_RESET_VOLTAGE: {=u8:?}, LP_BUF_ITRIM: {=u8:?}, HP_BUF_ITRIM: {=u8:?}, PEAK_DET_ITRIM: {=u8:?}, RESERVED: {=u8:?} }}",
            self.IDAC_STEP(),
            self.LPM_IBIAS_WAIT_CNT(),
            self.HPM_IBIAS_WAIT_CNT(),
            self.DBLR_LOOP_FILTER_RESET_VOLTAGE(),
            self.LP_BUF_ITRIM(),
            self.HP_BUF_ITRIM(),
            self.PEAK_DET_ITRIM(),
            self.RESERVED()
        )
    }
}
#[doc = "Miscellaneous Trim Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MISC_TRIM(pub u32);
impl MISC_TRIM {
    #[doc = "7:0\\] Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
    #[must_use]
    #[inline(always)]
    pub const fn TEMPVSLOPE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
    #[inline(always)]
    pub const fn set_TEMPVSLOPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for MISC_TRIM {
    #[inline(always)]
    fn default() -> MISC_TRIM {
        MISC_TRIM(0)
    }
}
impl core::fmt::Debug for MISC_TRIM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC_TRIM")
            .field("TEMPVSLOPE", &self.TEMPVSLOPE())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MISC_TRIM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MISC_TRIM {{ TEMPVSLOPE: {=u8:?}, RESERVED: {=u32:?} }}",
            self.TEMPVSLOPE(),
            self.RESERVED()
        )
    }
}
#[doc = "OSC Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OSC_CONF(pub u32);
impl OSC_CONF {
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPOSC_DIV3_BYPASS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPOSC_DIV3_BYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "2:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPOSC_SERIES_CAP(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "2:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPOSC_SERIES_CAP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "4:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "4:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "6:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPOSC_BIAS_RECHARGE_DELAY(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "6:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPOSC_BIAS_RECHARGE_DELAY(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "7:7\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPOSC_FILTER_EN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPOSC_FILTER_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPOSC_BIAS_RES_SET(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPOSC_BIAS_RES_SET(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPOSC_CURRMIRR_RATIO(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPOSC_CURRMIRR_RATIO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPOSC_BIAS_HOLD_MODE_EN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPOSC_BIAS_HOLD_MODE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPOSC_OPTION(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPOSC_OPTION(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] 0: XOSC_HF unavailable (may not be bonded out) 1: XOSC_HF available (default)."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_OPTION(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: XOSC_HF unavailable (may not be bonded out) 1: XOSC_HF available (default)."]
    #[inline(always)]
    pub const fn set_XOSC_OPTION(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "20:19\\] Trim value for DDI_0_OSC:CTL1.XOSC_HF_FAST_START."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_HF_FAST_START(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x03;
        val as u8
    }
    #[doc = "20:19\\] Trim value for DDI_0_OSC:CTL1.XOSC_HF_FAST_START."]
    #[inline(always)]
    pub const fn set_XOSC_HF_FAST_START(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
    }
    #[doc = "24:21\\] Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_CMIRRWR_RATIO."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSCLF_CMIRRWR_RATIO(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x0f;
        val as u8
    }
    #[doc = "24:21\\] Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_CMIRRWR_RATIO."]
    #[inline(always)]
    pub const fn set_XOSCLF_CMIRRWR_RATIO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
    }
    #[doc = "26:25\\] Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_REGULATOR_TRIM."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSCLF_REGULATOR_TRIM(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x03;
        val as u8
    }
    #[doc = "26:25\\] Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_REGULATOR_TRIM."]
    #[inline(always)]
    pub const fn set_XOSCLF_REGULATOR_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
    }
    #[doc = "27:27\\] Trim value for DDI_0_OSC:ATESTCTL.ATESTLF_RCOSCLF_IBIAS_TRIM."]
    #[must_use]
    #[inline(always)]
    pub const fn ATESTLF_RCOSCLF_IBIAS_TRIM(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Trim value for DDI_0_OSC:ATESTCTL.ATESTLF_RCOSCLF_IBIAS_TRIM."]
    #[inline(always)]
    pub const fn set_ATESTLF_RCOSCLF_IBIAS_TRIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_MODE_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_SH_MODE_EN(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_MODE_EN."]
    #[inline(always)]
    pub const fn set_ADC_SH_MODE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_VBUF_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_SH_VBUF_EN(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_VBUF_EN."]
    #[inline(always)]
    pub const fn set_ADC_SH_VBUF_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "31:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "31:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for OSC_CONF {
    #[inline(always)]
    fn default() -> OSC_CONF {
        OSC_CONF(0)
    }
}
impl core::fmt::Debug for OSC_CONF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSC_CONF")
            .field("HPOSC_DIV3_BYPASS", &self.HPOSC_DIV3_BYPASS())
            .field("HPOSC_SERIES_CAP", &self.HPOSC_SERIES_CAP())
            .field("RESERVED2", &self.RESERVED2())
            .field(
                "HPOSC_BIAS_RECHARGE_DELAY",
                &self.HPOSC_BIAS_RECHARGE_DELAY(),
            )
            .field("HPOSC_FILTER_EN", &self.HPOSC_FILTER_EN())
            .field("HPOSC_BIAS_RES_SET", &self.HPOSC_BIAS_RES_SET())
            .field("HPOSC_CURRMIRR_RATIO", &self.HPOSC_CURRMIRR_RATIO())
            .field("HPOSC_BIAS_HOLD_MODE_EN", &self.HPOSC_BIAS_HOLD_MODE_EN())
            .field("HPOSC_OPTION", &self.HPOSC_OPTION())
            .field("XOSC_OPTION", &self.XOSC_OPTION())
            .field("XOSC_HF_FAST_START", &self.XOSC_HF_FAST_START())
            .field("XOSCLF_CMIRRWR_RATIO", &self.XOSCLF_CMIRRWR_RATIO())
            .field("XOSCLF_REGULATOR_TRIM", &self.XOSCLF_REGULATOR_TRIM())
            .field(
                "ATESTLF_RCOSCLF_IBIAS_TRIM",
                &self.ATESTLF_RCOSCLF_IBIAS_TRIM(),
            )
            .field("ADC_SH_MODE_EN", &self.ADC_SH_MODE_EN())
            .field("ADC_SH_VBUF_EN", &self.ADC_SH_VBUF_EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OSC_CONF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OSC_CONF {{ HPOSC_DIV3_BYPASS: {=bool:?}, HPOSC_SERIES_CAP: {=u8:?}, RESERVED2: {=u8:?}, HPOSC_BIAS_RECHARGE_DELAY: {=u8:?}, HPOSC_FILTER_EN: {=bool:?}, HPOSC_BIAS_RES_SET: {=u8:?}, HPOSC_CURRMIRR_RATIO: {=u8:?}, HPOSC_BIAS_HOLD_MODE_EN: {=bool:?}, HPOSC_OPTION: {=bool:?}, XOSC_OPTION: {=bool:?}, XOSC_HF_FAST_START: {=u8:?}, XOSCLF_CMIRRWR_RATIO: {=u8:?}, XOSCLF_REGULATOR_TRIM: {=u8:?}, ATESTLF_RCOSCLF_IBIAS_TRIM: {=bool:?}, ADC_SH_MODE_EN: {=bool:?}, ADC_SH_VBUF_EN: {=bool:?}, RESERVED1: {=u8:?} }}",
            self.HPOSC_DIV3_BYPASS(),
            self.HPOSC_SERIES_CAP(),
            self.RESERVED2(),
            self.HPOSC_BIAS_RECHARGE_DELAY(),
            self.HPOSC_FILTER_EN(),
            self.HPOSC_BIAS_RES_SET(),
            self.HPOSC_CURRMIRR_RATIO(),
            self.HPOSC_BIAS_HOLD_MODE_EN(),
            self.HPOSC_OPTION(),
            self.XOSC_OPTION(),
            self.XOSC_HF_FAST_START(),
            self.XOSCLF_CMIRRWR_RATIO(),
            self.XOSCLF_REGULATOR_TRIM(),
            self.ATESTLF_RCOSCLF_IBIAS_TRIM(),
            self.ADC_SH_MODE_EN(),
            self.ADC_SH_VBUF_EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "Power Down Current Control 110C."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_CURR_110C(pub u32);
impl PWD_CURR_110C {
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[must_use]
    #[inline(always)]
    pub const fn BASELINE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[inline(always)]
    pub const fn set_BASELINE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_XOSC_LPM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[inline(always)]
    pub const fn set_DELTA_XOSC_LPM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_RFMEM_RET(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[inline(always)]
    pub const fn set_DELTA_RFMEM_RET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_CACHE_REF(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[inline(always)]
    pub const fn set_DELTA_CACHE_REF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for PWD_CURR_110C {
    #[inline(always)]
    fn default() -> PWD_CURR_110C {
        PWD_CURR_110C(0)
    }
}
impl core::fmt::Debug for PWD_CURR_110C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_CURR_110C")
            .field("BASELINE", &self.BASELINE())
            .field("DELTA_XOSC_LPM", &self.DELTA_XOSC_LPM())
            .field("DELTA_RFMEM_RET", &self.DELTA_RFMEM_RET())
            .field("DELTA_CACHE_REF", &self.DELTA_CACHE_REF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_CURR_110C {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_CURR_110C {{ BASELINE: {=u8:?}, DELTA_XOSC_LPM: {=u8:?}, DELTA_RFMEM_RET: {=u8:?}, DELTA_CACHE_REF: {=u8:?} }}",
            self.BASELINE(),
            self.DELTA_XOSC_LPM(),
            self.DELTA_RFMEM_RET(),
            self.DELTA_CACHE_REF()
        )
    }
}
#[doc = "Power Down Current Control 125C."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_CURR_125C(pub u32);
impl PWD_CURR_125C {
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[must_use]
    #[inline(always)]
    pub const fn BASELINE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[inline(always)]
    pub const fn set_BASELINE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_XOSC_LPM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[inline(always)]
    pub const fn set_DELTA_XOSC_LPM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_RFMEM_RET(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[inline(always)]
    pub const fn set_DELTA_RFMEM_RET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_CACHE_REF(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[inline(always)]
    pub const fn set_DELTA_CACHE_REF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for PWD_CURR_125C {
    #[inline(always)]
    fn default() -> PWD_CURR_125C {
        PWD_CURR_125C(0)
    }
}
impl core::fmt::Debug for PWD_CURR_125C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_CURR_125C")
            .field("BASELINE", &self.BASELINE())
            .field("DELTA_XOSC_LPM", &self.DELTA_XOSC_LPM())
            .field("DELTA_RFMEM_RET", &self.DELTA_RFMEM_RET())
            .field("DELTA_CACHE_REF", &self.DELTA_CACHE_REF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_CURR_125C {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_CURR_125C {{ BASELINE: {=u8:?}, DELTA_XOSC_LPM: {=u8:?}, DELTA_RFMEM_RET: {=u8:?}, DELTA_CACHE_REF: {=u8:?} }}",
            self.BASELINE(),
            self.DELTA_XOSC_LPM(),
            self.DELTA_RFMEM_RET(),
            self.DELTA_CACHE_REF()
        )
    }
}
#[doc = "Power Down Current Control 20C."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_CURR_20C(pub u32);
impl PWD_CURR_20C {
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[must_use]
    #[inline(always)]
    pub const fn BASELINE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[inline(always)]
    pub const fn set_BASELINE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_XOSC_LPM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[inline(always)]
    pub const fn set_DELTA_XOSC_LPM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_RFMEM_RET(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[inline(always)]
    pub const fn set_DELTA_RFMEM_RET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_CACHE_REF(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[inline(always)]
    pub const fn set_DELTA_CACHE_REF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for PWD_CURR_20C {
    #[inline(always)]
    fn default() -> PWD_CURR_20C {
        PWD_CURR_20C(0)
    }
}
impl core::fmt::Debug for PWD_CURR_20C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_CURR_20C")
            .field("BASELINE", &self.BASELINE())
            .field("DELTA_XOSC_LPM", &self.DELTA_XOSC_LPM())
            .field("DELTA_RFMEM_RET", &self.DELTA_RFMEM_RET())
            .field("DELTA_CACHE_REF", &self.DELTA_CACHE_REF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_CURR_20C {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_CURR_20C {{ BASELINE: {=u8:?}, DELTA_XOSC_LPM: {=u8:?}, DELTA_RFMEM_RET: {=u8:?}, DELTA_CACHE_REF: {=u8:?} }}",
            self.BASELINE(),
            self.DELTA_XOSC_LPM(),
            self.DELTA_RFMEM_RET(),
            self.DELTA_CACHE_REF()
        )
    }
}
#[doc = "Power Down Current Control 35C."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_CURR_35C(pub u32);
impl PWD_CURR_35C {
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[must_use]
    #[inline(always)]
    pub const fn BASELINE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[inline(always)]
    pub const fn set_BASELINE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_XOSC_LPM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[inline(always)]
    pub const fn set_DELTA_XOSC_LPM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_RFMEM_RET(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[inline(always)]
    pub const fn set_DELTA_RFMEM_RET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_CACHE_REF(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[inline(always)]
    pub const fn set_DELTA_CACHE_REF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for PWD_CURR_35C {
    #[inline(always)]
    fn default() -> PWD_CURR_35C {
        PWD_CURR_35C(0)
    }
}
impl core::fmt::Debug for PWD_CURR_35C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_CURR_35C")
            .field("BASELINE", &self.BASELINE())
            .field("DELTA_XOSC_LPM", &self.DELTA_XOSC_LPM())
            .field("DELTA_RFMEM_RET", &self.DELTA_RFMEM_RET())
            .field("DELTA_CACHE_REF", &self.DELTA_CACHE_REF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_CURR_35C {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_CURR_35C {{ BASELINE: {=u8:?}, DELTA_XOSC_LPM: {=u8:?}, DELTA_RFMEM_RET: {=u8:?}, DELTA_CACHE_REF: {=u8:?} }}",
            self.BASELINE(),
            self.DELTA_XOSC_LPM(),
            self.DELTA_RFMEM_RET(),
            self.DELTA_CACHE_REF()
        )
    }
}
#[doc = "Power Down Current Control 50C."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_CURR_50C(pub u32);
impl PWD_CURR_50C {
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[must_use]
    #[inline(always)]
    pub const fn BASELINE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[inline(always)]
    pub const fn set_BASELINE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_XOSC_LPM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[inline(always)]
    pub const fn set_DELTA_XOSC_LPM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_RFMEM_RET(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[inline(always)]
    pub const fn set_DELTA_RFMEM_RET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_CACHE_REF(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[inline(always)]
    pub const fn set_DELTA_CACHE_REF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for PWD_CURR_50C {
    #[inline(always)]
    fn default() -> PWD_CURR_50C {
        PWD_CURR_50C(0)
    }
}
impl core::fmt::Debug for PWD_CURR_50C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_CURR_50C")
            .field("BASELINE", &self.BASELINE())
            .field("DELTA_XOSC_LPM", &self.DELTA_XOSC_LPM())
            .field("DELTA_RFMEM_RET", &self.DELTA_RFMEM_RET())
            .field("DELTA_CACHE_REF", &self.DELTA_CACHE_REF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_CURR_50C {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_CURR_50C {{ BASELINE: {=u8:?}, DELTA_XOSC_LPM: {=u8:?}, DELTA_RFMEM_RET: {=u8:?}, DELTA_CACHE_REF: {=u8:?} }}",
            self.BASELINE(),
            self.DELTA_XOSC_LPM(),
            self.DELTA_RFMEM_RET(),
            self.DELTA_CACHE_REF()
        )
    }
}
#[doc = "Power Down Current Control 65C."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_CURR_65C(pub u32);
impl PWD_CURR_65C {
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[must_use]
    #[inline(always)]
    pub const fn BASELINE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[inline(always)]
    pub const fn set_BASELINE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_XOSC_LPM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[inline(always)]
    pub const fn set_DELTA_XOSC_LPM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_RFMEM_RET(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[inline(always)]
    pub const fn set_DELTA_RFMEM_RET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_CACHE_REF(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[inline(always)]
    pub const fn set_DELTA_CACHE_REF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for PWD_CURR_65C {
    #[inline(always)]
    fn default() -> PWD_CURR_65C {
        PWD_CURR_65C(0)
    }
}
impl core::fmt::Debug for PWD_CURR_65C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_CURR_65C")
            .field("BASELINE", &self.BASELINE())
            .field("DELTA_XOSC_LPM", &self.DELTA_XOSC_LPM())
            .field("DELTA_RFMEM_RET", &self.DELTA_RFMEM_RET())
            .field("DELTA_CACHE_REF", &self.DELTA_CACHE_REF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_CURR_65C {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_CURR_65C {{ BASELINE: {=u8:?}, DELTA_XOSC_LPM: {=u8:?}, DELTA_RFMEM_RET: {=u8:?}, DELTA_CACHE_REF: {=u8:?} }}",
            self.BASELINE(),
            self.DELTA_XOSC_LPM(),
            self.DELTA_RFMEM_RET(),
            self.DELTA_CACHE_REF()
        )
    }
}
#[doc = "Power Down Current Control 80C."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_CURR_80C(pub u32);
impl PWD_CURR_80C {
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[must_use]
    #[inline(always)]
    pub const fn BASELINE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[inline(always)]
    pub const fn set_BASELINE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_XOSC_LPM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[inline(always)]
    pub const fn set_DELTA_XOSC_LPM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_RFMEM_RET(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[inline(always)]
    pub const fn set_DELTA_RFMEM_RET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_CACHE_REF(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[inline(always)]
    pub const fn set_DELTA_CACHE_REF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for PWD_CURR_80C {
    #[inline(always)]
    fn default() -> PWD_CURR_80C {
        PWD_CURR_80C(0)
    }
}
impl core::fmt::Debug for PWD_CURR_80C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_CURR_80C")
            .field("BASELINE", &self.BASELINE())
            .field("DELTA_XOSC_LPM", &self.DELTA_XOSC_LPM())
            .field("DELTA_RFMEM_RET", &self.DELTA_RFMEM_RET())
            .field("DELTA_CACHE_REF", &self.DELTA_CACHE_REF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_CURR_80C {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_CURR_80C {{ BASELINE: {=u8:?}, DELTA_XOSC_LPM: {=u8:?}, DELTA_RFMEM_RET: {=u8:?}, DELTA_CACHE_REF: {=u8:?} }}",
            self.BASELINE(),
            self.DELTA_XOSC_LPM(),
            self.DELTA_RFMEM_RET(),
            self.DELTA_CACHE_REF()
        )
    }
}
#[doc = "Power Down Current Control 95C."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_CURR_95C(pub u32);
impl PWD_CURR_95C {
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[must_use]
    #[inline(always)]
    pub const fn BASELINE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA."]
    #[inline(always)]
    pub const fn set_BASELINE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_XOSC_LPM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode."]
    #[inline(always)]
    pub const fn set_DELTA_XOSC_LPM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_RFMEM_RET(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Additional maximum current, in 1uA units, with RF memory retention."]
    #[inline(always)]
    pub const fn set_DELTA_RFMEM_RET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_CACHE_REF(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Additional maximum current, in units of 1uA, with cache retention."]
    #[inline(always)]
    pub const fn set_DELTA_CACHE_REF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for PWD_CURR_95C {
    #[inline(always)]
    fn default() -> PWD_CURR_95C {
        PWD_CURR_95C(0)
    }
}
impl core::fmt::Debug for PWD_CURR_95C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_CURR_95C")
            .field("BASELINE", &self.BASELINE())
            .field("DELTA_XOSC_LPM", &self.DELTA_XOSC_LPM())
            .field("DELTA_RFMEM_RET", &self.DELTA_RFMEM_RET())
            .field("DELTA_CACHE_REF", &self.DELTA_CACHE_REF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_CURR_95C {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_CURR_95C {{ BASELINE: {=u8:?}, DELTA_XOSC_LPM: {=u8:?}, DELTA_RFMEM_RET: {=u8:?}, DELTA_CACHE_REF: {=u8:?} }}",
            self.BASELINE(),
            self.DELTA_XOSC_LPM(),
            self.DELTA_RFMEM_RET(),
            self.DELTA_CACHE_REF()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCOSC_HF_TEMPCOMP(pub u32);
impl RCOSC_HF_TEMPCOMP {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CTRIMFRACT_SLOPE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CTRIMFRACT_SLOPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CTRIMFRACT_QUAD(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CTRIMFRACT_QUAD(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn CTRIM(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_CTRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FINE_RESISTOR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FINE_RESISTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for RCOSC_HF_TEMPCOMP {
    #[inline(always)]
    fn default() -> RCOSC_HF_TEMPCOMP {
        RCOSC_HF_TEMPCOMP(0)
    }
}
impl core::fmt::Debug for RCOSC_HF_TEMPCOMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCOSC_HF_TEMPCOMP")
            .field("CTRIMFRACT_SLOPE", &self.CTRIMFRACT_SLOPE())
            .field("CTRIMFRACT_QUAD", &self.CTRIMFRACT_QUAD())
            .field("CTRIM", &self.CTRIM())
            .field("FINE_RESISTOR", &self.FINE_RESISTOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCOSC_HF_TEMPCOMP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCOSC_HF_TEMPCOMP {{ CTRIMFRACT_SLOPE: {=u8:?}, CTRIMFRACT_QUAD: {=u8:?}, CTRIM: {=u8:?}, FINE_RESISTOR: {=u8:?} }}",
            self.CTRIMFRACT_SLOPE(),
            self.CTRIMFRACT_QUAD(),
            self.CTRIM(),
            self.FINE_RESISTOR()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SHDW_ANA_TRIM(pub u32);
impl SHDW_ANA_TRIM {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIMTEMP(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIMTEMP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "10:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIMBOD_EXTMODE(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "10:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIMBOD_EXTMODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "15:11\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIMBOD_INTMODE(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "15:11\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIMBOD_INTMODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
    #[doc = "20:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_TRIM(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "20:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VDDR_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "22:21\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IPTAT_TRIM(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "22:21\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IPTAT_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
    #[doc = "23:23\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_OK_HYS(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VDDR_OK_HYS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_ENABLE_PG1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VDDR_ENABLE_PG1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "26:25\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BOD_BANDGAP_TRIM_CNF(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x03;
        val as u8
    }
    #[doc = "26:25\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BOD_BANDGAP_TRIM_CNF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
    }
    #[doc = "31:27\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x1f;
        val as u8
    }
    #[doc = "31:27\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
    }
}
impl Default for SHDW_ANA_TRIM {
    #[inline(always)]
    fn default() -> SHDW_ANA_TRIM {
        SHDW_ANA_TRIM(0)
    }
}
impl core::fmt::Debug for SHDW_ANA_TRIM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHDW_ANA_TRIM")
            .field("TRIMTEMP", &self.TRIMTEMP())
            .field("TRIMBOD_EXTMODE", &self.TRIMBOD_EXTMODE())
            .field("TRIMBOD_INTMODE", &self.TRIMBOD_INTMODE())
            .field("VDDR_TRIM", &self.VDDR_TRIM())
            .field("IPTAT_TRIM", &self.IPTAT_TRIM())
            .field("VDDR_OK_HYS", &self.VDDR_OK_HYS())
            .field("VDDR_ENABLE_PG1", &self.VDDR_ENABLE_PG1())
            .field("BOD_BANDGAP_TRIM_CNF", &self.BOD_BANDGAP_TRIM_CNF())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SHDW_ANA_TRIM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SHDW_ANA_TRIM {{ TRIMTEMP: {=u8:?}, TRIMBOD_EXTMODE: {=u8:?}, TRIMBOD_INTMODE: {=u8:?}, VDDR_TRIM: {=u8:?}, IPTAT_TRIM: {=u8:?}, VDDR_OK_HYS: {=bool:?}, VDDR_ENABLE_PG1: {=bool:?}, BOD_BANDGAP_TRIM_CNF: {=u8:?}, RESERVED: {=u8:?} }}",
            self.TRIMTEMP(),
            self.TRIMBOD_EXTMODE(),
            self.TRIMBOD_INTMODE(),
            self.VDDR_TRIM(),
            self.IPTAT_TRIM(),
            self.VDDR_OK_HYS(),
            self.VDDR_ENABLE_PG1(),
            self.BOD_BANDGAP_TRIM_CNF(),
            self.RESERVED()
        )
    }
}
#[doc = "Shadow of DIE_ID_0 register in eFuse."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SHDW_DIE_ID_0(pub u32);
impl SHDW_DIE_ID_0 {
    #[doc = "31:0\\] Shadow of DIE_ID_0 register in eFuse row number 3."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Shadow of DIE_ID_0 register in eFuse row number 3."]
    #[inline(always)]
    pub const fn set_ID_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SHDW_DIE_ID_0 {
    #[inline(always)]
    fn default() -> SHDW_DIE_ID_0 {
        SHDW_DIE_ID_0(0)
    }
}
impl core::fmt::Debug for SHDW_DIE_ID_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHDW_DIE_ID_0")
            .field("ID_31_0", &self.ID_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SHDW_DIE_ID_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SHDW_DIE_ID_0 {{ ID_31_0: {=u32:?} }}", self.ID_31_0())
    }
}
#[doc = "Shadow of DIE_ID_1 register in eFuse."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SHDW_DIE_ID_1(pub u32);
impl SHDW_DIE_ID_1 {
    #[doc = "31:0\\] Shadow of DIE_ID_1 register in eFuse row number 4."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_63_32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Shadow of DIE_ID_1 register in eFuse row number 4."]
    #[inline(always)]
    pub const fn set_ID_63_32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SHDW_DIE_ID_1 {
    #[inline(always)]
    fn default() -> SHDW_DIE_ID_1 {
        SHDW_DIE_ID_1(0)
    }
}
impl core::fmt::Debug for SHDW_DIE_ID_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHDW_DIE_ID_1")
            .field("ID_63_32", &self.ID_63_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SHDW_DIE_ID_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SHDW_DIE_ID_1 {{ ID_63_32: {=u32:?} }}", self.ID_63_32())
    }
}
#[doc = "Shadow of DIE_ID_2 register in eFuse."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SHDW_DIE_ID_2(pub u32);
impl SHDW_DIE_ID_2 {
    #[doc = "31:0\\] Shadow of DIE_ID_2 register in eFuse row number 5."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_95_64(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Shadow of DIE_ID_2 register in eFuse row number 5."]
    #[inline(always)]
    pub const fn set_ID_95_64(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SHDW_DIE_ID_2 {
    #[inline(always)]
    fn default() -> SHDW_DIE_ID_2 {
        SHDW_DIE_ID_2(0)
    }
}
impl core::fmt::Debug for SHDW_DIE_ID_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHDW_DIE_ID_2")
            .field("ID_95_64", &self.ID_95_64())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SHDW_DIE_ID_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SHDW_DIE_ID_2 {{ ID_95_64: {=u32:?} }}", self.ID_95_64())
    }
}
#[doc = "Shadow of DIE_ID_3 register in eFuse."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SHDW_DIE_ID_3(pub u32);
impl SHDW_DIE_ID_3 {
    #[doc = "31:0\\] Shadow of DIE_ID_3 register in eFuse row number 6."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_127_96(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Shadow of DIE_ID_3 register in eFuse row number 6."]
    #[inline(always)]
    pub const fn set_ID_127_96(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SHDW_DIE_ID_3 {
    #[inline(always)]
    fn default() -> SHDW_DIE_ID_3 {
        SHDW_DIE_ID_3(0)
    }
}
impl core::fmt::Debug for SHDW_DIE_ID_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHDW_DIE_ID_3")
            .field("ID_127_96", &self.ID_127_96())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SHDW_DIE_ID_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SHDW_DIE_ID_3 {{ ID_127_96: {=u32:?} }}",
            self.ID_127_96()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SHDW_OSC_BIAS_LDO_TRIM(pub u32);
impl SHDW_OSC_BIAS_LDO_TRIM {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSCHF_CTRIM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RCOSCHF_CTRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VTRIM_COARSE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "11:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VTRIM_COARSE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VTRIM_DIG(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VTRIM_DIG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "17:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ITRIM_DIG_LDO(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "17:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ITRIM_DIG_LDO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "22:18\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIMIREF(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "22:18\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIMIREF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "26:23\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIMMAG(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x0f;
        val as u8
    }
    #[doc = "26:23\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIMMAG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val as u32) & 0x0f) << 23usize);
    }
    #[doc = "28:27\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SET_RCOSC_HF_COARSE_RESISTOR(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "28:27\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SET_RCOSC_HF_COARSE_RESISTOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[doc = "31:29\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "31:29\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for SHDW_OSC_BIAS_LDO_TRIM {
    #[inline(always)]
    fn default() -> SHDW_OSC_BIAS_LDO_TRIM {
        SHDW_OSC_BIAS_LDO_TRIM(0)
    }
}
impl core::fmt::Debug for SHDW_OSC_BIAS_LDO_TRIM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHDW_OSC_BIAS_LDO_TRIM")
            .field("RCOSCHF_CTRIM", &self.RCOSCHF_CTRIM())
            .field("VTRIM_COARSE", &self.VTRIM_COARSE())
            .field("VTRIM_DIG", &self.VTRIM_DIG())
            .field("ITRIM_DIG_LDO", &self.ITRIM_DIG_LDO())
            .field("TRIMIREF", &self.TRIMIREF())
            .field("TRIMMAG", &self.TRIMMAG())
            .field(
                "SET_RCOSC_HF_COARSE_RESISTOR",
                &self.SET_RCOSC_HF_COARSE_RESISTOR(),
            )
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SHDW_OSC_BIAS_LDO_TRIM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SHDW_OSC_BIAS_LDO_TRIM {{ RCOSCHF_CTRIM: {=u8:?}, VTRIM_COARSE: {=u8:?}, VTRIM_DIG: {=u8:?}, ITRIM_DIG_LDO: {=u8:?}, TRIMIREF: {=u8:?}, TRIMMAG: {=u8:?}, SET_RCOSC_HF_COARSE_RESISTOR: {=u8:?}, RESERVED: {=u8:?} }}",
            self.RCOSCHF_CTRIM(),
            self.VTRIM_COARSE(),
            self.VTRIM_DIG(),
            self.ITRIM_DIG_LDO(),
            self.TRIMIREF(),
            self.TRIMMAG(),
            self.SET_RCOSC_HF_COARSE_RESISTOR(),
            self.RESERVED()
        )
    }
}
#[doc = "AUX_ADC Gain in Absolute Reference Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SOC_ADC_ABS_GAIN(pub u32);
impl SOC_ADC_ABS_GAIN {
    #[doc = "15:0\\] SOC_ADC gain in absolute reference mode at temperature 1 (30C). Calculated in production test.."]
    #[must_use]
    #[inline(always)]
    pub const fn SOC_ADC_ABS_GAIN_TEMP1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] SOC_ADC gain in absolute reference mode at temperature 1 (30C). Calculated in production test.."]
    #[inline(always)]
    pub const fn set_SOC_ADC_ABS_GAIN_TEMP1(&mut self, val: u16) {
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
impl Default for SOC_ADC_ABS_GAIN {
    #[inline(always)]
    fn default() -> SOC_ADC_ABS_GAIN {
        SOC_ADC_ABS_GAIN(0)
    }
}
impl core::fmt::Debug for SOC_ADC_ABS_GAIN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOC_ADC_ABS_GAIN")
            .field("SOC_ADC_ABS_GAIN_TEMP1", &self.SOC_ADC_ABS_GAIN_TEMP1())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SOC_ADC_ABS_GAIN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SOC_ADC_ABS_GAIN {{ SOC_ADC_ABS_GAIN_TEMP1: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.SOC_ADC_ABS_GAIN_TEMP1(),
            self.RESERVED16()
        )
    }
}
#[doc = "AUX_ADC Temperature Offsets in Absolute Reference Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SOC_ADC_OFFSET_INT(pub u32);
impl SOC_ADC_OFFSET_INT {
    #[doc = "7:0\\] SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[must_use]
    #[inline(always)]
    pub const fn SOC_ADC_ABS_OFFSET_TEMP1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    pub const fn set_SOC_ADC_ABS_OFFSET_TEMP1(&mut self, val: u8) {
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
    #[doc = "23:16\\] SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[must_use]
    #[inline(always)]
    pub const fn SOC_ADC_REL_OFFSET_TEMP1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    pub const fn set_SOC_ADC_REL_OFFSET_TEMP1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
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
impl Default for SOC_ADC_OFFSET_INT {
    #[inline(always)]
    fn default() -> SOC_ADC_OFFSET_INT {
        SOC_ADC_OFFSET_INT(0)
    }
}
impl core::fmt::Debug for SOC_ADC_OFFSET_INT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOC_ADC_OFFSET_INT")
            .field("SOC_ADC_ABS_OFFSET_TEMP1", &self.SOC_ADC_ABS_OFFSET_TEMP1())
            .field("RESERVED8", &self.RESERVED8())
            .field("SOC_ADC_REL_OFFSET_TEMP1", &self.SOC_ADC_REL_OFFSET_TEMP1())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SOC_ADC_OFFSET_INT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SOC_ADC_OFFSET_INT {{ SOC_ADC_ABS_OFFSET_TEMP1: {=u8:?}, RESERVED8: {=u8:?}, SOC_ADC_REL_OFFSET_TEMP1: {=u8:?}, RESERVED24: {=u8:?} }}",
            self.SOC_ADC_ABS_OFFSET_TEMP1(),
            self.RESERVED8(),
            self.SOC_ADC_REL_OFFSET_TEMP1(),
            self.RESERVED24()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SOC_ADC_REF_TRIM_AND_OFFSET_EXT(pub u32);
impl SOC_ADC_REF_TRIM_AND_OFFSET_EXT {
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SOC_ADC_REF_VOLTAGE_TRIM_TEMP1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "5:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SOC_ADC_REF_VOLTAGE_TRIM_TEMP1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
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
impl Default for SOC_ADC_REF_TRIM_AND_OFFSET_EXT {
    #[inline(always)]
    fn default() -> SOC_ADC_REF_TRIM_AND_OFFSET_EXT {
        SOC_ADC_REF_TRIM_AND_OFFSET_EXT(0)
    }
}
impl core::fmt::Debug for SOC_ADC_REF_TRIM_AND_OFFSET_EXT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOC_ADC_REF_TRIM_AND_OFFSET_EXT")
            .field(
                "SOC_ADC_REF_VOLTAGE_TRIM_TEMP1",
                &self.SOC_ADC_REF_VOLTAGE_TRIM_TEMP1(),
            )
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SOC_ADC_REF_TRIM_AND_OFFSET_EXT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SOC_ADC_REF_TRIM_AND_OFFSET_EXT {{ SOC_ADC_REF_VOLTAGE_TRIM_TEMP1: {=u8:?}, RESERVED6: {=u32:?} }}",
            self.SOC_ADC_REF_VOLTAGE_TRIM_TEMP1(),
            self.RESERVED6()
        )
    }
}
#[doc = "AUX_ADC Gain in Relative Reference Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SOC_ADC_REL_GAIN(pub u32);
impl SOC_ADC_REL_GAIN {
    #[doc = "15:0\\] SOC_ADC gain in relative reference mode at temperature 1 (30C). Calculated in production test.."]
    #[must_use]
    #[inline(always)]
    pub const fn SOC_ADC_REL_GAIN_TEMP1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] SOC_ADC gain in relative reference mode at temperature 1 (30C). Calculated in production test.."]
    #[inline(always)]
    pub const fn set_SOC_ADC_REL_GAIN_TEMP1(&mut self, val: u16) {
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
impl Default for SOC_ADC_REL_GAIN {
    #[inline(always)]
    fn default() -> SOC_ADC_REL_GAIN {
        SOC_ADC_REL_GAIN(0)
    }
}
impl core::fmt::Debug for SOC_ADC_REL_GAIN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOC_ADC_REL_GAIN")
            .field("SOC_ADC_REL_GAIN_TEMP1", &self.SOC_ADC_REL_GAIN_TEMP1())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SOC_ADC_REL_GAIN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SOC_ADC_REL_GAIN {{ SOC_ADC_REL_GAIN_TEMP1: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.SOC_ADC_REL_GAIN_TEMP1(),
            self.RESERVED16()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TRIM_CAL_REVISION(pub u32);
impl TRIM_CAL_REVISION {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn MP1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_MP1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FT1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FT1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for TRIM_CAL_REVISION {
    #[inline(always)]
    fn default() -> TRIM_CAL_REVISION {
        TRIM_CAL_REVISION(0)
    }
}
impl core::fmt::Debug for TRIM_CAL_REVISION {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIM_CAL_REVISION")
            .field("MP1", &self.MP1())
            .field("FT1", &self.FT1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TRIM_CAL_REVISION {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TRIM_CAL_REVISION {{ MP1: {=u16:?}, FT1: {=u16:?} }}",
            self.MP1(),
            self.FT1()
        )
    }
}
#[doc = "User Identification. Reading this register and the ICEPICK_DEVICE_ID register is the only support way of identifying a device. The value of this register will be written to AON_WUC:JTAGUSERCODE by boot FW while in safezone."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USER_ID(pub u32);
impl USER_ID {
    #[doc = "11:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "11:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "15:12\\] Protocols supported. 0x1: BLE 0x2: RF4CE 0x4: Zigbee/6lowpan 0x8: Proprietary More than one protocol can be supported on same device - values above are then combined."]
    #[must_use]
    #[inline(always)]
    pub const fn PROTOCOL(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Protocols supported. 0x1: BLE 0x2: RF4CE 0x4: Zigbee/6lowpan 0x8: Proprietary More than one protocol can be supported on same device - values above are then combined."]
    #[inline(always)]
    pub const fn set_PROTOCOL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "18:16\\] Package type. 0x0: 4x4mm QFN (RHB) package 0x1: 5x5mm QFN (RSM) package 0x2: 7x7mm QFN (RGZ) package 0x3: Wafer sale package (naked die) 0x4: 2.7x2.7mm WCSP (YFV) 0x5: 7x7mm QFN package with Wettable Flanks Other values are reserved for future use. Packages available for a specific device are shown in the device datasheet."]
    #[must_use]
    #[inline(always)]
    pub const fn PKG(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "18:16\\] Package type. 0x0: 4x4mm QFN (RHB) package 0x1: 5x5mm QFN (RSM) package 0x2: 7x7mm QFN (RGZ) package 0x3: Wafer sale package (naked die) 0x4: 2.7x2.7mm WCSP (YFV) 0x5: 7x7mm QFN package with Wettable Flanks Other values are reserved for future use. Packages available for a specific device are shown in the device datasheet."]
    #[inline(always)]
    pub const fn set_PKG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "22:19\\] Sequence. Used to differentiate between marketing/orderable product where other fields of USER_ID is the same (temp range, flash size, voltage range etc)."]
    #[must_use]
    #[inline(always)]
    pub const fn SEQUENCE(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x0f;
        val as u8
    }
    #[doc = "22:19\\] Sequence. Used to differentiate between marketing/orderable product where other fields of USER_ID is the same (temp range, flash size, voltage range etc)."]
    #[inline(always)]
    pub const fn set_SEQUENCE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 19usize)) | (((val as u32) & 0x0f) << 19usize);
    }
    #[doc = "25:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED23(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x07;
        val as u8
    }
    #[doc = "25:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED23(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
    }
    #[doc = "27:26\\] Version number. 0x0: Bits \\[25:12\\] of this register has the stated meaning. Any other setting indicate a different encoding of these bits."]
    #[must_use]
    #[inline(always)]
    pub const fn VER(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "27:26\\] Version number. 0x0: Bits \\[25:12\\] of this register has the stated meaning. Any other setting indicate a different encoding of these bits."]
    #[inline(always)]
    pub const fn set_VER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "31:28\\] Field used to distinguish revisions of the device."]
    #[must_use]
    #[inline(always)]
    pub const fn PG_REV(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Field used to distinguish revisions of the device."]
    #[inline(always)]
    pub const fn set_PG_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for USER_ID {
    #[inline(always)]
    fn default() -> USER_ID {
        USER_ID(0)
    }
}
impl core::fmt::Debug for USER_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER_ID")
            .field("RESERVED0", &self.RESERVED0())
            .field("PROTOCOL", &self.PROTOCOL())
            .field("PKG", &self.PKG())
            .field("SEQUENCE", &self.SEQUENCE())
            .field("RESERVED23", &self.RESERVED23())
            .field("VER", &self.VER())
            .field("PG_REV", &self.PG_REV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USER_ID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USER_ID {{ RESERVED0: {=u16:?}, PROTOCOL: {=u8:?}, PKG: {=u8:?}, SEQUENCE: {=u8:?}, RESERVED23: {=u8:?}, VER: {=u8:?}, PG_REV: {=u8:?} }}",
            self.RESERVED0(),
            self.PROTOCOL(),
            self.PKG(),
            self.SEQUENCE(),
            self.RESERVED23(),
            self.VER(),
            self.PG_REV()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VOLT_TRIM(pub u32);
impl VOLT_TRIM {
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIMBOD_H(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "4:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_TRIMBOD_H(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "7:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "7:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "12:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_TRIM_SLEEP_H(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "12:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VDDR_TRIM_SLEEP_H(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "15:13\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "15:13\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "20:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_TRIM_H(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "20:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VDDR_TRIM_H(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "23:21\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "23:21\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "28:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_TRIM_HH(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "28:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_VDDR_TRIM_HH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "31:29\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "31:29\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for VOLT_TRIM {
    #[inline(always)]
    fn default() -> VOLT_TRIM {
        VOLT_TRIM(0)
    }
}
impl core::fmt::Debug for VOLT_TRIM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VOLT_TRIM")
            .field("TRIMBOD_H", &self.TRIMBOD_H())
            .field("RESERVED0", &self.RESERVED0())
            .field("VDDR_TRIM_SLEEP_H", &self.VDDR_TRIM_SLEEP_H())
            .field("RESERVED1", &self.RESERVED1())
            .field("VDDR_TRIM_H", &self.VDDR_TRIM_H())
            .field("RESERVED2", &self.RESERVED2())
            .field("VDDR_TRIM_HH", &self.VDDR_TRIM_HH())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VOLT_TRIM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VOLT_TRIM {{ TRIMBOD_H: {=u8:?}, RESERVED0: {=u8:?}, VDDR_TRIM_SLEEP_H: {=u8:?}, RESERVED1: {=u8:?}, VDDR_TRIM_H: {=u8:?}, RESERVED2: {=u8:?}, VDDR_TRIM_HH: {=u8:?}, RESERVED3: {=u8:?} }}",
            self.TRIMBOD_H(),
            self.RESERVED0(),
            self.VDDR_TRIM_SLEEP_H(),
            self.RESERVED1(),
            self.VDDR_TRIM_H(),
            self.RESERVED2(),
            self.VDDR_TRIM_HH(),
            self.RESERVED3()
        )
    }
}

#[doc = "ADC Doubler Nanoamp Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADCDOUBLERNANOAMPCTL(pub u32);
impl ADCDOUBLERNANOAMPCTL {
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_IREF_CTRL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ADC_IREF_CTRL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
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
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_SH_VBUF_EN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ADC_SH_VBUF_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_SH_MODE_EN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_ADC_SH_MODE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "22:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "22:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 6usize)) | (((val as u32) & 0x0001_ffff) << 6usize);
    }
    #[doc = "23:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_SPARE23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn NANOAMP_BIAS_ENABLE(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_NANOAMP_BIAS_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED25(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for ADCDOUBLERNANOAMPCTL {
    #[inline(always)]
    fn default() -> ADCDOUBLERNANOAMPCTL {
        ADCDOUBLERNANOAMPCTL(0)
    }
}
impl core::fmt::Debug for ADCDOUBLERNANOAMPCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCDOUBLERNANOAMPCTL")
            .field("ADC_IREF_CTRL", &self.ADC_IREF_CTRL())
            .field("RESERVED2", &self.RESERVED2())
            .field("ADC_SH_VBUF_EN", &self.ADC_SH_VBUF_EN())
            .field("ADC_SH_MODE_EN", &self.ADC_SH_MODE_EN())
            .field("RESERVED6", &self.RESERVED6())
            .field("SPARE23", &self.SPARE23())
            .field("NANOAMP_BIAS_ENABLE", &self.NANOAMP_BIAS_ENABLE())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADCDOUBLERNANOAMPCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADCDOUBLERNANOAMPCTL {{ ADC_IREF_CTRL: {=u8:?}, RESERVED2: {=u8:?}, ADC_SH_VBUF_EN: {=bool:?}, ADC_SH_MODE_EN: {=bool:?}, RESERVED6: {=u32:?}, SPARE23: {=bool:?}, NANOAMP_BIAS_ENABLE: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.ADC_IREF_CTRL(),
            self.RESERVED2(),
            self.ADC_SH_VBUF_EN(),
            self.ADC_SH_MODE_EN(),
            self.RESERVED6(),
            self.SPARE23(),
            self.NANOAMP_BIAS_ENABLE(),
            self.RESERVED25()
        )
    }
}
#[doc = "Amplitude Compensation Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AMPCOMPCTL(pub u32);
impl AMPCOMPCTL {
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
    #[doc = "25:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED24(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "25:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED24(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "26:26\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn AMPCOMP_SW_EN(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_AMPCOMP_SW_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn AMPCOMP_SW_CTRL(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_AMPCOMP_SW_CTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "29:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn AMPCOMP_FSM_UPDATE_RATE(&self) -> super::vals::AMPCOMP_FSM_UPDATE_RATE {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::AMPCOMP_FSM_UPDATE_RATE::from_bits(val as u8)
    }
    #[doc = "29:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_AMPCOMP_FSM_UPDATE_RATE(&mut self, val: super::vals::AMPCOMP_FSM_UPDATE_RATE) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
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
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_SPARE31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for AMPCOMPCTL {
    #[inline(always)]
    fn default() -> AMPCOMPCTL {
        AMPCOMPCTL(0)
    }
}
impl core::fmt::Debug for AMPCOMPCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMPCOMPCTL")
            .field("IBIASCAP_HPTOLP_OL_CNT", &self.IBIASCAP_HPTOLP_OL_CNT())
            .field("CAP_STEP", &self.CAP_STEP())
            .field("LPM_IBIAS_WAIT_CNT_FINAL", &self.LPM_IBIAS_WAIT_CNT_FINAL())
            .field("IBIAS_INIT", &self.IBIAS_INIT())
            .field("IBIAS_OFFSET", &self.IBIAS_OFFSET())
            .field("RESERVED24", &self.RESERVED24())
            .field("AMPCOMP_SW_EN", &self.AMPCOMP_SW_EN())
            .field("AMPCOMP_SW_CTRL", &self.AMPCOMP_SW_CTRL())
            .field("AMPCOMP_FSM_UPDATE_RATE", &self.AMPCOMP_FSM_UPDATE_RATE())
            .field("AMPCOMP_REQ_MODE", &self.AMPCOMP_REQ_MODE())
            .field("SPARE31", &self.SPARE31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AMPCOMPCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AMPCOMPCTL {{ IBIASCAP_HPTOLP_OL_CNT: {=u8:?}, CAP_STEP: {=u8:?}, LPM_IBIAS_WAIT_CNT_FINAL: {=u8:?}, IBIAS_INIT: {=u8:?}, IBIAS_OFFSET: {=u8:?}, RESERVED24: {=u8:?}, AMPCOMP_SW_EN: {=bool:?}, AMPCOMP_SW_CTRL: {=bool:?}, AMPCOMP_FSM_UPDATE_RATE: {:?}, AMPCOMP_REQ_MODE: {=bool:?}, SPARE31: {=bool:?} }}",
            self.IBIASCAP_HPTOLP_OL_CNT(),
            self.CAP_STEP(),
            self.LPM_IBIAS_WAIT_CNT_FINAL(),
            self.IBIAS_INIT(),
            self.IBIAS_OFFSET(),
            self.RESERVED24(),
            self.AMPCOMP_SW_EN(),
            self.AMPCOMP_SW_CTRL(),
            self.AMPCOMP_FSM_UPDATE_RATE(),
            self.AMPCOMP_REQ_MODE(),
            self.SPARE31()
        )
    }
}
#[doc = "Amplitude Compensation Threshold 1 This register contains threshold values for amplitude compensation algorithm."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AMPCOMPTH1(pub u32);
impl AMPCOMPTH1 {
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
    #[doc = "17:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE16(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "17:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_SPARE16(&mut self, val: u8) {
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
    #[doc = "31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE24(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_SPARE24(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for AMPCOMPTH1 {
    #[inline(always)]
    fn default() -> AMPCOMPTH1 {
        AMPCOMPTH1(0)
    }
}
impl core::fmt::Debug for AMPCOMPTH1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMPCOMPTH1")
            .field("HPMRAMP1_TH", &self.HPMRAMP1_TH())
            .field("IBIASCAP_LPTOHP_OL_CNT", &self.IBIASCAP_LPTOHP_OL_CNT())
            .field("HPMRAMP3_HTH", &self.HPMRAMP3_HTH())
            .field("SPARE16", &self.SPARE16())
            .field("HPMRAMP3_LTH", &self.HPMRAMP3_LTH())
            .field("SPARE24", &self.SPARE24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AMPCOMPTH1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AMPCOMPTH1 {{ HPMRAMP1_TH: {=u8:?}, IBIASCAP_LPTOHP_OL_CNT: {=u8:?}, HPMRAMP3_HTH: {=u8:?}, SPARE16: {=u8:?}, HPMRAMP3_LTH: {=u8:?}, SPARE24: {=u8:?} }}",
            self.HPMRAMP1_TH(),
            self.IBIASCAP_LPTOHP_OL_CNT(),
            self.HPMRAMP3_HTH(),
            self.SPARE16(),
            self.HPMRAMP3_LTH(),
            self.SPARE24()
        )
    }
}
#[doc = "Amplitude Compensation Threshold 2 This register contains threshold values for amplitude compensation algorithm."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AMPCOMPTH2(pub u32);
impl AMPCOMPTH2 {
    #[doc = "1:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_SPARE0(&mut self, val: u8) {
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
    #[doc = "9:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE8(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "9:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_SPARE8(&mut self, val: u8) {
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
    #[doc = "17:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE16(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "17:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_SPARE16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "23:18\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LPMUPDATE_HTH(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "23:18\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LPMUPDATE_HTH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "25:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE24(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "25:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_SPARE24(&mut self, val: u8) {
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
impl Default for AMPCOMPTH2 {
    #[inline(always)]
    fn default() -> AMPCOMPTH2 {
        AMPCOMPTH2(0)
    }
}
impl core::fmt::Debug for AMPCOMPTH2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMPCOMPTH2")
            .field("SPARE0", &self.SPARE0())
            .field("ADC_COMP_AMPTH_HPM", &self.ADC_COMP_AMPTH_HPM())
            .field("SPARE8", &self.SPARE8())
            .field("ADC_COMP_AMPTH_LPM", &self.ADC_COMP_AMPTH_LPM())
            .field("SPARE16", &self.SPARE16())
            .field("LPMUPDATE_HTH", &self.LPMUPDATE_HTH())
            .field("SPARE24", &self.SPARE24())
            .field("LPMUPDATE_LTH", &self.LPMUPDATE_LTH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AMPCOMPTH2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AMPCOMPTH2 {{ SPARE0: {=u8:?}, ADC_COMP_AMPTH_HPM: {=u8:?}, SPARE8: {=u8:?}, ADC_COMP_AMPTH_LPM: {=u8:?}, SPARE16: {=u8:?}, LPMUPDATE_HTH: {=u8:?}, SPARE24: {=u8:?}, LPMUPDATE_LTH: {=u8:?} }}",
            self.SPARE0(),
            self.ADC_COMP_AMPTH_HPM(),
            self.SPARE8(),
            self.ADC_COMP_AMPTH_LPM(),
            self.SPARE16(),
            self.LPMUPDATE_HTH(),
            self.SPARE24(),
            self.LPMUPDATE_LTH()
        )
    }
}
#[doc = "Analog Bypass Values 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANABYPASSVAL1(pub u32);
impl ANABYPASSVAL1 {
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_HF_COLUMN_Q12(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_XOSC_HF_COLUMN_Q12(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_HF_ROW_Q12(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_XOSC_HF_ROW_Q12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "31:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED20(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "31:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED20(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for ANABYPASSVAL1 {
    #[inline(always)]
    fn default() -> ANABYPASSVAL1 {
        ANABYPASSVAL1(0)
    }
}
impl core::fmt::Debug for ANABYPASSVAL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANABYPASSVAL1")
            .field("XOSC_HF_COLUMN_Q12", &self.XOSC_HF_COLUMN_Q12())
            .field("XOSC_HF_ROW_Q12", &self.XOSC_HF_ROW_Q12())
            .field("RESERVED20", &self.RESERVED20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANABYPASSVAL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANABYPASSVAL1 {{ XOSC_HF_COLUMN_Q12: {=u16:?}, XOSC_HF_ROW_Q12: {=u8:?}, RESERVED20: {=u16:?} }}",
            self.XOSC_HF_COLUMN_Q12(),
            self.XOSC_HF_ROW_Q12(),
            self.RESERVED20()
        )
    }
}
#[doc = "Internal. Only to be used through TI provided API."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANABYPASSVAL2(pub u32);
impl ANABYPASSVAL2 {
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
impl Default for ANABYPASSVAL2 {
    #[inline(always)]
    fn default() -> ANABYPASSVAL2 {
        ANABYPASSVAL2(0)
    }
}
impl core::fmt::Debug for ANABYPASSVAL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANABYPASSVAL2")
            .field("XOSC_HF_IBIASTHERM", &self.XOSC_HF_IBIASTHERM())
            .field("RESERVED14", &self.RESERVED14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANABYPASSVAL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANABYPASSVAL2 {{ XOSC_HF_IBIASTHERM: {=u16:?}, RESERVED14: {=u32:?} }}",
            self.XOSC_HF_IBIASTHERM(),
            self.RESERVED14()
        )
    }
}
#[doc = "Analog Test Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ATESTCTL(pub u32);
impl ATESTCTL {
    #[doc = "28:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "28:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
    #[doc = "29:29\\] Enable 32 kHz clock to AUX_COMPB."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLK_LF_AUX_EN(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Enable 32 kHz clock to AUX_COMPB."]
    #[inline(always)]
    pub const fn set_SCLK_LF_AUX_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "31:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE30(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "31:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_SPARE30(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ATESTCTL {
    #[inline(always)]
    fn default() -> ATESTCTL {
        ATESTCTL(0)
    }
}
impl core::fmt::Debug for ATESTCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATESTCTL")
            .field("RESERVED0", &self.RESERVED0())
            .field("SCLK_LF_AUX_EN", &self.SCLK_LF_AUX_EN())
            .field("SPARE30", &self.SPARE30())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ATESTCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ATESTCTL {{ RESERVED0: {=u32:?}, SCLK_LF_AUX_EN: {=bool:?}, SPARE30: {=u8:?} }}",
            self.RESERVED0(),
            self.SCLK_LF_AUX_EN(),
            self.SPARE30()
        )
    }
}
#[doc = "Control 0 Controls clock source selects."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTL0(pub u32);
impl CTL0 {
    #[doc = "0:0\\] Source select for sclk_hf. XOSC option is supported for test and debug only and should be used when the XOSC_HF is running."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLK_HF_SRC_SEL(&self) -> super::vals::SCLK_HF_SRC_SEL {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SCLK_HF_SRC_SEL::from_bits(val as u8)
    }
    #[doc = "0:0\\] Source select for sclk_hf. XOSC option is supported for test and debug only and should be used when the XOSC_HF is running."]
    #[inline(always)]
    pub const fn set_SCLK_HF_SRC_SEL(&mut self, val: super::vals::SCLK_HF_SRC_SEL) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLK_MF_SRC_SEL(&self) -> super::vals::SCLK_MF_SRC_SEL {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SCLK_MF_SRC_SEL::from_bits(val as u8)
    }
    #[doc = "1:1\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_SCLK_MF_SRC_SEL(&mut self, val: super::vals::SCLK_MF_SRC_SEL) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "3:2\\] Source select for sclk_lf."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLK_LF_SRC_SEL(&self) -> super::vals::SCLK_LF_SRC_SEL {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SCLK_LF_SRC_SEL::from_bits(val as u8)
    }
    #[doc = "3:2\\] Source select for sclk_lf."]
    #[inline(always)]
    pub const fn set_SCLK_LF_SRC_SEL(&mut self, val: super::vals::SCLK_LF_SRC_SEL) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_SPARE4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "6:5\\] Source select for aclk_ref 00: RCOSC_HF derived (31.25kHz) 01: XOSC_HF derived (31.25kHz) 10: RCOSC_LF (32kHz) 11: XOSC_LF (32.768kHz)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACLK_REF_SRC_SEL(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "6:5\\] Source select for aclk_ref 00: RCOSC_HF derived (31.25kHz) 01: XOSC_HF derived (31.25kHz) 10: RCOSC_LF (32kHz) 11: XOSC_LF (32.768kHz)."]
    #[inline(always)]
    pub const fn set_ACLK_REF_SRC_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "8:7\\] Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used."]
    #[must_use]
    #[inline(always)]
    pub const fn ACLK_TDC_SRC_SEL(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "8:7\\] Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used."]
    #[inline(always)]
    pub const fn set_ACLK_TDC_SRC_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "9:9\\] Enable clock loss detection and hence the indicators to system controller. Checks both SCLK_HF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_LOSS_EN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Enable clock loss detection and hence the indicators to system controller. Checks both SCLK_HF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[inline(always)]
    pub const fn set_CLK_LOSS_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_LF_DIG_BYPASS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
    #[inline(always)]
    pub const fn set_XOSC_LF_DIG_BYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_HF_POWER_MODE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_XOSC_HF_POWER_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSC_LF_TRIMMED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RCOSC_LF_TRIMMED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPOSC_MODE_EN(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPOSC_MODE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
    #[doc = "16:16\\] 0: Default - Switching of HF clock source is disabled . 1: Allows switching of sclk_hf source. Provided to prevent switching of the SCLK_HF source when running from flash (a long period during switching could corrupt flash). When sclk_hf switching is disabled, a new source can be started when SCLK_HF_SRC_SEL is changed, but the switch will not occur until this bit is set. This bit should be set to enable clock switching after STAT0.PENDINGSCLKHFSWITCHING indicates the new HF clock is ready. When switching completes (also indicated by STAT0.PENDINGSCLKHFSWITCHING) sclk_hf switching should be disabled to prevent flash corruption. Switching should not be enabled when running from flash."]
    #[must_use]
    #[inline(always)]
    pub const fn ALLOW_SCLK_HF_SWITCHING(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] 0: Default - Switching of HF clock source is disabled . 1: Allows switching of sclk_hf source. Provided to prevent switching of the SCLK_HF source when running from flash (a long period during switching could corrupt flash). When sclk_hf switching is disabled, a new source can be started when SCLK_HF_SRC_SEL is changed, but the switch will not occur until this bit is set. This bit should be set to enable clock switching after STAT0.PENDINGSCLKHFSWITCHING indicates the new HF clock is ready. When switching completes (also indicated by STAT0.PENDINGSCLKHFSWITCHING) sclk_hf switching should be disabled to prevent flash corruption. Switching should not be enabled when running from flash."]
    #[inline(always)]
    pub const fn set_ALLOW_SCLK_HF_SWITCHING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "21:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x1f;
        val as u8
    }
    #[doc = "21:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val as u32) & 0x1f) << 17usize);
    }
    #[doc = "22:22\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCE_KICKSTART_EN(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_FORCE_KICKSTART_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "24:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED23(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x03;
        val as u8
    }
    #[doc = "24:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED23(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
    }
    #[doc = "25:25\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DOUBLER_RESET_DURATION(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DOUBLER_RESET_DURATION(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "27:26\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn DOUBLER_START_DURATION(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "27:26\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_DOUBLER_START_DURATION(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "28:28\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASS_RCOSC_LF_CLK_QUAL(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BYPASS_RCOSC_LF_CLK_QUAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASS_XOSC_LF_CLK_QUAL(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BYPASS_XOSC_LF_CLK_QUAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Set based on the accurate high frequency XTAL."]
    #[must_use]
    #[inline(always)]
    pub const fn XTAL_IS_24M(&self) -> super::vals::XTAL_IS_24M {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::XTAL_IS_24M::from_bits(val as u8)
    }
    #[doc = "31:31\\] Set based on the accurate high frequency XTAL."]
    #[inline(always)]
    pub const fn set_XTAL_IS_24M(&mut self, val: super::vals::XTAL_IS_24M) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for CTL0 {
    #[inline(always)]
    fn default() -> CTL0 {
        CTL0(0)
    }
}
impl core::fmt::Debug for CTL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL0")
            .field("SCLK_HF_SRC_SEL", &self.SCLK_HF_SRC_SEL())
            .field("SCLK_MF_SRC_SEL", &self.SCLK_MF_SRC_SEL())
            .field("SCLK_LF_SRC_SEL", &self.SCLK_LF_SRC_SEL())
            .field("SPARE4", &self.SPARE4())
            .field("ACLK_REF_SRC_SEL", &self.ACLK_REF_SRC_SEL())
            .field("ACLK_TDC_SRC_SEL", &self.ACLK_TDC_SRC_SEL())
            .field("CLK_LOSS_EN", &self.CLK_LOSS_EN())
            .field("XOSC_LF_DIG_BYPASS", &self.XOSC_LF_DIG_BYPASS())
            .field("XOSC_HF_POWER_MODE", &self.XOSC_HF_POWER_MODE())
            .field("RCOSC_LF_TRIMMED", &self.RCOSC_LF_TRIMMED())
            .field("RESERVED13", &self.RESERVED13())
            .field("HPOSC_MODE_EN", &self.HPOSC_MODE_EN())
            .field("RESERVED15", &self.RESERVED15())
            .field("ALLOW_SCLK_HF_SWITCHING", &self.ALLOW_SCLK_HF_SWITCHING())
            .field("RESERVED17", &self.RESERVED17())
            .field("FORCE_KICKSTART_EN", &self.FORCE_KICKSTART_EN())
            .field("RESERVED23", &self.RESERVED23())
            .field("DOUBLER_RESET_DURATION", &self.DOUBLER_RESET_DURATION())
            .field("DOUBLER_START_DURATION", &self.DOUBLER_START_DURATION())
            .field("BYPASS_RCOSC_LF_CLK_QUAL", &self.BYPASS_RCOSC_LF_CLK_QUAL())
            .field("BYPASS_XOSC_LF_CLK_QUAL", &self.BYPASS_XOSC_LF_CLK_QUAL())
            .field("RESERVED30", &self.RESERVED30())
            .field("XTAL_IS_24M", &self.XTAL_IS_24M())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTL0 {{ SCLK_HF_SRC_SEL: {:?}, SCLK_MF_SRC_SEL: {:?}, SCLK_LF_SRC_SEL: {:?}, SPARE4: {=bool:?}, ACLK_REF_SRC_SEL: {=u8:?}, ACLK_TDC_SRC_SEL: {=u8:?}, CLK_LOSS_EN: {=bool:?}, XOSC_LF_DIG_BYPASS: {=bool:?}, XOSC_HF_POWER_MODE: {=bool:?}, RCOSC_LF_TRIMMED: {=bool:?}, RESERVED13: {=bool:?}, HPOSC_MODE_EN: {=bool:?}, RESERVED15: {=bool:?}, ALLOW_SCLK_HF_SWITCHING: {=bool:?}, RESERVED17: {=u8:?}, FORCE_KICKSTART_EN: {=bool:?}, RESERVED23: {=u8:?}, DOUBLER_RESET_DURATION: {=bool:?}, DOUBLER_START_DURATION: {=u8:?}, BYPASS_RCOSC_LF_CLK_QUAL: {=bool:?}, BYPASS_XOSC_LF_CLK_QUAL: {=bool:?}, RESERVED30: {=bool:?}, XTAL_IS_24M: {:?} }}",
            self.SCLK_HF_SRC_SEL(),
            self.SCLK_MF_SRC_SEL(),
            self.SCLK_LF_SRC_SEL(),
            self.SPARE4(),
            self.ACLK_REF_SRC_SEL(),
            self.ACLK_TDC_SRC_SEL(),
            self.CLK_LOSS_EN(),
            self.XOSC_LF_DIG_BYPASS(),
            self.XOSC_HF_POWER_MODE(),
            self.RCOSC_LF_TRIMMED(),
            self.RESERVED13(),
            self.HPOSC_MODE_EN(),
            self.RESERVED15(),
            self.ALLOW_SCLK_HF_SWITCHING(),
            self.RESERVED17(),
            self.FORCE_KICKSTART_EN(),
            self.RESERVED23(),
            self.DOUBLER_RESET_DURATION(),
            self.DOUBLER_START_DURATION(),
            self.BYPASS_RCOSC_LF_CLK_QUAL(),
            self.BYPASS_XOSC_LF_CLK_QUAL(),
            self.RESERVED30(),
            self.XTAL_IS_24M()
        )
    }
}
#[doc = "Control 1 This register contains OSC_DIG configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTL1(pub u32);
impl CTL1 {
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_HF_FAST_START(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_XOSC_HF_FAST_START(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "16:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE2(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x7fff;
        val as u16
    }
    #[doc = "16:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_SPARE2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 2usize)) | (((val as u32) & 0x7fff) << 2usize);
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSCHFCTRIMFRACT_EN(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RCOSCHFCTRIMFRACT_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "22:18\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSCHFCTRIMFRACT(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "22:18\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RCOSCHFCTRIMFRACT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "31:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED23(&self) -> u16 {
        let val = (self.0 >> 23usize) & 0x01ff;
        val as u16
    }
    #[doc = "31:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED23(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 23usize)) | (((val as u32) & 0x01ff) << 23usize);
    }
}
impl Default for CTL1 {
    #[inline(always)]
    fn default() -> CTL1 {
        CTL1(0)
    }
}
impl core::fmt::Debug for CTL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL1")
            .field("XOSC_HF_FAST_START", &self.XOSC_HF_FAST_START())
            .field("SPARE2", &self.SPARE2())
            .field("RCOSCHFCTRIMFRACT_EN", &self.RCOSCHFCTRIMFRACT_EN())
            .field("RCOSCHFCTRIMFRACT", &self.RCOSCHFCTRIMFRACT())
            .field("RESERVED23", &self.RESERVED23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTL1 {{ XOSC_HF_FAST_START: {=u8:?}, SPARE2: {=u16:?}, RCOSCHFCTRIMFRACT_EN: {=bool:?}, RCOSCHFCTRIMFRACT: {=u8:?}, RESERVED23: {=u16:?} }}",
            self.XOSC_HF_FAST_START(),
            self.SPARE2(),
            self.RCOSCHFCTRIMFRACT_EN(),
            self.RCOSCHFCTRIMFRACT(),
            self.RESERVED23()
        )
    }
}
#[doc = "Low Frequency Oscillator Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LFOSCCTL(pub u32);
impl LFOSCCTL {
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSCLF_CTUNE_TRIM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RCOSCLF_CTUNE_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "9:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSCLF_RTUNE_TRIM(&self) -> super::vals::RCOSCLF_RTUNE_TRIM {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RCOSCLF_RTUNE_TRIM::from_bits(val as u8)
    }
    #[doc = "9:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RCOSCLF_RTUNE_TRIM(&mut self, val: super::vals::RCOSCLF_RTUNE_TRIM) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "17:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED10(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0xff;
        val as u8
    }
    #[doc = "17:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED10(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 10usize)) | (((val as u32) & 0xff) << 10usize);
    }
    #[doc = "21:18\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSCLF_CMIRRWR_RATIO(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "21:18\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_XOSCLF_CMIRRWR_RATIO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "23:22\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSCLF_REGULATOR_TRIM(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "23:22\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_XOSCLF_REGULATOR_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
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
impl Default for LFOSCCTL {
    #[inline(always)]
    fn default() -> LFOSCCTL {
        LFOSCCTL(0)
    }
}
impl core::fmt::Debug for LFOSCCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFOSCCTL")
            .field("RCOSCLF_CTUNE_TRIM", &self.RCOSCLF_CTUNE_TRIM())
            .field("RCOSCLF_RTUNE_TRIM", &self.RCOSCLF_RTUNE_TRIM())
            .field("RESERVED10", &self.RESERVED10())
            .field("XOSCLF_CMIRRWR_RATIO", &self.XOSCLF_CMIRRWR_RATIO())
            .field("XOSCLF_REGULATOR_TRIM", &self.XOSCLF_REGULATOR_TRIM())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LFOSCCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LFOSCCTL {{ RCOSCLF_CTUNE_TRIM: {=u8:?}, RCOSCLF_RTUNE_TRIM: {:?}, RESERVED10: {=u8:?}, XOSCLF_CMIRRWR_RATIO: {=u8:?}, XOSCLF_REGULATOR_TRIM: {=u8:?}, RESERVED24: {=u8:?} }}",
            self.RCOSCLF_CTUNE_TRIM(),
            self.RCOSCLF_RTUNE_TRIM(),
            self.RESERVED10(),
            self.XOSCLF_CMIRRWR_RATIO(),
            self.XOSCLF_REGULATOR_TRIM(),
            self.RESERVED24()
        )
    }
}
#[doc = "RADC External Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RADCEXTCFG(pub u32);
impl RADCEXTCFG {
    #[doc = "4:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "4:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RADC_MODE_IS_SAR(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RADC_MODE_IS_SAR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RADC_DAC_TH(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "11:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RADC_DAC_TH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn IDAC_STEP(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_IDAC_STEP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "21:16\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_IBIAS_WAIT_CNT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "21:16\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LPM_IBIAS_WAIT_CNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "31:22\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HPM_IBIAS_WAIT_CNT(&self) -> u16 {
        let val = (self.0 >> 22usize) & 0x03ff;
        val as u16
    }
    #[doc = "31:22\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HPM_IBIAS_WAIT_CNT(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 22usize)) | (((val as u32) & 0x03ff) << 22usize);
    }
}
impl Default for RADCEXTCFG {
    #[inline(always)]
    fn default() -> RADCEXTCFG {
        RADCEXTCFG(0)
    }
}
impl core::fmt::Debug for RADCEXTCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RADCEXTCFG")
            .field("RESERVED0", &self.RESERVED0())
            .field("RADC_MODE_IS_SAR", &self.RADC_MODE_IS_SAR())
            .field("RADC_DAC_TH", &self.RADC_DAC_TH())
            .field("IDAC_STEP", &self.IDAC_STEP())
            .field("LPM_IBIAS_WAIT_CNT", &self.LPM_IBIAS_WAIT_CNT())
            .field("HPM_IBIAS_WAIT_CNT", &self.HPM_IBIAS_WAIT_CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RADCEXTCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RADCEXTCFG {{ RESERVED0: {=u8:?}, RADC_MODE_IS_SAR: {=bool:?}, RADC_DAC_TH: {=u8:?}, IDAC_STEP: {=u8:?}, LPM_IBIAS_WAIT_CNT: {=u8:?}, HPM_IBIAS_WAIT_CNT: {=u16:?} }}",
            self.RESERVED0(),
            self.RADC_MODE_IS_SAR(),
            self.RADC_DAC_TH(),
            self.IDAC_STEP(),
            self.LPM_IBIAS_WAIT_CNT(),
            self.HPM_IBIAS_WAIT_CNT()
        )
    }
}
#[doc = "RCOSCHF Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCOSCHFCTL(pub u32);
impl RCOSCHFCTL {
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
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSCHF_CTRIM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_RCOSCHF_CTRIM(&mut self, val: u8) {
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
impl Default for RCOSCHFCTL {
    #[inline(always)]
    fn default() -> RCOSCHFCTL {
        RCOSCHFCTL(0)
    }
}
impl core::fmt::Debug for RCOSCHFCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCOSCHFCTL")
            .field("RESERVED0", &self.RESERVED0())
            .field("RCOSCHF_CTRIM", &self.RCOSCHF_CTRIM())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCOSCHFCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCOSCHFCTL {{ RESERVED0: {=u8:?}, RCOSCHF_CTRIM: {=u8:?}, RESERVED16: {=u16:?} }}",
            self.RESERVED0(),
            self.RCOSCHF_CTRIM(),
            self.RESERVED16()
        )
    }
}
#[doc = "Status 0 This register contains status signals from OSC_DIG."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STAT0(pub u32);
impl STAT0 {
    #[doc = "0:0\\] Indicates when sclk_hf is ready to be switched."]
    #[must_use]
    #[inline(always)]
    pub const fn PENDINGSCLKHFSWITCHING(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Indicates when sclk_hf is ready to be switched."]
    #[inline(always)]
    pub const fn set_PENDINGSCLKHFSWITCHING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "6:1\\] adc_data."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_DATA(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "6:1\\] adc_data."]
    #[inline(always)]
    pub const fn set_ADC_DATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "7:7\\] indicates when adc_data is ready."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_DATA_READY(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] indicates when adc_data is ready."]
    #[inline(always)]
    pub const fn set_ADC_DATA_READY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] ADC_THMET."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_THMET(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] ADC_THMET."]
    #[inline(always)]
    pub const fn set_ADC_THMET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] XOSC_HF_HP_BUF_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_HF_HP_BUF_EN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] XOSC_HF_HP_BUF_EN."]
    #[inline(always)]
    pub const fn set_XOSC_HF_HP_BUF_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] XOSC_HF_LP_BUF_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_HF_LP_BUF_EN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] XOSC_HF_LP_BUF_EN."]
    #[inline(always)]
    pub const fn set_XOSC_HF_LP_BUF_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Indicates that the 48MHz clock from the DOUBLER is enabled. It will be enabled if 24 or 48 MHz crystal is used (enabled in doubler bypass for the 48MHz crystal)."]
    #[must_use]
    #[inline(always)]
    pub const fn XB_48M_CLK_EN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Indicates that the 48MHz clock from the DOUBLER is enabled. It will be enabled if 24 or 48 MHz crystal is used (enabled in doubler bypass for the 48MHz crystal)."]
    #[inline(always)]
    pub const fn set_XB_48M_CLK_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Indicates that XOSC_HF is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_HF_EN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Indicates that XOSC_HF is enabled."]
    #[inline(always)]
    pub const fn set_XOSC_HF_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Indicates sclk_lf is lost."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLK_LF_LOSS(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Indicates sclk_lf is lost."]
    #[inline(always)]
    pub const fn set_SCLK_LF_LOSS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Indicates sclk_hf is lost."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLK_HF_LOSS(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Indicates sclk_hf is lost."]
    #[inline(always)]
    pub const fn set_SCLK_HF_LOSS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] CLK_DCDC_RDY_ACK."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_DCDC_RDY_ACK(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] CLK_DCDC_RDY_ACK."]
    #[inline(always)]
    pub const fn set_CLK_DCDC_RDY_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] CLK_DCDC_RDY."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_DCDC_RDY(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] CLK_DCDC_RDY."]
    #[inline(always)]
    pub const fn set_CLK_DCDC_RDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] XOSC_LF_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_LF_EN(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] XOSC_LF_EN."]
    #[inline(always)]
    pub const fn set_XOSC_LF_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] RCOSC_LF_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSC_LF_EN(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] RCOSC_LF_EN."]
    #[inline(always)]
    pub const fn set_RCOSC_LF_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] RCOSC_HF_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn RCOSC_HF_EN(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] RCOSC_HF_EN."]
    #[inline(always)]
    pub const fn set_RCOSC_HF_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "27:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED23(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x1f;
        val as u8
    }
    #[doc = "27:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED23(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 23usize)) | (((val as u32) & 0x1f) << 23usize);
    }
    #[doc = "28:28\\] Indicates source for the sclk_hf."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLK_HF_SRC(&self) -> super::vals::SCLK_HF_SRC {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::SCLK_HF_SRC::from_bits(val as u8)
    }
    #[doc = "28:28\\] Indicates source for the sclk_hf."]
    #[inline(always)]
    pub const fn set_SCLK_HF_SRC(&mut self, val: super::vals::SCLK_HF_SRC) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "30:29\\] Indicates source for the sclk_lf."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLK_LF_SRC(&self) -> super::vals::SCLK_LF_SRC {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::SCLK_LF_SRC::from_bits(val as u8)
    }
    #[doc = "30:29\\] Indicates source for the sclk_lf."]
    #[inline(always)]
    pub const fn set_SCLK_LF_SRC(&mut self, val: super::vals::SCLK_LF_SRC) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn SPARE31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_SPARE31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for STAT0 {
    #[inline(always)]
    fn default() -> STAT0 {
        STAT0(0)
    }
}
impl core::fmt::Debug for STAT0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT0")
            .field("PENDINGSCLKHFSWITCHING", &self.PENDINGSCLKHFSWITCHING())
            .field("ADC_DATA", &self.ADC_DATA())
            .field("ADC_DATA_READY", &self.ADC_DATA_READY())
            .field("ADC_THMET", &self.ADC_THMET())
            .field("RESERVED9", &self.RESERVED9())
            .field("XOSC_HF_HP_BUF_EN", &self.XOSC_HF_HP_BUF_EN())
            .field("XOSC_HF_LP_BUF_EN", &self.XOSC_HF_LP_BUF_EN())
            .field("RESERVED12", &self.RESERVED12())
            .field("XB_48M_CLK_EN", &self.XB_48M_CLK_EN())
            .field("RESERVED14", &self.RESERVED14())
            .field("XOSC_HF_EN", &self.XOSC_HF_EN())
            .field("SCLK_LF_LOSS", &self.SCLK_LF_LOSS())
            .field("SCLK_HF_LOSS", &self.SCLK_HF_LOSS())
            .field("CLK_DCDC_RDY_ACK", &self.CLK_DCDC_RDY_ACK())
            .field("CLK_DCDC_RDY", &self.CLK_DCDC_RDY())
            .field("XOSC_LF_EN", &self.XOSC_LF_EN())
            .field("RCOSC_LF_EN", &self.RCOSC_LF_EN())
            .field("RCOSC_HF_EN", &self.RCOSC_HF_EN())
            .field("RESERVED23", &self.RESERVED23())
            .field("SCLK_HF_SRC", &self.SCLK_HF_SRC())
            .field("SCLK_LF_SRC", &self.SCLK_LF_SRC())
            .field("SPARE31", &self.SPARE31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STAT0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STAT0 {{ PENDINGSCLKHFSWITCHING: {=bool:?}, ADC_DATA: {=u8:?}, ADC_DATA_READY: {=bool:?}, ADC_THMET: {=bool:?}, RESERVED9: {=bool:?}, XOSC_HF_HP_BUF_EN: {=bool:?}, XOSC_HF_LP_BUF_EN: {=bool:?}, RESERVED12: {=bool:?}, XB_48M_CLK_EN: {=bool:?}, RESERVED14: {=bool:?}, XOSC_HF_EN: {=bool:?}, SCLK_LF_LOSS: {=bool:?}, SCLK_HF_LOSS: {=bool:?}, CLK_DCDC_RDY_ACK: {=bool:?}, CLK_DCDC_RDY: {=bool:?}, XOSC_LF_EN: {=bool:?}, RCOSC_LF_EN: {=bool:?}, RCOSC_HF_EN: {=bool:?}, RESERVED23: {=u8:?}, SCLK_HF_SRC: {:?}, SCLK_LF_SRC: {:?}, SPARE31: {=bool:?} }}",
            self.PENDINGSCLKHFSWITCHING(),
            self.ADC_DATA(),
            self.ADC_DATA_READY(),
            self.ADC_THMET(),
            self.RESERVED9(),
            self.XOSC_HF_HP_BUF_EN(),
            self.XOSC_HF_LP_BUF_EN(),
            self.RESERVED12(),
            self.XB_48M_CLK_EN(),
            self.RESERVED14(),
            self.XOSC_HF_EN(),
            self.SCLK_LF_LOSS(),
            self.SCLK_HF_LOSS(),
            self.CLK_DCDC_RDY_ACK(),
            self.CLK_DCDC_RDY(),
            self.XOSC_LF_EN(),
            self.RCOSC_LF_EN(),
            self.RCOSC_HF_EN(),
            self.RESERVED23(),
            self.SCLK_HF_SRC(),
            self.SCLK_LF_SRC(),
            self.SPARE31()
        )
    }
}
#[doc = "Status 1 This register contains status signals from OSC_DIG."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STAT1(pub u32);
impl STAT1 {
    #[doc = "0:0\\] CLK_DCDC_GOOD."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_DCDC_GOOD(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] CLK_DCDC_GOOD."]
    #[inline(always)]
    pub const fn set_CLK_DCDC_GOOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] CLK_CHP_GOOD."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_CHP_GOOD(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] CLK_CHP_GOOD."]
    #[inline(always)]
    pub const fn set_CLK_CHP_GOOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] ACLK_REF_GOOD."]
    #[must_use]
    #[inline(always)]
    pub const fn ACLK_REF_GOOD(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] ACLK_REF_GOOD."]
    #[inline(always)]
    pub const fn set_ACLK_REF_GOOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] ACLK_TDC_GOOD."]
    #[must_use]
    #[inline(always)]
    pub const fn ACLK_TDC_GOOD(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] ACLK_TDC_GOOD."]
    #[inline(always)]
    pub const fn set_ACLK_TDC_GOOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] ACLK_ADC_GOOD."]
    #[must_use]
    #[inline(always)]
    pub const fn ACLK_ADC_GOOD(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] ACLK_ADC_GOOD."]
    #[inline(always)]
    pub const fn set_ACLK_ADC_GOOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] SCLK_LF_GOOD."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLK_LF_GOOD(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] SCLK_LF_GOOD."]
    #[inline(always)]
    pub const fn set_SCLK_LF_GOOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] SCLK_MF_GOOD."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLK_MF_GOOD(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] SCLK_MF_GOOD."]
    #[inline(always)]
    pub const fn set_SCLK_MF_GOOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] SCLK_HF_GOOD."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLK_HF_GOOD(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] SCLK_HF_GOOD."]
    #[inline(always)]
    pub const fn set_SCLK_HF_GOOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] CLK_DCDC_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_DCDC_EN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] CLK_DCDC_EN."]
    #[inline(always)]
    pub const fn set_CLK_DCDC_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] CLK_CHP_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_CHP_EN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] CLK_CHP_EN."]
    #[inline(always)]
    pub const fn set_CLK_CHP_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] ACLK_REF_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn ACLK_REF_EN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] ACLK_REF_EN."]
    #[inline(always)]
    pub const fn set_ACLK_REF_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] ACLK_TDC_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn ACLK_TDC_EN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] ACLK_TDC_EN."]
    #[inline(always)]
    pub const fn set_ACLK_TDC_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] ACLK_ADC_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn ACLK_ADC_EN(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] ACLK_ADC_EN."]
    #[inline(always)]
    pub const fn set_ACLK_ADC_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] SCLK_MF_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLK_MF_EN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] SCLK_MF_EN."]
    #[inline(always)]
    pub const fn set_SCLK_MF_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] SCLK_HF_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLK_HF_EN(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] SCLK_HF_EN."]
    #[inline(always)]
    pub const fn set_SCLK_HF_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] force_rcosc_hf."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCE_RCOSC_HF(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] force_rcosc_hf."]
    #[inline(always)]
    pub const fn set_FORCE_RCOSC_HF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "21:16\\] OSC amplitude during LPM_UPDATE state When amplitude compensation of XOSC_HF is enabled in low power mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_UPDATE_AMP(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "21:16\\] OSC amplitude during LPM_UPDATE state When amplitude compensation of XOSC_HF is enabled in low power mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[inline(always)]
    pub const fn set_LPM_UPDATE_AMP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "27:22\\] OSC amplitude during HPM_UPDATE state. When amplitude compensation of XOSC_HF is enabled in high performance mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[must_use]
    #[inline(always)]
    pub const fn HPM_UPDATE_AMP(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x3f;
        val as u8
    }
    #[doc = "27:22\\] OSC amplitude during HPM_UPDATE state. When amplitude compensation of XOSC_HF is enabled in high performance mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[inline(always)]
    pub const fn set_HPM_UPDATE_AMP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 22usize)) | (((val as u32) & 0x3f) << 22usize);
    }
    #[doc = "31:28\\] AMPCOMP FSM State."]
    #[must_use]
    #[inline(always)]
    pub const fn RAMPSTATE(&self) -> super::vals::RAMPSTATE {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::RAMPSTATE::from_bits(val as u8)
    }
    #[doc = "31:28\\] AMPCOMP FSM State."]
    #[inline(always)]
    pub const fn set_RAMPSTATE(&mut self, val: super::vals::RAMPSTATE) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for STAT1 {
    #[inline(always)]
    fn default() -> STAT1 {
        STAT1(0)
    }
}
impl core::fmt::Debug for STAT1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT1")
            .field("CLK_DCDC_GOOD", &self.CLK_DCDC_GOOD())
            .field("CLK_CHP_GOOD", &self.CLK_CHP_GOOD())
            .field("ACLK_REF_GOOD", &self.ACLK_REF_GOOD())
            .field("ACLK_TDC_GOOD", &self.ACLK_TDC_GOOD())
            .field("ACLK_ADC_GOOD", &self.ACLK_ADC_GOOD())
            .field("SCLK_LF_GOOD", &self.SCLK_LF_GOOD())
            .field("SCLK_MF_GOOD", &self.SCLK_MF_GOOD())
            .field("SCLK_HF_GOOD", &self.SCLK_HF_GOOD())
            .field("CLK_DCDC_EN", &self.CLK_DCDC_EN())
            .field("CLK_CHP_EN", &self.CLK_CHP_EN())
            .field("ACLK_REF_EN", &self.ACLK_REF_EN())
            .field("ACLK_TDC_EN", &self.ACLK_TDC_EN())
            .field("ACLK_ADC_EN", &self.ACLK_ADC_EN())
            .field("SCLK_MF_EN", &self.SCLK_MF_EN())
            .field("SCLK_HF_EN", &self.SCLK_HF_EN())
            .field("FORCE_RCOSC_HF", &self.FORCE_RCOSC_HF())
            .field("LPM_UPDATE_AMP", &self.LPM_UPDATE_AMP())
            .field("HPM_UPDATE_AMP", &self.HPM_UPDATE_AMP())
            .field("RAMPSTATE", &self.RAMPSTATE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STAT1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STAT1 {{ CLK_DCDC_GOOD: {=bool:?}, CLK_CHP_GOOD: {=bool:?}, ACLK_REF_GOOD: {=bool:?}, ACLK_TDC_GOOD: {=bool:?}, ACLK_ADC_GOOD: {=bool:?}, SCLK_LF_GOOD: {=bool:?}, SCLK_MF_GOOD: {=bool:?}, SCLK_HF_GOOD: {=bool:?}, CLK_DCDC_EN: {=bool:?}, CLK_CHP_EN: {=bool:?}, ACLK_REF_EN: {=bool:?}, ACLK_TDC_EN: {=bool:?}, ACLK_ADC_EN: {=bool:?}, SCLK_MF_EN: {=bool:?}, SCLK_HF_EN: {=bool:?}, FORCE_RCOSC_HF: {=bool:?}, LPM_UPDATE_AMP: {=u8:?}, HPM_UPDATE_AMP: {=u8:?}, RAMPSTATE: {:?} }}",
            self.CLK_DCDC_GOOD(),
            self.CLK_CHP_GOOD(),
            self.ACLK_REF_GOOD(),
            self.ACLK_TDC_GOOD(),
            self.ACLK_ADC_GOOD(),
            self.SCLK_LF_GOOD(),
            self.SCLK_MF_GOOD(),
            self.SCLK_HF_GOOD(),
            self.CLK_DCDC_EN(),
            self.CLK_CHP_EN(),
            self.ACLK_REF_EN(),
            self.ACLK_TDC_EN(),
            self.ACLK_ADC_EN(),
            self.SCLK_MF_EN(),
            self.SCLK_HF_EN(),
            self.FORCE_RCOSC_HF(),
            self.LPM_UPDATE_AMP(),
            self.HPM_UPDATE_AMP(),
            self.RAMPSTATE()
        )
    }
}
#[doc = "Status 2 This register contains status signals from AMPCOMP FSM."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STAT2(pub u32);
impl STAT2 {
    #[doc = "0:0\\] frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_HF_RF_FREQGOOD(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
    #[inline(always)]
    pub const fn set_XOSC_HF_RF_FREQGOOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] frequency of xosc_hf is good to use for the digital clocks."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_HF_FREQGOOD(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] frequency of xosc_hf is good to use for the digital clocks."]
    #[inline(always)]
    pub const fn set_XOSC_HF_FREQGOOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_HF_AMPGOOD(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status."]
    #[inline(always)]
    pub const fn set_XOSC_HF_AMPGOOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] ampcomp_req."]
    #[must_use]
    #[inline(always)]
    pub const fn AMPCOMP_REQ(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] ampcomp_req."]
    #[inline(always)]
    pub const fn set_AMPCOMP_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "11:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "11:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "15:12\\] xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
    #[must_use]
    #[inline(always)]
    pub const fn RAMPSTATE(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
    #[inline(always)]
    pub const fn set_RAMPSTATE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "22:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "22:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "23:23\\] Indication of threshold is met for hpm_ramp3."]
    #[must_use]
    #[inline(always)]
    pub const fn HPM_RAMP3_THMET(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Indication of threshold is met for hpm_ramp3."]
    #[inline(always)]
    pub const fn set_HPM_RAMP3_THMET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Indication of threshold is met for hpm_ramp2."]
    #[must_use]
    #[inline(always)]
    pub const fn HPM_RAMP2_THMET(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Indication of threshold is met for hpm_ramp2."]
    #[inline(always)]
    pub const fn set_HPM_RAMP2_THMET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Indication of threshold is met for hpm_ramp1."]
    #[must_use]
    #[inline(always)]
    pub const fn HPM_RAMP1_THMET(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Indication of threshold is met for hpm_ramp1."]
    #[inline(always)]
    pub const fn set_HPM_RAMP1_THMET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "31:26\\] DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_DCBIAS(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "31:26\\] DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
    #[inline(always)]
    pub const fn set_ADC_DCBIAS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for STAT2 {
    #[inline(always)]
    fn default() -> STAT2 {
        STAT2(0)
    }
}
impl core::fmt::Debug for STAT2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT2")
            .field("XOSC_HF_RF_FREQGOOD", &self.XOSC_HF_RF_FREQGOOD())
            .field("XOSC_HF_FREQGOOD", &self.XOSC_HF_FREQGOOD())
            .field("XOSC_HF_AMPGOOD", &self.XOSC_HF_AMPGOOD())
            .field("AMPCOMP_REQ", &self.AMPCOMP_REQ())
            .field("RESERVED4", &self.RESERVED4())
            .field("RAMPSTATE", &self.RAMPSTATE())
            .field("RESERVED16", &self.RESERVED16())
            .field("HPM_RAMP3_THMET", &self.HPM_RAMP3_THMET())
            .field("HPM_RAMP2_THMET", &self.HPM_RAMP2_THMET())
            .field("HPM_RAMP1_THMET", &self.HPM_RAMP1_THMET())
            .field("ADC_DCBIAS", &self.ADC_DCBIAS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STAT2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STAT2 {{ XOSC_HF_RF_FREQGOOD: {=bool:?}, XOSC_HF_FREQGOOD: {=bool:?}, XOSC_HF_AMPGOOD: {=bool:?}, AMPCOMP_REQ: {=bool:?}, RESERVED4: {=u8:?}, RAMPSTATE: {=u8:?}, RESERVED16: {=u8:?}, HPM_RAMP3_THMET: {=bool:?}, HPM_RAMP2_THMET: {=bool:?}, HPM_RAMP1_THMET: {=bool:?}, ADC_DCBIAS: {=u8:?} }}",
            self.XOSC_HF_RF_FREQGOOD(),
            self.XOSC_HF_FREQGOOD(),
            self.XOSC_HF_AMPGOOD(),
            self.AMPCOMP_REQ(),
            self.RESERVED4(),
            self.RAMPSTATE(),
            self.RESERVED16(),
            self.HPM_RAMP3_THMET(),
            self.HPM_RAMP2_THMET(),
            self.HPM_RAMP1_THMET(),
            self.ADC_DCBIAS()
        )
    }
}
#[doc = "XOSCHF Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XOSCHFCTL(pub u32);
impl XOSCHFCTL {
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn LP_BUF_ITRIM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_LP_BUF_ITRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "4:2\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn HP_BUF_ITRIM(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "4:2\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_HP_BUF_ITRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "5:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASS(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_BYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "9:8\\] Internal. Only to be used through TI provided API."]
    #[must_use]
    #[inline(always)]
    pub const fn PEAK_DET_ITRIM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "9:8\\] Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn set_PEAK_DET_ITRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "31:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED10(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "31:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED10(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for XOSCHFCTL {
    #[inline(always)]
    fn default() -> XOSCHFCTL {
        XOSCHFCTL(0)
    }
}
impl core::fmt::Debug for XOSCHFCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XOSCHFCTL")
            .field("LP_BUF_ITRIM", &self.LP_BUF_ITRIM())
            .field("HP_BUF_ITRIM", &self.HP_BUF_ITRIM())
            .field("RESERVED5", &self.RESERVED5())
            .field("BYPASS", &self.BYPASS())
            .field("RESERVED7", &self.RESERVED7())
            .field("PEAK_DET_ITRIM", &self.PEAK_DET_ITRIM())
            .field("RESERVED10", &self.RESERVED10())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XOSCHFCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "XOSCHFCTL {{ LP_BUF_ITRIM: {=u8:?}, HP_BUF_ITRIM: {=u8:?}, RESERVED5: {=bool:?}, BYPASS: {=bool:?}, RESERVED7: {=bool:?}, PEAK_DET_ITRIM: {=u8:?}, RESERVED10: {=u32:?} }}",
            self.LP_BUF_ITRIM(),
            self.HP_BUF_ITRIM(),
            self.RESERVED5(),
            self.BYPASS(),
            self.RESERVED7(),
            self.PEAK_DET_ITRIM(),
            self.RESERVED10()
        )
    }
}

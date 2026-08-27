#[doc = "Factory configuration area (FCFG1)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCFG1 {
    ptr: *mut u8,
}
unsafe impl Send for FCFG1 {}
unsafe impl Sync for FCFG1 {}
impl FCFG1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Misc configurations."]
    #[inline(always)]
    pub const fn MISC_CONF_1(self) -> crate::common::Reg<regs::MISC_CONF_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn MISC_CONF_2(self) -> crate::common::Reg<regs::MISC_CONF_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_RF_FRONTEND_DIV5(
        self,
    ) -> crate::common::Reg<regs::CONFIG_RF_FRONTEND_DIV5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_RF_FRONTEND_DIV6(
        self,
    ) -> crate::common::Reg<regs::CONFIG_RF_FRONTEND_DIV6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_RF_FRONTEND_DIV10(
        self,
    ) -> crate::common::Reg<regs::CONFIG_RF_FRONTEND_DIV10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_RF_FRONTEND_DIV12(
        self,
    ) -> crate::common::Reg<regs::CONFIG_RF_FRONTEND_DIV12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_RF_FRONTEND_DIV15(
        self,
    ) -> crate::common::Reg<regs::CONFIG_RF_FRONTEND_DIV15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_RF_FRONTEND_DIV30(
        self,
    ) -> crate::common::Reg<regs::CONFIG_RF_FRONTEND_DIV30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_SYNTH_DIV5(
        self,
    ) -> crate::common::Reg<regs::CONFIG_SYNTH_DIV5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_SYNTH_DIV6(
        self,
    ) -> crate::common::Reg<regs::CONFIG_SYNTH_DIV6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_SYNTH_DIV10(
        self,
    ) -> crate::common::Reg<regs::CONFIG_SYNTH_DIV10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_SYNTH_DIV12(
        self,
    ) -> crate::common::Reg<regs::CONFIG_SYNTH_DIV12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_SYNTH_DIV15(
        self,
    ) -> crate::common::Reg<regs::CONFIG_SYNTH_DIV15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_SYNTH_DIV30(
        self,
    ) -> crate::common::Reg<regs::CONFIG_SYNTH_DIV30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_MISC_ADC_DIV5(
        self,
    ) -> crate::common::Reg<regs::CONFIG_MISC_ADC_DIV5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_MISC_ADC_DIV6(
        self,
    ) -> crate::common::Reg<regs::CONFIG_MISC_ADC_DIV6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_MISC_ADC_DIV10(
        self,
    ) -> crate::common::Reg<regs::CONFIG_MISC_ADC_DIV10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_MISC_ADC_DIV12(
        self,
    ) -> crate::common::Reg<regs::CONFIG_MISC_ADC_DIV12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_MISC_ADC_DIV15(
        self,
    ) -> crate::common::Reg<regs::CONFIG_MISC_ADC_DIV15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_MISC_ADC_DIV30(
        self,
    ) -> crate::common::Reg<regs::CONFIG_MISC_ADC_DIV30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Shadow of DIE_ID_0 register in eFuse."]
    #[inline(always)]
    pub const fn SHDW_DIE_ID_0(self) -> crate::common::Reg<regs::SHDW_DIE_ID_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Shadow of DIE_ID_1 register in eFuse."]
    #[inline(always)]
    pub const fn SHDW_DIE_ID_1(self) -> crate::common::Reg<regs::SHDW_DIE_ID_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Shadow of DIE_ID_2 register in eFuse."]
    #[inline(always)]
    pub const fn SHDW_DIE_ID_2(self) -> crate::common::Reg<regs::SHDW_DIE_ID_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Shadow of DIE_ID_3 register in eFuse."]
    #[inline(always)]
    pub const fn SHDW_DIE_ID_3(self) -> crate::common::Reg<regs::SHDW_DIE_ID_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn SHDW_OSC_BIAS_LDO_TRIM(
        self,
    ) -> crate::common::Reg<regs::SHDW_OSC_BIAS_LDO_TRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn SHDW_ANA_TRIM(self) -> crate::common::Reg<regs::SHDW_ANA_TRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "FLASH_NUMBER."]
    #[inline(always)]
    pub const fn FLASH_NUMBER(self) -> crate::common::Reg<regs::FLASH_NUMBER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "FLASH_COORDINATE."]
    #[inline(always)]
    pub const fn FLASH_COORDINATE(
        self,
    ) -> crate::common::Reg<regs::FLASH_COORDINATE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn FLASH_E_P(self) -> crate::common::Reg<regs::FLASH_E_P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn FLASH_C_E_P_R(self) -> crate::common::Reg<regs::FLASH_C_E_P_R, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn FLASH_P_R_PV(self) -> crate::common::Reg<regs::FLASH_P_R_PV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn FLASH_EH_SEQ(self) -> crate::common::Reg<regs::FLASH_EH_SEQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn FLASH_VHV_E(self) -> crate::common::Reg<regs::FLASH_VHV_E, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn FLASH_PP(self) -> crate::common::Reg<regs::FLASH_PP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn FLASH_PROG_EP(self) -> crate::common::Reg<regs::FLASH_PROG_EP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn FLASH_ERA_PW(self) -> crate::common::Reg<regs::FLASH_ERA_PW, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn FLASH_VHV(self) -> crate::common::Reg<regs::FLASH_VHV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn FLASH_VHV_PV(self) -> crate::common::Reg<regs::FLASH_VHV_PV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn FLASH_V(self) -> crate::common::Reg<regs::FLASH_V, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "User Identification. Reading this register and the ICEPICK_DEVICE_ID register is the only support way of identifying a device. The value of this register will be written to AON_WUC:JTAGUSERCODE by boot FW while in safezone."]
    #[inline(always)]
    pub const fn USER_ID(self) -> crate::common::Reg<regs::USER_ID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn FLASH_OTP_DATA3(
        self,
    ) -> crate::common::Reg<regs::FLASH_OTP_DATA3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ANA2_TRIM(self) -> crate::common::Reg<regs::ANA2_TRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn LDO_TRIM(self) -> crate::common::Reg<regs::LDO_TRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn BAT_RC_LDO_TRIM(
        self,
    ) -> crate::common::Reg<regs::BAT_RC_LDO_TRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "MAC BLE Address 0."]
    #[inline(always)]
    pub const fn MAC_BLE_0(self) -> crate::common::Reg<regs::MAC_BLE_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e8usize) as _) }
    }
    #[doc = "MAC BLE Address 1."]
    #[inline(always)]
    pub const fn MAC_BLE_1(self) -> crate::common::Reg<regs::MAC_BLE_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ecusize) as _) }
    }
    #[doc = "MAC IEEE 802.15.4 Address 0."]
    #[inline(always)]
    pub const fn MAC_15_4_0(self) -> crate::common::Reg<regs::MAC_15_4_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[doc = "MAC IEEE 802.15.4 Address 1."]
    #[inline(always)]
    pub const fn MAC_15_4_1(self) -> crate::common::Reg<regs::MAC_15_4_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f4usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn FLASH_OTP_DATA4(
        self,
    ) -> crate::common::Reg<regs::FLASH_OTP_DATA4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "Miscellaneous Trim Parameters."]
    #[inline(always)]
    pub const fn MISC_TRIM(self) -> crate::common::Reg<regs::MISC_TRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn RCOSC_HF_TEMPCOMP(
        self,
    ) -> crate::common::Reg<regs::RCOSC_HF_TEMPCOMP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn TRIM_CAL_REVISION(
        self,
    ) -> crate::common::Reg<regs::TRIM_CAL_REVISION, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "IcePick Device Identification Reading this register and the USER_ID register is the only support way of identifying a device."]
    #[inline(always)]
    pub const fn ICEPICK_DEVICE_ID(
        self,
    ) -> crate::common::Reg<regs::ICEPICK_DEVICE_ID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "Factory Configuration (FCFG1) Revision."]
    #[inline(always)]
    pub const fn FCFG1_REVISION(
        self,
    ) -> crate::common::Reg<regs::FCFG1_REVISION, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[doc = "Misc OTP Data."]
    #[inline(always)]
    pub const fn MISC_OTP_DATA(self) -> crate::common::Reg<regs::MISC_OTP_DATA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "IO Configuration."]
    #[inline(always)]
    pub const fn IOCONF(self) -> crate::common::Reg<regs::IOCONF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_IF_ADC(self) -> crate::common::Reg<regs::CONFIG_IF_ADC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x034cusize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_OSC_TOP(
        self,
    ) -> crate::common::Reg<regs::CONFIG_OSC_TOP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0350usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_RF_FRONTEND(
        self,
    ) -> crate::common::Reg<regs::CONFIG_RF_FRONTEND, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0354usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_SYNTH(self) -> crate::common::Reg<regs::CONFIG_SYNTH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0358usize) as _) }
    }
    #[doc = "AUX_ADC Gain in Absolute Reference Mode."]
    #[inline(always)]
    pub const fn SOC_ADC_ABS_GAIN(
        self,
    ) -> crate::common::Reg<regs::SOC_ADC_ABS_GAIN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x035cusize) as _) }
    }
    #[doc = "AUX_ADC Gain in Relative Reference Mode."]
    #[inline(always)]
    pub const fn SOC_ADC_REL_GAIN(
        self,
    ) -> crate::common::Reg<regs::SOC_ADC_REL_GAIN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "AUX_ADC Temperature Offsets in Absolute Reference Mode."]
    #[inline(always)]
    pub const fn SOC_ADC_OFFSET_INT(
        self,
    ) -> crate::common::Reg<regs::SOC_ADC_OFFSET_INT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn SOC_ADC_REF_TRIM_AND_OFFSET_EXT(
        self,
    ) -> crate::common::Reg<regs::SOC_ADC_REF_TRIM_AND_OFFSET_EXT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn AMPCOMP_TH1(self) -> crate::common::Reg<regs::AMPCOMP_TH1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn AMPCOMP_TH2(self) -> crate::common::Reg<regs::AMPCOMP_TH2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn AMPCOMP_CTRL1(self) -> crate::common::Reg<regs::AMPCOMP_CTRL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ANABYPASS_VALUE2(
        self,
    ) -> crate::common::Reg<regs::ANABYPASS_VALUE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CONFIG_MISC_ADC(
        self,
    ) -> crate::common::Reg<regs::CONFIG_MISC_ADC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn VOLT_TRIM(self) -> crate::common::Reg<regs::VOLT_TRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "OSC Configuration."]
    #[inline(always)]
    pub const fn OSC_CONF(self) -> crate::common::Reg<regs::OSC_CONF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn FREQ_OFFSET(self) -> crate::common::Reg<regs::FREQ_OFFSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn CAP_TRIM(self) -> crate::common::Reg<regs::CAP_TRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn MISC_OTP_DATA_1(
        self,
    ) -> crate::common::Reg<regs::MISC_OTP_DATA_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "Power Down Current Control 20C."]
    #[inline(always)]
    pub const fn PWD_CURR_20C(self) -> crate::common::Reg<regs::PWD_CURR_20C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[doc = "Power Down Current Control 35C."]
    #[inline(always)]
    pub const fn PWD_CURR_35C(self) -> crate::common::Reg<regs::PWD_CURR_35C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "Power Down Current Control 50C."]
    #[inline(always)]
    pub const fn PWD_CURR_50C(self) -> crate::common::Reg<regs::PWD_CURR_50C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "Power Down Current Control 65C."]
    #[inline(always)]
    pub const fn PWD_CURR_65C(self) -> crate::common::Reg<regs::PWD_CURR_65C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a8usize) as _) }
    }
    #[doc = "Power Down Current Control 80C."]
    #[inline(always)]
    pub const fn PWD_CURR_80C(self) -> crate::common::Reg<regs::PWD_CURR_80C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "Power Down Current Control 95C."]
    #[inline(always)]
    pub const fn PWD_CURR_95C(self) -> crate::common::Reg<regs::PWD_CURR_95C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize) as _) }
    }
    #[doc = "Power Down Current Control 110C."]
    #[inline(always)]
    pub const fn PWD_CURR_110C(self) -> crate::common::Reg<regs::PWD_CURR_110C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "Power Down Current Control 125C."]
    #[inline(always)]
    pub const fn PWD_CURR_125C(self) -> crate::common::Reg<regs::PWD_CURR_125C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b8usize) as _) }
    }
}
pub mod regs;

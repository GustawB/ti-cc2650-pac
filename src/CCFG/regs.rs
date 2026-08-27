#[doc = "Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BL_CONFIG(pub u32);
impl BL_CONFIG {
    #[doc = "7:0\\] Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn BL_ENABLE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Enables the boot loader backdoor. 0xC5: Boot loader backdoor is enabled. Any other value: Boot loader backdoor is disabled. NOTE! Boot loader must be enabled (see BOOTLOADER_ENABLE) if boot loader backdoor is enabled."]
    #[inline(always)]
    pub const fn set_BL_ENABLE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
    #[must_use]
    #[inline(always)]
    pub const fn BL_PIN_NUMBER(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] DIO number that is level checked if the boot loader backdoor is enabled by the BL_ENABLE field."]
    #[inline(always)]
    pub const fn set_BL_PIN_NUMBER(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "16:16\\] Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
    #[must_use]
    #[inline(always)]
    pub const fn BL_LEVEL(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Sets the active level of the selected DIO number BL_PIN_NUMBER if boot loader backdoor is enabled by the BL_ENABLE field. 0: Active low. 1: Active high."]
    #[inline(always)]
    pub const fn set_BL_LEVEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "31:24\\] Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is non-zero or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOTLOADER_ENABLE(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Bootloader enable. Boot loader can be accessed if IMAGE_VALID_CONF.IMAGE_VALID is non-zero or BL_ENABLE is enabled (and conditions for boot loader backdoor are met). 0xC5: Boot loader is enabled. Any other value: Boot loader is disabled."]
    #[inline(always)]
    pub const fn set_BOOTLOADER_ENABLE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for BL_CONFIG {
    #[inline(always)]
    fn default() -> BL_CONFIG {
        BL_CONFIG(0)
    }
}
impl core::fmt::Debug for BL_CONFIG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BL_CONFIG")
            .field("BL_ENABLE", &self.BL_ENABLE())
            .field("BL_PIN_NUMBER", &self.BL_PIN_NUMBER())
            .field("BL_LEVEL", &self.BL_LEVEL())
            .field("RESERVED", &self.RESERVED())
            .field("BOOTLOADER_ENABLE", &self.BOOTLOADER_ENABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BL_CONFIG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BL_CONFIG {{ BL_ENABLE: {=u8:?}, BL_PIN_NUMBER: {=u8:?}, BL_LEVEL: {=bool:?}, RESERVED: {=u8:?}, BOOTLOADER_ENABLE: {=u8:?} }}",
            self.BL_ENABLE(),
            self.BL_PIN_NUMBER(),
            self.BL_LEVEL(),
            self.RESERVED(),
            self.BOOTLOADER_ENABLE()
        )
    }
}
#[doc = "Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCFG_PROT_127_96(pub u32);
impl CCFG_PROT_127_96 {
    #[doc = "0:0\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_96(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_96(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_97(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_97(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_98(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_98(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_99(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_99(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_100(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_100(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_101(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_101(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_102(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_102(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_103(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_103(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_104(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_104(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_105(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_105(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_106(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_106(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_107(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_107(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_108(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_108(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_109(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_109(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_110(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_110(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_111(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_111(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_112(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_112(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_113(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_113(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_114(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_114(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_115(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_115(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_116(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_116(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_117(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_117(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_118(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_118(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_119(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_119(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_120(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_120(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_121(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_121(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_122(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_122(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_123(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_123(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_124(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_124(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_125(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_125(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_126(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_126(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_127(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_127(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CCFG_PROT_127_96 {
    #[inline(always)]
    fn default() -> CCFG_PROT_127_96 {
        CCFG_PROT_127_96(0)
    }
}
impl core::fmt::Debug for CCFG_PROT_127_96 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCFG_PROT_127_96")
            .field("WRT_PROT_SEC_96", &self.WRT_PROT_SEC_96())
            .field("WRT_PROT_SEC_97", &self.WRT_PROT_SEC_97())
            .field("WRT_PROT_SEC_98", &self.WRT_PROT_SEC_98())
            .field("WRT_PROT_SEC_99", &self.WRT_PROT_SEC_99())
            .field("WRT_PROT_SEC_100", &self.WRT_PROT_SEC_100())
            .field("WRT_PROT_SEC_101", &self.WRT_PROT_SEC_101())
            .field("WRT_PROT_SEC_102", &self.WRT_PROT_SEC_102())
            .field("WRT_PROT_SEC_103", &self.WRT_PROT_SEC_103())
            .field("WRT_PROT_SEC_104", &self.WRT_PROT_SEC_104())
            .field("WRT_PROT_SEC_105", &self.WRT_PROT_SEC_105())
            .field("WRT_PROT_SEC_106", &self.WRT_PROT_SEC_106())
            .field("WRT_PROT_SEC_107", &self.WRT_PROT_SEC_107())
            .field("WRT_PROT_SEC_108", &self.WRT_PROT_SEC_108())
            .field("WRT_PROT_SEC_109", &self.WRT_PROT_SEC_109())
            .field("WRT_PROT_SEC_110", &self.WRT_PROT_SEC_110())
            .field("WRT_PROT_SEC_111", &self.WRT_PROT_SEC_111())
            .field("WRT_PROT_SEC_112", &self.WRT_PROT_SEC_112())
            .field("WRT_PROT_SEC_113", &self.WRT_PROT_SEC_113())
            .field("WRT_PROT_SEC_114", &self.WRT_PROT_SEC_114())
            .field("WRT_PROT_SEC_115", &self.WRT_PROT_SEC_115())
            .field("WRT_PROT_SEC_116", &self.WRT_PROT_SEC_116())
            .field("WRT_PROT_SEC_117", &self.WRT_PROT_SEC_117())
            .field("WRT_PROT_SEC_118", &self.WRT_PROT_SEC_118())
            .field("WRT_PROT_SEC_119", &self.WRT_PROT_SEC_119())
            .field("WRT_PROT_SEC_120", &self.WRT_PROT_SEC_120())
            .field("WRT_PROT_SEC_121", &self.WRT_PROT_SEC_121())
            .field("WRT_PROT_SEC_122", &self.WRT_PROT_SEC_122())
            .field("WRT_PROT_SEC_123", &self.WRT_PROT_SEC_123())
            .field("WRT_PROT_SEC_124", &self.WRT_PROT_SEC_124())
            .field("WRT_PROT_SEC_125", &self.WRT_PROT_SEC_125())
            .field("WRT_PROT_SEC_126", &self.WRT_PROT_SEC_126())
            .field("WRT_PROT_SEC_127", &self.WRT_PROT_SEC_127())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCFG_PROT_127_96 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCFG_PROT_127_96 {{ WRT_PROT_SEC_96: {=bool:?}, WRT_PROT_SEC_97: {=bool:?}, WRT_PROT_SEC_98: {=bool:?}, WRT_PROT_SEC_99: {=bool:?}, WRT_PROT_SEC_100: {=bool:?}, WRT_PROT_SEC_101: {=bool:?}, WRT_PROT_SEC_102: {=bool:?}, WRT_PROT_SEC_103: {=bool:?}, WRT_PROT_SEC_104: {=bool:?}, WRT_PROT_SEC_105: {=bool:?}, WRT_PROT_SEC_106: {=bool:?}, WRT_PROT_SEC_107: {=bool:?}, WRT_PROT_SEC_108: {=bool:?}, WRT_PROT_SEC_109: {=bool:?}, WRT_PROT_SEC_110: {=bool:?}, WRT_PROT_SEC_111: {=bool:?}, WRT_PROT_SEC_112: {=bool:?}, WRT_PROT_SEC_113: {=bool:?}, WRT_PROT_SEC_114: {=bool:?}, WRT_PROT_SEC_115: {=bool:?}, WRT_PROT_SEC_116: {=bool:?}, WRT_PROT_SEC_117: {=bool:?}, WRT_PROT_SEC_118: {=bool:?}, WRT_PROT_SEC_119: {=bool:?}, WRT_PROT_SEC_120: {=bool:?}, WRT_PROT_SEC_121: {=bool:?}, WRT_PROT_SEC_122: {=bool:?}, WRT_PROT_SEC_123: {=bool:?}, WRT_PROT_SEC_124: {=bool:?}, WRT_PROT_SEC_125: {=bool:?}, WRT_PROT_SEC_126: {=bool:?}, WRT_PROT_SEC_127: {=bool:?} }}",
            self.WRT_PROT_SEC_96(),
            self.WRT_PROT_SEC_97(),
            self.WRT_PROT_SEC_98(),
            self.WRT_PROT_SEC_99(),
            self.WRT_PROT_SEC_100(),
            self.WRT_PROT_SEC_101(),
            self.WRT_PROT_SEC_102(),
            self.WRT_PROT_SEC_103(),
            self.WRT_PROT_SEC_104(),
            self.WRT_PROT_SEC_105(),
            self.WRT_PROT_SEC_106(),
            self.WRT_PROT_SEC_107(),
            self.WRT_PROT_SEC_108(),
            self.WRT_PROT_SEC_109(),
            self.WRT_PROT_SEC_110(),
            self.WRT_PROT_SEC_111(),
            self.WRT_PROT_SEC_112(),
            self.WRT_PROT_SEC_113(),
            self.WRT_PROT_SEC_114(),
            self.WRT_PROT_SEC_115(),
            self.WRT_PROT_SEC_116(),
            self.WRT_PROT_SEC_117(),
            self.WRT_PROT_SEC_118(),
            self.WRT_PROT_SEC_119(),
            self.WRT_PROT_SEC_120(),
            self.WRT_PROT_SEC_121(),
            self.WRT_PROT_SEC_122(),
            self.WRT_PROT_SEC_123(),
            self.WRT_PROT_SEC_124(),
            self.WRT_PROT_SEC_125(),
            self.WRT_PROT_SEC_126(),
            self.WRT_PROT_SEC_127()
        )
    }
}
#[doc = "Protect Sectors 0-31 Each bit write protects one 4KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCFG_PROT_31_0(pub u32);
impl CCFG_PROT_31_0 {
    #[doc = "0:0\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CCFG_PROT_31_0 {
    #[inline(always)]
    fn default() -> CCFG_PROT_31_0 {
        CCFG_PROT_31_0(0)
    }
}
impl core::fmt::Debug for CCFG_PROT_31_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCFG_PROT_31_0")
            .field("WRT_PROT_SEC_0", &self.WRT_PROT_SEC_0())
            .field("WRT_PROT_SEC_1", &self.WRT_PROT_SEC_1())
            .field("WRT_PROT_SEC_2", &self.WRT_PROT_SEC_2())
            .field("WRT_PROT_SEC_3", &self.WRT_PROT_SEC_3())
            .field("WRT_PROT_SEC_4", &self.WRT_PROT_SEC_4())
            .field("WRT_PROT_SEC_5", &self.WRT_PROT_SEC_5())
            .field("WRT_PROT_SEC_6", &self.WRT_PROT_SEC_6())
            .field("WRT_PROT_SEC_7", &self.WRT_PROT_SEC_7())
            .field("WRT_PROT_SEC_8", &self.WRT_PROT_SEC_8())
            .field("WRT_PROT_SEC_9", &self.WRT_PROT_SEC_9())
            .field("WRT_PROT_SEC_10", &self.WRT_PROT_SEC_10())
            .field("WRT_PROT_SEC_11", &self.WRT_PROT_SEC_11())
            .field("WRT_PROT_SEC_12", &self.WRT_PROT_SEC_12())
            .field("WRT_PROT_SEC_13", &self.WRT_PROT_SEC_13())
            .field("WRT_PROT_SEC_14", &self.WRT_PROT_SEC_14())
            .field("WRT_PROT_SEC_15", &self.WRT_PROT_SEC_15())
            .field("WRT_PROT_SEC_16", &self.WRT_PROT_SEC_16())
            .field("WRT_PROT_SEC_17", &self.WRT_PROT_SEC_17())
            .field("WRT_PROT_SEC_18", &self.WRT_PROT_SEC_18())
            .field("WRT_PROT_SEC_19", &self.WRT_PROT_SEC_19())
            .field("WRT_PROT_SEC_20", &self.WRT_PROT_SEC_20())
            .field("WRT_PROT_SEC_21", &self.WRT_PROT_SEC_21())
            .field("WRT_PROT_SEC_22", &self.WRT_PROT_SEC_22())
            .field("WRT_PROT_SEC_23", &self.WRT_PROT_SEC_23())
            .field("WRT_PROT_SEC_24", &self.WRT_PROT_SEC_24())
            .field("WRT_PROT_SEC_25", &self.WRT_PROT_SEC_25())
            .field("WRT_PROT_SEC_26", &self.WRT_PROT_SEC_26())
            .field("WRT_PROT_SEC_27", &self.WRT_PROT_SEC_27())
            .field("WRT_PROT_SEC_28", &self.WRT_PROT_SEC_28())
            .field("WRT_PROT_SEC_29", &self.WRT_PROT_SEC_29())
            .field("WRT_PROT_SEC_30", &self.WRT_PROT_SEC_30())
            .field("WRT_PROT_SEC_31", &self.WRT_PROT_SEC_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCFG_PROT_31_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCFG_PROT_31_0 {{ WRT_PROT_SEC_0: {=bool:?}, WRT_PROT_SEC_1: {=bool:?}, WRT_PROT_SEC_2: {=bool:?}, WRT_PROT_SEC_3: {=bool:?}, WRT_PROT_SEC_4: {=bool:?}, WRT_PROT_SEC_5: {=bool:?}, WRT_PROT_SEC_6: {=bool:?}, WRT_PROT_SEC_7: {=bool:?}, WRT_PROT_SEC_8: {=bool:?}, WRT_PROT_SEC_9: {=bool:?}, WRT_PROT_SEC_10: {=bool:?}, WRT_PROT_SEC_11: {=bool:?}, WRT_PROT_SEC_12: {=bool:?}, WRT_PROT_SEC_13: {=bool:?}, WRT_PROT_SEC_14: {=bool:?}, WRT_PROT_SEC_15: {=bool:?}, WRT_PROT_SEC_16: {=bool:?}, WRT_PROT_SEC_17: {=bool:?}, WRT_PROT_SEC_18: {=bool:?}, WRT_PROT_SEC_19: {=bool:?}, WRT_PROT_SEC_20: {=bool:?}, WRT_PROT_SEC_21: {=bool:?}, WRT_PROT_SEC_22: {=bool:?}, WRT_PROT_SEC_23: {=bool:?}, WRT_PROT_SEC_24: {=bool:?}, WRT_PROT_SEC_25: {=bool:?}, WRT_PROT_SEC_26: {=bool:?}, WRT_PROT_SEC_27: {=bool:?}, WRT_PROT_SEC_28: {=bool:?}, WRT_PROT_SEC_29: {=bool:?}, WRT_PROT_SEC_30: {=bool:?}, WRT_PROT_SEC_31: {=bool:?} }}",
            self.WRT_PROT_SEC_0(),
            self.WRT_PROT_SEC_1(),
            self.WRT_PROT_SEC_2(),
            self.WRT_PROT_SEC_3(),
            self.WRT_PROT_SEC_4(),
            self.WRT_PROT_SEC_5(),
            self.WRT_PROT_SEC_6(),
            self.WRT_PROT_SEC_7(),
            self.WRT_PROT_SEC_8(),
            self.WRT_PROT_SEC_9(),
            self.WRT_PROT_SEC_10(),
            self.WRT_PROT_SEC_11(),
            self.WRT_PROT_SEC_12(),
            self.WRT_PROT_SEC_13(),
            self.WRT_PROT_SEC_14(),
            self.WRT_PROT_SEC_15(),
            self.WRT_PROT_SEC_16(),
            self.WRT_PROT_SEC_17(),
            self.WRT_PROT_SEC_18(),
            self.WRT_PROT_SEC_19(),
            self.WRT_PROT_SEC_20(),
            self.WRT_PROT_SEC_21(),
            self.WRT_PROT_SEC_22(),
            self.WRT_PROT_SEC_23(),
            self.WRT_PROT_SEC_24(),
            self.WRT_PROT_SEC_25(),
            self.WRT_PROT_SEC_26(),
            self.WRT_PROT_SEC_27(),
            self.WRT_PROT_SEC_28(),
            self.WRT_PROT_SEC_29(),
            self.WRT_PROT_SEC_30(),
            self.WRT_PROT_SEC_31()
        )
    }
}
#[doc = "Protect Sectors 32-63 Each bit write protects one 4KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCFG_PROT_63_32(pub u32);
impl CCFG_PROT_63_32 {
    #[doc = "0:0\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_40(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_40(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_41(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_41(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_42(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_42(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_43(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_43(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_44(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_44(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_45(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_46(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_46(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_47(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_47(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_48(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_48(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_49(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_49(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_50(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_50(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_51(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_51(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_52(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_52(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_53(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_53(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_54(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_54(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_55(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_55(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_56(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_56(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_57(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_57(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_58(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_58(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_59(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_59(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_60(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_60(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_61(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_61(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_62(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_62(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_63(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_63(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CCFG_PROT_63_32 {
    #[inline(always)]
    fn default() -> CCFG_PROT_63_32 {
        CCFG_PROT_63_32(0)
    }
}
impl core::fmt::Debug for CCFG_PROT_63_32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCFG_PROT_63_32")
            .field("WRT_PROT_SEC_32", &self.WRT_PROT_SEC_32())
            .field("WRT_PROT_SEC_33", &self.WRT_PROT_SEC_33())
            .field("WRT_PROT_SEC_34", &self.WRT_PROT_SEC_34())
            .field("WRT_PROT_SEC_35", &self.WRT_PROT_SEC_35())
            .field("WRT_PROT_SEC_36", &self.WRT_PROT_SEC_36())
            .field("WRT_PROT_SEC_37", &self.WRT_PROT_SEC_37())
            .field("WRT_PROT_SEC_38", &self.WRT_PROT_SEC_38())
            .field("WRT_PROT_SEC_39", &self.WRT_PROT_SEC_39())
            .field("WRT_PROT_SEC_40", &self.WRT_PROT_SEC_40())
            .field("WRT_PROT_SEC_41", &self.WRT_PROT_SEC_41())
            .field("WRT_PROT_SEC_42", &self.WRT_PROT_SEC_42())
            .field("WRT_PROT_SEC_43", &self.WRT_PROT_SEC_43())
            .field("WRT_PROT_SEC_44", &self.WRT_PROT_SEC_44())
            .field("WRT_PROT_SEC_45", &self.WRT_PROT_SEC_45())
            .field("WRT_PROT_SEC_46", &self.WRT_PROT_SEC_46())
            .field("WRT_PROT_SEC_47", &self.WRT_PROT_SEC_47())
            .field("WRT_PROT_SEC_48", &self.WRT_PROT_SEC_48())
            .field("WRT_PROT_SEC_49", &self.WRT_PROT_SEC_49())
            .field("WRT_PROT_SEC_50", &self.WRT_PROT_SEC_50())
            .field("WRT_PROT_SEC_51", &self.WRT_PROT_SEC_51())
            .field("WRT_PROT_SEC_52", &self.WRT_PROT_SEC_52())
            .field("WRT_PROT_SEC_53", &self.WRT_PROT_SEC_53())
            .field("WRT_PROT_SEC_54", &self.WRT_PROT_SEC_54())
            .field("WRT_PROT_SEC_55", &self.WRT_PROT_SEC_55())
            .field("WRT_PROT_SEC_56", &self.WRT_PROT_SEC_56())
            .field("WRT_PROT_SEC_57", &self.WRT_PROT_SEC_57())
            .field("WRT_PROT_SEC_58", &self.WRT_PROT_SEC_58())
            .field("WRT_PROT_SEC_59", &self.WRT_PROT_SEC_59())
            .field("WRT_PROT_SEC_60", &self.WRT_PROT_SEC_60())
            .field("WRT_PROT_SEC_61", &self.WRT_PROT_SEC_61())
            .field("WRT_PROT_SEC_62", &self.WRT_PROT_SEC_62())
            .field("WRT_PROT_SEC_63", &self.WRT_PROT_SEC_63())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCFG_PROT_63_32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCFG_PROT_63_32 {{ WRT_PROT_SEC_32: {=bool:?}, WRT_PROT_SEC_33: {=bool:?}, WRT_PROT_SEC_34: {=bool:?}, WRT_PROT_SEC_35: {=bool:?}, WRT_PROT_SEC_36: {=bool:?}, WRT_PROT_SEC_37: {=bool:?}, WRT_PROT_SEC_38: {=bool:?}, WRT_PROT_SEC_39: {=bool:?}, WRT_PROT_SEC_40: {=bool:?}, WRT_PROT_SEC_41: {=bool:?}, WRT_PROT_SEC_42: {=bool:?}, WRT_PROT_SEC_43: {=bool:?}, WRT_PROT_SEC_44: {=bool:?}, WRT_PROT_SEC_45: {=bool:?}, WRT_PROT_SEC_46: {=bool:?}, WRT_PROT_SEC_47: {=bool:?}, WRT_PROT_SEC_48: {=bool:?}, WRT_PROT_SEC_49: {=bool:?}, WRT_PROT_SEC_50: {=bool:?}, WRT_PROT_SEC_51: {=bool:?}, WRT_PROT_SEC_52: {=bool:?}, WRT_PROT_SEC_53: {=bool:?}, WRT_PROT_SEC_54: {=bool:?}, WRT_PROT_SEC_55: {=bool:?}, WRT_PROT_SEC_56: {=bool:?}, WRT_PROT_SEC_57: {=bool:?}, WRT_PROT_SEC_58: {=bool:?}, WRT_PROT_SEC_59: {=bool:?}, WRT_PROT_SEC_60: {=bool:?}, WRT_PROT_SEC_61: {=bool:?}, WRT_PROT_SEC_62: {=bool:?}, WRT_PROT_SEC_63: {=bool:?} }}",
            self.WRT_PROT_SEC_32(),
            self.WRT_PROT_SEC_33(),
            self.WRT_PROT_SEC_34(),
            self.WRT_PROT_SEC_35(),
            self.WRT_PROT_SEC_36(),
            self.WRT_PROT_SEC_37(),
            self.WRT_PROT_SEC_38(),
            self.WRT_PROT_SEC_39(),
            self.WRT_PROT_SEC_40(),
            self.WRT_PROT_SEC_41(),
            self.WRT_PROT_SEC_42(),
            self.WRT_PROT_SEC_43(),
            self.WRT_PROT_SEC_44(),
            self.WRT_PROT_SEC_45(),
            self.WRT_PROT_SEC_46(),
            self.WRT_PROT_SEC_47(),
            self.WRT_PROT_SEC_48(),
            self.WRT_PROT_SEC_49(),
            self.WRT_PROT_SEC_50(),
            self.WRT_PROT_SEC_51(),
            self.WRT_PROT_SEC_52(),
            self.WRT_PROT_SEC_53(),
            self.WRT_PROT_SEC_54(),
            self.WRT_PROT_SEC_55(),
            self.WRT_PROT_SEC_56(),
            self.WRT_PROT_SEC_57(),
            self.WRT_PROT_SEC_58(),
            self.WRT_PROT_SEC_59(),
            self.WRT_PROT_SEC_60(),
            self.WRT_PROT_SEC_61(),
            self.WRT_PROT_SEC_62(),
            self.WRT_PROT_SEC_63()
        )
    }
}
#[doc = "Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCFG_PROT_95_64(pub u32);
impl CCFG_PROT_95_64 {
    #[doc = "0:0\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_64(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_64(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_65(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_65(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_66(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_66(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_67(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_67(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_68(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_68(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_69(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_69(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_70(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_70(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_71(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_71(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_72(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_72(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_73(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_73(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_74(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_74(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_75(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_75(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_76(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_76(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_77(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_77(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_78(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_78(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_79(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_79(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_80(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_80(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_81(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_81(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_82(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_82(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_83(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_83(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_84(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_84(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_85(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_85(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_86(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_86(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_87(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_87(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_88(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_88(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_89(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_89(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_90(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_90(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_91(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_91(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_92(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_92(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_93(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_93(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_94(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_94(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] 0: Sector protected."]
    #[must_use]
    #[inline(always)]
    pub const fn WRT_PROT_SEC_95(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] 0: Sector protected."]
    #[inline(always)]
    pub const fn set_WRT_PROT_SEC_95(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CCFG_PROT_95_64 {
    #[inline(always)]
    fn default() -> CCFG_PROT_95_64 {
        CCFG_PROT_95_64(0)
    }
}
impl core::fmt::Debug for CCFG_PROT_95_64 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCFG_PROT_95_64")
            .field("WRT_PROT_SEC_64", &self.WRT_PROT_SEC_64())
            .field("WRT_PROT_SEC_65", &self.WRT_PROT_SEC_65())
            .field("WRT_PROT_SEC_66", &self.WRT_PROT_SEC_66())
            .field("WRT_PROT_SEC_67", &self.WRT_PROT_SEC_67())
            .field("WRT_PROT_SEC_68", &self.WRT_PROT_SEC_68())
            .field("WRT_PROT_SEC_69", &self.WRT_PROT_SEC_69())
            .field("WRT_PROT_SEC_70", &self.WRT_PROT_SEC_70())
            .field("WRT_PROT_SEC_71", &self.WRT_PROT_SEC_71())
            .field("WRT_PROT_SEC_72", &self.WRT_PROT_SEC_72())
            .field("WRT_PROT_SEC_73", &self.WRT_PROT_SEC_73())
            .field("WRT_PROT_SEC_74", &self.WRT_PROT_SEC_74())
            .field("WRT_PROT_SEC_75", &self.WRT_PROT_SEC_75())
            .field("WRT_PROT_SEC_76", &self.WRT_PROT_SEC_76())
            .field("WRT_PROT_SEC_77", &self.WRT_PROT_SEC_77())
            .field("WRT_PROT_SEC_78", &self.WRT_PROT_SEC_78())
            .field("WRT_PROT_SEC_79", &self.WRT_PROT_SEC_79())
            .field("WRT_PROT_SEC_80", &self.WRT_PROT_SEC_80())
            .field("WRT_PROT_SEC_81", &self.WRT_PROT_SEC_81())
            .field("WRT_PROT_SEC_82", &self.WRT_PROT_SEC_82())
            .field("WRT_PROT_SEC_83", &self.WRT_PROT_SEC_83())
            .field("WRT_PROT_SEC_84", &self.WRT_PROT_SEC_84())
            .field("WRT_PROT_SEC_85", &self.WRT_PROT_SEC_85())
            .field("WRT_PROT_SEC_86", &self.WRT_PROT_SEC_86())
            .field("WRT_PROT_SEC_87", &self.WRT_PROT_SEC_87())
            .field("WRT_PROT_SEC_88", &self.WRT_PROT_SEC_88())
            .field("WRT_PROT_SEC_89", &self.WRT_PROT_SEC_89())
            .field("WRT_PROT_SEC_90", &self.WRT_PROT_SEC_90())
            .field("WRT_PROT_SEC_91", &self.WRT_PROT_SEC_91())
            .field("WRT_PROT_SEC_92", &self.WRT_PROT_SEC_92())
            .field("WRT_PROT_SEC_93", &self.WRT_PROT_SEC_93())
            .field("WRT_PROT_SEC_94", &self.WRT_PROT_SEC_94())
            .field("WRT_PROT_SEC_95", &self.WRT_PROT_SEC_95())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCFG_PROT_95_64 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCFG_PROT_95_64 {{ WRT_PROT_SEC_64: {=bool:?}, WRT_PROT_SEC_65: {=bool:?}, WRT_PROT_SEC_66: {=bool:?}, WRT_PROT_SEC_67: {=bool:?}, WRT_PROT_SEC_68: {=bool:?}, WRT_PROT_SEC_69: {=bool:?}, WRT_PROT_SEC_70: {=bool:?}, WRT_PROT_SEC_71: {=bool:?}, WRT_PROT_SEC_72: {=bool:?}, WRT_PROT_SEC_73: {=bool:?}, WRT_PROT_SEC_74: {=bool:?}, WRT_PROT_SEC_75: {=bool:?}, WRT_PROT_SEC_76: {=bool:?}, WRT_PROT_SEC_77: {=bool:?}, WRT_PROT_SEC_78: {=bool:?}, WRT_PROT_SEC_79: {=bool:?}, WRT_PROT_SEC_80: {=bool:?}, WRT_PROT_SEC_81: {=bool:?}, WRT_PROT_SEC_82: {=bool:?}, WRT_PROT_SEC_83: {=bool:?}, WRT_PROT_SEC_84: {=bool:?}, WRT_PROT_SEC_85: {=bool:?}, WRT_PROT_SEC_86: {=bool:?}, WRT_PROT_SEC_87: {=bool:?}, WRT_PROT_SEC_88: {=bool:?}, WRT_PROT_SEC_89: {=bool:?}, WRT_PROT_SEC_90: {=bool:?}, WRT_PROT_SEC_91: {=bool:?}, WRT_PROT_SEC_92: {=bool:?}, WRT_PROT_SEC_93: {=bool:?}, WRT_PROT_SEC_94: {=bool:?}, WRT_PROT_SEC_95: {=bool:?} }}",
            self.WRT_PROT_SEC_64(),
            self.WRT_PROT_SEC_65(),
            self.WRT_PROT_SEC_66(),
            self.WRT_PROT_SEC_67(),
            self.WRT_PROT_SEC_68(),
            self.WRT_PROT_SEC_69(),
            self.WRT_PROT_SEC_70(),
            self.WRT_PROT_SEC_71(),
            self.WRT_PROT_SEC_72(),
            self.WRT_PROT_SEC_73(),
            self.WRT_PROT_SEC_74(),
            self.WRT_PROT_SEC_75(),
            self.WRT_PROT_SEC_76(),
            self.WRT_PROT_SEC_77(),
            self.WRT_PROT_SEC_78(),
            self.WRT_PROT_SEC_79(),
            self.WRT_PROT_SEC_80(),
            self.WRT_PROT_SEC_81(),
            self.WRT_PROT_SEC_82(),
            self.WRT_PROT_SEC_83(),
            self.WRT_PROT_SEC_84(),
            self.WRT_PROT_SEC_85(),
            self.WRT_PROT_SEC_86(),
            self.WRT_PROT_SEC_87(),
            self.WRT_PROT_SEC_88(),
            self.WRT_PROT_SEC_89(),
            self.WRT_PROT_SEC_90(),
            self.WRT_PROT_SEC_91(),
            self.WRT_PROT_SEC_92(),
            self.WRT_PROT_SEC_93(),
            self.WRT_PROT_SEC_94(),
            self.WRT_PROT_SEC_95()
        )
    }
}
#[doc = "Test Access Points Enable 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCFG_TAP_DAP_0(pub u32);
impl CCFG_TAP_DAP_0 {
    #[doc = "7:0\\] Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
    #[must_use]
    #[inline(always)]
    pub const fn TEST_TAP_ENABLE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub const fn set_TEST_TAP_ENABLE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Enable PRCM TAP. 0xC5: PRCM TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PRCM TAP access will remain disabled out of power-up/system-reset."]
    #[must_use]
    #[inline(always)]
    pub const fn PRCM_TAP_ENABLE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Enable PRCM TAP. 0xC5: PRCM TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PRCM TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub const fn set_PRCM_TAP_ENABLE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU_DAP_ENABLE(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub const fn set_CPU_DAP_ENABLE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for CCFG_TAP_DAP_0 {
    #[inline(always)]
    fn default() -> CCFG_TAP_DAP_0 {
        CCFG_TAP_DAP_0(0)
    }
}
impl core::fmt::Debug for CCFG_TAP_DAP_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCFG_TAP_DAP_0")
            .field("TEST_TAP_ENABLE", &self.TEST_TAP_ENABLE())
            .field("PRCM_TAP_ENABLE", &self.PRCM_TAP_ENABLE())
            .field("CPU_DAP_ENABLE", &self.CPU_DAP_ENABLE())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCFG_TAP_DAP_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCFG_TAP_DAP_0 {{ TEST_TAP_ENABLE: {=u8:?}, PRCM_TAP_ENABLE: {=u8:?}, CPU_DAP_ENABLE: {=u8:?}, RESERVED: {=u8:?} }}",
            self.TEST_TAP_ENABLE(),
            self.PRCM_TAP_ENABLE(),
            self.CPU_DAP_ENABLE(),
            self.RESERVED()
        )
    }
}
#[doc = "Test Access Points Enable 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCFG_TAP_DAP_1(pub u32);
impl CCFG_TAP_DAP_1 {
    #[doc = "7:0\\] Enable WUC TAP 0xC5: WUC TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: WUC TAP access will remain disabled out of power-up/system-reset."]
    #[must_use]
    #[inline(always)]
    pub const fn WUC_TAP_ENABLE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Enable WUC TAP 0xC5: WUC TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: WUC TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub const fn set_WUC_TAP_ENABLE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Enable PBIST1 TAP. 0xC5: PBIST1 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST1 TAP access will remain disabled out of power-up/system-reset."]
    #[must_use]
    #[inline(always)]
    pub const fn PBIST1_TAP_ENABLE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Enable PBIST1 TAP. 0xC5: PBIST1 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST1 TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub const fn set_PBIST1_TAP_ENABLE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Enable PBIST2 TAP. 0xC5: PBIST2 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST2 TAP access will remain disabled out of power-up/system-reset."]
    #[must_use]
    #[inline(always)]
    pub const fn PBIST2_TAP_ENABLE(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Enable PBIST2 TAP. 0xC5: PBIST2 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST2 TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub const fn set_PBIST2_TAP_ENABLE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for CCFG_TAP_DAP_1 {
    #[inline(always)]
    fn default() -> CCFG_TAP_DAP_1 {
        CCFG_TAP_DAP_1(0)
    }
}
impl core::fmt::Debug for CCFG_TAP_DAP_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCFG_TAP_DAP_1")
            .field("WUC_TAP_ENABLE", &self.WUC_TAP_ENABLE())
            .field("PBIST1_TAP_ENABLE", &self.PBIST1_TAP_ENABLE())
            .field("PBIST2_TAP_ENABLE", &self.PBIST2_TAP_ENABLE())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCFG_TAP_DAP_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCFG_TAP_DAP_1 {{ WUC_TAP_ENABLE: {=u8:?}, PBIST1_TAP_ENABLE: {=u8:?}, PBIST2_TAP_ENABLE: {=u8:?}, RESERVED: {=u8:?} }}",
            self.WUC_TAP_ENABLE(),
            self.PBIST1_TAP_ENABLE(),
            self.PBIST2_TAP_ENABLE(),
            self.RESERVED()
        )
    }
}
#[doc = "TI Options."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCFG_TI_OPTIONS(pub u32);
impl CCFG_TI_OPTIONS {
    #[doc = "7:0\\] TI Failure Analysis. 0xC5: Enable the functionality of unlocking the TI FA (TI Failure Analysis) option with the unlock code. All other values: Disable the functionality of unlocking the TI FA option with the unlock code."]
    #[must_use]
    #[inline(always)]
    pub const fn TI_FA_ENABLE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] TI Failure Analysis. 0xC5: Enable the functionality of unlocking the TI FA (TI Failure Analysis) option with the unlock code. All other values: Disable the functionality of unlocking the TI FA option with the unlock code."]
    #[inline(always)]
    pub const fn set_TI_FA_ENABLE(&mut self, val: u8) {
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
impl Default for CCFG_TI_OPTIONS {
    #[inline(always)]
    fn default() -> CCFG_TI_OPTIONS {
        CCFG_TI_OPTIONS(0)
    }
}
impl core::fmt::Debug for CCFG_TI_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCFG_TI_OPTIONS")
            .field("TI_FA_ENABLE", &self.TI_FA_ENABLE())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCFG_TI_OPTIONS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCFG_TI_OPTIONS {{ TI_FA_ENABLE: {=u8:?}, RESERVED: {=u32:?} }}",
            self.TI_FA_ENABLE(),
            self.RESERVED()
        )
    }
}
#[doc = "Erase Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ERASE_CONF(pub u32);
impl ERASE_CONF {
    #[doc = "0:0\\] Bank erase. This bit controls if the ROM serial boot loader will accept a received Bank Erase command (COMMAND_BANK_ERASE). A successful Bank Erase operation will erase all main bank sectors not protected by write protect configuration bits in CCFG. 0: Disable the boot loader bank erase function. 1: Enable the boot loader bank erase function."]
    #[must_use]
    #[inline(always)]
    pub const fn BANK_ERASE_DIS_N(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Bank erase. This bit controls if the ROM serial boot loader will accept a received Bank Erase command (COMMAND_BANK_ERASE). A successful Bank Erase operation will erase all main bank sectors not protected by write protect configuration bits in CCFG. 0: Disable the boot loader bank erase function. 1: Enable the boot loader bank erase function."]
    #[inline(always)]
    pub const fn set_BANK_ERASE_DIS_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "8:8\\] Chip erase. This bit controls if a chip erase requested through the JTAG WUC TAP will be ignored in a following boot caused by a reset of the MCU VD. A successful chip erase operation will force the content of the flash main bank back to the state as it was when delivered by TI. 0: Disable. Any chip erase request detected during boot will be ignored. 1: Enable. Any chip erase request detected during boot will be performed by the boot FW."]
    #[must_use]
    #[inline(always)]
    pub const fn CHIP_ERASE_DIS_N(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Chip erase. This bit controls if a chip erase requested through the JTAG WUC TAP will be ignored in a following boot caused by a reset of the MCU VD. A successful chip erase operation will force the content of the flash main bank back to the state as it was when delivered by TI. 0: Disable. Any chip erase request detected during boot will be ignored. 1: Enable. Any chip erase request detected during boot will be performed by the boot FW."]
    #[inline(always)]
    pub const fn set_CHIP_ERASE_DIS_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "31:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "31:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for ERASE_CONF {
    #[inline(always)]
    fn default() -> ERASE_CONF {
        ERASE_CONF(0)
    }
}
impl core::fmt::Debug for ERASE_CONF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERASE_CONF")
            .field("BANK_ERASE_DIS_N", &self.BANK_ERASE_DIS_N())
            .field("RESERVED1", &self.RESERVED1())
            .field("CHIP_ERASE_DIS_N", &self.CHIP_ERASE_DIS_N())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ERASE_CONF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ERASE_CONF {{ BANK_ERASE_DIS_N: {=bool:?}, RESERVED1: {=u8:?}, CHIP_ERASE_DIS_N: {=bool:?}, RESERVED2: {=u32:?} }}",
            self.BANK_ERASE_DIS_N(),
            self.RESERVED1(),
            self.CHIP_ERASE_DIS_N(),
            self.RESERVED2()
        )
    }
}
#[doc = "Extern LF clock configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EXT_LF_CLK(pub u32);
impl EXT_LF_CLK {
    #[doc = "23:0\\] Unsigned integer, defining the input frequency of the external clock and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows: EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_INCREMENT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Unsigned integer, defining the input frequency of the external clock and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows: EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)."]
    #[inline(always)]
    pub const fn set_RTC_INCREMENT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "31:24\\] Unsigned integer, selecting the DIO to supply external 32kHz clock as SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL. The selected DIO will be marked as reserved by the pin driver (TI-RTOS environment) and hence not selectable for other usage."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Unsigned integer, selecting the DIO to supply external 32kHz clock as SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL. The selected DIO will be marked as reserved by the pin driver (TI-RTOS environment) and hence not selectable for other usage."]
    #[inline(always)]
    pub const fn set_DIO(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for EXT_LF_CLK {
    #[inline(always)]
    fn default() -> EXT_LF_CLK {
        EXT_LF_CLK(0)
    }
}
impl core::fmt::Debug for EXT_LF_CLK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_LF_CLK")
            .field("RTC_INCREMENT", &self.RTC_INCREMENT())
            .field("DIO", &self.DIO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EXT_LF_CLK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EXT_LF_CLK {{ RTC_INCREMENT: {=u32:?}, DIO: {=u8:?} }}",
            self.RTC_INCREMENT(),
            self.DIO()
        )
    }
}
#[doc = "Frequency Offset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FREQ_OFFSET(pub u32);
impl FREQ_OFFSET {
    #[doc = "7:0\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn HF_COMP_P2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_HF_COMP_P2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn HF_COMP_P1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_HF_COMP_P1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "31:16\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn HF_COMP_P0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_HF_COMP_P0(&mut self, val: u16) {
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
            .field("HF_COMP_P2", &self.HF_COMP_P2())
            .field("HF_COMP_P1", &self.HF_COMP_P1())
            .field("HF_COMP_P0", &self.HF_COMP_P0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FREQ_OFFSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FREQ_OFFSET {{ HF_COMP_P2: {=u8:?}, HF_COMP_P1: {=u8:?}, HF_COMP_P0: {=u16:?} }}",
            self.HF_COMP_P2(),
            self.HF_COMP_P1(),
            self.HF_COMP_P0()
        )
    }
}
#[doc = "IEEE BLE Address 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IEEE_BLE_0(pub u32);
impl IEEE_BLE_0 {
    #[doc = "31:0\\] Bits\\[31:0\\] of the 64-bits custom IEEE BLE address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Bits\\[31:0\\] of the 64-bits custom IEEE BLE address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[inline(always)]
    pub const fn set_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IEEE_BLE_0 {
    #[inline(always)]
    fn default() -> IEEE_BLE_0 {
        IEEE_BLE_0(0)
    }
}
impl core::fmt::Debug for IEEE_BLE_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEEE_BLE_0")
            .field("ADDR", &self.ADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IEEE_BLE_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IEEE_BLE_0 {{ ADDR: {=u32:?} }}", self.ADDR())
    }
}
#[doc = "IEEE BLE Address 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IEEE_BLE_1(pub u32);
impl IEEE_BLE_1 {
    #[doc = "31:0\\] Bits\\[63:32\\] of the 64-bits custom IEEE BLE address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Bits\\[63:32\\] of the 64-bits custom IEEE BLE address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[inline(always)]
    pub const fn set_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IEEE_BLE_1 {
    #[inline(always)]
    fn default() -> IEEE_BLE_1 {
        IEEE_BLE_1(0)
    }
}
impl core::fmt::Debug for IEEE_BLE_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEEE_BLE_1")
            .field("ADDR", &self.ADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IEEE_BLE_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IEEE_BLE_1 {{ ADDR: {=u32:?} }}", self.ADDR())
    }
}
#[doc = "IEEE MAC Address 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IEEE_MAC_0(pub u32);
impl IEEE_MAC_0 {
    #[doc = "31:0\\] Bits\\[31:0\\] of the 64-bits custom IEEE MAC address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Bits\\[31:0\\] of the 64-bits custom IEEE MAC address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[inline(always)]
    pub const fn set_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IEEE_MAC_0 {
    #[inline(always)]
    fn default() -> IEEE_MAC_0 {
        IEEE_MAC_0(0)
    }
}
impl core::fmt::Debug for IEEE_MAC_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEEE_MAC_0")
            .field("ADDR", &self.ADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IEEE_MAC_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IEEE_MAC_0 {{ ADDR: {=u32:?} }}", self.ADDR())
    }
}
#[doc = "IEEE MAC Address 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IEEE_MAC_1(pub u32);
impl IEEE_MAC_1 {
    #[doc = "31:0\\] Bits\\[63:32\\] of the 64-bits custom IEEE MAC address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Bits\\[63:32\\] of the 64-bits custom IEEE MAC address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[inline(always)]
    pub const fn set_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IEEE_MAC_1 {
    #[inline(always)]
    fn default() -> IEEE_MAC_1 {
        IEEE_MAC_1(0)
    }
}
impl core::fmt::Debug for IEEE_MAC_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEEE_MAC_1")
            .field("ADDR", &self.ADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IEEE_MAC_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IEEE_MAC_1 {{ ADDR: {=u32:?} }}", self.ADDR())
    }
}
#[doc = "Image Valid."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IMAGE_VALID_CONF(pub u32);
impl IMAGE_VALID_CONF {
    #[doc = "31:0\\] This field must have a value of 0x00000000 in order for enabling the boot sequence to transfer control to a flash image. A non-zero value forces the boot sequence to call the boot loader. For CC2640R2: This field must have the address value of the start of the flash vector table in order for enabling the boot sequence to transfer control to a flash image. Any illegal vector table start address value forces the boot sequence to call the boot loader. Note that if any other legal vector table start address value than 0x0 is selected the PRCM:WARMRESET.WR_TO_PINRESET must be set to 1."]
    #[must_use]
    #[inline(always)]
    pub const fn IMAGE_VALID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] This field must have a value of 0x00000000 in order for enabling the boot sequence to transfer control to a flash image. A non-zero value forces the boot sequence to call the boot loader. For CC2640R2: This field must have the address value of the start of the flash vector table in order for enabling the boot sequence to transfer control to a flash image. Any illegal vector table start address value forces the boot sequence to call the boot loader. Note that if any other legal vector table start address value than 0x0 is selected the PRCM:WARMRESET.WR_TO_PINRESET must be set to 1."]
    #[inline(always)]
    pub const fn set_IMAGE_VALID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IMAGE_VALID_CONF {
    #[inline(always)]
    fn default() -> IMAGE_VALID_CONF {
        IMAGE_VALID_CONF(0)
    }
}
impl core::fmt::Debug for IMAGE_VALID_CONF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMAGE_VALID_CONF")
            .field("IMAGE_VALID", &self.IMAGE_VALID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IMAGE_VALID_CONF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IMAGE_VALID_CONF {{ IMAGE_VALID: {=u32:?} }}",
            self.IMAGE_VALID()
        )
    }
}
#[doc = "Mode Configuration 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MODE_CONF(pub u32);
impl MODE_CONF {
    #[doc = "7:0\\] Unsigned 8-bit integer, representing the minimum decoupling capacitance (worst case) on VDDR, in units of 100nF. This should take into account capacitor tolerance and voltage dependent capacitance variation. This bit affects the recharge period calculation when going into powerdown or standby. NOTE! If using the following functions this field must be configured (used by TI RTOS): SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_CAP(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Unsigned 8-bit integer, representing the minimum decoupling capacitance (worst case) on VDDR, in units of 100nF. This should take into account capacitor tolerance and voltage dependent capacitance variation. This bit affects the recharge period calculation when going into powerdown or standby. NOTE! If using the following functions this field must be configured (used by TI RTOS): SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()."]
    #[inline(always)]
    pub const fn set_VDDR_CAP(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Signed 8-bit value, directly modifying trimmed XOSC cap-array step value. Enabled by XOSC_CAP_MOD."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_CAPARRAY_DELTA(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Signed 8-bit value, directly modifying trimmed XOSC cap-array step value. Enabled by XOSC_CAP_MOD."]
    #[inline(always)]
    pub const fn set_XOSC_CAPARRAY_DELTA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "16:16\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn HF_COMP(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_HF_COMP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Enable modification (delta) to XOSC cap-array. Value specified in XOSC_CAPARRAY_DELTA. 0: Apply cap-array delta 1: Do not apply cap-array delta (default)."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_CAP_MOD(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Enable modification (delta) to XOSC cap-array. Value specified in XOSC_CAPARRAY_DELTA. 0: Apply cap-array delta 1: Do not apply cap-array delta (default)."]
    #[inline(always)]
    pub const fn set_XOSC_CAP_MOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "19:18\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_FREQ(&self) -> super::vals::XOSC_FREQ {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::XOSC_FREQ::from_bits(val as u8)
    }
    #[doc = "19:18\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_XOSC_FREQ(&mut self, val: super::vals::XOSC_FREQ) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "20:20\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_COMP(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RTC_COMP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] 0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time standby mode is entered. This improves low-temperature RCOSC_LF frequency stability in standby mode. When temperature compensation is performed, the delta is calculates this way: Delta = max (delta, min(8, floor(62-temp)/8)) Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current temperature in degrees C."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_TRIM_SLEEP_TC(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] 0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time standby mode is entered. This improves low-temperature RCOSC_LF frequency stability in standby mode. When temperature compensation is performed, the delta is calculates this way: Delta = max (delta, min(8, floor(62-temp)/8)) Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current temperature in degrees C."]
    #[inline(always)]
    pub const fn set_VDDR_TRIM_SLEEP_TC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "23:22\\] Select source for SCLK_LF."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLK_LF_OPTION(&self) -> super::vals::SCLK_LF_OPTION {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::SCLK_LF_OPTION::from_bits(val as u8)
    }
    #[doc = "23:22\\] Select source for SCLK_LF."]
    #[inline(always)]
    pub const fn set_SCLK_LF_OPTION(&mut self, val: super::vals::SCLK_LF_OPTION) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "24:24\\] VDDS BOD level. 0: VDDS BOD level is 2.0 V (necessary for maximum PA output power on CC13x0). 1: VDDS BOD level is 1.8 V (or 1.7 V for external regulator mode) (default)."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDS_BOD_LEVEL(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] VDDS BOD level. 0: VDDS BOD level is 2.0 V (necessary for maximum PA output power on CC13x0). 1: VDDS BOD level is 1.8 V (or 1.7 V for external regulator mode) (default)."]
    #[inline(always)]
    pub const fn set_VDDS_BOD_LEVEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_EXT_LOAD(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_VDDR_EXT_LOAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] DC/DC in active mode. 0: Use the DC/DC during active mode. 1: Do not use the DC/DC during active mode (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDC_ACTIVE(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] DC/DC in active mode. 0: Use the DC/DC during active mode. 1: Do not use the DC/DC during active mode (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub const fn set_DCDC_ACTIVE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] DC/DC during recharge in powerdown. 0: Use the DC/DC during recharge in powerdown. 1: Do not use the DC/DC during recharge in powerdown (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDC_RECHARGE(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] DC/DC during recharge in powerdown. 0: Use the DC/DC during recharge in powerdown. 1: Do not use the DC/DC during recharge in powerdown (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub const fn set_DCDC_RECHARGE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "31:28\\] Signed delta value to apply to the VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H. 0x8 (-8) : Delta = -7 ... 0xF (-1) : Delta = 0 0x0 (0) : Delta = +1 ... 0x7 (7) : Delta = +8."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_TRIM_SLEEP_DELTA(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Signed delta value to apply to the VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H. 0x8 (-8) : Delta = -7 ... 0xF (-1) : Delta = 0 0x0 (0) : Delta = +1 ... 0x7 (7) : Delta = +8."]
    #[inline(always)]
    pub const fn set_VDDR_TRIM_SLEEP_DELTA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for MODE_CONF {
    #[inline(always)]
    fn default() -> MODE_CONF {
        MODE_CONF(0)
    }
}
impl core::fmt::Debug for MODE_CONF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODE_CONF")
            .field("VDDR_CAP", &self.VDDR_CAP())
            .field("XOSC_CAPARRAY_DELTA", &self.XOSC_CAPARRAY_DELTA())
            .field("HF_COMP", &self.HF_COMP())
            .field("XOSC_CAP_MOD", &self.XOSC_CAP_MOD())
            .field("XOSC_FREQ", &self.XOSC_FREQ())
            .field("RTC_COMP", &self.RTC_COMP())
            .field("VDDR_TRIM_SLEEP_TC", &self.VDDR_TRIM_SLEEP_TC())
            .field("SCLK_LF_OPTION", &self.SCLK_LF_OPTION())
            .field("VDDS_BOD_LEVEL", &self.VDDS_BOD_LEVEL())
            .field("VDDR_EXT_LOAD", &self.VDDR_EXT_LOAD())
            .field("DCDC_ACTIVE", &self.DCDC_ACTIVE())
            .field("DCDC_RECHARGE", &self.DCDC_RECHARGE())
            .field("VDDR_TRIM_SLEEP_DELTA", &self.VDDR_TRIM_SLEEP_DELTA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MODE_CONF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MODE_CONF {{ VDDR_CAP: {=u8:?}, XOSC_CAPARRAY_DELTA: {=u8:?}, HF_COMP: {=bool:?}, XOSC_CAP_MOD: {=bool:?}, XOSC_FREQ: {:?}, RTC_COMP: {=bool:?}, VDDR_TRIM_SLEEP_TC: {=bool:?}, SCLK_LF_OPTION: {:?}, VDDS_BOD_LEVEL: {=bool:?}, VDDR_EXT_LOAD: {=bool:?}, DCDC_ACTIVE: {=bool:?}, DCDC_RECHARGE: {=bool:?}, VDDR_TRIM_SLEEP_DELTA: {=u8:?} }}",
            self.VDDR_CAP(),
            self.XOSC_CAPARRAY_DELTA(),
            self.HF_COMP(),
            self.XOSC_CAP_MOD(),
            self.XOSC_FREQ(),
            self.RTC_COMP(),
            self.VDDR_TRIM_SLEEP_TC(),
            self.SCLK_LF_OPTION(),
            self.VDDS_BOD_LEVEL(),
            self.VDDR_EXT_LOAD(),
            self.DCDC_ACTIVE(),
            self.DCDC_RECHARGE(),
            self.VDDR_TRIM_SLEEP_DELTA()
        )
    }
}
#[doc = "Mode Configuration 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MODE_CONF_1(pub u32);
impl MODE_CONF_1 {
    #[doc = "7:0\\] Unsigned value of maximum XOSC startup time (worst case) in units of 100us. Value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0."]
    #[must_use]
    #[inline(always)]
    pub const fn XOSC_MAX_START(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Unsigned value of maximum XOSC startup time (worst case) in units of 100us. Value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0."]
    #[inline(always)]
    pub const fn set_XOSC_MAX_START(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "11:8\\] Signed delta value for IBIAS_OFFSET. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_OFFSET."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_IBIAS_OFFSET(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "11:8\\] Signed delta value for IBIAS_OFFSET. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_OFFSET."]
    #[inline(always)]
    pub const fn set_DELTA_IBIAS_OFFSET(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "15:12\\] Signed delta value for IBIAS_INIT. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_INIT."]
    #[must_use]
    #[inline(always)]
    pub const fn DELTA_IBIAS_INIT(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "15:12\\] Signed delta value for IBIAS_INIT. Delta value only applies if SIZE_AND_DIS_FLAGS.DIS_XOSC_OVR=0. See FCFG1:AMPCOMP_CTRL1.IBIAS_INIT."]
    #[inline(always)]
    pub const fn set_DELTA_IBIAS_INIT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "18:16\\] Inductor peak current if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Assuming 10uH external inductor! Peak current = 31 + ( 4 * ALT_DCDC_IPEAK ) : 0: 31mA (min) ... 4: 47mA ... 7: 59mA (max)."]
    #[must_use]
    #[inline(always)]
    pub const fn ALT_DCDC_IPEAK(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "18:16\\] Inductor peak current if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Assuming 10uH external inductor! Peak current = 31 + ( 4 * ALT_DCDC_IPEAK ) : 0: 31mA (min) ... 4: 47mA ... 7: 59mA (max)."]
    #[inline(always)]
    pub const fn set_ALT_DCDC_IPEAK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "19:19\\] Enable DC/DC dithering if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). 0: Dither disable 1: Dither enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ALT_DCDC_DITHER_EN(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Enable DC/DC dithering if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). 0: Dither disable 1: Dither enable."]
    #[inline(always)]
    pub const fn set_ALT_DCDC_DITHER_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "23:20\\] Minimum voltage for when DC/DC should be used if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Voltage = (28 + ALT_DCDC_VMIN) / 16. 0: 1.75V 1: 1.8125V ... 14: 2.625V 15: 2.6875V NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[must_use]
    #[inline(always)]
    pub const fn ALT_DCDC_VMIN(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Minimum voltage for when DC/DC should be used if alternate DC/DC setting is enabled (SIZE_AND_DIS_FLAGS.DIS_ALT_DCDC_SETTING=0). Voltage = (28 + ALT_DCDC_VMIN) / 16. 0: 1.75V 1: 1.8125V ... 14: 2.625V 15: 2.6875V NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub const fn set_ALT_DCDC_VMIN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for MODE_CONF_1 {
    #[inline(always)]
    fn default() -> MODE_CONF_1 {
        MODE_CONF_1(0)
    }
}
impl core::fmt::Debug for MODE_CONF_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODE_CONF_1")
            .field("XOSC_MAX_START", &self.XOSC_MAX_START())
            .field("DELTA_IBIAS_OFFSET", &self.DELTA_IBIAS_OFFSET())
            .field("DELTA_IBIAS_INIT", &self.DELTA_IBIAS_INIT())
            .field("ALT_DCDC_IPEAK", &self.ALT_DCDC_IPEAK())
            .field("ALT_DCDC_DITHER_EN", &self.ALT_DCDC_DITHER_EN())
            .field("ALT_DCDC_VMIN", &self.ALT_DCDC_VMIN())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MODE_CONF_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MODE_CONF_1 {{ XOSC_MAX_START: {=u8:?}, DELTA_IBIAS_OFFSET: {=u8:?}, DELTA_IBIAS_INIT: {=u8:?}, ALT_DCDC_IPEAK: {=u8:?}, ALT_DCDC_DITHER_EN: {=bool:?}, ALT_DCDC_VMIN: {=u8:?}, RESERVED: {=u8:?} }}",
            self.XOSC_MAX_START(),
            self.DELTA_IBIAS_OFFSET(),
            self.DELTA_IBIAS_INIT(),
            self.ALT_DCDC_IPEAK(),
            self.ALT_DCDC_DITHER_EN(),
            self.ALT_DCDC_VMIN(),
            self.RESERVED()
        )
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESERVED_0(pub u32);
impl RESERVED_0 {
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
impl Default for RESERVED_0 {
    #[inline(always)]
    fn default() -> RESERVED_0 {
        RESERVED_0(0)
    }
}
impl core::fmt::Debug for RESERVED_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVED_0")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESERVED_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RESERVED_0 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTC_OFFSET(pub u32);
impl RTC_OFFSET {
    #[doc = "7:0\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_COMP_P2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RTC_COMP_P2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_COMP_P1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RTC_COMP_P1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "31:16\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RTC_COMP_P0(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RTC_COMP_P0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for RTC_OFFSET {
    #[inline(always)]
    fn default() -> RTC_OFFSET {
        RTC_OFFSET(0)
    }
}
impl core::fmt::Debug for RTC_OFFSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_OFFSET")
            .field("RTC_COMP_P2", &self.RTC_COMP_P2())
            .field("RTC_COMP_P1", &self.RTC_COMP_P1())
            .field("RTC_COMP_P0", &self.RTC_COMP_P0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RTC_OFFSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RTC_OFFSET {{ RTC_COMP_P2: {=u8:?}, RTC_COMP_P1: {=u8:?}, RTC_COMP_P0: {=u16:?} }}",
            self.RTC_COMP_P2(),
            self.RTC_COMP_P1(),
            self.RTC_COMP_P0()
        )
    }
}
#[doc = "CCFG Size and Disable Flags."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SIZE_AND_DIS_FLAGS(pub u32);
impl SIZE_AND_DIS_FLAGS {
    #[doc = "0:0\\] Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_XOSC_OVR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START."]
    #[inline(always)]
    pub const fn set_DIS_XOSC_OVR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_ALT_DCDC_SETTING(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub const fn set_DIS_ALT_DCDC_SETTING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_GPRAM(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE."]
    #[inline(always)]
    pub const fn set_DIS_GPRAM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Disable TCXO. 0: TCXO functionality enabled. 1: TCXO functionality disabled. Note: An external TCXO is required if DIS_TCXO = 0."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_TCXO(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Disable TCXO. 0: TCXO functionality enabled. 1: TCXO functionality disabled. Note: An external TCXO is required if DIS_TCXO = 0."]
    #[inline(always)]
    pub const fn set_DIS_TCXO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "15:4\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE_FLAGS(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "15:4\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_DISABLE_FLAGS(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "31:16\\] Total size of CCFG in bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn SIZE_OF_CCFG(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Total size of CCFG in bytes."]
    #[inline(always)]
    pub const fn set_SIZE_OF_CCFG(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for SIZE_AND_DIS_FLAGS {
    #[inline(always)]
    fn default() -> SIZE_AND_DIS_FLAGS {
        SIZE_AND_DIS_FLAGS(0)
    }
}
impl core::fmt::Debug for SIZE_AND_DIS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIZE_AND_DIS_FLAGS")
            .field("DIS_XOSC_OVR", &self.DIS_XOSC_OVR())
            .field("DIS_ALT_DCDC_SETTING", &self.DIS_ALT_DCDC_SETTING())
            .field("DIS_GPRAM", &self.DIS_GPRAM())
            .field("DIS_TCXO", &self.DIS_TCXO())
            .field("DISABLE_FLAGS", &self.DISABLE_FLAGS())
            .field("SIZE_OF_CCFG", &self.SIZE_OF_CCFG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SIZE_AND_DIS_FLAGS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SIZE_AND_DIS_FLAGS {{ DIS_XOSC_OVR: {=bool:?}, DIS_ALT_DCDC_SETTING: {=bool:?}, DIS_GPRAM: {=bool:?}, DIS_TCXO: {=bool:?}, DISABLE_FLAGS: {=u16:?}, SIZE_OF_CCFG: {=u16:?} }}",
            self.DIS_XOSC_OVR(),
            self.DIS_ALT_DCDC_SETTING(),
            self.DIS_GPRAM(),
            self.DIS_TCXO(),
            self.DISABLE_FLAGS(),
            self.SIZE_OF_CCFG()
        )
    }
}
#[doc = "Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VOLT_LOAD_0(pub u32);
impl VOLT_LOAD_0 {
    #[doc = "7:0\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_EXT_TM15(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_VDDR_EXT_TM15(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_EXT_TP5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_VDDR_EXT_TP5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_EXT_TP25(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_VDDR_EXT_TP25(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_EXT_TP45(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_VDDR_EXT_TP45(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for VOLT_LOAD_0 {
    #[inline(always)]
    fn default() -> VOLT_LOAD_0 {
        VOLT_LOAD_0(0)
    }
}
impl core::fmt::Debug for VOLT_LOAD_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VOLT_LOAD_0")
            .field("VDDR_EXT_TM15", &self.VDDR_EXT_TM15())
            .field("VDDR_EXT_TP5", &self.VDDR_EXT_TP5())
            .field("VDDR_EXT_TP25", &self.VDDR_EXT_TP25())
            .field("VDDR_EXT_TP45", &self.VDDR_EXT_TP45())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VOLT_LOAD_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VOLT_LOAD_0 {{ VDDR_EXT_TM15: {=u8:?}, VDDR_EXT_TP5: {=u8:?}, VDDR_EXT_TP25: {=u8:?}, VDDR_EXT_TP45: {=u8:?} }}",
            self.VDDR_EXT_TM15(),
            self.VDDR_EXT_TP5(),
            self.VDDR_EXT_TP25(),
            self.VDDR_EXT_TP45()
        )
    }
}
#[doc = "Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VOLT_LOAD_1(pub u32);
impl VOLT_LOAD_1 {
    #[doc = "7:0\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_EXT_TP65(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_VDDR_EXT_TP65(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_EXT_TP85(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_VDDR_EXT_TP85(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_EXT_TP105(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_VDDR_EXT_TP105(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn VDDR_EXT_TP125(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_VDDR_EXT_TP125(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for VOLT_LOAD_1 {
    #[inline(always)]
    fn default() -> VOLT_LOAD_1 {
        VOLT_LOAD_1(0)
    }
}
impl core::fmt::Debug for VOLT_LOAD_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VOLT_LOAD_1")
            .field("VDDR_EXT_TP65", &self.VDDR_EXT_TP65())
            .field("VDDR_EXT_TP85", &self.VDDR_EXT_TP85())
            .field("VDDR_EXT_TP105", &self.VDDR_EXT_TP105())
            .field("VDDR_EXT_TP125", &self.VDDR_EXT_TP125())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VOLT_LOAD_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VOLT_LOAD_1 {{ VDDR_EXT_TP65: {=u8:?}, VDDR_EXT_TP85: {=u8:?}, VDDR_EXT_TP105: {=u8:?}, VDDR_EXT_TP125: {=u8:?} }}",
            self.VDDR_EXT_TP65(),
            self.VDDR_EXT_TP85(),
            self.VDDR_EXT_TP105(),
            self.VDDR_EXT_TP125()
        )
    }
}

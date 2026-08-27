#[doc = "Customer configuration area (CCFG)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCFG {
    ptr: *mut u8,
}
unsafe impl Send for CCFG {}
unsafe impl Sync for CCFG {}
impl CCFG {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn RESERVED_0(self) -> crate::common::Reg<regs::RESERVED_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Extern LF clock configuration."]
    #[inline(always)]
    pub const fn EXT_LF_CLK(self) -> crate::common::Reg<regs::EXT_LF_CLK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa8usize) as _) }
    }
    #[doc = "Mode Configuration 1."]
    #[inline(always)]
    pub const fn MODE_CONF_1(self) -> crate::common::Reg<regs::MODE_CONF_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0facusize) as _) }
    }
    #[doc = "CCFG Size and Disable Flags."]
    #[inline(always)]
    pub const fn SIZE_AND_DIS_FLAGS(
        self,
    ) -> crate::common::Reg<regs::SIZE_AND_DIS_FLAGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb0usize) as _) }
    }
    #[doc = "Mode Configuration 0."]
    #[inline(always)]
    pub const fn MODE_CONF(self) -> crate::common::Reg<regs::MODE_CONF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb4usize) as _) }
    }
    #[doc = "Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    #[inline(always)]
    pub const fn VOLT_LOAD_0(self) -> crate::common::Reg<regs::VOLT_LOAD_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb8usize) as _) }
    }
    #[doc = "Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD."]
    #[inline(always)]
    pub const fn VOLT_LOAD_1(self) -> crate::common::Reg<regs::VOLT_LOAD_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fbcusize) as _) }
    }
    #[doc = "Real Time Clock Offset Enabled by MODE_CONF.RTC_COMP."]
    #[inline(always)]
    pub const fn RTC_OFFSET(self) -> crate::common::Reg<regs::RTC_OFFSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc0usize) as _) }
    }
    #[doc = "Frequency Offset."]
    #[inline(always)]
    pub const fn FREQ_OFFSET(self) -> crate::common::Reg<regs::FREQ_OFFSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc4usize) as _) }
    }
    #[doc = "IEEE MAC Address 0."]
    #[inline(always)]
    pub const fn IEEE_MAC_0(self) -> crate::common::Reg<regs::IEEE_MAC_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc8usize) as _) }
    }
    #[doc = "IEEE MAC Address 1."]
    #[inline(always)]
    pub const fn IEEE_MAC_1(self) -> crate::common::Reg<regs::IEEE_MAC_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fccusize) as _) }
    }
    #[doc = "IEEE BLE Address 0."]
    #[inline(always)]
    pub const fn IEEE_BLE_0(self) -> crate::common::Reg<regs::IEEE_BLE_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
    }
    #[doc = "IEEE BLE Address 1."]
    #[inline(always)]
    pub const fn IEEE_BLE_1(self) -> crate::common::Reg<regs::IEEE_BLE_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd4usize) as _) }
    }
    #[doc = "Bootloader Configuration Configures the functionality of the ROM boot loader. If both the boot loader is enabled by the BOOTLOADER_ENABLE field and the boot loader backdoor is enabled by the BL_ENABLE field it is possible to force entry of the ROM boot loader even if a valid image is present in flash."]
    #[inline(always)]
    pub const fn BL_CONFIG(self) -> crate::common::Reg<regs::BL_CONFIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd8usize) as _) }
    }
    #[doc = "Erase Configuration."]
    #[inline(always)]
    pub const fn ERASE_CONF(self) -> crate::common::Reg<regs::ERASE_CONF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fdcusize) as _) }
    }
    #[doc = "TI Options."]
    #[inline(always)]
    pub const fn CCFG_TI_OPTIONS(
        self,
    ) -> crate::common::Reg<regs::CCFG_TI_OPTIONS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe0usize) as _) }
    }
    #[doc = "Test Access Points Enable 0."]
    #[inline(always)]
    pub const fn CCFG_TAP_DAP_0(
        self,
    ) -> crate::common::Reg<regs::CCFG_TAP_DAP_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe4usize) as _) }
    }
    #[doc = "Test Access Points Enable 1."]
    #[inline(always)]
    pub const fn CCFG_TAP_DAP_1(
        self,
    ) -> crate::common::Reg<regs::CCFG_TAP_DAP_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe8usize) as _) }
    }
    #[doc = "Image Valid."]
    #[inline(always)]
    pub const fn IMAGE_VALID_CONF(
        self,
    ) -> crate::common::Reg<regs::IMAGE_VALID_CONF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "Protect Sectors 0-31 Each bit write protects one 4KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect."]
    #[inline(always)]
    pub const fn CCFG_PROT_31_0(
        self,
    ) -> crate::common::Reg<regs::CCFG_PROT_31_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
    }
    #[doc = "Protect Sectors 32-63 Each bit write protects one 4KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0."]
    #[inline(always)]
    pub const fn CCFG_PROT_63_32(
        self,
    ) -> crate::common::Reg<regs::CCFG_PROT_63_32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff4usize) as _) }
    }
    #[doc = "Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0."]
    #[inline(always)]
    pub const fn CCFG_PROT_95_64(
        self,
    ) -> crate::common::Reg<regs::CCFG_PROT_95_64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use by CC26x0 and CC13x0."]
    #[inline(always)]
    pub const fn CCFG_PROT_127_96(
        self,
    ) -> crate::common::Reg<regs::CCFG_PROT_127_96, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;

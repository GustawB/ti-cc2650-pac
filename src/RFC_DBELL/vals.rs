#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BOOT_DONE {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl BOOT_DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BOOT_DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BOOT_DONE {
    #[inline(always)]
    fn from(val: u8) -> BOOT_DONE {
        BOOT_DONE::from_bits(val)
    }
}
impl From<BOOT_DONE> for u8 {
    #[inline(always)]
    fn from(val: BOOT_DONE) -> u8 {
        BOOT_DONE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum COMMAND_DONE {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl COMMAND_DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> COMMAND_DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for COMMAND_DONE {
    #[inline(always)]
    fn from(val: u8) -> COMMAND_DONE {
        COMMAND_DONE::from_bits(val)
    }
}
impl From<COMMAND_DONE> for u8 {
    #[inline(always)]
    fn from(val: COMMAND_DONE) -> u8 {
        COMMAND_DONE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FG_COMMAND_DONE {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl FG_COMMAND_DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FG_COMMAND_DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FG_COMMAND_DONE {
    #[inline(always)]
    fn from(val: u8) -> FG_COMMAND_DONE {
        FG_COMMAND_DONE::from_bits(val)
    }
}
impl From<FG_COMMAND_DONE> for u8 {
    #[inline(always)]
    fn from(val: FG_COMMAND_DONE) -> u8 {
        FG_COMMAND_DONE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPOCTL0 {
    #[doc = "CPE GPO line 0."]
    CPEGPO0 = 0x0,
    #[doc = "CPE GPO line 1."]
    CPEGPO1 = 0x01,
    #[doc = "CPE GPO line 2."]
    CPEGPO2 = 0x02,
    #[doc = "CPE GPO line 3."]
    CPEGPO3 = 0x03,
    #[doc = "MCE GPO line 0."]
    MCEGPO0 = 0x04,
    #[doc = "MCE GPO line 1."]
    MCEGPO1 = 0x05,
    #[doc = "MCE GPO line 2."]
    MCEGPO2 = 0x06,
    #[doc = "MCE GPO line 3."]
    MCEGPO3 = 0x07,
    #[doc = "RFE GPO line 0."]
    RFEGPO0 = 0x08,
    #[doc = "RFE GPO line 1."]
    RFEGPO1 = 0x09,
    #[doc = "RFE GPO line 2."]
    RFEGPO2 = 0x0a,
    #[doc = "RFE GPO line 3."]
    RFEGPO3 = 0x0b,
    #[doc = "RAT GPO line 0."]
    RATGPO0 = 0x0c,
    #[doc = "RAT GPO line 1."]
    RATGPO1 = 0x0d,
    #[doc = "RAT GPO line 2."]
    RATGPO2 = 0x0e,
    #[doc = "RAT GPO line 3."]
    RATGPO3 = 0x0f,
}
impl GPOCTL0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPOCTL0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPOCTL0 {
    #[inline(always)]
    fn from(val: u8) -> GPOCTL0 {
        GPOCTL0::from_bits(val)
    }
}
impl From<GPOCTL0> for u8 {
    #[inline(always)]
    fn from(val: GPOCTL0) -> u8 {
        GPOCTL0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPOCTL1 {
    #[doc = "CPE GPO line 0."]
    CPEGPO0 = 0x0,
    #[doc = "CPE GPO line 1."]
    CPEGPO1 = 0x01,
    #[doc = "CPE GPO line 2."]
    CPEGPO2 = 0x02,
    #[doc = "CPE GPO line 3."]
    CPEGPO3 = 0x03,
    #[doc = "MCE GPO line 0."]
    MCEGPO0 = 0x04,
    #[doc = "MCE GPO line 1."]
    MCEGPO1 = 0x05,
    #[doc = "MCE GPO line 2."]
    MCEGPO2 = 0x06,
    #[doc = "MCE GPO line 3."]
    MCEGPO3 = 0x07,
    #[doc = "RFE GPO line 0."]
    RFEGPO0 = 0x08,
    #[doc = "RFE GPO line 1."]
    RFEGPO1 = 0x09,
    #[doc = "RFE GPO line 2."]
    RFEGPO2 = 0x0a,
    #[doc = "RFE GPO line 3."]
    RFEGPO3 = 0x0b,
    #[doc = "RAT GPO line 0."]
    RATGPO0 = 0x0c,
    #[doc = "RAT GPO line 1."]
    RATGPO1 = 0x0d,
    #[doc = "RAT GPO line 2."]
    RATGPO2 = 0x0e,
    #[doc = "RAT GPO line 3."]
    RATGPO3 = 0x0f,
}
impl GPOCTL1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPOCTL1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPOCTL1 {
    #[inline(always)]
    fn from(val: u8) -> GPOCTL1 {
        GPOCTL1::from_bits(val)
    }
}
impl From<GPOCTL1> for u8 {
    #[inline(always)]
    fn from(val: GPOCTL1) -> u8 {
        GPOCTL1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPOCTL2 {
    #[doc = "CPE GPO line 0."]
    CPEGPO0 = 0x0,
    #[doc = "CPE GPO line 1."]
    CPEGPO1 = 0x01,
    #[doc = "CPE GPO line 2."]
    CPEGPO2 = 0x02,
    #[doc = "CPE GPO line 3."]
    CPEGPO3 = 0x03,
    #[doc = "MCE GPO line 0."]
    MCEGPO0 = 0x04,
    #[doc = "MCE GPO line 1."]
    MCEGPO1 = 0x05,
    #[doc = "MCE GPO line 2."]
    MCEGPO2 = 0x06,
    #[doc = "MCE GPO line 3."]
    MCEGPO3 = 0x07,
    #[doc = "RFE GPO line 0."]
    RFEGPO0 = 0x08,
    #[doc = "RFE GPO line 1."]
    RFEGPO1 = 0x09,
    #[doc = "RFE GPO line 2."]
    RFEGPO2 = 0x0a,
    #[doc = "RFE GPO line 3."]
    RFEGPO3 = 0x0b,
    #[doc = "RAT GPO line 0."]
    RATGPO0 = 0x0c,
    #[doc = "RAT GPO line 1."]
    RATGPO1 = 0x0d,
    #[doc = "RAT GPO line 2."]
    RATGPO2 = 0x0e,
    #[doc = "RAT GPO line 3."]
    RATGPO3 = 0x0f,
}
impl GPOCTL2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPOCTL2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPOCTL2 {
    #[inline(always)]
    fn from(val: u8) -> GPOCTL2 {
        GPOCTL2::from_bits(val)
    }
}
impl From<GPOCTL2> for u8 {
    #[inline(always)]
    fn from(val: GPOCTL2) -> u8 {
        GPOCTL2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPOCTL3 {
    #[doc = "CPE GPO line 0."]
    CPEGPO0 = 0x0,
    #[doc = "CPE GPO line 1."]
    CPEGPO1 = 0x01,
    #[doc = "CPE GPO line 2."]
    CPEGPO2 = 0x02,
    #[doc = "CPE GPO line 3."]
    CPEGPO3 = 0x03,
    #[doc = "MCE GPO line 0."]
    MCEGPO0 = 0x04,
    #[doc = "MCE GPO line 1."]
    MCEGPO1 = 0x05,
    #[doc = "MCE GPO line 2."]
    MCEGPO2 = 0x06,
    #[doc = "MCE GPO line 3."]
    MCEGPO3 = 0x07,
    #[doc = "RFE GPO line 0."]
    RFEGPO0 = 0x08,
    #[doc = "RFE GPO line 1."]
    RFEGPO1 = 0x09,
    #[doc = "RFE GPO line 2."]
    RFEGPO2 = 0x0a,
    #[doc = "RFE GPO line 3."]
    RFEGPO3 = 0x0b,
    #[doc = "RAT GPO line 0."]
    RATGPO0 = 0x0c,
    #[doc = "RAT GPO line 1."]
    RATGPO1 = 0x0d,
    #[doc = "RAT GPO line 2."]
    RATGPO2 = 0x0e,
    #[doc = "RAT GPO line 3."]
    RATGPO3 = 0x0f,
}
impl GPOCTL3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPOCTL3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPOCTL3 {
    #[inline(always)]
    fn from(val: u8) -> GPOCTL3 {
        GPOCTL3::from_bits(val)
    }
}
impl From<GPOCTL3> for u8 {
    #[inline(always)]
    fn from(val: GPOCTL3) -> u8 {
        GPOCTL3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTERNAL_ERROR {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl INTERNAL_ERROR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTERNAL_ERROR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTERNAL_ERROR {
    #[inline(always)]
    fn from(val: u8) -> INTERNAL_ERROR {
        INTERNAL_ERROR::from_bits(val)
    }
}
impl From<INTERNAL_ERROR> for u8 {
    #[inline(always)]
    fn from(val: INTERNAL_ERROR) -> u8 {
        INTERNAL_ERROR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IRQ12 {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl IRQ12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IRQ12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IRQ12 {
    #[inline(always)]
    fn from(val: u8) -> IRQ12 {
        IRQ12::from_bits(val)
    }
}
impl From<IRQ12> for u8 {
    #[inline(always)]
    fn from(val: IRQ12) -> u8 {
        IRQ12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IRQ13 {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl IRQ13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IRQ13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IRQ13 {
    #[inline(always)]
    fn from(val: u8) -> IRQ13 {
        IRQ13::from_bits(val)
    }
}
impl From<IRQ13> for u8 {
    #[inline(always)]
    fn from(val: IRQ13) -> u8 {
        IRQ13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IRQ14 {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl IRQ14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IRQ14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IRQ14 {
    #[inline(always)]
    fn from(val: u8) -> IRQ14 {
        IRQ14::from_bits(val)
    }
}
impl From<IRQ14> for u8 {
    #[inline(always)]
    fn from(val: IRQ14) -> u8 {
        IRQ14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IRQ15 {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl IRQ15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IRQ15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IRQ15 {
    #[inline(always)]
    fn from(val: u8) -> IRQ15 {
        IRQ15::from_bits(val)
    }
}
impl From<IRQ15> for u8 {
    #[inline(always)]
    fn from(val: IRQ15) -> u8 {
        IRQ15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IRQ27 {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl IRQ27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IRQ27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IRQ27 {
    #[inline(always)]
    fn from(val: u8) -> IRQ27 {
        IRQ27::from_bits(val)
    }
}
impl From<IRQ27> for u8 {
    #[inline(always)]
    fn from(val: IRQ27) -> u8 {
        IRQ27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LAST_COMMAND_DONE {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl LAST_COMMAND_DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LAST_COMMAND_DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LAST_COMMAND_DONE {
    #[inline(always)]
    fn from(val: u8) -> LAST_COMMAND_DONE {
        LAST_COMMAND_DONE::from_bits(val)
    }
}
impl From<LAST_COMMAND_DONE> for u8 {
    #[inline(always)]
    fn from(val: LAST_COMMAND_DONE) -> u8 {
        LAST_COMMAND_DONE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LAST_FG_COMMAND_DONE {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl LAST_FG_COMMAND_DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LAST_FG_COMMAND_DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LAST_FG_COMMAND_DONE {
    #[inline(always)]
    fn from(val: u8) -> LAST_FG_COMMAND_DONE {
        LAST_FG_COMMAND_DONE::from_bits(val)
    }
}
impl From<LAST_FG_COMMAND_DONE> for u8 {
    #[inline(always)]
    fn from(val: LAST_FG_COMMAND_DONE) -> u8 {
        LAST_FG_COMMAND_DONE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODULES_UNLOCKED {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl MODULES_UNLOCKED {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODULES_UNLOCKED {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODULES_UNLOCKED {
    #[inline(always)]
    fn from(val: u8) -> MODULES_UNLOCKED {
        MODULES_UNLOCKED::from_bits(val)
    }
}
impl From<MODULES_UNLOCKED> for u8 {
    #[inline(always)]
    fn from(val: MODULES_UNLOCKED) -> u8 {
        MODULES_UNLOCKED::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_ABORTED {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl RX_ABORTED {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_ABORTED {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_ABORTED {
    #[inline(always)]
    fn from(val: u8) -> RX_ABORTED {
        RX_ABORTED::from_bits(val)
    }
}
impl From<RX_ABORTED> for u8 {
    #[inline(always)]
    fn from(val: RX_ABORTED) -> u8 {
        RX_ABORTED::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_BUF_FULL {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl RX_BUF_FULL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_BUF_FULL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_BUF_FULL {
    #[inline(always)]
    fn from(val: u8) -> RX_BUF_FULL {
        RX_BUF_FULL::from_bits(val)
    }
}
impl From<RX_BUF_FULL> for u8 {
    #[inline(always)]
    fn from(val: RX_BUF_FULL) -> u8 {
        RX_BUF_FULL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_CTRL {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl RX_CTRL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_CTRL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_CTRL {
    #[inline(always)]
    fn from(val: u8) -> RX_CTRL {
        RX_CTRL::from_bits(val)
    }
}
impl From<RX_CTRL> for u8 {
    #[inline(always)]
    fn from(val: RX_CTRL) -> u8 {
        RX_CTRL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_CTRL_ACK {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl RX_CTRL_ACK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_CTRL_ACK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_CTRL_ACK {
    #[inline(always)]
    fn from(val: u8) -> RX_CTRL_ACK {
        RX_CTRL_ACK::from_bits(val)
    }
}
impl From<RX_CTRL_ACK> for u8 {
    #[inline(always)]
    fn from(val: RX_CTRL_ACK) -> u8 {
        RX_CTRL_ACK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_DATA_WRITTEN {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl RX_DATA_WRITTEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_DATA_WRITTEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_DATA_WRITTEN {
    #[inline(always)]
    fn from(val: u8) -> RX_DATA_WRITTEN {
        RX_DATA_WRITTEN::from_bits(val)
    }
}
impl From<RX_DATA_WRITTEN> for u8 {
    #[inline(always)]
    fn from(val: RX_DATA_WRITTEN) -> u8 {
        RX_DATA_WRITTEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_EMPTY {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl RX_EMPTY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_EMPTY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_EMPTY {
    #[inline(always)]
    fn from(val: u8) -> RX_EMPTY {
        RX_EMPTY::from_bits(val)
    }
}
impl From<RX_EMPTY> for u8 {
    #[inline(always)]
    fn from(val: RX_EMPTY) -> u8 {
        RX_EMPTY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_ENTRY_DONE {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl RX_ENTRY_DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_ENTRY_DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_ENTRY_DONE {
    #[inline(always)]
    fn from(val: u8) -> RX_ENTRY_DONE {
        RX_ENTRY_DONE::from_bits(val)
    }
}
impl From<RX_ENTRY_DONE> for u8 {
    #[inline(always)]
    fn from(val: RX_ENTRY_DONE) -> u8 {
        RX_ENTRY_DONE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_IGNORED {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl RX_IGNORED {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_IGNORED {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_IGNORED {
    #[inline(always)]
    fn from(val: u8) -> RX_IGNORED {
        RX_IGNORED::from_bits(val)
    }
}
impl From<RX_IGNORED> for u8 {
    #[inline(always)]
    fn from(val: RX_IGNORED) -> u8 {
        RX_IGNORED::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_NOK {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl RX_NOK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_NOK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_NOK {
    #[inline(always)]
    fn from(val: u8) -> RX_NOK {
        RX_NOK::from_bits(val)
    }
}
impl From<RX_NOK> for u8 {
    #[inline(always)]
    fn from(val: RX_NOK) -> u8 {
        RX_NOK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_N_DATA_WRITTEN {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl RX_N_DATA_WRITTEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_N_DATA_WRITTEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_N_DATA_WRITTEN {
    #[inline(always)]
    fn from(val: u8) -> RX_N_DATA_WRITTEN {
        RX_N_DATA_WRITTEN::from_bits(val)
    }
}
impl From<RX_N_DATA_WRITTEN> for u8 {
    #[inline(always)]
    fn from(val: RX_N_DATA_WRITTEN) -> u8 {
        RX_N_DATA_WRITTEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_OK {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl RX_OK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_OK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_OK {
    #[inline(always)]
    fn from(val: u8) -> RX_OK {
        RX_OK::from_bits(val)
    }
}
impl From<RX_OK> for u8 {
    #[inline(always)]
    fn from(val: RX_OK) -> u8 {
        RX_OK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYNTH_NO_LOCK {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl SYNTH_NO_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYNTH_NO_LOCK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYNTH_NO_LOCK {
    #[inline(always)]
    fn from(val: u8) -> SYNTH_NO_LOCK {
        SYNTH_NO_LOCK::from_bits(val)
    }
}
impl From<SYNTH_NO_LOCK> for u8 {
    #[inline(always)]
    fn from(val: SYNTH_NO_LOCK) -> u8 {
        SYNTH_NO_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TX_ACK {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl TX_ACK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TX_ACK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TX_ACK {
    #[inline(always)]
    fn from(val: u8) -> TX_ACK {
        TX_ACK::from_bits(val)
    }
}
impl From<TX_ACK> for u8 {
    #[inline(always)]
    fn from(val: TX_ACK) -> u8 {
        TX_ACK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TX_BUFFER_CHANGED {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl TX_BUFFER_CHANGED {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TX_BUFFER_CHANGED {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TX_BUFFER_CHANGED {
    #[inline(always)]
    fn from(val: u8) -> TX_BUFFER_CHANGED {
        TX_BUFFER_CHANGED::from_bits(val)
    }
}
impl From<TX_BUFFER_CHANGED> for u8 {
    #[inline(always)]
    fn from(val: TX_BUFFER_CHANGED) -> u8 {
        TX_BUFFER_CHANGED::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TX_CTRL {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl TX_CTRL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TX_CTRL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TX_CTRL {
    #[inline(always)]
    fn from(val: u8) -> TX_CTRL {
        TX_CTRL::from_bits(val)
    }
}
impl From<TX_CTRL> for u8 {
    #[inline(always)]
    fn from(val: TX_CTRL) -> u8 {
        TX_CTRL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TX_CTRL_ACK {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl TX_CTRL_ACK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TX_CTRL_ACK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TX_CTRL_ACK {
    #[inline(always)]
    fn from(val: u8) -> TX_CTRL_ACK {
        TX_CTRL_ACK::from_bits(val)
    }
}
impl From<TX_CTRL_ACK> for u8 {
    #[inline(always)]
    fn from(val: TX_CTRL_ACK) -> u8 {
        TX_CTRL_ACK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TX_CTRL_ACK_ACK {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl TX_CTRL_ACK_ACK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TX_CTRL_ACK_ACK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TX_CTRL_ACK_ACK {
    #[inline(always)]
    fn from(val: u8) -> TX_CTRL_ACK_ACK {
        TX_CTRL_ACK_ACK::from_bits(val)
    }
}
impl From<TX_CTRL_ACK_ACK> for u8 {
    #[inline(always)]
    fn from(val: TX_CTRL_ACK_ACK) -> u8 {
        TX_CTRL_ACK_ACK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TX_DONE {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl TX_DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TX_DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TX_DONE {
    #[inline(always)]
    fn from(val: u8) -> TX_DONE {
        TX_DONE::from_bits(val)
    }
}
impl From<TX_DONE> for u8 {
    #[inline(always)]
    fn from(val: TX_DONE) -> u8 {
        TX_DONE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TX_ENTRY_DONE {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl TX_ENTRY_DONE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TX_ENTRY_DONE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TX_ENTRY_DONE {
    #[inline(always)]
    fn from(val: u8) -> TX_ENTRY_DONE {
        TX_ENTRY_DONE::from_bits(val)
    }
}
impl From<TX_ENTRY_DONE> for u8 {
    #[inline(always)]
    fn from(val: TX_ENTRY_DONE) -> u8 {
        TX_ENTRY_DONE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TX_RETRANS {
    #[doc = "Associate this interrupt line with INT_RF_CPE0 interrupt vector."]
    CPE0 = 0x0,
    #[doc = "Associate this interrupt line with INT_RF_CPE1 interrupt vector."]
    CPE1 = 0x01,
}
impl TX_RETRANS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TX_RETRANS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TX_RETRANS {
    #[inline(always)]
    fn from(val: u8) -> TX_RETRANS {
        TX_RETRANS::from_bits(val)
    }
}
impl From<TX_RETRANS> for u8 {
    #[inline(always)]
    fn from(val: TX_RETRANS) -> u8 {
        TX_RETRANS::to_bits(val)
    }
}

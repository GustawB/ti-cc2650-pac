#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AHB_MST1_BIGEND {
    #[doc = "Little Endian."]
    LITTLE_ENDIAN = 0x0,
    #[doc = "Big Endian."]
    BIG_ENDIAN = 0x01,
}
impl AHB_MST1_BIGEND {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AHB_MST1_BIGEND {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AHB_MST1_BIGEND {
    #[inline(always)]
    fn from(val: u8) -> AHB_MST1_BIGEND {
        AHB_MST1_BIGEND::from_bits(val)
    }
}
impl From<AHB_MST1_BIGEND> for u8 {
    #[inline(always)]
    fn from(val: AHB_MST1_BIGEND) -> u8 {
        AHB_MST1_BIGEND::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AHB_MST1_BURST_SIZE {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "4 bytes."]
    _4_BYTE = 0x02,
    #[doc = "8 bytes."]
    _8_BYTE = 0x03,
    #[doc = "16 bytes."]
    _16_BYTE = 0x04,
    #[doc = "32 bytes."]
    _32_BYTE = 0x05,
    #[doc = "64 bytes."]
    _64_BYTE = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl AHB_MST1_BURST_SIZE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AHB_MST1_BURST_SIZE {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AHB_MST1_BURST_SIZE {
    #[inline(always)]
    fn from(val: u8) -> AHB_MST1_BURST_SIZE {
        AHB_MST1_BURST_SIZE::from_bits(val)
    }
}
impl From<AHB_MST1_BURST_SIZE> for u8 {
    #[inline(always)]
    fn from(val: AHB_MST1_BURST_SIZE) -> u8 {
        AHB_MST1_BURST_SIZE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AHB_MST1_IDLE_EN {
    #[doc = "Do not insert idle transfers."]
    NO_IDLE = 0x0,
    #[doc = "Idle transfer insertion enabled."]
    IDLE = 0x01,
}
impl AHB_MST1_IDLE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AHB_MST1_IDLE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AHB_MST1_IDLE_EN {
    #[inline(always)]
    fn from(val: u8) -> AHB_MST1_IDLE_EN {
        AHB_MST1_IDLE_EN::from_bits(val)
    }
}
impl From<AHB_MST1_IDLE_EN> for u8 {
    #[inline(always)]
    fn from(val: AHB_MST1_IDLE_EN) -> u8 {
        AHB_MST1_IDLE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AHB_MST1_INCR_EN {
    #[doc = "Unspecified length burst transfers."]
    UNSPECIFIED = 0x0,
    #[doc = "Fixed length bursts or single transfers."]
    SPECIFIED = 0x01,
}
impl AHB_MST1_INCR_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AHB_MST1_INCR_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AHB_MST1_INCR_EN {
    #[inline(always)]
    fn from(val: u8) -> AHB_MST1_INCR_EN {
        AHB_MST1_INCR_EN::from_bits(val)
    }
}
impl From<AHB_MST1_INCR_EN> for u8 {
    #[inline(always)]
    fn from(val: AHB_MST1_INCR_EN) -> u8 {
        AHB_MST1_INCR_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AHB_MST1_LOCK_EN {
    #[doc = "Transfers are not locked."]
    NOT_LOCKED = 0x0,
    #[doc = "Transfers are locked."]
    LOCKED = 0x01,
}
impl AHB_MST1_LOCK_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AHB_MST1_LOCK_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AHB_MST1_LOCK_EN {
    #[inline(always)]
    fn from(val: u8) -> AHB_MST1_LOCK_EN {
        AHB_MST1_LOCK_EN::from_bits(val)
    }
}
impl From<AHB_MST1_LOCK_EN> for u8 {
    #[inline(always)]
    fn from(val: AHB_MST1_LOCK_EN) -> u8 {
        AHB_MST1_LOCK_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTR_WIDTH {
    #[doc = "32 bits."]
    _32_BIT = 0x0,
    #[doc = "64 bits."]
    _64_BIT = 0x01,
    #[doc = "96 bits."]
    _96_BIT = 0x02,
    #[doc = "128 bits."]
    _128_BIT = 0x03,
}
impl CTR_WIDTH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTR_WIDTH {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTR_WIDTH {
    #[inline(always)]
    fn from(val: u8) -> CTR_WIDTH {
        CTR_WIDTH::from_bits(val)
    }
}
impl From<CTR_WIDTH> for u8 {
    #[inline(always)]
    fn from(val: CTR_WIDTH) -> u8 {
        CTR_WIDTH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMACH0CTL_EN {
    #[doc = "Channel disabled."]
    DIS = 0x0,
    #[doc = "Channel enabled."]
    EN = 0x01,
}
impl DMACH0CTL_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMACH0CTL_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMACH0CTL_EN {
    #[inline(always)]
    fn from(val: u8) -> DMACH0CTL_EN {
        DMACH0CTL_EN::from_bits(val)
    }
}
impl From<DMACH0CTL_EN> for u8 {
    #[inline(always)]
    fn from(val: DMACH0CTL_EN) -> u8 {
        DMACH0CTL_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMACH0CTL_PRIO {
    #[doc = "Priority low."]
    LOW = 0x0,
    #[doc = "Priority high."]
    HIGH = 0x01,
}
impl DMACH0CTL_PRIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMACH0CTL_PRIO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMACH0CTL_PRIO {
    #[inline(always)]
    fn from(val: u8) -> DMACH0CTL_PRIO {
        DMACH0CTL_PRIO::from_bits(val)
    }
}
impl From<DMACH0CTL_PRIO> for u8 {
    #[inline(always)]
    fn from(val: DMACH0CTL_PRIO) -> u8 {
        DMACH0CTL_PRIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMACH1CTL_EN {
    #[doc = "Channel disabled."]
    DIS = 0x0,
    #[doc = "Channel enabled."]
    EN = 0x01,
}
impl DMACH1CTL_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMACH1CTL_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMACH1CTL_EN {
    #[inline(always)]
    fn from(val: u8) -> DMACH1CTL_EN {
        DMACH1CTL_EN::from_bits(val)
    }
}
impl From<DMACH1CTL_EN> for u8 {
    #[inline(always)]
    fn from(val: DMACH1CTL_EN) -> u8 {
        DMACH1CTL_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMACH1CTL_PRIO {
    #[doc = "Priority low."]
    LOW = 0x0,
    #[doc = "Priority high."]
    HIGH = 0x01,
}
impl DMACH1CTL_PRIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMACH1CTL_PRIO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMACH1CTL_PRIO {
    #[inline(always)]
    fn from(val: u8) -> DMACH1CTL_PRIO {
        DMACH1CTL_PRIO::from_bits(val)
    }
}
impl From<DMACH1CTL_PRIO> for u8 {
    #[inline(always)]
    fn from(val: DMACH1CTL_PRIO) -> u8 {
        DMACH1CTL_PRIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA {
    #[doc = "RAM Area 0."]
    RAM_AREA0 = 0x0,
    #[doc = "RAM Area 1."]
    RAM_AREA1 = 0x01,
    #[doc = "RAM Area 2."]
    RAM_AREA2 = 0x02,
    #[doc = "RAM Area 3."]
    RAM_AREA3 = 0x03,
    #[doc = "RAM Area 4."]
    RAM_AREA4 = 0x04,
    #[doc = "RAM Area 5."]
    RAM_AREA5 = 0x05,
    #[doc = "RAM Area 6."]
    RAM_AREA6 = 0x06,
    #[doc = "RAM Area 7."]
    RAM_AREA7 = 0x07,
    #[doc = "No RAM."]
    NO_RAM = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl RAM_AREA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA {
        RAM_AREA::from_bits(val)
    }
}
impl From<RAM_AREA> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA) -> u8 {
        RAM_AREA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA0 {
    #[doc = "This RAM area is not selected to be written."]
    NOT_SEL = 0x0,
    #[doc = "This RAM area is selected to be written."]
    SEL = 0x01,
}
impl RAM_AREA0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA0 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA0 {
        RAM_AREA0::from_bits(val)
    }
}
impl From<RAM_AREA0> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA0) -> u8 {
        RAM_AREA0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA1 {
    #[doc = "This RAM area is not selected to be written."]
    NOT_SEL = 0x0,
    #[doc = "This RAM area is selected to be written."]
    SEL = 0x01,
}
impl RAM_AREA1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA1 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA1 {
        RAM_AREA1::from_bits(val)
    }
}
impl From<RAM_AREA1> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA1) -> u8 {
        RAM_AREA1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA2 {
    #[doc = "This RAM area is not selected to be written."]
    NOT_SEL = 0x0,
    #[doc = "This RAM area is selected to be written."]
    SEL = 0x01,
}
impl RAM_AREA2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA2 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA2 {
        RAM_AREA2::from_bits(val)
    }
}
impl From<RAM_AREA2> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA2) -> u8 {
        RAM_AREA2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA3 {
    #[doc = "This RAM area is not selected to be written."]
    NOT_SEL = 0x0,
    #[doc = "This RAM area is selected to be written."]
    SEL = 0x01,
}
impl RAM_AREA3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA3 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA3 {
        RAM_AREA3::from_bits(val)
    }
}
impl From<RAM_AREA3> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA3) -> u8 {
        RAM_AREA3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA4 {
    #[doc = "This RAM area is not selected to be written."]
    NOT_SEL = 0x0,
    #[doc = "This RAM area is selected to be written."]
    SEL = 0x01,
}
impl RAM_AREA4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA4 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA4 {
        RAM_AREA4::from_bits(val)
    }
}
impl From<RAM_AREA4> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA4) -> u8 {
        RAM_AREA4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA5 {
    #[doc = "This RAM area is not selected to be written."]
    NOT_SEL = 0x0,
    #[doc = "This RAM area is selected to be written."]
    SEL = 0x01,
}
impl RAM_AREA5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA5 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA5 {
        RAM_AREA5::from_bits(val)
    }
}
impl From<RAM_AREA5> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA5) -> u8 {
        RAM_AREA5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA6 {
    #[doc = "This RAM area is not selected to be written."]
    NOT_SEL = 0x0,
    #[doc = "This RAM area is selected to be written."]
    SEL = 0x01,
}
impl RAM_AREA6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA6 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA6 {
        RAM_AREA6::from_bits(val)
    }
}
impl From<RAM_AREA6> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA6) -> u8 {
        RAM_AREA6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA7 {
    #[doc = "This RAM area is not selected to be written."]
    NOT_SEL = 0x0,
    #[doc = "This RAM area is selected to be written."]
    SEL = 0x01,
}
impl RAM_AREA7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA7 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA7 {
        RAM_AREA7::from_bits(val)
    }
}
impl From<RAM_AREA7> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA7) -> u8 {
        RAM_AREA7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA_WRITTEN0 {
    #[doc = "This RAM area is not written with valid key information."]
    NOT_WRITTEN = 0x0,
    #[doc = "This RAM area is written with valid key information."]
    WRITTEN = 0x01,
}
impl RAM_AREA_WRITTEN0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA_WRITTEN0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA_WRITTEN0 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA_WRITTEN0 {
        RAM_AREA_WRITTEN0::from_bits(val)
    }
}
impl From<RAM_AREA_WRITTEN0> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA_WRITTEN0) -> u8 {
        RAM_AREA_WRITTEN0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA_WRITTEN1 {
    #[doc = "This RAM area is not written with valid key information."]
    NOT_WRITTEN = 0x0,
    #[doc = "This RAM area is written with valid key information."]
    WRITTEN = 0x01,
}
impl RAM_AREA_WRITTEN1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA_WRITTEN1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA_WRITTEN1 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA_WRITTEN1 {
        RAM_AREA_WRITTEN1::from_bits(val)
    }
}
impl From<RAM_AREA_WRITTEN1> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA_WRITTEN1) -> u8 {
        RAM_AREA_WRITTEN1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA_WRITTEN2 {
    #[doc = "This RAM area is not written with valid key information."]
    NOT_WRITTEN = 0x0,
    #[doc = "This RAM area is written with valid key information."]
    WRITTEN = 0x01,
}
impl RAM_AREA_WRITTEN2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA_WRITTEN2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA_WRITTEN2 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA_WRITTEN2 {
        RAM_AREA_WRITTEN2::from_bits(val)
    }
}
impl From<RAM_AREA_WRITTEN2> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA_WRITTEN2) -> u8 {
        RAM_AREA_WRITTEN2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA_WRITTEN3 {
    #[doc = "This RAM area is not written with valid key information."]
    NOT_WRITTEN = 0x0,
    #[doc = "This RAM area is written with valid key information."]
    WRITTEN = 0x01,
}
impl RAM_AREA_WRITTEN3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA_WRITTEN3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA_WRITTEN3 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA_WRITTEN3 {
        RAM_AREA_WRITTEN3::from_bits(val)
    }
}
impl From<RAM_AREA_WRITTEN3> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA_WRITTEN3) -> u8 {
        RAM_AREA_WRITTEN3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA_WRITTEN4 {
    #[doc = "This RAM area is not written with valid key information."]
    NOT_WRITTEN = 0x0,
    #[doc = "This RAM area is written with valid key information."]
    WRITTEN = 0x01,
}
impl RAM_AREA_WRITTEN4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA_WRITTEN4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA_WRITTEN4 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA_WRITTEN4 {
        RAM_AREA_WRITTEN4::from_bits(val)
    }
}
impl From<RAM_AREA_WRITTEN4> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA_WRITTEN4) -> u8 {
        RAM_AREA_WRITTEN4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA_WRITTEN5 {
    #[doc = "This RAM area is not written with valid key information."]
    NOT_WRITTEN = 0x0,
    #[doc = "This RAM area is written with valid key information."]
    WRITTEN = 0x01,
}
impl RAM_AREA_WRITTEN5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA_WRITTEN5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA_WRITTEN5 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA_WRITTEN5 {
        RAM_AREA_WRITTEN5::from_bits(val)
    }
}
impl From<RAM_AREA_WRITTEN5> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA_WRITTEN5) -> u8 {
        RAM_AREA_WRITTEN5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA_WRITTEN6 {
    #[doc = "This RAM area is not written with valid key information."]
    NOT_WRITTEN = 0x0,
    #[doc = "This RAM area is written with valid key information."]
    WRITTEN = 0x01,
}
impl RAM_AREA_WRITTEN6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA_WRITTEN6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA_WRITTEN6 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA_WRITTEN6 {
        RAM_AREA_WRITTEN6::from_bits(val)
    }
}
impl From<RAM_AREA_WRITTEN6> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA_WRITTEN6) -> u8 {
        RAM_AREA_WRITTEN6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RAM_AREA_WRITTEN7 {
    #[doc = "This RAM area is not written with valid key information."]
    NOT_WRITTEN = 0x0,
    #[doc = "This RAM area is written with valid key information."]
    WRITTEN = 0x01,
}
impl RAM_AREA_WRITTEN7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RAM_AREA_WRITTEN7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RAM_AREA_WRITTEN7 {
    #[inline(always)]
    fn from(val: u8) -> RAM_AREA_WRITTEN7 {
        RAM_AREA_WRITTEN7::from_bits(val)
    }
}
impl From<RAM_AREA_WRITTEN7> for u8 {
    #[inline(always)]
    fn from(val: RAM_AREA_WRITTEN7) -> u8 {
        RAM_AREA_WRITTEN7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SIZE {
    _RESERVED_0 = 0x0,
    #[doc = "128 bits."]
    _128_BIT = 0x01,
    #[doc = "Not supported."]
    _192_BIT = 0x02,
    #[doc = "Not supported."]
    _256_BIT = 0x03,
}
impl SIZE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SIZE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SIZE {
    #[inline(always)]
    fn from(val: u8) -> SIZE {
        SIZE::from_bits(val)
    }
}
impl From<SIZE> for u8 {
    #[inline(always)]
    fn from(val: SIZE) -> u8 {
        SIZE::to_bits(val)
    }
}

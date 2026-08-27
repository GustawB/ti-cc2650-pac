#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTSEN {
    #[doc = "CTS hardware flow control disabled."]
    DIS = 0x0,
    #[doc = "CTS hardware flow control enabled."]
    EN = 0x01,
}
impl CTSEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTSEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTSEN {
    #[inline(always)]
    fn from(val: u8) -> CTSEN {
        CTSEN::from_bits(val)
    }
}
impl From<CTSEN> for u8 {
    #[inline(always)]
    fn from(val: CTSEN) -> u8 {
        CTSEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EPS {
    #[doc = "Odd parity: The UART generates or checks for an odd number of 1s in the data and parity bits."]
    ODD = 0x0,
    #[doc = "Even parity: The UART generates or checks for an even number of 1s in the data and parity bits."]
    EVEN = 0x01,
}
impl EPS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EPS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EPS {
    #[inline(always)]
    fn from(val: u8) -> EPS {
        EPS::from_bits(val)
    }
}
impl From<EPS> for u8 {
    #[inline(always)]
    fn from(val: EPS) -> u8 {
        EPS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FEN {
    #[doc = "FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers."]
    DIS = 0x0,
    #[doc = "Transmit and receive FIFO buffers are enabled (FIFO mode)."]
    EN = 0x01,
}
impl FEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FEN {
    #[inline(always)]
    fn from(val: u8) -> FEN {
        FEN::from_bits(val)
    }
}
impl From<FEN> for u8 {
    #[inline(always)]
    fn from(val: FEN) -> u8 {
        FEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LBE {
    #[doc = "Loop Back disabled."]
    DIS = 0x0,
    #[doc = "Loop Back enabled."]
    EN = 0x01,
}
impl LBE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LBE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LBE {
    #[inline(always)]
    fn from(val: u8) -> LBE {
        LBE::from_bits(val)
    }
}
impl From<LBE> for u8 {
    #[inline(always)]
    fn from(val: LBE) -> u8 {
        LBE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PEN {
    #[doc = "Parity is disabled and no parity bit is added to the data frame."]
    DIS = 0x0,
    #[doc = "Parity checking and generation is enabled."]
    EN = 0x01,
}
impl PEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PEN {
    #[inline(always)]
    fn from(val: u8) -> PEN {
        PEN::from_bits(val)
    }
}
impl From<PEN> for u8 {
    #[inline(always)]
    fn from(val: PEN) -> u8 {
        PEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RTSEN {
    #[doc = "RTS hardware flow control disabled."]
    DIS = 0x0,
    #[doc = "RTS hardware flow control enabled."]
    EN = 0x01,
}
impl RTSEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RTSEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RTSEN {
    #[inline(always)]
    fn from(val: u8) -> RTSEN {
        RTSEN::from_bits(val)
    }
}
impl From<RTSEN> for u8 {
    #[inline(always)]
    fn from(val: RTSEN) -> u8 {
        RTSEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RXE {
    #[doc = "UART Receive disabled."]
    DIS = 0x0,
    #[doc = "UART Receive enabled."]
    EN = 0x01,
}
impl RXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RXE {
    #[inline(always)]
    fn from(val: u8) -> RXE {
        RXE::from_bits(val)
    }
}
impl From<RXE> for u8 {
    #[inline(always)]
    fn from(val: RXE) -> u8 {
        RXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RXSEL {
    #[doc = "Receive FIFO becomes >= 1/8 full."]
    _1_8 = 0x0,
    #[doc = "Receive FIFO becomes >= 1/4 full."]
    _2_8 = 0x01,
    #[doc = "Receive FIFO becomes >= 1/2 full."]
    _4_8 = 0x02,
    #[doc = "Receive FIFO becomes >= 3/4 full."]
    _6_8 = 0x03,
    #[doc = "Receive FIFO becomes >= 7/8 full."]
    _7_8 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RXSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RXSEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RXSEL {
    #[inline(always)]
    fn from(val: u8) -> RXSEL {
        RXSEL::from_bits(val)
    }
}
impl From<RXSEL> for u8 {
    #[inline(always)]
    fn from(val: RXSEL) -> u8 {
        RXSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXE {
    #[doc = "UART Transmit disabled."]
    DIS = 0x0,
    #[doc = "UART Transmit enabled."]
    EN = 0x01,
}
impl TXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXE {
    #[inline(always)]
    fn from(val: u8) -> TXE {
        TXE::from_bits(val)
    }
}
impl From<TXE> for u8 {
    #[inline(always)]
    fn from(val: TXE) -> u8 {
        TXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXSEL {
    #[doc = "Transmit FIFO becomes <= 1/8 full."]
    _1_8 = 0x0,
    #[doc = "Transmit FIFO becomes <= 1/4 full."]
    _2_8 = 0x01,
    #[doc = "Transmit FIFO becomes <= 1/2 full."]
    _4_8 = 0x02,
    #[doc = "Transmit FIFO becomes <= 3/4 full."]
    _6_8 = 0x03,
    #[doc = "Transmit FIFO becomes <= 7/8 full."]
    _7_8 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl TXSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXSEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXSEL {
    #[inline(always)]
    fn from(val: u8) -> TXSEL {
        TXSEL::from_bits(val)
    }
}
impl From<TXSEL> for u8 {
    #[inline(always)]
    fn from(val: TXSEL) -> u8 {
        TXSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UARTEN {
    #[doc = "UART disabled."]
    DIS = 0x0,
    #[doc = "UART enabled."]
    EN = 0x01,
}
impl UARTEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UARTEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UARTEN {
    #[inline(always)]
    fn from(val: u8) -> UARTEN {
        UARTEN::from_bits(val)
    }
}
impl From<UARTEN> for u8 {
    #[inline(always)]
    fn from(val: UARTEN) -> u8 {
        UARTEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WLEN {
    #[doc = "Word Length 5 bits."]
    _5 = 0x0,
    #[doc = "Word Length 6 bits."]
    _6 = 0x01,
    #[doc = "Word Length 7 bits."]
    _7 = 0x02,
    #[doc = "Word Length 8 bits."]
    _8 = 0x03,
}
impl WLEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WLEN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WLEN {
    #[inline(always)]
    fn from(val: u8) -> WLEN {
        WLEN::from_bits(val)
    }
}
impl From<WLEN> for u8 {
    #[inline(always)]
    fn from(val: WLEN) -> u8 {
        WLEN::to_bits(val)
    }
}

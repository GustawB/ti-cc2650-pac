#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG0_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG0_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG0_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG0_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG0_EDGE_DET {
        IOCFG0_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG0_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG0_EDGE_DET) -> u8 {
        IOCFG0_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG0_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG0_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG0_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG0_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG0_IOCURR {
        IOCFG0_IOCURR::from_bits(val)
    }
}
impl From<IOCFG0_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG0_IOCURR) -> u8 {
        IOCFG0_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG0_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / outut."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input/output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG0_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG0_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG0_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG0_IOMODE {
        IOCFG0_IOMODE::from_bits(val)
    }
}
impl From<IOCFG0_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG0_IOMODE) -> u8 {
        IOCFG0_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG0_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG0_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG0_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG0_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG0_IOSTR {
        IOCFG0_IOSTR::from_bits(val)
    }
}
impl From<IOCFG0_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG0_IOSTR) -> u8 {
        IOCFG0_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG0_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG0_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG0_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG0_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG0_PORT_ID {
        IOCFG0_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG0_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG0_PORT_ID) -> u8 {
        IOCFG0_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG0_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG0_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG0_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG0_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG0_PULL_CTL {
        IOCFG0_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG0_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG0_PULL_CTL) -> u8 {
        IOCFG0_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG10_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG10_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG10_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG10_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG10_EDGE_DET {
        IOCFG10_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG10_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG10_EDGE_DET) -> u8 {
        IOCFG10_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG10_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG10_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG10_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG10_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG10_IOCURR {
        IOCFG10_IOCURR::from_bits(val)
    }
}
impl From<IOCFG10_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG10_IOCURR) -> u8 {
        IOCFG10_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG10_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG10_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG10_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG10_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG10_IOMODE {
        IOCFG10_IOMODE::from_bits(val)
    }
}
impl From<IOCFG10_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG10_IOMODE) -> u8 {
        IOCFG10_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG10_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG10_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG10_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG10_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG10_IOSTR {
        IOCFG10_IOSTR::from_bits(val)
    }
}
impl From<IOCFG10_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG10_IOSTR) -> u8 {
        IOCFG10_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG10_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG10_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG10_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG10_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG10_PORT_ID {
        IOCFG10_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG10_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG10_PORT_ID) -> u8 {
        IOCFG10_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG10_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG10_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG10_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG10_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG10_PULL_CTL {
        IOCFG10_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG10_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG10_PULL_CTL) -> u8 {
        IOCFG10_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG11_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG11_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG11_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG11_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG11_EDGE_DET {
        IOCFG11_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG11_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG11_EDGE_DET) -> u8 {
        IOCFG11_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG11_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG11_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG11_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG11_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG11_IOCURR {
        IOCFG11_IOCURR::from_bits(val)
    }
}
impl From<IOCFG11_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG11_IOCURR) -> u8 {
        IOCFG11_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG11_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG11_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG11_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG11_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG11_IOMODE {
        IOCFG11_IOMODE::from_bits(val)
    }
}
impl From<IOCFG11_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG11_IOMODE) -> u8 {
        IOCFG11_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG11_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG11_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG11_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG11_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG11_IOSTR {
        IOCFG11_IOSTR::from_bits(val)
    }
}
impl From<IOCFG11_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG11_IOSTR) -> u8 {
        IOCFG11_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG11_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG11_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG11_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG11_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG11_PORT_ID {
        IOCFG11_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG11_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG11_PORT_ID) -> u8 {
        IOCFG11_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG11_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG11_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG11_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG11_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG11_PULL_CTL {
        IOCFG11_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG11_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG11_PULL_CTL) -> u8 {
        IOCFG11_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG12_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG12_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG12_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG12_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG12_EDGE_DET {
        IOCFG12_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG12_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG12_EDGE_DET) -> u8 {
        IOCFG12_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG12_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG12_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG12_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG12_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG12_IOCURR {
        IOCFG12_IOCURR::from_bits(val)
    }
}
impl From<IOCFG12_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG12_IOCURR) -> u8 {
        IOCFG12_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG12_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG12_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG12_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG12_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG12_IOMODE {
        IOCFG12_IOMODE::from_bits(val)
    }
}
impl From<IOCFG12_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG12_IOMODE) -> u8 {
        IOCFG12_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG12_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG12_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG12_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG12_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG12_IOSTR {
        IOCFG12_IOSTR::from_bits(val)
    }
}
impl From<IOCFG12_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG12_IOSTR) -> u8 {
        IOCFG12_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG12_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG12_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG12_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG12_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG12_PORT_ID {
        IOCFG12_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG12_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG12_PORT_ID) -> u8 {
        IOCFG12_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG12_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG12_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG12_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG12_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG12_PULL_CTL {
        IOCFG12_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG12_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG12_PULL_CTL) -> u8 {
        IOCFG12_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG13_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG13_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG13_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG13_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG13_EDGE_DET {
        IOCFG13_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG13_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG13_EDGE_DET) -> u8 {
        IOCFG13_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG13_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG13_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG13_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG13_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG13_IOCURR {
        IOCFG13_IOCURR::from_bits(val)
    }
}
impl From<IOCFG13_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG13_IOCURR) -> u8 {
        IOCFG13_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG13_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG13_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG13_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG13_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG13_IOMODE {
        IOCFG13_IOMODE::from_bits(val)
    }
}
impl From<IOCFG13_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG13_IOMODE) -> u8 {
        IOCFG13_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG13_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG13_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG13_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG13_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG13_IOSTR {
        IOCFG13_IOSTR::from_bits(val)
    }
}
impl From<IOCFG13_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG13_IOSTR) -> u8 {
        IOCFG13_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG13_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG13_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG13_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG13_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG13_PORT_ID {
        IOCFG13_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG13_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG13_PORT_ID) -> u8 {
        IOCFG13_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG13_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG13_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG13_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG13_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG13_PULL_CTL {
        IOCFG13_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG13_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG13_PULL_CTL) -> u8 {
        IOCFG13_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG14_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG14_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG14_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG14_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG14_EDGE_DET {
        IOCFG14_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG14_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG14_EDGE_DET) -> u8 {
        IOCFG14_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG14_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG14_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG14_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG14_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG14_IOCURR {
        IOCFG14_IOCURR::from_bits(val)
    }
}
impl From<IOCFG14_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG14_IOCURR) -> u8 {
        IOCFG14_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG14_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG14_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG14_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG14_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG14_IOMODE {
        IOCFG14_IOMODE::from_bits(val)
    }
}
impl From<IOCFG14_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG14_IOMODE) -> u8 {
        IOCFG14_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG14_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG14_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG14_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG14_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG14_IOSTR {
        IOCFG14_IOSTR::from_bits(val)
    }
}
impl From<IOCFG14_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG14_IOSTR) -> u8 {
        IOCFG14_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG14_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG14_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG14_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG14_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG14_PORT_ID {
        IOCFG14_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG14_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG14_PORT_ID) -> u8 {
        IOCFG14_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG14_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG14_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG14_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG14_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG14_PULL_CTL {
        IOCFG14_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG14_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG14_PULL_CTL) -> u8 {
        IOCFG14_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG15_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG15_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG15_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG15_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG15_EDGE_DET {
        IOCFG15_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG15_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG15_EDGE_DET) -> u8 {
        IOCFG15_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG15_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG15_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG15_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG15_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG15_IOCURR {
        IOCFG15_IOCURR::from_bits(val)
    }
}
impl From<IOCFG15_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG15_IOCURR) -> u8 {
        IOCFG15_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG15_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG15_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG15_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG15_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG15_IOMODE {
        IOCFG15_IOMODE::from_bits(val)
    }
}
impl From<IOCFG15_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG15_IOMODE) -> u8 {
        IOCFG15_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG15_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG15_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG15_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG15_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG15_IOSTR {
        IOCFG15_IOSTR::from_bits(val)
    }
}
impl From<IOCFG15_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG15_IOSTR) -> u8 {
        IOCFG15_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG15_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG15_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG15_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG15_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG15_PORT_ID {
        IOCFG15_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG15_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG15_PORT_ID) -> u8 {
        IOCFG15_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG15_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG15_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG15_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG15_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG15_PULL_CTL {
        IOCFG15_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG15_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG15_PULL_CTL) -> u8 {
        IOCFG15_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG16_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG16_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG16_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG16_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG16_EDGE_DET {
        IOCFG16_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG16_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG16_EDGE_DET) -> u8 {
        IOCFG16_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG16_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG16_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG16_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG16_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG16_IOCURR {
        IOCFG16_IOCURR::from_bits(val)
    }
}
impl From<IOCFG16_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG16_IOCURR) -> u8 {
        IOCFG16_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG16_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG16_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG16_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG16_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG16_IOMODE {
        IOCFG16_IOMODE::from_bits(val)
    }
}
impl From<IOCFG16_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG16_IOMODE) -> u8 {
        IOCFG16_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG16_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG16_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG16_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG16_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG16_IOSTR {
        IOCFG16_IOSTR::from_bits(val)
    }
}
impl From<IOCFG16_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG16_IOSTR) -> u8 {
        IOCFG16_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG16_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG16_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG16_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG16_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG16_PORT_ID {
        IOCFG16_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG16_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG16_PORT_ID) -> u8 {
        IOCFG16_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG16_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG16_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG16_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG16_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG16_PULL_CTL {
        IOCFG16_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG16_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG16_PULL_CTL) -> u8 {
        IOCFG16_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG17_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG17_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG17_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG17_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG17_EDGE_DET {
        IOCFG17_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG17_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG17_EDGE_DET) -> u8 {
        IOCFG17_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG17_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG17_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG17_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG17_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG17_IOCURR {
        IOCFG17_IOCURR::from_bits(val)
    }
}
impl From<IOCFG17_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG17_IOCURR) -> u8 {
        IOCFG17_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG17_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG17_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG17_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG17_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG17_IOMODE {
        IOCFG17_IOMODE::from_bits(val)
    }
}
impl From<IOCFG17_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG17_IOMODE) -> u8 {
        IOCFG17_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG17_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG17_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG17_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG17_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG17_IOSTR {
        IOCFG17_IOSTR::from_bits(val)
    }
}
impl From<IOCFG17_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG17_IOSTR) -> u8 {
        IOCFG17_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG17_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG17_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG17_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG17_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG17_PORT_ID {
        IOCFG17_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG17_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG17_PORT_ID) -> u8 {
        IOCFG17_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG17_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG17_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG17_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG17_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG17_PULL_CTL {
        IOCFG17_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG17_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG17_PULL_CTL) -> u8 {
        IOCFG17_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG18_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG18_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG18_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG18_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG18_EDGE_DET {
        IOCFG18_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG18_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG18_EDGE_DET) -> u8 {
        IOCFG18_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG18_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG18_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG18_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG18_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG18_IOCURR {
        IOCFG18_IOCURR::from_bits(val)
    }
}
impl From<IOCFG18_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG18_IOCURR) -> u8 {
        IOCFG18_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG18_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG18_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG18_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG18_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG18_IOMODE {
        IOCFG18_IOMODE::from_bits(val)
    }
}
impl From<IOCFG18_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG18_IOMODE) -> u8 {
        IOCFG18_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG18_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG18_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG18_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG18_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG18_IOSTR {
        IOCFG18_IOSTR::from_bits(val)
    }
}
impl From<IOCFG18_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG18_IOSTR) -> u8 {
        IOCFG18_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG18_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG18_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG18_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG18_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG18_PORT_ID {
        IOCFG18_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG18_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG18_PORT_ID) -> u8 {
        IOCFG18_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG18_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG18_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG18_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG18_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG18_PULL_CTL {
        IOCFG18_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG18_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG18_PULL_CTL) -> u8 {
        IOCFG18_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG19_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG19_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG19_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG19_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG19_EDGE_DET {
        IOCFG19_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG19_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG19_EDGE_DET) -> u8 {
        IOCFG19_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG19_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG19_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG19_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG19_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG19_IOCURR {
        IOCFG19_IOCURR::from_bits(val)
    }
}
impl From<IOCFG19_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG19_IOCURR) -> u8 {
        IOCFG19_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG19_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG19_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG19_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG19_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG19_IOMODE {
        IOCFG19_IOMODE::from_bits(val)
    }
}
impl From<IOCFG19_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG19_IOMODE) -> u8 {
        IOCFG19_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG19_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG19_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG19_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG19_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG19_IOSTR {
        IOCFG19_IOSTR::from_bits(val)
    }
}
impl From<IOCFG19_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG19_IOSTR) -> u8 {
        IOCFG19_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG19_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG19_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG19_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG19_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG19_PORT_ID {
        IOCFG19_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG19_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG19_PORT_ID) -> u8 {
        IOCFG19_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG19_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG19_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG19_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG19_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG19_PULL_CTL {
        IOCFG19_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG19_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG19_PULL_CTL) -> u8 {
        IOCFG19_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG1_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG1_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG1_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG1_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG1_EDGE_DET {
        IOCFG1_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG1_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG1_EDGE_DET) -> u8 {
        IOCFG1_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG1_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG1_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG1_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG1_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG1_IOCURR {
        IOCFG1_IOCURR::from_bits(val)
    }
}
impl From<IOCFG1_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG1_IOCURR) -> u8 {
        IOCFG1_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG1_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG1_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG1_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG1_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG1_IOMODE {
        IOCFG1_IOMODE::from_bits(val)
    }
}
impl From<IOCFG1_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG1_IOMODE) -> u8 {
        IOCFG1_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG1_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG1_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG1_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG1_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG1_IOSTR {
        IOCFG1_IOSTR::from_bits(val)
    }
}
impl From<IOCFG1_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG1_IOSTR) -> u8 {
        IOCFG1_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG1_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG1_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG1_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG1_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG1_PORT_ID {
        IOCFG1_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG1_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG1_PORT_ID) -> u8 {
        IOCFG1_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG1_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG1_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG1_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG1_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG1_PULL_CTL {
        IOCFG1_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG1_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG1_PULL_CTL) -> u8 {
        IOCFG1_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG20_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG20_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG20_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG20_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG20_EDGE_DET {
        IOCFG20_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG20_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG20_EDGE_DET) -> u8 {
        IOCFG20_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG20_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG20_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG20_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG20_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG20_IOCURR {
        IOCFG20_IOCURR::from_bits(val)
    }
}
impl From<IOCFG20_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG20_IOCURR) -> u8 {
        IOCFG20_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG20_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG20_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG20_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG20_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG20_IOMODE {
        IOCFG20_IOMODE::from_bits(val)
    }
}
impl From<IOCFG20_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG20_IOMODE) -> u8 {
        IOCFG20_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG20_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG20_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG20_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG20_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG20_IOSTR {
        IOCFG20_IOSTR::from_bits(val)
    }
}
impl From<IOCFG20_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG20_IOSTR) -> u8 {
        IOCFG20_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG20_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG20_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG20_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG20_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG20_PORT_ID {
        IOCFG20_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG20_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG20_PORT_ID) -> u8 {
        IOCFG20_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG20_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG20_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG20_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG20_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG20_PULL_CTL {
        IOCFG20_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG20_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG20_PULL_CTL) -> u8 {
        IOCFG20_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG21_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG21_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG21_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG21_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG21_EDGE_DET {
        IOCFG21_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG21_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG21_EDGE_DET) -> u8 {
        IOCFG21_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG21_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG21_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG21_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG21_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG21_IOCURR {
        IOCFG21_IOCURR::from_bits(val)
    }
}
impl From<IOCFG21_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG21_IOCURR) -> u8 {
        IOCFG21_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG21_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG21_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG21_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG21_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG21_IOMODE {
        IOCFG21_IOMODE::from_bits(val)
    }
}
impl From<IOCFG21_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG21_IOMODE) -> u8 {
        IOCFG21_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG21_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG21_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG21_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG21_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG21_IOSTR {
        IOCFG21_IOSTR::from_bits(val)
    }
}
impl From<IOCFG21_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG21_IOSTR) -> u8 {
        IOCFG21_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG21_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG21_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG21_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG21_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG21_PORT_ID {
        IOCFG21_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG21_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG21_PORT_ID) -> u8 {
        IOCFG21_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG21_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG21_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG21_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG21_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG21_PULL_CTL {
        IOCFG21_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG21_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG21_PULL_CTL) -> u8 {
        IOCFG21_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG22_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG22_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG22_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG22_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG22_EDGE_DET {
        IOCFG22_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG22_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG22_EDGE_DET) -> u8 {
        IOCFG22_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG22_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG22_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG22_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG22_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG22_IOCURR {
        IOCFG22_IOCURR::from_bits(val)
    }
}
impl From<IOCFG22_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG22_IOCURR) -> u8 {
        IOCFG22_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG22_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG22_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG22_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG22_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG22_IOMODE {
        IOCFG22_IOMODE::from_bits(val)
    }
}
impl From<IOCFG22_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG22_IOMODE) -> u8 {
        IOCFG22_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG22_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG22_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG22_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG22_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG22_IOSTR {
        IOCFG22_IOSTR::from_bits(val)
    }
}
impl From<IOCFG22_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG22_IOSTR) -> u8 {
        IOCFG22_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG22_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG22_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG22_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG22_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG22_PORT_ID {
        IOCFG22_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG22_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG22_PORT_ID) -> u8 {
        IOCFG22_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG22_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG22_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG22_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG22_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG22_PULL_CTL {
        IOCFG22_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG22_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG22_PULL_CTL) -> u8 {
        IOCFG22_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG23_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG23_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG23_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG23_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG23_EDGE_DET {
        IOCFG23_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG23_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG23_EDGE_DET) -> u8 {
        IOCFG23_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG23_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG23_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG23_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG23_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG23_IOCURR {
        IOCFG23_IOCURR::from_bits(val)
    }
}
impl From<IOCFG23_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG23_IOCURR) -> u8 {
        IOCFG23_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG23_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG23_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG23_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG23_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG23_IOMODE {
        IOCFG23_IOMODE::from_bits(val)
    }
}
impl From<IOCFG23_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG23_IOMODE) -> u8 {
        IOCFG23_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG23_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG23_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG23_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG23_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG23_IOSTR {
        IOCFG23_IOSTR::from_bits(val)
    }
}
impl From<IOCFG23_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG23_IOSTR) -> u8 {
        IOCFG23_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG23_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG23_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG23_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG23_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG23_PORT_ID {
        IOCFG23_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG23_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG23_PORT_ID) -> u8 {
        IOCFG23_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG23_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG23_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG23_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG23_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG23_PULL_CTL {
        IOCFG23_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG23_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG23_PULL_CTL) -> u8 {
        IOCFG23_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG24_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG24_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG24_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG24_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG24_EDGE_DET {
        IOCFG24_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG24_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG24_EDGE_DET) -> u8 {
        IOCFG24_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG24_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG24_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG24_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG24_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG24_IOCURR {
        IOCFG24_IOCURR::from_bits(val)
    }
}
impl From<IOCFG24_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG24_IOCURR) -> u8 {
        IOCFG24_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG24_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG24_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG24_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG24_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG24_IOMODE {
        IOCFG24_IOMODE::from_bits(val)
    }
}
impl From<IOCFG24_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG24_IOMODE) -> u8 {
        IOCFG24_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG24_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG24_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG24_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG24_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG24_IOSTR {
        IOCFG24_IOSTR::from_bits(val)
    }
}
impl From<IOCFG24_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG24_IOSTR) -> u8 {
        IOCFG24_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG24_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG24_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG24_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG24_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG24_PORT_ID {
        IOCFG24_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG24_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG24_PORT_ID) -> u8 {
        IOCFG24_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG24_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG24_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG24_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG24_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG24_PULL_CTL {
        IOCFG24_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG24_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG24_PULL_CTL) -> u8 {
        IOCFG24_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG25_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG25_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG25_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG25_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG25_EDGE_DET {
        IOCFG25_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG25_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG25_EDGE_DET) -> u8 {
        IOCFG25_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG25_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG25_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG25_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG25_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG25_IOCURR {
        IOCFG25_IOCURR::from_bits(val)
    }
}
impl From<IOCFG25_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG25_IOCURR) -> u8 {
        IOCFG25_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG25_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG25_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG25_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG25_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG25_IOMODE {
        IOCFG25_IOMODE::from_bits(val)
    }
}
impl From<IOCFG25_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG25_IOMODE) -> u8 {
        IOCFG25_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG25_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG25_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG25_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG25_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG25_IOSTR {
        IOCFG25_IOSTR::from_bits(val)
    }
}
impl From<IOCFG25_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG25_IOSTR) -> u8 {
        IOCFG25_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG25_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG25_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG25_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG25_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG25_PORT_ID {
        IOCFG25_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG25_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG25_PORT_ID) -> u8 {
        IOCFG25_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG25_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG25_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG25_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG25_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG25_PULL_CTL {
        IOCFG25_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG25_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG25_PULL_CTL) -> u8 {
        IOCFG25_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG26_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG26_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG26_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG26_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG26_EDGE_DET {
        IOCFG26_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG26_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG26_EDGE_DET) -> u8 {
        IOCFG26_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG26_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG26_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG26_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG26_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG26_IOCURR {
        IOCFG26_IOCURR::from_bits(val)
    }
}
impl From<IOCFG26_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG26_IOCURR) -> u8 {
        IOCFG26_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG26_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG26_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG26_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG26_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG26_IOMODE {
        IOCFG26_IOMODE::from_bits(val)
    }
}
impl From<IOCFG26_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG26_IOMODE) -> u8 {
        IOCFG26_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG26_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG26_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG26_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG26_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG26_IOSTR {
        IOCFG26_IOSTR::from_bits(val)
    }
}
impl From<IOCFG26_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG26_IOSTR) -> u8 {
        IOCFG26_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG26_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG26_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG26_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG26_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG26_PORT_ID {
        IOCFG26_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG26_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG26_PORT_ID) -> u8 {
        IOCFG26_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG26_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG26_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG26_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG26_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG26_PULL_CTL {
        IOCFG26_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG26_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG26_PULL_CTL) -> u8 {
        IOCFG26_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG27_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG27_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG27_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG27_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG27_EDGE_DET {
        IOCFG27_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG27_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG27_EDGE_DET) -> u8 {
        IOCFG27_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG27_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG27_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG27_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG27_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG27_IOCURR {
        IOCFG27_IOCURR::from_bits(val)
    }
}
impl From<IOCFG27_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG27_IOCURR) -> u8 {
        IOCFG27_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG27_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG27_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG27_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG27_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG27_IOMODE {
        IOCFG27_IOMODE::from_bits(val)
    }
}
impl From<IOCFG27_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG27_IOMODE) -> u8 {
        IOCFG27_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG27_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG27_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG27_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG27_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG27_IOSTR {
        IOCFG27_IOSTR::from_bits(val)
    }
}
impl From<IOCFG27_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG27_IOSTR) -> u8 {
        IOCFG27_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG27_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG27_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG27_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG27_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG27_PORT_ID {
        IOCFG27_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG27_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG27_PORT_ID) -> u8 {
        IOCFG27_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG27_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG27_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG27_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG27_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG27_PULL_CTL {
        IOCFG27_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG27_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG27_PULL_CTL) -> u8 {
        IOCFG27_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG28_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG28_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG28_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG28_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG28_EDGE_DET {
        IOCFG28_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG28_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG28_EDGE_DET) -> u8 {
        IOCFG28_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG28_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG28_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG28_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG28_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG28_IOCURR {
        IOCFG28_IOCURR::from_bits(val)
    }
}
impl From<IOCFG28_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG28_IOCURR) -> u8 {
        IOCFG28_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG28_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG28_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG28_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG28_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG28_IOMODE {
        IOCFG28_IOMODE::from_bits(val)
    }
}
impl From<IOCFG28_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG28_IOMODE) -> u8 {
        IOCFG28_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG28_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG28_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG28_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG28_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG28_IOSTR {
        IOCFG28_IOSTR::from_bits(val)
    }
}
impl From<IOCFG28_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG28_IOSTR) -> u8 {
        IOCFG28_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG28_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG28_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG28_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG28_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG28_PORT_ID {
        IOCFG28_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG28_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG28_PORT_ID) -> u8 {
        IOCFG28_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG28_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG28_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG28_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG28_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG28_PULL_CTL {
        IOCFG28_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG28_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG28_PULL_CTL) -> u8 {
        IOCFG28_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG29_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG29_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG29_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG29_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG29_EDGE_DET {
        IOCFG29_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG29_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG29_EDGE_DET) -> u8 {
        IOCFG29_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG29_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG29_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG29_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG29_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG29_IOCURR {
        IOCFG29_IOCURR::from_bits(val)
    }
}
impl From<IOCFG29_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG29_IOCURR) -> u8 {
        IOCFG29_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG29_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG29_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG29_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG29_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG29_IOMODE {
        IOCFG29_IOMODE::from_bits(val)
    }
}
impl From<IOCFG29_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG29_IOMODE) -> u8 {
        IOCFG29_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG29_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG29_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG29_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG29_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG29_IOSTR {
        IOCFG29_IOSTR::from_bits(val)
    }
}
impl From<IOCFG29_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG29_IOSTR) -> u8 {
        IOCFG29_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG29_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG29_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG29_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG29_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG29_PORT_ID {
        IOCFG29_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG29_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG29_PORT_ID) -> u8 {
        IOCFG29_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG29_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG29_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG29_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG29_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG29_PULL_CTL {
        IOCFG29_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG29_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG29_PULL_CTL) -> u8 {
        IOCFG29_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG2_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG2_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG2_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG2_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG2_EDGE_DET {
        IOCFG2_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG2_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG2_EDGE_DET) -> u8 {
        IOCFG2_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG2_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG2_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG2_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG2_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG2_IOCURR {
        IOCFG2_IOCURR::from_bits(val)
    }
}
impl From<IOCFG2_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG2_IOCURR) -> u8 {
        IOCFG2_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG2_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG2_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG2_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG2_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG2_IOMODE {
        IOCFG2_IOMODE::from_bits(val)
    }
}
impl From<IOCFG2_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG2_IOMODE) -> u8 {
        IOCFG2_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG2_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG2_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG2_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG2_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG2_IOSTR {
        IOCFG2_IOSTR::from_bits(val)
    }
}
impl From<IOCFG2_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG2_IOSTR) -> u8 {
        IOCFG2_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG2_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG2_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG2_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG2_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG2_PORT_ID {
        IOCFG2_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG2_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG2_PORT_ID) -> u8 {
        IOCFG2_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG2_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG2_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG2_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG2_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG2_PULL_CTL {
        IOCFG2_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG2_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG2_PULL_CTL) -> u8 {
        IOCFG2_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG30_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG30_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG30_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG30_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG30_EDGE_DET {
        IOCFG30_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG30_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG30_EDGE_DET) -> u8 {
        IOCFG30_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG30_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG30_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG30_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG30_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG30_IOCURR {
        IOCFG30_IOCURR::from_bits(val)
    }
}
impl From<IOCFG30_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG30_IOCURR) -> u8 {
        IOCFG30_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG30_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG30_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG30_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG30_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG30_IOMODE {
        IOCFG30_IOMODE::from_bits(val)
    }
}
impl From<IOCFG30_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG30_IOMODE) -> u8 {
        IOCFG30_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG30_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG30_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG30_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG30_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG30_IOSTR {
        IOCFG30_IOSTR::from_bits(val)
    }
}
impl From<IOCFG30_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG30_IOSTR) -> u8 {
        IOCFG30_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG30_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG30_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG30_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG30_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG30_PORT_ID {
        IOCFG30_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG30_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG30_PORT_ID) -> u8 {
        IOCFG30_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG30_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG30_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG30_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG30_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG30_PULL_CTL {
        IOCFG30_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG30_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG30_PULL_CTL) -> u8 {
        IOCFG30_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG31_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG31_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG31_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG31_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG31_EDGE_DET {
        IOCFG31_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG31_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG31_EDGE_DET) -> u8 {
        IOCFG31_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG31_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG31_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG31_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG31_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG31_IOCURR {
        IOCFG31_IOCURR::from_bits(val)
    }
}
impl From<IOCFG31_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG31_IOCURR) -> u8 {
        IOCFG31_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG31_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG31_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG31_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG31_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG31_IOMODE {
        IOCFG31_IOMODE::from_bits(val)
    }
}
impl From<IOCFG31_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG31_IOMODE) -> u8 {
        IOCFG31_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG31_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG31_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG31_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG31_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG31_IOSTR {
        IOCFG31_IOSTR::from_bits(val)
    }
}
impl From<IOCFG31_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG31_IOSTR) -> u8 {
        IOCFG31_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG31_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG31_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG31_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG31_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG31_PORT_ID {
        IOCFG31_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG31_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG31_PORT_ID) -> u8 {
        IOCFG31_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG31_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG31_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG31_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG31_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG31_PULL_CTL {
        IOCFG31_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG31_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG31_PULL_CTL) -> u8 {
        IOCFG31_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG3_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG3_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG3_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG3_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG3_EDGE_DET {
        IOCFG3_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG3_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG3_EDGE_DET) -> u8 {
        IOCFG3_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG3_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG3_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG3_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG3_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG3_IOCURR {
        IOCFG3_IOCURR::from_bits(val)
    }
}
impl From<IOCFG3_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG3_IOCURR) -> u8 {
        IOCFG3_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG3_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG3_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG3_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG3_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG3_IOMODE {
        IOCFG3_IOMODE::from_bits(val)
    }
}
impl From<IOCFG3_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG3_IOMODE) -> u8 {
        IOCFG3_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG3_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG3_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG3_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG3_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG3_IOSTR {
        IOCFG3_IOSTR::from_bits(val)
    }
}
impl From<IOCFG3_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG3_IOSTR) -> u8 {
        IOCFG3_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG3_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG3_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG3_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG3_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG3_PORT_ID {
        IOCFG3_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG3_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG3_PORT_ID) -> u8 {
        IOCFG3_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG3_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG3_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG3_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG3_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG3_PULL_CTL {
        IOCFG3_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG3_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG3_PULL_CTL) -> u8 {
        IOCFG3_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG4_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG4_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG4_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG4_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG4_EDGE_DET {
        IOCFG4_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG4_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG4_EDGE_DET) -> u8 {
        IOCFG4_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG4_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG4_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG4_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG4_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG4_IOCURR {
        IOCFG4_IOCURR::from_bits(val)
    }
}
impl From<IOCFG4_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG4_IOCURR) -> u8 {
        IOCFG4_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG4_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG4_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG4_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG4_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG4_IOMODE {
        IOCFG4_IOMODE::from_bits(val)
    }
}
impl From<IOCFG4_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG4_IOMODE) -> u8 {
        IOCFG4_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG4_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG4_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG4_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG4_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG4_IOSTR {
        IOCFG4_IOSTR::from_bits(val)
    }
}
impl From<IOCFG4_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG4_IOSTR) -> u8 {
        IOCFG4_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG4_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG4_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG4_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG4_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG4_PORT_ID {
        IOCFG4_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG4_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG4_PORT_ID) -> u8 {
        IOCFG4_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG4_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG4_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG4_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG4_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG4_PULL_CTL {
        IOCFG4_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG4_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG4_PULL_CTL) -> u8 {
        IOCFG4_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG5_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG5_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG5_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG5_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG5_EDGE_DET {
        IOCFG5_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG5_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG5_EDGE_DET) -> u8 {
        IOCFG5_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG5_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG5_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG5_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG5_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG5_IOCURR {
        IOCFG5_IOCURR::from_bits(val)
    }
}
impl From<IOCFG5_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG5_IOCURR) -> u8 {
        IOCFG5_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG5_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG5_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG5_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG5_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG5_IOMODE {
        IOCFG5_IOMODE::from_bits(val)
    }
}
impl From<IOCFG5_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG5_IOMODE) -> u8 {
        IOCFG5_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG5_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG5_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG5_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG5_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG5_IOSTR {
        IOCFG5_IOSTR::from_bits(val)
    }
}
impl From<IOCFG5_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG5_IOSTR) -> u8 {
        IOCFG5_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG5_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG5_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG5_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG5_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG5_PORT_ID {
        IOCFG5_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG5_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG5_PORT_ID) -> u8 {
        IOCFG5_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG5_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG5_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG5_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG5_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG5_PULL_CTL {
        IOCFG5_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG5_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG5_PULL_CTL) -> u8 {
        IOCFG5_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG6_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG6_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG6_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG6_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG6_EDGE_DET {
        IOCFG6_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG6_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG6_EDGE_DET) -> u8 {
        IOCFG6_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG6_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG6_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG6_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG6_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG6_IOCURR {
        IOCFG6_IOCURR::from_bits(val)
    }
}
impl From<IOCFG6_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG6_IOCURR) -> u8 {
        IOCFG6_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG6_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG6_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG6_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG6_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG6_IOMODE {
        IOCFG6_IOMODE::from_bits(val)
    }
}
impl From<IOCFG6_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG6_IOMODE) -> u8 {
        IOCFG6_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG6_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG6_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG6_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG6_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG6_IOSTR {
        IOCFG6_IOSTR::from_bits(val)
    }
}
impl From<IOCFG6_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG6_IOSTR) -> u8 {
        IOCFG6_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG6_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG6_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG6_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG6_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG6_PORT_ID {
        IOCFG6_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG6_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG6_PORT_ID) -> u8 {
        IOCFG6_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG6_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG6_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG6_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG6_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG6_PULL_CTL {
        IOCFG6_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG6_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG6_PULL_CTL) -> u8 {
        IOCFG6_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG7_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG7_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG7_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG7_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG7_EDGE_DET {
        IOCFG7_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG7_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG7_EDGE_DET) -> u8 {
        IOCFG7_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG7_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG7_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG7_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG7_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG7_IOCURR {
        IOCFG7_IOCURR::from_bits(val)
    }
}
impl From<IOCFG7_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG7_IOCURR) -> u8 {
        IOCFG7_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG7_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG7_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG7_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG7_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG7_IOMODE {
        IOCFG7_IOMODE::from_bits(val)
    }
}
impl From<IOCFG7_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG7_IOMODE) -> u8 {
        IOCFG7_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG7_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG7_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG7_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG7_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG7_IOSTR {
        IOCFG7_IOSTR::from_bits(val)
    }
}
impl From<IOCFG7_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG7_IOSTR) -> u8 {
        IOCFG7_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG7_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG7_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG7_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG7_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG7_PORT_ID {
        IOCFG7_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG7_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG7_PORT_ID) -> u8 {
        IOCFG7_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG7_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG7_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG7_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG7_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG7_PULL_CTL {
        IOCFG7_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG7_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG7_PULL_CTL) -> u8 {
        IOCFG7_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG8_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG8_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG8_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG8_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG8_EDGE_DET {
        IOCFG8_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG8_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG8_EDGE_DET) -> u8 {
        IOCFG8_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG8_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG8_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG8_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG8_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG8_IOCURR {
        IOCFG8_IOCURR::from_bits(val)
    }
}
impl From<IOCFG8_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG8_IOCURR) -> u8 {
        IOCFG8_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG8_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG8_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG8_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG8_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG8_IOMODE {
        IOCFG8_IOMODE::from_bits(val)
    }
}
impl From<IOCFG8_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG8_IOMODE) -> u8 {
        IOCFG8_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG8_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG8_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG8_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG8_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG8_IOSTR {
        IOCFG8_IOSTR::from_bits(val)
    }
}
impl From<IOCFG8_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG8_IOSTR) -> u8 {
        IOCFG8_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG8_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG8_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG8_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG8_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG8_PORT_ID {
        IOCFG8_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG8_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG8_PORT_ID) -> u8 {
        IOCFG8_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG8_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG8_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG8_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG8_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG8_PULL_CTL {
        IOCFG8_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG8_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG8_PULL_CTL) -> u8 {
        IOCFG8_PULL_CTL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG9_EDGE_DET {
    #[doc = "No edge detection."]
    NONE = 0x0,
    #[doc = "Negative edge detection."]
    NEG = 0x01,
    #[doc = "Positive edge detection."]
    POS = 0x02,
    #[doc = "Positive and negative edge detection."]
    BOTH = 0x03,
}
impl IOCFG9_EDGE_DET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG9_EDGE_DET {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG9_EDGE_DET {
    #[inline(always)]
    fn from(val: u8) -> IOCFG9_EDGE_DET {
        IOCFG9_EDGE_DET::from_bits(val)
    }
}
impl From<IOCFG9_EDGE_DET> for u8 {
    #[inline(always)]
    fn from(val: IOCFG9_EDGE_DET) -> u8 {
        IOCFG9_EDGE_DET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG9_IOCURR {
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO."]
    _2MA = 0x0,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO."]
    _4MA = 0x01,
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO."]
    _4_8MA = 0x02,
    _RESERVED_3 = 0x03,
}
impl IOCFG9_IOCURR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG9_IOCURR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG9_IOCURR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG9_IOCURR {
        IOCFG9_IOCURR::from_bits(val)
    }
}
impl From<IOCFG9_IOCURR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG9_IOCURR) -> u8 {
        IOCFG9_IOCURR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG9_IOMODE {
    #[doc = "Normal input / output."]
    NORMAL = 0x0,
    #[doc = "Inverted input / ouput."]
    INV = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Open Drain, Normal input / output."]
    OPENDR = 0x04,
    #[doc = "Open Drain Inverted input / output."]
    OPENDR_INV = 0x05,
    #[doc = "Open Source Normal input / output."]
    OPENSRC = 0x06,
    #[doc = "Open Source Inverted input / output."]
    OPENSRC_INV = 0x07,
}
impl IOCFG9_IOMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG9_IOMODE {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG9_IOMODE {
    #[inline(always)]
    fn from(val: u8) -> IOCFG9_IOMODE {
        IOCFG9_IOMODE::from_bits(val)
    }
}
impl From<IOCFG9_IOMODE> for u8 {
    #[inline(always)]
    fn from(val: IOCFG9_IOMODE) -> u8 {
        IOCFG9_IOMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG9_IOSTR {
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)."]
    AUTO = 0x0,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)."]
    MIN = 0x01,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)."]
    MED = 0x02,
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)."]
    MAX = 0x03,
}
impl IOCFG9_IOSTR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG9_IOSTR {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG9_IOSTR {
    #[inline(always)]
    fn from(val: u8) -> IOCFG9_IOSTR {
        IOCFG9_IOSTR::from_bits(val)
    }
}
impl From<IOCFG9_IOSTR> for u8 {
    #[inline(always)]
    fn from(val: IOCFG9_IOSTR) -> u8 {
        IOCFG9_IOSTR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG9_PORT_ID {
    #[doc = "General Purpose IO."]
    GPIO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "AON 32 KHz clock (SCLK_LF)."]
    AON_CLK32K = 0x07,
    #[doc = "AUX IO."]
    AUX_IO = 0x08,
    #[doc = "SSI0 RX."]
    SSI0_RX = 0x09,
    #[doc = "SSI0 TX."]
    SSI0_TX = 0x0a,
    #[doc = "SSI0 FSS."]
    SSI0_FSS = 0x0b,
    #[doc = "SSI0 CLK."]
    SSI0_CLK = 0x0c,
    #[doc = "I2C Data."]
    I2C_MSSDA = 0x0d,
    #[doc = "I2C Clock."]
    I2C_MSSCL = 0x0e,
    #[doc = "UART0 RX."]
    UART0_RX = 0x0f,
    #[doc = "UART0 TX."]
    UART0_TX = 0x10,
    #[doc = "UART0 CTS."]
    UART0_CTS = 0x11,
    #[doc = "UART0 RTS."]
    UART0_RTS = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT0 = 0x17,
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT1 = 0x18,
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT2 = 0x19,
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT3 = 0x1a,
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT4 = 0x1b,
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT5 = 0x1c,
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT6 = 0x1d,
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc."]
    PORT_EVENT7 = 0x1e,
    _RESERVED_1f = 0x1f,
    #[doc = "CPU SWV."]
    CPU_SWV = 0x20,
    #[doc = "SSI1 RX."]
    SSI1_RX = 0x21,
    #[doc = "SSI1 TX."]
    SSI1_TX = 0x22,
    #[doc = "SSI1 FSS."]
    SSI1_FSS = 0x23,
    #[doc = "SSI1 CLK."]
    SSI1_CLK = 0x24,
    #[doc = "I2S Data 0."]
    I2S_AD0 = 0x25,
    #[doc = "I2S Data 1."]
    I2S_AD1 = 0x26,
    #[doc = "I2S WCLK."]
    I2S_WCLK = 0x27,
    #[doc = "I2S BCLK."]
    I2S_BCLK = 0x28,
    #[doc = "I2S MCLK."]
    I2S_MCLK = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "RF Core Trace."]
    RFC_TRC = 0x2e,
    #[doc = "RF Core Data Out 0."]
    RFC_GPO0 = 0x2f,
    #[doc = "RF Core Data Out 1."]
    RFC_GPO1 = 0x30,
    #[doc = "RF Core Data Out 2."]
    RFC_GPO2 = 0x31,
    #[doc = "RF Core Data Out 3."]
    RFC_GPO3 = 0x32,
    #[doc = "RF Core Data In 0."]
    RFC_GPI0 = 0x33,
    #[doc = "RF Core Data In 1."]
    RFC_GPI1 = 0x34,
    #[doc = "RF Core SMI Data Link Out."]
    RFC_SMI_DL_OUT = 0x35,
    #[doc = "RF Core SMI Data Link In."]
    RFC_SMI_DL_IN = 0x36,
    #[doc = "RF Core SMI Command Link Out."]
    RFC_SMI_CL_OUT = 0x37,
    #[doc = "RF Core SMI Command Link In."]
    RFC_SMI_CL_IN = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl IOCFG9_PORT_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG9_PORT_ID {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG9_PORT_ID {
    #[inline(always)]
    fn from(val: u8) -> IOCFG9_PORT_ID {
        IOCFG9_PORT_ID::from_bits(val)
    }
}
impl From<IOCFG9_PORT_ID> for u8 {
    #[inline(always)]
    fn from(val: IOCFG9_PORT_ID) -> u8 {
        IOCFG9_PORT_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCFG9_PULL_CTL {
    _RESERVED_0 = 0x0,
    #[doc = "Pull down."]
    DWN = 0x01,
    #[doc = "Pull up."]
    UP = 0x02,
    #[doc = "No pull."]
    DIS = 0x03,
}
impl IOCFG9_PULL_CTL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCFG9_PULL_CTL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCFG9_PULL_CTL {
    #[inline(always)]
    fn from(val: u8) -> IOCFG9_PULL_CTL {
        IOCFG9_PULL_CTL::from_bits(val)
    }
}
impl From<IOCFG9_PULL_CTL> for u8 {
    #[inline(always)]
    fn from(val: IOCFG9_PULL_CTL) -> u8 {
        IOCFG9_PULL_CTL::to_bits(val)
    }
}

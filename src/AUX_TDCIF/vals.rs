#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CMD {
    #[doc = "Clear STAT.SAT, STAT.DONE, and RESULT.VALUE. This is not needed as prerequisite for a measurement. Reliable clear is only guaranteed from IDLE state."]
    CLR_RESULT = 0x0,
    #[doc = "Synchronous counter start. The counter looks for the opposite edge of the selected start event before it starts to count when the selected edge occurs. This guarantees an edge-triggered start and is recommended for frequency measurements."]
    RUN_SYNC_START = 0x01,
    #[doc = "Asynchronous counter start. The counter starts to count when the start event is high. To achieve precise edge-to-edge measurements you must ensure that the start event is low for at least 420 ns after you write this command."]
    RUN = 0x02,
    #[doc = "Force TDC state machine back to IDLE state. Never write this command while AUX_TDC:STAT.STATE equals CLR_CNT or WAIT_CLR_CNT_DONE."]
    ABORT = 0x03,
}
impl CMD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CMD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CMD {
    #[inline(always)]
    fn from(val: u8) -> CMD {
        CMD::from_bits(val)
    }
}
impl From<CMD> for u8 {
    #[inline(always)]
    fn from(val: CMD) -> u8 {
        CMD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LIMIT {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Result bit 12: TDC conversion saturates and stops when RESULT.VALUE\\[12\\] is set."]
    R12 = 0x03,
    #[doc = "Result bit 13: TDC conversion saturates and stops when RESULT.VALUE\\[13\\] is set."]
    R13 = 0x04,
    #[doc = "Result bit 14: TDC conversion saturates and stops when RESULT.VALUE\\[14\\] is set."]
    R14 = 0x05,
    #[doc = "Result bit 15: TDC conversion saturates and stops when RESULT.VALUE\\[15\\] is set."]
    R15 = 0x06,
    #[doc = "Result bit 16: TDC conversion saturates and stops when RESULT.VALUE\\[16\\] is set."]
    R16 = 0x07,
    #[doc = "Result bit 17: TDC conversion saturates and stops when RESULT.VALUE\\[17\\] is set."]
    R17 = 0x08,
    #[doc = "Result bit 18: TDC conversion saturates and stops when RESULT.VALUE\\[18\\] is set."]
    R18 = 0x09,
    #[doc = "Result bit 19: TDC conversion saturates and stops when RESULT.VALUE\\[19\\] is set."]
    R19 = 0x0a,
    #[doc = "Result bit 20: TDC conversion saturates and stops when RESULT.VALUE\\[20\\] is set."]
    R20 = 0x0b,
    #[doc = "Result bit 21: TDC conversion saturates and stops when RESULT.VALUE\\[21\\] is set."]
    R21 = 0x0c,
    #[doc = "Result bit 22: TDC conversion saturates and stops when RESULT.VALUE\\[22\\] is set."]
    R22 = 0x0d,
    #[doc = "Result bit 23: TDC conversion saturates and stops when RESULT.VALUE\\[23\\] is set."]
    R23 = 0x0e,
    #[doc = "Result bit 24: TDC conversion saturates and stops when RESULT.VALUE\\[24\\] is set."]
    R24 = 0x0f,
}
impl LIMIT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LIMIT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LIMIT {
    #[inline(always)]
    fn from(val: u8) -> LIMIT {
        LIMIT::from_bits(val)
    }
}
impl From<LIMIT> for u8 {
    #[inline(always)]
    fn from(val: LIMIT) -> u8 {
        LIMIT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RATIO {
    #[doc = "Prescaler divides input by 16. AUX_TDC_PRE event has a rising edge for every 16 rising edges of the input. AUX_TDC_PRE event toggles on every 8th rising edge of the input."]
    DIV16 = 0x0,
    #[doc = "Prescaler divides input by 64. AUX_TDC_PRE event has a rising edge for every 64 rising edges of the input. AUX_TDC_PRE event toggles on every 32nd rising edge of the input."]
    DIV64 = 0x01,
}
impl RATIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RATIO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RATIO {
    #[inline(always)]
    fn from(val: u8) -> RATIO {
        RATIO::from_bits(val)
    }
}
impl From<RATIO> for u8 {
    #[inline(always)]
    fn from(val: RATIO) -> u8 {
        RATIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRC {
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2."]
    AON_RTC_CH2 = 0x0,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA."]
    AUX_COMPA = 0x01,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB."]
    AUX_COMPB = 0x02,
    #[doc = "AUX_ANAIF:ISRCCTL.RESET_N."]
    ISRC_RESET = 0x03,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER0_EV."]
    TIMER0_EV = 0x04,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV."]
    TIMER1_EV = 0x05,
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE."]
    SMPH_AUTOTAKE_DONE = 0x06,
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_DONE."]
    ADC_DONE = 0x07,
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_FIFO_ALMOST_FULL."]
    ADC_FIFO_ALMOST_FULL = 0x08,
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX0."]
    OBSMUX0 = 0x09,
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX1."]
    OBSMUX1 = 0x0a,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW."]
    AON_SW = 0x0b,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU."]
    AON_PROG_WU = 0x0c,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0."]
    AUXIO0 = 0x0d,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1."]
    AUXIO1 = 0x0e,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2."]
    AUXIO2 = 0x0f,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3."]
    AUXIO3 = 0x10,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4."]
    AUXIO4 = 0x11,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5."]
    AUXIO5 = 0x12,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6."]
    AUXIO6 = 0x13,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7."]
    AUXIO7 = 0x14,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8."]
    AUXIO8 = 0x15,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9."]
    AUXIO9 = 0x16,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10."]
    AUXIO10 = 0x17,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11."]
    AUXIO11 = 0x18,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12."]
    AUXIO12 = 0x19,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13."]
    AUXIO13 = 0x1a,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14."]
    AUXIO14 = 0x1b,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15."]
    AUXIO15 = 0x1c,
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF."]
    ACLK_REF = 0x1d,
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV."]
    MCU_EV = 0x1e,
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ."]
    ADC_IRQ = 0x1f,
}
impl SRC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRC {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRC {
    #[inline(always)]
    fn from(val: u8) -> SRC {
        SRC::from_bits(val)
    }
}
impl From<SRC> for u8 {
    #[inline(always)]
    fn from(val: SRC) -> u8 {
        SRC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum START_POL {
    #[doc = "TDC conversion starts when high level is detected."]
    HIGH = 0x0,
    #[doc = "TDC conversion starts when low level is detected."]
    LOW = 0x01,
}
impl START_POL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> START_POL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for START_POL {
    #[inline(always)]
    fn from(val: u8) -> START_POL {
        START_POL::from_bits(val)
    }
}
impl From<START_POL> for u8 {
    #[inline(always)]
    fn from(val: START_POL) -> u8 {
        START_POL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum START_SRC {
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2."]
    AON_RTC_CH2 = 0x0,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA."]
    AUX_COMPA = 0x01,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB."]
    AUX_COMPB = 0x02,
    #[doc = "AUX_ANAIF:ISRCCTL.RESET_N."]
    ISRC_RESET = 0x03,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER0_EV."]
    TIMER0_EV = 0x04,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV."]
    TIMER1_EV = 0x05,
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE."]
    SMPH_AUTOTAKE_DONE = 0x06,
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_DONE."]
    ADC_DONE = 0x07,
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_FIFO_ALMOST_FULL."]
    ADC_FIFO_ALMOST_FULL = 0x08,
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX0."]
    OBSMUX0 = 0x09,
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX1."]
    OBSMUX1 = 0x0a,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW."]
    AON_SW = 0x0b,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU."]
    AON_PROG_WU = 0x0c,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0."]
    AUXIO0 = 0x0d,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1."]
    AUXIO1 = 0x0e,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2."]
    AUXIO2 = 0x0f,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3."]
    AUXIO3 = 0x10,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4."]
    AUXIO4 = 0x11,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5."]
    AUXIO5 = 0x12,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6."]
    AUXIO6 = 0x13,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7."]
    AUXIO7 = 0x14,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8."]
    AUXIO8 = 0x15,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9."]
    AUXIO9 = 0x16,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10."]
    AUXIO10 = 0x17,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11."]
    AUXIO11 = 0x18,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12."]
    AUXIO12 = 0x19,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13."]
    AUXIO13 = 0x1a,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14."]
    AUXIO14 = 0x1b,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15."]
    AUXIO15 = 0x1c,
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF."]
    ACLK_REF = 0x1d,
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV."]
    MCU_EV = 0x1e,
    #[doc = "Select TDC Prescaler event which is generated by configuration of PRECTL."]
    TDC_PRE = 0x1f,
}
impl START_SRC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> START_SRC {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for START_SRC {
    #[inline(always)]
    fn from(val: u8) -> START_SRC {
        START_SRC::from_bits(val)
    }
}
impl From<START_SRC> for u8 {
    #[inline(always)]
    fn from(val: START_SRC) -> u8 {
        START_SRC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STATE {
    #[doc = "Current state is TDC_STATE_WAIT_START. The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    WAIT_START = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Current state is TDC_STATE_WAIT_STARTSTOPCNTEN. The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    WAIT_START_STOP_CNT_EN = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Current state is TDC_STATE_IDLE. This is the default state after reset and abortion. State will change when you write CTL.CMD to either RUN_SYNC_START or RUN."]
    IDLE = 0x06,
    #[doc = "Current state is TDC_STATE_CLRCNT. The fast-counter circuit is reset."]
    CLR_CNT = 0x07,
    #[doc = "Current state is TDC_STATE_WAIT_STOP. The state machine waits for the fast-counter circuit to stop."]
    WAIT_STOP = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "Current state is TDC_STATE_WAIT_STOPCNTDOWN. The fast-counter circuit looks for the stop condition. It will ignore a number of stop events configured in TRIGCNTLOAD.CNT."]
    WAIT_STOP_CNTDWN = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Current state is TDC_STATE_GETRESULTS. The state machine copies the counter value from the fast-counter circuit."]
    GET_RESULT = 0x0e,
    #[doc = "Current state is TDC_STATE_POR. This is the reset state."]
    POR = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    #[doc = "Current state is TDC_STATE_WAIT_CLRCNT_DONE. The state machine waits for fast-counter circuit to finish reset."]
    WAIT_CLR_CNT_DONE = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Current state is TDC_WAIT_STARTFALL. The fast-counter circuit waits for a falling edge on the start event."]
    START_FALL = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    #[doc = "Current state is TDC_FORCESTOP. You wrote ABORT to CTL.CMD to abort the TDC measurement."]
    FORCE_STOP = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl STATE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STATE {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STATE {
    #[inline(always)]
    fn from(val: u8) -> STATE {
        STATE::from_bits(val)
    }
}
impl From<STATE> for u8 {
    #[inline(always)]
    fn from(val: STATE) -> u8 {
        STATE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STOP_POL {
    #[doc = "TDC conversion stops when high level is detected."]
    HIGH = 0x0,
    #[doc = "TDC conversion stops when low level is detected."]
    LOW = 0x01,
}
impl STOP_POL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STOP_POL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STOP_POL {
    #[inline(always)]
    fn from(val: u8) -> STOP_POL {
        STOP_POL::from_bits(val)
    }
}
impl From<STOP_POL> for u8 {
    #[inline(always)]
    fn from(val: STOP_POL) -> u8 {
        STOP_POL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STOP_SRC {
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2."]
    AON_RTC_CH2 = 0x0,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA."]
    AUX_COMPA = 0x01,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB."]
    AUX_COMPB = 0x02,
    #[doc = "AUX_ANAIF:ISRCCTL.RESET_N."]
    ISRC_RESET = 0x03,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER0_EV."]
    TIMER0_EV = 0x04,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV."]
    TIMER1_EV = 0x05,
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE."]
    SMPH_AUTOTAKE_DONE = 0x06,
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_DONE."]
    ADC_DONE = 0x07,
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_FIFO_ALMOST_FULL."]
    ADC_FIFO_ALMOST_FULL = 0x08,
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX0."]
    OBSMUX0 = 0x09,
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX1."]
    OBSMUX1 = 0x0a,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW."]
    AON_SW = 0x0b,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU."]
    AON_PROG_WU = 0x0c,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0."]
    AUXIO0 = 0x0d,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1."]
    AUXIO1 = 0x0e,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2."]
    AUXIO2 = 0x0f,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3."]
    AUXIO3 = 0x10,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4."]
    AUXIO4 = 0x11,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5."]
    AUXIO5 = 0x12,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6."]
    AUXIO6 = 0x13,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7."]
    AUXIO7 = 0x14,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8."]
    AUXIO8 = 0x15,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9."]
    AUXIO9 = 0x16,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10."]
    AUXIO10 = 0x17,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11."]
    AUXIO11 = 0x18,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12."]
    AUXIO12 = 0x19,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13."]
    AUXIO13 = 0x1a,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14."]
    AUXIO14 = 0x1b,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15."]
    AUXIO15 = 0x1c,
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF."]
    ACLK_REF = 0x1d,
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV."]
    MCU_EV = 0x1e,
    #[doc = "Select TDC Prescaler event which is generated by configuration of PRECTL."]
    TDC_PRE = 0x1f,
}
impl STOP_SRC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STOP_SRC {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STOP_SRC {
    #[inline(always)]
    fn from(val: u8) -> STOP_SRC {
        STOP_SRC::from_bits(val)
    }
}
impl From<STOP_SRC> for u8 {
    #[inline(always)]
    fn from(val: STOP_SRC) -> u8 {
        STOP_SRC::to_bits(val)
    }
}

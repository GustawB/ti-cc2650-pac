#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AON_PROG0_EV {
    #[doc = "Edge detect on PAD0."]
    PAD0 = 0x0,
    #[doc = "Edge detect on PAD1."]
    PAD1 = 0x01,
    #[doc = "Edge detect on PAD2."]
    PAD2 = 0x02,
    #[doc = "Edge detect on PAD3."]
    PAD3 = 0x03,
    #[doc = "Edge detect on PAD4."]
    PAD4 = 0x04,
    #[doc = "Edge detect on PAD5."]
    PAD5 = 0x05,
    #[doc = "Edge detect on PAD6."]
    PAD6 = 0x06,
    #[doc = "Edge detect on PAD7."]
    PAD7 = 0x07,
    #[doc = "Edge detect on PAD8."]
    PAD8 = 0x08,
    #[doc = "Edge detect on PAD9."]
    PAD9 = 0x09,
    #[doc = "Edge detect on PAD10."]
    PAD10 = 0x0a,
    #[doc = "Edge detect on PAD11."]
    PAD11 = 0x0b,
    #[doc = "Edge detect on PAD12."]
    PAD12 = 0x0c,
    #[doc = "Edge detect on PAD13."]
    PAD13 = 0x0d,
    #[doc = "Edge detect on PAD14."]
    PAD14 = 0x0e,
    #[doc = "Edge detect on PAD15."]
    PAD15 = 0x0f,
    #[doc = "Edge detect on PAD16."]
    PAD16 = 0x10,
    #[doc = "Edge detect on PAD17."]
    PAD17 = 0x11,
    #[doc = "Edge detect on PAD18."]
    PAD18 = 0x12,
    #[doc = "Edge detect on PAD19."]
    PAD19 = 0x13,
    #[doc = "Edge detect on PAD20."]
    PAD20 = 0x14,
    #[doc = "Edge detect on PAD21."]
    PAD21 = 0x15,
    #[doc = "Edge detect on PAD22."]
    PAD22 = 0x16,
    #[doc = "Edge detect on PAD23."]
    PAD23 = 0x17,
    #[doc = "Edge detect on PAD24."]
    PAD24 = 0x18,
    #[doc = "Edge detect on PAD25."]
    PAD25 = 0x19,
    #[doc = "Edge detect on PAD26."]
    PAD26 = 0x1a,
    #[doc = "Edge detect on PAD27."]
    PAD27 = 0x1b,
    #[doc = "Edge detect on PAD28."]
    PAD28 = 0x1c,
    #[doc = "Edge detect on PAD29."]
    PAD29 = 0x1d,
    #[doc = "Edge detect on PAD30."]
    PAD30 = 0x1e,
    #[doc = "Edge detect on PAD31."]
    PAD31 = 0x1f,
    #[doc = "Edge detect on any PAD."]
    PAD = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "RTC channel 0 event."]
    RTC_CH0 = 0x23,
    #[doc = "RTC channel 1 event."]
    RTC_CH1 = 0x24,
    #[doc = "RTC channel 2 event."]
    RTC_CH2 = 0x25,
    #[doc = "RTC channel 0 - delayed event."]
    RTC_CH0_DLY = 0x26,
    #[doc = "RTC channel 1 - delayed event."]
    RTC_CH1_DLY = 0x27,
    #[doc = "RTC channel 2 - delayed event."]
    RTC_CH2_DLY = 0x28,
    #[doc = "RTC combined delayed event."]
    RTC_COMB_DLY = 0x29,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)."]
    RTC_UPD = 0x2a,
    #[doc = "JTAG generated event."]
    JTAG = 0x2b,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0."]
    AUX_SWEV0 = 0x2c,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1."]
    AUX_SWEV1 = 0x2d,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2."]
    AUX_SWEV2 = 0x2e,
    #[doc = "Comparator A triggered."]
    AUX_COMPA = 0x2f,
    #[doc = "Comparator B triggered."]
    AUX_COMPB = 0x30,
    #[doc = "ADC conversion completed."]
    AUX_ADC_DONE = 0x31,
    #[doc = "TDC completed or timed out."]
    AUX_TDC_DONE = 0x32,
    #[doc = "AUX Timer 0 Event."]
    AUX_TIMER0_EV = 0x33,
    #[doc = "AUX Timer 1 Event."]
    AUX_TIMER1_EV = 0x34,
    #[doc = "BATMON temperature update event."]
    BATMON_TEMP = 0x35,
    #[doc = "BATMON voltage update event."]
    BATMON_VOLT = 0x36,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC = 0x37,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC_N = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "No event, always low."]
    NONE = 0x3f,
}
impl AON_PROG0_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AON_PROG0_EV {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AON_PROG0_EV {
    #[inline(always)]
    fn from(val: u8) -> AON_PROG0_EV {
        AON_PROG0_EV::from_bits(val)
    }
}
impl From<AON_PROG0_EV> for u8 {
    #[inline(always)]
    fn from(val: AON_PROG0_EV) -> u8 {
        AON_PROG0_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AON_PROG1_EV {
    #[doc = "Edge detect on PAD0."]
    PAD0 = 0x0,
    #[doc = "Edge detect on PAD1."]
    PAD1 = 0x01,
    #[doc = "Edge detect on PAD2."]
    PAD2 = 0x02,
    #[doc = "Edge detect on PAD3."]
    PAD3 = 0x03,
    #[doc = "Edge detect on PAD4."]
    PAD4 = 0x04,
    #[doc = "Edge detect on PAD5."]
    PAD5 = 0x05,
    #[doc = "Edge detect on PAD6."]
    PAD6 = 0x06,
    #[doc = "Edge detect on PAD7."]
    PAD7 = 0x07,
    #[doc = "Edge detect on PAD8."]
    PAD8 = 0x08,
    #[doc = "Edge detect on PAD9."]
    PAD9 = 0x09,
    #[doc = "Edge detect on PAD10."]
    PAD10 = 0x0a,
    #[doc = "Edge detect on PAD11."]
    PAD11 = 0x0b,
    #[doc = "Edge detect on PAD12."]
    PAD12 = 0x0c,
    #[doc = "Edge detect on PAD13."]
    PAD13 = 0x0d,
    #[doc = "Edge detect on PAD14."]
    PAD14 = 0x0e,
    #[doc = "Edge detect on PAD15."]
    PAD15 = 0x0f,
    #[doc = "Edge detect on PAD16."]
    PAD16 = 0x10,
    #[doc = "Edge detect on PAD17."]
    PAD17 = 0x11,
    #[doc = "Edge detect on PAD18."]
    PAD18 = 0x12,
    #[doc = "Edge detect on PAD19."]
    PAD19 = 0x13,
    #[doc = "Edge detect on PAD20."]
    PAD20 = 0x14,
    #[doc = "Edge detect on PAD21."]
    PAD21 = 0x15,
    #[doc = "Edge detect on PAD22."]
    PAD22 = 0x16,
    #[doc = "Edge detect on PAD23."]
    PAD23 = 0x17,
    #[doc = "Edge detect on PAD24."]
    PAD24 = 0x18,
    #[doc = "Edge detect on PAD25."]
    PAD25 = 0x19,
    #[doc = "Edge detect on PAD26."]
    PAD26 = 0x1a,
    #[doc = "Edge detect on PAD27."]
    PAD27 = 0x1b,
    #[doc = "Edge detect on PAD28."]
    PAD28 = 0x1c,
    #[doc = "Edge detect on PAD29."]
    PAD29 = 0x1d,
    #[doc = "Edge detect on PAD30."]
    PAD30 = 0x1e,
    #[doc = "Edge detect on PAD31."]
    PAD31 = 0x1f,
    #[doc = "Edge detect on any PAD."]
    PAD = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "RTC channel 0 event."]
    RTC_CH0 = 0x23,
    #[doc = "RTC channel 1 event."]
    RTC_CH1 = 0x24,
    #[doc = "RTC channel 2 event."]
    RTC_CH2 = 0x25,
    #[doc = "RTC channel 0 - delayed event."]
    RTC_CH0_DLY = 0x26,
    #[doc = "RTC channel 1 - delayed event."]
    RTC_CH1_DLY = 0x27,
    #[doc = "RTC channel 2 - delayed event."]
    RTC_CH2_DLY = 0x28,
    #[doc = "RTC combined delayed event."]
    RTC_COMB_DLY = 0x29,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)."]
    RTC_UPD = 0x2a,
    #[doc = "JTAG generated event."]
    JTAG = 0x2b,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0."]
    AUX_SWEV0 = 0x2c,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1."]
    AUX_SWEV1 = 0x2d,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2."]
    AUX_SWEV2 = 0x2e,
    #[doc = "Comparator A triggered."]
    AUX_COMPA = 0x2f,
    #[doc = "Comparator B triggered."]
    AUX_COMPB = 0x30,
    #[doc = "ADC conversion completed."]
    AUX_ADC_DONE = 0x31,
    #[doc = "TDC completed or timed out."]
    AUX_TDC_DONE = 0x32,
    #[doc = "AUX Timer 0 Event."]
    AUX_TIMER0_EV = 0x33,
    #[doc = "AUX Timer 1 Event."]
    AUX_TIMER1_EV = 0x34,
    #[doc = "BATMON temperature update event."]
    BATMON_TEMP = 0x35,
    #[doc = "BATMON voltage update event."]
    BATMON_VOLT = 0x36,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC = 0x37,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC_N = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "No event, always low."]
    NONE = 0x3f,
}
impl AON_PROG1_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AON_PROG1_EV {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AON_PROG1_EV {
    #[inline(always)]
    fn from(val: u8) -> AON_PROG1_EV {
        AON_PROG1_EV::from_bits(val)
    }
}
impl From<AON_PROG1_EV> for u8 {
    #[inline(always)]
    fn from(val: AON_PROG1_EV) -> u8 {
        AON_PROG1_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AON_PROG2_EV {
    #[doc = "Edge detect on PAD0."]
    PAD0 = 0x0,
    #[doc = "Edge detect on PAD1."]
    PAD1 = 0x01,
    #[doc = "Edge detect on PAD2."]
    PAD2 = 0x02,
    #[doc = "Edge detect on PAD3."]
    PAD3 = 0x03,
    #[doc = "Edge detect on PAD4."]
    PAD4 = 0x04,
    #[doc = "Edge detect on PAD5."]
    PAD5 = 0x05,
    #[doc = "Edge detect on PAD6."]
    PAD6 = 0x06,
    #[doc = "Edge detect on PAD7."]
    PAD7 = 0x07,
    #[doc = "Edge detect on PAD8."]
    PAD8 = 0x08,
    #[doc = "Edge detect on PAD9."]
    PAD9 = 0x09,
    #[doc = "Edge detect on PAD10."]
    PAD10 = 0x0a,
    #[doc = "Edge detect on PAD11."]
    PAD11 = 0x0b,
    #[doc = "Edge detect on PAD12."]
    PAD12 = 0x0c,
    #[doc = "Edge detect on PAD13."]
    PAD13 = 0x0d,
    #[doc = "Edge detect on PAD14."]
    PAD14 = 0x0e,
    #[doc = "Edge detect on PAD15."]
    PAD15 = 0x0f,
    #[doc = "Edge detect on PAD16."]
    PAD16 = 0x10,
    #[doc = "Edge detect on PAD17."]
    PAD17 = 0x11,
    #[doc = "Edge detect on PAD18."]
    PAD18 = 0x12,
    #[doc = "Edge detect on PAD19."]
    PAD19 = 0x13,
    #[doc = "Edge detect on PAD20."]
    PAD20 = 0x14,
    #[doc = "Edge detect on PAD21."]
    PAD21 = 0x15,
    #[doc = "Edge detect on PAD22."]
    PAD22 = 0x16,
    #[doc = "Edge detect on PAD23."]
    PAD23 = 0x17,
    #[doc = "Edge detect on PAD24."]
    PAD24 = 0x18,
    #[doc = "Edge detect on PAD25."]
    PAD25 = 0x19,
    #[doc = "Edge detect on PAD26."]
    PAD26 = 0x1a,
    #[doc = "Edge detect on PAD27."]
    PAD27 = 0x1b,
    #[doc = "Edge detect on PAD28."]
    PAD28 = 0x1c,
    #[doc = "Edge detect on PAD29."]
    PAD29 = 0x1d,
    #[doc = "Edge detect on PAD30."]
    PAD30 = 0x1e,
    #[doc = "Edge detect on PAD31."]
    PAD31 = 0x1f,
    #[doc = "Edge detect on any PAD."]
    PAD = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "RTC channel 0 event."]
    RTC_CH0 = 0x23,
    #[doc = "RTC channel 1 event."]
    RTC_CH1 = 0x24,
    #[doc = "RTC channel 2 event."]
    RTC_CH2 = 0x25,
    #[doc = "RTC channel 0 - delayed event."]
    RTC_CH0_DLY = 0x26,
    #[doc = "RTC channel 1 - delayed event."]
    RTC_CH1_DLY = 0x27,
    #[doc = "RTC channel 2 - delayed event."]
    RTC_CH2_DLY = 0x28,
    #[doc = "RTC combined delayed event."]
    RTC_COMB_DLY = 0x29,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)."]
    RTC_UPD = 0x2a,
    #[doc = "JTAG generated event."]
    JTAG = 0x2b,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0."]
    AUX_SWEV0 = 0x2c,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1."]
    AUX_SWEV1 = 0x2d,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2."]
    AUX_SWEV2 = 0x2e,
    #[doc = "Comparator A triggered."]
    AUX_COMPA = 0x2f,
    #[doc = "Comparator B triggered."]
    AUX_COMPB = 0x30,
    #[doc = "ADC conversion completed."]
    AUX_ADC_DONE = 0x31,
    #[doc = "TDC completed or timed out."]
    AUX_TDC_DONE = 0x32,
    #[doc = "AUX Timer 0 Event."]
    AUX_TIMER0_EV = 0x33,
    #[doc = "AUX Timer 1 Event."]
    AUX_TIMER1_EV = 0x34,
    #[doc = "BATMON temperature update event."]
    BATMON_TEMP = 0x35,
    #[doc = "BATMON voltage update event."]
    BATMON_VOLT = 0x36,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC = 0x37,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC_N = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "No event, always low."]
    NONE = 0x3f,
}
impl AON_PROG2_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AON_PROG2_EV {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AON_PROG2_EV {
    #[inline(always)]
    fn from(val: u8) -> AON_PROG2_EV {
        AON_PROG2_EV::from_bits(val)
    }
}
impl From<AON_PROG2_EV> for u8 {
    #[inline(always)]
    fn from(val: AON_PROG2_EV) -> u8 {
        AON_PROG2_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AUXWUSEL_WU0_EV {
    #[doc = "Edge detect on PAD0."]
    PAD0 = 0x0,
    #[doc = "Edge detect on PAD1."]
    PAD1 = 0x01,
    #[doc = "Edge detect on PAD2."]
    PAD2 = 0x02,
    #[doc = "Edge detect on PAD3."]
    PAD3 = 0x03,
    #[doc = "Edge detect on PAD4."]
    PAD4 = 0x04,
    #[doc = "Edge detect on PAD5."]
    PAD5 = 0x05,
    #[doc = "Edge detect on PAD6."]
    PAD6 = 0x06,
    #[doc = "Edge detect on PAD7."]
    PAD7 = 0x07,
    #[doc = "Edge detect on PAD8."]
    PAD8 = 0x08,
    #[doc = "Edge detect on PAD9."]
    PAD9 = 0x09,
    #[doc = "Edge detect on PAD10."]
    PAD10 = 0x0a,
    #[doc = "Edge detect on PAD11."]
    PAD11 = 0x0b,
    #[doc = "Edge detect on PAD12."]
    PAD12 = 0x0c,
    #[doc = "Edge detect on PAD13."]
    PAD13 = 0x0d,
    #[doc = "Edge detect on PAD14."]
    PAD14 = 0x0e,
    #[doc = "Edge detect on PAD15."]
    PAD15 = 0x0f,
    #[doc = "Edge detect on PAD16."]
    PAD16 = 0x10,
    #[doc = "Edge detect on PAD17."]
    PAD17 = 0x11,
    #[doc = "Edge detect on PAD18."]
    PAD18 = 0x12,
    #[doc = "Edge detect on PAD19."]
    PAD19 = 0x13,
    #[doc = "Edge detect on PAD20."]
    PAD20 = 0x14,
    #[doc = "Edge detect on PAD21."]
    PAD21 = 0x15,
    #[doc = "Edge detect on PAD22."]
    PAD22 = 0x16,
    #[doc = "Edge detect on PAD23."]
    PAD23 = 0x17,
    #[doc = "Edge detect on PAD24."]
    PAD24 = 0x18,
    #[doc = "Edge detect on PAD25."]
    PAD25 = 0x19,
    #[doc = "Edge detect on PAD26."]
    PAD26 = 0x1a,
    #[doc = "Edge detect on PAD27."]
    PAD27 = 0x1b,
    #[doc = "Edge detect on PAD28."]
    PAD28 = 0x1c,
    #[doc = "Edge detect on PAD29."]
    PAD29 = 0x1d,
    #[doc = "Edge detect on PAD30."]
    PAD30 = 0x1e,
    #[doc = "Edge detect on PAD31."]
    PAD31 = 0x1f,
    #[doc = "Edge detect on any PAD."]
    PAD = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "RTC channel 0 event."]
    RTC_CH0 = 0x23,
    #[doc = "RTC channel 1 event."]
    RTC_CH1 = 0x24,
    #[doc = "RTC channel 2 event."]
    RTC_CH2 = 0x25,
    #[doc = "RTC channel 0 - delayed event."]
    RTC_CH0_DLY = 0x26,
    #[doc = "RTC channel 1 - delayed event."]
    RTC_CH1_DLY = 0x27,
    #[doc = "RTC channel 2 - delayed event."]
    RTC_CH2_DLY = 0x28,
    #[doc = "RTC combined delayed event."]
    RTC_COMB_DLY = 0x29,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)."]
    RTC_UPD = 0x2a,
    #[doc = "JTAG generated event."]
    JTAG = 0x2b,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0."]
    AUX_SWEV0 = 0x2c,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1."]
    AUX_SWEV1 = 0x2d,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2."]
    AUX_SWEV2 = 0x2e,
    #[doc = "Comparator A triggered."]
    AUX_COMPA = 0x2f,
    #[doc = "Comparator B triggered."]
    AUX_COMPB = 0x30,
    #[doc = "ADC conversion completed."]
    AUX_ADC_DONE = 0x31,
    #[doc = "TDC completed or timed out."]
    AUX_TDC_DONE = 0x32,
    #[doc = "AUX Timer 0 Event."]
    AUX_TIMER0_EV = 0x33,
    #[doc = "AUX Timer 1 Event."]
    AUX_TIMER1_EV = 0x34,
    #[doc = "BATMON temperature update event."]
    BATMON_TEMP = 0x35,
    #[doc = "BATMON voltage update event."]
    BATMON_VOLT = 0x36,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC = 0x37,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC_N = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "No event, always low."]
    NONE = 0x3f,
}
impl AUXWUSEL_WU0_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AUXWUSEL_WU0_EV {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AUXWUSEL_WU0_EV {
    #[inline(always)]
    fn from(val: u8) -> AUXWUSEL_WU0_EV {
        AUXWUSEL_WU0_EV::from_bits(val)
    }
}
impl From<AUXWUSEL_WU0_EV> for u8 {
    #[inline(always)]
    fn from(val: AUXWUSEL_WU0_EV) -> u8 {
        AUXWUSEL_WU0_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AUXWUSEL_WU1_EV {
    #[doc = "Edge detect on PAD0."]
    PAD0 = 0x0,
    #[doc = "Edge detect on PAD1."]
    PAD1 = 0x01,
    #[doc = "Edge detect on PAD2."]
    PAD2 = 0x02,
    #[doc = "Edge detect on PAD3."]
    PAD3 = 0x03,
    #[doc = "Edge detect on PAD4."]
    PAD4 = 0x04,
    #[doc = "Edge detect on PAD5."]
    PAD5 = 0x05,
    #[doc = "Edge detect on PAD6."]
    PAD6 = 0x06,
    #[doc = "Edge detect on PAD7."]
    PAD7 = 0x07,
    #[doc = "Edge detect on PAD8."]
    PAD8 = 0x08,
    #[doc = "Edge detect on PAD9."]
    PAD9 = 0x09,
    #[doc = "Edge detect on PAD10."]
    PAD10 = 0x0a,
    #[doc = "Edge detect on PAD11."]
    PAD11 = 0x0b,
    #[doc = "Edge detect on PAD12."]
    PAD12 = 0x0c,
    #[doc = "Edge detect on PAD13."]
    PAD13 = 0x0d,
    #[doc = "Edge detect on PAD14."]
    PAD14 = 0x0e,
    #[doc = "Edge detect on PAD15."]
    PAD15 = 0x0f,
    #[doc = "Edge detect on PAD16."]
    PAD16 = 0x10,
    #[doc = "Edge detect on PAD17."]
    PAD17 = 0x11,
    #[doc = "Edge detect on PAD18."]
    PAD18 = 0x12,
    #[doc = "Edge detect on PAD19."]
    PAD19 = 0x13,
    #[doc = "Edge detect on PAD20."]
    PAD20 = 0x14,
    #[doc = "Edge detect on PAD21."]
    PAD21 = 0x15,
    #[doc = "Edge detect on PAD22."]
    PAD22 = 0x16,
    #[doc = "Edge detect on PAD23."]
    PAD23 = 0x17,
    #[doc = "Edge detect on PAD24."]
    PAD24 = 0x18,
    #[doc = "Edge detect on PAD25."]
    PAD25 = 0x19,
    #[doc = "Edge detect on PAD26."]
    PAD26 = 0x1a,
    #[doc = "Edge detect on PAD27."]
    PAD27 = 0x1b,
    #[doc = "Edge detect on PAD28."]
    PAD28 = 0x1c,
    #[doc = "Edge detect on PAD29."]
    PAD29 = 0x1d,
    #[doc = "Edge detect on PAD30."]
    PAD30 = 0x1e,
    #[doc = "Edge detect on PAD31."]
    PAD31 = 0x1f,
    #[doc = "Edge detect on any PAD."]
    PAD = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "RTC channel 0 event."]
    RTC_CH0 = 0x23,
    #[doc = "RTC channel 1 event."]
    RTC_CH1 = 0x24,
    #[doc = "RTC channel 2 event."]
    RTC_CH2 = 0x25,
    #[doc = "RTC channel 0 - delayed event."]
    RTC_CH0_DLY = 0x26,
    #[doc = "RTC channel 1 - delayed event."]
    RTC_CH1_DLY = 0x27,
    #[doc = "RTC channel 2 - delayed event."]
    RTC_CH2_DLY = 0x28,
    #[doc = "RTC combined delayed event."]
    RTC_COMB_DLY = 0x29,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)."]
    RTC_UPD = 0x2a,
    #[doc = "JTAG generated event."]
    JTAG = 0x2b,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0."]
    AUX_SWEV0 = 0x2c,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1."]
    AUX_SWEV1 = 0x2d,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2."]
    AUX_SWEV2 = 0x2e,
    #[doc = "Comparator A triggered."]
    AUX_COMPA = 0x2f,
    #[doc = "Comparator B triggered."]
    AUX_COMPB = 0x30,
    #[doc = "ADC conversion completed."]
    AUX_ADC_DONE = 0x31,
    #[doc = "TDC completed or timed out."]
    AUX_TDC_DONE = 0x32,
    #[doc = "AUX Timer 0 Event."]
    AUX_TIMER0_EV = 0x33,
    #[doc = "AUX Timer 1 Event."]
    AUX_TIMER1_EV = 0x34,
    #[doc = "BATMON temperature update event."]
    BATMON_TEMP = 0x35,
    #[doc = "BATMON voltage update event."]
    BATMON_VOLT = 0x36,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC = 0x37,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC_N = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "No event, always low."]
    NONE = 0x3f,
}
impl AUXWUSEL_WU1_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AUXWUSEL_WU1_EV {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AUXWUSEL_WU1_EV {
    #[inline(always)]
    fn from(val: u8) -> AUXWUSEL_WU1_EV {
        AUXWUSEL_WU1_EV::from_bits(val)
    }
}
impl From<AUXWUSEL_WU1_EV> for u8 {
    #[inline(always)]
    fn from(val: AUXWUSEL_WU1_EV) -> u8 {
        AUXWUSEL_WU1_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AUXWUSEL_WU2_EV {
    #[doc = "Edge detect on PAD0."]
    PAD0 = 0x0,
    #[doc = "Edge detect on PAD1."]
    PAD1 = 0x01,
    #[doc = "Edge detect on PAD2."]
    PAD2 = 0x02,
    #[doc = "Edge detect on PAD3."]
    PAD3 = 0x03,
    #[doc = "Edge detect on PAD4."]
    PAD4 = 0x04,
    #[doc = "Edge detect on PAD5."]
    PAD5 = 0x05,
    #[doc = "Edge detect on PAD6."]
    PAD6 = 0x06,
    #[doc = "Edge detect on PAD7."]
    PAD7 = 0x07,
    #[doc = "Edge detect on PAD8."]
    PAD8 = 0x08,
    #[doc = "Edge detect on PAD9."]
    PAD9 = 0x09,
    #[doc = "Edge detect on PAD10."]
    PAD10 = 0x0a,
    #[doc = "Edge detect on PAD11."]
    PAD11 = 0x0b,
    #[doc = "Edge detect on PAD12."]
    PAD12 = 0x0c,
    #[doc = "Edge detect on PAD13."]
    PAD13 = 0x0d,
    #[doc = "Edge detect on PAD14."]
    PAD14 = 0x0e,
    #[doc = "Edge detect on PAD15."]
    PAD15 = 0x0f,
    #[doc = "Edge detect on PAD16."]
    PAD16 = 0x10,
    #[doc = "Edge detect on PAD17."]
    PAD17 = 0x11,
    #[doc = "Edge detect on PAD18."]
    PAD18 = 0x12,
    #[doc = "Edge detect on PAD19."]
    PAD19 = 0x13,
    #[doc = "Edge detect on PAD20."]
    PAD20 = 0x14,
    #[doc = "Edge detect on PAD21."]
    PAD21 = 0x15,
    #[doc = "Edge detect on PAD22."]
    PAD22 = 0x16,
    #[doc = "Edge detect on PAD23."]
    PAD23 = 0x17,
    #[doc = "Edge detect on PAD24."]
    PAD24 = 0x18,
    #[doc = "Edge detect on PAD25."]
    PAD25 = 0x19,
    #[doc = "Edge detect on PAD26."]
    PAD26 = 0x1a,
    #[doc = "Edge detect on PAD27."]
    PAD27 = 0x1b,
    #[doc = "Edge detect on PAD28."]
    PAD28 = 0x1c,
    #[doc = "Edge detect on PAD29."]
    PAD29 = 0x1d,
    #[doc = "Edge detect on PAD30."]
    PAD30 = 0x1e,
    #[doc = "Edge detect on PAD31."]
    PAD31 = 0x1f,
    #[doc = "Edge detect on any PAD."]
    PAD = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "RTC channel 0 event."]
    RTC_CH0 = 0x23,
    #[doc = "RTC channel 1 event."]
    RTC_CH1 = 0x24,
    #[doc = "RTC channel 2 event."]
    RTC_CH2 = 0x25,
    #[doc = "RTC channel 0 - delayed event."]
    RTC_CH0_DLY = 0x26,
    #[doc = "RTC channel 1 - delayed event."]
    RTC_CH1_DLY = 0x27,
    #[doc = "RTC channel 2 - delayed event."]
    RTC_CH2_DLY = 0x28,
    #[doc = "RTC combined delayed event."]
    RTC_COMB_DLY = 0x29,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)."]
    RTC_UPD = 0x2a,
    #[doc = "JTAG generated event."]
    JTAG = 0x2b,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0."]
    AUX_SWEV0 = 0x2c,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1."]
    AUX_SWEV1 = 0x2d,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2."]
    AUX_SWEV2 = 0x2e,
    #[doc = "Comparator A triggered."]
    AUX_COMPA = 0x2f,
    #[doc = "Comparator B triggered."]
    AUX_COMPB = 0x30,
    #[doc = "ADC conversion completed."]
    AUX_ADC_DONE = 0x31,
    #[doc = "TDC completed or timed out."]
    AUX_TDC_DONE = 0x32,
    #[doc = "AUX Timer 0 Event."]
    AUX_TIMER0_EV = 0x33,
    #[doc = "AUX Timer 1 Event."]
    AUX_TIMER1_EV = 0x34,
    #[doc = "BATMON temperature update event."]
    BATMON_TEMP = 0x35,
    #[doc = "BATMON voltage update event."]
    BATMON_VOLT = 0x36,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC = 0x37,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC_N = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "No event, always low."]
    NONE = 0x3f,
}
impl AUXWUSEL_WU2_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AUXWUSEL_WU2_EV {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AUXWUSEL_WU2_EV {
    #[inline(always)]
    fn from(val: u8) -> AUXWUSEL_WU2_EV {
        AUXWUSEL_WU2_EV::from_bits(val)
    }
}
impl From<AUXWUSEL_WU2_EV> for u8 {
    #[inline(always)]
    fn from(val: AUXWUSEL_WU2_EV) -> u8 {
        AUXWUSEL_WU2_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MCUWUSEL_WU0_EV {
    #[doc = "Edge detect on PAD0."]
    PAD0 = 0x0,
    #[doc = "Edge detect on PAD1."]
    PAD1 = 0x01,
    #[doc = "Edge detect on PAD2."]
    PAD2 = 0x02,
    #[doc = "Edge detect on PAD3."]
    PAD3 = 0x03,
    #[doc = "Edge detect on PAD4."]
    PAD4 = 0x04,
    #[doc = "Edge detect on PAD5."]
    PAD5 = 0x05,
    #[doc = "Edge detect on PAD6."]
    PAD6 = 0x06,
    #[doc = "Edge detect on PAD7."]
    PAD7 = 0x07,
    #[doc = "Edge detect on PAD8."]
    PAD8 = 0x08,
    #[doc = "Edge detect on PAD9."]
    PAD9 = 0x09,
    #[doc = "Edge detect on PAD10."]
    PAD10 = 0x0a,
    #[doc = "Edge detect on PAD11."]
    PAD11 = 0x0b,
    #[doc = "Edge detect on PAD12."]
    PAD12 = 0x0c,
    #[doc = "Edge detect on PAD13."]
    PAD13 = 0x0d,
    #[doc = "Edge detect on PAD14."]
    PAD14 = 0x0e,
    #[doc = "Edge detect on PAD15."]
    PAD15 = 0x0f,
    #[doc = "Edge detect on PAD16."]
    PAD16 = 0x10,
    #[doc = "Edge detect on PAD17."]
    PAD17 = 0x11,
    #[doc = "Edge detect on PAD18."]
    PAD18 = 0x12,
    #[doc = "Edge detect on PAD19."]
    PAD19 = 0x13,
    #[doc = "Edge detect on PAD20."]
    PAD20 = 0x14,
    #[doc = "Edge detect on PAD21."]
    PAD21 = 0x15,
    #[doc = "Edge detect on PAD22."]
    PAD22 = 0x16,
    #[doc = "Edge detect on PAD23."]
    PAD23 = 0x17,
    #[doc = "Edge detect on PAD24."]
    PAD24 = 0x18,
    #[doc = "Edge detect on PAD25."]
    PAD25 = 0x19,
    #[doc = "Edge detect on PAD26."]
    PAD26 = 0x1a,
    #[doc = "Edge detect on PAD27."]
    PAD27 = 0x1b,
    #[doc = "Edge detect on PAD28."]
    PAD28 = 0x1c,
    #[doc = "Edge detect on PAD29."]
    PAD29 = 0x1d,
    #[doc = "Edge detect on PAD30."]
    PAD30 = 0x1e,
    #[doc = "Edge detect on PAD31."]
    PAD31 = 0x1f,
    #[doc = "Edge detect on any PAD."]
    PAD = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "RTC channel 0 event."]
    RTC_CH0 = 0x23,
    #[doc = "RTC channel 1 event."]
    RTC_CH1 = 0x24,
    #[doc = "RTC channel 2 event."]
    RTC_CH2 = 0x25,
    #[doc = "RTC channel 0 - delayed event."]
    RTC_CH0_DLY = 0x26,
    #[doc = "RTC channel 1 - delayed event."]
    RTC_CH1_DLY = 0x27,
    #[doc = "RTC channel 2 - delayed event."]
    RTC_CH2_DLY = 0x28,
    #[doc = "RTC combined delayed event."]
    RTC_COMB_DLY = 0x29,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)."]
    RTC_UPD = 0x2a,
    #[doc = "JTAG generated event."]
    JTAG = 0x2b,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0."]
    AUX_SWEV0 = 0x2c,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1."]
    AUX_SWEV1 = 0x2d,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2."]
    AUX_SWEV2 = 0x2e,
    #[doc = "Comparator A triggered."]
    AUX_COMPA = 0x2f,
    #[doc = "Comparator B triggered."]
    AUX_COMPB = 0x30,
    #[doc = "ADC conversion completed."]
    AUX_ADC_DONE = 0x31,
    #[doc = "TDC completed or timed out."]
    AUX_TDC_DONE = 0x32,
    #[doc = "AUX Timer 0 Event."]
    AUX_TIMER0_EV = 0x33,
    #[doc = "AUX Timer 1 Event."]
    AUX_TIMER1_EV = 0x34,
    #[doc = "BATMON temperature update event."]
    BATMON_TEMP = 0x35,
    #[doc = "BATMON voltage update event."]
    BATMON_VOLT = 0x36,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC = 0x37,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC_N = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "No event, always low."]
    NONE = 0x3f,
}
impl MCUWUSEL_WU0_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MCUWUSEL_WU0_EV {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MCUWUSEL_WU0_EV {
    #[inline(always)]
    fn from(val: u8) -> MCUWUSEL_WU0_EV {
        MCUWUSEL_WU0_EV::from_bits(val)
    }
}
impl From<MCUWUSEL_WU0_EV> for u8 {
    #[inline(always)]
    fn from(val: MCUWUSEL_WU0_EV) -> u8 {
        MCUWUSEL_WU0_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MCUWUSEL_WU1_EV {
    #[doc = "Edge detect on PAD0."]
    PAD0 = 0x0,
    #[doc = "Edge detect on PAD1."]
    PAD1 = 0x01,
    #[doc = "Edge detect on PAD2."]
    PAD2 = 0x02,
    #[doc = "Edge detect on PAD3."]
    PAD3 = 0x03,
    #[doc = "Edge detect on PAD4."]
    PAD4 = 0x04,
    #[doc = "Edge detect on PAD5."]
    PAD5 = 0x05,
    #[doc = "Edge detect on PAD6."]
    PAD6 = 0x06,
    #[doc = "Edge detect on PAD7."]
    PAD7 = 0x07,
    #[doc = "Edge detect on PAD8."]
    PAD8 = 0x08,
    #[doc = "Edge detect on PAD9."]
    PAD9 = 0x09,
    #[doc = "Edge detect on PAD10."]
    PAD10 = 0x0a,
    #[doc = "Edge detect on PAD11."]
    PAD11 = 0x0b,
    #[doc = "Edge detect on PAD12."]
    PAD12 = 0x0c,
    #[doc = "Edge detect on PAD13."]
    PAD13 = 0x0d,
    #[doc = "Edge detect on PAD14."]
    PAD14 = 0x0e,
    #[doc = "Edge detect on PAD15."]
    PAD15 = 0x0f,
    #[doc = "Edge detect on PAD16."]
    PAD16 = 0x10,
    #[doc = "Edge detect on PAD17."]
    PAD17 = 0x11,
    #[doc = "Edge detect on PAD18."]
    PAD18 = 0x12,
    #[doc = "Edge detect on PAD19."]
    PAD19 = 0x13,
    #[doc = "Edge detect on PAD20."]
    PAD20 = 0x14,
    #[doc = "Edge detect on PAD21."]
    PAD21 = 0x15,
    #[doc = "Edge detect on PAD22."]
    PAD22 = 0x16,
    #[doc = "Edge detect on PAD23."]
    PAD23 = 0x17,
    #[doc = "Edge detect on PAD24."]
    PAD24 = 0x18,
    #[doc = "Edge detect on PAD25."]
    PAD25 = 0x19,
    #[doc = "Edge detect on PAD26."]
    PAD26 = 0x1a,
    #[doc = "Edge detect on PAD27."]
    PAD27 = 0x1b,
    #[doc = "Edge detect on PAD28."]
    PAD28 = 0x1c,
    #[doc = "Edge detect on PAD29."]
    PAD29 = 0x1d,
    #[doc = "Edge detect on PAD30."]
    PAD30 = 0x1e,
    #[doc = "Edge detect on PAD31."]
    PAD31 = 0x1f,
    #[doc = "Edge detect on any PAD."]
    PAD = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "RTC channel 0 event."]
    RTC_CH0 = 0x23,
    #[doc = "RTC channel 1 event."]
    RTC_CH1 = 0x24,
    #[doc = "RTC channel 2 event."]
    RTC_CH2 = 0x25,
    #[doc = "RTC channel 0 - delayed event."]
    RTC_CH0_DLY = 0x26,
    #[doc = "RTC channel 1 - delayed event."]
    RTC_CH1_DLY = 0x27,
    #[doc = "RTC channel 2 - delayed event."]
    RTC_CH2_DLY = 0x28,
    #[doc = "RTC combined delayed event."]
    RTC_COMB_DLY = 0x29,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)."]
    RTC_UPD = 0x2a,
    #[doc = "JTAG generated event."]
    JTAG = 0x2b,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0."]
    AUX_SWEV0 = 0x2c,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1."]
    AUX_SWEV1 = 0x2d,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2."]
    AUX_SWEV2 = 0x2e,
    #[doc = "Comparator A triggered."]
    AUX_COMPA = 0x2f,
    #[doc = "Comparator B triggered."]
    AUX_COMPB = 0x30,
    #[doc = "ADC conversion completed."]
    AUX_ADC_DONE = 0x31,
    #[doc = "TDC completed or timed out."]
    AUX_TDC_DONE = 0x32,
    #[doc = "AUX Timer 0 Event."]
    AUX_TIMER0_EV = 0x33,
    #[doc = "AUX Timer 1 Event."]
    AUX_TIMER1_EV = 0x34,
    #[doc = "BATMON temperature update event."]
    BATMON_TEMP = 0x35,
    #[doc = "BATMON voltage update event."]
    BATMON_VOLT = 0x36,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC = 0x37,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC_N = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "No event, always low."]
    NONE = 0x3f,
}
impl MCUWUSEL_WU1_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MCUWUSEL_WU1_EV {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MCUWUSEL_WU1_EV {
    #[inline(always)]
    fn from(val: u8) -> MCUWUSEL_WU1_EV {
        MCUWUSEL_WU1_EV::from_bits(val)
    }
}
impl From<MCUWUSEL_WU1_EV> for u8 {
    #[inline(always)]
    fn from(val: MCUWUSEL_WU1_EV) -> u8 {
        MCUWUSEL_WU1_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MCUWUSEL_WU2_EV {
    #[doc = "Edge detect on PAD0."]
    PAD0 = 0x0,
    #[doc = "Edge detect on PAD1."]
    PAD1 = 0x01,
    #[doc = "Edge detect on PAD2."]
    PAD2 = 0x02,
    #[doc = "Edge detect on PAD3."]
    PAD3 = 0x03,
    #[doc = "Edge detect on PAD4."]
    PAD4 = 0x04,
    #[doc = "Edge detect on PAD5."]
    PAD5 = 0x05,
    #[doc = "Edge detect on PAD6."]
    PAD6 = 0x06,
    #[doc = "Edge detect on PAD7."]
    PAD7 = 0x07,
    #[doc = "Edge detect on PAD8."]
    PAD8 = 0x08,
    #[doc = "Edge detect on PAD9."]
    PAD9 = 0x09,
    #[doc = "Edge detect on PAD10."]
    PAD10 = 0x0a,
    #[doc = "Edge detect on PAD11."]
    PAD11 = 0x0b,
    #[doc = "Edge detect on PAD12."]
    PAD12 = 0x0c,
    #[doc = "Edge detect on PAD13."]
    PAD13 = 0x0d,
    #[doc = "Edge detect on PAD14."]
    PAD14 = 0x0e,
    #[doc = "Edge detect on PAD15."]
    PAD15 = 0x0f,
    #[doc = "Edge detect on PAD16."]
    PAD16 = 0x10,
    #[doc = "Edge detect on PAD17."]
    PAD17 = 0x11,
    #[doc = "Edge detect on PAD18."]
    PAD18 = 0x12,
    #[doc = "Edge detect on PAD19."]
    PAD19 = 0x13,
    #[doc = "Edge detect on PAD20."]
    PAD20 = 0x14,
    #[doc = "Edge detect on PAD21."]
    PAD21 = 0x15,
    #[doc = "Edge detect on PAD22."]
    PAD22 = 0x16,
    #[doc = "Edge detect on PAD23."]
    PAD23 = 0x17,
    #[doc = "Edge detect on PAD24."]
    PAD24 = 0x18,
    #[doc = "Edge detect on PAD25."]
    PAD25 = 0x19,
    #[doc = "Edge detect on PAD26."]
    PAD26 = 0x1a,
    #[doc = "Edge detect on PAD27."]
    PAD27 = 0x1b,
    #[doc = "Edge detect on PAD28."]
    PAD28 = 0x1c,
    #[doc = "Edge detect on PAD29."]
    PAD29 = 0x1d,
    #[doc = "Edge detect on PAD30."]
    PAD30 = 0x1e,
    #[doc = "Edge detect on PAD31."]
    PAD31 = 0x1f,
    #[doc = "Edge detect on any PAD."]
    PAD = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "RTC channel 0 event."]
    RTC_CH0 = 0x23,
    #[doc = "RTC channel 1 event."]
    RTC_CH1 = 0x24,
    #[doc = "RTC channel 2 event."]
    RTC_CH2 = 0x25,
    #[doc = "RTC channel 0 - delayed event."]
    RTC_CH0_DLY = 0x26,
    #[doc = "RTC channel 1 - delayed event."]
    RTC_CH1_DLY = 0x27,
    #[doc = "RTC channel 2 - delayed event."]
    RTC_CH2_DLY = 0x28,
    #[doc = "RTC combined delayed event."]
    RTC_COMB_DLY = 0x29,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)."]
    RTC_UPD = 0x2a,
    #[doc = "JTAG generated event."]
    JTAG = 0x2b,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0."]
    AUX_SWEV0 = 0x2c,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1."]
    AUX_SWEV1 = 0x2d,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2."]
    AUX_SWEV2 = 0x2e,
    #[doc = "Comparator A triggered."]
    AUX_COMPA = 0x2f,
    #[doc = "Comparator B triggered."]
    AUX_COMPB = 0x30,
    #[doc = "ADC conversion completed."]
    AUX_ADC_DONE = 0x31,
    #[doc = "TDC completed or timed out."]
    AUX_TDC_DONE = 0x32,
    #[doc = "AUX Timer 0 Event."]
    AUX_TIMER0_EV = 0x33,
    #[doc = "AUX Timer 1 Event."]
    AUX_TIMER1_EV = 0x34,
    #[doc = "BATMON temperature update event."]
    BATMON_TEMP = 0x35,
    #[doc = "BATMON voltage update event."]
    BATMON_VOLT = 0x36,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC = 0x37,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC_N = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "No event, always low."]
    NONE = 0x3f,
}
impl MCUWUSEL_WU2_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MCUWUSEL_WU2_EV {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MCUWUSEL_WU2_EV {
    #[inline(always)]
    fn from(val: u8) -> MCUWUSEL_WU2_EV {
        MCUWUSEL_WU2_EV::from_bits(val)
    }
}
impl From<MCUWUSEL_WU2_EV> for u8 {
    #[inline(always)]
    fn from(val: MCUWUSEL_WU2_EV) -> u8 {
        MCUWUSEL_WU2_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RTC_CH1_CAPT_EV {
    #[doc = "Edge detect on PAD0."]
    PAD0 = 0x0,
    #[doc = "Edge detect on PAD1."]
    PAD1 = 0x01,
    #[doc = "Edge detect on PAD2."]
    PAD2 = 0x02,
    #[doc = "Edge detect on PAD3."]
    PAD3 = 0x03,
    #[doc = "Edge detect on PAD4."]
    PAD4 = 0x04,
    #[doc = "Edge detect on PAD5."]
    PAD5 = 0x05,
    #[doc = "Edge detect on PAD6."]
    PAD6 = 0x06,
    #[doc = "Edge detect on PAD7."]
    PAD7 = 0x07,
    #[doc = "Edge detect on PAD8."]
    PAD8 = 0x08,
    #[doc = "Edge detect on PAD9."]
    PAD9 = 0x09,
    #[doc = "Edge detect on PAD10."]
    PAD10 = 0x0a,
    #[doc = "Edge detect on PAD11."]
    PAD11 = 0x0b,
    #[doc = "Edge detect on PAD12."]
    PAD12 = 0x0c,
    #[doc = "Edge detect on PAD13."]
    PAD13 = 0x0d,
    #[doc = "Edge detect on PAD14."]
    PAD14 = 0x0e,
    #[doc = "Edge detect on PAD15."]
    PAD15 = 0x0f,
    #[doc = "Edge detect on PAD16."]
    PAD16 = 0x10,
    #[doc = "Edge detect on PAD17."]
    PAD17 = 0x11,
    #[doc = "Edge detect on PAD18."]
    PAD18 = 0x12,
    #[doc = "Edge detect on PAD19."]
    PAD19 = 0x13,
    #[doc = "Edge detect on PAD20."]
    PAD20 = 0x14,
    #[doc = "Edge detect on PAD21."]
    PAD21 = 0x15,
    #[doc = "Edge detect on PAD22."]
    PAD22 = 0x16,
    #[doc = "Edge detect on PAD23."]
    PAD23 = 0x17,
    #[doc = "Edge detect on PAD24."]
    PAD24 = 0x18,
    #[doc = "Edge detect on PAD25."]
    PAD25 = 0x19,
    #[doc = "Edge detect on PAD26."]
    PAD26 = 0x1a,
    #[doc = "Edge detect on PAD27."]
    PAD27 = 0x1b,
    #[doc = "Edge detect on PAD28."]
    PAD28 = 0x1c,
    #[doc = "Edge detect on PAD29."]
    PAD29 = 0x1d,
    #[doc = "Edge detect on PAD30."]
    PAD30 = 0x1e,
    #[doc = "Edge detect on PAD31."]
    PAD31 = 0x1f,
    #[doc = "Edge detect on any PAD."]
    PAD = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "RTC channel 0 event."]
    RTC_CH0 = 0x23,
    #[doc = "RTC channel 1 event."]
    RTC_CH1 = 0x24,
    #[doc = "RTC channel 2 event."]
    RTC_CH2 = 0x25,
    #[doc = "RTC channel 0 - delayed event."]
    RTC_CH0_DLY = 0x26,
    #[doc = "RTC channel 1 - delayed event."]
    RTC_CH1_DLY = 0x27,
    #[doc = "RTC channel 2 - delayed event."]
    RTC_CH2_DLY = 0x28,
    #[doc = "RTC combined delayed event."]
    RTC_COMB_DLY = 0x29,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)."]
    RTC_UPD = 0x2a,
    #[doc = "JTAG generated event."]
    JTAG = 0x2b,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0."]
    AUX_SWEV0 = 0x2c,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1."]
    AUX_SWEV1 = 0x2d,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2."]
    AUX_SWEV2 = 0x2e,
    #[doc = "Comparator A triggered."]
    AUX_COMPA = 0x2f,
    #[doc = "Comparator B triggered."]
    AUX_COMPB = 0x30,
    #[doc = "ADC conversion completed."]
    AUX_ADC_DONE = 0x31,
    #[doc = "TDC completed or timed out."]
    AUX_TDC_DONE = 0x32,
    #[doc = "AUX Timer 0 Event."]
    AUX_TIMER0_EV = 0x33,
    #[doc = "AUX Timer 1 Event."]
    AUX_TIMER1_EV = 0x34,
    #[doc = "BATMON temperature update event."]
    BATMON_TEMP = 0x35,
    #[doc = "BATMON voltage update event."]
    BATMON_VOLT = 0x36,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC = 0x37,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC_N = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "No event, always low."]
    NONE = 0x3f,
}
impl RTC_CH1_CAPT_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RTC_CH1_CAPT_EV {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RTC_CH1_CAPT_EV {
    #[inline(always)]
    fn from(val: u8) -> RTC_CH1_CAPT_EV {
        RTC_CH1_CAPT_EV::from_bits(val)
    }
}
impl From<RTC_CH1_CAPT_EV> for u8 {
    #[inline(always)]
    fn from(val: RTC_CH1_CAPT_EV) -> u8 {
        RTC_CH1_CAPT_EV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WU3_EV {
    #[doc = "Edge detect on PAD0."]
    PAD0 = 0x0,
    #[doc = "Edge detect on PAD1."]
    PAD1 = 0x01,
    #[doc = "Edge detect on PAD2."]
    PAD2 = 0x02,
    #[doc = "Edge detect on PAD3."]
    PAD3 = 0x03,
    #[doc = "Edge detect on PAD4."]
    PAD4 = 0x04,
    #[doc = "Edge detect on PAD5."]
    PAD5 = 0x05,
    #[doc = "Edge detect on PAD6."]
    PAD6 = 0x06,
    #[doc = "Edge detect on PAD7."]
    PAD7 = 0x07,
    #[doc = "Edge detect on PAD8."]
    PAD8 = 0x08,
    #[doc = "Edge detect on PAD9."]
    PAD9 = 0x09,
    #[doc = "Edge detect on PAD10."]
    PAD10 = 0x0a,
    #[doc = "Edge detect on PAD11."]
    PAD11 = 0x0b,
    #[doc = "Edge detect on PAD12."]
    PAD12 = 0x0c,
    #[doc = "Edge detect on PAD13."]
    PAD13 = 0x0d,
    #[doc = "Edge detect on PAD14."]
    PAD14 = 0x0e,
    #[doc = "Edge detect on PAD15."]
    PAD15 = 0x0f,
    #[doc = "Edge detect on PAD16."]
    PAD16 = 0x10,
    #[doc = "Edge detect on PAD17."]
    PAD17 = 0x11,
    #[doc = "Edge detect on PAD18."]
    PAD18 = 0x12,
    #[doc = "Edge detect on PAD19."]
    PAD19 = 0x13,
    #[doc = "Edge detect on PAD20."]
    PAD20 = 0x14,
    #[doc = "Edge detect on PAD21."]
    PAD21 = 0x15,
    #[doc = "Edge detect on PAD22."]
    PAD22 = 0x16,
    #[doc = "Edge detect on PAD23."]
    PAD23 = 0x17,
    #[doc = "Edge detect on PAD24."]
    PAD24 = 0x18,
    #[doc = "Edge detect on PAD25."]
    PAD25 = 0x19,
    #[doc = "Edge detect on PAD26."]
    PAD26 = 0x1a,
    #[doc = "Edge detect on PAD27."]
    PAD27 = 0x1b,
    #[doc = "Edge detect on PAD28."]
    PAD28 = 0x1c,
    #[doc = "Edge detect on PAD29."]
    PAD29 = 0x1d,
    #[doc = "Edge detect on PAD30."]
    PAD30 = 0x1e,
    #[doc = "Edge detect on PAD31."]
    PAD31 = 0x1f,
    #[doc = "Edge detect on any PAD."]
    PAD = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "RTC channel 0 event."]
    RTC_CH0 = 0x23,
    #[doc = "RTC channel 1 event."]
    RTC_CH1 = 0x24,
    #[doc = "RTC channel 2 event."]
    RTC_CH2 = 0x25,
    #[doc = "RTC channel 0 - delayed event."]
    RTC_CH0_DLY = 0x26,
    #[doc = "RTC channel 1 - delayed event."]
    RTC_CH1_DLY = 0x27,
    #[doc = "RTC channel 2 - delayed event."]
    RTC_CH2_DLY = 0x28,
    #[doc = "RTC combined delayed event."]
    RTC_COMB_DLY = 0x29,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)."]
    RTC_UPD = 0x2a,
    #[doc = "JTAG generated event."]
    JTAG = 0x2b,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0."]
    AUX_SWEV0 = 0x2c,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1."]
    AUX_SWEV1 = 0x2d,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2."]
    AUX_SWEV2 = 0x2e,
    #[doc = "Comparator A triggered."]
    AUX_COMPA = 0x2f,
    #[doc = "Comparator B triggered."]
    AUX_COMPB = 0x30,
    #[doc = "ADC conversion completed."]
    AUX_ADC_DONE = 0x31,
    #[doc = "TDC completed or timed out."]
    AUX_TDC_DONE = 0x32,
    #[doc = "AUX Timer 0 Event."]
    AUX_TIMER0_EV = 0x33,
    #[doc = "AUX Timer 1 Event."]
    AUX_TIMER1_EV = 0x34,
    #[doc = "BATMON temperature update event."]
    BATMON_TEMP = 0x35,
    #[doc = "BATMON voltage update event."]
    BATMON_VOLT = 0x36,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC = 0x37,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX."]
    AUX_COMPB_ASYNC_N = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "No event, always low."]
    NONE = 0x3f,
}
impl WU3_EV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WU3_EV {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WU3_EV {
    #[inline(always)]
    fn from(val: u8) -> WU3_EV {
        WU3_EV::from_bits(val)
    }
}
impl From<WU3_EV> for u8 {
    #[inline(always)]
    fn from(val: WU3_EV) -> u8 {
        WU3_EV::to_bits(val)
    }
}

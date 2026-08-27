unsafe extern "C" {
    fn GPIO();
    fn I2C();
    fn RFC_PE0();
    fn AON_RTC();
    fn UART0();
    fn UART1();
    fn SSI0();
    fn SSI1();
    fn RFC_PE1();
    fn RFC();
    fn RFC_CA();
    fn I2S();
    fn WDT();
    fn GPT0A();
    fn GPT0B();
    fn GPT1A();
    fn GPT1B();
    fn GPT2A();
    fn GPT2B();
    fn GPT3A();
    fn GPT3B();
    fn CRYPTO();
    fn UDMA();
    fn UDMA_ERR();
    fn FLASH();
    fn SWE0();
    fn AUX_CE();
    fn AON_EVENT();
    fn DYN_EVENT();
    fn AUX_COMPA();
    fn AUX_MISC();
    fn TRNG();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 34] = [
    Vector { _handler: GPIO },
    Vector { _handler: I2C },
    Vector { _handler: RFC_PE0 },
    Vector { _reserved: 0 },
    Vector { _handler: AON_RTC },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: SSI0 },
    Vector { _handler: SSI1 },
    Vector { _handler: RFC_PE1 },
    Vector { _handler: RFC },
    Vector { _handler: RFC_CA },
    Vector { _handler: I2S },
    Vector { _reserved: 0 },
    Vector { _handler: WDT },
    Vector { _handler: GPT0A },
    Vector { _handler: GPT0B },
    Vector { _handler: GPT1A },
    Vector { _handler: GPT1B },
    Vector { _handler: GPT2A },
    Vector { _handler: GPT2B },
    Vector { _handler: GPT3A },
    Vector { _handler: GPT3B },
    Vector { _handler: CRYPTO },
    Vector { _handler: UDMA },
    Vector { _handler: UDMA_ERR },
    Vector { _handler: FLASH },
    Vector { _handler: SWE0 },
    Vector { _handler: AUX_CE },
    Vector {
        _handler: AON_EVENT,
    },
    Vector {
        _handler: DYN_EVENT,
    },
    Vector {
        _handler: AUX_COMPA,
    },
    Vector { _handler: AUX_MISC },
    Vector { _handler: TRNG },
];

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#![no_std]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - GPIO edge detect"]
    GPIO = 0,
    #[doc = "1 - I2C"]
    I2C = 1,
    #[doc = "2 - RF Core and packet engine 1"]
    RFC_PE0 = 2,
    #[doc = "4 - AON RTC"]
    AON_RTC = 4,
    #[doc = "5 - UART0"]
    UART0 = 5,
    #[doc = "6 - UART1"]
    UART1 = 6,
    #[doc = "7 - SSI0"]
    SSI0 = 7,
    #[doc = "8 - SSI1"]
    SSI1 = 8,
    #[doc = "9 - RF Core and packet engine 2"]
    RFC_PE1 = 9,
    #[doc = "10 - RF Core hardware"]
    RFC = 10,
    #[doc = "11 - RF command acknowledge"]
    RFC_CA = 11,
    #[doc = "12 - I2S"]
    I2S = 12,
    #[doc = "14 - Watchdog timer"]
    WDT = 14,
    #[doc = "15 - GPTimer 0A"]
    GPT0A = 15,
    #[doc = "16 - GPTimer 0B"]
    GPT0B = 16,
    #[doc = "17 - GPTimer 1A"]
    GPT1A = 17,
    #[doc = "18 - GPTimer 1B"]
    GPT1B = 18,
    #[doc = "19 - GPTimer 2A"]
    GPT2A = 19,
    #[doc = "20 - GPTimer 2B"]
    GPT2B = 20,
    #[doc = "21 - GPTimer 3A"]
    GPT3A = 21,
    #[doc = "22 - GPTimer 3B"]
    GPT3B = 22,
    #[doc = "23 - Crypto"]
    CRYPTO = 23,
    #[doc = "24 - μDMA software defined"]
    UDMA = 24,
    #[doc = "25 - μDMA error"]
    UDMA_ERR = 25,
    #[doc = "26 - Flash"]
    FLASH = 26,
    #[doc = "27 - Software event 0"]
    SWE0 = 27,
    #[doc = "28 - AUX combined event"]
    AUX_CE = 28,
    #[doc = "29 - AON programmable event"]
    AON_EVENT = 29,
    #[doc = "30 - Dynamic programmable event"]
    DYN_EVENT = 30,
    #[doc = "31 - AUX comparator A"]
    AUX_COMPA = 31,
    #[doc = "32 - AUX ADC new sample available or ADC DMA done, ADC underflow and overflow"]
    AUX_MISC = 32,
    #[doc = "33 - True random number generator"]
    TRNG = 33,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
pub const _INTERRUPTS: _INTERRUPTS::_INTERRUPTS =
    unsafe { _INTERRUPTS::_INTERRUPTS::from_ptr(0x0usize as _) };
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub const SSI0: SSI0::SSI0 = unsafe { SSI0::SSI0::from_ptr(0x4000_0000usize as _) };
#[doc = "Universal Asynchronous Receiver/Transmitter (UART) interface"]
pub const UART0: UART0::UART0 = unsafe { UART0::UART0::from_ptr(0x4000_1000usize as _) };
#[doc = "I2CMaster/Slave Serial Controler"]
pub const I2C0: I2C0::I2C0 = unsafe { I2C0::I2C0::from_ptr(0x4000_2000usize as _) };
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub const SSI1: SSI1::SSI1 = unsafe { SSI1::SSI1::from_ptr(0x4000_8000usize as _) };
#[doc = "General Purpose Timer."]
pub const GPT0: GPT0::GPT0 = unsafe { GPT0::GPT0::from_ptr(0x4001_0000usize as _) };
#[doc = "General Purpose Timer."]
pub const GPT1: GPT1::GPT1 = unsafe { GPT1::GPT1::from_ptr(0x4001_1000usize as _) };
#[doc = "General Purpose Timer."]
pub const GPT2: GPT2::GPT2 = unsafe { GPT2::GPT2::from_ptr(0x4001_2000usize as _) };
#[doc = "General Purpose Timer."]
pub const GPT3: GPT3::GPT3 = unsafe { GPT3::GPT3::from_ptr(0x4001_3000usize as _) };
#[doc = "ARM Micro Direct Memory Access Controller"]
pub const UDMA0: UDMA0::UDMA0 = unsafe { UDMA0::UDMA0::from_ptr(0x4002_0000usize as _) };
#[doc = "I2S Audio DMA module supporting formats I2S, LJF, RJF and DSP"]
pub const I2S0: I2S0::I2S0 = unsafe { I2S0::I2S0::from_ptr(0x4002_1000usize as _) };
#[doc = "MCU GPIO - I/F for controlling and reading IO status and IO event status"]
pub const GPIO: GPIO::GPIO = unsafe { GPIO::GPIO::from_ptr(0x4002_2000usize as _) };
#[doc = "Crypto core with DMA capability and local key storage"]
pub const CRYPTO: CRYPTO::CRYPTO = unsafe { CRYPTO::CRYPTO::from_ptr(0x4002_4000usize as _) };
#[doc = "True Random Number Generator"]
pub const TRNG: TRNG::TRNG = unsafe { TRNG::TRNG::from_ptr(0x4002_8000usize as _) };
#[doc = "Flash sub-system registers, includes the Flash Memory Controller (FMC), flash read path, and an integrated Efuse controller and EFUSEROM."]
pub const FLASH: FLASH::FLASH = unsafe { FLASH::FLASH::from_ptr(0x4003_0000usize as _) };
#[doc = "Versatile Instruction Memory System Controls memory access to the Flash and encapsulates the following instruction memories: - Boot ROM - Cache / GPRAM"]
pub const VIMS: VIMS::VIMS = unsafe { VIMS::VIMS::from_ptr(0x4003_4000usize as _) };
#[doc = "RF Core Power Management"]
pub const RFC_PWR: RFC_PWR::RFC_PWR = unsafe { RFC_PWR::RFC_PWR::from_ptr(0x4004_0000usize as _) };
#[doc = "RF Core Doorbell"]
pub const RFC_DBELL: RFC_DBELL::RFC_DBELL =
    unsafe { RFC_DBELL::RFC_DBELL::from_ptr(0x4004_1000usize as _) };
#[doc = "RF Core Radio Timer"]
pub const RFC_RAT: RFC_RAT::RFC_RAT = unsafe { RFC_RAT::RFC_RAT::from_ptr(0x4004_3000usize as _) };
#[doc = "Watchdog Timer"]
pub const WDT: WDT::WDT = unsafe { WDT::WDT::from_ptr(0x4008_0000usize as _) };
#[doc = "IO Controller (IOC) - configures all the DIOs and resides in the MCU domain."]
pub const IOC: IOC::IOC = unsafe { IOC::IOC::from_ptr(0x4008_1000usize as _) };
#[doc = "Power, Reset and Clock Management"]
pub const PRCM: PRCM::PRCM = unsafe { PRCM::PRCM::from_ptr(0x4008_2000usize as _) };
#[doc = "Event Fabric Component Definition"]
pub const EVENT: EVENT::EVENT = unsafe { EVENT::EVENT::from_ptr(0x4008_3000usize as _) };
#[doc = "MCU Semaphore Module This module provides 32 binary semaphores. The state of a binary semaphore is either taken or available. A semaphore does not implement any ownership attribute. Still, a semaphore can be used to handle mutual exclusion scenarios."]
pub const SMPH: SMPH::SMPH = unsafe { SMPH::SMPH::from_ptr(0x4008_4000usize as _) };
#[doc = "This component controls AON_SYSCTL, which is the device's system controller. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub const AON_SYSCTL: AON_SYSCTL::AON_SYSCTL =
    unsafe { AON_SYSCTL::AON_SYSCTL::from_ptr(0x4009_0000usize as _) };
#[doc = "This component control the Wakeup controller residing in the AON domain. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub const AON_WUC: AON_WUC::AON_WUC = unsafe { AON_WUC::AON_WUC::from_ptr(0x4009_1000usize as _) };
#[doc = "This component control the Real Time Clock residing in AON Note: This module is only supporting 32 bit ReadWrite access."]
pub const AON_RTC: AON_RTC::AON_RTC = unsafe { AON_RTC::AON_RTC::from_ptr(0x4009_2000usize as _) };
#[doc = "This module configures the event fabric located in the AON domain. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub const AON_EVENT: AON_EVENT::AON_EVENT =
    unsafe { AON_EVENT::AON_EVENT::from_ptr(0x4009_3000usize as _) };
#[doc = "Always On (AON) IO Controller - controls IO operation when the MCU IO Controller (IOC) is powered off and resides in the AON domain. Note: This module only supports 32 bit Read/Write access from MCU."]
pub const AON_IOC: AON_IOC::AON_IOC = unsafe { AON_IOC::AON_IOC::from_ptr(0x4009_4000usize as _) };
#[doc = "Always On (AON) Battery And Temperature MONitor (BATMON) residing in the AON domain Note: This module only supports 32 bit Read/Write access from MCU."]
pub const AON_BATMON: AON_BATMON::AON_BATMON =
    unsafe { AON_BATMON::AON_BATMON::from_ptr(0x4009_5000usize as _) };
#[doc = "AUX Analog/Digital Input Output Controller"]
pub const AUX_AIODIO0: AUX_AIODIO0::AUX_AIODIO0 =
    unsafe { AUX_AIODIO0::AUX_AIODIO0::from_ptr(0x400c_1000usize as _) };
#[doc = "AUX Analog/Digital Input Output Controller"]
pub const AUX_AIODIO1: AUX_AIODIO1::AUX_AIODIO1 =
    unsafe { AUX_AIODIO1::AUX_AIODIO1::from_ptr(0x400c_2000usize as _) };
#[doc = "AUX Time To Digital Converter"]
pub const AUX_TDCIF: AUX_TDCIF::AUX_TDCIF =
    unsafe { AUX_TDCIF::AUX_TDCIF::from_ptr(0x400c_4000usize as _) };
#[doc = "AUX Event Controller"]
pub const AUX_EVCTL: AUX_EVCTL::AUX_EVCTL =
    unsafe { AUX_EVCTL::AUX_EVCTL::from_ptr(0x400c_5000usize as _) };
#[doc = "AUX Wake-up controller"]
pub const AUX_WUC: AUX_WUC::AUX_WUC = unsafe { AUX_WUC::AUX_WUC::from_ptr(0x400c_6000usize as _) };
#[doc = "AUX Timer"]
pub const AUX_TIMER: AUX_TIMER::AUX_TIMER =
    unsafe { AUX_TIMER::AUX_TIMER::from_ptr(0x400c_7000usize as _) };
#[doc = "AUX Semaphore Controller"]
pub const AUX_SMPH: AUX_SMPH::AUX_SMPH =
    unsafe { AUX_SMPH::AUX_SMPH::from_ptr(0x400c_8000usize as _) };
#[doc = "AUX Analog Peripheral Control Module"]
pub const AUX_ANAIF: AUX_ANAIF::AUX_ANAIF =
    unsafe { AUX_ANAIF::AUX_ANAIF::from_ptr(0x400c_9000usize as _) };
#[doc = "This is the DDI for the digital block that controls all the analog clock oscillators (OSC_DIG) and performs qualification of the clocks generated."]
pub const AUX_DDI0_OSC: AUX_DDI0_OSC::AUX_DDI0_OSC =
    unsafe { AUX_DDI0_OSC::AUX_DDI0_OSC::from_ptr(0x400c_a000usize as _) };
#[doc = "Configuration registers controlling analog peripherals of AUX. Registers Fields should be considered static unless otherwise noted (as dynamic)"]
pub const AUX_ADI4: AUX_ADI4::AUX_ADI4 =
    unsafe { AUX_ADI4::AUX_ADI4::from_ptr(0x400c_b000usize as _) };
#[doc = "AUX Sensor Control Engine Control Module"]
pub const AUX_SCE: AUX_SCE::AUX_SCE = unsafe { AUX_SCE::AUX_SCE::from_ptr(0x400e_1000usize as _) };
#[doc = "Factory configuration area (FCFG1)"]
pub const FCFG1: FCFG1::FCFG1 = unsafe { FCFG1::FCFG1::from_ptr(0x5000_1000usize as _) };
#[doc = "Customer configuration area (CCFG)"]
pub const CCFG: CCFG::CCFG = unsafe { CCFG::CCFG::from_ptr(0x5000_3000usize as _) };
#[doc = "Cortex-M's Instrumentation Trace Macrocell (ITM)"]
pub const CPU_ITM: CPU_ITM::CPU_ITM = unsafe { CPU_ITM::CPU_ITM::from_ptr(0xe000_0000usize as _) };
#[doc = "Cortex-M's Data watchpoint and Trace (DWT)"]
pub const CPU_DWT: CPU_DWT::CPU_DWT = unsafe { CPU_DWT::CPU_DWT::from_ptr(0xe000_1000usize as _) };
#[doc = "Cortex-M's Flash Patch and Breakpoint (FPB)"]
pub const CPU_FPB: CPU_FPB::CPU_FPB = unsafe { CPU_FPB::CPU_FPB::from_ptr(0xe000_2000usize as _) };
#[doc = "Cortex-M's System Control Space (SCS)"]
pub const CPU_SCS: CPU_SCS::CPU_SCS = unsafe { CPU_SCS::CPU_SCS::from_ptr(0xe000_e000usize as _) };
#[doc = "Cortex-M3's Trace Port Interface Unit (TPIU)"]
pub const CPU_TPIU: CPU_TPIU::CPU_TPIU =
    unsafe { CPU_TPIU::CPU_TPIU::from_ptr(0xe004_0000usize as _) };
#[doc = "Cortex-M's TI proprietary registers"]
pub const CPU_TIPROP: CPU_TIPROP::CPU_TIPROP =
    unsafe { CPU_TIPROP::CPU_TIPROP::from_ptr(0xe00f_e000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
pub mod AON_BATMON;
pub mod AON_EVENT;
pub mod AON_IOC;
pub mod AON_RTC;
pub mod AON_SYSCTL;
pub mod AON_WUC;
pub mod AUX_ADI4;
pub mod AUX_AIODIO0;
pub mod AUX_AIODIO1;
pub mod AUX_ANAIF;
pub mod AUX_DDI0_OSC;
pub mod AUX_EVCTL;
pub mod AUX_SCE;
pub mod AUX_SMPH;
pub mod AUX_TDCIF;
pub mod AUX_TIMER;
pub mod AUX_WUC;
pub mod CCFG;
pub mod CPU_DWT;
pub mod CPU_FPB;
pub mod CPU_ITM;
pub mod CPU_SCS;
pub mod CPU_TIPROP;
pub mod CPU_TPIU;
pub mod CRYPTO;
pub mod EVENT;
pub mod FCFG1;
pub mod FLASH;
pub mod GPIO;
pub mod GPT0;
pub mod GPT1;
pub mod GPT2;
pub mod GPT3;
pub mod I2C0;
pub mod I2S0;
pub mod IOC;
pub mod PRCM;
pub mod RFC_DBELL;
pub mod RFC_PWR;
pub mod RFC_RAT;
pub mod SMPH;
pub mod SSI0;
pub mod SSI1;
pub mod TRNG;
pub mod UART0;
pub mod UDMA0;
pub mod VIMS;
pub mod WDT;
pub mod _INTERRUPTS;
pub mod common;

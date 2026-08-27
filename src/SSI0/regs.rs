#[doc = "Clock Prescale."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPSR(pub u32);
impl CPSR {
    #[doc = "7:0\\] Clock prescale divisor: This field specifies the division factor by which the input system clock to SSI must be internally divided before further use. The value programmed into this field must be an even non-zero number (2-254). The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero."]
    #[must_use]
    #[inline(always)]
    pub const fn CPSDVSR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Clock prescale divisor: This field specifies the division factor by which the input system clock to SSI must be internally divided before further use. The value programmed into this field must be an even non-zero number (2-254). The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero."]
    #[inline(always)]
    pub const fn set_CPSDVSR(&mut self, val: u8) {
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
impl Default for CPSR {
    #[inline(always)]
    fn default() -> CPSR {
        CPSR(0)
    }
}
impl core::fmt::Debug for CPSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPSR")
            .field("CPSDVSR", &self.CPSDVSR())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPSR {{ CPSDVSR: {=u8:?}, RESERVED: {=u32:?} }}",
            self.CPSDVSR(),
            self.RESERVED()
        )
    }
}
#[doc = "Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CR0(pub u32);
impl CR0 {
    #[doc = "3:0\\] Data Size Select. Values 0b0000, 0b0001, 0b0010 are reserved and shall not be used."]
    #[must_use]
    #[inline(always)]
    pub const fn DSS(&self) -> super::vals::DSS {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::DSS::from_bits(val as u8)
    }
    #[doc = "3:0\\] Data Size Select. Values 0b0000, 0b0001, 0b0010 are reserved and shall not be used."]
    #[inline(always)]
    pub const fn set_DSS(&mut self, val: super::vals::DSS) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "5:4\\] Frame format. The supported frame formats are Motorola SPI, TI synchronous serial and National Microwire. Value 0'b11 is reserved and shall not be used."]
    #[must_use]
    #[inline(always)]
    pub const fn FRF(&self) -> super::vals::FRF {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::FRF::from_bits(val as u8)
    }
    #[doc = "5:4\\] Frame format. The supported frame formats are Motorola SPI, TI synchronous serial and National Microwire. Value 0'b11 is reserved and shall not be used."]
    #[inline(always)]
    pub const fn set_FRF(&mut self, val: super::vals::FRF) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "6:6\\] CLKOUT polarity (Motorola SPI frame format only)."]
    #[must_use]
    #[inline(always)]
    pub const fn SPO(&self) -> super::vals::SPO {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SPO::from_bits(val as u8)
    }
    #[doc = "6:6\\] CLKOUT polarity (Motorola SPI frame format only)."]
    #[inline(always)]
    pub const fn set_SPO(&mut self, val: super::vals::SPO) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[must_use]
    #[inline(always)]
    pub const fn SPH(&self) -> super::vals::SPH {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SPH::from_bits(val as u8)
    }
    #[doc = "7:7\\] CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[inline(always)]
    pub const fn set_SPH(&mut self, val: super::vals::SPH) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "15:8\\] Serial clock rate: This is used to generate the transmit and receive bit rate of the SSI. The bit rate is (SSI's clock frequency)/((SCR+1)*CPSR.CPSDVSR). SCR is a value from 0-255."]
    #[must_use]
    #[inline(always)]
    pub const fn SCR(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Serial clock rate: This is used to generate the transmit and receive bit rate of the SSI. The bit rate is (SSI's clock frequency)/((SCR+1)*CPSR.CPSDVSR). SCR is a value from 0-255."]
    #[inline(always)]
    pub const fn set_SCR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CR0 {
    #[inline(always)]
    fn default() -> CR0 {
        CR0(0)
    }
}
impl core::fmt::Debug for CR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR0")
            .field("DSS", &self.DSS())
            .field("FRF", &self.FRF())
            .field("SPO", &self.SPO())
            .field("SPH", &self.SPH())
            .field("SCR", &self.SCR())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CR0 {{ DSS: {:?}, FRF: {:?}, SPO: {:?}, SPH: {:?}, SCR: {=u8:?}, RESERVED: {=u16:?} }}",
            self.DSS(),
            self.FRF(),
            self.SPO(),
            self.SPH(),
            self.SCR(),
            self.RESERVED()
        )
    }
}
#[doc = "Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CR1(pub u32);
impl CR1 {
    #[doc = "0:0\\] Loop back mode: 0: Normal serial port operation enabled. 1: Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[must_use]
    #[inline(always)]
    pub const fn LBM(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Loop back mode: 0: Normal serial port operation enabled. 1: Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    #[inline(always)]
    pub const fn set_LBM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Synchronous serial interface enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SSE(&self) -> super::vals::SSE {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SSE::from_bits(val as u8)
    }
    #[doc = "1:1\\] Synchronous serial interface enable."]
    #[inline(always)]
    pub const fn set_SSE(&mut self, val: super::vals::SSE) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0."]
    #[must_use]
    #[inline(always)]
    pub const fn MS(&self) -> super::vals::MS {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::MS::from_bits(val as u8)
    }
    #[doc = "2:2\\] Master or slave mode select. This bit can be modified only when SSI is disabled, SSE=0."]
    #[inline(always)]
    pub const fn set_MS(&mut self, val: super::vals::MS) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Slave-mode output disabled This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SSI slave is not supposed to drive the TXD line: 0: SSI can drive the TXD output in slave mode. 1: SSI cannot drive the TXD output in slave mode."]
    #[must_use]
    #[inline(always)]
    pub const fn SOD(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Slave-mode output disabled This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an SSI master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, this bitfield can be set if the SSI slave is not supposed to drive the TXD line: 0: SSI can drive the TXD output in slave mode. 1: SSI cannot drive the TXD output in slave mode."]
    #[inline(always)]
    pub const fn set_SOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for CR1 {
    #[inline(always)]
    fn default() -> CR1 {
        CR1(0)
    }
}
impl core::fmt::Debug for CR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("LBM", &self.LBM())
            .field("SSE", &self.SSE())
            .field("MS", &self.MS())
            .field("SOD", &self.SOD())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CR1 {{ LBM: {=bool:?}, SSE: {:?}, MS: {:?}, SOD: {=bool:?}, RESERVED: {=u32:?} }}",
            self.LBM(),
            self.SSE(),
            self.MS(),
            self.SOD(),
            self.RESERVED()
        )
    }
}
#[doc = "DMA Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACR(pub u32);
impl DMACR {
    #[doc = "0:0\\] Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXDMAE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    pub const fn set_RXDMAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn TXDMAE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    pub const fn set_TXDMAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DMACR {
    #[inline(always)]
    fn default() -> DMACR {
        DMACR(0)
    }
}
impl core::fmt::Debug for DMACR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACR")
            .field("RXDMAE", &self.RXDMAE())
            .field("TXDMAE", &self.TXDMAE())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACR {{ RXDMAE: {=bool:?}, TXDMAE: {=bool:?}, RESERVED: {=u32:?} }}",
            self.RXDMAE(),
            self.TXDMAE(),
            self.RESERVED()
        )
    }
}
#[doc = "Data 16-bits wide data register: When read, the entry in the receive FIFO, pointed to by the current FIFO read pointer, is accessed. As data values are removed by the receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current FIFO write pointer. When written, the entry in the transmit FIFO, pointed to by the write pointer, is written to. Data values are removed from the transmit FIFO one value at a time by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the TXD output pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits. Received data less than 16 bits is automatically right-justified in the receive buffer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DR(pub u32);
impl DR {
    #[doc = "15:0\\] Transmit/receive data The values read from this field or written to this field must be right-justified when SSI is programmed for a data size that is less than 16 bits (CR0.DSS != 0b1111). Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] Transmit/receive data The values read from this field or written to this field must be right-justified when SSI is programmed for a data size that is less than 16 bits (CR0.DSS != 0b1111). Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for DR {
    #[inline(always)]
    fn default() -> DR {
        DR(0)
    }
}
impl core::fmt::Debug for DR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR")
            .field("DATA", &self.DATA())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DR {{ DATA: {=u16:?}, RESERVED: {=u16:?} }}",
            self.DATA(),
            self.RESERVED()
        )
    }
}
#[doc = "Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ICR(pub u32);
impl ICR {
    #[doc = "0:0\\] Clear the receive overrun interrupt: Writing 1 to this field clears the overrun error interrupt (RIS.RORRIS). Writing 0 has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RORIC(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Clear the receive overrun interrupt: Writing 1 to this field clears the overrun error interrupt (RIS.RORRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub const fn set_RORIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Clear the receive timeout interrupt: Writing 1 to this field clears the timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RTIC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Clear the receive timeout interrupt: Writing 1 to this field clears the timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub const fn set_RTIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for ICR {
    #[inline(always)]
    fn default() -> ICR {
        ICR(0)
    }
}
impl core::fmt::Debug for ICR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("RORIC", &self.RORIC())
            .field("RTIC", &self.RTIC())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ICR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ICR {{ RORIC: {=bool:?}, RTIC: {=bool:?}, RESERVED: {=u32:?} }}",
            self.RORIC(),
            self.RTIC(),
            self.RESERVED()
        )
    }
}
#[doc = "Interrupt Mask Set and Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IMSC(pub u32);
impl IMSC {
    #[doc = "0:0\\] Receive overrun interrupt mask: A read returns the current mask for receive overrun interrupt. On a write of 1, the mask for receive overrun interrupt is set which means the interrupt state will be reflected in MIS.RORMIS. A write of 0 clears the mask which means MIS.RORMIS will not reflect the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn RORIM(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Receive overrun interrupt mask: A read returns the current mask for receive overrun interrupt. On a write of 1, the mask for receive overrun interrupt is set which means the interrupt state will be reflected in MIS.RORMIS. A write of 0 clears the mask which means MIS.RORMIS will not reflect the interrupt."]
    #[inline(always)]
    pub const fn set_RORIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Receive timeout interrupt mask: A read returns the current mask for receive timeout interrupt. On a write of 1, the mask for receive timeout interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means MIS.RTMIS will not reflect the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn RTIM(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Receive timeout interrupt mask: A read returns the current mask for receive timeout interrupt. On a write of 1, the mask for receive timeout interrupt is set which means the interrupt state will be reflected in MIS.RTMIS. A write of 0 clears the mask which means MIS.RTMIS will not reflect the interrupt."]
    #[inline(always)]
    pub const fn set_RTIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Receive FIFO interrupt mask: A read returns the current mask for receive FIFO interrupt. On a write of 1, the mask for receive FIFO interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn RXIM(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Receive FIFO interrupt mask: A read returns the current mask for receive FIFO interrupt. On a write of 1, the mask for receive FIFO interrupt is set which means the interrupt state will be reflected in MIS.RXMIS. A write of 0 clears the mask which means MIS.RXMIS will not reflect the interrupt."]
    #[inline(always)]
    pub const fn set_RXIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Transmit FIFO interrupt mask: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn TXIM(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Transmit FIFO interrupt mask: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
    #[inline(always)]
    pub const fn set_TXIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for IMSC {
    #[inline(always)]
    fn default() -> IMSC {
        IMSC(0)
    }
}
impl core::fmt::Debug for IMSC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMSC")
            .field("RORIM", &self.RORIM())
            .field("RTIM", &self.RTIM())
            .field("RXIM", &self.RXIM())
            .field("TXIM", &self.TXIM())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IMSC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IMSC {{ RORIM: {=bool:?}, RTIM: {=bool:?}, RXIM: {=bool:?}, TXIM: {=bool:?}, RESERVED: {=u32:?} }}",
            self.RORIM(),
            self.RTIM(),
            self.RXIM(),
            self.TXIM(),
            self.RESERVED()
        )
    }
}
#[doc = "Masked Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MIS(pub u32);
impl MIS {
    #[doc = "0:0\\] Masked interrupt state of receive overrun interrupt: This field returns the masked interrupt state of receive overrun interrupt which is the AND product of raw interrupt state RIS.RORRIS and the mask setting IMSC.RORIM."]
    #[must_use]
    #[inline(always)]
    pub const fn RORMIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Masked interrupt state of receive overrun interrupt: This field returns the masked interrupt state of receive overrun interrupt which is the AND product of raw interrupt state RIS.RORRIS and the mask setting IMSC.RORIM."]
    #[inline(always)]
    pub const fn set_RORMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Masked interrupt state of receive timeout interrupt: This field returns the masked interrupt state of receive timeout interrupt which is the AND product of raw interrupt state RIS.RTRIS and the mask setting IMSC.RTIM."]
    #[must_use]
    #[inline(always)]
    pub const fn RTMIS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Masked interrupt state of receive timeout interrupt: This field returns the masked interrupt state of receive timeout interrupt which is the AND product of raw interrupt state RIS.RTRIS and the mask setting IMSC.RTIM."]
    #[inline(always)]
    pub const fn set_RTMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Masked interrupt state of receive FIFO interrupt: This field returns the masked interrupt state of receive FIFO interrupt which is the AND product of raw interrupt state RIS.RXRIS and the mask setting IMSC.RXIM."]
    #[must_use]
    #[inline(always)]
    pub const fn RXMIS(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Masked interrupt state of receive FIFO interrupt: This field returns the masked interrupt state of receive FIFO interrupt which is the AND product of raw interrupt state RIS.RXRIS and the mask setting IMSC.RXIM."]
    #[inline(always)]
    pub const fn set_RXMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Masked interrupt state of transmit FIFO interrupt: This field returns the masked interrupt state of transmit FIFO interrupt which is the AND product of raw interrupt state RIS.TXRIS and the mask setting IMSC.TXIM."]
    #[must_use]
    #[inline(always)]
    pub const fn TXMIS(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Masked interrupt state of transmit FIFO interrupt: This field returns the masked interrupt state of transmit FIFO interrupt which is the AND product of raw interrupt state RIS.TXRIS and the mask setting IMSC.TXIM."]
    #[inline(always)]
    pub const fn set_TXMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for MIS {
    #[inline(always)]
    fn default() -> MIS {
        MIS(0)
    }
}
impl core::fmt::Debug for MIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIS")
            .field("RORMIS", &self.RORMIS())
            .field("RTMIS", &self.RTMIS())
            .field("RXMIS", &self.RXMIS())
            .field("TXMIS", &self.TXMIS())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MIS {{ RORMIS: {=bool:?}, RTMIS: {=bool:?}, RXMIS: {=bool:?}, TXMIS: {=bool:?}, RESERVED: {=u32:?} }}",
            self.RORMIS(),
            self.RTMIS(),
            self.RXMIS(),
            self.TXMIS(),
            self.RESERVED()
        )
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESERVED1(pub u32);
impl RESERVED1 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RESERVED1 {
    #[inline(always)]
    fn default() -> RESERVED1 {
        RESERVED1(0)
    }
}
impl core::fmt::Debug for RESERVED1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVED1")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESERVED1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RESERVED1 {{ RESERVED: {=u32:?} }}", self.RESERVED())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESERVED2(pub u32);
impl RESERVED2 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RESERVED2 {
    #[inline(always)]
    fn default() -> RESERVED2 {
        RESERVED2(0)
    }
}
impl core::fmt::Debug for RESERVED2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVED2")
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESERVED2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RESERVED2 {{ RESERVED: {=u32:?} }}", self.RESERVED())
    }
}
#[doc = "Raw Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RIS(pub u32);
impl RIS {
    #[doc = "0:0\\] Raw interrupt state of receive overrun interrupt: The receive overrun interrupt is asserted when the FIFO is already full and an additional data frame is received, causing an overrun of the FIFO. Data is over-written in the receive shift register, but not the FIFO so the FIFO contents stay valid. It can also be cleared by writing to ICR.RORIC."]
    #[must_use]
    #[inline(always)]
    pub const fn RORRIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Raw interrupt state of receive overrun interrupt: The receive overrun interrupt is asserted when the FIFO is already full and an additional data frame is received, causing an overrun of the FIFO. Data is over-written in the receive shift register, but not the FIFO so the FIFO contents stay valid. It can also be cleared by writing to ICR.RORIC."]
    #[inline(always)]
    pub const fn set_RORRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Raw interrupt state of receive timeout interrupt: The receive timeout interrupt is asserted when the receive FIFO is not empty and SSI has remained idle for a fixed 32 bit period. This mechanism can be used to notify the user that data is still present in the receive FIFO and requires servicing. This interrupt is deasserted if the receive FIFO becomes empty by subsequent reads, or if new data is received on RXD. It can also be cleared by writing to ICR.RTIC."]
    #[must_use]
    #[inline(always)]
    pub const fn RTRIS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Raw interrupt state of receive timeout interrupt: The receive timeout interrupt is asserted when the receive FIFO is not empty and SSI has remained idle for a fixed 32 bit period. This mechanism can be used to notify the user that data is still present in the receive FIFO and requires servicing. This interrupt is deasserted if the receive FIFO becomes empty by subsequent reads, or if new data is received on RXD. It can also be cleared by writing to ICR.RTIC."]
    #[inline(always)]
    pub const fn set_RTRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Raw interrupt state of receive FIFO interrupt: The receive interrupt is asserted when there are four or more valid entries in the receive FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn RXRIS(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Raw interrupt state of receive FIFO interrupt: The receive interrupt is asserted when there are four or more valid entries in the receive FIFO."]
    #[inline(always)]
    pub const fn set_RXRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Raw transmit FIFO interrupt status: The transmit interrupt is asserted when there are four or fewer valid entries in the transmit FIFO. The transmit interrupt is not qualified with the SSI enable signal. Therefore one of the following ways can be used: - data can be written to the transmit FIFO prior to enabling the SSI and the interrupts. - SSI and interrupts can be enabled so that data can be written to the transmit FIFO by an interrupt service routine."]
    #[must_use]
    #[inline(always)]
    pub const fn TXRIS(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Raw transmit FIFO interrupt status: The transmit interrupt is asserted when there are four or fewer valid entries in the transmit FIFO. The transmit interrupt is not qualified with the SSI enable signal. Therefore one of the following ways can be used: - data can be written to the transmit FIFO prior to enabling the SSI and the interrupts. - SSI and interrupts can be enabled so that data can be written to the transmit FIFO by an interrupt service routine."]
    #[inline(always)]
    pub const fn set_TXRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for RIS {
    #[inline(always)]
    fn default() -> RIS {
        RIS(0)
    }
}
impl core::fmt::Debug for RIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("RORRIS", &self.RORRIS())
            .field("RTRIS", &self.RTRIS())
            .field("RXRIS", &self.RXRIS())
            .field("TXRIS", &self.TXRIS())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RIS {{ RORRIS: {=bool:?}, RTRIS: {=bool:?}, RXRIS: {=bool:?}, TXRIS: {=bool:?}, RESERVED: {=u32:?} }}",
            self.RORRIS(),
            self.RTRIS(),
            self.RXRIS(),
            self.TXRIS(),
            self.RESERVED()
        )
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SR(pub u32);
impl SR {
    #[doc = "0:0\\] Transmit FIFO empty: 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
    #[must_use]
    #[inline(always)]
    pub const fn TFE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Transmit FIFO empty: 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
    #[inline(always)]
    pub const fn set_TFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Transmit FIFO not full: 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
    #[must_use]
    #[inline(always)]
    pub const fn TNF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Transmit FIFO not full: 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
    #[inline(always)]
    pub const fn set_TNF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Receive FIFO not empty 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
    #[must_use]
    #[inline(always)]
    pub const fn RNE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Receive FIFO not empty 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
    #[inline(always)]
    pub const fn set_RNE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Receive FIFO full: 0: Receive FIFO is not full. 1: Receive FIFO is full."]
    #[must_use]
    #[inline(always)]
    pub const fn RFF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Receive FIFO full: 0: Receive FIFO is not full. 1: Receive FIFO is full."]
    #[inline(always)]
    pub const fn set_RFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Serial interface busy: 0: SSI is idle 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[must_use]
    #[inline(always)]
    pub const fn BSY(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Serial interface busy: 0: SSI is idle 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    pub const fn set_BSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for SR {
    #[inline(always)]
    fn default() -> SR {
        SR(0)
    }
}
impl core::fmt::Debug for SR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("TFE", &self.TFE())
            .field("TNF", &self.TNF())
            .field("RNE", &self.RNE())
            .field("RFF", &self.RFF())
            .field("BSY", &self.BSY())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SR {{ TFE: {=bool:?}, TNF: {=bool:?}, RNE: {=bool:?}, RFF: {=bool:?}, BSY: {=bool:?}, RESERVED: {=u32:?} }}",
            self.TFE(),
            self.TNF(),
            self.RNE(),
            self.RFF(),
            self.BSY(),
            self.RESERVED()
        )
    }
}

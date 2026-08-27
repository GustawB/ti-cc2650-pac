#[doc = "Master Configuration This register configures the mode (Master or Slave) and sets the interface for test mode loopback."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MCR(pub u32);
impl MCR {
    #[doc = "0:0\\] I2C loopback 0: Normal operation 1: Loopback operation (test mode)."]
    #[must_use]
    #[inline(always)]
    pub const fn LPBK(&self) -> super::vals::LPBK {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LPBK::from_bits(val as u8)
    }
    #[doc = "0:0\\] I2C loopback 0: Normal operation 1: Loopback operation (test mode)."]
    #[inline(always)]
    pub const fn set_LPBK(&mut self, val: super::vals::LPBK) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "3:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "3:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "4:4\\] I2C master function enable."]
    #[must_use]
    #[inline(always)]
    pub const fn MFE(&self) -> super::vals::MFE {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::MFE::from_bits(val as u8)
    }
    #[doc = "4:4\\] I2C master function enable."]
    #[inline(always)]
    pub const fn set_MFE(&mut self, val: super::vals::MFE) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] I2C slave function enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SFE(&self) -> super::vals::SFE {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SFE::from_bits(val as u8)
    }
    #[doc = "5:5\\] I2C slave function enable."]
    #[inline(always)]
    pub const fn set_SFE(&mut self, val: super::vals::SFE) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for MCR {
    #[inline(always)]
    fn default() -> MCR {
        MCR(0)
    }
}
impl core::fmt::Debug for MCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCR")
            .field("LPBK", &self.LPBK())
            .field("RESERVED1", &self.RESERVED1())
            .field("MFE", &self.MFE())
            .field("SFE", &self.SFE())
            .field("RESERVED6", &self.RESERVED6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MCR {{ LPBK: {:?}, RESERVED1: {=u8:?}, MFE: {:?}, SFE: {:?}, RESERVED6: {=u32:?} }}",
            self.LPBK(),
            self.RESERVED1(),
            self.MFE(),
            self.SFE(),
            self.RESERVED6()
        )
    }
}
#[doc = "Master Control This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller as stated in MSTAT. When written, the control register configures the I2C controller operation. To generate a single transmit cycle, the I2C Master Slave Address (MSA) register is written with the desired address, the MSA.RS bit is cleared, and this register is written with * ACK=X (0 or 1), * STOP=1, * START=1, * RUN=1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the MDR register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MCTRL(pub u32);
impl MCTRL {
    #[doc = "0:0\\] I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data."]
    #[must_use]
    #[inline(always)]
    pub const fn RUN(&self) -> super::vals::RUN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RUN::from_bits(val as u8)
    }
    #[doc = "0:0\\] I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data."]
    #[inline(always)]
    pub const fn set_RUN(&mut self, val: super::vals::RUN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] This bit-field generates the Start or Repeated Start condition. 0: The controller does not generate the Start condition. 1: The controller generates the Start condition."]
    #[must_use]
    #[inline(always)]
    pub const fn START(&self) -> super::vals::START {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::START::from_bits(val as u8)
    }
    #[doc = "1:1\\] This bit-field generates the Start or Repeated Start condition. 0: The controller does not generate the Start condition. 1: The controller generates the Start condition."]
    #[inline(always)]
    pub const fn set_START(&mut self, val: super::vals::START) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0: The controller does not generate the Stop condition. 1: The controller generates the Stop condition."]
    #[must_use]
    #[inline(always)]
    pub const fn STOP(&self) -> super::vals::STOP {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::STOP::from_bits(val as u8)
    }
    #[doc = "2:2\\] This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0: The controller does not generate the Stop condition. 1: The controller generates the Stop condition."]
    #[inline(always)]
    pub const fn set_STOP(&mut self, val: super::vals::STOP) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master. This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
    #[must_use]
    #[inline(always)]
    pub const fn ACK(&self) -> super::vals::ACK {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ACK::from_bits(val as u8)
    }
    #[doc = "3:3\\] Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master. This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
    #[inline(always)]
    pub const fn set_ACK(&mut self, val: super::vals::ACK) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for MCTRL {
    #[inline(always)]
    fn default() -> MCTRL {
        MCTRL(0)
    }
}
impl core::fmt::Debug for MCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCTRL")
            .field("RUN", &self.RUN())
            .field("START", &self.START())
            .field("STOP", &self.STOP())
            .field("ACK", &self.ACK())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MCTRL {{ RUN: {:?}, START: {:?}, STOP: {:?}, ACK: {:?}, RESERVED4: {=u32:?} }}",
            self.RUN(),
            self.START(),
            self.STOP(),
            self.ACK(),
            self.RESERVED4()
        )
    }
}
#[doc = "Master Data This register contains the data to be transmitted when in the Master Transmit state and the data received when in the Master Receive state."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MDR(pub u32);
impl MDR {
    #[doc = "7:0\\] When Read: Last RX Data is returned When Written: Data is transferred during TX transaction."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] When Read: Last RX Data is returned When Written: Data is transferred during TX transaction."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for MDR {
    #[inline(always)]
    fn default() -> MDR {
        MDR(0)
    }
}
impl core::fmt::Debug for MDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDR")
            .field("DATA", &self.DATA())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MDR {{ DATA: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.DATA(),
            self.RESERVED8()
        )
    }
}
#[doc = "Master Interrupt Clear This register clears the raw and masked interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MICR(pub u32);
impl MICR {
    #[doc = "0:0\\] Interrupt clear Writing 1 to this bit clears MRIS.RIS and MMIS.MIS . Reading this register returns no meaningful data."]
    #[must_use]
    #[inline(always)]
    pub const fn IC(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Interrupt clear Writing 1 to this bit clears MRIS.RIS and MMIS.MIS . Reading this register returns no meaningful data."]
    #[inline(always)]
    pub const fn set_IC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for MICR {
    #[inline(always)]
    fn default() -> MICR {
        MICR(0)
    }
}
impl core::fmt::Debug for MICR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MICR")
            .field("IC", &self.IC())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MICR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MICR {{ IC: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.IC(),
            self.RESERVED1()
        )
    }
}
#[doc = "Master Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MIMR(pub u32);
impl MIMR {
    #[doc = "0:0\\] Interrupt mask 0: The MRIS.RIS interrupt is suppressed and not sent to the interrupt controller. 1: The master interrupt is sent to the interrupt controller when the MRIS.RIS is set."]
    #[must_use]
    #[inline(always)]
    pub const fn IM(&self) -> super::vals::IM {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::IM::from_bits(val as u8)
    }
    #[doc = "0:0\\] Interrupt mask 0: The MRIS.RIS interrupt is suppressed and not sent to the interrupt controller. 1: The master interrupt is sent to the interrupt controller when the MRIS.RIS is set."]
    #[inline(always)]
    pub const fn set_IM(&mut self, val: super::vals::IM) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for MIMR {
    #[inline(always)]
    fn default() -> MIMR {
        MIMR(0)
    }
}
impl core::fmt::Debug for MIMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIMR")
            .field("IM", &self.IM())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MIMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MIMR {{ IM: {:?}, RESERVED1: {=u32:?} }}",
            self.IM(),
            self.RESERVED1()
        )
    }
}
#[doc = "Master Masked Interrupt Status This register show which interrupt is active (based on result from MRIS and MIMR)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MMIS(pub u32);
impl MMIS {
    #[doc = "0:0\\] Masked interrupt status 0: An interrupt has not occurred or is masked. 1: A master interrupt is pending. This bit is cleared by writing 1 to the MICR.IC bit."]
    #[must_use]
    #[inline(always)]
    pub const fn MIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Masked interrupt status 0: An interrupt has not occurred or is masked. 1: A master interrupt is pending. This bit is cleared by writing 1 to the MICR.IC bit."]
    #[inline(always)]
    pub const fn set_MIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for MMIS {
    #[inline(always)]
    fn default() -> MMIS {
        MMIS(0)
    }
}
impl core::fmt::Debug for MMIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMIS")
            .field("MIS", &self.MIS())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MMIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MMIS {{ MIS: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.MIS(),
            self.RESERVED1()
        )
    }
}
#[doc = "Master Raw Interrupt Status This register show the unmasked interrupt status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MRIS(pub u32);
impl MRIS {
    #[doc = "0:0\\] Raw interrupt status 0: No interrupt 1: A master interrupt is pending. This bit is cleared by writing 1 to the MICR.IC bit."]
    #[must_use]
    #[inline(always)]
    pub const fn RIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Raw interrupt status 0: No interrupt 1: A master interrupt is pending. This bit is cleared by writing 1 to the MICR.IC bit."]
    #[inline(always)]
    pub const fn set_RIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for MRIS {
    #[inline(always)]
    fn default() -> MRIS {
        MRIS(0)
    }
}
impl core::fmt::Debug for MRIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MRIS")
            .field("RIS", &self.RIS())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MRIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MRIS {{ RIS: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.RIS(),
            self.RESERVED1()
        )
    }
}
#[doc = "Master Salve Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MSA(pub u32);
impl MSA {
    #[doc = "0:0\\] Receive or Send This bit-field specifies if the next operation is a receive (high) or a transmit/send (low) from the addressed slave SA."]
    #[must_use]
    #[inline(always)]
    pub const fn RS(&self) -> super::vals::RS {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RS::from_bits(val as u8)
    }
    #[doc = "0:0\\] Receive or Send This bit-field specifies if the next operation is a receive (high) or a transmit/send (low) from the addressed slave SA."]
    #[inline(always)]
    pub const fn set_RS(&mut self, val: super::vals::RS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "7:1\\] I2C master slave address Defines which slave is addressed for the transaction in master mode."]
    #[must_use]
    #[inline(always)]
    pub const fn SA(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "7:1\\] I2C master slave address Defines which slave is addressed for the transaction in master mode."]
    #[inline(always)]
    pub const fn set_SA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for MSA {
    #[inline(always)]
    fn default() -> MSA {
        MSA(0)
    }
}
impl core::fmt::Debug for MSA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSA")
            .field("RS", &self.RS())
            .field("SA", &self.SA())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MSA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MSA {{ RS: {:?}, SA: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.RS(),
            self.SA(),
            self.RESERVED8()
        )
    }
}
#[doc = "Master Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MSTAT(pub u32);
impl MSTAT {
    #[doc = "0:0\\] I2C busy 0: The controller is idle. 1: The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The I2C controller requires four SYSBUS clock cycles to assert the BUSY status after I2C master operation has been initiated through MCTRL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through MSTAT register. Any prior inquiry would result in wrong status being reported."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] I2C busy 0: The controller is idle. 1: The controller is busy. When this bit-field is set, the other status bits are not valid. Note: The I2C controller requires four SYSBUS clock cycles to assert the BUSY status after I2C master operation has been initiated through MCTRL register. Hence after programming MCTRL register, application is requested to wait for four SYSBUS clock cycles before issuing a controller status inquiry through MSTAT register. Any prior inquiry would result in wrong status being reported."]
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Error 0: No error was detected on the last operation. 1: An error occurred on the last operation."]
    #[inline(always)]
    pub const fn set_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Address Was Not Acknowledge 0: The transmitted address was acknowledged. 1: The transmitted address was not acknowledged."]
    #[must_use]
    #[inline(always)]
    pub const fn ADRACK_N(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Address Was Not Acknowledge 0: The transmitted address was acknowledged. 1: The transmitted address was not acknowledged."]
    #[inline(always)]
    pub const fn set_ADRACK_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Data Was Not Acknowledge 0: The transmitted data was acknowledged. 1: The transmitted data was not acknowledged."]
    #[must_use]
    #[inline(always)]
    pub const fn DATACK_N(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Data Was Not Acknowledge 0: The transmitted data was acknowledged. 1: The transmitted data was not acknowledged."]
    #[inline(always)]
    pub const fn set_DATACK_N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
    #[must_use]
    #[inline(always)]
    pub const fn ARBLST(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Arbitration lost 0: The I2C controller won arbitration. 1: The I2C controller lost arbitration."]
    #[inline(always)]
    pub const fn set_ARBLST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
    #[must_use]
    #[inline(always)]
    pub const fn IDLE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] I2C idle 0: The I2C controller is not idle. 1: The I2C controller is idle."]
    #[inline(always)]
    pub const fn set_IDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the MCTRL.START and MCTRL.STOP conditions."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSBSY(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Bus busy 0: The I2C bus is idle. 1: The I2C bus is busy. The bit changes based on the MCTRL.START and MCTRL.STOP conditions."]
    #[inline(always)]
    pub const fn set_BUSBSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED7(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for MSTAT {
    #[inline(always)]
    fn default() -> MSTAT {
        MSTAT(0)
    }
}
impl core::fmt::Debug for MSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTAT")
            .field("BUSY", &self.BUSY())
            .field("ERR", &self.ERR())
            .field("ADRACK_N", &self.ADRACK_N())
            .field("DATACK_N", &self.DATACK_N())
            .field("ARBLST", &self.ARBLST())
            .field("IDLE", &self.IDLE())
            .field("BUSBSY", &self.BUSBSY())
            .field("RESERVED7", &self.RESERVED7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MSTAT {{ BUSY: {=bool:?}, ERR: {=bool:?}, ADRACK_N: {=bool:?}, DATACK_N: {=bool:?}, ARBLST: {=bool:?}, IDLE: {=bool:?}, BUSBSY: {=bool:?}, RESERVED7: {=u32:?} }}",
            self.BUSY(),
            self.ERR(),
            self.ADRACK_N(),
            self.DATACK_N(),
            self.ARBLST(),
            self.IDLE(),
            self.BUSBSY(),
            self.RESERVED7()
        )
    }
}
#[doc = "I2C Master Timer Period This register specifies the period of the SCL clock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MTPR(pub u32);
impl MTPR {
    #[doc = "6:0\\] SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2*(1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
    #[must_use]
    #[inline(always)]
    pub const fn TPR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "6:0\\] SCL clock period This field specifies the period of the SCL clock. SCL_PRD = 2*(1+TPR)*(SCL_LP + SCL_HP)*CLK_PRD where: SCL_PRD is the SCL line period (I2C clock). TPR is the timer period register value (range of 1 to 127) SCL_LP is the SCL low period (fixed at 6). SCL_HP is the SCL high period (fixed at 4). CLK_PRD is the system clock period in ns."]
    #[inline(always)]
    pub const fn set_TPR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "7:7\\] Must be set to 0 to set TPR. If set to 1, a write to TPR will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn TPR_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Must be set to 0 to set TPR. If set to 1, a write to TPR will be ignored."]
    #[inline(always)]
    pub const fn set_TPR_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for MTPR {
    #[inline(always)]
    fn default() -> MTPR {
        MTPR(0)
    }
}
impl core::fmt::Debug for MTPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTPR")
            .field("TPR", &self.TPR())
            .field("TPR_7", &self.TPR_7())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MTPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MTPR {{ TPR: {=u8:?}, TPR_7: {=bool:?}, RESERVED8: {=u32:?} }}",
            self.TPR(),
            self.TPR_7(),
            self.RESERVED8()
        )
    }
}
#[doc = "Slave Control Note: This register shares address with SSTAT, meaning that this register functions as a control register when written, and a status register when read."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCTL(pub u32);
impl SCTL {
    #[doc = "0:0\\] Device active 0: Disables the I2C slave operation 1: Enables the I2C slave operation."]
    #[must_use]
    #[inline(always)]
    pub const fn DA(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Device active 0: Disables the I2C slave operation 1: Enables the I2C slave operation."]
    #[inline(always)]
    pub const fn set_DA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for SCTL {
    #[inline(always)]
    fn default() -> SCTL {
        SCTL(0)
    }
}
impl core::fmt::Debug for SCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCTL")
            .field("DA", &self.DA())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SCTL {{ DA: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.DA(),
            self.RESERVED1()
        )
    }
}
#[doc = "Slave Data This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SDR(pub u32);
impl SDR {
    #[doc = "7:0\\] Data for transfer This field contains the data for transfer during a slave receive or transmit operation. When written the register data is used as transmit data. When read, this register returns the last data received. Data is stored until next update, either by a system write for transmit or by an external master for receive."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Data for transfer This field contains the data for transfer during a slave receive or transmit operation. When written the register data is used as transmit data. When read, this register returns the last data received. Data is stored until next update, either by a system write for transmit or by an external master for receive."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for SDR {
    #[inline(always)]
    fn default() -> SDR {
        SDR(0)
    }
}
impl core::fmt::Debug for SDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDR")
            .field("DATA", &self.DATA())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SDR {{ DATA: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.DATA(),
            self.RESERVED8()
        )
    }
}
#[doc = "Slave Interrupt Clear This register clears the raw interrupt SRIS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SICR(pub u32);
impl SICR {
    #[doc = "0:0\\] Data interrupt clear Writing 1 to this bit clears SRIS.DATARIS SMIS.DATAMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn DATAIC(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Data interrupt clear Writing 1 to this bit clears SRIS.DATARIS SMIS.DATAMIS."]
    #[inline(always)]
    pub const fn set_DATAIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Start condition interrupt clear Writing 1 to this bit clears SRIS.STARTRIS SMIS.STARTMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn STARTIC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Start condition interrupt clear Writing 1 to this bit clears SRIS.STARTRIS SMIS.STARTMIS."]
    #[inline(always)]
    pub const fn set_STARTIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Stop condition interrupt clear Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn STOPIC(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Stop condition interrupt clear Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
    #[inline(always)]
    pub const fn set_STOPIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for SICR {
    #[inline(always)]
    fn default() -> SICR {
        SICR(0)
    }
}
impl core::fmt::Debug for SICR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SICR")
            .field("DATAIC", &self.DATAIC())
            .field("STARTIC", &self.STARTIC())
            .field("STOPIC", &self.STOPIC())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SICR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SICR {{ DATAIC: {=bool:?}, STARTIC: {=bool:?}, STOPIC: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.DATAIC(),
            self.STARTIC(),
            self.STOPIC(),
            self.RESERVED3()
        )
    }
}
#[doc = "Slave Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SIMR(pub u32);
impl SIMR {
    #[doc = "0:0\\] Data interrupt mask 0: The SRIS.DATARIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.DATARIS interrupt is enabled and sent to the interrupt controller."]
    #[must_use]
    #[inline(always)]
    pub const fn DATAIM(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Data interrupt mask 0: The SRIS.DATARIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.DATARIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    pub const fn set_DATAIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller."]
    #[must_use]
    #[inline(always)]
    pub const fn STARTIM(&self) -> super::vals::STARTIM {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::STARTIM::from_bits(val as u8)
    }
    #[doc = "1:1\\] Start condition interrupt mask 0: The SRIS.STARTRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STARTRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    pub const fn set_STARTIM(&mut self, val: super::vals::STARTIM) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller."]
    #[must_use]
    #[inline(always)]
    pub const fn STOPIM(&self) -> super::vals::STOPIM {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::STOPIM::from_bits(val as u8)
    }
    #[doc = "2:2\\] Stop condition interrupt mask 0: The SRIS.STOPRIS interrupt is suppressed and not sent to the interrupt controller. 1: The SRIS.STOPRIS interrupt is enabled and sent to the interrupt controller."]
    #[inline(always)]
    pub const fn set_STOPIM(&mut self, val: super::vals::STOPIM) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for SIMR {
    #[inline(always)]
    fn default() -> SIMR {
        SIMR(0)
    }
}
impl core::fmt::Debug for SIMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIMR")
            .field("DATAIM", &self.DATAIM())
            .field("STARTIM", &self.STARTIM())
            .field("STOPIM", &self.STOPIM())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SIMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SIMR {{ DATAIM: {=bool:?}, STARTIM: {:?}, STOPIM: {:?}, RESERVED3: {=u32:?} }}",
            self.DATAIM(),
            self.STARTIM(),
            self.STOPIM(),
            self.RESERVED3()
        )
    }
}
#[doc = "Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SMIS(pub u32);
impl SMIS {
    #[doc = "0:0\\] Data masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
    #[must_use]
    #[inline(always)]
    pub const fn DATAMIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Data masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
    #[inline(always)]
    pub const fn set_DATAMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Start condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Start condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STARTIC."]
    #[must_use]
    #[inline(always)]
    pub const fn STARTMIS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Start condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Start condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STARTIC."]
    #[inline(always)]
    pub const fn set_STARTMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Stop condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Stop condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STOPIC."]
    #[must_use]
    #[inline(always)]
    pub const fn STOPMIS(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Stop condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Stop condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STOPIC."]
    #[inline(always)]
    pub const fn set_STOPMIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for SMIS {
    #[inline(always)]
    fn default() -> SMIS {
        SMIS(0)
    }
}
impl core::fmt::Debug for SMIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMIS")
            .field("DATAMIS", &self.DATAMIS())
            .field("STARTMIS", &self.STARTMIS())
            .field("STOPMIS", &self.STOPMIS())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SMIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SMIS {{ DATAMIS: {=bool:?}, STARTMIS: {=bool:?}, STOPMIS: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.DATAMIS(),
            self.STARTMIS(),
            self.STOPMIS(),
            self.RESERVED3()
        )
    }
}
#[doc = "Slave Own Address This register consists of seven address bits that identify this I2C device on the I2C bus."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SOAR(pub u32);
impl SOAR {
    #[doc = "6:0\\] I2C slave own address This field specifies bits a6 through a0 of the slave address."]
    #[must_use]
    #[inline(always)]
    pub const fn OAR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "6:0\\] I2C slave own address This field specifies bits a6 through a0 of the slave address."]
    #[inline(always)]
    pub const fn set_OAR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED7(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "31:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for SOAR {
    #[inline(always)]
    fn default() -> SOAR {
        SOAR(0)
    }
}
impl core::fmt::Debug for SOAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOAR")
            .field("OAR", &self.OAR())
            .field("RESERVED7", &self.RESERVED7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SOAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SOAR {{ OAR: {=u8:?}, RESERVED7: {=u32:?} }}",
            self.OAR(),
            self.RESERVED7()
        )
    }
}
#[doc = "Slave Raw Interrupt Status This register shows the unmasked interrupt status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SRIS(pub u32);
impl SRIS {
    #[doc = "0:0\\] Data raw interrupt status 0: No interrupt 1: A data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
    #[must_use]
    #[inline(always)]
    pub const fn DATARIS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Data raw interrupt status 0: No interrupt 1: A data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
    #[inline(always)]
    pub const fn set_DATARIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Start condition raw interrupt status 0: No interrupt 1: A Start condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STARTIC."]
    #[must_use]
    #[inline(always)]
    pub const fn STARTRIS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Start condition raw interrupt status 0: No interrupt 1: A Start condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STARTIC."]
    #[inline(always)]
    pub const fn set_STARTRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Stop condition raw interrupt status 0: No interrupt 1: A Stop condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STOPIC."]
    #[must_use]
    #[inline(always)]
    pub const fn STOPRIS(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Stop condition raw interrupt status 0: No interrupt 1: A Stop condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STOPIC."]
    #[inline(always)]
    pub const fn set_STOPRIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for SRIS {
    #[inline(always)]
    fn default() -> SRIS {
        SRIS(0)
    }
}
impl core::fmt::Debug for SRIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRIS")
            .field("DATARIS", &self.DATARIS())
            .field("STARTRIS", &self.STARTRIS())
            .field("STOPRIS", &self.STOPRIS())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SRIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SRIS {{ DATARIS: {=bool:?}, STARTRIS: {=bool:?}, STOPRIS: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.DATARIS(),
            self.STARTRIS(),
            self.STOPRIS(),
            self.RESERVED3()
        )
    }
}
#[doc = "Slave Status Note: This register shares address with SCTL, meaning that this register functions as a control register when written, and a status register when read."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SSTAT(pub u32);
impl SSTAT {
    #[doc = "0:0\\] Receive request 0: No outstanding receive data 1: The I2C controller has outstanding receive data from the I2C master and is using clock stretching to delay the master until data has been read from the SDR register."]
    #[must_use]
    #[inline(always)]
    pub const fn RREQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Receive request 0: No outstanding receive data 1: The I2C controller has outstanding receive data from the I2C master and is using clock stretching to delay the master until data has been read from the SDR register."]
    #[inline(always)]
    pub const fn set_RREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Transmit request 0: No outstanding transmit request. 1: The I2C controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the SDR register."]
    #[must_use]
    #[inline(always)]
    pub const fn TREQ(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Transmit request 0: No outstanding transmit request. 1: The I2C controller has been addressed as a slave transmitter and is using clock stretching to delay the master until data has been written to the SDR register."]
    #[inline(always)]
    pub const fn set_TREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] First byte received 0: The first byte has not been received. 1: The first byte following the slave's own address has been received. This bit is only valid when the RREQ bit is set and is automatically cleared when data has been read from the SDR register. Note: This bit is not used for slave transmit operations."]
    #[must_use]
    #[inline(always)]
    pub const fn FBR(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] First byte received 0: The first byte has not been received. 1: The first byte following the slave's own address has been received. This bit is only valid when the RREQ bit is set and is automatically cleared when data has been read from the SDR register. Note: This bit is not used for slave transmit operations."]
    #[inline(always)]
    pub const fn set_FBR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for SSTAT {
    #[inline(always)]
    fn default() -> SSTAT {
        SSTAT(0)
    }
}
impl core::fmt::Debug for SSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSTAT")
            .field("RREQ", &self.RREQ())
            .field("TREQ", &self.TREQ())
            .field("FBR", &self.FBR())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SSTAT {{ RREQ: {=bool:?}, TREQ: {=bool:?}, FBR: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.RREQ(),
            self.TREQ(),
            self.FBR(),
            self.RESERVED3()
        )
    }
}

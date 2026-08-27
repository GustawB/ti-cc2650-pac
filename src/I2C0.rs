#[doc = "I2CMaster/Slave Serial Controler."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C0 {
    ptr: *mut u8,
}
unsafe impl Send for I2C0 {}
unsafe impl Sync for I2C0 {}
impl I2C0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Slave Own Address This register consists of seven address bits that identify this I2C device on the I2C bus."]
    #[inline(always)]
    pub const fn SOAR(self) -> crate::common::Reg<regs::SOAR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Slave Control Note: This register shares address with SSTAT, meaning that this register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub const fn SCTL(self) -> crate::common::Reg<regs::SCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Slave Status Note: This register shares address with SCTL, meaning that this register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub const fn SSTAT(self) -> crate::common::Reg<regs::SSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Slave Data This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state."]
    #[inline(always)]
    pub const fn SDR(self) -> crate::common::Reg<regs::SDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Slave Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
    #[inline(always)]
    pub const fn SIMR(self) -> crate::common::Reg<regs::SIMR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Slave Raw Interrupt Status This register shows the unmasked interrupt status."]
    #[inline(always)]
    pub const fn SRIS(self) -> crate::common::Reg<regs::SRIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR)."]
    #[inline(always)]
    pub const fn SMIS(self) -> crate::common::Reg<regs::SMIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Slave Interrupt Clear This register clears the raw interrupt SRIS."]
    #[inline(always)]
    pub const fn SICR(self) -> crate::common::Reg<regs::SICR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Master Salve Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit."]
    #[inline(always)]
    pub const fn MSA(self) -> crate::common::Reg<regs::MSA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Master Control This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller as stated in MSTAT. When written, the control register configures the I2C controller operation. To generate a single transmit cycle, the I2C Master Slave Address (MSA) register is written with the desired address, the MSA.RS bit is cleared, and this register is written with * ACK=X (0 or 1), * STOP=1, * START=1, * RUN=1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the MDR register."]
    #[inline(always)]
    pub const fn MCTRL(self) -> crate::common::Reg<regs::MCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "Master Status."]
    #[inline(always)]
    pub const fn MSTAT(self) -> crate::common::Reg<regs::MSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "Master Data This register contains the data to be transmitted when in the Master Transmit state and the data received when in the Master Receive state."]
    #[inline(always)]
    pub const fn MDR(self) -> crate::common::Reg<regs::MDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0808usize) as _) }
    }
    #[doc = "I2C Master Timer Period This register specifies the period of the SCL clock."]
    #[inline(always)]
    pub const fn MTPR(self) -> crate::common::Reg<regs::MTPR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
    #[doc = "Master Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
    #[inline(always)]
    pub const fn MIMR(self) -> crate::common::Reg<regs::MIMR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0810usize) as _) }
    }
    #[doc = "Master Raw Interrupt Status This register show the unmasked interrupt status."]
    #[inline(always)]
    pub const fn MRIS(self) -> crate::common::Reg<regs::MRIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0814usize) as _) }
    }
    #[doc = "Master Masked Interrupt Status This register show which interrupt is active (based on result from MRIS and MIMR)."]
    #[inline(always)]
    pub const fn MMIS(self) -> crate::common::Reg<regs::MMIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0818usize) as _) }
    }
    #[doc = "Master Interrupt Clear This register clears the raw and masked interrupt."]
    #[inline(always)]
    pub const fn MICR(self) -> crate::common::Reg<regs::MICR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x081cusize) as _) }
    }
    #[doc = "Master Configuration This register configures the mode (Master or Slave) and sets the interface for test mode loopback."]
    #[inline(always)]
    pub const fn MCR(self) -> crate::common::Reg<regs::MCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0820usize) as _) }
    }
}
pub mod regs;
pub mod vals;

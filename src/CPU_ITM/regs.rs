#[doc = "Lock Access This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LAR(pub u32);
impl LAR {
    #[doc = "31:0\\] A privileged write of 0xC5ACCE55 enables more write access to Control Registers TER, TPR and TCR. An invalid write removes write access."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_ACCESS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A privileged write of 0xC5ACCE55 enables more write access to Control Registers TER, TPR and TCR. An invalid write removes write access."]
    #[inline(always)]
    pub const fn set_LOCK_ACCESS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LAR {
    #[inline(always)]
    fn default() -> LAR {
        LAR(0)
    }
}
impl core::fmt::Debug for LAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LAR")
            .field("LOCK_ACCESS", &self.LOCK_ACCESS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LAR {{ LOCK_ACCESS: {=u32:?} }}", self.LOCK_ACCESS())
    }
}
#[doc = "Lock Status Use this register to enable write accesses to the Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LSR(pub u32);
impl LSR {
    #[doc = "0:0\\] Indicates that a lock mechanism exists for this component."]
    #[must_use]
    #[inline(always)]
    pub const fn PRESENT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Indicates that a lock mechanism exists for this component."]
    #[inline(always)]
    pub const fn set_PRESENT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Write access to component is blocked. All writes are ignored, reads are permitted."]
    #[must_use]
    #[inline(always)]
    pub const fn ACCESS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Write access to component is blocked. All writes are ignored, reads are permitted."]
    #[inline(always)]
    pub const fn set_ACCESS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Reads 0 which means 8-bit lock access is not be implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn BYTEACC(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Reads 0 which means 8-bit lock access is not be implemented."]
    #[inline(always)]
    pub const fn set_BYTEACC(&mut self, val: bool) {
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
impl Default for LSR {
    #[inline(always)]
    fn default() -> LSR {
        LSR(0)
    }
}
impl core::fmt::Debug for LSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LSR")
            .field("PRESENT", &self.PRESENT())
            .field("ACCESS", &self.ACCESS())
            .field("BYTEACC", &self.BYTEACC())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LSR {{ PRESENT: {=bool:?}, ACCESS: {=bool:?}, BYTEACC: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.PRESENT(),
            self.ACCESS(),
            self.BYTEACC(),
            self.RESERVED3()
        )
    }
}
#[doc = "Stimulus Port 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM0(pub u32);
impl STIM0 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA0 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA0 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM0 {
    #[inline(always)]
    fn default() -> STIM0 {
        STIM0(0)
    }
}
impl core::fmt::Debug for STIM0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM0")
            .field("STIM0", &self.STIM0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM0 {{ STIM0: {=u32:?} }}", self.STIM0())
    }
}
#[doc = "Stimulus Port 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM1(pub u32);
impl STIM1 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA1 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA1 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM1 {
    #[inline(always)]
    fn default() -> STIM1 {
        STIM1(0)
    }
}
impl core::fmt::Debug for STIM1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM1")
            .field("STIM1", &self.STIM1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM1 {{ STIM1: {=u32:?} }}", self.STIM1())
    }
}
#[doc = "Stimulus Port 10."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM10(pub u32);
impl STIM10 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA10 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM10(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA10 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM10(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM10 {
    #[inline(always)]
    fn default() -> STIM10 {
        STIM10(0)
    }
}
impl core::fmt::Debug for STIM10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM10")
            .field("STIM10", &self.STIM10())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM10 {{ STIM10: {=u32:?} }}", self.STIM10())
    }
}
#[doc = "Stimulus Port 11."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM11(pub u32);
impl STIM11 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA11 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM11(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA11 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM11(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM11 {
    #[inline(always)]
    fn default() -> STIM11 {
        STIM11(0)
    }
}
impl core::fmt::Debug for STIM11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM11")
            .field("STIM11", &self.STIM11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM11 {{ STIM11: {=u32:?} }}", self.STIM11())
    }
}
#[doc = "Stimulus Port 12."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM12(pub u32);
impl STIM12 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA12 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM12(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA12 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM12(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM12 {
    #[inline(always)]
    fn default() -> STIM12 {
        STIM12(0)
    }
}
impl core::fmt::Debug for STIM12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM12")
            .field("STIM12", &self.STIM12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM12 {{ STIM12: {=u32:?} }}", self.STIM12())
    }
}
#[doc = "Stimulus Port 13."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM13(pub u32);
impl STIM13 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA13 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM13(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA13 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM13(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM13 {
    #[inline(always)]
    fn default() -> STIM13 {
        STIM13(0)
    }
}
impl core::fmt::Debug for STIM13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM13")
            .field("STIM13", &self.STIM13())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM13 {{ STIM13: {=u32:?} }}", self.STIM13())
    }
}
#[doc = "Stimulus Port 14."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM14(pub u32);
impl STIM14 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA14 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM14(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA14 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM14(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM14 {
    #[inline(always)]
    fn default() -> STIM14 {
        STIM14(0)
    }
}
impl core::fmt::Debug for STIM14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM14")
            .field("STIM14", &self.STIM14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM14 {{ STIM14: {=u32:?} }}", self.STIM14())
    }
}
#[doc = "Stimulus Port 15."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM15(pub u32);
impl STIM15 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA15 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM15(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA15 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM15(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM15 {
    #[inline(always)]
    fn default() -> STIM15 {
        STIM15(0)
    }
}
impl core::fmt::Debug for STIM15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM15")
            .field("STIM15", &self.STIM15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM15 {{ STIM15: {=u32:?} }}", self.STIM15())
    }
}
#[doc = "Stimulus Port 16."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM16(pub u32);
impl STIM16 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA16 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM16(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA16 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM16(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM16 {
    #[inline(always)]
    fn default() -> STIM16 {
        STIM16(0)
    }
}
impl core::fmt::Debug for STIM16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM16")
            .field("STIM16", &self.STIM16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM16 {{ STIM16: {=u32:?} }}", self.STIM16())
    }
}
#[doc = "Stimulus Port 17."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM17(pub u32);
impl STIM17 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA17 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM17(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA17 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM17(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM17 {
    #[inline(always)]
    fn default() -> STIM17 {
        STIM17(0)
    }
}
impl core::fmt::Debug for STIM17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM17")
            .field("STIM17", &self.STIM17())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM17 {{ STIM17: {=u32:?} }}", self.STIM17())
    }
}
#[doc = "Stimulus Port 18."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM18(pub u32);
impl STIM18 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA18 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM18(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA18 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM18(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM18 {
    #[inline(always)]
    fn default() -> STIM18 {
        STIM18(0)
    }
}
impl core::fmt::Debug for STIM18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM18")
            .field("STIM18", &self.STIM18())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM18 {{ STIM18: {=u32:?} }}", self.STIM18())
    }
}
#[doc = "Stimulus Port 19."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM19(pub u32);
impl STIM19 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA19 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM19(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA19 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM19(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM19 {
    #[inline(always)]
    fn default() -> STIM19 {
        STIM19(0)
    }
}
impl core::fmt::Debug for STIM19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM19")
            .field("STIM19", &self.STIM19())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM19 {{ STIM19: {=u32:?} }}", self.STIM19())
    }
}
#[doc = "Stimulus Port 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM2(pub u32);
impl STIM2 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA2 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA2 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM2 {
    #[inline(always)]
    fn default() -> STIM2 {
        STIM2(0)
    }
}
impl core::fmt::Debug for STIM2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM2")
            .field("STIM2", &self.STIM2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM2 {{ STIM2: {=u32:?} }}", self.STIM2())
    }
}
#[doc = "Stimulus Port 20."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM20(pub u32);
impl STIM20 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA20 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM20(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA20 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM20(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM20 {
    #[inline(always)]
    fn default() -> STIM20 {
        STIM20(0)
    }
}
impl core::fmt::Debug for STIM20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM20")
            .field("STIM20", &self.STIM20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM20 {{ STIM20: {=u32:?} }}", self.STIM20())
    }
}
#[doc = "Stimulus Port 21."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM21(pub u32);
impl STIM21 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA21 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM21(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA21 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM21(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM21 {
    #[inline(always)]
    fn default() -> STIM21 {
        STIM21(0)
    }
}
impl core::fmt::Debug for STIM21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM21")
            .field("STIM21", &self.STIM21())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM21 {{ STIM21: {=u32:?} }}", self.STIM21())
    }
}
#[doc = "Stimulus Port 22."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM22(pub u32);
impl STIM22 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA22 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM22(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA22 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM22(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM22 {
    #[inline(always)]
    fn default() -> STIM22 {
        STIM22(0)
    }
}
impl core::fmt::Debug for STIM22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM22")
            .field("STIM22", &self.STIM22())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM22 {{ STIM22: {=u32:?} }}", self.STIM22())
    }
}
#[doc = "Stimulus Port 23."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM23(pub u32);
impl STIM23 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA23 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM23(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA23 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM23(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM23 {
    #[inline(always)]
    fn default() -> STIM23 {
        STIM23(0)
    }
}
impl core::fmt::Debug for STIM23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM23")
            .field("STIM23", &self.STIM23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM23 {{ STIM23: {=u32:?} }}", self.STIM23())
    }
}
#[doc = "Stimulus Port 24."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM24(pub u32);
impl STIM24 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA24 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM24(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA24 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM24(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM24 {
    #[inline(always)]
    fn default() -> STIM24 {
        STIM24(0)
    }
}
impl core::fmt::Debug for STIM24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM24")
            .field("STIM24", &self.STIM24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM24 {{ STIM24: {=u32:?} }}", self.STIM24())
    }
}
#[doc = "Stimulus Port 25."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM25(pub u32);
impl STIM25 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA25 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM25(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA25 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM25(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM25 {
    #[inline(always)]
    fn default() -> STIM25 {
        STIM25(0)
    }
}
impl core::fmt::Debug for STIM25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM25")
            .field("STIM25", &self.STIM25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM25 {{ STIM25: {=u32:?} }}", self.STIM25())
    }
}
#[doc = "Stimulus Port 26."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM26(pub u32);
impl STIM26 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA26 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM26(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA26 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM26(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM26 {
    #[inline(always)]
    fn default() -> STIM26 {
        STIM26(0)
    }
}
impl core::fmt::Debug for STIM26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM26")
            .field("STIM26", &self.STIM26())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM26 {{ STIM26: {=u32:?} }}", self.STIM26())
    }
}
#[doc = "Stimulus Port 27."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM27(pub u32);
impl STIM27 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA27 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM27(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA27 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM27(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM27 {
    #[inline(always)]
    fn default() -> STIM27 {
        STIM27(0)
    }
}
impl core::fmt::Debug for STIM27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM27")
            .field("STIM27", &self.STIM27())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM27 {{ STIM27: {=u32:?} }}", self.STIM27())
    }
}
#[doc = "Stimulus Port 28."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM28(pub u32);
impl STIM28 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA28 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM28(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA28 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM28(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM28 {
    #[inline(always)]
    fn default() -> STIM28 {
        STIM28(0)
    }
}
impl core::fmt::Debug for STIM28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM28")
            .field("STIM28", &self.STIM28())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM28 {{ STIM28: {=u32:?} }}", self.STIM28())
    }
}
#[doc = "Stimulus Port 29."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM29(pub u32);
impl STIM29 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA29 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM29(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA29 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM29(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM29 {
    #[inline(always)]
    fn default() -> STIM29 {
        STIM29(0)
    }
}
impl core::fmt::Debug for STIM29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM29")
            .field("STIM29", &self.STIM29())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM29 {{ STIM29: {=u32:?} }}", self.STIM29())
    }
}
#[doc = "Stimulus Port 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM3(pub u32);
impl STIM3 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA3 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA3 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM3 {
    #[inline(always)]
    fn default() -> STIM3 {
        STIM3(0)
    }
}
impl core::fmt::Debug for STIM3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM3")
            .field("STIM3", &self.STIM3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM3 {{ STIM3: {=u32:?} }}", self.STIM3())
    }
}
#[doc = "Stimulus Port 30."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM30(pub u32);
impl STIM30 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA30 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM30(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA30 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM30(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM30 {
    #[inline(always)]
    fn default() -> STIM30 {
        STIM30(0)
    }
}
impl core::fmt::Debug for STIM30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM30")
            .field("STIM30", &self.STIM30())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM30 {{ STIM30: {=u32:?} }}", self.STIM30())
    }
}
#[doc = "Stimulus Port 31."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM31(pub u32);
impl STIM31 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA31 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM31(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA31 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM31(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM31 {
    #[inline(always)]
    fn default() -> STIM31 {
        STIM31(0)
    }
}
impl core::fmt::Debug for STIM31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM31")
            .field("STIM31", &self.STIM31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM31 {{ STIM31: {=u32:?} }}", self.STIM31())
    }
}
#[doc = "Stimulus Port 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM4(pub u32);
impl STIM4 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA4 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA4 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM4(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM4 {
    #[inline(always)]
    fn default() -> STIM4 {
        STIM4(0)
    }
}
impl core::fmt::Debug for STIM4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM4")
            .field("STIM4", &self.STIM4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM4 {{ STIM4: {=u32:?} }}", self.STIM4())
    }
}
#[doc = "Stimulus Port 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM5(pub u32);
impl STIM5 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA5 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA5 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM5(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM5 {
    #[inline(always)]
    fn default() -> STIM5 {
        STIM5(0)
    }
}
impl core::fmt::Debug for STIM5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM5")
            .field("STIM5", &self.STIM5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM5 {{ STIM5: {=u32:?} }}", self.STIM5())
    }
}
#[doc = "Stimulus Port 6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM6(pub u32);
impl STIM6 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA6 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA6 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM6(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM6 {
    #[inline(always)]
    fn default() -> STIM6 {
        STIM6(0)
    }
}
impl core::fmt::Debug for STIM6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM6")
            .field("STIM6", &self.STIM6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM6 {{ STIM6: {=u32:?} }}", self.STIM6())
    }
}
#[doc = "Stimulus Port 7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM7(pub u32);
impl STIM7 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA7 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA7 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM7(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM7 {
    #[inline(always)]
    fn default() -> STIM7 {
        STIM7(0)
    }
}
impl core::fmt::Debug for STIM7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM7")
            .field("STIM7", &self.STIM7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM7 {{ STIM7: {=u32:?} }}", self.STIM7())
    }
}
#[doc = "Stimulus Port 8."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM8(pub u32);
impl STIM8 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA8 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM8(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA8 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM8(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM8 {
    #[inline(always)]
    fn default() -> STIM8 {
        STIM8(0)
    }
}
impl core::fmt::Debug for STIM8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM8")
            .field("STIM8", &self.STIM8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM8 {{ STIM8: {=u32:?} }}", self.STIM8())
    }
}
#[doc = "Stimulus Port 9."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIM9(pub u32);
impl STIM9 {
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA9 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[must_use]
    #[inline(always)]
    pub const fn STIM9(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] A write to this location causes data to be written into the FIFO if TER.STIMENA9 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub const fn set_STIM9(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for STIM9 {
    #[inline(always)]
    fn default() -> STIM9 {
        STIM9(0)
    }
}
impl core::fmt::Debug for STIM9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIM9")
            .field("STIM9", &self.STIM9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIM9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "STIM9 {{ STIM9: {=u32:?} }}", self.STIM9())
    }
}
#[doc = "Trace Control Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TCR(pub u32);
impl TCR {
    #[doc = "0:0\\] Enables ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
    #[must_use]
    #[inline(always)]
    pub const fn ITMENA(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Enables ITM. This is the master enable, and must be set before ITM Stimulus and Trace Enable registers can be written."]
    #[inline(always)]
    pub const fn set_ITMENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of two million cycles. This provides a time reference for packets and inter-packet gaps. If SWOENA (bit \\[4\\]) is set, timestamps are triggered by activity on the internal trace bus only. In this case there is no regular timestamp output when the ITM is idle."]
    #[must_use]
    #[inline(always)]
    pub const fn TSENA(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Enables differential timestamps. Differential timestamps are emitted when a packet is written to the FIFO with a non-zero timestamp counter, and when the timestamp counter overflows. Timestamps are emitted during idle times after a fixed number of two million cycles. This provides a time reference for packets and inter-packet gaps. If SWOENA (bit \\[4\\]) is set, timestamps are triggered by activity on the internal trace bus only. In this case there is no regular timestamp output when the ITM is idle."]
    #[inline(always)]
    pub const fn set_TSENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Enables synchronization packet transmission for a synchronous TPIU. CPU_DWT:CTRL.SYNCTAP must be configured for the correct synchronization speed."]
    #[must_use]
    #[inline(always)]
    pub const fn SYNCENA(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Enables synchronization packet transmission for a synchronous TPIU. CPU_DWT:CTRL.SYNCTAP must be configured for the correct synchronization speed."]
    #[inline(always)]
    pub const fn set_SYNCENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Enables the DWT stimulus (hardware event packet emission to the TPIU from the DWT)."]
    #[must_use]
    #[inline(always)]
    pub const fn DWTENA(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Enables the DWT stimulus (hardware event packet emission to the TPIU from the DWT)."]
    #[inline(always)]
    pub const fn set_DWTENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Enables asynchronous clocking of the timestamp counter (when TSENA = 1). If TSENA = 0, writing this bit to 1 does not enable asynchronous clocking of the timestamp counter. 0x0: Mode disabled. Timestamp counter uses system clock from the core and counts continuously. 0x1: Timestamp counter uses lineout (data related) clock from TPIU interface. The timestamp counter is held in reset while the output line is idle."]
    #[must_use]
    #[inline(always)]
    pub const fn SWOENA(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Enables asynchronous clocking of the timestamp counter (when TSENA = 1). If TSENA = 0, writing this bit to 1 does not enable asynchronous clocking of the timestamp counter. 0x0: Mode disabled. Timestamp counter uses system clock from the core and counts continuously. 0x1: Timestamp counter uses lineout (data related) clock from TPIU interface. The timestamp counter is held in reset while the output line is idle."]
    #[inline(always)]
    pub const fn set_SWOENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "7:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "7:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "9:8\\] Timestamp prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn TSPRESCALE(&self) -> super::vals::TSPRESCALE {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::TSPRESCALE::from_bits(val as u8)
    }
    #[doc = "9:8\\] Timestamp prescaler."]
    #[inline(always)]
    pub const fn set_TSPRESCALE(&mut self, val: super::vals::TSPRESCALE) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "15:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED10(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "15:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[doc = "22:16\\] Trace Bus ID for CoreSight system. Optional identifier for multi-source trace stream formatting. If multi-source trace is in use, this field must be written with a non-zero value."]
    #[must_use]
    #[inline(always)]
    pub const fn ATBID(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "22:16\\] Trace Bus ID for CoreSight system. Optional identifier for multi-source trace stream formatting. If multi-source trace is in use, this field must be written with a non-zero value."]
    #[inline(always)]
    pub const fn set_ATBID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "23:23\\] Set when ITM events present and being drained."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Set when ITM events present and being drained."]
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED24(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED24(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for TCR {
    #[inline(always)]
    fn default() -> TCR {
        TCR(0)
    }
}
impl core::fmt::Debug for TCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCR")
            .field("ITMENA", &self.ITMENA())
            .field("TSENA", &self.TSENA())
            .field("SYNCENA", &self.SYNCENA())
            .field("DWTENA", &self.DWTENA())
            .field("SWOENA", &self.SWOENA())
            .field("RESERVED5", &self.RESERVED5())
            .field("TSPRESCALE", &self.TSPRESCALE())
            .field("RESERVED10", &self.RESERVED10())
            .field("ATBID", &self.ATBID())
            .field("BUSY", &self.BUSY())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TCR {{ ITMENA: {=bool:?}, TSENA: {=bool:?}, SYNCENA: {=bool:?}, DWTENA: {=bool:?}, SWOENA: {=bool:?}, RESERVED5: {=u8:?}, TSPRESCALE: {:?}, RESERVED10: {=u8:?}, ATBID: {=u8:?}, BUSY: {=bool:?}, RESERVED24: {=u8:?} }}",
            self.ITMENA(),
            self.TSENA(),
            self.SYNCENA(),
            self.DWTENA(),
            self.SWOENA(),
            self.RESERVED5(),
            self.TSPRESCALE(),
            self.RESERVED10(),
            self.ATBID(),
            self.BUSY(),
            self.RESERVED24()
        )
    }
}
#[doc = "Trace Enable Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TER(pub u32);
impl TER {
    #[doc = "0:0\\] Bit mask to enable tracing on ITM stimulus port 0."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Bit mask to enable tracing on ITM stimulus port 0."]
    #[inline(always)]
    pub const fn set_STIMENA0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Bit mask to enable tracing on ITM stimulus port 1."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Bit mask to enable tracing on ITM stimulus port 1."]
    #[inline(always)]
    pub const fn set_STIMENA1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Bit mask to enable tracing on ITM stimulus port 2."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Bit mask to enable tracing on ITM stimulus port 2."]
    #[inline(always)]
    pub const fn set_STIMENA2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Bit mask to enable tracing on ITM stimulus port 3."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Bit mask to enable tracing on ITM stimulus port 3."]
    #[inline(always)]
    pub const fn set_STIMENA3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Bit mask to enable tracing on ITM stimulus port 4."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Bit mask to enable tracing on ITM stimulus port 4."]
    #[inline(always)]
    pub const fn set_STIMENA4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Bit mask to enable tracing on ITM stimulus port 5."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Bit mask to enable tracing on ITM stimulus port 5."]
    #[inline(always)]
    pub const fn set_STIMENA5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Bit mask to enable tracing on ITM stimulus port 6."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Bit mask to enable tracing on ITM stimulus port 6."]
    #[inline(always)]
    pub const fn set_STIMENA6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Bit mask to enable tracing on ITM stimulus port 7."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Bit mask to enable tracing on ITM stimulus port 7."]
    #[inline(always)]
    pub const fn set_STIMENA7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Bit mask to enable tracing on ITM stimulus port 8."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Bit mask to enable tracing on ITM stimulus port 8."]
    #[inline(always)]
    pub const fn set_STIMENA8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Bit mask to enable tracing on ITM stimulus port 9."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Bit mask to enable tracing on ITM stimulus port 9."]
    #[inline(always)]
    pub const fn set_STIMENA9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Bit mask to enable tracing on ITM stimulus port 10."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Bit mask to enable tracing on ITM stimulus port 10."]
    #[inline(always)]
    pub const fn set_STIMENA10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Bit mask to enable tracing on ITM stimulus port 11."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Bit mask to enable tracing on ITM stimulus port 11."]
    #[inline(always)]
    pub const fn set_STIMENA11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Bit mask to enable tracing on ITM stimulus port 12."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Bit mask to enable tracing on ITM stimulus port 12."]
    #[inline(always)]
    pub const fn set_STIMENA12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Bit mask to enable tracing on ITM stimulus port 13."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Bit mask to enable tracing on ITM stimulus port 13."]
    #[inline(always)]
    pub const fn set_STIMENA13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Bit mask to enable tracing on ITM stimulus port 14."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Bit mask to enable tracing on ITM stimulus port 14."]
    #[inline(always)]
    pub const fn set_STIMENA14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Bit mask to enable tracing on ITM stimulus port 15."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Bit mask to enable tracing on ITM stimulus port 15."]
    #[inline(always)]
    pub const fn set_STIMENA15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Bit mask to enable tracing on ITM stimulus port 16."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Bit mask to enable tracing on ITM stimulus port 16."]
    #[inline(always)]
    pub const fn set_STIMENA16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Bit mask to enable tracing on ITM stimulus port 17."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Bit mask to enable tracing on ITM stimulus port 17."]
    #[inline(always)]
    pub const fn set_STIMENA17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Bit mask to enable tracing on ITM stimulus port 18."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Bit mask to enable tracing on ITM stimulus port 18."]
    #[inline(always)]
    pub const fn set_STIMENA18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Bit mask to enable tracing on ITM stimulus port 19."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Bit mask to enable tracing on ITM stimulus port 19."]
    #[inline(always)]
    pub const fn set_STIMENA19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Bit mask to enable tracing on ITM stimulus port 20."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Bit mask to enable tracing on ITM stimulus port 20."]
    #[inline(always)]
    pub const fn set_STIMENA20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Bit mask to enable tracing on ITM stimulus port 21."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Bit mask to enable tracing on ITM stimulus port 21."]
    #[inline(always)]
    pub const fn set_STIMENA21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Bit mask to enable tracing on ITM stimulus port 22."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Bit mask to enable tracing on ITM stimulus port 22."]
    #[inline(always)]
    pub const fn set_STIMENA22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Bit mask to enable tracing on ITM stimulus port 23."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Bit mask to enable tracing on ITM stimulus port 23."]
    #[inline(always)]
    pub const fn set_STIMENA23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Bit mask to enable tracing on ITM stimulus port 24."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Bit mask to enable tracing on ITM stimulus port 24."]
    #[inline(always)]
    pub const fn set_STIMENA24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Bit mask to enable tracing on ITM stimulus port 25."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Bit mask to enable tracing on ITM stimulus port 25."]
    #[inline(always)]
    pub const fn set_STIMENA25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Bit mask to enable tracing on ITM stimulus port 26."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Bit mask to enable tracing on ITM stimulus port 26."]
    #[inline(always)]
    pub const fn set_STIMENA26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Bit mask to enable tracing on ITM stimulus port 27."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Bit mask to enable tracing on ITM stimulus port 27."]
    #[inline(always)]
    pub const fn set_STIMENA27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Bit mask to enable tracing on ITM stimulus port 28."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Bit mask to enable tracing on ITM stimulus port 28."]
    #[inline(always)]
    pub const fn set_STIMENA28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Bit mask to enable tracing on ITM stimulus port 29."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Bit mask to enable tracing on ITM stimulus port 29."]
    #[inline(always)]
    pub const fn set_STIMENA29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Bit mask to enable tracing on ITM stimulus port 30."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Bit mask to enable tracing on ITM stimulus port 30."]
    #[inline(always)]
    pub const fn set_STIMENA30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Bit mask to enable tracing on ITM stimulus port 31."]
    #[must_use]
    #[inline(always)]
    pub const fn STIMENA31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Bit mask to enable tracing on ITM stimulus port 31."]
    #[inline(always)]
    pub const fn set_STIMENA31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TER {
    #[inline(always)]
    fn default() -> TER {
        TER(0)
    }
}
impl core::fmt::Debug for TER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TER")
            .field("STIMENA0", &self.STIMENA0())
            .field("STIMENA1", &self.STIMENA1())
            .field("STIMENA2", &self.STIMENA2())
            .field("STIMENA3", &self.STIMENA3())
            .field("STIMENA4", &self.STIMENA4())
            .field("STIMENA5", &self.STIMENA5())
            .field("STIMENA6", &self.STIMENA6())
            .field("STIMENA7", &self.STIMENA7())
            .field("STIMENA8", &self.STIMENA8())
            .field("STIMENA9", &self.STIMENA9())
            .field("STIMENA10", &self.STIMENA10())
            .field("STIMENA11", &self.STIMENA11())
            .field("STIMENA12", &self.STIMENA12())
            .field("STIMENA13", &self.STIMENA13())
            .field("STIMENA14", &self.STIMENA14())
            .field("STIMENA15", &self.STIMENA15())
            .field("STIMENA16", &self.STIMENA16())
            .field("STIMENA17", &self.STIMENA17())
            .field("STIMENA18", &self.STIMENA18())
            .field("STIMENA19", &self.STIMENA19())
            .field("STIMENA20", &self.STIMENA20())
            .field("STIMENA21", &self.STIMENA21())
            .field("STIMENA22", &self.STIMENA22())
            .field("STIMENA23", &self.STIMENA23())
            .field("STIMENA24", &self.STIMENA24())
            .field("STIMENA25", &self.STIMENA25())
            .field("STIMENA26", &self.STIMENA26())
            .field("STIMENA27", &self.STIMENA27())
            .field("STIMENA28", &self.STIMENA28())
            .field("STIMENA29", &self.STIMENA29())
            .field("STIMENA30", &self.STIMENA30())
            .field("STIMENA31", &self.STIMENA31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TER {{ STIMENA0: {=bool:?}, STIMENA1: {=bool:?}, STIMENA2: {=bool:?}, STIMENA3: {=bool:?}, STIMENA4: {=bool:?}, STIMENA5: {=bool:?}, STIMENA6: {=bool:?}, STIMENA7: {=bool:?}, STIMENA8: {=bool:?}, STIMENA9: {=bool:?}, STIMENA10: {=bool:?}, STIMENA11: {=bool:?}, STIMENA12: {=bool:?}, STIMENA13: {=bool:?}, STIMENA14: {=bool:?}, STIMENA15: {=bool:?}, STIMENA16: {=bool:?}, STIMENA17: {=bool:?}, STIMENA18: {=bool:?}, STIMENA19: {=bool:?}, STIMENA20: {=bool:?}, STIMENA21: {=bool:?}, STIMENA22: {=bool:?}, STIMENA23: {=bool:?}, STIMENA24: {=bool:?}, STIMENA25: {=bool:?}, STIMENA26: {=bool:?}, STIMENA27: {=bool:?}, STIMENA28: {=bool:?}, STIMENA29: {=bool:?}, STIMENA30: {=bool:?}, STIMENA31: {=bool:?} }}",
            self.STIMENA0(),
            self.STIMENA1(),
            self.STIMENA2(),
            self.STIMENA3(),
            self.STIMENA4(),
            self.STIMENA5(),
            self.STIMENA6(),
            self.STIMENA7(),
            self.STIMENA8(),
            self.STIMENA9(),
            self.STIMENA10(),
            self.STIMENA11(),
            self.STIMENA12(),
            self.STIMENA13(),
            self.STIMENA14(),
            self.STIMENA15(),
            self.STIMENA16(),
            self.STIMENA17(),
            self.STIMENA18(),
            self.STIMENA19(),
            self.STIMENA20(),
            self.STIMENA21(),
            self.STIMENA22(),
            self.STIMENA23(),
            self.STIMENA24(),
            self.STIMENA25(),
            self.STIMENA26(),
            self.STIMENA27(),
            self.STIMENA28(),
            self.STIMENA29(),
            self.STIMENA30(),
            self.STIMENA31()
        )
    }
}
#[doc = "Trace Privilege This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPR(pub u32);
impl TPR {
    #[doc = "3:0\\] Bit mask to enable unprivileged (User) access to ITM stimulus ports: Bit \\[0\\] enables stimulus ports 0, 1, ..., and 7. Bit \\[1\\] enables stimulus ports 8, 9, ..., and 15. Bit \\[2\\] enables stimulus ports 16, 17, ..., and 23. Bit \\[3\\] enables stimulus ports 24, 25, ..., and 31. 0: User access allowed to stimulus ports 1: Privileged access only to stimulus ports."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIVMASK(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Bit mask to enable unprivileged (User) access to ITM stimulus ports: Bit \\[0\\] enables stimulus ports 0, 1, ..., and 7. Bit \\[1\\] enables stimulus ports 8, 9, ..., and 15. Bit \\[2\\] enables stimulus ports 16, 17, ..., and 23. Bit \\[3\\] enables stimulus ports 24, 25, ..., and 31. 0: User access allowed to stimulus ports 1: Privileged access only to stimulus ports."]
    #[inline(always)]
    pub const fn set_PRIVMASK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
impl Default for TPR {
    #[inline(always)]
    fn default() -> TPR {
        TPR(0)
    }
}
impl core::fmt::Debug for TPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPR")
            .field("PRIVMASK", &self.PRIVMASK())
            .field("RESERVED4", &self.RESERVED4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TPR {{ PRIVMASK: {=u8:?}, RESERVED4: {=u32:?} }}",
            self.PRIVMASK(),
            self.RESERVED4()
        )
    }
}

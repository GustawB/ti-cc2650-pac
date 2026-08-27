#[doc = "Channel Alternate Control Data Base Pointer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ALTCTRL(pub u32);
impl ALTCTRL {
    #[doc = "31:0\\] This register shows the base address for the alternate data structures and is calculated by module, thus read only."]
    #[must_use]
    #[inline(always)]
    pub const fn BASEPTR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] This register shows the base address for the alternate data structures and is calculated by module, thus read only."]
    #[inline(always)]
    pub const fn set_BASEPTR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ALTCTRL {
    #[inline(always)]
    fn default() -> ALTCTRL {
        ALTCTRL(0)
    }
}
impl core::fmt::Debug for ALTCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALTCTRL")
            .field("BASEPTR", &self.BASEPTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ALTCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ALTCTRL {{ BASEPTR: {=u32:?} }}", self.BASEPTR())
    }
}
#[doc = "Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG(pub u32);
impl CFG {
    #[doc = "0:0\\] Enables the controller: 0: Disables the controller 1: Enables the controller."]
    #[must_use]
    #[inline(always)]
    pub const fn MASTERENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Enables the controller: 0: Disables the controller 1: Enables the controller."]
    #[inline(always)]
    pub const fn set_MASTERENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "4:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "4:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "7:5\\] Sets the AHB-Lite bus protocol protection state by controlling the AHB signal HProt\\[3:1\\] as follows: Bit \\[7\\] Controls HProt\\[3\\] to indicate if a cacheable access is occurring. Bit \\[6\\] Controls HProt\\[2\\] to indicate if a bufferable access is occurring. Bit \\[5\\] Controls HProt\\[1\\] to indicate if a privileged access is occurring. When bit \\[n\\] = 1 then the corresponding HProt bit is high. When bit \\[n\\] = 0 then the corresponding HProt bit is low. This field controls HProt\\[3:1\\] signal for all transactions initiated by uDMA except two transactions below: - the read from the address indicated by source address pointer - the write to the address indicated by destination address pointer HProt\\[3:1\\] for these two exceptions can be controlled by dedicated fields in the channel configutation descriptor."]
    #[must_use]
    #[inline(always)]
    pub const fn PRTOCTRL(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "7:5\\] Sets the AHB-Lite bus protocol protection state by controlling the AHB signal HProt\\[3:1\\] as follows: Bit \\[7\\] Controls HProt\\[3\\] to indicate if a cacheable access is occurring. Bit \\[6\\] Controls HProt\\[2\\] to indicate if a bufferable access is occurring. Bit \\[5\\] Controls HProt\\[1\\] to indicate if a privileged access is occurring. When bit \\[n\\] = 1 then the corresponding HProt bit is high. When bit \\[n\\] = 0 then the corresponding HProt bit is low. This field controls HProt\\[3:1\\] signal for all transactions initiated by uDMA except two transactions below: - the read from the address indicated by source address pointer - the write to the address indicated by destination address pointer HProt\\[3:1\\] for these two exceptions can be controlled by dedicated fields in the channel configutation descriptor."]
    #[inline(always)]
    pub const fn set_PRTOCTRL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
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
impl Default for CFG {
    #[inline(always)]
    fn default() -> CFG {
        CFG(0)
    }
}
impl core::fmt::Debug for CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("MASTERENABLE", &self.MASTERENABLE())
            .field("RESERVED1", &self.RESERVED1())
            .field("PRTOCTRL", &self.PRTOCTRL())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFG {{ MASTERENABLE: {=bool:?}, RESERVED1: {=u8:?}, PRTOCTRL: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.MASTERENABLE(),
            self.RESERVED1(),
            self.PRTOCTRL(),
            self.RESERVED8()
        )
    }
}
#[doc = "Channel Clear UseBurst."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLEARBURST(pub u32);
impl CLEARBURST {
    #[doc = "31:0\\] Set the appropriate bit to enable single transfer requests. Write as: Bit \\[Ch\\] = 0: No effect. Use the SETBURST.CHNLS to disable single transfer requests. Bit \\[Ch\\] = 1: Enables single transfer requests on channel Ch. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn CHNLS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Set the appropriate bit to enable single transfer requests. Write as: Bit \\[Ch\\] = 0: No effect. Use the SETBURST.CHNLS to disable single transfer requests. Bit \\[Ch\\] = 1: Enables single transfer requests on channel Ch. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub const fn set_CHNLS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CLEARBURST {
    #[inline(always)]
    fn default() -> CLEARBURST {
        CLEARBURST(0)
    }
}
impl core::fmt::Debug for CLEARBURST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLEARBURST")
            .field("CHNLS", &self.CHNLS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLEARBURST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLEARBURST {{ CHNLS: {=u32:?} }}", self.CHNLS())
    }
}
#[doc = "Clear Channel Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLEARCHANNELEN(pub u32);
impl CLEARCHANNELEN {
    #[doc = "31:0\\] Set the appropriate bit to disable the corresponding uDMA channel. Write as: Bit \\[Ch\\] = 0: No effect. Use the SETCHANNELEN.CHNLS to enable uDMA channels. Bit \\[Ch\\] = 1: Disables channel Ch Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn CHNLS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Set the appropriate bit to disable the corresponding uDMA channel. Write as: Bit \\[Ch\\] = 0: No effect. Use the SETCHANNELEN.CHNLS to enable uDMA channels. Bit \\[Ch\\] = 1: Disables channel Ch Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[inline(always)]
    pub const fn set_CHNLS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CLEARCHANNELEN {
    #[inline(always)]
    fn default() -> CLEARCHANNELEN {
        CLEARCHANNELEN(0)
    }
}
impl core::fmt::Debug for CLEARCHANNELEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLEARCHANNELEN")
            .field("CHNLS", &self.CHNLS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLEARCHANNELEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLEARCHANNELEN {{ CHNLS: {=u32:?} }}", self.CHNLS())
    }
}
#[doc = "Channel Clear Primary-Alternate."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLEARCHNLPRIALT(pub u32);
impl CLEARCHNLPRIALT {
    #[doc = "31:0\\] Clears the appropriate bit to select the primary data structure for the corresponding uDMA channel. Write as: Bit \\[Ch\\] = 0: No effect. Use the SETCHNLPRIALT.CHNLS to select the alternate data structure. Bit \\[Ch\\] = 1: Selects the primary data structure for channel Ch. Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn CHNLS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Clears the appropriate bit to select the primary data structure for the corresponding uDMA channel. Write as: Bit \\[Ch\\] = 0: No effect. Use the SETCHNLPRIALT.CHNLS to select the alternate data structure. Bit \\[Ch\\] = 1: Selects the primary data structure for channel Ch. Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[inline(always)]
    pub const fn set_CHNLS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CLEARCHNLPRIALT {
    #[inline(always)]
    fn default() -> CLEARCHNLPRIALT {
        CLEARCHNLPRIALT(0)
    }
}
impl core::fmt::Debug for CLEARCHNLPRIALT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLEARCHNLPRIALT")
            .field("CHNLS", &self.CHNLS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLEARCHNLPRIALT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLEARCHNLPRIALT {{ CHNLS: {=u32:?} }}", self.CHNLS())
    }
}
#[doc = "Clear Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLEARCHNLPRIORITY(pub u32);
impl CLEARCHNLPRIORITY {
    #[doc = "31:0\\] Clear the appropriate bit to select the default priority level for the specified uDMA channel. Write as: Bit \\[Ch\\] = 0: No effect. Use the SETCHNLPRIORITY.CHNLS to set channel Ch to the high priority level. Bit \\[Ch\\] = 1: Channel Ch uses the default priority level. Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn CHNLS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Clear the appropriate bit to select the default priority level for the specified uDMA channel. Write as: Bit \\[Ch\\] = 0: No effect. Use the SETCHNLPRIORITY.CHNLS to set channel Ch to the high priority level. Bit \\[Ch\\] = 1: Channel Ch uses the default priority level. Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[inline(always)]
    pub const fn set_CHNLS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CLEARCHNLPRIORITY {
    #[inline(always)]
    fn default() -> CLEARCHNLPRIORITY {
        CLEARCHNLPRIORITY(0)
    }
}
impl core::fmt::Debug for CLEARCHNLPRIORITY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLEARCHNLPRIORITY")
            .field("CHNLS", &self.CHNLS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLEARCHNLPRIORITY {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLEARCHNLPRIORITY {{ CHNLS: {=u32:?} }}", self.CHNLS())
    }
}
#[doc = "Clear Channel Request Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLEARREQMASK(pub u32);
impl CLEARREQMASK {
    #[doc = "31:0\\] Set the appropriate bit to enable DMA request for the channel. Write as: Bit \\[Ch\\] = 0: No effect. Use the SETREQMASK.CHNLS to disable channel C from generating requests. Bit \\[Ch\\] = 1: Enables channel \\[C\\] to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn CHNLS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Set the appropriate bit to enable DMA request for the channel. Write as: Bit \\[Ch\\] = 0: No effect. Use the SETREQMASK.CHNLS to disable channel C from generating requests. Bit \\[Ch\\] = 1: Enables channel \\[C\\] to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub const fn set_CHNLS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CLEARREQMASK {
    #[inline(always)]
    fn default() -> CLEARREQMASK {
        CLEARREQMASK(0)
    }
}
impl core::fmt::Debug for CLEARREQMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLEARREQMASK")
            .field("CHNLS", &self.CHNLS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLEARREQMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLEARREQMASK {{ CHNLS: {=u32:?} }}", self.CHNLS())
    }
}
#[doc = "Channel Control Data Base Pointer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "9:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "9:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "31:10\\] This register point to the base address for the primary data structures of each DMA channel. This is not stored in module, but in system memory, thus space must be allocated for this usage when DMA is in usage."]
    #[must_use]
    #[inline(always)]
    pub const fn BASEPTR(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "31:10\\] This register point to the base address for the primary data structures of each DMA channel. This is not stored in module, but in system memory, thus space must be allocated for this usage when DMA is in usage."]
    #[inline(always)]
    pub const fn set_BASEPTR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for CTRL {
    #[inline(always)]
    fn default() -> CTRL {
        CTRL(0)
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("RESERVED0", &self.RESERVED0())
            .field("BASEPTR", &self.BASEPTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ RESERVED0: {=u16:?}, BASEPTR: {=u32:?} }}",
            self.RESERVED0(),
            self.BASEPTR()
        )
    }
}
#[doc = "Channel Request Done Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DONEMASK(pub u32);
impl DONEMASK {
    #[doc = "31:0\\] Controls the propagation of the uDMA done and active state to the assigned peripheral. Specifically used for software channels. Read as: Bit \\[Ch\\] = 0: uDMA done and active state for channel Ch is not blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\] is blocked from contributing to generation of combined uDMA done signal Bit \\[Ch\\] = 1: uDMA done and active state for channel Ch is blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\] is not blocked from contributing to generation of combined uDMA done signal Write as: Bit \\[Ch\\] = 0: Allows uDMA done and active stat to propagate to the peripherals. Note that this disables uDMA done state for channel \\[Ch\\] from contributing to generation of combined uDMA done signal Bit \\[Ch\\] = 1: Blocks uDMA done and active state to propagate to the peripherals. Note that this enables uDMA done for channel \\[Ch\\] to contribute to generation of combined uDMA done signal."]
    #[must_use]
    #[inline(always)]
    pub const fn CHNLS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Controls the propagation of the uDMA done and active state to the assigned peripheral. Specifically used for software channels. Read as: Bit \\[Ch\\] = 0: uDMA done and active state for channel Ch is not blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\] is blocked from contributing to generation of combined uDMA done signal Bit \\[Ch\\] = 1: uDMA done and active state for channel Ch is blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\] is not blocked from contributing to generation of combined uDMA done signal Write as: Bit \\[Ch\\] = 0: Allows uDMA done and active stat to propagate to the peripherals. Note that this disables uDMA done state for channel \\[Ch\\] from contributing to generation of combined uDMA done signal Bit \\[Ch\\] = 1: Blocks uDMA done and active state to propagate to the peripherals. Note that this enables uDMA done for channel \\[Ch\\] to contribute to generation of combined uDMA done signal."]
    #[inline(always)]
    pub const fn set_CHNLS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DONEMASK {
    #[inline(always)]
    fn default() -> DONEMASK {
        DONEMASK(0)
    }
}
impl core::fmt::Debug for DONEMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DONEMASK")
            .field("CHNLS", &self.CHNLS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DONEMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DONEMASK {{ CHNLS: {=u32:?} }}", self.CHNLS())
    }
}
#[doc = "Error Status and Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ERROR(pub u32);
impl ERROR {
    #[doc = "0:0\\] Returns the status of bus error flag in uDMA, or clears this bit Read as: 0: No bus error detected 1: Bus error detected Write as: 0: No effect, status of bus error flag is unchanged. 1: Clears the bus error flag."]
    #[must_use]
    #[inline(always)]
    pub const fn STATUS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Returns the status of bus error flag in uDMA, or clears this bit Read as: 0: No bus error detected 1: Bus error detected Write as: 0: No effect, status of bus error flag is unchanged. 1: Clears the bus error flag."]
    #[inline(always)]
    pub const fn set_STATUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "31:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for ERROR {
    #[inline(always)]
    fn default() -> ERROR {
        ERROR(0)
    }
}
impl core::fmt::Debug for ERROR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERROR")
            .field("STATUS", &self.STATUS())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ERROR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ERROR {{ STATUS: {=bool:?}, RESERVED: {=u32:?} }}",
            self.STATUS(),
            self.RESERVED()
        )
    }
}
#[doc = "Channel Request Done."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct REQDONE(pub u32);
impl REQDONE {
    #[doc = "31:0\\] Reflects the uDMA done status for the given channel, channel \\[Ch\\]. It's a sticky done bit. Unless cleared by writing a 1, it holds the value of 1. Read as: Bit \\[Ch\\] = 0: Request has not completed for channel Ch Bit \\[Ch\\] = 1: Request has completed for the channel Ch Writing a 1 to individual bits would clear the corresponding bit. Write as: Bit \\[Ch\\] = 0: No effect. Bit \\[Ch\\] = 1: The corresponding \\[Ch\\] bit is cleared and is set to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn CHNLS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Reflects the uDMA done status for the given channel, channel \\[Ch\\]. It's a sticky done bit. Unless cleared by writing a 1, it holds the value of 1. Read as: Bit \\[Ch\\] = 0: Request has not completed for channel Ch Bit \\[Ch\\] = 1: Request has completed for the channel Ch Writing a 1 to individual bits would clear the corresponding bit. Write as: Bit \\[Ch\\] = 0: No effect. Bit \\[Ch\\] = 1: The corresponding \\[Ch\\] bit is cleared and is set to 0."]
    #[inline(always)]
    pub const fn set_CHNLS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for REQDONE {
    #[inline(always)]
    fn default() -> REQDONE {
        REQDONE(0)
    }
}
impl core::fmt::Debug for REQDONE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REQDONE")
            .field("CHNLS", &self.CHNLS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for REQDONE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "REQDONE {{ CHNLS: {=u32:?} }}", self.CHNLS())
    }
}
#[doc = "Channel Set UseBurst."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SETBURST(pub u32);
impl SETBURST {
    #[doc = "31:0\\] Returns the useburst status, or disables individual channels from generating single uDMA requests. The value R is the arbitration rate and stored in the controller data structure. Read as: Bit \\[Ch\\] = 0: uDMA channel Ch responds to both burst and single requests on channel C. The controller performs 2^R, or single, bus transfers. Bit \\[Ch\\] = 1: uDMA channel Ch does not respond to single transfer requests. The controller only responds to burst transfer requests and performs 2^R transfers. Write as: Bit \\[Ch\\] = 0: No effect. Use the CLEARBURST.CHNLS to set bit \\[Ch\\] to 0. Bit \\[Ch\\] = 1: Disables single transfer requests on channel Ch. The controller performs 2^R transfers for burst requests. Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn CHNLS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Returns the useburst status, or disables individual channels from generating single uDMA requests. The value R is the arbitration rate and stored in the controller data structure. Read as: Bit \\[Ch\\] = 0: uDMA channel Ch responds to both burst and single requests on channel C. The controller performs 2^R, or single, bus transfers. Bit \\[Ch\\] = 1: uDMA channel Ch does not respond to single transfer requests. The controller only responds to burst transfer requests and performs 2^R transfers. Write as: Bit \\[Ch\\] = 0: No effect. Use the CLEARBURST.CHNLS to set bit \\[Ch\\] to 0. Bit \\[Ch\\] = 1: Disables single transfer requests on channel Ch. The controller performs 2^R transfers for burst requests. Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[inline(always)]
    pub const fn set_CHNLS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SETBURST {
    #[inline(always)]
    fn default() -> SETBURST {
        SETBURST(0)
    }
}
impl core::fmt::Debug for SETBURST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SETBURST")
            .field("CHNLS", &self.CHNLS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SETBURST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SETBURST {{ CHNLS: {=u32:?} }}", self.CHNLS())
    }
}
#[doc = "Set Channel Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SETCHANNELEN(pub u32);
impl SETCHANNELEN {
    #[doc = "31:0\\] Returns the enable status of the channels, or enables the corresponding channels. Read as: Bit \\[Ch\\] = 0: Channel Ch is disabled. Bit \\[Ch\\] = 1: Channel Ch is enabled. Write as: Bit \\[Ch\\] = 0: No effect. Use the CLEARCHANNELEN.CHNLS to disable a channel Bit \\[Ch\\] = 1: Enables channel Ch Writing to a bit where a DMA channel is not implemented has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn CHNLS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Returns the enable status of the channels, or enables the corresponding channels. Read as: Bit \\[Ch\\] = 0: Channel Ch is disabled. Bit \\[Ch\\] = 1: Channel Ch is enabled. Write as: Bit \\[Ch\\] = 0: No effect. Use the CLEARCHANNELEN.CHNLS to disable a channel Bit \\[Ch\\] = 1: Enables channel Ch Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub const fn set_CHNLS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SETCHANNELEN {
    #[inline(always)]
    fn default() -> SETCHANNELEN {
        SETCHANNELEN(0)
    }
}
impl core::fmt::Debug for SETCHANNELEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SETCHANNELEN")
            .field("CHNLS", &self.CHNLS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SETCHANNELEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SETCHANNELEN {{ CHNLS: {=u32:?} }}", self.CHNLS())
    }
}
#[doc = "Channel Set Primary-Alternate."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SETCHNLPRIALT(pub u32);
impl SETCHNLPRIALT {
    #[doc = "31:0\\] Returns the channel control data structure status, or selects the alternate data structure for the corresponding uDMA channel. Read as: Bit \\[Ch\\] = 0: uDMA channel Ch is using the primary data structure. Bit \\[Ch\\] = 1: uDMA channel Ch is using the alternate data structure. Write as: Bit \\[Ch\\] = 0: No effect. Use the CLEARCHNLPRIALT.CHNLS to disable a channel Bit \\[Ch\\] = 1: Selects the alternate data structure for channel Ch Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn CHNLS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Returns the channel control data structure status, or selects the alternate data structure for the corresponding uDMA channel. Read as: Bit \\[Ch\\] = 0: uDMA channel Ch is using the primary data structure. Bit \\[Ch\\] = 1: uDMA channel Ch is using the alternate data structure. Write as: Bit \\[Ch\\] = 0: No effect. Use the CLEARCHNLPRIALT.CHNLS to disable a channel Bit \\[Ch\\] = 1: Selects the alternate data structure for channel Ch Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[inline(always)]
    pub const fn set_CHNLS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SETCHNLPRIALT {
    #[inline(always)]
    fn default() -> SETCHNLPRIALT {
        SETCHNLPRIALT(0)
    }
}
impl core::fmt::Debug for SETCHNLPRIALT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SETCHNLPRIALT")
            .field("CHNLS", &self.CHNLS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SETCHNLPRIALT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SETCHNLPRIALT {{ CHNLS: {=u32:?} }}", self.CHNLS())
    }
}
#[doc = "Set Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SETCHNLPRIORITY(pub u32);
impl SETCHNLPRIORITY {
    #[doc = "31:0\\] Returns the channel priority mask status, or sets the channel priority to high. Read as: Bit \\[Ch\\] = 0: uDMA channel Ch is using the default priority level. Bit \\[Ch\\] = 1: uDMA channel Ch is using a high priority level. Write as: Bit \\[Ch\\] = 0: No effect. Use the CLEARCHNLPRIORITY.CHNLS to set channel Ch to the default priority level. Bit \\[Ch\\] = 1: Channel Ch uses the high priority level. Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn CHNLS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Returns the channel priority mask status, or sets the channel priority to high. Read as: Bit \\[Ch\\] = 0: uDMA channel Ch is using the default priority level. Bit \\[Ch\\] = 1: uDMA channel Ch is using a high priority level. Write as: Bit \\[Ch\\] = 0: No effect. Use the CLEARCHNLPRIORITY.CHNLS to set channel Ch to the default priority level. Bit \\[Ch\\] = 1: Channel Ch uses the high priority level. Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[inline(always)]
    pub const fn set_CHNLS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SETCHNLPRIORITY {
    #[inline(always)]
    fn default() -> SETCHNLPRIORITY {
        SETCHNLPRIORITY(0)
    }
}
impl core::fmt::Debug for SETCHNLPRIORITY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SETCHNLPRIORITY")
            .field("CHNLS", &self.CHNLS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SETCHNLPRIORITY {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SETCHNLPRIORITY {{ CHNLS: {=u32:?} }}", self.CHNLS())
    }
}
#[doc = "Channel Set Request Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SETREQMASK(pub u32);
impl SETREQMASK {
    #[doc = "31:0\\] Returns the burst and single request mask status, or disables the corresponding channel from generating uDMA requests. Read as: Bit \\[Ch\\] = 0: External requests are enabled for channel Ch. Bit \\[Ch\\] = 1: External requests are disabled for channel Ch. Write as: Bit \\[Ch\\] = 0: No effect. Use the CLEARREQMASK.CHNLS to enable uDMA requests. Bit \\[Ch\\] = 1: Disables uDMA burst request channel \\[C\\] and uDMA single request channel \\[C\\] input from generating uDMA requests. Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn CHNLS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Returns the burst and single request mask status, or disables the corresponding channel from generating uDMA requests. Read as: Bit \\[Ch\\] = 0: External requests are enabled for channel Ch. Bit \\[Ch\\] = 1: External requests are disabled for channel Ch. Write as: Bit \\[Ch\\] = 0: No effect. Use the CLEARREQMASK.CHNLS to enable uDMA requests. Bit \\[Ch\\] = 1: Disables uDMA burst request channel \\[C\\] and uDMA single request channel \\[C\\] input from generating uDMA requests. Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[inline(always)]
    pub const fn set_CHNLS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SETREQMASK {
    #[inline(always)]
    fn default() -> SETREQMASK {
        SETREQMASK(0)
    }
}
impl core::fmt::Debug for SETREQMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SETREQMASK")
            .field("CHNLS", &self.CHNLS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SETREQMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SETREQMASK {{ CHNLS: {=u32:?} }}", self.CHNLS())
    }
}
#[doc = "Channel Software Request."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SOFTREQ(pub u32);
impl SOFTREQ {
    #[doc = "31:0\\] Set the appropriate bit to generate a software uDMA request on the corresponding uDMA channel Bit \\[Ch\\] = 0: Does not create a uDMA request for channel Ch Bit \\[Ch\\] = 1: Creates a uDMA request for channel Ch Writing to a bit where a uDMA channel is not implemented does not create a uDMA request for that channel."]
    #[must_use]
    #[inline(always)]
    pub const fn CHNLS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Set the appropriate bit to generate a software uDMA request on the corresponding uDMA channel Bit \\[Ch\\] = 0: Does not create a uDMA request for channel Ch Bit \\[Ch\\] = 1: Creates a uDMA request for channel Ch Writing to a bit where a uDMA channel is not implemented does not create a uDMA request for that channel."]
    #[inline(always)]
    pub const fn set_CHNLS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SOFTREQ {
    #[inline(always)]
    fn default() -> SOFTREQ {
        SOFTREQ(0)
    }
}
impl core::fmt::Debug for SOFTREQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOFTREQ")
            .field("CHNLS", &self.CHNLS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SOFTREQ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SOFTREQ {{ CHNLS: {=u32:?} }}", self.CHNLS())
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATUS(pub u32);
impl STATUS {
    #[doc = "0:0\\] Shows the enable status of the controller as configured by CFG.MASTERENABLE: 0: Controller is disabled 1: Controller is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn MASTERENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Shows the enable status of the controller as configured by CFG.MASTERENABLE: 0: Controller is disabled 1: Controller is enabled."]
    #[inline(always)]
    pub const fn set_MASTERENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
    #[doc = "7:4\\] Current state of the control state machine. State can be one of the following: 0x0: Idle 0x1: Reading channel controller data 0x2: Reading source data end pointer 0x3: Reading destination data end pointer 0x4: Reading source data 0x5: Writing destination data 0x6: Waiting for uDMA request to clear 0x7: Writing channel controller data 0x8: Stalled 0x9: Done 0xA: Peripheral scatter-gather transition 0xB: Undefined ... 0xF: Undefined."]
    #[must_use]
    #[inline(always)]
    pub const fn STATE(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "7:4\\] Current state of the control state machine. State can be one of the following: 0x0: Idle 0x1: Reading channel controller data 0x2: Reading source data end pointer 0x3: Reading destination data end pointer 0x4: Reading source data 0x5: Writing destination data 0x6: Waiting for uDMA request to clear 0x7: Writing channel controller data 0x8: Stalled 0x9: Done 0xA: Peripheral scatter-gather transition 0xB: Undefined ... 0xF: Undefined."]
    #[inline(always)]
    pub const fn set_STATE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "15:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED8(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "20:16\\] Register value returns number of available uDMA channels minus one. For example a read out value of: 0x00: Show that the controller is configured to use 1 uDMA channel 0x01: Shows that the controller is configured to use 2 uDMA channels ... 0x1F: Shows that the controller is configured to use 32 uDMA channels (32-1=31=0x1F)."]
    #[must_use]
    #[inline(always)]
    pub const fn TOTALCHANNELS(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "20:16\\] Register value returns number of available uDMA channels minus one. For example a read out value of: 0x00: Show that the controller is configured to use 1 uDMA channel 0x01: Shows that the controller is configured to use 2 uDMA channels ... 0x1F: Shows that the controller is configured to use 32 uDMA channels (32-1=31=0x1F)."]
    #[inline(always)]
    pub const fn set_TOTALCHANNELS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "27:21\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED21(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x7f;
        val as u8
    }
    #[doc = "27:21\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED21(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 21usize)) | (((val as u32) & 0x7f) << 21usize);
    }
    #[doc = "31:28\\] 0x0: Controller does not include the integration test logic 0x1: Controller includes the integration test logic 0x2: Undefined ... 0xF: Undefined."]
    #[must_use]
    #[inline(always)]
    pub const fn TEST(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] 0x0: Controller does not include the integration test logic 0x1: Controller includes the integration test logic 0x2: Undefined ... 0xF: Undefined."]
    #[inline(always)]
    pub const fn set_TEST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for STATUS {
    #[inline(always)]
    fn default() -> STATUS {
        STATUS(0)
    }
}
impl core::fmt::Debug for STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("MASTERENABLE", &self.MASTERENABLE())
            .field("RESERVED1", &self.RESERVED1())
            .field("STATE", &self.STATE())
            .field("RESERVED8", &self.RESERVED8())
            .field("TOTALCHANNELS", &self.TOTALCHANNELS())
            .field("RESERVED21", &self.RESERVED21())
            .field("TEST", &self.TEST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STATUS {{ MASTERENABLE: {=bool:?}, RESERVED1: {=u8:?}, STATE: {=u8:?}, RESERVED8: {=u8:?}, TOTALCHANNELS: {=u8:?}, RESERVED21: {=u8:?}, TEST: {=u8:?} }}",
            self.MASTERENABLE(),
            self.RESERVED1(),
            self.STATE(),
            self.RESERVED8(),
            self.TOTALCHANNELS(),
            self.RESERVED21(),
            self.TEST()
        )
    }
}
#[doc = "Channel Wait On Request Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WAITONREQ(pub u32);
impl WAITONREQ {
    #[doc = "31:0\\] Channel wait on request status: Bit \\[Ch\\] = 0: Once uDMA receives a single or burst request on channel Ch, this channel may come out of active state even if request is still present. Bit \\[Ch\\] = 1: Once uDMA receives a single or burst request on channel Ch, it keeps channel Ch in active state until the requests are deasserted. This handshake is necessary for channels where the requester is in an asynchronous domain or can run at slower clock speed than uDMA."]
    #[must_use]
    #[inline(always)]
    pub const fn CHNLSTATUS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Channel wait on request status: Bit \\[Ch\\] = 0: Once uDMA receives a single or burst request on channel Ch, this channel may come out of active state even if request is still present. Bit \\[Ch\\] = 1: Once uDMA receives a single or burst request on channel Ch, it keeps channel Ch in active state until the requests are deasserted. This handshake is necessary for channels where the requester is in an asynchronous domain or can run at slower clock speed than uDMA."]
    #[inline(always)]
    pub const fn set_CHNLSTATUS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WAITONREQ {
    #[inline(always)]
    fn default() -> WAITONREQ {
        WAITONREQ(0)
    }
}
impl core::fmt::Debug for WAITONREQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAITONREQ")
            .field("CHNLSTATUS", &self.CHNLSTATUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WAITONREQ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "WAITONREQ {{ CHNLSTATUS: {=u32:?} }}", self.CHNLSTATUS())
    }
}

#[doc = "Doorbell Command Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDR(pub u32);
impl CMDR {
    #[doc = "31:0\\] Command register. Raises an interrupt to the Command and packet engine (CPE) upon write."]
    #[must_use]
    #[inline(always)]
    pub const fn CMD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Command register. Raises an interrupt to the Command and packet engine (CPE) upon write."]
    #[inline(always)]
    pub const fn set_CMD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CMDR {
    #[inline(always)]
    fn default() -> CMDR {
        CMDR(0)
    }
}
impl core::fmt::Debug for CMDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDR").field("CMD", &self.CMD()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CMDR {{ CMD: {=u32:?} }}", self.CMD())
    }
}
#[doc = "Doorbell Command Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDSTA(pub u32);
impl CMDSTA {
    #[doc = "31:0\\] Status of the last command used."]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Status of the last command used."]
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CMDSTA {
    #[inline(always)]
    fn default() -> CMDSTA {
        CMDSTA(0)
    }
}
impl core::fmt::Debug for CMDSTA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDSTA")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDSTA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CMDSTA {{ STAT: {=u32:?} }}", self.STAT())
    }
}
#[doc = "Doorbell Command Acknowledgement Interrupt Flag."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFACKIFG(pub u32);
impl RFACKIFG {
    #[doc = "0:0\\] Interrupt flag for Command ACK."]
    #[must_use]
    #[inline(always)]
    pub const fn ACKFLAG(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Interrupt flag for Command ACK."]
    #[inline(always)]
    pub const fn set_ACKFLAG(&mut self, val: bool) {
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
impl Default for RFACKIFG {
    #[inline(always)]
    fn default() -> RFACKIFG {
        RFACKIFG(0)
    }
}
impl core::fmt::Debug for RFACKIFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFACKIFG")
            .field("ACKFLAG", &self.ACKFLAG())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFACKIFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFACKIFG {{ ACKFLAG: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.ACKFLAG(),
            self.RESERVED1()
        )
    }
}
#[doc = "Interrupt Enable For Command and Packet Engine Generated Interrupts."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCPEIEN(pub u32);
impl RFCPEIEN {
    #[doc = "0:0\\] Interrupt enable for RFCPEIFG.COMMAND_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn COMMAND_DONE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Interrupt enable for RFCPEIFG.COMMAND_DONE."]
    #[inline(always)]
    pub const fn set_COMMAND_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Interrupt enable for RFCPEIFG.LAST_COMMAND_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn LAST_COMMAND_DONE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Interrupt enable for RFCPEIFG.LAST_COMMAND_DONE."]
    #[inline(always)]
    pub const fn set_LAST_COMMAND_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Interrupt enable for RFCPEIFG.FG_COMMAND_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn FG_COMMAND_DONE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Interrupt enable for RFCPEIFG.FG_COMMAND_DONE."]
    #[inline(always)]
    pub const fn set_FG_COMMAND_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Interrupt enable for RFCPEIFG.LAST_FG_COMMAND_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn LAST_FG_COMMAND_DONE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Interrupt enable for RFCPEIFG.LAST_FG_COMMAND_DONE."]
    #[inline(always)]
    pub const fn set_LAST_FG_COMMAND_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Interrupt enable for RFCPEIFG.TX_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_DONE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Interrupt enable for RFCPEIFG.TX_DONE."]
    #[inline(always)]
    pub const fn set_TX_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Interrupt enable for RFCPEIFG.TX_ACK."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_ACK(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Interrupt enable for RFCPEIFG.TX_ACK."]
    #[inline(always)]
    pub const fn set_TX_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Interrupt enable for RFCPEIFG.TX_CTRL."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CTRL(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Interrupt enable for RFCPEIFG.TX_CTRL."]
    #[inline(always)]
    pub const fn set_TX_CTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Interrupt enable for RFCPEIFG.TX_CTRL_ACK."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CTRL_ACK(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Interrupt enable for RFCPEIFG.TX_CTRL_ACK."]
    #[inline(always)]
    pub const fn set_TX_CTRL_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Interrupt enable for RFCPEIFG.TX_CTRL_ACK_ACK."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CTRL_ACK_ACK(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Interrupt enable for RFCPEIFG.TX_CTRL_ACK_ACK."]
    #[inline(always)]
    pub const fn set_TX_CTRL_ACK_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Interrupt enable for RFCPEIFG.TX_RETRANS."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_RETRANS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Interrupt enable for RFCPEIFG.TX_RETRANS."]
    #[inline(always)]
    pub const fn set_TX_RETRANS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Interrupt enable for RFCPEIFG.TX_ENTRY_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_ENTRY_DONE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Interrupt enable for RFCPEIFG.TX_ENTRY_DONE."]
    #[inline(always)]
    pub const fn set_TX_ENTRY_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Interrupt enable for RFCPEIFG.TX_BUFFER_CHANGED."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_BUFFER_CHANGED(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Interrupt enable for RFCPEIFG.TX_BUFFER_CHANGED."]
    #[inline(always)]
    pub const fn set_TX_BUFFER_CHANGED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Interrupt enable for RFCPEIFG.IRQ12."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Interrupt enable for RFCPEIFG.IRQ12."]
    #[inline(always)]
    pub const fn set_IRQ12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Interrupt enable for RFCPEIFG.IRQ13."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Interrupt enable for RFCPEIFG.IRQ13."]
    #[inline(always)]
    pub const fn set_IRQ13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Interrupt enable for RFCPEIFG.IRQ14."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Interrupt enable for RFCPEIFG.IRQ14."]
    #[inline(always)]
    pub const fn set_IRQ14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Interrupt enable for RFCPEIFG.IRQ15."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Interrupt enable for RFCPEIFG.IRQ15."]
    #[inline(always)]
    pub const fn set_IRQ15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Interrupt enable for RFCPEIFG.RX_OK."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_OK(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Interrupt enable for RFCPEIFG.RX_OK."]
    #[inline(always)]
    pub const fn set_RX_OK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Interrupt enable for RFCPEIFG.RX_NOK."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_NOK(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Interrupt enable for RFCPEIFG.RX_NOK."]
    #[inline(always)]
    pub const fn set_RX_NOK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Interrupt enable for RFCPEIFG.RX_IGNORED."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_IGNORED(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Interrupt enable for RFCPEIFG.RX_IGNORED."]
    #[inline(always)]
    pub const fn set_RX_IGNORED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Interrupt enable for RFCPEIFG.RX_EMPTY."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_EMPTY(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Interrupt enable for RFCPEIFG.RX_EMPTY."]
    #[inline(always)]
    pub const fn set_RX_EMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Interrupt enable for RFCPEIFG.RX_CTRL."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_CTRL(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Interrupt enable for RFCPEIFG.RX_CTRL."]
    #[inline(always)]
    pub const fn set_RX_CTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Interrupt enable for RFCPEIFG.RX_CTRL_ACK."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_CTRL_ACK(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Interrupt enable for RFCPEIFG.RX_CTRL_ACK."]
    #[inline(always)]
    pub const fn set_RX_CTRL_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Interrupt enable for RFCPEIFG.RX_BUF_FULL."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_BUF_FULL(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Interrupt enable for RFCPEIFG.RX_BUF_FULL."]
    #[inline(always)]
    pub const fn set_RX_BUF_FULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Interrupt enable for RFCPEIFG.RX_ENTRY_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_ENTRY_DONE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Interrupt enable for RFCPEIFG.RX_ENTRY_DONE."]
    #[inline(always)]
    pub const fn set_RX_ENTRY_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Interrupt enable for RFCPEIFG.RX_DATA_WRITTEN."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_DATA_WRITTEN(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Interrupt enable for RFCPEIFG.RX_DATA_WRITTEN."]
    #[inline(always)]
    pub const fn set_RX_DATA_WRITTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Interrupt enable for RFCPEIFG.RX_N_DATA_WRITTEN."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_N_DATA_WRITTEN(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Interrupt enable for RFCPEIFG.RX_N_DATA_WRITTEN."]
    #[inline(always)]
    pub const fn set_RX_N_DATA_WRITTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Interrupt enable for RFCPEIFG.RX_ABORTED."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_ABORTED(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Interrupt enable for RFCPEIFG.RX_ABORTED."]
    #[inline(always)]
    pub const fn set_RX_ABORTED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Interrupt enable for RFCPEIFG.IRQ27."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Interrupt enable for RFCPEIFG.IRQ27."]
    #[inline(always)]
    pub const fn set_IRQ27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Interrupt enable for RFCPEIFG.SYNTH_NO_LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn SYNTH_NO_LOCK(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Interrupt enable for RFCPEIFG.SYNTH_NO_LOCK."]
    #[inline(always)]
    pub const fn set_SYNTH_NO_LOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Interrupt enable for RFCPEIFG.MODULES_UNLOCKED."]
    #[must_use]
    #[inline(always)]
    pub const fn MODULES_UNLOCKED(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Interrupt enable for RFCPEIFG.MODULES_UNLOCKED."]
    #[inline(always)]
    pub const fn set_MODULES_UNLOCKED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Interrupt enable for RFCPEIFG.BOOT_DONE."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_DONE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Interrupt enable for RFCPEIFG.BOOT_DONE."]
    #[inline(always)]
    pub const fn set_BOOT_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Interrupt enable for RFCPEIFG.INTERNAL_ERROR."]
    #[must_use]
    #[inline(always)]
    pub const fn INTERNAL_ERROR(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Interrupt enable for RFCPEIFG.INTERNAL_ERROR."]
    #[inline(always)]
    pub const fn set_INTERNAL_ERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RFCPEIEN {
    #[inline(always)]
    fn default() -> RFCPEIEN {
        RFCPEIEN(0)
    }
}
impl core::fmt::Debug for RFCPEIEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCPEIEN")
            .field("COMMAND_DONE", &self.COMMAND_DONE())
            .field("LAST_COMMAND_DONE", &self.LAST_COMMAND_DONE())
            .field("FG_COMMAND_DONE", &self.FG_COMMAND_DONE())
            .field("LAST_FG_COMMAND_DONE", &self.LAST_FG_COMMAND_DONE())
            .field("TX_DONE", &self.TX_DONE())
            .field("TX_ACK", &self.TX_ACK())
            .field("TX_CTRL", &self.TX_CTRL())
            .field("TX_CTRL_ACK", &self.TX_CTRL_ACK())
            .field("TX_CTRL_ACK_ACK", &self.TX_CTRL_ACK_ACK())
            .field("TX_RETRANS", &self.TX_RETRANS())
            .field("TX_ENTRY_DONE", &self.TX_ENTRY_DONE())
            .field("TX_BUFFER_CHANGED", &self.TX_BUFFER_CHANGED())
            .field("IRQ12", &self.IRQ12())
            .field("IRQ13", &self.IRQ13())
            .field("IRQ14", &self.IRQ14())
            .field("IRQ15", &self.IRQ15())
            .field("RX_OK", &self.RX_OK())
            .field("RX_NOK", &self.RX_NOK())
            .field("RX_IGNORED", &self.RX_IGNORED())
            .field("RX_EMPTY", &self.RX_EMPTY())
            .field("RX_CTRL", &self.RX_CTRL())
            .field("RX_CTRL_ACK", &self.RX_CTRL_ACK())
            .field("RX_BUF_FULL", &self.RX_BUF_FULL())
            .field("RX_ENTRY_DONE", &self.RX_ENTRY_DONE())
            .field("RX_DATA_WRITTEN", &self.RX_DATA_WRITTEN())
            .field("RX_N_DATA_WRITTEN", &self.RX_N_DATA_WRITTEN())
            .field("RX_ABORTED", &self.RX_ABORTED())
            .field("IRQ27", &self.IRQ27())
            .field("SYNTH_NO_LOCK", &self.SYNTH_NO_LOCK())
            .field("MODULES_UNLOCKED", &self.MODULES_UNLOCKED())
            .field("BOOT_DONE", &self.BOOT_DONE())
            .field("INTERNAL_ERROR", &self.INTERNAL_ERROR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCPEIEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCPEIEN {{ COMMAND_DONE: {=bool:?}, LAST_COMMAND_DONE: {=bool:?}, FG_COMMAND_DONE: {=bool:?}, LAST_FG_COMMAND_DONE: {=bool:?}, TX_DONE: {=bool:?}, TX_ACK: {=bool:?}, TX_CTRL: {=bool:?}, TX_CTRL_ACK: {=bool:?}, TX_CTRL_ACK_ACK: {=bool:?}, TX_RETRANS: {=bool:?}, TX_ENTRY_DONE: {=bool:?}, TX_BUFFER_CHANGED: {=bool:?}, IRQ12: {=bool:?}, IRQ13: {=bool:?}, IRQ14: {=bool:?}, IRQ15: {=bool:?}, RX_OK: {=bool:?}, RX_NOK: {=bool:?}, RX_IGNORED: {=bool:?}, RX_EMPTY: {=bool:?}, RX_CTRL: {=bool:?}, RX_CTRL_ACK: {=bool:?}, RX_BUF_FULL: {=bool:?}, RX_ENTRY_DONE: {=bool:?}, RX_DATA_WRITTEN: {=bool:?}, RX_N_DATA_WRITTEN: {=bool:?}, RX_ABORTED: {=bool:?}, IRQ27: {=bool:?}, SYNTH_NO_LOCK: {=bool:?}, MODULES_UNLOCKED: {=bool:?}, BOOT_DONE: {=bool:?}, INTERNAL_ERROR: {=bool:?} }}",
            self.COMMAND_DONE(),
            self.LAST_COMMAND_DONE(),
            self.FG_COMMAND_DONE(),
            self.LAST_FG_COMMAND_DONE(),
            self.TX_DONE(),
            self.TX_ACK(),
            self.TX_CTRL(),
            self.TX_CTRL_ACK(),
            self.TX_CTRL_ACK_ACK(),
            self.TX_RETRANS(),
            self.TX_ENTRY_DONE(),
            self.TX_BUFFER_CHANGED(),
            self.IRQ12(),
            self.IRQ13(),
            self.IRQ14(),
            self.IRQ15(),
            self.RX_OK(),
            self.RX_NOK(),
            self.RX_IGNORED(),
            self.RX_EMPTY(),
            self.RX_CTRL(),
            self.RX_CTRL_ACK(),
            self.RX_BUF_FULL(),
            self.RX_ENTRY_DONE(),
            self.RX_DATA_WRITTEN(),
            self.RX_N_DATA_WRITTEN(),
            self.RX_ABORTED(),
            self.IRQ27(),
            self.SYNTH_NO_LOCK(),
            self.MODULES_UNLOCKED(),
            self.BOOT_DONE(),
            self.INTERNAL_ERROR()
        )
    }
}
#[doc = "Interrupt Flags For Command and Packet Engine Generated Interrupts."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCPEIFG(pub u32);
impl RFCPEIFG {
    #[doc = "0:0\\] Interrupt flag 0. A radio operation has finished. (IEEE 802.15.4 mode: A background level radio operation command has finished.) Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn COMMAND_DONE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Interrupt flag 0. A radio operation has finished. (IEEE 802.15.4 mode: A background level radio operation command has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_COMMAND_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Interrupt flag 1. The last radio operation command in a chain of commands has finished. (IEEE 802.15.4 mode: The last background level radio operation command in a chain of commands has finished.) Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn LAST_COMMAND_DONE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Interrupt flag 1. The last radio operation command in a chain of commands has finished. (IEEE 802.15.4 mode: The last background level radio operation command in a chain of commands has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_LAST_COMMAND_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Interrupt flag 2. IEEE 802.15.4 mode only: A foreground radio operation command has finished. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn FG_COMMAND_DONE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Interrupt flag 2. IEEE 802.15.4 mode only: A foreground radio operation command has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_FG_COMMAND_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Interrupt flag 3. IEEE 802.15.4 mode only: The last foreground radio operation command in a chain of commands has finished. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn LAST_FG_COMMAND_DONE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Interrupt flag 3. IEEE 802.15.4 mode only: The last foreground radio operation command in a chain of commands has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_LAST_FG_COMMAND_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Interrupt flag 4. Packet transmitted. (BLE mode: A packet has been transmitted.) (IEEE 802.15.4 mode: A frame has been transmitted). Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_DONE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Interrupt flag 4. Packet transmitted. (BLE mode: A packet has been transmitted.) (IEEE 802.15.4 mode: A frame has been transmitted). Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_TX_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Interrupt flag 5. BLE mode: Acknowledgement received on a transmitted packet. IEEE 802.15.4 mode: Transmitted automatic ACK frame. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_ACK(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Interrupt flag 5. BLE mode: Acknowledgement received on a transmitted packet. IEEE 802.15.4 mode: Transmitted automatic ACK frame. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_TX_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Interrupt flag 6. BLE mode: Transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CTRL(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Interrupt flag 6. BLE mode: Transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_TX_CTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Interrupt flag 7. BLE mode: Acknowledgement received on a transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CTRL_ACK(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Interrupt flag 7. BLE mode: Acknowledgement received on a transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_TX_CTRL_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Interrupt flag 8. BLE mode only: Acknowledgement received on a transmitted LL control packet, and acknowledgement transmitted for that packet. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CTRL_ACK_ACK(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Interrupt flag 8. BLE mode only: Acknowledgement received on a transmitted LL control packet, and acknowledgement transmitted for that packet. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_TX_CTRL_ACK_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Interrupt flag 9. BLE mode only: Packet retransmitted. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_RETRANS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Interrupt flag 9. BLE mode only: Packet retransmitted. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_TX_RETRANS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Interrupt flag 10. Tx queue data entry state changed to finished. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_ENTRY_DONE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Interrupt flag 10. Tx queue data entry state changed to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_TX_ENTRY_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Interrupt flag 11. BLE mode only: A buffer change is complete after CMD_BLE_ADV_PAYLOAD. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_BUFFER_CHANGED(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Interrupt flag 11. BLE mode only: A buffer change is complete after CMD_BLE_ADV_PAYLOAD. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_TX_BUFFER_CHANGED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Interrupt flag 12. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Interrupt flag 12. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_IRQ12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Interrupt flag 13. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Interrupt flag 13. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_IRQ13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Interrupt flag 14. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Interrupt flag 14. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_IRQ14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Interrupt flag 15. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Interrupt flag 15. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_IRQ15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Interrupt flag 16. Packet received correctly. BLE mode: Packet received with CRC OK, payload, and not to be ignored. IEEE 802.15.4 mode: Frame received with CRC OK. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_OK(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Interrupt flag 16. Packet received correctly. BLE mode: Packet received with CRC OK, payload, and not to be ignored. IEEE 802.15.4 mode: Frame received with CRC OK. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RX_OK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Interrupt flag 17. Packet received with CRC error. BLE mode: Packet received with CRC error. IEEE 802.15.4 mode: Frame received with CRC error. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_NOK(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Interrupt flag 17. Packet received with CRC error. BLE mode: Packet received with CRC error. IEEE 802.15.4 mode: Frame received with CRC error. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RX_NOK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Interrupt flag 18. Packet received, but can be ignored. BLE mode: Packet received with CRC OK, but to be ignored. IEEE 802.15.4 mode: Frame received with ignore flag set. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_IGNORED(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Interrupt flag 18. Packet received, but can be ignored. BLE mode: Packet received with CRC OK, but to be ignored. IEEE 802.15.4 mode: Frame received with ignore flag set. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RX_IGNORED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Interrupt flag 19. BLE mode only: Packet received with CRC OK, not to be ignored, no payload. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_EMPTY(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Interrupt flag 19. BLE mode only: Packet received with CRC OK, not to be ignored, no payload. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RX_EMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Interrupt flag 20. BLE mode only: LL control packet received with CRC OK, not to be ignored. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_CTRL(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Interrupt flag 20. BLE mode only: LL control packet received with CRC OK, not to be ignored. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RX_CTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Interrupt flag 21. BLE mode only: LL control packet received with CRC OK, not to be ignored, then acknowledgement sent. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_CTRL_ACK(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Interrupt flag 21. BLE mode only: LL control packet received with CRC OK, not to be ignored, then acknowledgement sent. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RX_CTRL_ACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Interrupt flag 22. Packet received that did not fit in Rx queue. BLE mode: Packet received that did not fit in the Rx queue. IEEE 802.15.4 mode: Frame received that did not fit in the Rx queue. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_BUF_FULL(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Interrupt flag 22. Packet received that did not fit in Rx queue. BLE mode: Packet received that did not fit in the Rx queue. IEEE 802.15.4 mode: Frame received that did not fit in the Rx queue. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RX_BUF_FULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Interrupt flag 23. Rx queue data entry changing state to finished. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_ENTRY_DONE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Interrupt flag 23. Rx queue data entry changing state to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RX_ENTRY_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Interrupt flag 24. Data written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_DATA_WRITTEN(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Interrupt flag 24. Data written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RX_DATA_WRITTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Interrupt flag 25. Specified number of bytes written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_N_DATA_WRITTEN(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Interrupt flag 25. Specified number of bytes written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RX_N_DATA_WRITTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Interrupt flag 26. Packet reception stopped before packet was done. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_ABORTED(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Interrupt flag 26. Packet reception stopped before packet was done. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RX_ABORTED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Interrupt flag 27. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Interrupt flag 27. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_IRQ27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Interrupt flag 28. The phase-locked loop in frequency synthesizer has reported loss of lock. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn SYNTH_NO_LOCK(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Interrupt flag 28. The phase-locked loop in frequency synthesizer has reported loss of lock. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_SYNTH_NO_LOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Interrupt flag 29. As part of command and packet engine (CPE) boot process, it has opened access to RF Core modules and memories. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn MODULES_UNLOCKED(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Interrupt flag 29. As part of command and packet engine (CPE) boot process, it has opened access to RF Core modules and memories. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_MODULES_UNLOCKED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Interrupt flag 30. The command and packet engine (CPE) boot is finished. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_DONE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Interrupt flag 30. The command and packet engine (CPE) boot is finished. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_BOOT_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Interrupt flag 31. The command and packet engine (CPE) has observed an unexpected error. A reset of the CPE is needed. This can be done by switching the RF Core power domain off and on in PRCM:PDCTL1RFC. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn INTERNAL_ERROR(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Interrupt flag 31. The command and packet engine (CPE) has observed an unexpected error. A reset of the CPE is needed. This can be done by switching the RF Core power domain off and on in PRCM:PDCTL1RFC. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_INTERNAL_ERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RFCPEIFG {
    #[inline(always)]
    fn default() -> RFCPEIFG {
        RFCPEIFG(0)
    }
}
impl core::fmt::Debug for RFCPEIFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCPEIFG")
            .field("COMMAND_DONE", &self.COMMAND_DONE())
            .field("LAST_COMMAND_DONE", &self.LAST_COMMAND_DONE())
            .field("FG_COMMAND_DONE", &self.FG_COMMAND_DONE())
            .field("LAST_FG_COMMAND_DONE", &self.LAST_FG_COMMAND_DONE())
            .field("TX_DONE", &self.TX_DONE())
            .field("TX_ACK", &self.TX_ACK())
            .field("TX_CTRL", &self.TX_CTRL())
            .field("TX_CTRL_ACK", &self.TX_CTRL_ACK())
            .field("TX_CTRL_ACK_ACK", &self.TX_CTRL_ACK_ACK())
            .field("TX_RETRANS", &self.TX_RETRANS())
            .field("TX_ENTRY_DONE", &self.TX_ENTRY_DONE())
            .field("TX_BUFFER_CHANGED", &self.TX_BUFFER_CHANGED())
            .field("IRQ12", &self.IRQ12())
            .field("IRQ13", &self.IRQ13())
            .field("IRQ14", &self.IRQ14())
            .field("IRQ15", &self.IRQ15())
            .field("RX_OK", &self.RX_OK())
            .field("RX_NOK", &self.RX_NOK())
            .field("RX_IGNORED", &self.RX_IGNORED())
            .field("RX_EMPTY", &self.RX_EMPTY())
            .field("RX_CTRL", &self.RX_CTRL())
            .field("RX_CTRL_ACK", &self.RX_CTRL_ACK())
            .field("RX_BUF_FULL", &self.RX_BUF_FULL())
            .field("RX_ENTRY_DONE", &self.RX_ENTRY_DONE())
            .field("RX_DATA_WRITTEN", &self.RX_DATA_WRITTEN())
            .field("RX_N_DATA_WRITTEN", &self.RX_N_DATA_WRITTEN())
            .field("RX_ABORTED", &self.RX_ABORTED())
            .field("IRQ27", &self.IRQ27())
            .field("SYNTH_NO_LOCK", &self.SYNTH_NO_LOCK())
            .field("MODULES_UNLOCKED", &self.MODULES_UNLOCKED())
            .field("BOOT_DONE", &self.BOOT_DONE())
            .field("INTERNAL_ERROR", &self.INTERNAL_ERROR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCPEIFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCPEIFG {{ COMMAND_DONE: {=bool:?}, LAST_COMMAND_DONE: {=bool:?}, FG_COMMAND_DONE: {=bool:?}, LAST_FG_COMMAND_DONE: {=bool:?}, TX_DONE: {=bool:?}, TX_ACK: {=bool:?}, TX_CTRL: {=bool:?}, TX_CTRL_ACK: {=bool:?}, TX_CTRL_ACK_ACK: {=bool:?}, TX_RETRANS: {=bool:?}, TX_ENTRY_DONE: {=bool:?}, TX_BUFFER_CHANGED: {=bool:?}, IRQ12: {=bool:?}, IRQ13: {=bool:?}, IRQ14: {=bool:?}, IRQ15: {=bool:?}, RX_OK: {=bool:?}, RX_NOK: {=bool:?}, RX_IGNORED: {=bool:?}, RX_EMPTY: {=bool:?}, RX_CTRL: {=bool:?}, RX_CTRL_ACK: {=bool:?}, RX_BUF_FULL: {=bool:?}, RX_ENTRY_DONE: {=bool:?}, RX_DATA_WRITTEN: {=bool:?}, RX_N_DATA_WRITTEN: {=bool:?}, RX_ABORTED: {=bool:?}, IRQ27: {=bool:?}, SYNTH_NO_LOCK: {=bool:?}, MODULES_UNLOCKED: {=bool:?}, BOOT_DONE: {=bool:?}, INTERNAL_ERROR: {=bool:?} }}",
            self.COMMAND_DONE(),
            self.LAST_COMMAND_DONE(),
            self.FG_COMMAND_DONE(),
            self.LAST_FG_COMMAND_DONE(),
            self.TX_DONE(),
            self.TX_ACK(),
            self.TX_CTRL(),
            self.TX_CTRL_ACK(),
            self.TX_CTRL_ACK_ACK(),
            self.TX_RETRANS(),
            self.TX_ENTRY_DONE(),
            self.TX_BUFFER_CHANGED(),
            self.IRQ12(),
            self.IRQ13(),
            self.IRQ14(),
            self.IRQ15(),
            self.RX_OK(),
            self.RX_NOK(),
            self.RX_IGNORED(),
            self.RX_EMPTY(),
            self.RX_CTRL(),
            self.RX_CTRL_ACK(),
            self.RX_BUF_FULL(),
            self.RX_ENTRY_DONE(),
            self.RX_DATA_WRITTEN(),
            self.RX_N_DATA_WRITTEN(),
            self.RX_ABORTED(),
            self.IRQ27(),
            self.SYNTH_NO_LOCK(),
            self.MODULES_UNLOCKED(),
            self.BOOT_DONE(),
            self.INTERNAL_ERROR()
        )
    }
}
#[doc = "Interrupt Vector Selection For Command and Packet Engine Generated Interrupts."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFCPEISL(pub u32);
impl RFCPEISL {
    #[doc = "0:0\\] Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn COMMAND_DONE(&self) -> super::vals::COMMAND_DONE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::COMMAND_DONE::from_bits(val as u8)
    }
    #[doc = "0:0\\] Select which CPU interrupt vector the RFCPEIFG.COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub const fn set_COMMAND_DONE(&mut self, val: super::vals::COMMAND_DONE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn LAST_COMMAND_DONE(&self) -> super::vals::LAST_COMMAND_DONE {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::LAST_COMMAND_DONE::from_bits(val as u8)
    }
    #[doc = "1:1\\] Select which CPU interrupt vector the RFCPEIFG.LAST_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub const fn set_LAST_COMMAND_DONE(&mut self, val: super::vals::LAST_COMMAND_DONE) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn FG_COMMAND_DONE(&self) -> super::vals::FG_COMMAND_DONE {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::FG_COMMAND_DONE::from_bits(val as u8)
    }
    #[doc = "2:2\\] Select which CPU interrupt vector the RFCPEIFG.FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub const fn set_FG_COMMAND_DONE(&mut self, val: super::vals::FG_COMMAND_DONE) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn LAST_FG_COMMAND_DONE(&self) -> super::vals::LAST_FG_COMMAND_DONE {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::LAST_FG_COMMAND_DONE::from_bits(val as u8)
    }
    #[doc = "3:3\\] Select which CPU interrupt vector the RFCPEIFG.LAST_FG_COMMAND_DONE interrupt should use."]
    #[inline(always)]
    pub const fn set_LAST_FG_COMMAND_DONE(&mut self, val: super::vals::LAST_FG_COMMAND_DONE) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_DONE(&self) -> super::vals::TX_DONE {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::TX_DONE::from_bits(val as u8)
    }
    #[doc = "4:4\\] Select which CPU interrupt vector the RFCPEIFG.TX_DONE interrupt should use."]
    #[inline(always)]
    pub const fn set_TX_DONE(&mut self, val: super::vals::TX_DONE) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_ACK(&self) -> super::vals::TX_ACK {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::TX_ACK::from_bits(val as u8)
    }
    #[doc = "5:5\\] Select which CPU interrupt vector the RFCPEIFG.TX_ACK interrupt should use."]
    #[inline(always)]
    pub const fn set_TX_ACK(&mut self, val: super::vals::TX_ACK) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CTRL(&self) -> super::vals::TX_CTRL {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::TX_CTRL::from_bits(val as u8)
    }
    #[doc = "6:6\\] Select which CPU interrupt vector the RFCPEIFG.TX_CTRL interrupt should use."]
    #[inline(always)]
    pub const fn set_TX_CTRL(&mut self, val: super::vals::TX_CTRL) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CTRL_ACK(&self) -> super::vals::TX_CTRL_ACK {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::TX_CTRL_ACK::from_bits(val as u8)
    }
    #[doc = "7:7\\] Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    pub const fn set_TX_CTRL_ACK(&mut self, val: super::vals::TX_CTRL_ACK) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CTRL_ACK_ACK(&self) -> super::vals::TX_CTRL_ACK_ACK {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::TX_CTRL_ACK_ACK::from_bits(val as u8)
    }
    #[doc = "8:8\\] Select which CPU interrupt vector the RFCPEIFG.TX_CTRL_ACK_ACK interrupt should use."]
    #[inline(always)]
    pub const fn set_TX_CTRL_ACK_ACK(&mut self, val: super::vals::TX_CTRL_ACK_ACK) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_RETRANS(&self) -> super::vals::TX_RETRANS {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::TX_RETRANS::from_bits(val as u8)
    }
    #[doc = "9:9\\] Select which CPU interrupt vector the RFCPEIFG.TX_RETRANS interrupt should use."]
    #[inline(always)]
    pub const fn set_TX_RETRANS(&mut self, val: super::vals::TX_RETRANS) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_ENTRY_DONE(&self) -> super::vals::TX_ENTRY_DONE {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::TX_ENTRY_DONE::from_bits(val as u8)
    }
    #[doc = "10:10\\] Select which CPU interrupt vector the RFCPEIFG.TX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    pub const fn set_TX_ENTRY_DONE(&mut self, val: super::vals::TX_ENTRY_DONE) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_BUFFER_CHANGED(&self) -> super::vals::TX_BUFFER_CHANGED {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::TX_BUFFER_CHANGED::from_bits(val as u8)
    }
    #[doc = "11:11\\] Select which CPU interrupt vector the RFCPEIFG.TX_BUFFER_CHANGED interrupt should use."]
    #[inline(always)]
    pub const fn set_TX_BUFFER_CHANGED(&mut self, val: super::vals::TX_BUFFER_CHANGED) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ12(&self) -> super::vals::IRQ12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::IRQ12::from_bits(val as u8)
    }
    #[doc = "12:12\\] Select which CPU interrupt vector the RFCPEIFG.IRQ12 interrupt should use."]
    #[inline(always)]
    pub const fn set_IRQ12(&mut self, val: super::vals::IRQ12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ13(&self) -> super::vals::IRQ13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::IRQ13::from_bits(val as u8)
    }
    #[doc = "13:13\\] Select which CPU interrupt vector the RFCPEIFG.IRQ13 interrupt should use."]
    #[inline(always)]
    pub const fn set_IRQ13(&mut self, val: super::vals::IRQ13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ14(&self) -> super::vals::IRQ14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::IRQ14::from_bits(val as u8)
    }
    #[doc = "14:14\\] Select which CPU interrupt vector the RFCPEIFG.IRQ14 interrupt should use."]
    #[inline(always)]
    pub const fn set_IRQ14(&mut self, val: super::vals::IRQ14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ15(&self) -> super::vals::IRQ15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::IRQ15::from_bits(val as u8)
    }
    #[doc = "15:15\\] Select which CPU interrupt vector the RFCPEIFG.IRQ15 interrupt should use."]
    #[inline(always)]
    pub const fn set_IRQ15(&mut self, val: super::vals::IRQ15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_OK(&self) -> super::vals::RX_OK {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::RX_OK::from_bits(val as u8)
    }
    #[doc = "16:16\\] Select which CPU interrupt vector the RFCPEIFG.RX_OK interrupt should use."]
    #[inline(always)]
    pub const fn set_RX_OK(&mut self, val: super::vals::RX_OK) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_NOK(&self) -> super::vals::RX_NOK {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::RX_NOK::from_bits(val as u8)
    }
    #[doc = "17:17\\] Select which CPU interrupt vector the RFCPEIFG.RX_NOK interrupt should use."]
    #[inline(always)]
    pub const fn set_RX_NOK(&mut self, val: super::vals::RX_NOK) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_IGNORED(&self) -> super::vals::RX_IGNORED {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::RX_IGNORED::from_bits(val as u8)
    }
    #[doc = "18:18\\] Select which CPU interrupt vector the RFCPEIFG.RX_IGNORED interrupt should use."]
    #[inline(always)]
    pub const fn set_RX_IGNORED(&mut self, val: super::vals::RX_IGNORED) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_EMPTY(&self) -> super::vals::RX_EMPTY {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::RX_EMPTY::from_bits(val as u8)
    }
    #[doc = "19:19\\] Select which CPU interrupt vector the RFCPEIFG.RX_EMPTY interrupt should use."]
    #[inline(always)]
    pub const fn set_RX_EMPTY(&mut self, val: super::vals::RX_EMPTY) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_CTRL(&self) -> super::vals::RX_CTRL {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::RX_CTRL::from_bits(val as u8)
    }
    #[doc = "20:20\\] Select which CPU interrupt vector the RFCPEIFG.RX_CTRL interrupt should use."]
    #[inline(always)]
    pub const fn set_RX_CTRL(&mut self, val: super::vals::RX_CTRL) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_CTRL_ACK(&self) -> super::vals::RX_CTRL_ACK {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::RX_CTRL_ACK::from_bits(val as u8)
    }
    #[doc = "21:21\\] Select which CPU interrupt vector the RFCPEIFG.RX_CTRL_ACK interrupt should use."]
    #[inline(always)]
    pub const fn set_RX_CTRL_ACK(&mut self, val: super::vals::RX_CTRL_ACK) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_BUF_FULL(&self) -> super::vals::RX_BUF_FULL {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::RX_BUF_FULL::from_bits(val as u8)
    }
    #[doc = "22:22\\] Select which CPU interrupt vector the RFCPEIFG.RX_BUF_FULL interrupt should use."]
    #[inline(always)]
    pub const fn set_RX_BUF_FULL(&mut self, val: super::vals::RX_BUF_FULL) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_ENTRY_DONE(&self) -> super::vals::RX_ENTRY_DONE {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::RX_ENTRY_DONE::from_bits(val as u8)
    }
    #[doc = "23:23\\] Select which CPU interrupt vector the RFCPEIFG.RX_ENTRY_DONE interrupt should use."]
    #[inline(always)]
    pub const fn set_RX_ENTRY_DONE(&mut self, val: super::vals::RX_ENTRY_DONE) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_DATA_WRITTEN(&self) -> super::vals::RX_DATA_WRITTEN {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::RX_DATA_WRITTEN::from_bits(val as u8)
    }
    #[doc = "24:24\\] Select which CPU interrupt vector the RFCPEIFG.RX_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    pub const fn set_RX_DATA_WRITTEN(&mut self, val: super::vals::RX_DATA_WRITTEN) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_N_DATA_WRITTEN(&self) -> super::vals::RX_N_DATA_WRITTEN {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::RX_N_DATA_WRITTEN::from_bits(val as u8)
    }
    #[doc = "25:25\\] Select which CPU interrupt vector the RFCPEIFG.RX_N_DATA_WRITTEN interrupt should use."]
    #[inline(always)]
    pub const fn set_RX_N_DATA_WRITTEN(&mut self, val: super::vals::RX_N_DATA_WRITTEN) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn RX_ABORTED(&self) -> super::vals::RX_ABORTED {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::RX_ABORTED::from_bits(val as u8)
    }
    #[doc = "26:26\\] Select which CPU interrupt vector the RFCPEIFG.RX_ABORTED interrupt should use."]
    #[inline(always)]
    pub const fn set_RX_ABORTED(&mut self, val: super::vals::RX_ABORTED) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn IRQ27(&self) -> super::vals::IRQ27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::IRQ27::from_bits(val as u8)
    }
    #[doc = "27:27\\] Select which CPU interrupt vector the RFCPEIFG.IRQ27 interrupt should use."]
    #[inline(always)]
    pub const fn set_IRQ27(&mut self, val: super::vals::IRQ27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn SYNTH_NO_LOCK(&self) -> super::vals::SYNTH_NO_LOCK {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::SYNTH_NO_LOCK::from_bits(val as u8)
    }
    #[doc = "28:28\\] Select which CPU interrupt vector the RFCPEIFG.SYNTH_NO_LOCK interrupt should use."]
    #[inline(always)]
    pub const fn set_SYNTH_NO_LOCK(&mut self, val: super::vals::SYNTH_NO_LOCK) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn MODULES_UNLOCKED(&self) -> super::vals::MODULES_UNLOCKED {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::MODULES_UNLOCKED::from_bits(val as u8)
    }
    #[doc = "29:29\\] Select which CPU interrupt vector the RFCPEIFG.MODULES_UNLOCKED interrupt should use."]
    #[inline(always)]
    pub const fn set_MODULES_UNLOCKED(&mut self, val: super::vals::MODULES_UNLOCKED) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_DONE(&self) -> super::vals::BOOT_DONE {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::BOOT_DONE::from_bits(val as u8)
    }
    #[doc = "30:30\\] Select which CPU interrupt vector the RFCPEIFG.BOOT_DONE interrupt should use."]
    #[inline(always)]
    pub const fn set_BOOT_DONE(&mut self, val: super::vals::BOOT_DONE) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
    #[must_use]
    #[inline(always)]
    pub const fn INTERNAL_ERROR(&self) -> super::vals::INTERNAL_ERROR {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::INTERNAL_ERROR::from_bits(val as u8)
    }
    #[doc = "31:31\\] Select which CPU interrupt vector the RFCPEIFG.INTERNAL_ERROR interrupt should use."]
    #[inline(always)]
    pub const fn set_INTERNAL_ERROR(&mut self, val: super::vals::INTERNAL_ERROR) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for RFCPEISL {
    #[inline(always)]
    fn default() -> RFCPEISL {
        RFCPEISL(0)
    }
}
impl core::fmt::Debug for RFCPEISL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFCPEISL")
            .field("COMMAND_DONE", &self.COMMAND_DONE())
            .field("LAST_COMMAND_DONE", &self.LAST_COMMAND_DONE())
            .field("FG_COMMAND_DONE", &self.FG_COMMAND_DONE())
            .field("LAST_FG_COMMAND_DONE", &self.LAST_FG_COMMAND_DONE())
            .field("TX_DONE", &self.TX_DONE())
            .field("TX_ACK", &self.TX_ACK())
            .field("TX_CTRL", &self.TX_CTRL())
            .field("TX_CTRL_ACK", &self.TX_CTRL_ACK())
            .field("TX_CTRL_ACK_ACK", &self.TX_CTRL_ACK_ACK())
            .field("TX_RETRANS", &self.TX_RETRANS())
            .field("TX_ENTRY_DONE", &self.TX_ENTRY_DONE())
            .field("TX_BUFFER_CHANGED", &self.TX_BUFFER_CHANGED())
            .field("IRQ12", &self.IRQ12())
            .field("IRQ13", &self.IRQ13())
            .field("IRQ14", &self.IRQ14())
            .field("IRQ15", &self.IRQ15())
            .field("RX_OK", &self.RX_OK())
            .field("RX_NOK", &self.RX_NOK())
            .field("RX_IGNORED", &self.RX_IGNORED())
            .field("RX_EMPTY", &self.RX_EMPTY())
            .field("RX_CTRL", &self.RX_CTRL())
            .field("RX_CTRL_ACK", &self.RX_CTRL_ACK())
            .field("RX_BUF_FULL", &self.RX_BUF_FULL())
            .field("RX_ENTRY_DONE", &self.RX_ENTRY_DONE())
            .field("RX_DATA_WRITTEN", &self.RX_DATA_WRITTEN())
            .field("RX_N_DATA_WRITTEN", &self.RX_N_DATA_WRITTEN())
            .field("RX_ABORTED", &self.RX_ABORTED())
            .field("IRQ27", &self.IRQ27())
            .field("SYNTH_NO_LOCK", &self.SYNTH_NO_LOCK())
            .field("MODULES_UNLOCKED", &self.MODULES_UNLOCKED())
            .field("BOOT_DONE", &self.BOOT_DONE())
            .field("INTERNAL_ERROR", &self.INTERNAL_ERROR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFCPEISL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFCPEISL {{ COMMAND_DONE: {:?}, LAST_COMMAND_DONE: {:?}, FG_COMMAND_DONE: {:?}, LAST_FG_COMMAND_DONE: {:?}, TX_DONE: {:?}, TX_ACK: {:?}, TX_CTRL: {:?}, TX_CTRL_ACK: {:?}, TX_CTRL_ACK_ACK: {:?}, TX_RETRANS: {:?}, TX_ENTRY_DONE: {:?}, TX_BUFFER_CHANGED: {:?}, IRQ12: {:?}, IRQ13: {:?}, IRQ14: {:?}, IRQ15: {:?}, RX_OK: {:?}, RX_NOK: {:?}, RX_IGNORED: {:?}, RX_EMPTY: {:?}, RX_CTRL: {:?}, RX_CTRL_ACK: {:?}, RX_BUF_FULL: {:?}, RX_ENTRY_DONE: {:?}, RX_DATA_WRITTEN: {:?}, RX_N_DATA_WRITTEN: {:?}, RX_ABORTED: {:?}, IRQ27: {:?}, SYNTH_NO_LOCK: {:?}, MODULES_UNLOCKED: {:?}, BOOT_DONE: {:?}, INTERNAL_ERROR: {:?} }}",
            self.COMMAND_DONE(),
            self.LAST_COMMAND_DONE(),
            self.FG_COMMAND_DONE(),
            self.LAST_FG_COMMAND_DONE(),
            self.TX_DONE(),
            self.TX_ACK(),
            self.TX_CTRL(),
            self.TX_CTRL_ACK(),
            self.TX_CTRL_ACK_ACK(),
            self.TX_RETRANS(),
            self.TX_ENTRY_DONE(),
            self.TX_BUFFER_CHANGED(),
            self.IRQ12(),
            self.IRQ13(),
            self.IRQ14(),
            self.IRQ15(),
            self.RX_OK(),
            self.RX_NOK(),
            self.RX_IGNORED(),
            self.RX_EMPTY(),
            self.RX_CTRL(),
            self.RX_CTRL_ACK(),
            self.RX_BUF_FULL(),
            self.RX_ENTRY_DONE(),
            self.RX_DATA_WRITTEN(),
            self.RX_N_DATA_WRITTEN(),
            self.RX_ABORTED(),
            self.IRQ27(),
            self.SYNTH_NO_LOCK(),
            self.MODULES_UNLOCKED(),
            self.BOOT_DONE(),
            self.INTERNAL_ERROR()
        )
    }
}
#[doc = "Interrupt Enable For RF Hardware Modules."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFHWIEN(pub u32);
impl RFHWIEN {
    #[doc = "0:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Interrupt enable for RFHWIFG.FSCA."]
    #[must_use]
    #[inline(always)]
    pub const fn FSCA(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Interrupt enable for RFHWIFG.FSCA."]
    #[inline(always)]
    pub const fn set_FSCA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Interrupt enable for RFHWIFG.MDMDONE."]
    #[must_use]
    #[inline(always)]
    pub const fn MDMDONE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Interrupt enable for RFHWIFG.MDMDONE."]
    #[inline(always)]
    pub const fn set_MDMDONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Interrupt enable for RFHWIFG.MDMIN."]
    #[must_use]
    #[inline(always)]
    pub const fn MDMIN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Interrupt enable for RFHWIFG.MDMIN."]
    #[inline(always)]
    pub const fn set_MDMIN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Interrupt enable for RFHWIFG.MDMOUT."]
    #[must_use]
    #[inline(always)]
    pub const fn MDMOUT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Interrupt enable for RFHWIFG.MDMOUT."]
    #[inline(always)]
    pub const fn set_MDMOUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Interrupt enable for RFHWIFG.MDMSOFT."]
    #[must_use]
    #[inline(always)]
    pub const fn MDMSOFT(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Interrupt enable for RFHWIFG.MDMSOFT."]
    #[inline(always)]
    pub const fn set_MDMSOFT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Interrupt enable for RFHWIFG.TRCTK."]
    #[must_use]
    #[inline(always)]
    pub const fn TRCTK(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Interrupt enable for RFHWIFG.TRCTK."]
    #[inline(always)]
    pub const fn set_TRCTK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Interrupt enable for RFHWIFG.RFEDONE."]
    #[must_use]
    #[inline(always)]
    pub const fn RFEDONE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Interrupt enable for RFHWIFG.RFEDONE."]
    #[inline(always)]
    pub const fn set_RFEDONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Interrupt enable for RFHWIFG.RFESOFT0."]
    #[must_use]
    #[inline(always)]
    pub const fn RFESOFT0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Interrupt enable for RFHWIFG.RFESOFT0."]
    #[inline(always)]
    pub const fn set_RFESOFT0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Interrupt enable for RFHWIFG.RFESOFT1."]
    #[must_use]
    #[inline(always)]
    pub const fn RFESOFT1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Interrupt enable for RFHWIFG.RFESOFT1."]
    #[inline(always)]
    pub const fn set_RFESOFT1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Interrupt enable for RFHWIFG.RFESOFT2."]
    #[must_use]
    #[inline(always)]
    pub const fn RFESOFT2(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Interrupt enable for RFHWIFG.RFESOFT2."]
    #[inline(always)]
    pub const fn set_RFESOFT2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Interrupt enable for RFHWIFG.RATCH0."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Interrupt enable for RFHWIFG.RATCH0."]
    #[inline(always)]
    pub const fn set_RATCH0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Interrupt enable for RFHWIFG.RATCH1."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Interrupt enable for RFHWIFG.RATCH1."]
    #[inline(always)]
    pub const fn set_RATCH1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Interrupt enable for RFHWIFG.RATCH2."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Interrupt enable for RFHWIFG.RATCH2."]
    #[inline(always)]
    pub const fn set_RATCH2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Interrupt enable for RFHWIFG.RATCH3."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Interrupt enable for RFHWIFG.RATCH3."]
    #[inline(always)]
    pub const fn set_RATCH3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Interrupt enable for RFHWIFG.RATCH4."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH4(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Interrupt enable for RFHWIFG.RATCH4."]
    #[inline(always)]
    pub const fn set_RATCH4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Interrupt enable for RFHWIFG.RATCH5."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH5(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Interrupt enable for RFHWIFG.RATCH5."]
    #[inline(always)]
    pub const fn set_RATCH5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Interrupt enable for RFHWIFG.RATCH6."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH6(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Interrupt enable for RFHWIFG.RATCH6."]
    #[inline(always)]
    pub const fn set_RATCH6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Interrupt enable for RFHWIFG.RATCH7."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH7(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Interrupt enable for RFHWIFG.RATCH7."]
    #[inline(always)]
    pub const fn set_RATCH7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "31:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED20(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "31:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED20(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for RFHWIEN {
    #[inline(always)]
    fn default() -> RFHWIEN {
        RFHWIEN(0)
    }
}
impl core::fmt::Debug for RFHWIEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFHWIEN")
            .field("RESERVED0", &self.RESERVED0())
            .field("FSCA", &self.FSCA())
            .field("MDMDONE", &self.MDMDONE())
            .field("MDMIN", &self.MDMIN())
            .field("MDMOUT", &self.MDMOUT())
            .field("MDMSOFT", &self.MDMSOFT())
            .field("TRCTK", &self.TRCTK())
            .field("RESERVED7", &self.RESERVED7())
            .field("RFEDONE", &self.RFEDONE())
            .field("RFESOFT0", &self.RFESOFT0())
            .field("RFESOFT1", &self.RFESOFT1())
            .field("RFESOFT2", &self.RFESOFT2())
            .field("RATCH0", &self.RATCH0())
            .field("RATCH1", &self.RATCH1())
            .field("RATCH2", &self.RATCH2())
            .field("RATCH3", &self.RATCH3())
            .field("RATCH4", &self.RATCH4())
            .field("RATCH5", &self.RATCH5())
            .field("RATCH6", &self.RATCH6())
            .field("RATCH7", &self.RATCH7())
            .field("RESERVED20", &self.RESERVED20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFHWIEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFHWIEN {{ RESERVED0: {=bool:?}, FSCA: {=bool:?}, MDMDONE: {=bool:?}, MDMIN: {=bool:?}, MDMOUT: {=bool:?}, MDMSOFT: {=bool:?}, TRCTK: {=bool:?}, RESERVED7: {=bool:?}, RFEDONE: {=bool:?}, RFESOFT0: {=bool:?}, RFESOFT1: {=bool:?}, RFESOFT2: {=bool:?}, RATCH0: {=bool:?}, RATCH1: {=bool:?}, RATCH2: {=bool:?}, RATCH3: {=bool:?}, RATCH4: {=bool:?}, RATCH5: {=bool:?}, RATCH6: {=bool:?}, RATCH7: {=bool:?}, RESERVED20: {=u16:?} }}",
            self.RESERVED0(),
            self.FSCA(),
            self.MDMDONE(),
            self.MDMIN(),
            self.MDMOUT(),
            self.MDMSOFT(),
            self.TRCTK(),
            self.RESERVED7(),
            self.RFEDONE(),
            self.RFESOFT0(),
            self.RFESOFT1(),
            self.RFESOFT2(),
            self.RATCH0(),
            self.RATCH1(),
            self.RATCH2(),
            self.RATCH3(),
            self.RATCH4(),
            self.RATCH5(),
            self.RATCH6(),
            self.RATCH7(),
            self.RESERVED20()
        )
    }
}
#[doc = "Interrupt Flags From RF Hardware Modules."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RFHWIFG(pub u32);
impl RFHWIFG {
    #[doc = "0:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Frequency synthesizer calibration accelerator interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn FSCA(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Frequency synthesizer calibration accelerator interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_FSCA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Modem command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn MDMDONE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Modem command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_MDMDONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Modem FIFO input interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn MDMIN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Modem FIFO input interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_MDMIN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Modem FIFO output interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn MDMOUT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Modem FIFO output interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_MDMOUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Modem synchronization word detection interrupt flag. This interrupt will be raised by modem when the synchronization word is received. The CPE may decide to reject the packet based on its header (protocol specific). Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn MDMSOFT(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Modem synchronization word detection interrupt flag. This interrupt will be raised by modem when the synchronization word is received. The CPE may decide to reject the packet based on its header (protocol specific). Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_MDMSOFT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Debug tracer system tick interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn TRCTK(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Debug tracer system tick interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_TRCTK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] RF engine command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RFEDONE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] RF engine command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RFEDONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] RF engine software defined interrupt 0 flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RFESOFT0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] RF engine software defined interrupt 0 flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RFESOFT0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] RF engine software defined interrupt 1 flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RFESOFT1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] RF engine software defined interrupt 1 flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RFESOFT1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] RF engine software defined interrupt 2 flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RFESOFT2(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] RF engine software defined interrupt 2 flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RFESOFT2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Radio timer channel 0 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Radio timer channel 0 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RATCH0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Radio timer channel 1 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Radio timer channel 1 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RATCH1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Radio timer channel 2 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Radio timer channel 2 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RATCH2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Radio timer channel 3 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Radio timer channel 3 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RATCH3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Radio timer channel 4 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH4(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Radio timer channel 4 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RATCH4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Radio timer channel 5 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH5(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Radio timer channel 5 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RATCH5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Radio timer channel 6 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH6(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Radio timer channel 6 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RATCH6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Radio timer channel 7 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RATCH7(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Radio timer channel 7 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub const fn set_RATCH7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "31:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED20(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "31:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED20(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for RFHWIFG {
    #[inline(always)]
    fn default() -> RFHWIFG {
        RFHWIFG(0)
    }
}
impl core::fmt::Debug for RFHWIFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFHWIFG")
            .field("RESERVED0", &self.RESERVED0())
            .field("FSCA", &self.FSCA())
            .field("MDMDONE", &self.MDMDONE())
            .field("MDMIN", &self.MDMIN())
            .field("MDMOUT", &self.MDMOUT())
            .field("MDMSOFT", &self.MDMSOFT())
            .field("TRCTK", &self.TRCTK())
            .field("RESERVED7", &self.RESERVED7())
            .field("RFEDONE", &self.RFEDONE())
            .field("RFESOFT0", &self.RFESOFT0())
            .field("RFESOFT1", &self.RFESOFT1())
            .field("RFESOFT2", &self.RFESOFT2())
            .field("RATCH0", &self.RATCH0())
            .field("RATCH1", &self.RATCH1())
            .field("RATCH2", &self.RATCH2())
            .field("RATCH3", &self.RATCH3())
            .field("RATCH4", &self.RATCH4())
            .field("RATCH5", &self.RATCH5())
            .field("RATCH6", &self.RATCH6())
            .field("RATCH7", &self.RATCH7())
            .field("RESERVED20", &self.RESERVED20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RFHWIFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RFHWIFG {{ RESERVED0: {=bool:?}, FSCA: {=bool:?}, MDMDONE: {=bool:?}, MDMIN: {=bool:?}, MDMOUT: {=bool:?}, MDMSOFT: {=bool:?}, TRCTK: {=bool:?}, RESERVED7: {=bool:?}, RFEDONE: {=bool:?}, RFESOFT0: {=bool:?}, RFESOFT1: {=bool:?}, RFESOFT2: {=bool:?}, RATCH0: {=bool:?}, RATCH1: {=bool:?}, RATCH2: {=bool:?}, RATCH3: {=bool:?}, RATCH4: {=bool:?}, RATCH5: {=bool:?}, RATCH6: {=bool:?}, RATCH7: {=bool:?}, RESERVED20: {=u16:?} }}",
            self.RESERVED0(),
            self.FSCA(),
            self.MDMDONE(),
            self.MDMIN(),
            self.MDMOUT(),
            self.MDMSOFT(),
            self.TRCTK(),
            self.RESERVED7(),
            self.RFEDONE(),
            self.RFESOFT0(),
            self.RFESOFT1(),
            self.RFESOFT2(),
            self.RATCH0(),
            self.RATCH1(),
            self.RATCH2(),
            self.RATCH3(),
            self.RATCH4(),
            self.RATCH5(),
            self.RATCH6(),
            self.RATCH7(),
            self.RESERVED20()
        )
    }
}
#[doc = "RF Core General Purpose Output Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYSGPOCTL(pub u32);
impl SYSGPOCTL {
    #[doc = "3:0\\] RF Core GPO control bit 0. Selects which signal to output on the RF Core GPO line 0."]
    #[must_use]
    #[inline(always)]
    pub const fn GPOCTL0(&self) -> super::vals::GPOCTL0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::GPOCTL0::from_bits(val as u8)
    }
    #[doc = "3:0\\] RF Core GPO control bit 0. Selects which signal to output on the RF Core GPO line 0."]
    #[inline(always)]
    pub const fn set_GPOCTL0(&mut self, val: super::vals::GPOCTL0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "7:4\\] RF Core GPO control bit 1. Selects which signal to output on the RF Core GPO line 1."]
    #[must_use]
    #[inline(always)]
    pub const fn GPOCTL1(&self) -> super::vals::GPOCTL1 {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::GPOCTL1::from_bits(val as u8)
    }
    #[doc = "7:4\\] RF Core GPO control bit 1. Selects which signal to output on the RF Core GPO line 1."]
    #[inline(always)]
    pub const fn set_GPOCTL1(&mut self, val: super::vals::GPOCTL1) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "11:8\\] RF Core GPO control bit 2. Selects which signal to output on the RF Core GPO line 2."]
    #[must_use]
    #[inline(always)]
    pub const fn GPOCTL2(&self) -> super::vals::GPOCTL2 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::GPOCTL2::from_bits(val as u8)
    }
    #[doc = "11:8\\] RF Core GPO control bit 2. Selects which signal to output on the RF Core GPO line 2."]
    #[inline(always)]
    pub const fn set_GPOCTL2(&mut self, val: super::vals::GPOCTL2) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "15:12\\] RF Core GPO control bit 3. Selects which signal to output on the RF Core GPO line 3."]
    #[must_use]
    #[inline(always)]
    pub const fn GPOCTL3(&self) -> super::vals::GPOCTL3 {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::GPOCTL3::from_bits(val as u8)
    }
    #[doc = "15:12\\] RF Core GPO control bit 3. Selects which signal to output on the RF Core GPO line 3."]
    #[inline(always)]
    pub const fn set_GPOCTL3(&mut self, val: super::vals::GPOCTL3) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED16(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for SYSGPOCTL {
    #[inline(always)]
    fn default() -> SYSGPOCTL {
        SYSGPOCTL(0)
    }
}
impl core::fmt::Debug for SYSGPOCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSGPOCTL")
            .field("GPOCTL0", &self.GPOCTL0())
            .field("GPOCTL1", &self.GPOCTL1())
            .field("GPOCTL2", &self.GPOCTL2())
            .field("GPOCTL3", &self.GPOCTL3())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SYSGPOCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SYSGPOCTL {{ GPOCTL0: {:?}, GPOCTL1: {:?}, GPOCTL2: {:?}, GPOCTL3: {:?}, RESERVED16: {=u16:?} }}",
            self.GPOCTL0(),
            self.GPOCTL1(),
            self.GPOCTL2(),
            self.GPOCTL3(),
            self.RESERVED16()
        )
    }
}

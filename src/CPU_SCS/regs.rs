#[doc = "Auxiliary Control This register is used to disable certain aspects of functionality within the processor."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ACTLR(pub u32);
impl ACTLR {
    #[doc = "0:0\\] Disables interruption of multi-cycle instructions. This increases the interrupt latency of the processor becuase LDM/STM completes before interrupt stacking occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn DISMCYCINT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Disables interruption of multi-cycle instructions. This increases the interrupt latency of the processor becuase LDM/STM completes before interrupt stacking occurs."]
    #[inline(always)]
    pub const fn set_DISMCYCINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Disables write buffer use during default memory map accesses. This causes all bus faults to be precise bus faults but decreases the performance of the processor because the stores to memory have to complete before the next instruction can be executed."]
    #[must_use]
    #[inline(always)]
    pub const fn DISDEFWBUF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Disables write buffer use during default memory map accesses. This causes all bus faults to be precise bus faults but decreases the performance of the processor because the stores to memory have to complete before the next instruction can be executed."]
    #[inline(always)]
    pub const fn set_DISDEFWBUF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Disables folding of IT instruction."]
    #[must_use]
    #[inline(always)]
    pub const fn DISFOLD(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Disables folding of IT instruction."]
    #[inline(always)]
    pub const fn set_DISFOLD(&mut self, val: bool) {
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
impl Default for ACTLR {
    #[inline(always)]
    fn default() -> ACTLR {
        ACTLR(0)
    }
}
impl core::fmt::Debug for ACTLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTLR")
            .field("DISMCYCINT", &self.DISMCYCINT())
            .field("DISDEFWBUF", &self.DISDEFWBUF())
            .field("DISFOLD", &self.DISFOLD())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ACTLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ACTLR {{ DISMCYCINT: {=bool:?}, DISDEFWBUF: {=bool:?}, DISFOLD: {=bool:?}, RESERVED3: {=u32:?} }}",
            self.DISMCYCINT(),
            self.DISDEFWBUF(),
            self.DISFOLD(),
            self.RESERVED3()
        )
    }
}
#[doc = "Auxiliary Fault Status This register is used to determine additional system fault information to software. Single-cycle high level on an auxiliary faults is latched as one. The bit can only be cleared by writing a one to the corresponding bit. Auxiliary fault inputs to the CPU are tied to 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AFSR(pub u32);
impl AFSR {
    #[doc = "31:0\\] Implementation defined. The bits map directly onto the signal assignment to the auxiliary fault inputs. Tied to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn IMPDEF(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Implementation defined. The bits map directly onto the signal assignment to the auxiliary fault inputs. Tied to 0."]
    #[inline(always)]
    pub const fn set_IMPDEF(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AFSR {
    #[inline(always)]
    fn default() -> AFSR {
        AFSR(0)
    }
}
impl core::fmt::Debug for AFSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFSR")
            .field("IMPDEF", &self.IMPDEF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AFSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AFSR {{ IMPDEF: {=u32:?} }}", self.IMPDEF())
    }
}
#[doc = "Application Interrupt/Reset Control This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AIRCR(pub u32);
impl AIRCR {
    #[doc = "0:0\\] System Reset bit. Resets the system, with the exception of debug components. This bit is reserved for debug use and can be written to 1 only when the core is halted. The bit self-clears. Writing this bit to 1 while core is not halted may result in unpredictable behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn VECTRESET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] System Reset bit. Resets the system, with the exception of debug components. This bit is reserved for debug use and can be written to 1 only when the core is halted. The bit self-clears. Writing this bit to 1 while core is not halted may result in unpredictable behavior."]
    #[inline(always)]
    pub const fn set_VECTRESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Clears all active state information for active NMI, fault, and interrupts. It is the responsibility of the application to reinitialize the stack. This bit is for returning to a known state during debug. The bit self-clears. IPSR is not cleared by this operation. So, if used by an application, it must only be used at the base level of activation, or within a system handler whose active bit can be set."]
    #[must_use]
    #[inline(always)]
    pub const fn VECTCLRACTIVE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Clears all active state information for active NMI, fault, and interrupts. It is the responsibility of the application to reinitialize the stack. This bit is for returning to a known state during debug. The bit self-clears. IPSR is not cleared by this operation. So, if used by an application, it must only be used at the base level of activation, or within a system handler whose active bit can be set."]
    #[inline(always)]
    pub const fn set_VECTCLRACTIVE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Requests a warm reset. Setting this bit does not prevent Halting Debug from running."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSRESETREQ(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Requests a warm reset. Setting this bit does not prevent Halting Debug from running."]
    #[inline(always)]
    pub const fn set_SYSRESETREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "7:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "7:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
    }
    #[doc = "10:8\\] Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIGROUP(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "10:8\\] Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
    #[inline(always)]
    pub const fn set_PRIGROUP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "14:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED11(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "14:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "15:15\\] Data endianness bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDIANESS(&self) -> super::vals::ENDIANESS {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::ENDIANESS::from_bits(val as u8)
    }
    #[doc = "15:15\\] Data endianness bit."]
    #[inline(always)]
    pub const fn set_ENDIANESS(&mut self, val: super::vals::ENDIANESS) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "31:16\\] Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
    #[must_use]
    #[inline(always)]
    pub const fn VECTKEY(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "31:16\\] Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
    #[inline(always)]
    pub const fn set_VECTKEY(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AIRCR {
    #[inline(always)]
    fn default() -> AIRCR {
        AIRCR(0)
    }
}
impl core::fmt::Debug for AIRCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIRCR")
            .field("VECTRESET", &self.VECTRESET())
            .field("VECTCLRACTIVE", &self.VECTCLRACTIVE())
            .field("SYSRESETREQ", &self.SYSRESETREQ())
            .field("RESERVED3", &self.RESERVED3())
            .field("PRIGROUP", &self.PRIGROUP())
            .field("RESERVED11", &self.RESERVED11())
            .field("ENDIANESS", &self.ENDIANESS())
            .field("VECTKEY", &self.VECTKEY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AIRCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AIRCR {{ VECTRESET: {=bool:?}, VECTCLRACTIVE: {=bool:?}, SYSRESETREQ: {=bool:?}, RESERVED3: {=u8:?}, PRIGROUP: {=u8:?}, RESERVED11: {=u8:?}, ENDIANESS: {:?}, VECTKEY: {=u16:?} }}",
            self.VECTRESET(),
            self.VECTCLRACTIVE(),
            self.SYSRESETREQ(),
            self.RESERVED3(),
            self.PRIGROUP(),
            self.RESERVED11(),
            self.ENDIANESS(),
            self.VECTKEY()
        )
    }
}
#[doc = "Bus Fault Address This register is used to read the address of the location that generated a Bus Fault."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BFAR(pub u32);
impl BFAR {
    #[doc = "31:0\\] Bus fault address field. This field is the data address of a faulted load or store attempt. When an unaligned access faults, the address is the address requested by the instruction, even if that is not the address that faulted. Flags CFSR.IBUSERR, CFSR.PRECISERR, CFSR.IMPRECISERR, CFSR.UNSTKERR and CFSR.STKERR in combination with CFSR.BFARVALID indicate the cause of the fault."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDRESS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Bus fault address field. This field is the data address of a faulted load or store attempt. When an unaligned access faults, the address is the address requested by the instruction, even if that is not the address that faulted. Flags CFSR.IBUSERR, CFSR.PRECISERR, CFSR.IMPRECISERR, CFSR.UNSTKERR and CFSR.STKERR in combination with CFSR.BFARVALID indicate the cause of the fault."]
    #[inline(always)]
    pub const fn set_ADDRESS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BFAR {
    #[inline(always)]
    fn default() -> BFAR {
        BFAR(0)
    }
}
impl core::fmt::Debug for BFAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BFAR")
            .field("ADDRESS", &self.ADDRESS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BFAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BFAR {{ ADDRESS: {=u32:?} }}", self.ADDRESS())
    }
}
#[doc = "Configuration Control This register is used to enable NMI, HardFault and FAULTMASK to ignore bus fault, trap divide by zero and unaligned accesses, enable user access to the Software Trigger Interrupt Register (STIR), control entry to Thread Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCR(pub u32);
impl CCR {
    #[doc = "0:0\\] Indicates how the processor enters Thread mode: 0: Processor can enter Thread mode only when no exception is active. 1: Processor can enter Thread mode from any level using the appropriate return value (EXC_RETURN). Exception returns occur when one of the following instructions loads a value of 0xFXXXXXXX into the PC while in Handler mode: - POP/LDM which includes loading the PC. - LDR with PC as a destination. - BX with any register. The value written to the PC is intercepted and is referred to as the EXC_RETURN value."]
    #[must_use]
    #[inline(always)]
    pub const fn NONBASETHREDENA(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Indicates how the processor enters Thread mode: 0: Processor can enter Thread mode only when no exception is active. 1: Processor can enter Thread mode from any level using the appropriate return value (EXC_RETURN). Exception returns occur when one of the following instructions loads a value of 0xFXXXXXXX into the PC while in Handler mode: - POP/LDM which includes loading the PC. - LDR with PC as a destination. - BX with any register. The value written to the PC is intercepted and is referred to as the EXC_RETURN value."]
    #[inline(always)]
    pub const fn set_NONBASETHREDENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Enables unprivileged software access to STIR: 0: User code is not allowed to write to the Software Trigger Interrupt register (STIR). 1: User code can write the Software Trigger Interrupt register (STIR) to trigger (pend) a Main exception, which is associated with the Main stack pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn USERSETMPEND(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Enables unprivileged software access to STIR: 0: User code is not allowed to write to the Software Trigger Interrupt register (STIR). 1: User code can write the Software Trigger Interrupt register (STIR) to trigger (pend) a Main exception, which is associated with the Main stack pointer."]
    #[inline(always)]
    pub const fn set_USERSETMPEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Enables unaligned access traps: 0: Do not trap unaligned halfword and word accesses 1: Trap unaligned halfword and word accesses. The relevant Usage Fault Status Register bit is CFSR.UNALIGNED. If this bit is set to 1, an unaligned access generates a UsageFault. Unaligned LDM, STM, LDRD, and STRD instructions always fault regardless of the value in UNALIGN_TRP."]
    #[must_use]
    #[inline(always)]
    pub const fn UNALIGN_TRP(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Enables unaligned access traps: 0: Do not trap unaligned halfword and word accesses 1: Trap unaligned halfword and word accesses. The relevant Usage Fault Status Register bit is CFSR.UNALIGNED. If this bit is set to 1, an unaligned access generates a UsageFault. Unaligned LDM, STM, LDRD, and STRD instructions always fault regardless of the value in UNALIGN_TRP."]
    #[inline(always)]
    pub const fn set_UNALIGN_TRP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0: 0: Do not trap divide by 0. In this mode, a divide by zero returns a quotient of 0. 1: Trap divide by 0. The relevant Usage Fault Status Register bit is CFSR.DIVBYZERO."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV_0_TRP(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0: 0: Do not trap divide by 0. In this mode, a divide by zero returns a quotient of 0. 1: Trap divide by 0. The relevant Usage Fault Status Register bit is CFSR.DIVBYZERO."]
    #[inline(always)]
    pub const fn set_DIV_0_TRP(&mut self, val: bool) {
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
    #[doc = "8:8\\] Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the HardFault, NMI, and FAULTMASK escalated handlers: 0: Data BusFaults caused by load and store instructions cause a lock-up 1: Data BusFaults caused by load and store instructions are ignored. Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use of this bit is to probe system devices and bridges to detect problems."]
    #[must_use]
    #[inline(always)]
    pub const fn BFHFNMIGN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the HardFault, NMI, and FAULTMASK escalated handlers: 0: Data BusFaults caused by load and store instructions cause a lock-up 1: Data BusFaults caused by load and store instructions are ignored. Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use of this bit is to probe system devices and bridges to detect problems."]
    #[inline(always)]
    pub const fn set_BFHFNMIGN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Stack alignment bit. 0: Only 4-byte alignment is guaranteed for the SP used prior to the exception on exception entry. 1: On exception entry, the SP used prior to the exception is adjusted to be 8-byte aligned and the context to restore it is saved. The SP is restored on the associated exception return."]
    #[must_use]
    #[inline(always)]
    pub const fn STKALIGN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Stack alignment bit. 0: Only 4-byte alignment is guaranteed for the SP used prior to the exception on exception entry. 1: On exception entry, the SP used prior to the exception is adjusted to be 8-byte aligned and the context to restore it is saved. The SP is restored on the associated exception return."]
    #[inline(always)]
    pub const fn set_STKALIGN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "31:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED10(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "31:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED10(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for CCR {
    #[inline(always)]
    fn default() -> CCR {
        CCR(0)
    }
}
impl core::fmt::Debug for CCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("NONBASETHREDENA", &self.NONBASETHREDENA())
            .field("USERSETMPEND", &self.USERSETMPEND())
            .field("RESERVED2", &self.RESERVED2())
            .field("UNALIGN_TRP", &self.UNALIGN_TRP())
            .field("DIV_0_TRP", &self.DIV_0_TRP())
            .field("RESERVED5", &self.RESERVED5())
            .field("BFHFNMIGN", &self.BFHFNMIGN())
            .field("STKALIGN", &self.STKALIGN())
            .field("RESERVED10", &self.RESERVED10())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCR {{ NONBASETHREDENA: {=bool:?}, USERSETMPEND: {=bool:?}, RESERVED2: {=bool:?}, UNALIGN_TRP: {=bool:?}, DIV_0_TRP: {=bool:?}, RESERVED5: {=u8:?}, BFHFNMIGN: {=bool:?}, STKALIGN: {=bool:?}, RESERVED10: {=u32:?} }}",
            self.NONBASETHREDENA(),
            self.USERSETMPEND(),
            self.RESERVED2(),
            self.UNALIGN_TRP(),
            self.DIV_0_TRP(),
            self.RESERVED5(),
            self.BFHFNMIGN(),
            self.STKALIGN(),
            self.RESERVED10()
        )
    }
}
#[doc = "Configurable Fault Status This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit. The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows: The following accesses are possible to the CFSR register: - access the complete register with a word access to 0xE000ED28. - access the MMFSR with a byte access to 0xE000ED28 - access the MMFSR and BFSR with a halfword access to 0xE000ED28 - access the BFSR with a byte access to 0xE000ED29 - access the UFSR with a halfword access to 0xE000ED2A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFSR(pub u32);
impl CFSR {
    #[doc = "0:0\\] Instruction access violation flag. Attempting to fetch an instruction from a location that does not permit execution sets this flag. This occurs on any access to an XN region, even when the MPU is disabled or not present. The return PC points to the faulting instruction. MMFAR is not written."]
    #[must_use]
    #[inline(always)]
    pub const fn IACCVIOL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Instruction access violation flag. Attempting to fetch an instruction from a location that does not permit execution sets this flag. This occurs on any access to an XN region, even when the MPU is disabled or not present. The return PC points to the faulting instruction. MMFAR is not written."]
    #[inline(always)]
    pub const fn set_IACCVIOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Data access violation flag. Attempting to load or store at a location that does not permit the operation sets this flag. The return PC points to the faulting instruction. This error loads MMFAR with the address of the attempted access."]
    #[must_use]
    #[inline(always)]
    pub const fn DACCVIOL(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Data access violation flag. Attempting to load or store at a location that does not permit the operation sets this flag. The return PC points to the faulting instruction. This error loads MMFAR with the address of the attempted access."]
    #[inline(always)]
    pub const fn set_DACCVIOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Unstack from exception return has caused one or more access violations. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. MMFAR is not written."]
    #[must_use]
    #[inline(always)]
    pub const fn MUNSTKERR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Unstack from exception return has caused one or more access violations. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. MMFAR is not written."]
    #[inline(always)]
    pub const fn set_MUNSTKERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Stacking from exception has caused one or more access violations. The SP is still adjusted and the values in the context area on the stack might be incorrect. MMFAR is not written."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTKERR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Stacking from exception has caused one or more access violations. The SP is still adjusted and the values in the context area on the stack might be incorrect. MMFAR is not written."]
    #[inline(always)]
    pub const fn set_MSTKERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "6:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "6:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "7:7\\] Memory Manage Address Register (MMFAR) address valid flag. A later-arriving fault, such as a bus fault, can clear a memory manage fault.. If a MemManage fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems on return to a stacked active MemManage handler whose MMFAR value has been overwritten."]
    #[must_use]
    #[inline(always)]
    pub const fn MMARVALID(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Memory Manage Address Register (MMFAR) address valid flag. A later-arriving fault, such as a bus fault, can clear a memory manage fault.. If a MemManage fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems on return to a stacked active MemManage handler whose MMFAR value has been overwritten."]
    #[inline(always)]
    pub const fn set_MMARVALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Instruction bus error flag. This flag is set by a prefetch error. The fault stops on the instruction, so if the error occurs under a branch shadow, no fault occurs. BFAR is not written."]
    #[must_use]
    #[inline(always)]
    pub const fn IBUSERR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Instruction bus error flag. This flag is set by a prefetch error. The fault stops on the instruction, so if the error occurs under a branch shadow, no fault occurs. BFAR is not written."]
    #[inline(always)]
    pub const fn set_IBUSERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Precise data bus error return."]
    #[must_use]
    #[inline(always)]
    pub const fn PRECISERR(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Precise data bus error return."]
    #[inline(always)]
    pub const fn set_PRECISERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Imprecise data bus error. It is a BusFault, but the Return PC is not related to the causing instruction. This is not a synchronous fault. So, if detected when the priority of the current activation is higher than the Bus Fault, it only pends. Bus fault activates when returning to a lower priority activation. If a precise fault occurs before returning to a lower priority exception, the handler detects both IMPRECISERR set and one of the precise fault status bits set at the same time. BFAR is not written."]
    #[must_use]
    #[inline(always)]
    pub const fn IMPRECISERR(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Imprecise data bus error. It is a BusFault, but the Return PC is not related to the causing instruction. This is not a synchronous fault. So, if detected when the priority of the current activation is higher than the Bus Fault, it only pends. Bus fault activates when returning to a lower priority activation. If a precise fault occurs before returning to a lower priority exception, the handler detects both IMPRECISERR set and one of the precise fault status bits set at the same time. BFAR is not written."]
    #[inline(always)]
    pub const fn set_IMPRECISERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Unstack from exception return has caused one or more bus faults. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. BFAR is not written."]
    #[must_use]
    #[inline(always)]
    pub const fn UNSTKERR(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Unstack from exception return has caused one or more bus faults. This is chained to the handler, so that the original return stack is still present. SP is not adjusted from failing return and new save is not performed. BFAR is not written."]
    #[inline(always)]
    pub const fn set_UNSTKERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Stacking from exception has caused one or more bus faults. The SP is still adjusted and the values in the context area on the stack might be incorrect. BFAR is not written."]
    #[must_use]
    #[inline(always)]
    pub const fn STKERR(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Stacking from exception has caused one or more bus faults. The SP is still adjusted and the values in the context area on the stack might be incorrect. BFAR is not written."]
    #[inline(always)]
    pub const fn set_STKERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "14:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED13(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "14:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "15:15\\] This bit is set if the Bus Fault Address Register (BFAR) contains a valid address. This is true after a bus fault where the address is known. Other faults can clear this bit, such as a Mem Manage fault occurring later. If a Bus fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems if returning to a stacked active Bus fault handler whose BFAR value has been overwritten."]
    #[must_use]
    #[inline(always)]
    pub const fn BFARVALID(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] This bit is set if the Bus Fault Address Register (BFAR) contains a valid address. This is true after a bus fault where the address is known. Other faults can clear this bit, such as a Mem Manage fault occurring later. If a Bus fault occurs that is escalated to a Hard Fault because of priority, the Hard Fault handler must clear this bit. This prevents problems if returning to a stacked active Bus fault handler whose BFAR value has been overwritten."]
    #[inline(always)]
    pub const fn set_BFARVALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] This bit is set when the processor attempts to execute an undefined instruction. This is an instruction that the processor cannot decode. The return PC points to the undefined instruction."]
    #[must_use]
    #[inline(always)]
    pub const fn UNDEFINSTR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] This bit is set when the processor attempts to execute an undefined instruction. This is an instruction that the processor cannot decode. The return PC points to the undefined instruction."]
    #[inline(always)]
    pub const fn set_UNDEFINSTR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Indicates an attempt to execute in an invalid EPSR state (e.g. after a BX type instruction has changed state). This includes state change after entry to or return from exception, as well as from inter-working instructions. Return PC points to faulting instruction, with the invalid state."]
    #[must_use]
    #[inline(always)]
    pub const fn INVSTATE(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Indicates an attempt to execute in an invalid EPSR state (e.g. after a BX type instruction has changed state). This includes state change after entry to or return from exception, as well as from inter-working instructions. Return PC points to faulting instruction, with the invalid state."]
    #[inline(always)]
    pub const fn set_INVSTATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Attempt to load EXC_RETURN into PC illegally. Invalid instruction, invalid context, invalid value. The return PC points to the instruction that tried to set the PC."]
    #[must_use]
    #[inline(always)]
    pub const fn INVPC(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Attempt to load EXC_RETURN into PC illegally. Invalid instruction, invalid context, invalid value. The return PC points to the instruction that tried to set the PC."]
    #[inline(always)]
    pub const fn set_INVPC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Attempt to use a coprocessor instruction. The processor does not support coprocessor instructions."]
    #[must_use]
    #[inline(always)]
    pub const fn NOCP(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Attempt to use a coprocessor instruction. The processor does not support coprocessor instructions."]
    #[inline(always)]
    pub const fn set_NOCP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "23:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED20(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED20(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "24:24\\] When CCR.UNALIGN_TRP is enabled, and there is an attempt to make an unaligned memory access, then this fault occurs. Unaligned LDM/STM/LDRD/STRD instructions always fault irrespective of the setting of CCR.UNALIGN_TRP."]
    #[must_use]
    #[inline(always)]
    pub const fn UNALIGNED(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] When CCR.UNALIGN_TRP is enabled, and there is an attempt to make an unaligned memory access, then this fault occurs. Unaligned LDM/STM/LDRD/STRD instructions always fault irrespective of the setting of CCR.UNALIGN_TRP."]
    #[inline(always)]
    pub const fn set_UNALIGNED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] When CCR.DIV_0_TRP (see Configuration Control Register on page 8-26) is enabled and an SDIV or UDIV instruction is used with a divisor of 0, this fault occurs The instruction is executed and the return PC points to it. If CCR.DIV_0_TRP is not set, then the divide returns a quotient of 0."]
    #[must_use]
    #[inline(always)]
    pub const fn DIVBYZERO(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] When CCR.DIV_0_TRP (see Configuration Control Register on page 8-26) is enabled and an SDIV or UDIV instruction is used with a divisor of 0, this fault occurs The instruction is executed and the return PC points to it. If CCR.DIV_0_TRP is not set, then the divide returns a quotient of 0."]
    #[inline(always)]
    pub const fn set_DIVBYZERO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "31:26\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED26(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "31:26\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED26(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for CFSR {
    #[inline(always)]
    fn default() -> CFSR {
        CFSR(0)
    }
}
impl core::fmt::Debug for CFSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFSR")
            .field("IACCVIOL", &self.IACCVIOL())
            .field("DACCVIOL", &self.DACCVIOL())
            .field("RESERVED2", &self.RESERVED2())
            .field("MUNSTKERR", &self.MUNSTKERR())
            .field("MSTKERR", &self.MSTKERR())
            .field("RESERVED5", &self.RESERVED5())
            .field("MMARVALID", &self.MMARVALID())
            .field("IBUSERR", &self.IBUSERR())
            .field("PRECISERR", &self.PRECISERR())
            .field("IMPRECISERR", &self.IMPRECISERR())
            .field("UNSTKERR", &self.UNSTKERR())
            .field("STKERR", &self.STKERR())
            .field("RESERVED13", &self.RESERVED13())
            .field("BFARVALID", &self.BFARVALID())
            .field("UNDEFINSTR", &self.UNDEFINSTR())
            .field("INVSTATE", &self.INVSTATE())
            .field("INVPC", &self.INVPC())
            .field("NOCP", &self.NOCP())
            .field("RESERVED20", &self.RESERVED20())
            .field("UNALIGNED", &self.UNALIGNED())
            .field("DIVBYZERO", &self.DIVBYZERO())
            .field("RESERVED26", &self.RESERVED26())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFSR {{ IACCVIOL: {=bool:?}, DACCVIOL: {=bool:?}, RESERVED2: {=bool:?}, MUNSTKERR: {=bool:?}, MSTKERR: {=bool:?}, RESERVED5: {=u8:?}, MMARVALID: {=bool:?}, IBUSERR: {=bool:?}, PRECISERR: {=bool:?}, IMPRECISERR: {=bool:?}, UNSTKERR: {=bool:?}, STKERR: {=bool:?}, RESERVED13: {=u8:?}, BFARVALID: {=bool:?}, UNDEFINSTR: {=bool:?}, INVSTATE: {=bool:?}, INVPC: {=bool:?}, NOCP: {=bool:?}, RESERVED20: {=u8:?}, UNALIGNED: {=bool:?}, DIVBYZERO: {=bool:?}, RESERVED26: {=u8:?} }}",
            self.IACCVIOL(),
            self.DACCVIOL(),
            self.RESERVED2(),
            self.MUNSTKERR(),
            self.MSTKERR(),
            self.RESERVED5(),
            self.MMARVALID(),
            self.IBUSERR(),
            self.PRECISERR(),
            self.IMPRECISERR(),
            self.UNSTKERR(),
            self.STKERR(),
            self.RESERVED13(),
            self.BFARVALID(),
            self.UNDEFINSTR(),
            self.INVSTATE(),
            self.INVPC(),
            self.NOCP(),
            self.RESERVED20(),
            self.UNALIGNED(),
            self.DIVBYZERO(),
            self.RESERVED26()
        )
    }
}
#[doc = "Coprocessor Access Control This register specifies the access privileges for coprocessors."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPACR(pub u32);
impl CPACR {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CPACR {
    #[inline(always)]
    fn default() -> CPACR {
        CPACR(0)
    }
}
impl core::fmt::Debug for CPACR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPACR")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPACR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CPACR {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPUID(pub u32);
impl CPUID {
    #[doc = "3:0\\] Implementation defined revision number."]
    #[must_use]
    #[inline(always)]
    pub const fn REVISION(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] Implementation defined revision number."]
    #[inline(always)]
    pub const fn set_REVISION(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "15:4\\] Number of processor within family."]
    #[must_use]
    #[inline(always)]
    pub const fn PARTNO(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "15:4\\] Number of processor within family."]
    #[inline(always)]
    pub const fn set_PARTNO(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "19:16\\] Reads as 0xF."]
    #[must_use]
    #[inline(always)]
    pub const fn CONSTANT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Reads as 0xF."]
    #[inline(always)]
    pub const fn set_CONSTANT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "23:20\\] Implementation defined variant number."]
    #[must_use]
    #[inline(always)]
    pub const fn VARIANT(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Implementation defined variant number."]
    #[inline(always)]
    pub const fn set_VARIANT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "31:24\\] Implementor code."]
    #[must_use]
    #[inline(always)]
    pub const fn IMPLEMENTER(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Implementor code."]
    #[inline(always)]
    pub const fn set_IMPLEMENTER(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for CPUID {
    #[inline(always)]
    fn default() -> CPUID {
        CPUID(0)
    }
}
impl core::fmt::Debug for CPUID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUID")
            .field("REVISION", &self.REVISION())
            .field("PARTNO", &self.PARTNO())
            .field("CONSTANT", &self.CONSTANT())
            .field("VARIANT", &self.VARIANT())
            .field("IMPLEMENTER", &self.IMPLEMENTER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPUID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPUID {{ REVISION: {=u8:?}, PARTNO: {=u16:?}, CONSTANT: {=u8:?}, VARIANT: {=u8:?}, IMPLEMENTER: {=u8:?} }}",
            self.REVISION(),
            self.PARTNO(),
            self.CONSTANT(),
            self.VARIANT(),
            self.IMPLEMENTER()
        )
    }
}
#[doc = "Debug Core Register Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCRDR(pub u32);
impl DCRDR {
    #[doc = "31:0\\] This register holds data for reading and writing registers to and from the processor. This is the data value written to the register selected by DCRSR. When the processor receives a request from DCRSR, this register is read or written by the processor using a normal load-store unit operation. If core register transfers are not being performed, software-based debug monitors can use this register for communication in non-halting debug. This enables flags and bits to acknowledge state and indicate if commands have been accepted to, replied to, or accepted and replied to."]
    #[must_use]
    #[inline(always)]
    pub const fn DCRDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] This register holds data for reading and writing registers to and from the processor. This is the data value written to the register selected by DCRSR. When the processor receives a request from DCRSR, this register is read or written by the processor using a normal load-store unit operation. If core register transfers are not being performed, software-based debug monitors can use this register for communication in non-halting debug. This enables flags and bits to acknowledge state and indicate if commands have been accepted to, replied to, or accepted and replied to."]
    #[inline(always)]
    pub const fn set_DCRDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DCRDR {
    #[inline(always)]
    fn default() -> DCRDR {
        DCRDR(0)
    }
}
impl core::fmt::Debug for DCRDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCRDR")
            .field("DCRDR", &self.DCRDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCRDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DCRDR {{ DCRDR: {=u32:?} }}", self.DCRDR())
    }
}
#[doc = "Deubg Core Register Selector The purpose of this register is to select the processor register to transfer data to or from. This write-only register generates a handshake to the core to transfer data to or from Debug Core Register Data Register and the selected register. Until this core transaction is complete, DHCSR.S_REGRDY is 0. Note that writes to this register in any size but word are Unpredictable. Note that PSR registers are fully accessible this way, whereas some read as 0 when using MRS instructions. Note that all bits can be written, but some combinations cause a fault when execution is resumed."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCRSR(pub u32);
impl DCRSR {
    #[doc = "4:0\\] Register select 0x00: R0 0x01: R1 0x02: R2 0x03: R3 0x04: R4 0x05: R5 0x06: R6 0x07: R7 0x08: R8 0x09: R9 0x0A: R10 0x0B: R11 0x0C: R12 0x0D: Current SP 0x0E: LR 0x0F: DebugReturnAddress 0x10: XPSR/flags, execution state information, and exception number 0x11: MSP (Main SP) 0x12: PSP (Process SP) 0x14: CONTROL<<24 | FAULTMASK<<16 | BASEPRI<<8 | PRIMASK."]
    #[must_use]
    #[inline(always)]
    pub const fn REGSEL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "4:0\\] Register select 0x00: R0 0x01: R1 0x02: R2 0x03: R3 0x04: R4 0x05: R5 0x06: R6 0x07: R7 0x08: R8 0x09: R9 0x0A: R10 0x0B: R11 0x0C: R12 0x0D: Current SP 0x0E: LR 0x0F: DebugReturnAddress 0x10: XPSR/flags, execution state information, and exception number 0x11: MSP (Main SP) 0x12: PSP (Process SP) 0x14: CONTROL<<24 | FAULTMASK<<16 | BASEPRI<<8 | PRIMASK."]
    #[inline(always)]
    pub const fn set_REGSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "15:5\\] Software should not rely on the value of a reserved. Write 0."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u16 {
        let val = (self.0 >> 5usize) & 0x07ff;
        val as u16
    }
    #[doc = "15:5\\] Software should not rely on the value of a reserved. Write 0."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 5usize)) | (((val as u32) & 0x07ff) << 5usize);
    }
    #[doc = "16:16\\] 1: Write 0: Read."]
    #[must_use]
    #[inline(always)]
    pub const fn REGWNR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] 1: Write 0: Read."]
    #[inline(always)]
    pub const fn set_REGWNR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "31:17\\] Software should not rely on the value of a reserved. Write 0."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "31:17\\] Software should not rely on the value of a reserved. Write 0."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for DCRSR {
    #[inline(always)]
    fn default() -> DCRSR {
        DCRSR(0)
    }
}
impl core::fmt::Debug for DCRSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCRSR")
            .field("REGSEL", &self.REGSEL())
            .field("RESERVED5", &self.RESERVED5())
            .field("REGWNR", &self.REGWNR())
            .field("RESERVED17", &self.RESERVED17())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCRSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCRSR {{ REGSEL: {=u8:?}, RESERVED5: {=u16:?}, REGWNR: {=bool:?}, RESERVED17: {=u16:?} }}",
            self.REGSEL(),
            self.RESERVED5(),
            self.REGWNR(),
            self.RESERVED17()
        )
    }
}
#[doc = "Debug Exception and Monitor Control The purpose of this register is vector catching and debug monitor control. This register manages exception behavior under debug. Vector catching is only available to halting debug. The upper halfword is for monitor controls and the lower halfword is for halting exception support. This register is not reset on a system reset. This register is reset by a power-on reset. The fields MON_EN, MON_PEND, MON_STEP and MON_REQ are always cleared on a core reset. The debug monitor is enabled by software in the reset handler or later, or by the **AHB-AP** port. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during a vector read or stack push error the halt occurs on the corresponding fault handler for the vector error or stack push. 2. If a late arriving interrupt detected during a vector read or stack push error it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEMCR(pub u32);
impl DEMCR {
    #[doc = "0:0\\] Reset Vector Catch. Halt running system if Core reset occurs. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn VC_CORERESET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Reset Vector Catch. Halt running system if Core reset occurs. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub const fn set_VC_CORERESET(&mut self, val: bool) {
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
    #[doc = "4:4\\] Debug trap on Memory Management faults. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn VC_MMERR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Debug trap on Memory Management faults. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub const fn set_VC_MMERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Debug trap on a UsageFault access to a Coprocessor. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn VC_NOCPERR(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Debug trap on a UsageFault access to a Coprocessor. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub const fn set_VC_NOCPERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Debug trap on Usage Fault enabled checking errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn VC_CHKERR(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Debug trap on Usage Fault enabled checking errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub const fn set_VC_CHKERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Debug trap on Usage Fault state errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn VC_STATERR(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Debug trap on Usage Fault state errors. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub const fn set_VC_STATERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Debug Trap on normal Bus error. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn VC_BUSERR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Debug Trap on normal Bus error. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub const fn set_VC_BUSERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Debug trap on a fault occurring during an exception entry or return sequence. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn VC_INTERR(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Debug trap on a fault occurring during an exception entry or return sequence. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub const fn set_VC_INTERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Debug trap on Hard Fault. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn VC_HARDERR(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Debug trap on Hard Fault. Ignored when DHCSR.C_DEBUGEN is cleared."]
    #[inline(always)]
    pub const fn set_VC_HARDERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "15:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED11(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "15:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
    #[doc = "16:16\\] Enable the debug monitor. When enabled, the System handler priority register controls its priority level. If disabled, then all debug events go to Hard fault. DHCSR.C_DEBUGEN overrides this bit. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during vectoring, vector read or stack push error, the halt occurs on the corresponding fault handler, for the vector error or stack push. 2. If a late arriving interrupt comes in during vectoring, it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
    #[must_use]
    #[inline(always)]
    pub const fn MON_EN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Enable the debug monitor. When enabled, the System handler priority register controls its priority level. If disabled, then all debug events go to Hard fault. DHCSR.C_DEBUGEN overrides this bit. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during vectoring, vector read or stack push error, the halt occurs on the corresponding fault handler, for the vector error or stack push. 2. If a late arriving interrupt comes in during vectoring, it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
    #[inline(always)]
    pub const fn set_MON_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Pend the monitor to activate when priority permits. This can wake up the monitor through the AHB-AP port. It is the equivalent to DHCSR.C_HALT for Monitor debug. This register does not reset on a system reset. It is only reset by a power-on reset. Software in the reset handler or later, or by the DAP must enable the debug monitor."]
    #[must_use]
    #[inline(always)]
    pub const fn MON_PEND(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Pend the monitor to activate when priority permits. This can wake up the monitor through the AHB-AP port. It is the equivalent to DHCSR.C_HALT for Monitor debug. This register does not reset on a system reset. It is only reset by a power-on reset. Software in the reset handler or later, or by the DAP must enable the debug monitor."]
    #[inline(always)]
    pub const fn set_MON_PEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] When MON_EN = 1, this steps the core. When MON_EN = 0, this bit is ignored. This is the equivalent to DHCSR.C_STEP. Interrupts are only stepped according to the priority of the monitor and settings of PRIMASK, FAULTMASK, or BASEPRI."]
    #[must_use]
    #[inline(always)]
    pub const fn MON_STEP(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] When MON_EN = 1, this steps the core. When MON_EN = 0, this bit is ignored. This is the equivalent to DHCSR.C_STEP. Interrupts are only stepped according to the priority of the monitor and settings of PRIMASK, FAULTMASK, or BASEPRI."]
    #[inline(always)]
    pub const fn set_MON_STEP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] This enables the monitor to identify how it wakes up. This bit clears on a Core Reset. 0x0: Woken up by debug exception. 0x1: Woken up by MON_PEND."]
    #[must_use]
    #[inline(always)]
    pub const fn MON_REQ(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] This enables the monitor to identify how it wakes up. This bit clears on a Core Reset. 0x0: Woken up by debug exception. 0x1: Woken up by MON_PEND."]
    #[inline(always)]
    pub const fn set_MON_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "23:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED20(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED20(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "24:24\\] This bit must be set to 1 to enable use of the trace and debug blocks: DWT, ITM, ETM and TPIU. This enables control of power usage unless tracing is required. The application can enable this, for ITM use, or use by a debugger."]
    #[must_use]
    #[inline(always)]
    pub const fn TRCENA(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] This bit must be set to 1 to enable use of the trace and debug blocks: DWT, ITM, ETM and TPIU. This enables control of power usage unless tracing is required. The application can enable this, for ITM use, or use by a debugger."]
    #[inline(always)]
    pub const fn set_TRCENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED25(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for DEMCR {
    #[inline(always)]
    fn default() -> DEMCR {
        DEMCR(0)
    }
}
impl core::fmt::Debug for DEMCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEMCR")
            .field("VC_CORERESET", &self.VC_CORERESET())
            .field("RESERVED1", &self.RESERVED1())
            .field("VC_MMERR", &self.VC_MMERR())
            .field("VC_NOCPERR", &self.VC_NOCPERR())
            .field("VC_CHKERR", &self.VC_CHKERR())
            .field("VC_STATERR", &self.VC_STATERR())
            .field("VC_BUSERR", &self.VC_BUSERR())
            .field("VC_INTERR", &self.VC_INTERR())
            .field("VC_HARDERR", &self.VC_HARDERR())
            .field("RESERVED11", &self.RESERVED11())
            .field("MON_EN", &self.MON_EN())
            .field("MON_PEND", &self.MON_PEND())
            .field("MON_STEP", &self.MON_STEP())
            .field("MON_REQ", &self.MON_REQ())
            .field("RESERVED20", &self.RESERVED20())
            .field("TRCENA", &self.TRCENA())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DEMCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DEMCR {{ VC_CORERESET: {=bool:?}, RESERVED1: {=u8:?}, VC_MMERR: {=bool:?}, VC_NOCPERR: {=bool:?}, VC_CHKERR: {=bool:?}, VC_STATERR: {=bool:?}, VC_BUSERR: {=bool:?}, VC_INTERR: {=bool:?}, VC_HARDERR: {=bool:?}, RESERVED11: {=u8:?}, MON_EN: {=bool:?}, MON_PEND: {=bool:?}, MON_STEP: {=bool:?}, MON_REQ: {=bool:?}, RESERVED20: {=u8:?}, TRCENA: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.VC_CORERESET(),
            self.RESERVED1(),
            self.VC_MMERR(),
            self.VC_NOCPERR(),
            self.VC_CHKERR(),
            self.VC_STATERR(),
            self.VC_BUSERR(),
            self.VC_INTERR(),
            self.VC_HARDERR(),
            self.RESERVED11(),
            self.MON_EN(),
            self.MON_PEND(),
            self.MON_STEP(),
            self.MON_REQ(),
            self.RESERVED20(),
            self.TRCENA(),
            self.RESERVED25()
        )
    }
}
#[doc = "Debug Fault Status This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DFSR(pub u32);
impl DFSR {
    #[doc = "0:0\\] Halt request flag. The processor is halted on the next instruction. 0x0: No halt request 0x1: Halt requested by NVIC, including step."]
    #[must_use]
    #[inline(always)]
    pub const fn HALTED(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Halt request flag. The processor is halted on the next instruction. 0x0: No halt request 0x1: Halt requested by NVIC, including step."]
    #[inline(always)]
    pub const fn set_HALTED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] BKPT flag. The BKPT flag is set by a BKPT instruction in flash patch code, and also by normal code. Return PC points to breakpoint containing instruction. 0x0: No BKPT instruction execution 0x1: BKPT instruction execution."]
    #[must_use]
    #[inline(always)]
    pub const fn BKPT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] BKPT flag. The BKPT flag is set by a BKPT instruction in flash patch code, and also by normal code. Return PC points to breakpoint containing instruction. 0x0: No BKPT instruction execution 0x1: BKPT instruction execution."]
    #[inline(always)]
    pub const fn set_BKPT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Data Watchpoint and Trace (DWT) flag. The processor stops at the current instruction or at the next instruction. 0x0: No DWT match 0x1: DWT match."]
    #[must_use]
    #[inline(always)]
    pub const fn DWTTRAP(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Data Watchpoint and Trace (DWT) flag. The processor stops at the current instruction or at the next instruction. 0x0: No DWT match 0x1: DWT match."]
    #[inline(always)]
    pub const fn set_DWTTRAP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Vector catch flag. When this flag is set, a flag in one of the local fault status registers is also set to indicate the type of fault. 0x0: No vector catch occurred 0x1: Vector catch occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn VCATCH(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Vector catch flag. When this flag is set, a flag in one of the local fault status registers is also set to indicate the type of fault. 0x0: No vector catch occurred 0x1: Vector catch occurred."]
    #[inline(always)]
    pub const fn set_VCATCH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] External debug request flag. The processor stops on next instruction boundary. 0x0: External debug request signal not asserted 0x1: External debug request signal asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn EXTERNAL(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] External debug request flag. The processor stops on next instruction boundary. 0x0: External debug request signal not asserted 0x1: External debug request signal asserted."]
    #[inline(always)]
    pub const fn set_EXTERNAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for DFSR {
    #[inline(always)]
    fn default() -> DFSR {
        DFSR(0)
    }
}
impl core::fmt::Debug for DFSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFSR")
            .field("HALTED", &self.HALTED())
            .field("BKPT", &self.BKPT())
            .field("DWTTRAP", &self.DWTTRAP())
            .field("VCATCH", &self.VCATCH())
            .field("EXTERNAL", &self.EXTERNAL())
            .field("RESERVED5", &self.RESERVED5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DFSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DFSR {{ HALTED: {=bool:?}, BKPT: {=bool:?}, DWTTRAP: {=bool:?}, VCATCH: {=bool:?}, EXTERNAL: {=bool:?}, RESERVED5: {=u32:?} }}",
            self.HALTED(),
            self.BKPT(),
            self.DWTTRAP(),
            self.VCATCH(),
            self.EXTERNAL(),
            self.RESERVED5()
        )
    }
}
#[doc = "Debug Halting Control and Status The purpose of this register is to provide status information about the state of the processor, enable core debug, halt and step the processor. For writes, 0xA05F must be written to higher half-word of this register, otherwise the write operation is ignored and no bits are written into the register. If not enabled for Halting mode, C_DEBUGEN = 1, all other fields are disabled. This register is not reset on a core reset. It is reset by a power-on reset. However, C_HALT always clears on a core reset. To halt on a reset, the following bits must be enabled: DEMCR.VC_CORERESET and C_DEBUGEN. Note that writes to this register in any size other than word are unpredictable. It is acceptable to read in any size, and it can be used to avoid or intentionally change a sticky bit. Behavior of the system when writing to this register while CPU is halted (i.e. C_DEBUGEN = 1 and S_HALT= 1): C_HALT=0, C_STEP=0, C_MASKINTS=0 Exit Debug state and start instruction execution. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=0, C_MASKINTS=1 Exit Debug state and start instruction execution. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=0 Exit Debug state, step an instruction and halt. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=1 Exit Debug state, step an instruction and halt. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=1, C_STEP=x, C_MASKINTS=x Remain in Debug state."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DHCSR(pub u32);
impl DHCSR {
    #[doc = "0:0\\] Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it. The core must write a 1 to it when writing C_HALT to halt itself. The values of C_HALT, C_STEP and C_MASKINTS are ignored by hardware when C_DEBUGEN = 0. The read values for C_HALT, C_STEP and C_MASKINTS fields will be unknown to software when C_DEBUGEN = 0."]
    #[must_use]
    #[inline(always)]
    pub const fn C_DEBUGEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Enables debug. This can only be written by AHB-AP and not by the core. It is ignored when written by the core, which cannot set or clear it. The core must write a 1 to it when writing C_HALT to halt itself. The values of C_HALT, C_STEP and C_MASKINTS are ignored by hardware when C_DEBUGEN = 0. The read values for C_HALT, C_STEP and C_MASKINTS fields will be unknown to software when C_DEBUGEN = 0."]
    #[inline(always)]
    pub const fn set_C_DEBUGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset."]
    #[must_use]
    #[inline(always)]
    pub const fn C_HALT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Halts the core. This bit is set automatically when the core Halts. For example Breakpoint. This bit clears on core reset."]
    #[inline(always)]
    pub const fn set_C_HALT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1). Modifying C_STEP while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn C_STEP(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Steps the core in halted debug. When C_DEBUGEN = 0, this bit has no effect. Must only be modified when the processor is halted (S_HALT == 1). Modifying C_STEP while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[inline(always)]
    pub const fn set_C_STEP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Mask interrupts when stepping or running in halted debug. This masking does not affect NMI, fault exceptions and SVC caused by execution of the instructions. This bit must only be modified when the processor is halted (S_HALT == 1). C_MASKINTS must be set or cleared before halt is released (i.e., the writes to set or clear C_MASKINTS and to set or clear C_HALT must be separate). Modifying C_MASKINTS while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn C_MASKINTS(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Mask interrupts when stepping or running in halted debug. This masking does not affect NMI, fault exceptions and SVC caused by execution of the instructions. This bit must only be modified when the processor is halted (S_HALT == 1). C_MASKINTS must be set or cleared before halt is released (i.e., the writes to set or clear C_MASKINTS and to set or clear C_HALT must be separate). Modifying C_MASKINTS while the system is running with halting debug support enabled (C_DEBUGEN = 1, S_HALT = 0) may cause unpredictable behavior."]
    #[inline(always)]
    pub const fn set_C_MASKINTS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 and C_HALT = 1. The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE_ST can detect core stalls on load/store operations."]
    #[must_use]
    #[inline(always)]
    pub const fn C_SNAPSTALL(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] If the core is stalled on a load/store operation the stall ceases and the instruction is forced to complete. This enables Halting debug to gain control of the core. It can only be set if: C_DEBUGEN = 1 and C_HALT = 1. The core reads S_RETIRE_ST as 0. This indicates that no instruction has advanced. This prevents misuse. The bus state is Unpredictable when this is used. S_RETIRE_ST can detect core stalls on load/store operations."]
    #[inline(always)]
    pub const fn set_C_SNAPSTALL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "15:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED6(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x03ff;
        val as u16
    }
    #[doc = "15:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u32) & 0x03ff) << 6usize);
    }
    #[doc = "16:16\\] Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[must_use]
    #[inline(always)]
    pub const fn S_REGRDY(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Register Read/Write on the Debug Core Register Selector register is available. Last transfer is complete. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub const fn set_S_REGRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] The core is in debug state when this bit is set. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[must_use]
    #[inline(always)]
    pub const fn S_HALT(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] The core is in debug state when this bit is set. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub const fn set_S_HALT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Indicates that the core is sleeping (WFI, WFE, or **SLEEP-ON-EXIT**). Must use C_HALT to gain control or wait for interrupt to wake-up. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[must_use]
    #[inline(always)]
    pub const fn S_SLEEP(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Indicates that the core is sleeping (WFI, WFE, or **SLEEP-ON-EXIT**). Must use C_HALT to gain control or wait for interrupt to wake-up. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub const fn set_S_SLEEP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Reads as one if the core is running (not halted) and a lockup condition is present. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[must_use]
    #[inline(always)]
    pub const fn S_LOCKUP(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Reads as one if the core is running (not halted) and a lockup condition is present. When writing to this register, 1 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub const fn set_S_LOCKUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "23:20\\] Software should not rely on the value of a reserved. When writing to this register, 0x5 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED20(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Software should not rely on the value of a reserved. When writing to this register, 0x5 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub const fn set_RESERVED20(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "24:24\\] Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch. When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[must_use]
    #[inline(always)]
    pub const fn S_RETIRE_ST(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Indicates that an instruction has completed since last read. This is a sticky bit that clears on read. This determines if the core is stalled on a load/store or fetch. When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub const fn set_S_RETIRE_ST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still). When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[must_use]
    #[inline(always)]
    pub const fn S_RESET_ST(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Indicates that the core has been reset, or is now being reset, since the last time this bit was read. This a sticky bit that clears on read. So, reading twice and getting 1 then 0 means it was reset in the past. Reading twice and getting 1 both times means that it is being reset now (held in reset still). When writing to this register, 0 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub const fn set_S_RESET_ST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "31:26\\] Software should not rely on the value of a reserved. When writing to this register, 0x28 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED26(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "31:26\\] Software should not rely on the value of a reserved. When writing to this register, 0x28 must be written this bit-field, otherwise the write operation is ignored and no bits are written into the register."]
    #[inline(always)]
    pub const fn set_RESERVED26(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for DHCSR {
    #[inline(always)]
    fn default() -> DHCSR {
        DHCSR(0)
    }
}
impl core::fmt::Debug for DHCSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHCSR")
            .field("C_DEBUGEN", &self.C_DEBUGEN())
            .field("C_HALT", &self.C_HALT())
            .field("C_STEP", &self.C_STEP())
            .field("C_MASKINTS", &self.C_MASKINTS())
            .field("RESERVED4", &self.RESERVED4())
            .field("C_SNAPSTALL", &self.C_SNAPSTALL())
            .field("RESERVED6", &self.RESERVED6())
            .field("S_REGRDY", &self.S_REGRDY())
            .field("S_HALT", &self.S_HALT())
            .field("S_SLEEP", &self.S_SLEEP())
            .field("S_LOCKUP", &self.S_LOCKUP())
            .field("RESERVED20", &self.RESERVED20())
            .field("S_RETIRE_ST", &self.S_RETIRE_ST())
            .field("S_RESET_ST", &self.S_RESET_ST())
            .field("RESERVED26", &self.RESERVED26())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DHCSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DHCSR {{ C_DEBUGEN: {=bool:?}, C_HALT: {=bool:?}, C_STEP: {=bool:?}, C_MASKINTS: {=bool:?}, RESERVED4: {=bool:?}, C_SNAPSTALL: {=bool:?}, RESERVED6: {=u16:?}, S_REGRDY: {=bool:?}, S_HALT: {=bool:?}, S_SLEEP: {=bool:?}, S_LOCKUP: {=bool:?}, RESERVED20: {=u8:?}, S_RETIRE_ST: {=bool:?}, S_RESET_ST: {=bool:?}, RESERVED26: {=u8:?} }}",
            self.C_DEBUGEN(),
            self.C_HALT(),
            self.C_STEP(),
            self.C_MASKINTS(),
            self.RESERVED4(),
            self.C_SNAPSTALL(),
            self.RESERVED6(),
            self.S_REGRDY(),
            self.S_HALT(),
            self.S_SLEEP(),
            self.S_LOCKUP(),
            self.RESERVED20(),
            self.S_RETIRE_ST(),
            self.S_RESET_ST(),
            self.RESERVED26()
        )
    }
}
#[doc = "Hard Fault Status This register is used to obtain information about events that activate the Hard Fault handler. This register is a write-clear register. This means that writing a 1 to a bit clears that bit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HFSR(pub u32);
impl HFSR {
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
    #[doc = "1:1\\] This bit is set if there is a fault because of vector table read on exception processing (Bus Fault). This case is always a Hard Fault. The return PC points to the pre-empted instruction."]
    #[must_use]
    #[inline(always)]
    pub const fn VECTTBL(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] This bit is set if there is a fault because of vector table read on exception processing (Bus Fault). This case is always a Hard Fault. The return PC points to the pre-empted instruction."]
    #[inline(always)]
    pub const fn set_VECTTBL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "29:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "29:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 2usize)) | (((val as u32) & 0x0fff_ffff) << 2usize);
    }
    #[doc = "30:30\\] Hard Fault activated because a Configurable Fault was received and cannot activate because of priority or because the Configurable Fault is disabled. The Hard Fault handler then has to read the other fault status registers to determine cause."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCED(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Hard Fault activated because a Configurable Fault was received and cannot activate because of priority or because the Configurable Fault is disabled. The Hard Fault handler then has to read the other fault status registers to determine cause."]
    #[inline(always)]
    pub const fn set_FORCED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] This bit is set if there is a fault related to debug. This is only possible when halting debug is not enabled. For monitor enabled debug, it only happens for BKPT when the current priority is higher than the monitor. When both halting and monitor debug are disabled, it only happens for debug events that are not ignored (minimally, BKPT). The Debug Fault Status Register is updated."]
    #[must_use]
    #[inline(always)]
    pub const fn DEBUGEVT(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] This bit is set if there is a fault related to debug. This is only possible when halting debug is not enabled. For monitor enabled debug, it only happens for BKPT when the current priority is higher than the monitor. When both halting and monitor debug are disabled, it only happens for debug events that are not ignored (minimally, BKPT). The Debug Fault Status Register is updated."]
    #[inline(always)]
    pub const fn set_DEBUGEVT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HFSR {
    #[inline(always)]
    fn default() -> HFSR {
        HFSR(0)
    }
}
impl core::fmt::Debug for HFSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFSR")
            .field("RESERVED0", &self.RESERVED0())
            .field("VECTTBL", &self.VECTTBL())
            .field("RESERVED2", &self.RESERVED2())
            .field("FORCED", &self.FORCED())
            .field("DEBUGEVT", &self.DEBUGEVT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HFSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HFSR {{ RESERVED0: {=bool:?}, VECTTBL: {=bool:?}, RESERVED2: {=u32:?}, FORCED: {=bool:?}, DEBUGEVT: {=bool:?} }}",
            self.RESERVED0(),
            self.VECTTBL(),
            self.RESERVED2(),
            self.FORCED(),
            self.DEBUGEVT()
        )
    }
}
#[doc = "Interrupt Control State This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ICSR(pub u32);
impl ICSR {
    #[doc = "8:0\\] Active ISR number field. Reset clears this field."]
    #[must_use]
    #[inline(always)]
    pub const fn VECTACTIVE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "8:0\\] Active ISR number field. Reset clears this field."]
    #[inline(always)]
    pub const fn set_VECTACTIVE(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "10:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "10:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "11:11\\] Indicates whether there are preempted active exceptions: 0: There are preempted active exceptions to execute 1: There are no active exceptions, or the currently-executing exception is the only active exception."]
    #[must_use]
    #[inline(always)]
    pub const fn RETTOBASE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Indicates whether there are preempted active exceptions: 0: There are preempted active exceptions to execute 1: There are no active exceptions, or the currently-executing exception is the only active exception."]
    #[inline(always)]
    pub const fn set_RETTOBASE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "17:12\\] Pending ISR number field. This field contains the interrupt number of the highest priority pending ISR."]
    #[must_use]
    #[inline(always)]
    pub const fn VECTPENDING(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[doc = "17:12\\] Pending ISR number field. This field contains the interrupt number of the highest priority pending ISR."]
    #[inline(always)]
    pub const fn set_VECTPENDING(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
    #[doc = "21:18\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED18(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "21:18\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED18(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "22:22\\] Interrupt pending flag. Excludes NMI and faults. 0x0: Interrupt not pending 0x1: Interrupt pending."]
    #[must_use]
    #[inline(always)]
    pub const fn ISRPENDING(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Interrupt pending flag. Excludes NMI and faults. 0x0: Interrupt not pending 0x1: Interrupt pending."]
    #[inline(always)]
    pub const fn set_ISRPENDING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] This field can only be used at debug time. It indicates that a pending interrupt is to be taken in the next running cycle. If DHCSR.C_MASKINTS= 0, the interrupt is serviced. 0: A pending exception is not serviced. 1: A pending exception is serviced on exit from the debug halt state."]
    #[must_use]
    #[inline(always)]
    pub const fn ISRPREEMPT(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] This field can only be used at debug time. It indicates that a pending interrupt is to be taken in the next running cycle. If DHCSR.C_MASKINTS= 0, the interrupt is serviced. 0: A pending exception is not serviced. 1: A pending exception is serviced on exit from the debug halt state."]
    #[inline(always)]
    pub const fn set_ISRPREEMPT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Clear pending SysTick bit 0: No action 1: Clear pending SysTick."]
    #[must_use]
    #[inline(always)]
    pub const fn PENDSTCLR(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Clear pending SysTick bit 0: No action 1: Clear pending SysTick."]
    #[inline(always)]
    pub const fn set_PENDSTCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Set a pending SysTick bit. 0: No action 1: Set pending SysTick."]
    #[must_use]
    #[inline(always)]
    pub const fn PENDSTSET(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Set a pending SysTick bit. 0: No action 1: Set pending SysTick."]
    #[inline(always)]
    pub const fn set_PENDSTSET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Clear pending pendSV bit 0: No action 1: Clear pending pendSV."]
    #[must_use]
    #[inline(always)]
    pub const fn PENDSVCLR(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Clear pending pendSV bit 0: No action 1: Clear pending pendSV."]
    #[inline(always)]
    pub const fn set_PENDSVCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Set pending pendSV bit. 0: No action 1: Set pending PendSV."]
    #[must_use]
    #[inline(always)]
    pub const fn PENDSVSET(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Set pending pendSV bit. 0: No action 1: Set pending PendSV."]
    #[inline(always)]
    pub const fn set_PENDSVSET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "30:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED29(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "30:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED29(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
    #[doc = "31:31\\] Set pending NMI bit. Setting this bit pends and activates an NMI. Because NMI is the highest-priority interrupt, it takes effect as soon as it registers. 0: No action 1: Set pending NMI."]
    #[must_use]
    #[inline(always)]
    pub const fn NMIPENDSET(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Set pending NMI bit. Setting this bit pends and activates an NMI. Because NMI is the highest-priority interrupt, it takes effect as soon as it registers. 0: No action 1: Set pending NMI."]
    #[inline(always)]
    pub const fn set_NMIPENDSET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ICSR {
    #[inline(always)]
    fn default() -> ICSR {
        ICSR(0)
    }
}
impl core::fmt::Debug for ICSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICSR")
            .field("VECTACTIVE", &self.VECTACTIVE())
            .field("RESERVED9", &self.RESERVED9())
            .field("RETTOBASE", &self.RETTOBASE())
            .field("VECTPENDING", &self.VECTPENDING())
            .field("RESERVED18", &self.RESERVED18())
            .field("ISRPENDING", &self.ISRPENDING())
            .field("ISRPREEMPT", &self.ISRPREEMPT())
            .field("RESERVED24", &self.RESERVED24())
            .field("PENDSTCLR", &self.PENDSTCLR())
            .field("PENDSTSET", &self.PENDSTSET())
            .field("PENDSVCLR", &self.PENDSVCLR())
            .field("PENDSVSET", &self.PENDSVSET())
            .field("RESERVED29", &self.RESERVED29())
            .field("NMIPENDSET", &self.NMIPENDSET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ICSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ICSR {{ VECTACTIVE: {=u16:?}, RESERVED9: {=u8:?}, RETTOBASE: {=bool:?}, VECTPENDING: {=u8:?}, RESERVED18: {=u8:?}, ISRPENDING: {=bool:?}, ISRPREEMPT: {=bool:?}, RESERVED24: {=bool:?}, PENDSTCLR: {=bool:?}, PENDSTSET: {=bool:?}, PENDSVCLR: {=bool:?}, PENDSVSET: {=bool:?}, RESERVED29: {=u8:?}, NMIPENDSET: {=bool:?} }}",
            self.VECTACTIVE(),
            self.RESERVED9(),
            self.RETTOBASE(),
            self.VECTPENDING(),
            self.RESERVED18(),
            self.ISRPENDING(),
            self.ISRPREEMPT(),
            self.RESERVED24(),
            self.PENDSTCLR(),
            self.PENDSTSET(),
            self.PENDSVCLR(),
            self.PENDSVSET(),
            self.RESERVED29(),
            self.NMIPENDSET()
        )
    }
}
#[doc = "Interrupt Control Type Read this register to see the number of interrupt lines that the NVIC supports."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ICTR(pub u32);
impl ICTR {
    #[doc = "2:0\\] Total number of interrupt lines in groups of 32. 0: 0...32 1: 33...64 2: 65...96 3: 97...128 4: 129...160 5: 161...192 6: 193...224 7: 225...256."]
    #[must_use]
    #[inline(always)]
    pub const fn INTLINESNUM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "2:0\\] Total number of interrupt lines in groups of 32. 0: 0...32 1: 33...64 2: 65...96 3: 97...128 4: 129...160 5: 161...192 6: 193...224 7: 225...256."]
    #[inline(always)]
    pub const fn set_INTLINESNUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
impl Default for ICTR {
    #[inline(always)]
    fn default() -> ICTR {
        ICTR(0)
    }
}
impl core::fmt::Debug for ICTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICTR")
            .field("INTLINESNUM", &self.INTLINESNUM())
            .field("RESERVED3", &self.RESERVED3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ICTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ICTR {{ INTLINESNUM: {=u8:?}, RESERVED3: {=u32:?} }}",
            self.INTLINESNUM(),
            self.RESERVED3()
        )
    }
}
#[doc = "Auxiliary Feature 0 This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID_AFR0(pub u32);
impl ID_AFR0 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ID_AFR0 {
    #[inline(always)]
    fn default() -> ID_AFR0 {
        ID_AFR0(0)
    }
}
impl core::fmt::Debug for ID_AFR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID_AFR0")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID_AFR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ID_AFR0 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Debug Feature 0 This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID_DFR0(pub u32);
impl ID_DFR0 {
    #[doc = "19:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "19:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
    #[doc = "23:20\\] Microcontroller Debug Model - memory mapped 0x0: Not supported 0x1: Microcontroller debug v1 (ITMv1 and DWTv1)."]
    #[must_use]
    #[inline(always)]
    pub const fn MICROCONTROLLER_DEBUG_MODEL(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Microcontroller Debug Model - memory mapped 0x0: Not supported 0x1: Microcontroller debug v1 (ITMv1 and DWTv1)."]
    #[inline(always)]
    pub const fn set_MICROCONTROLLER_DEBUG_MODEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
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
impl Default for ID_DFR0 {
    #[inline(always)]
    fn default() -> ID_DFR0 {
        ID_DFR0(0)
    }
}
impl core::fmt::Debug for ID_DFR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID_DFR0")
            .field("RESERVED0", &self.RESERVED0())
            .field(
                "MICROCONTROLLER_DEBUG_MODEL",
                &self.MICROCONTROLLER_DEBUG_MODEL(),
            )
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID_DFR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ID_DFR0 {{ RESERVED0: {=u32:?}, MICROCONTROLLER_DEBUG_MODEL: {=u8:?}, RESERVED24: {=u8:?} }}",
            self.RESERVED0(),
            self.MICROCONTROLLER_DEBUG_MODEL(),
            self.RESERVED24()
        )
    }
}
#[doc = "ISA Feature 0 Information on the instruction set attributes register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID_ISAR0(pub u32);
impl ID_ISAR0 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ID_ISAR0 {
    #[inline(always)]
    fn default() -> ID_ISAR0 {
        ID_ISAR0(0)
    }
}
impl core::fmt::Debug for ID_ISAR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID_ISAR0")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID_ISAR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ID_ISAR0 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "ISA Feature 1 Information on the instruction set attributes register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID_ISAR1(pub u32);
impl ID_ISAR1 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ID_ISAR1 {
    #[inline(always)]
    fn default() -> ID_ISAR1 {
        ID_ISAR1(0)
    }
}
impl core::fmt::Debug for ID_ISAR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID_ISAR1")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID_ISAR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ID_ISAR1 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "ISA Feature 2 Information on the instruction set attributes register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID_ISAR2(pub u32);
impl ID_ISAR2 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ID_ISAR2 {
    #[inline(always)]
    fn default() -> ID_ISAR2 {
        ID_ISAR2(0)
    }
}
impl core::fmt::Debug for ID_ISAR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID_ISAR2")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID_ISAR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ID_ISAR2 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "ISA Feature 3 Information on the instruction set attributes register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID_ISAR3(pub u32);
impl ID_ISAR3 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ID_ISAR3 {
    #[inline(always)]
    fn default() -> ID_ISAR3 {
        ID_ISAR3(0)
    }
}
impl core::fmt::Debug for ID_ISAR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID_ISAR3")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID_ISAR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ID_ISAR3 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "ISA Feature 4 Information on the instruction set attributes register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID_ISAR4(pub u32);
impl ID_ISAR4 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ID_ISAR4 {
    #[inline(always)]
    fn default() -> ID_ISAR4 {
        ID_ISAR4(0)
    }
}
impl core::fmt::Debug for ID_ISAR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID_ISAR4")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID_ISAR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ID_ISAR4 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Memory Model Feature 0 General information on the memory model and memory management support."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID_MMFR0(pub u32);
impl ID_MMFR0 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ID_MMFR0 {
    #[inline(always)]
    fn default() -> ID_MMFR0 {
        ID_MMFR0(0)
    }
}
impl core::fmt::Debug for ID_MMFR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID_MMFR0")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID_MMFR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ID_MMFR0 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Memory Model Feature 1 General information on the memory model and memory management support."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID_MMFR1(pub u32);
impl ID_MMFR1 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ID_MMFR1 {
    #[inline(always)]
    fn default() -> ID_MMFR1 {
        ID_MMFR1(0)
    }
}
impl core::fmt::Debug for ID_MMFR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID_MMFR1")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID_MMFR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ID_MMFR1 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Memory Model Feature 2 General information on the memory model and memory management support."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID_MMFR2(pub u32);
impl ID_MMFR2 {
    #[doc = "23:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "24:24\\] wait for interrupt stalling 0x0: Not supported 0x1: Wait for interrupt supported."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_FOR_INTERRUPT_STALLING(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] wait for interrupt stalling 0x0: Not supported 0x1: Wait for interrupt supported."]
    #[inline(always)]
    pub const fn set_WAIT_FOR_INTERRUPT_STALLING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED28(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED28(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for ID_MMFR2 {
    #[inline(always)]
    fn default() -> ID_MMFR2 {
        ID_MMFR2(0)
    }
}
impl core::fmt::Debug for ID_MMFR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID_MMFR2")
            .field("RESERVED0", &self.RESERVED0())
            .field(
                "WAIT_FOR_INTERRUPT_STALLING",
                &self.WAIT_FOR_INTERRUPT_STALLING(),
            )
            .field("RESERVED28", &self.RESERVED28())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID_MMFR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ID_MMFR2 {{ RESERVED0: {=u32:?}, WAIT_FOR_INTERRUPT_STALLING: {=bool:?}, RESERVED28: {=u8:?} }}",
            self.RESERVED0(),
            self.WAIT_FOR_INTERRUPT_STALLING(),
            self.RESERVED28()
        )
    }
}
#[doc = "Memory Model Feature 3 General information on the memory model and memory management support."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID_MMFR3(pub u32);
impl ID_MMFR3 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ID_MMFR3 {
    #[inline(always)]
    fn default() -> ID_MMFR3 {
        ID_MMFR3(0)
    }
}
impl core::fmt::Debug for ID_MMFR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID_MMFR3")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID_MMFR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ID_MMFR3 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Processor Feature 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID_PFR0(pub u32);
impl ID_PFR0 {
    #[doc = "3:0\\] State0 (T-bit == 0) 0x0: No ARM encoding 0x1: N/A."]
    #[must_use]
    #[inline(always)]
    pub const fn STATE0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\] State0 (T-bit == 0) 0x0: No ARM encoding 0x1: N/A."]
    #[inline(always)]
    pub const fn set_STATE0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "7:4\\] State1 (T-bit == 1) 0x0: N/A 0x1: N/A 0x2: Thumb-2 encoding with the 16-bit basic instructions plus 32-bit Buncond/BL but no other 32-bit basic instructions (Note non-basic 32-bit instructions can be added using the appropriate instruction attribute, but other 32-bit basic instructions cannot.) 0x3: Thumb-2 encoding with all Thumb-2 basic instructions."]
    #[must_use]
    #[inline(always)]
    pub const fn STATE1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "7:4\\] State1 (T-bit == 1) 0x0: N/A 0x1: N/A 0x2: Thumb-2 encoding with the 16-bit basic instructions plus 32-bit Buncond/BL but no other 32-bit basic instructions (Note non-basic 32-bit instructions can be added using the appropriate instruction attribute, but other 32-bit basic instructions cannot.) 0x3: Thumb-2 encoding with all Thumb-2 basic instructions."]
    #[inline(always)]
    pub const fn set_STATE1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
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
impl Default for ID_PFR0 {
    #[inline(always)]
    fn default() -> ID_PFR0 {
        ID_PFR0(0)
    }
}
impl core::fmt::Debug for ID_PFR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID_PFR0")
            .field("STATE0", &self.STATE0())
            .field("STATE1", &self.STATE1())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID_PFR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ID_PFR0 {{ STATE0: {=u8:?}, STATE1: {=u8:?}, RESERVED8: {=u32:?} }}",
            self.STATE0(),
            self.STATE1(),
            self.RESERVED8()
        )
    }
}
#[doc = "Processor Feature 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID_PFR1(pub u32);
impl ID_PFR1 {
    #[doc = "7:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "11:8\\] Microcontroller programmer's model 0x0: Not supported 0x2: Two-stack support."]
    #[must_use]
    #[inline(always)]
    pub const fn MICROCONTROLLER_PROGRAMMERS_MODEL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "11:8\\] Microcontroller programmer's model 0x0: Not supported 0x2: Two-stack support."]
    #[inline(always)]
    pub const fn set_MICROCONTROLLER_PROGRAMMERS_MODEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "31:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED12(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "31:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED12(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for ID_PFR1 {
    #[inline(always)]
    fn default() -> ID_PFR1 {
        ID_PFR1(0)
    }
}
impl core::fmt::Debug for ID_PFR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID_PFR1")
            .field("RESERVED0", &self.RESERVED0())
            .field(
                "MICROCONTROLLER_PROGRAMMERS_MODEL",
                &self.MICROCONTROLLER_PROGRAMMERS_MODEL(),
            )
            .field("RESERVED12", &self.RESERVED12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID_PFR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ID_PFR1 {{ RESERVED0: {=u8:?}, MICROCONTROLLER_PROGRAMMERS_MODEL: {=u8:?}, RESERVED12: {=u32:?} }}",
            self.RESERVED0(),
            self.MICROCONTROLLER_PROGRAMMERS_MODEL(),
            self.RESERVED12()
        )
    }
}
#[doc = "Mem Manage Fault Address This register is used to read the address of the location that caused a Memory Manage Fault."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MMFAR(pub u32);
impl MMFAR {
    #[doc = "31:0\\] Mem Manage fault address field. This field is the data address of a faulted load or store attempt. When an unaligned access faults, the address is the actual address that faulted. Because an access can be split into multiple parts, each aligned, this address can be any offset in the range of the requested size. Flags CFSR.IACCVIOL, CFSR.DACCVIOL ,CFSR.MUNSTKERR and CFSR.MSTKERR in combination with CFSR.MMARVALIDindicate the cause of the fault."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDRESS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Mem Manage fault address field. This field is the data address of a faulted load or store attempt. When an unaligned access faults, the address is the actual address that faulted. Because an access can be split into multiple parts, each aligned, this address can be any offset in the range of the requested size. Flags CFSR.IACCVIOL, CFSR.DACCVIOL ,CFSR.MUNSTKERR and CFSR.MSTKERR in combination with CFSR.MMARVALIDindicate the cause of the fault."]
    #[inline(always)]
    pub const fn set_ADDRESS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MMFAR {
    #[inline(always)]
    fn default() -> MMFAR {
        MMFAR(0)
    }
}
impl core::fmt::Debug for MMFAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMFAR")
            .field("ADDRESS", &self.ADDRESS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MMFAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MMFAR {{ ADDRESS: {=u32:?} }}", self.ADDRESS())
    }
}
#[doc = "Irq 0 to 31 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_IABR0(pub u32);
impl NVIC_IABR0 {
    #[doc = "0:0\\] Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for NVIC_IABR0 {
    #[inline(always)]
    fn default() -> NVIC_IABR0 {
        NVIC_IABR0(0)
    }
}
impl core::fmt::Debug for NVIC_IABR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_IABR0")
            .field("ACTIVE0", &self.ACTIVE0())
            .field("ACTIVE1", &self.ACTIVE1())
            .field("ACTIVE2", &self.ACTIVE2())
            .field("ACTIVE3", &self.ACTIVE3())
            .field("ACTIVE4", &self.ACTIVE4())
            .field("ACTIVE5", &self.ACTIVE5())
            .field("ACTIVE6", &self.ACTIVE6())
            .field("ACTIVE7", &self.ACTIVE7())
            .field("ACTIVE8", &self.ACTIVE8())
            .field("ACTIVE9", &self.ACTIVE9())
            .field("ACTIVE10", &self.ACTIVE10())
            .field("ACTIVE11", &self.ACTIVE11())
            .field("ACTIVE12", &self.ACTIVE12())
            .field("ACTIVE13", &self.ACTIVE13())
            .field("ACTIVE14", &self.ACTIVE14())
            .field("ACTIVE15", &self.ACTIVE15())
            .field("ACTIVE16", &self.ACTIVE16())
            .field("ACTIVE17", &self.ACTIVE17())
            .field("ACTIVE18", &self.ACTIVE18())
            .field("ACTIVE19", &self.ACTIVE19())
            .field("ACTIVE20", &self.ACTIVE20())
            .field("ACTIVE21", &self.ACTIVE21())
            .field("ACTIVE22", &self.ACTIVE22())
            .field("ACTIVE23", &self.ACTIVE23())
            .field("ACTIVE24", &self.ACTIVE24())
            .field("ACTIVE25", &self.ACTIVE25())
            .field("ACTIVE26", &self.ACTIVE26())
            .field("ACTIVE27", &self.ACTIVE27())
            .field("ACTIVE28", &self.ACTIVE28())
            .field("ACTIVE29", &self.ACTIVE29())
            .field("ACTIVE30", &self.ACTIVE30())
            .field("ACTIVE31", &self.ACTIVE31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_IABR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_IABR0 {{ ACTIVE0: {=bool:?}, ACTIVE1: {=bool:?}, ACTIVE2: {=bool:?}, ACTIVE3: {=bool:?}, ACTIVE4: {=bool:?}, ACTIVE5: {=bool:?}, ACTIVE6: {=bool:?}, ACTIVE7: {=bool:?}, ACTIVE8: {=bool:?}, ACTIVE9: {=bool:?}, ACTIVE10: {=bool:?}, ACTIVE11: {=bool:?}, ACTIVE12: {=bool:?}, ACTIVE13: {=bool:?}, ACTIVE14: {=bool:?}, ACTIVE15: {=bool:?}, ACTIVE16: {=bool:?}, ACTIVE17: {=bool:?}, ACTIVE18: {=bool:?}, ACTIVE19: {=bool:?}, ACTIVE20: {=bool:?}, ACTIVE21: {=bool:?}, ACTIVE22: {=bool:?}, ACTIVE23: {=bool:?}, ACTIVE24: {=bool:?}, ACTIVE25: {=bool:?}, ACTIVE26: {=bool:?}, ACTIVE27: {=bool:?}, ACTIVE28: {=bool:?}, ACTIVE29: {=bool:?}, ACTIVE30: {=bool:?}, ACTIVE31: {=bool:?} }}",
            self.ACTIVE0(),
            self.ACTIVE1(),
            self.ACTIVE2(),
            self.ACTIVE3(),
            self.ACTIVE4(),
            self.ACTIVE5(),
            self.ACTIVE6(),
            self.ACTIVE7(),
            self.ACTIVE8(),
            self.ACTIVE9(),
            self.ACTIVE10(),
            self.ACTIVE11(),
            self.ACTIVE12(),
            self.ACTIVE13(),
            self.ACTIVE14(),
            self.ACTIVE15(),
            self.ACTIVE16(),
            self.ACTIVE17(),
            self.ACTIVE18(),
            self.ACTIVE19(),
            self.ACTIVE20(),
            self.ACTIVE21(),
            self.ACTIVE22(),
            self.ACTIVE23(),
            self.ACTIVE24(),
            self.ACTIVE25(),
            self.ACTIVE26(),
            self.ACTIVE27(),
            self.ACTIVE28(),
            self.ACTIVE29(),
            self.ACTIVE30(),
            self.ACTIVE31()
        )
    }
}
#[doc = "Irq 32 to 63 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_IABR1(pub u32);
impl NVIC_IABR1 {
    #[doc = "0:0\\] Reading 0 from this bit implies that interrupt line 32 is not active. Reading 1 from this bit implies that the interrupt line 32 is active (See EVENT:CPUIRQSEL32.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Reading 0 from this bit implies that interrupt line 32 is not active. Reading 1 from this bit implies that the interrupt line 32 is active (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Reading 0 from this bit implies that interrupt line 33 is not active. Reading 1 from this bit implies that the interrupt line 33 is active (See EVENT:CPUIRQSEL33.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Reading 0 from this bit implies that interrupt line 33 is not active. Reading 1 from this bit implies that the interrupt line 33 is active (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline(always)]
    pub const fn set_ACTIVE33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for NVIC_IABR1 {
    #[inline(always)]
    fn default() -> NVIC_IABR1 {
        NVIC_IABR1(0)
    }
}
impl core::fmt::Debug for NVIC_IABR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_IABR1")
            .field("ACTIVE32", &self.ACTIVE32())
            .field("ACTIVE33", &self.ACTIVE33())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_IABR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_IABR1 {{ ACTIVE32: {=bool:?}, ACTIVE33: {=bool:?}, RESERVED2: {=u32:?} }}",
            self.ACTIVE32(),
            self.ACTIVE33(),
            self.RESERVED2()
        )
    }
}
#[doc = "Irq 0 to 31 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_ICER0(pub u32);
impl NVIC_ICER0 {
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for NVIC_ICER0 {
    #[inline(always)]
    fn default() -> NVIC_ICER0 {
        NVIC_ICER0(0)
    }
}
impl core::fmt::Debug for NVIC_ICER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_ICER0")
            .field("CLRENA0", &self.CLRENA0())
            .field("CLRENA1", &self.CLRENA1())
            .field("CLRENA2", &self.CLRENA2())
            .field("CLRENA3", &self.CLRENA3())
            .field("CLRENA4", &self.CLRENA4())
            .field("CLRENA5", &self.CLRENA5())
            .field("CLRENA6", &self.CLRENA6())
            .field("CLRENA7", &self.CLRENA7())
            .field("CLRENA8", &self.CLRENA8())
            .field("CLRENA9", &self.CLRENA9())
            .field("CLRENA10", &self.CLRENA10())
            .field("CLRENA11", &self.CLRENA11())
            .field("CLRENA12", &self.CLRENA12())
            .field("CLRENA13", &self.CLRENA13())
            .field("CLRENA14", &self.CLRENA14())
            .field("CLRENA15", &self.CLRENA15())
            .field("CLRENA16", &self.CLRENA16())
            .field("CLRENA17", &self.CLRENA17())
            .field("CLRENA18", &self.CLRENA18())
            .field("CLRENA19", &self.CLRENA19())
            .field("CLRENA20", &self.CLRENA20())
            .field("CLRENA21", &self.CLRENA21())
            .field("CLRENA22", &self.CLRENA22())
            .field("CLRENA23", &self.CLRENA23())
            .field("CLRENA24", &self.CLRENA24())
            .field("CLRENA25", &self.CLRENA25())
            .field("CLRENA26", &self.CLRENA26())
            .field("CLRENA27", &self.CLRENA27())
            .field("CLRENA28", &self.CLRENA28())
            .field("CLRENA29", &self.CLRENA29())
            .field("CLRENA30", &self.CLRENA30())
            .field("CLRENA31", &self.CLRENA31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_ICER0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_ICER0 {{ CLRENA0: {=bool:?}, CLRENA1: {=bool:?}, CLRENA2: {=bool:?}, CLRENA3: {=bool:?}, CLRENA4: {=bool:?}, CLRENA5: {=bool:?}, CLRENA6: {=bool:?}, CLRENA7: {=bool:?}, CLRENA8: {=bool:?}, CLRENA9: {=bool:?}, CLRENA10: {=bool:?}, CLRENA11: {=bool:?}, CLRENA12: {=bool:?}, CLRENA13: {=bool:?}, CLRENA14: {=bool:?}, CLRENA15: {=bool:?}, CLRENA16: {=bool:?}, CLRENA17: {=bool:?}, CLRENA18: {=bool:?}, CLRENA19: {=bool:?}, CLRENA20: {=bool:?}, CLRENA21: {=bool:?}, CLRENA22: {=bool:?}, CLRENA23: {=bool:?}, CLRENA24: {=bool:?}, CLRENA25: {=bool:?}, CLRENA26: {=bool:?}, CLRENA27: {=bool:?}, CLRENA28: {=bool:?}, CLRENA29: {=bool:?}, CLRENA30: {=bool:?}, CLRENA31: {=bool:?} }}",
            self.CLRENA0(),
            self.CLRENA1(),
            self.CLRENA2(),
            self.CLRENA3(),
            self.CLRENA4(),
            self.CLRENA5(),
            self.CLRENA6(),
            self.CLRENA7(),
            self.CLRENA8(),
            self.CLRENA9(),
            self.CLRENA10(),
            self.CLRENA11(),
            self.CLRENA12(),
            self.CLRENA13(),
            self.CLRENA14(),
            self.CLRENA15(),
            self.CLRENA16(),
            self.CLRENA17(),
            self.CLRENA18(),
            self.CLRENA19(),
            self.CLRENA20(),
            self.CLRENA21(),
            self.CLRENA22(),
            self.CLRENA23(),
            self.CLRENA24(),
            self.CLRENA25(),
            self.CLRENA26(),
            self.CLRENA27(),
            self.CLRENA28(),
            self.CLRENA29(),
            self.CLRENA30(),
            self.CLRENA31()
        )
    }
}
#[doc = "Irq 32 to 63 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_ICER1(pub u32);
impl NVIC_ICER1 {
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRENA33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_CLRENA33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for NVIC_ICER1 {
    #[inline(always)]
    fn default() -> NVIC_ICER1 {
        NVIC_ICER1(0)
    }
}
impl core::fmt::Debug for NVIC_ICER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_ICER1")
            .field("CLRENA32", &self.CLRENA32())
            .field("CLRENA33", &self.CLRENA33())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_ICER1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_ICER1 {{ CLRENA32: {=bool:?}, CLRENA33: {=bool:?}, RESERVED2: {=u32:?} }}",
            self.CLRENA32(),
            self.CLRENA33(),
            self.RESERVED2()
        )
    }
}
#[doc = "Irq 0 to 31 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_ICPR0(pub u32);
impl NVIC_ICPR0 {
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for NVIC_ICPR0 {
    #[inline(always)]
    fn default() -> NVIC_ICPR0 {
        NVIC_ICPR0(0)
    }
}
impl core::fmt::Debug for NVIC_ICPR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_ICPR0")
            .field("CLRPEND0", &self.CLRPEND0())
            .field("CLRPEND1", &self.CLRPEND1())
            .field("CLRPEND2", &self.CLRPEND2())
            .field("CLRPEND3", &self.CLRPEND3())
            .field("CLRPEND4", &self.CLRPEND4())
            .field("CLRPEND5", &self.CLRPEND5())
            .field("CLRPEND6", &self.CLRPEND6())
            .field("CLRPEND7", &self.CLRPEND7())
            .field("CLRPEND8", &self.CLRPEND8())
            .field("CLRPEND9", &self.CLRPEND9())
            .field("CLRPEND10", &self.CLRPEND10())
            .field("CLRPEND11", &self.CLRPEND11())
            .field("CLRPEND12", &self.CLRPEND12())
            .field("CLRPEND13", &self.CLRPEND13())
            .field("CLRPEND14", &self.CLRPEND14())
            .field("CLRPEND15", &self.CLRPEND15())
            .field("CLRPEND16", &self.CLRPEND16())
            .field("CLRPEND17", &self.CLRPEND17())
            .field("CLRPEND18", &self.CLRPEND18())
            .field("CLRPEND19", &self.CLRPEND19())
            .field("CLRPEND20", &self.CLRPEND20())
            .field("CLRPEND21", &self.CLRPEND21())
            .field("CLRPEND22", &self.CLRPEND22())
            .field("CLRPEND23", &self.CLRPEND23())
            .field("CLRPEND24", &self.CLRPEND24())
            .field("CLRPEND25", &self.CLRPEND25())
            .field("CLRPEND26", &self.CLRPEND26())
            .field("CLRPEND27", &self.CLRPEND27())
            .field("CLRPEND28", &self.CLRPEND28())
            .field("CLRPEND29", &self.CLRPEND29())
            .field("CLRPEND30", &self.CLRPEND30())
            .field("CLRPEND31", &self.CLRPEND31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_ICPR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_ICPR0 {{ CLRPEND0: {=bool:?}, CLRPEND1: {=bool:?}, CLRPEND2: {=bool:?}, CLRPEND3: {=bool:?}, CLRPEND4: {=bool:?}, CLRPEND5: {=bool:?}, CLRPEND6: {=bool:?}, CLRPEND7: {=bool:?}, CLRPEND8: {=bool:?}, CLRPEND9: {=bool:?}, CLRPEND10: {=bool:?}, CLRPEND11: {=bool:?}, CLRPEND12: {=bool:?}, CLRPEND13: {=bool:?}, CLRPEND14: {=bool:?}, CLRPEND15: {=bool:?}, CLRPEND16: {=bool:?}, CLRPEND17: {=bool:?}, CLRPEND18: {=bool:?}, CLRPEND19: {=bool:?}, CLRPEND20: {=bool:?}, CLRPEND21: {=bool:?}, CLRPEND22: {=bool:?}, CLRPEND23: {=bool:?}, CLRPEND24: {=bool:?}, CLRPEND25: {=bool:?}, CLRPEND26: {=bool:?}, CLRPEND27: {=bool:?}, CLRPEND28: {=bool:?}, CLRPEND29: {=bool:?}, CLRPEND30: {=bool:?}, CLRPEND31: {=bool:?} }}",
            self.CLRPEND0(),
            self.CLRPEND1(),
            self.CLRPEND2(),
            self.CLRPEND3(),
            self.CLRPEND4(),
            self.CLRPEND5(),
            self.CLRPEND6(),
            self.CLRPEND7(),
            self.CLRPEND8(),
            self.CLRPEND9(),
            self.CLRPEND10(),
            self.CLRPEND11(),
            self.CLRPEND12(),
            self.CLRPEND13(),
            self.CLRPEND14(),
            self.CLRPEND15(),
            self.CLRPEND16(),
            self.CLRPEND17(),
            self.CLRPEND18(),
            self.CLRPEND19(),
            self.CLRPEND20(),
            self.CLRPEND21(),
            self.CLRPEND22(),
            self.CLRPEND23(),
            self.CLRPEND24(),
            self.CLRPEND25(),
            self.CLRPEND26(),
            self.CLRPEND27(),
            self.CLRPEND28(),
            self.CLRPEND29(),
            self.CLRPEND30(),
            self.CLRPEND31()
        )
    }
}
#[doc = "Irq 32 to 63 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_ICPR1(pub u32);
impl NVIC_ICPR1 {
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn CLRPEND33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_CLRPEND33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for NVIC_ICPR1 {
    #[inline(always)]
    fn default() -> NVIC_ICPR1 {
        NVIC_ICPR1(0)
    }
}
impl core::fmt::Debug for NVIC_ICPR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_ICPR1")
            .field("CLRPEND32", &self.CLRPEND32())
            .field("CLRPEND33", &self.CLRPEND33())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_ICPR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_ICPR1 {{ CLRPEND32: {=bool:?}, CLRPEND33: {=bool:?}, RESERVED2: {=u32:?} }}",
            self.CLRPEND32(),
            self.CLRPEND33(),
            self.RESERVED2()
        )
    }
}
#[doc = "Irq 0 to 3 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_IPR0(pub u32);
impl NVIC_IPR0 {
    #[doc = "7:0\\] Priority of interrupt 0 (See EVENT:CPUIRQSEL0.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Priority of interrupt 0 (See EVENT:CPUIRQSEL0.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Priority of interrupt 1 (See EVENT:CPUIRQSEL1.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Priority of interrupt 1 (See EVENT:CPUIRQSEL1.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Priority of interrupt 2 (See EVENT:CPUIRQSEL2.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Priority of interrupt 2 (See EVENT:CPUIRQSEL2.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Priority of interrupt 3 (See EVENT:CPUIRQSEL3.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Priority of interrupt 3 (See EVENT:CPUIRQSEL3.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for NVIC_IPR0 {
    #[inline(always)]
    fn default() -> NVIC_IPR0 {
        NVIC_IPR0(0)
    }
}
impl core::fmt::Debug for NVIC_IPR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_IPR0")
            .field("PRI_0", &self.PRI_0())
            .field("PRI_1", &self.PRI_1())
            .field("PRI_2", &self.PRI_2())
            .field("PRI_3", &self.PRI_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_IPR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_IPR0 {{ PRI_0: {=u8:?}, PRI_1: {=u8:?}, PRI_2: {=u8:?}, PRI_3: {=u8:?} }}",
            self.PRI_0(),
            self.PRI_1(),
            self.PRI_2(),
            self.PRI_3()
        )
    }
}
#[doc = "Irq 4 to 7 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_IPR1(pub u32);
impl NVIC_IPR1 {
    #[doc = "7:0\\] Priority of interrupt 4 (See EVENT:CPUIRQSEL4.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Priority of interrupt 4 (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Priority of interrupt 5 (See EVENT:CPUIRQSEL5.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Priority of interrupt 5 (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Priority of interrupt 6 (See EVENT:CPUIRQSEL6.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Priority of interrupt 6 (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Priority of interrupt 7 (See EVENT:CPUIRQSEL7.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Priority of interrupt 7 (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for NVIC_IPR1 {
    #[inline(always)]
    fn default() -> NVIC_IPR1 {
        NVIC_IPR1(0)
    }
}
impl core::fmt::Debug for NVIC_IPR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_IPR1")
            .field("PRI_4", &self.PRI_4())
            .field("PRI_5", &self.PRI_5())
            .field("PRI_6", &self.PRI_6())
            .field("PRI_7", &self.PRI_7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_IPR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_IPR1 {{ PRI_4: {=u8:?}, PRI_5: {=u8:?}, PRI_6: {=u8:?}, PRI_7: {=u8:?} }}",
            self.PRI_4(),
            self.PRI_5(),
            self.PRI_6(),
            self.PRI_7()
        )
    }
}
#[doc = "Irq 8 to 11 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_IPR2(pub u32);
impl NVIC_IPR2 {
    #[doc = "7:0\\] Priority of interrupt 8 (See EVENT:CPUIRQSEL8.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Priority of interrupt 8 (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Priority of interrupt 9 (See EVENT:CPUIRQSEL9.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_9(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Priority of interrupt 9 (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_9(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Priority of interrupt 10 (See EVENT:CPUIRQSEL10.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_10(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Priority of interrupt 10 (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_10(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Priority of interrupt 11 (See EVENT:CPUIRQSEL11.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_11(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Priority of interrupt 11 (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_11(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for NVIC_IPR2 {
    #[inline(always)]
    fn default() -> NVIC_IPR2 {
        NVIC_IPR2(0)
    }
}
impl core::fmt::Debug for NVIC_IPR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_IPR2")
            .field("PRI_8", &self.PRI_8())
            .field("PRI_9", &self.PRI_9())
            .field("PRI_10", &self.PRI_10())
            .field("PRI_11", &self.PRI_11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_IPR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_IPR2 {{ PRI_8: {=u8:?}, PRI_9: {=u8:?}, PRI_10: {=u8:?}, PRI_11: {=u8:?} }}",
            self.PRI_8(),
            self.PRI_9(),
            self.PRI_10(),
            self.PRI_11()
        )
    }
}
#[doc = "Irq 12 to 15 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_IPR3(pub u32);
impl NVIC_IPR3 {
    #[doc = "7:0\\] Priority of interrupt 12 (See EVENT:CPUIRQSEL12.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_12(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Priority of interrupt 12 (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_12(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Priority of interrupt 13 (See EVENT:CPUIRQSEL13.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_13(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Priority of interrupt 13 (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_13(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Priority of interrupt 14 (See EVENT:CPUIRQSEL14.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_14(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Priority of interrupt 14 (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_14(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Priority of interrupt 15 (See EVENT:CPUIRQSEL15.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_15(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Priority of interrupt 15 (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_15(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for NVIC_IPR3 {
    #[inline(always)]
    fn default() -> NVIC_IPR3 {
        NVIC_IPR3(0)
    }
}
impl core::fmt::Debug for NVIC_IPR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_IPR3")
            .field("PRI_12", &self.PRI_12())
            .field("PRI_13", &self.PRI_13())
            .field("PRI_14", &self.PRI_14())
            .field("PRI_15", &self.PRI_15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_IPR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_IPR3 {{ PRI_12: {=u8:?}, PRI_13: {=u8:?}, PRI_14: {=u8:?}, PRI_15: {=u8:?} }}",
            self.PRI_12(),
            self.PRI_13(),
            self.PRI_14(),
            self.PRI_15()
        )
    }
}
#[doc = "Irq 16 to 19 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_IPR4(pub u32);
impl NVIC_IPR4 {
    #[doc = "7:0\\] Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_16(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_16(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_17(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_17(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_18(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_18(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_19(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_19(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for NVIC_IPR4 {
    #[inline(always)]
    fn default() -> NVIC_IPR4 {
        NVIC_IPR4(0)
    }
}
impl core::fmt::Debug for NVIC_IPR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_IPR4")
            .field("PRI_16", &self.PRI_16())
            .field("PRI_17", &self.PRI_17())
            .field("PRI_18", &self.PRI_18())
            .field("PRI_19", &self.PRI_19())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_IPR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_IPR4 {{ PRI_16: {=u8:?}, PRI_17: {=u8:?}, PRI_18: {=u8:?}, PRI_19: {=u8:?} }}",
            self.PRI_16(),
            self.PRI_17(),
            self.PRI_18(),
            self.PRI_19()
        )
    }
}
#[doc = "Irq 20 to 23 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_IPR5(pub u32);
impl NVIC_IPR5 {
    #[doc = "7:0\\] Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_20(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_20(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_21(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_21(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_22(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_22(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_23(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_23(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for NVIC_IPR5 {
    #[inline(always)]
    fn default() -> NVIC_IPR5 {
        NVIC_IPR5(0)
    }
}
impl core::fmt::Debug for NVIC_IPR5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_IPR5")
            .field("PRI_20", &self.PRI_20())
            .field("PRI_21", &self.PRI_21())
            .field("PRI_22", &self.PRI_22())
            .field("PRI_23", &self.PRI_23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_IPR5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_IPR5 {{ PRI_20: {=u8:?}, PRI_21: {=u8:?}, PRI_22: {=u8:?}, PRI_23: {=u8:?} }}",
            self.PRI_20(),
            self.PRI_21(),
            self.PRI_22(),
            self.PRI_23()
        )
    }
}
#[doc = "Irq 24 to 27 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_IPR6(pub u32);
impl NVIC_IPR6 {
    #[doc = "7:0\\] Priority of interrupt 24 (See EVENT:CPUIRQSEL24.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_24(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Priority of interrupt 24 (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_24(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Priority of interrupt 25 (See EVENT:CPUIRQSEL25.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_25(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Priority of interrupt 25 (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_25(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Priority of interrupt 26 (See EVENT:CPUIRQSEL26.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_26(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Priority of interrupt 26 (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_26(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Priority of interrupt 27 (See EVENT:CPUIRQSEL27.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_27(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Priority of interrupt 27 (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_27(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for NVIC_IPR6 {
    #[inline(always)]
    fn default() -> NVIC_IPR6 {
        NVIC_IPR6(0)
    }
}
impl core::fmt::Debug for NVIC_IPR6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_IPR6")
            .field("PRI_24", &self.PRI_24())
            .field("PRI_25", &self.PRI_25())
            .field("PRI_26", &self.PRI_26())
            .field("PRI_27", &self.PRI_27())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_IPR6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_IPR6 {{ PRI_24: {=u8:?}, PRI_25: {=u8:?}, PRI_26: {=u8:?}, PRI_27: {=u8:?} }}",
            self.PRI_24(),
            self.PRI_25(),
            self.PRI_26(),
            self.PRI_27()
        )
    }
}
#[doc = "Irq 28 to 31 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_IPR7(pub u32);
impl NVIC_IPR7 {
    #[doc = "7:0\\] Priority of interrupt 28 (See EVENT:CPUIRQSEL28.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_28(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Priority of interrupt 28 (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_28(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Priority of interrupt 29 (See EVENT:CPUIRQSEL29.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_29(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Priority of interrupt 29 (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_29(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Priority of interrupt 30 (See EVENT:CPUIRQSEL30.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_30(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Priority of interrupt 30 (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_30(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Priority of interrupt 31 (See EVENT:CPUIRQSEL31.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_31(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Priority of interrupt 31 (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_31(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for NVIC_IPR7 {
    #[inline(always)]
    fn default() -> NVIC_IPR7 {
        NVIC_IPR7(0)
    }
}
impl core::fmt::Debug for NVIC_IPR7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_IPR7")
            .field("PRI_28", &self.PRI_28())
            .field("PRI_29", &self.PRI_29())
            .field("PRI_30", &self.PRI_30())
            .field("PRI_31", &self.PRI_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_IPR7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_IPR7 {{ PRI_28: {=u8:?}, PRI_29: {=u8:?}, PRI_30: {=u8:?}, PRI_31: {=u8:?} }}",
            self.PRI_28(),
            self.PRI_29(),
            self.PRI_30(),
            self.PRI_31()
        )
    }
}
#[doc = "Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_IPR8(pub u32);
impl NVIC_IPR8 {
    #[doc = "7:0\\] Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_32(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_32(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_33(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline(always)]
    pub const fn set_PRI_33(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
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
impl Default for NVIC_IPR8 {
    #[inline(always)]
    fn default() -> NVIC_IPR8 {
        NVIC_IPR8(0)
    }
}
impl core::fmt::Debug for NVIC_IPR8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_IPR8")
            .field("PRI_32", &self.PRI_32())
            .field("PRI_33", &self.PRI_33())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_IPR8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_IPR8 {{ PRI_32: {=u8:?}, PRI_33: {=u8:?}, RESERVED16: {=u16:?} }}",
            self.PRI_32(),
            self.PRI_33(),
            self.RESERVED16()
        )
    }
}
#[doc = "Irq 0 to 31 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_ISER0(pub u32);
impl NVIC_ISER0 {
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for NVIC_ISER0 {
    #[inline(always)]
    fn default() -> NVIC_ISER0 {
        NVIC_ISER0(0)
    }
}
impl core::fmt::Debug for NVIC_ISER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_ISER0")
            .field("SETENA0", &self.SETENA0())
            .field("SETENA1", &self.SETENA1())
            .field("SETENA2", &self.SETENA2())
            .field("SETENA3", &self.SETENA3())
            .field("SETENA4", &self.SETENA4())
            .field("SETENA5", &self.SETENA5())
            .field("SETENA6", &self.SETENA6())
            .field("SETENA7", &self.SETENA7())
            .field("SETENA8", &self.SETENA8())
            .field("SETENA9", &self.SETENA9())
            .field("SETENA10", &self.SETENA10())
            .field("SETENA11", &self.SETENA11())
            .field("SETENA12", &self.SETENA12())
            .field("SETENA13", &self.SETENA13())
            .field("SETENA14", &self.SETENA14())
            .field("SETENA15", &self.SETENA15())
            .field("SETENA16", &self.SETENA16())
            .field("SETENA17", &self.SETENA17())
            .field("SETENA18", &self.SETENA18())
            .field("SETENA19", &self.SETENA19())
            .field("SETENA20", &self.SETENA20())
            .field("SETENA21", &self.SETENA21())
            .field("SETENA22", &self.SETENA22())
            .field("SETENA23", &self.SETENA23())
            .field("SETENA24", &self.SETENA24())
            .field("SETENA25", &self.SETENA25())
            .field("SETENA26", &self.SETENA26())
            .field("SETENA27", &self.SETENA27())
            .field("SETENA28", &self.SETENA28())
            .field("SETENA29", &self.SETENA29())
            .field("SETENA30", &self.SETENA30())
            .field("SETENA31", &self.SETENA31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_ISER0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_ISER0 {{ SETENA0: {=bool:?}, SETENA1: {=bool:?}, SETENA2: {=bool:?}, SETENA3: {=bool:?}, SETENA4: {=bool:?}, SETENA5: {=bool:?}, SETENA6: {=bool:?}, SETENA7: {=bool:?}, SETENA8: {=bool:?}, SETENA9: {=bool:?}, SETENA10: {=bool:?}, SETENA11: {=bool:?}, SETENA12: {=bool:?}, SETENA13: {=bool:?}, SETENA14: {=bool:?}, SETENA15: {=bool:?}, SETENA16: {=bool:?}, SETENA17: {=bool:?}, SETENA18: {=bool:?}, SETENA19: {=bool:?}, SETENA20: {=bool:?}, SETENA21: {=bool:?}, SETENA22: {=bool:?}, SETENA23: {=bool:?}, SETENA24: {=bool:?}, SETENA25: {=bool:?}, SETENA26: {=bool:?}, SETENA27: {=bool:?}, SETENA28: {=bool:?}, SETENA29: {=bool:?}, SETENA30: {=bool:?}, SETENA31: {=bool:?} }}",
            self.SETENA0(),
            self.SETENA1(),
            self.SETENA2(),
            self.SETENA3(),
            self.SETENA4(),
            self.SETENA5(),
            self.SETENA6(),
            self.SETENA7(),
            self.SETENA8(),
            self.SETENA9(),
            self.SETENA10(),
            self.SETENA11(),
            self.SETENA12(),
            self.SETENA13(),
            self.SETENA14(),
            self.SETENA15(),
            self.SETENA16(),
            self.SETENA17(),
            self.SETENA18(),
            self.SETENA19(),
            self.SETENA20(),
            self.SETENA21(),
            self.SETENA22(),
            self.SETENA23(),
            self.SETENA24(),
            self.SETENA25(),
            self.SETENA26(),
            self.SETENA27(),
            self.SETENA28(),
            self.SETENA29(),
            self.SETENA30(),
            self.SETENA31()
        )
    }
}
#[doc = "Irq 32 to 63 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_ISER1(pub u32);
impl NVIC_ISER1 {
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENA33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub const fn set_SETENA33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for NVIC_ISER1 {
    #[inline(always)]
    fn default() -> NVIC_ISER1 {
        NVIC_ISER1(0)
    }
}
impl core::fmt::Debug for NVIC_ISER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_ISER1")
            .field("SETENA32", &self.SETENA32())
            .field("SETENA33", &self.SETENA33())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_ISER1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_ISER1 {{ SETENA32: {=bool:?}, SETENA33: {=bool:?}, RESERVED2: {=u32:?} }}",
            self.SETENA32(),
            self.SETENA33(),
            self.RESERVED2()
        )
    }
}
#[doc = "Irq 0 to 31 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_ISPR0(pub u32);
impl NVIC_ISPR0 {
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for NVIC_ISPR0 {
    #[inline(always)]
    fn default() -> NVIC_ISPR0 {
        NVIC_ISPR0(0)
    }
}
impl core::fmt::Debug for NVIC_ISPR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_ISPR0")
            .field("SETPEND0", &self.SETPEND0())
            .field("SETPEND1", &self.SETPEND1())
            .field("SETPEND2", &self.SETPEND2())
            .field("SETPEND3", &self.SETPEND3())
            .field("SETPEND4", &self.SETPEND4())
            .field("SETPEND5", &self.SETPEND5())
            .field("SETPEND6", &self.SETPEND6())
            .field("SETPEND7", &self.SETPEND7())
            .field("SETPEND8", &self.SETPEND8())
            .field("SETPEND9", &self.SETPEND9())
            .field("SETPEND10", &self.SETPEND10())
            .field("SETPEND11", &self.SETPEND11())
            .field("SETPEND12", &self.SETPEND12())
            .field("SETPEND13", &self.SETPEND13())
            .field("SETPEND14", &self.SETPEND14())
            .field("SETPEND15", &self.SETPEND15())
            .field("SETPEND16", &self.SETPEND16())
            .field("SETPEND17", &self.SETPEND17())
            .field("SETPEND18", &self.SETPEND18())
            .field("SETPEND19", &self.SETPEND19())
            .field("SETPEND20", &self.SETPEND20())
            .field("SETPEND21", &self.SETPEND21())
            .field("SETPEND22", &self.SETPEND22())
            .field("SETPEND23", &self.SETPEND23())
            .field("SETPEND24", &self.SETPEND24())
            .field("SETPEND25", &self.SETPEND25())
            .field("SETPEND26", &self.SETPEND26())
            .field("SETPEND27", &self.SETPEND27())
            .field("SETPEND28", &self.SETPEND28())
            .field("SETPEND29", &self.SETPEND29())
            .field("SETPEND30", &self.SETPEND30())
            .field("SETPEND31", &self.SETPEND31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_ISPR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_ISPR0 {{ SETPEND0: {=bool:?}, SETPEND1: {=bool:?}, SETPEND2: {=bool:?}, SETPEND3: {=bool:?}, SETPEND4: {=bool:?}, SETPEND5: {=bool:?}, SETPEND6: {=bool:?}, SETPEND7: {=bool:?}, SETPEND8: {=bool:?}, SETPEND9: {=bool:?}, SETPEND10: {=bool:?}, SETPEND11: {=bool:?}, SETPEND12: {=bool:?}, SETPEND13: {=bool:?}, SETPEND14: {=bool:?}, SETPEND15: {=bool:?}, SETPEND16: {=bool:?}, SETPEND17: {=bool:?}, SETPEND18: {=bool:?}, SETPEND19: {=bool:?}, SETPEND20: {=bool:?}, SETPEND21: {=bool:?}, SETPEND22: {=bool:?}, SETPEND23: {=bool:?}, SETPEND24: {=bool:?}, SETPEND25: {=bool:?}, SETPEND26: {=bool:?}, SETPEND27: {=bool:?}, SETPEND28: {=bool:?}, SETPEND29: {=bool:?}, SETPEND30: {=bool:?}, SETPEND31: {=bool:?} }}",
            self.SETPEND0(),
            self.SETPEND1(),
            self.SETPEND2(),
            self.SETPEND3(),
            self.SETPEND4(),
            self.SETPEND5(),
            self.SETPEND6(),
            self.SETPEND7(),
            self.SETPEND8(),
            self.SETPEND9(),
            self.SETPEND10(),
            self.SETPEND11(),
            self.SETPEND12(),
            self.SETPEND13(),
            self.SETPEND14(),
            self.SETPEND15(),
            self.SETPEND16(),
            self.SETPEND17(),
            self.SETPEND18(),
            self.SETPEND19(),
            self.SETPEND20(),
            self.SETPEND21(),
            self.SETPEND22(),
            self.SETPEND23(),
            self.SETPEND24(),
            self.SETPEND25(),
            self.SETPEND26(),
            self.SETPEND27(),
            self.SETPEND28(),
            self.SETPEND29(),
            self.SETPEND30(),
            self.SETPEND31()
        )
    }
}
#[doc = "Irq 32 to 63 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC_ISPR1(pub u32);
impl NVIC_ISPR1 {
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
    #[must_use]
    #[inline(always)]
    pub const fn SETPEND33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub const fn set_SETPEND33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for NVIC_ISPR1 {
    #[inline(always)]
    fn default() -> NVIC_ISPR1 {
        NVIC_ISPR1(0)
    }
}
impl core::fmt::Debug for NVIC_ISPR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVIC_ISPR1")
            .field("SETPEND32", &self.SETPEND32())
            .field("SETPEND33", &self.SETPEND33())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NVIC_ISPR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NVIC_ISPR1 {{ SETPEND32: {=bool:?}, SETPEND33: {=bool:?}, RESERVED2: {=u32:?} }}",
            self.SETPEND32(),
            self.SETPEND33(),
            self.RESERVED2()
        )
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESERVED0(pub u32);
impl RESERVED0 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RESERVED0 {
    #[inline(always)]
    fn default() -> RESERVED0 {
        RESERVED0(0)
    }
}
impl core::fmt::Debug for RESERVED0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVED0")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESERVED0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RESERVED0 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESERVED000(pub u32);
impl RESERVED000 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RESERVED000 {
    #[inline(always)]
    fn default() -> RESERVED000 {
        RESERVED000(0)
    }
}
impl core::fmt::Debug for RESERVED000 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVED000")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESERVED000 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RESERVED000 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
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
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
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
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESERVED1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RESERVED1 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
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
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
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
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESERVED2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RESERVED2 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESERVED3(pub u32);
impl RESERVED3 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RESERVED3 {
    #[inline(always)]
    fn default() -> RESERVED3 {
        RESERVED3(0)
    }
}
impl core::fmt::Debug for RESERVED3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVED3")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESERVED3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RESERVED3 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESERVED4(pub u32);
impl RESERVED4 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RESERVED4 {
    #[inline(always)]
    fn default() -> RESERVED4 {
        RESERVED4(0)
    }
}
impl core::fmt::Debug for RESERVED4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVED4")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESERVED4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RESERVED4 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESERVED5(pub u32);
impl RESERVED5 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RESERVED5 {
    #[inline(always)]
    fn default() -> RESERVED5 {
        RESERVED5(0)
    }
}
impl core::fmt::Debug for RESERVED5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVED5")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESERVED5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RESERVED5 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESERVED6(pub u32);
impl RESERVED6 {
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RESERVED6 {
    #[inline(always)]
    fn default() -> RESERVED6 {
        RESERVED6(0)
    }
}
impl core::fmt::Debug for RESERVED6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESERVED6")
            .field("RESERVED0", &self.RESERVED0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESERVED6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RESERVED6 {{ RESERVED0: {=u32:?} }}", self.RESERVED0())
    }
}
#[doc = "System Control This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCR(pub u32);
impl SCR {
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
    #[doc = "1:1\\] Sleep on exit when returning from Handler mode to Thread mode. Enables interrupt driven applications to avoid returning to empty main application. 0: Do not sleep when returning to thread mode 1: Sleep on ISR exit."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEEPONEXIT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Sleep on exit when returning from Handler mode to Thread mode. Enables interrupt driven applications to avoid returning to empty main application. 0: Do not sleep when returning to thread mode 1: Sleep on ISR exit."]
    #[inline(always)]
    pub const fn set_SLEEPONEXIT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Controls whether the processor uses sleep or deep sleep as its low power mode."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEEPDEEP(&self) -> super::vals::SLEEPDEEP {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SLEEPDEEP::from_bits(val as u8)
    }
    #[doc = "2:2\\] Controls whether the processor uses sleep or deep sleep as its low power mode."]
    #[inline(always)]
    pub const fn set_SLEEPDEEP(&mut self, val: super::vals::SLEEPDEEP) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Send Event on Pending bit: 0: Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded 1: Enabled events and all interrupts, including disabled interrupts, can wakeup the processor. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction."]
    #[must_use]
    #[inline(always)]
    pub const fn SEVONPEND(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Send Event on Pending bit: 0: Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded 1: Enabled events and all interrupts, including disabled interrupts, can wakeup the processor. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction."]
    #[inline(always)]
    pub const fn set_SEVONPEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED5(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for SCR {
    #[inline(always)]
    fn default() -> SCR {
        SCR(0)
    }
}
impl core::fmt::Debug for SCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR")
            .field("RESERVED0", &self.RESERVED0())
            .field("SLEEPONEXIT", &self.SLEEPONEXIT())
            .field("SLEEPDEEP", &self.SLEEPDEEP())
            .field("RESERVED3", &self.RESERVED3())
            .field("SEVONPEND", &self.SEVONPEND())
            .field("RESERVED5", &self.RESERVED5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SCR {{ RESERVED0: {=bool:?}, SLEEPONEXIT: {=bool:?}, SLEEPDEEP: {:?}, RESERVED3: {=bool:?}, SEVONPEND: {=bool:?}, RESERVED5: {=u32:?} }}",
            self.RESERVED0(),
            self.SLEEPONEXIT(),
            self.SLEEPDEEP(),
            self.RESERVED3(),
            self.SEVONPEND(),
            self.RESERVED5()
        )
    }
}
#[doc = "System Handler Control and State This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SHCSR(pub u32);
impl SHCSR {
    #[doc = "0:0\\] MemManage exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn MEMFAULTACT(&self) -> super::vals::MEMFAULTACT {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MEMFAULTACT::from_bits(val as u8)
    }
    #[doc = "0:0\\] MemManage exception active."]
    #[inline(always)]
    pub const fn set_MEMFAULTACT(&mut self, val: super::vals::MEMFAULTACT) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] BusFault exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSFAULTACT(&self) -> super::vals::BUSFAULTACT {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::BUSFAULTACT::from_bits(val as u8)
    }
    #[doc = "1:1\\] BusFault exception active."]
    #[inline(always)]
    pub const fn set_BUSFAULTACT(&mut self, val: super::vals::BUSFAULTACT) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] UsageFault exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn USGFAULTACT(&self) -> super::vals::USGFAULTACT {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::USGFAULTACT::from_bits(val as u8)
    }
    #[doc = "3:3\\] UsageFault exception active."]
    #[inline(always)]
    pub const fn set_USGFAULTACT(&mut self, val: super::vals::USGFAULTACT) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "6:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "6:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "7:7\\] SVCall active."]
    #[must_use]
    #[inline(always)]
    pub const fn SVCALLACT(&self) -> super::vals::SVCALLACT {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SVCALLACT::from_bits(val as u8)
    }
    #[doc = "7:7\\] SVCall active."]
    #[inline(always)]
    pub const fn set_SVCALLACT(&mut self, val: super::vals::SVCALLACT) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Debug monitor active."]
    #[must_use]
    #[inline(always)]
    pub const fn MONITORACT(&self) -> super::vals::MONITORACT {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::MONITORACT::from_bits(val as u8)
    }
    #[doc = "8:8\\] Debug monitor active."]
    #[inline(always)]
    pub const fn set_MONITORACT(&mut self, val: super::vals::MONITORACT) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] PendSV active 0x0: Not active 0x1: Active."]
    #[must_use]
    #[inline(always)]
    pub const fn PENDSVACT(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] PendSV active 0x0: Not active 0x1: Active."]
    #[inline(always)]
    pub const fn set_PENDSVACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] SysTick active flag. 0x0: Not active 0x1: Active."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSTICKACT(&self) -> super::vals::SYSTICKACT {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SYSTICKACT::from_bits(val as u8)
    }
    #[doc = "11:11\\] SysTick active flag. 0x0: Not active 0x1: Active."]
    #[inline(always)]
    pub const fn set_SYSTICKACT(&mut self, val: super::vals::SYSTICKACT) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Usage fault pending."]
    #[must_use]
    #[inline(always)]
    pub const fn USGFAULTPENDED(&self) -> super::vals::USGFAULTPENDED {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::USGFAULTPENDED::from_bits(val as u8)
    }
    #[doc = "12:12\\] Usage fault pending."]
    #[inline(always)]
    pub const fn set_USGFAULTPENDED(&mut self, val: super::vals::USGFAULTPENDED) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] MemManage exception pending."]
    #[must_use]
    #[inline(always)]
    pub const fn MEMFAULTPENDED(&self) -> super::vals::MEMFAULTPENDED {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::MEMFAULTPENDED::from_bits(val as u8)
    }
    #[doc = "13:13\\] MemManage exception pending."]
    #[inline(always)]
    pub const fn set_MEMFAULTPENDED(&mut self, val: super::vals::MEMFAULTPENDED) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] BusFault pending."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSFAULTPENDED(&self) -> super::vals::BUSFAULTPENDED {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::BUSFAULTPENDED::from_bits(val as u8)
    }
    #[doc = "14:14\\] BusFault pending."]
    #[inline(always)]
    pub const fn set_BUSFAULTPENDED(&mut self, val: super::vals::BUSFAULTPENDED) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] SVCall pending."]
    #[must_use]
    #[inline(always)]
    pub const fn SVCALLPENDED(&self) -> super::vals::SVCALLPENDED {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::SVCALLPENDED::from_bits(val as u8)
    }
    #[doc = "15:15\\] SVCall pending."]
    #[inline(always)]
    pub const fn set_SVCALLPENDED(&mut self, val: super::vals::SVCALLPENDED) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] MemManage fault system handler enable."]
    #[must_use]
    #[inline(always)]
    pub const fn MEMFAULTENA(&self) -> super::vals::MEMFAULTENA {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::MEMFAULTENA::from_bits(val as u8)
    }
    #[doc = "16:16\\] MemManage fault system handler enable."]
    #[inline(always)]
    pub const fn set_MEMFAULTENA(&mut self, val: super::vals::MEMFAULTENA) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Bus fault system handler enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSFAULTENA(&self) -> super::vals::BUSFAULTENA {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::BUSFAULTENA::from_bits(val as u8)
    }
    #[doc = "17:17\\] Bus fault system handler enable."]
    #[inline(always)]
    pub const fn set_BUSFAULTENA(&mut self, val: super::vals::BUSFAULTENA) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Usage fault system handler enable."]
    #[must_use]
    #[inline(always)]
    pub const fn USGFAULTENA(&self) -> super::vals::USGFAULTENA {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::USGFAULTENA::from_bits(val as u8)
    }
    #[doc = "18:18\\] Usage fault system handler enable."]
    #[inline(always)]
    pub const fn set_USGFAULTENA(&mut self, val: super::vals::USGFAULTENA) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "31:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED19(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x1fff;
        val as u16
    }
    #[doc = "31:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED19(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 19usize)) | (((val as u32) & 0x1fff) << 19usize);
    }
}
impl Default for SHCSR {
    #[inline(always)]
    fn default() -> SHCSR {
        SHCSR(0)
    }
}
impl core::fmt::Debug for SHCSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHCSR")
            .field("MEMFAULTACT", &self.MEMFAULTACT())
            .field("BUSFAULTACT", &self.BUSFAULTACT())
            .field("RESERVED2", &self.RESERVED2())
            .field("USGFAULTACT", &self.USGFAULTACT())
            .field("RESERVED4", &self.RESERVED4())
            .field("SVCALLACT", &self.SVCALLACT())
            .field("MONITORACT", &self.MONITORACT())
            .field("RESERVED9", &self.RESERVED9())
            .field("PENDSVACT", &self.PENDSVACT())
            .field("SYSTICKACT", &self.SYSTICKACT())
            .field("USGFAULTPENDED", &self.USGFAULTPENDED())
            .field("MEMFAULTPENDED", &self.MEMFAULTPENDED())
            .field("BUSFAULTPENDED", &self.BUSFAULTPENDED())
            .field("SVCALLPENDED", &self.SVCALLPENDED())
            .field("MEMFAULTENA", &self.MEMFAULTENA())
            .field("BUSFAULTENA", &self.BUSFAULTENA())
            .field("USGFAULTENA", &self.USGFAULTENA())
            .field("RESERVED19", &self.RESERVED19())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SHCSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SHCSR {{ MEMFAULTACT: {:?}, BUSFAULTACT: {:?}, RESERVED2: {=bool:?}, USGFAULTACT: {:?}, RESERVED4: {=u8:?}, SVCALLACT: {:?}, MONITORACT: {:?}, RESERVED9: {=bool:?}, PENDSVACT: {=bool:?}, SYSTICKACT: {:?}, USGFAULTPENDED: {:?}, MEMFAULTPENDED: {:?}, BUSFAULTPENDED: {:?}, SVCALLPENDED: {:?}, MEMFAULTENA: {:?}, BUSFAULTENA: {:?}, USGFAULTENA: {:?}, RESERVED19: {=u16:?} }}",
            self.MEMFAULTACT(),
            self.BUSFAULTACT(),
            self.RESERVED2(),
            self.USGFAULTACT(),
            self.RESERVED4(),
            self.SVCALLACT(),
            self.MONITORACT(),
            self.RESERVED9(),
            self.PENDSVACT(),
            self.SYSTICKACT(),
            self.USGFAULTPENDED(),
            self.MEMFAULTPENDED(),
            self.BUSFAULTPENDED(),
            self.SVCALLPENDED(),
            self.MEMFAULTENA(),
            self.BUSFAULTENA(),
            self.USGFAULTENA(),
            self.RESERVED19()
        )
    }
}
#[doc = "System Handlers 4-7 Priority This register is used to prioritize the following system handlers: Memory manage, Bus fault, and Usage fault. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SHPR1(pub u32);
impl SHPR1 {
    #[doc = "7:0\\] Priority of system handler 4: MemManage."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Priority of system handler 4: MemManage."]
    #[inline(always)]
    pub const fn set_PRI_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Priority of system handler 5: BusFault."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Priority of system handler 5: BusFault."]
    #[inline(always)]
    pub const fn set_PRI_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "23:16\\] Priority of system handler 6. UsageFault."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Priority of system handler 6. UsageFault."]
    #[inline(always)]
    pub const fn set_PRI_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
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
impl Default for SHPR1 {
    #[inline(always)]
    fn default() -> SHPR1 {
        SHPR1(0)
    }
}
impl core::fmt::Debug for SHPR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHPR1")
            .field("PRI_4", &self.PRI_4())
            .field("PRI_5", &self.PRI_5())
            .field("PRI_6", &self.PRI_6())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SHPR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SHPR1 {{ PRI_4: {=u8:?}, PRI_5: {=u8:?}, PRI_6: {=u8:?}, RESERVED24: {=u8:?} }}",
            self.PRI_4(),
            self.PRI_5(),
            self.PRI_6(),
            self.RESERVED24()
        )
    }
}
#[doc = "System Handlers 8-11 Priority This register is used to prioritize the SVC handler. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SHPR2(pub u32);
impl SHPR2 {
    #[doc = "23:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "31:24\\] Priority of system handler 11. SVCall."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_11(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Priority of system handler 11. SVCall."]
    #[inline(always)]
    pub const fn set_PRI_11(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for SHPR2 {
    #[inline(always)]
    fn default() -> SHPR2 {
        SHPR2(0)
    }
}
impl core::fmt::Debug for SHPR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHPR2")
            .field("RESERVED0", &self.RESERVED0())
            .field("PRI_11", &self.PRI_11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SHPR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SHPR2 {{ RESERVED0: {=u32:?}, PRI_11: {=u8:?} }}",
            self.RESERVED0(),
            self.PRI_11()
        )
    }
}
#[doc = "System Handlers 12-15 Priority This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SHPR3(pub u32);
impl SHPR3 {
    #[doc = "7:0\\] Priority of system handler 12. Debug Monitor."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_12(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Priority of system handler 12. Debug Monitor."]
    #[inline(always)]
    pub const fn set_PRI_12(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
    #[doc = "23:16\\] Priority of system handler 14. Pend SV."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_14(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "23:16\\] Priority of system handler 14. Pend SV."]
    #[inline(always)]
    pub const fn set_PRI_14(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "31:24\\] Priority of system handler 15. SysTick exception."]
    #[must_use]
    #[inline(always)]
    pub const fn PRI_15(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "31:24\\] Priority of system handler 15. SysTick exception."]
    #[inline(always)]
    pub const fn set_PRI_15(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for SHPR3 {
    #[inline(always)]
    fn default() -> SHPR3 {
        SHPR3(0)
    }
}
impl core::fmt::Debug for SHPR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHPR3")
            .field("PRI_12", &self.PRI_12())
            .field("RESERVED8", &self.RESERVED8())
            .field("PRI_14", &self.PRI_14())
            .field("PRI_15", &self.PRI_15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SHPR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SHPR3 {{ PRI_12: {=u8:?}, RESERVED8: {=u8:?}, PRI_14: {=u8:?}, PRI_15: {=u8:?} }}",
            self.PRI_12(),
            self.RESERVED8(),
            self.PRI_14(),
            self.PRI_15()
        )
    }
}
#[doc = "SysTick Calibration Value Used to enable software to scale to any required speed using divide and multiply."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STCR(pub u32);
impl STCR {
    #[doc = "23:0\\] An optional Reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. The value read is valid only when core clock is at 48MHz."]
    #[must_use]
    #[inline(always)]
    pub const fn TENMS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] An optional Reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. The value read is valid only when core clock is at 48MHz."]
    #[inline(always)]
    pub const fn set_TENMS(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "29:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED24(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "29:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED24(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "30:30\\] Reads as one. The calibration value is not exactly 10ms because of clock frequency. This could affect its suitability as a software real time clock."]
    #[must_use]
    #[inline(always)]
    pub const fn SKEW(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Reads as one. The calibration value is not exactly 10ms because of clock frequency. This could affect its suitability as a software real time clock."]
    #[inline(always)]
    pub const fn set_SKEW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Reads as one. Indicates that no separate reference clock is provided."]
    #[must_use]
    #[inline(always)]
    pub const fn NOREF(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Reads as one. Indicates that no separate reference clock is provided."]
    #[inline(always)]
    pub const fn set_NOREF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for STCR {
    #[inline(always)]
    fn default() -> STCR {
        STCR(0)
    }
}
impl core::fmt::Debug for STCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STCR")
            .field("TENMS", &self.TENMS())
            .field("RESERVED24", &self.RESERVED24())
            .field("SKEW", &self.SKEW())
            .field("NOREF", &self.NOREF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STCR {{ TENMS: {=u32:?}, RESERVED24: {=u8:?}, SKEW: {=bool:?}, NOREF: {=bool:?} }}",
            self.TENMS(),
            self.RESERVED24(),
            self.SKEW(),
            self.NOREF()
        )
    }
}
#[doc = "SysTick Control and Status This register enables the SysTick features and returns status flags related to SysTick."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STCSR(pub u32);
impl STCSR {
    #[doc = "0:0\\] Enable SysTick counter 0: Counter disabled 1: Counter operates in a multi-shot way. That is, counter loads with the Reload value STRVR.RELOAD and then begins counting down. On reaching 0, it sets COUNTFLAG to 1 and optionally pends the SysTick handler, based on TICKINT. It then loads STRVR.RELOAD again, and begins counting."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Enable SysTick counter 0: Counter disabled 1: Counter operates in a multi-shot way. That is, counter loads with the Reload value STRVR.RELOAD and then begins counting down. On reaching 0, it sets COUNTFLAG to 1 and optionally pends the SysTick handler, based on TICKINT. It then loads STRVR.RELOAD again, and begins counting."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] 0: Counting down to zero does not pend the SysTick handler. Software can use COUNTFLAG to determine if the SysTick handler has ever counted to zero. 1: Counting down to zero pends the SysTick handler."]
    #[must_use]
    #[inline(always)]
    pub const fn TICKINT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] 0: Counting down to zero does not pend the SysTick handler. Software can use COUNTFLAG to determine if the SysTick handler has ever counted to zero. 1: Counting down to zero pends the SysTick handler."]
    #[inline(always)]
    pub const fn set_TICKINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Clock source: 0: External reference clock. 1: Core clock External clock is not available in this device. Writes to this field will be ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKSOURCE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Clock source: 0: External reference clock. 1: Core clock External clock is not available in this device. Writes to this field will be ignored."]
    #[inline(always)]
    pub const fn set_CLKSOURCE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "15:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED3(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x1fff;
        val as u16
    }
    #[doc = "15:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
    }
    #[doc = "16:16\\] Returns 1 if timer counted to 0 since last time this was read. Clears on read by application of any part of the SysTick Control and Status Register. If read by the debugger using the DAP, this bit is cleared on read-only if the MasterType bit in the **AHB-AP** Control Register is set to 0. Otherwise, COUNTFLAG is not changed by the debugger read."]
    #[must_use]
    #[inline(always)]
    pub const fn COUNTFLAG(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Returns 1 if timer counted to 0 since last time this was read. Clears on read by application of any part of the SysTick Control and Status Register. If read by the debugger using the DAP, this bit is cleared on read-only if the MasterType bit in the **AHB-AP** Control Register is set to 0. Otherwise, COUNTFLAG is not changed by the debugger read."]
    #[inline(always)]
    pub const fn set_COUNTFLAG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "31:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "31:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for STCSR {
    #[inline(always)]
    fn default() -> STCSR {
        STCSR(0)
    }
}
impl core::fmt::Debug for STCSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STCSR")
            .field("ENABLE", &self.ENABLE())
            .field("TICKINT", &self.TICKINT())
            .field("CLKSOURCE", &self.CLKSOURCE())
            .field("RESERVED3", &self.RESERVED3())
            .field("COUNTFLAG", &self.COUNTFLAG())
            .field("RESERVED17", &self.RESERVED17())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STCSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STCSR {{ ENABLE: {=bool:?}, TICKINT: {=bool:?}, CLKSOURCE: {=bool:?}, RESERVED3: {=u16:?}, COUNTFLAG: {=bool:?}, RESERVED17: {=u16:?} }}",
            self.ENABLE(),
            self.TICKINT(),
            self.CLKSOURCE(),
            self.RESERVED3(),
            self.COUNTFLAG(),
            self.RESERVED17()
        )
    }
}
#[doc = "SysTick Current Value Read from this register returns the current value of SysTick counter. Writing to this register resets the SysTick counter (as well as STCSR.COUNTFLAG)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STCVR(pub u32);
impl STCVR {
    #[doc = "23:0\\] Current value at the time the register is accessed. No read-modify-write protection is provided, so change with care. Writing to it with any value clears the register to 0. Clearing this register also clears STCSR.COUNTFLAG."]
    #[must_use]
    #[inline(always)]
    pub const fn CURRENT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Current value at the time the register is accessed. No read-modify-write protection is provided, so change with care. Writing to it with any value clears the register to 0. Clearing this register also clears STCSR.COUNTFLAG."]
    #[inline(always)]
    pub const fn set_CURRENT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
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
impl Default for STCVR {
    #[inline(always)]
    fn default() -> STCVR {
        STCVR(0)
    }
}
impl core::fmt::Debug for STCVR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STCVR")
            .field("CURRENT", &self.CURRENT())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STCVR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STCVR {{ CURRENT: {=u32:?}, RESERVED24: {=u8:?} }}",
            self.CURRENT(),
            self.RESERVED24()
        )
    }
}
#[doc = "Software Trigger Interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STIR(pub u32);
impl STIR {
    #[doc = "8:0\\] Interrupt ID field. Writing a value to this bit-field is the same as manually pending an interrupt by setting the corresponding interrupt bit in an Interrupt Set Pending Register in NVIC_ISPR0 or NVIC_ISPR1."]
    #[must_use]
    #[inline(always)]
    pub const fn INTID(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "8:0\\] Interrupt ID field. Writing a value to this bit-field is the same as manually pending an interrupt by setting the corresponding interrupt bit in an Interrupt Set Pending Register in NVIC_ISPR0 or NVIC_ISPR1."]
    #[inline(always)]
    pub const fn set_INTID(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "31:9\\] Software should not rely on the value of a reserved. Write 0."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "31:9\\] Software should not rely on the value of a reserved. Write 0."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for STIR {
    #[inline(always)]
    fn default() -> STIR {
        STIR(0)
    }
}
impl core::fmt::Debug for STIR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STIR")
            .field("INTID", &self.INTID())
            .field("RESERVED9", &self.RESERVED9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STIR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STIR {{ INTID: {=u16:?}, RESERVED9: {=u32:?} }}",
            self.INTID(),
            self.RESERVED9()
        )
    }
}
#[doc = "SysTick Reload Value This register is used to specify the start value to load into the current value register STCVR.CURRENT when the counter reaches 0. It can be any value between 1 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and STCSR.COUNTFLAG are activated when counting from 1 to 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STRVR(pub u32);
impl STRVR {
    #[doc = "23:0\\] Value to load into the SysTick Current Value Register STCVR.CURRENT when the counter reaches 0."]
    #[must_use]
    #[inline(always)]
    pub const fn RELOAD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "23:0\\] Value to load into the SysTick Current Value Register STCVR.CURRENT when the counter reaches 0."]
    #[inline(always)]
    pub const fn set_RELOAD(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
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
impl Default for STRVR {
    #[inline(always)]
    fn default() -> STRVR {
        STRVR(0)
    }
}
impl core::fmt::Debug for STRVR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STRVR")
            .field("RELOAD", &self.RELOAD())
            .field("RESERVED24", &self.RESERVED24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STRVR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STRVR {{ RELOAD: {=u32:?}, RESERVED24: {=u8:?} }}",
            self.RELOAD(),
            self.RESERVED24()
        )
    }
}
#[doc = "Vector Table Offset This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VTOR(pub u32);
impl VTOR {
    #[doc = "6:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "6:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "29:7\\] Bits 29 down to 7 of the vector table base offset."]
    #[must_use]
    #[inline(always)]
    pub const fn TBLOFF(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "29:7\\] Bits 29 down to 7 of the vector table base offset."]
    #[inline(always)]
    pub const fn set_TBLOFF(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 7usize)) | (((val as u32) & 0x007f_ffff) << 7usize);
    }
    #[doc = "31:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED30(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "31:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED30(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for VTOR {
    #[inline(always)]
    fn default() -> VTOR {
        VTOR(0)
    }
}
impl core::fmt::Debug for VTOR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VTOR")
            .field("RESERVED0", &self.RESERVED0())
            .field("TBLOFF", &self.TBLOFF())
            .field("RESERVED30", &self.RESERVED30())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VTOR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VTOR {{ RESERVED0: {=u8:?}, TBLOFF: {=u32:?}, RESERVED30: {=u8:?} }}",
            self.RESERVED0(),
            self.TBLOFF(),
            self.RESERVED30()
        )
    }
}

#[doc = "Cortex-M's Data watchpoint and Trace (DWT)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPU_DWT {
    ptr: *mut u8,
}
unsafe impl Send for CPU_DWT {}
unsafe impl Sync for CPU_DWT {}
impl CPU_DWT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Use the DWT Control Register to enable the DWT unit."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz)."]
    #[inline(always)]
    pub const fn CYCCNT(self) -> crate::common::Reg<regs::CYCCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CPI Count This register is used to count the total number of instruction cycles beyond the first cycle."]
    #[inline(always)]
    pub const fn CPICNT(self) -> crate::common::Reg<regs::CPICNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Exception Overhead Count This register is used to count the total cycles spent in interrupt processing."]
    #[inline(always)]
    pub const fn EXCCNT(self) -> crate::common::Reg<regs::EXCCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Sleep Count This register is used to count the total number of cycles during which the processor is sleeping."]
    #[inline(always)]
    pub const fn SLEEPCNT(self) -> crate::common::Reg<regs::SLEEPCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle."]
    #[inline(always)]
    pub const fn LSUCNT(self) -> crate::common::Reg<regs::LSUCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Fold Count This register is used to count the total number of folded instructions. The counter increments on each instruction which takes 0 cycles."]
    #[inline(always)]
    pub const fn FOLDCNT(self) -> crate::common::Reg<regs::FOLDCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Program Counter Sample This register is used to enable coarse-grained software profiling using a debug agent, without changing the currently executing code. If the core is not in debug state, the value returned is the instruction address of a recently executed instruction. If the core is in debug state, the value returned is 0xFFFFFFFF."]
    #[inline(always)]
    pub const fn PCSR(self) -> crate::common::Reg<regs::PCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Comparator 0 This register is used to write the reference value for comparator 0."]
    #[inline(always)]
    pub const fn COMP0(self) -> crate::common::Reg<regs::COMP0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Mask 0 Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0."]
    #[inline(always)]
    pub const fn MASK0(self) -> crate::common::Reg<regs::MASK0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Function 0 Use the DWT Function Registers 0 to control the operation of the comparator 0. This comparator can: 1. Match against either the PC or the data address. This is controlled by CYCMATCH. This function is only available for comparator 0 (COMP0). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    #[inline(always)]
    pub const fn FUNCTION0(self) -> crate::common::Reg<regs::FUNCTION0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Comparator 1 This register is used to write the reference value for comparator 1."]
    #[inline(always)]
    pub const fn COMP1(self) -> crate::common::Reg<regs::COMP1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Mask 1 Use the DWT Mask Registers 1 to apply a mask to data addresses when matching against COMP1."]
    #[inline(always)]
    pub const fn MASK1(self) -> crate::common::Reg<regs::MASK1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Function 1 Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can: 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    #[inline(always)]
    pub const fn FUNCTION1(self) -> crate::common::Reg<regs::FUNCTION1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Comparator 2 This register is used to write the reference value for comparator 2."]
    #[inline(always)]
    pub const fn COMP2(self) -> crate::common::Reg<regs::COMP2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Mask 2 Use the DWT Mask Registers 2 to apply a mask to data addresses when matching against COMP2."]
    #[inline(always)]
    pub const fn MASK2(self) -> crate::common::Reg<regs::MASK2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Function 2 Use the DWT Function Registers 2 to control the operation of the comparator 2. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    #[inline(always)]
    pub const fn FUNCTION2(self) -> crate::common::Reg<regs::FUNCTION2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Comparator 3 This register is used to write the reference value for comparator 3."]
    #[inline(always)]
    pub const fn COMP3(self) -> crate::common::Reg<regs::COMP3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Mask 3 Use the DWT Mask Registers 3 to apply a mask to data addresses when matching against COMP3."]
    #[inline(always)]
    pub const fn MASK3(self) -> crate::common::Reg<regs::MASK3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Function 3 Use the DWT Function Registers 3 to control the operation of the comparator 3. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    #[inline(always)]
    pub const fn FUNCTION3(self) -> crate::common::Reg<regs::FUNCTION3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
}
pub mod regs;
pub mod vals;

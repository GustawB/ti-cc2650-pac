#[doc = "Event Fabric Component Definition."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVENT {
    ptr: *mut u8,
}
unsafe impl Send for EVENT {}
unsafe impl Sync for EVENT {}
impl EVENT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Output Selection for CPU Interrupt 0."]
    #[inline(always)]
    pub const fn CPUIRQSEL0(self) -> crate::common::Reg<regs::CPUIRQSEL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 1."]
    #[inline(always)]
    pub const fn CPUIRQSEL1(self) -> crate::common::Reg<regs::CPUIRQSEL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 2."]
    #[inline(always)]
    pub const fn CPUIRQSEL2(self) -> crate::common::Reg<regs::CPUIRQSEL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 3."]
    #[inline(always)]
    pub const fn CPUIRQSEL3(self) -> crate::common::Reg<regs::CPUIRQSEL3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 4."]
    #[inline(always)]
    pub const fn CPUIRQSEL4(self) -> crate::common::Reg<regs::CPUIRQSEL4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 5."]
    #[inline(always)]
    pub const fn CPUIRQSEL5(self) -> crate::common::Reg<regs::CPUIRQSEL5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 6."]
    #[inline(always)]
    pub const fn CPUIRQSEL6(self) -> crate::common::Reg<regs::CPUIRQSEL6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 7."]
    #[inline(always)]
    pub const fn CPUIRQSEL7(self) -> crate::common::Reg<regs::CPUIRQSEL7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 8."]
    #[inline(always)]
    pub const fn CPUIRQSEL8(self) -> crate::common::Reg<regs::CPUIRQSEL8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 9."]
    #[inline(always)]
    pub const fn CPUIRQSEL9(self) -> crate::common::Reg<regs::CPUIRQSEL9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 10."]
    #[inline(always)]
    pub const fn CPUIRQSEL10(self) -> crate::common::Reg<regs::CPUIRQSEL10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 11."]
    #[inline(always)]
    pub const fn CPUIRQSEL11(self) -> crate::common::Reg<regs::CPUIRQSEL11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 12."]
    #[inline(always)]
    pub const fn CPUIRQSEL12(self) -> crate::common::Reg<regs::CPUIRQSEL12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 13."]
    #[inline(always)]
    pub const fn CPUIRQSEL13(self) -> crate::common::Reg<regs::CPUIRQSEL13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 14."]
    #[inline(always)]
    pub const fn CPUIRQSEL14(self) -> crate::common::Reg<regs::CPUIRQSEL14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 15."]
    #[inline(always)]
    pub const fn CPUIRQSEL15(self) -> crate::common::Reg<regs::CPUIRQSEL15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 16."]
    #[inline(always)]
    pub const fn CPUIRQSEL16(self) -> crate::common::Reg<regs::CPUIRQSEL16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 17."]
    #[inline(always)]
    pub const fn CPUIRQSEL17(self) -> crate::common::Reg<regs::CPUIRQSEL17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 18."]
    #[inline(always)]
    pub const fn CPUIRQSEL18(self) -> crate::common::Reg<regs::CPUIRQSEL18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 19."]
    #[inline(always)]
    pub const fn CPUIRQSEL19(self) -> crate::common::Reg<regs::CPUIRQSEL19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 20."]
    #[inline(always)]
    pub const fn CPUIRQSEL20(self) -> crate::common::Reg<regs::CPUIRQSEL20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 21."]
    #[inline(always)]
    pub const fn CPUIRQSEL21(self) -> crate::common::Reg<regs::CPUIRQSEL21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 22."]
    #[inline(always)]
    pub const fn CPUIRQSEL22(self) -> crate::common::Reg<regs::CPUIRQSEL22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 23."]
    #[inline(always)]
    pub const fn CPUIRQSEL23(self) -> crate::common::Reg<regs::CPUIRQSEL23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 24."]
    #[inline(always)]
    pub const fn CPUIRQSEL24(self) -> crate::common::Reg<regs::CPUIRQSEL24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 25."]
    #[inline(always)]
    pub const fn CPUIRQSEL25(self) -> crate::common::Reg<regs::CPUIRQSEL25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 26."]
    #[inline(always)]
    pub const fn CPUIRQSEL26(self) -> crate::common::Reg<regs::CPUIRQSEL26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 27."]
    #[inline(always)]
    pub const fn CPUIRQSEL27(self) -> crate::common::Reg<regs::CPUIRQSEL27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 28."]
    #[inline(always)]
    pub const fn CPUIRQSEL28(self) -> crate::common::Reg<regs::CPUIRQSEL28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 29."]
    #[inline(always)]
    pub const fn CPUIRQSEL29(self) -> crate::common::Reg<regs::CPUIRQSEL29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 30."]
    #[inline(always)]
    pub const fn CPUIRQSEL30(self) -> crate::common::Reg<regs::CPUIRQSEL30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 31."]
    #[inline(always)]
    pub const fn CPUIRQSEL31(self) -> crate::common::Reg<regs::CPUIRQSEL31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 32."]
    #[inline(always)]
    pub const fn CPUIRQSEL32(self) -> crate::common::Reg<regs::CPUIRQSEL32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Output Selection for CPU Interrupt 33."]
    #[inline(always)]
    pub const fn CPUIRQSEL33(self) -> crate::common::Reg<regs::CPUIRQSEL33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Output Selection for RFC Event 0."]
    #[inline(always)]
    pub const fn RFCSEL0(self) -> crate::common::Reg<regs::RFCSEL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Output Selection for RFC Event 1."]
    #[inline(always)]
    pub const fn RFCSEL1(self) -> crate::common::Reg<regs::RFCSEL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Output Selection for RFC Event 2."]
    #[inline(always)]
    pub const fn RFCSEL2(self) -> crate::common::Reg<regs::RFCSEL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Output Selection for RFC Event 3."]
    #[inline(always)]
    pub const fn RFCSEL3(self) -> crate::common::Reg<regs::RFCSEL3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Output Selection for RFC Event 4."]
    #[inline(always)]
    pub const fn RFCSEL4(self) -> crate::common::Reg<regs::RFCSEL4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Output Selection for RFC Event 5."]
    #[inline(always)]
    pub const fn RFCSEL5(self) -> crate::common::Reg<regs::RFCSEL5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Output Selection for RFC Event 6."]
    #[inline(always)]
    pub const fn RFCSEL6(self) -> crate::common::Reg<regs::RFCSEL6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Output Selection for RFC Event 7."]
    #[inline(always)]
    pub const fn RFCSEL7(self) -> crate::common::Reg<regs::RFCSEL7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Output Selection for RFC Event 8."]
    #[inline(always)]
    pub const fn RFCSEL8(self) -> crate::common::Reg<regs::RFCSEL8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Output Selection for RFC Event 9."]
    #[inline(always)]
    pub const fn RFCSEL9(self) -> crate::common::Reg<regs::RFCSEL9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Output Selection for GPT0 0."]
    #[inline(always)]
    pub const fn GPT0ACAPTSEL(self) -> crate::common::Reg<regs::GPT0ACAPTSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Output Selection for GPT0 1."]
    #[inline(always)]
    pub const fn GPT0BCAPTSEL(self) -> crate::common::Reg<regs::GPT0BCAPTSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Output Selection for GPT1 0."]
    #[inline(always)]
    pub const fn GPT1ACAPTSEL(self) -> crate::common::Reg<regs::GPT1ACAPTSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Output Selection for GPT1 1."]
    #[inline(always)]
    pub const fn GPT1BCAPTSEL(self) -> crate::common::Reg<regs::GPT1BCAPTSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Output Selection for GPT2 0."]
    #[inline(always)]
    pub const fn GPT2ACAPTSEL(self) -> crate::common::Reg<regs::GPT2ACAPTSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "Output Selection for GPT2 1."]
    #[inline(always)]
    pub const fn GPT2BCAPTSEL(self) -> crate::common::Reg<regs::GPT2BCAPTSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH0SSEL(self) -> crate::common::Reg<regs::UDMACH0SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH0BSEL(self) -> crate::common::Reg<regs::UDMACH0BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 1 SREQ."]
    #[inline(always)]
    pub const fn UDMACH1SSEL(self) -> crate::common::Reg<regs::UDMACH1SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 1 REQ."]
    #[inline(always)]
    pub const fn UDMACH1BSEL(self) -> crate::common::Reg<regs::UDMACH1BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 2 SREQ."]
    #[inline(always)]
    pub const fn UDMACH2SSEL(self) -> crate::common::Reg<regs::UDMACH2SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 2 REQ."]
    #[inline(always)]
    pub const fn UDMACH2BSEL(self) -> crate::common::Reg<regs::UDMACH2BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 3 SREQ."]
    #[inline(always)]
    pub const fn UDMACH3SSEL(self) -> crate::common::Reg<regs::UDMACH3SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 3 REQ."]
    #[inline(always)]
    pub const fn UDMACH3BSEL(self) -> crate::common::Reg<regs::UDMACH3BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 4 SREQ."]
    #[inline(always)]
    pub const fn UDMACH4SSEL(self) -> crate::common::Reg<regs::UDMACH4SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 4 REQ."]
    #[inline(always)]
    pub const fn UDMACH4BSEL(self) -> crate::common::Reg<regs::UDMACH4BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 5 SREQ."]
    #[inline(always)]
    pub const fn UDMACH5SSEL(self) -> crate::common::Reg<regs::UDMACH5SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 5 REQ."]
    #[inline(always)]
    pub const fn UDMACH5BSEL(self) -> crate::common::Reg<regs::UDMACH5BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 6 SREQ."]
    #[inline(always)]
    pub const fn UDMACH6SSEL(self) -> crate::common::Reg<regs::UDMACH6SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 6 REQ."]
    #[inline(always)]
    pub const fn UDMACH6BSEL(self) -> crate::common::Reg<regs::UDMACH6BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 7 SREQ."]
    #[inline(always)]
    pub const fn UDMACH7SSEL(self) -> crate::common::Reg<regs::UDMACH7SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0538usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 7 REQ."]
    #[inline(always)]
    pub const fn UDMACH7BSEL(self) -> crate::common::Reg<regs::UDMACH7BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x053cusize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 8 SREQ Single request is ignored for this channel."]
    #[inline(always)]
    pub const fn UDMACH8SSEL(self) -> crate::common::Reg<regs::UDMACH8SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 8 REQ."]
    #[inline(always)]
    pub const fn UDMACH8BSEL(self) -> crate::common::Reg<regs::UDMACH8BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 9 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS."]
    #[inline(always)]
    pub const fn UDMACH9SSEL(self) -> crate::common::Reg<regs::UDMACH9SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 9 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS."]
    #[inline(always)]
    pub const fn UDMACH9BSEL(self) -> crate::common::Reg<regs::UDMACH9BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 10 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS."]
    #[inline(always)]
    pub const fn UDMACH10SSEL(self) -> crate::common::Reg<regs::UDMACH10SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0550usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 10 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS."]
    #[inline(always)]
    pub const fn UDMACH10BSEL(self) -> crate::common::Reg<regs::UDMACH10BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 11 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS."]
    #[inline(always)]
    pub const fn UDMACH11SSEL(self) -> crate::common::Reg<regs::UDMACH11SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0558usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 11 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS."]
    #[inline(always)]
    pub const fn UDMACH11BSEL(self) -> crate::common::Reg<regs::UDMACH11BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x055cusize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 12 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS."]
    #[inline(always)]
    pub const fn UDMACH12SSEL(self) -> crate::common::Reg<regs::UDMACH12SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 12 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS."]
    #[inline(always)]
    pub const fn UDMACH12BSEL(self) -> crate::common::Reg<regs::UDMACH12BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH13SSEL(self) -> crate::common::Reg<regs::UDMACH13SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 13 REQ."]
    #[inline(always)]
    pub const fn UDMACH13BSEL(self) -> crate::common::Reg<regs::UDMACH13BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH14SSEL(self) -> crate::common::Reg<regs::UDMACH14SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 14 REQ."]
    #[inline(always)]
    pub const fn UDMACH14BSEL(self) -> crate::common::Reg<regs::UDMACH14BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0574usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH15SSEL(self) -> crate::common::Reg<regs::UDMACH15SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0578usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 15 REQ."]
    #[inline(always)]
    pub const fn UDMACH15BSEL(self) -> crate::common::Reg<regs::UDMACH15BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x057cusize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 16 SREQ."]
    #[inline(always)]
    pub const fn UDMACH16SSEL(self) -> crate::common::Reg<regs::UDMACH16SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 16 REQ."]
    #[inline(always)]
    pub const fn UDMACH16BSEL(self) -> crate::common::Reg<regs::UDMACH16BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 17 SREQ."]
    #[inline(always)]
    pub const fn UDMACH17SSEL(self) -> crate::common::Reg<regs::UDMACH17SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 17 REQ."]
    #[inline(always)]
    pub const fn UDMACH17BSEL(self) -> crate::common::Reg<regs::UDMACH17BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058cusize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH18SSEL(self) -> crate::common::Reg<regs::UDMACH18SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH18BSEL(self) -> crate::common::Reg<regs::UDMACH18BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH19SSEL(self) -> crate::common::Reg<regs::UDMACH19SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0598usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH19BSEL(self) -> crate::common::Reg<regs::UDMACH19BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x059cusize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH20SSEL(self) -> crate::common::Reg<regs::UDMACH20SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH20BSEL(self) -> crate::common::Reg<regs::UDMACH20BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a4usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 21 SREQ."]
    #[inline(always)]
    pub const fn UDMACH21SSEL(self) -> crate::common::Reg<regs::UDMACH21SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a8usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 21 REQ."]
    #[inline(always)]
    pub const fn UDMACH21BSEL(self) -> crate::common::Reg<regs::UDMACH21BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05acusize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 22 SREQ."]
    #[inline(always)]
    pub const fn UDMACH22SSEL(self) -> crate::common::Reg<regs::UDMACH22SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b0usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 22 REQ."]
    #[inline(always)]
    pub const fn UDMACH22BSEL(self) -> crate::common::Reg<regs::UDMACH22BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b4usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 23 SREQ."]
    #[inline(always)]
    pub const fn UDMACH23SSEL(self) -> crate::common::Reg<regs::UDMACH23SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b8usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 23 REQ."]
    #[inline(always)]
    pub const fn UDMACH23BSEL(self) -> crate::common::Reg<regs::UDMACH23BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05bcusize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 24 SREQ."]
    #[inline(always)]
    pub const fn UDMACH24SSEL(self) -> crate::common::Reg<regs::UDMACH24SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "Output Selection for DMA Channel 24 REQ."]
    #[inline(always)]
    pub const fn UDMACH24BSEL(self) -> crate::common::Reg<regs::UDMACH24BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c4usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH25SSEL(self) -> crate::common::Reg<regs::UDMACH25SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c8usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH25BSEL(self) -> crate::common::Reg<regs::UDMACH25BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ccusize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH26SSEL(self) -> crate::common::Reg<regs::UDMACH26SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d0usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH26BSEL(self) -> crate::common::Reg<regs::UDMACH26BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d4usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH27SSEL(self) -> crate::common::Reg<regs::UDMACH27SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d8usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH27BSEL(self) -> crate::common::Reg<regs::UDMACH27BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05dcusize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH28SSEL(self) -> crate::common::Reg<regs::UDMACH28SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH28BSEL(self) -> crate::common::Reg<regs::UDMACH28BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e4usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH29SSEL(self) -> crate::common::Reg<regs::UDMACH29SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e8usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH29BSEL(self) -> crate::common::Reg<regs::UDMACH29BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ecusize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH30SSEL(self) -> crate::common::Reg<regs::UDMACH30SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f0usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH30BSEL(self) -> crate::common::Reg<regs::UDMACH30BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f4usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH31SSEL(self) -> crate::common::Reg<regs::UDMACH31SSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f8usize) as _) }
    }
    #[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn UDMACH31BSEL(self) -> crate::common::Reg<regs::UDMACH31BSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05fcusize) as _) }
    }
    #[doc = "Output Selection for GPT3 0."]
    #[inline(always)]
    pub const fn GPT3ACAPTSEL(self) -> crate::common::Reg<regs::GPT3ACAPTSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "Output Selection for GPT3 1."]
    #[inline(always)]
    pub const fn GPT3BCAPTSEL(self) -> crate::common::Reg<regs::GPT3BCAPTSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "Output Selection for AUX Subscriber 0."]
    #[inline(always)]
    pub const fn AUXSEL0(self) -> crate::common::Reg<regs::AUXSEL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "Output Selection for NMI Subscriber 0."]
    #[inline(always)]
    pub const fn CM3NMISEL0(self) -> crate::common::Reg<regs::CM3NMISEL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Output Selection for I2S Subscriber 0."]
    #[inline(always)]
    pub const fn I2SSTMPSEL0(self) -> crate::common::Reg<regs::I2SSTMPSEL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize) as _) }
    }
    #[doc = "Output Selection for FRZ Subscriber The halted debug signal is passed to peripherals such as the General Purpose Timer, Sensor Controller with Digital and Analog Peripherals (AUX), Radio, and RTC. When the system CPU halts, the connected peripherals that have freeze enabled also halt. The programmable output can be set to static values of 0 or 1, and can also be set to pass the halted signal."]
    #[inline(always)]
    pub const fn FRZSEL0(self) -> crate::common::Reg<regs::FRZSEL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a00usize) as _) }
    }
    #[doc = "Set or Clear Software Events."]
    #[inline(always)]
    pub const fn SWEV(self) -> crate::common::Reg<regs::SWEV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f00usize) as _) }
    }
}
pub mod regs;
pub mod vals;

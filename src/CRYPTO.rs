#[doc = "Crypto core with DMA capability and local key storage."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CRYPTO {
    ptr: *mut u8,
}
unsafe impl Send for CRYPTO {}
unsafe impl Sync for CRYPTO {}
impl CRYPTO {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA Channel 0 Control."]
    #[inline(always)]
    pub const fn DMACH0CTL(self) -> crate::common::Reg<regs::DMACH0CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DMA Channel 0 External Address."]
    #[inline(always)]
    pub const fn DMACH0EXTADDR(self) -> crate::common::Reg<regs::DMACH0EXTADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DMA Channel 0 Length."]
    #[inline(always)]
    pub const fn DMACH0LEN(self) -> crate::common::Reg<regs::DMACH0LEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "DMA Controller Status."]
    #[inline(always)]
    pub const fn DMASTAT(self) -> crate::common::Reg<regs::DMASTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "DMA Controller Software Reset."]
    #[inline(always)]
    pub const fn DMASWRESET(self) -> crate::common::Reg<regs::DMASWRESET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "DMA Channel 1 Control."]
    #[inline(always)]
    pub const fn DMACH1CTL(self) -> crate::common::Reg<regs::DMACH1CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "DMA Channel 1 External Address."]
    #[inline(always)]
    pub const fn DMACH1EXTADDR(self) -> crate::common::Reg<regs::DMACH1EXTADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "DMA Channel 1 Length."]
    #[inline(always)]
    pub const fn DMACH1LEN(self) -> crate::common::Reg<regs::DMACH1LEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "DMA Controller Master Configuration."]
    #[inline(always)]
    pub const fn DMABUSCFG(self) -> crate::common::Reg<regs::DMABUSCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "DMA Controller Port Error."]
    #[inline(always)]
    pub const fn DMAPORTERR(self) -> crate::common::Reg<regs::DMAPORTERR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "DMA Controller Version."]
    #[inline(always)]
    pub const fn DMAHWVER(self) -> crate::common::Reg<regs::DMAHWVER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Key Write Area."]
    #[inline(always)]
    pub const fn KEYWRITEAREA(self) -> crate::common::Reg<regs::KEYWRITEAREA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "Key Written Area Status This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and will result in an error."]
    #[inline(always)]
    pub const fn KEYWRITTENAREA(
        self,
    ) -> crate::common::Reg<regs::KEYWRITTENAREA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "Key Size This register defines the size of the keys that are written with DMA."]
    #[inline(always)]
    pub const fn KEYSIZE(self) -> crate::common::Reg<regs::KEYSIZE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "Key Read Area."]
    #[inline(always)]
    pub const fn KEYREADAREA(self) -> crate::common::Reg<regs::KEYREADAREA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "Clear AES_KEY2/GHASH Key."]
    #[inline(always)]
    pub const fn AESKEY2(self) -> crate::common::Reg<regs::AESKEY2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "Clear AES_KEY3."]
    #[inline(always)]
    pub const fn AESKEY3(self) -> crate::common::Reg<regs::AESKEY3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "AES Initialization Vector."]
    #[inline(always)]
    pub const fn AESIV(self) -> crate::common::Reg<regs::AESIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "AES Input/Output Buffer Control."]
    #[inline(always)]
    pub const fn AESCTL(self) -> crate::common::Reg<regs::AESCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0550usize) as _) }
    }
    #[doc = "Crypto Data Length LSW."]
    #[inline(always)]
    pub const fn AESDATALEN0(self) -> crate::common::Reg<regs::AESDATALEN0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
    }
    #[doc = "Crypto Data Length MSW."]
    #[inline(always)]
    pub const fn AESDATALEN1(self) -> crate::common::Reg<regs::AESDATALEN1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0558usize) as _) }
    }
    #[doc = "AES Authentication Length."]
    #[inline(always)]
    pub const fn AESAUTHLEN(self) -> crate::common::Reg<regs::AESAUTHLEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x055cusize) as _) }
    }
    #[doc = "AES Data Input/Output 0."]
    #[inline(always)]
    pub const fn AESDATAIN0(self) -> crate::common::Reg<regs::AESDATAIN0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "Data Input/Output."]
    #[inline(always)]
    pub const fn AESDATAOUT0(self) -> crate::common::Reg<regs::AESDATAOUT0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "AES Data Input/Output 1."]
    #[inline(always)]
    pub const fn AESDATAIN1(self) -> crate::common::Reg<regs::AESDATAIN1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "AES Data Input/Output 3."]
    #[inline(always)]
    pub const fn AESDATAOUT1(self) -> crate::common::Reg<regs::AESDATAOUT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "AES Data Input/Output 2."]
    #[inline(always)]
    pub const fn AESDATAIN2(self) -> crate::common::Reg<regs::AESDATAIN2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "AES Data Input/Output 2."]
    #[inline(always)]
    pub const fn AESDATAOUT2(self) -> crate::common::Reg<regs::AESDATAOUT2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "Data Input/Output."]
    #[inline(always)]
    pub const fn AESDATAIN3(self) -> crate::common::Reg<regs::AESDATAIN3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "AES Data Input/Output 3."]
    #[inline(always)]
    pub const fn AESDATAOUT3(self) -> crate::common::Reg<regs::AESDATAOUT3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "AES Tag Output."]
    #[inline(always)]
    pub const fn AESTAGOUT(self) -> crate::common::Reg<regs::AESTAGOUT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
    }
    #[doc = "Master Algorithm Select This register configures the internal destination of the DMA controller."]
    #[inline(always)]
    pub const fn ALGSEL(self) -> crate::common::Reg<regs::ALGSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "Master Protection Control."]
    #[inline(always)]
    pub const fn DMAPROTCTL(self) -> crate::common::Reg<regs::DMAPROTCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn SWRESET(self) -> crate::common::Reg<regs::SWRESET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0740usize) as _) }
    }
    #[doc = "Control Interrupt Configuration."]
    #[inline(always)]
    pub const fn IRQTYPE(self) -> crate::common::Reg<regs::IRQTYPE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn IRQEN(self) -> crate::common::Reg<regs::IRQEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0784usize) as _) }
    }
    #[doc = "Interrupt Clear."]
    #[inline(always)]
    pub const fn IRQCLR(self) -> crate::common::Reg<regs::IRQCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0788usize) as _) }
    }
    #[doc = "Interrupt Set."]
    #[inline(always)]
    pub const fn IRQSET(self) -> crate::common::Reg<regs::IRQSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x078cusize) as _) }
    }
    #[doc = "Interrupt Status."]
    #[inline(always)]
    pub const fn IRQSTAT(self) -> crate::common::Reg<regs::IRQSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0790usize) as _) }
    }
    #[doc = "CTRL Module Version."]
    #[inline(always)]
    pub const fn HWVER(self) -> crate::common::Reg<regs::HWVER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07fcusize) as _) }
    }
}
pub mod regs;
pub mod vals;

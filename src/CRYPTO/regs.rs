#[doc = "AES Authentication Length."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESAUTHLEN(pub u32);
impl AESAUTHLEN {
    #[doc = "31:0\\] Authentication data length in bytes for combined mode, CCM only. Supported AAD-lengths for CCM are from 0 to (216 - 28) bytes. Once processing with this context is started, this length decrements to zero. Writing this register triggers the engine to start using this context for CCM."]
    #[must_use]
    #[inline(always)]
    pub const fn LEN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Authentication data length in bytes for combined mode, CCM only. Supported AAD-lengths for CCM are from 0 to (216 - 28) bytes. Once processing with this context is started, this length decrements to zero. Writing this register triggers the engine to start using this context for CCM."]
    #[inline(always)]
    pub const fn set_LEN(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AESAUTHLEN {
    #[inline(always)]
    fn default() -> AESAUTHLEN {
        AESAUTHLEN(0)
    }
}
impl core::fmt::Debug for AESAUTHLEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESAUTHLEN")
            .field("LEN", &self.LEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESAUTHLEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AESAUTHLEN {{ LEN: {=u32:?} }}", self.LEN())
    }
}
#[doc = "AES Input/Output Buffer Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESCTL(pub u32);
impl AESCTL {
    #[doc = "0:0\\] If read as 1, this status bit indicates that an AES output block is available to be retrieved by the Host. Writing a 0 clears the bit to zero and indicates that output data is read by the Host. The AES engine can provide a next output data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn OUTPUT_RDY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] If read as 1, this status bit indicates that an AES output block is available to be retrieved by the Host. Writing a 0 clears the bit to zero and indicates that output data is read by the Host. The AES engine can provide a next output data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    pub const fn set_OUTPUT_RDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] If read as 1, this status bit indicates that the 16-byte AES input buffer is empty. The Host is permitted to write the next block of data. Writing a 0 clears the bit to zero and indicates that the AES engine can use the provided input data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. After reset, this bit is 0. After writing a context (note 1), this bit will become 1. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn INPUT_RDY(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] If read as 1, this status bit indicates that the 16-byte AES input buffer is empty. The Host is permitted to write the next block of data. Writing a 0 clears the bit to zero and indicates that the AES engine can use the provided input data block. Writing a 1 to this bit will be ignored. Note: For DMA operations, this bit is automatically controlled by the Crypto peripheral. After reset, this bit is 0. After writing a context (note 1), this bit will become 1. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    pub const fn set_INPUT_RDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Direction. 0 : Decrypt operation is performed. 1 : Encrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Direction. 0 : Decrypt operation is performed. 1 : Encrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "4:3\\] This field specifies the key size. The key size is automatically configured when a new key is loaded via the key store module. 00 = N/A - reserved 01 = 128 bits 10 = N/A - reserved 11 = N/A - reserved For the Crypto peripheral this field is fixed to 128 bits."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY_SIZE(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "4:3\\] This field specifies the key size. The key size is automatically configured when a new key is loaded via the key store module. 00 = N/A - reserved 01 = 128 bits 10 = N/A - reserved 11 = N/A - reserved For the Crypto peripheral this field is fixed to 128 bits."]
    #[inline(always)]
    pub const fn set_KEY_SIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "5:5\\] CBC mode enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CBC(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] CBC mode enable."]
    #[inline(always)]
    pub const fn set_CBC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] AES-CTR mode enable This bit must also be set for CCM, when encryption/decryption is required."]
    #[must_use]
    #[inline(always)]
    pub const fn CTR(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] AES-CTR mode enable This bit must also be set for CCM, when encryption/decryption is required."]
    #[inline(always)]
    pub const fn set_CTR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "8:7\\] Specifies the counter width for AES-CTR mode."]
    #[must_use]
    #[inline(always)]
    pub const fn CTR_WIDTH(&self) -> super::vals::CTR_WIDTH {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::CTR_WIDTH::from_bits(val as u8)
    }
    #[doc = "8:7\\] Specifies the counter width for AES-CTR mode."]
    #[inline(always)]
    pub const fn set_CTR_WIDTH(&mut self, val: super::vals::CTR_WIDTH) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
    }
    #[doc = "14:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x3f;
        val as u8
    }
    #[doc = "14:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
    }
    #[doc = "15:15\\] MAC mode enable. The DIR bit must be set to 1 for this mode. Selecting this mode requires writing the AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW registers after all other registers."]
    #[must_use]
    #[inline(always)]
    pub const fn CBC_MAC(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] MAC mode enable. The DIR bit must be set to 1 for this mode. Selecting this mode requires writing the AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW registers after all other registers."]
    #[inline(always)]
    pub const fn set_CBC_MAC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "17:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "17:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "18:18\\] AES-CCM mode enable. AES-CCM is a combined mode, using AES for both authentication and encryption. Note: Selecting AES-CCM mode requires writing of AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[must_use]
    #[inline(always)]
    pub const fn CCM(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] AES-CCM mode enable. AES-CCM is a combined mode, using AES for both authentication and encryption. Note: Selecting AES-CCM mode requires writing of AESDATALEN1.LEN_MSW and AESDATALEN0.LEN_LSW after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[inline(always)]
    pub const fn set_CCM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "21:19\\] Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM_L plus one. All values are supported."]
    #[must_use]
    #[inline(always)]
    pub const fn CCM_L(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "21:19\\] Defines L that indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM_L plus one. All values are supported."]
    #[inline(always)]
    pub const fn set_CCM_L(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "24:22\\] Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times the value of CCM_M plus one. Note: The Crypto peripheral always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[must_use]
    #[inline(always)]
    pub const fn CCM_M(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "24:22\\] Defines M that indicates the length of the authentication field for CCM operations; the authentication field length equals two times the value of CCM_M plus one. Note: The Crypto peripheral always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    pub const fn set_CCM_M(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "28:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED25(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x0f;
        val as u8
    }
    #[doc = "28:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
    }
    #[doc = "29:29\\] IV must be read before the AES engine can start a new operation."]
    #[must_use]
    #[inline(always)]
    pub const fn SAVE_CONTEXT(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] IV must be read before the AES engine can start a new operation."]
    #[inline(always)]
    pub const fn set_SAVE_CONTEXT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] If read as 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if SAVE_CONTEXT is set to 1. The bit is mutually exclusive with CONTEXT_RDY. Writing 1 clears the bit to zero, indicating the Crypto peripheral can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes will be ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the Crypto peripheral for TAG read DMA operations. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn SAVED_CONTEXT_RDY(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] If read as 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the Host to retrieve. This bit is only asserted if SAVE_CONTEXT is set to 1. The bit is mutually exclusive with CONTEXT_RDY. Writing 1 clears the bit to zero, indicating the Crypto peripheral can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes will be ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the Crypto peripheral for TAG read DMA operations. For typical use, this bit does NOT need to be written, but is used for status reading only. In this case, this status bit is automatically maintained by the Crypto peripheral."]
    #[inline(always)]
    pub const fn set_SAVED_CONTEXT_RDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] If 1, this status bit indicates that the context data registers can be overwritten and the Host is permitted to write the next context. Writing a context means writing either a mode, the crypto length or AESDATALEN1.LEN_MSW, AESDATALEN0.LEN_LSW length registers."]
    #[must_use]
    #[inline(always)]
    pub const fn CONTEXT_RDY(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] If 1, this status bit indicates that the context data registers can be overwritten and the Host is permitted to write the next context. Writing a context means writing either a mode, the crypto length or AESDATALEN1.LEN_MSW, AESDATALEN0.LEN_LSW length registers."]
    #[inline(always)]
    pub const fn set_CONTEXT_RDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for AESCTL {
    #[inline(always)]
    fn default() -> AESCTL {
        AESCTL(0)
    }
}
impl core::fmt::Debug for AESCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESCTL")
            .field("OUTPUT_RDY", &self.OUTPUT_RDY())
            .field("INPUT_RDY", &self.INPUT_RDY())
            .field("DIR", &self.DIR())
            .field("KEY_SIZE", &self.KEY_SIZE())
            .field("CBC", &self.CBC())
            .field("CTR", &self.CTR())
            .field("CTR_WIDTH", &self.CTR_WIDTH())
            .field("RESERVED9", &self.RESERVED9())
            .field("CBC_MAC", &self.CBC_MAC())
            .field("RESERVED", &self.RESERVED())
            .field("CCM", &self.CCM())
            .field("CCM_L", &self.CCM_L())
            .field("CCM_M", &self.CCM_M())
            .field("RESERVED25", &self.RESERVED25())
            .field("SAVE_CONTEXT", &self.SAVE_CONTEXT())
            .field("SAVED_CONTEXT_RDY", &self.SAVED_CONTEXT_RDY())
            .field("CONTEXT_RDY", &self.CONTEXT_RDY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AESCTL {{ OUTPUT_RDY: {=bool:?}, INPUT_RDY: {=bool:?}, DIR: {=bool:?}, KEY_SIZE: {=u8:?}, CBC: {=bool:?}, CTR: {=bool:?}, CTR_WIDTH: {:?}, RESERVED9: {=u8:?}, CBC_MAC: {=bool:?}, RESERVED: {=u8:?}, CCM: {=bool:?}, CCM_L: {=u8:?}, CCM_M: {=u8:?}, RESERVED25: {=u8:?}, SAVE_CONTEXT: {=bool:?}, SAVED_CONTEXT_RDY: {=bool:?}, CONTEXT_RDY: {=bool:?} }}",
            self.OUTPUT_RDY(),
            self.INPUT_RDY(),
            self.DIR(),
            self.KEY_SIZE(),
            self.CBC(),
            self.CTR(),
            self.CTR_WIDTH(),
            self.RESERVED9(),
            self.CBC_MAC(),
            self.RESERVED(),
            self.CCM(),
            self.CCM_L(),
            self.CCM_M(),
            self.RESERVED25(),
            self.SAVE_CONTEXT(),
            self.SAVED_CONTEXT_RDY(),
            self.CONTEXT_RDY()
        )
    }
}
#[doc = "AES Data Input/Output 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESDATAIN0(pub u32);
impl AESDATAIN0 {
    #[doc = "31:0\\] Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[31:0\\] of \\[127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[31:0\\] of \\[127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AESDATAIN0 {
    #[inline(always)]
    fn default() -> AESDATAIN0 {
        AESDATAIN0(0)
    }
}
impl core::fmt::Debug for AESDATAIN0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESDATAIN0")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESDATAIN0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AESDATAIN0 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "AES Data Input/Output 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESDATAIN1(pub u32);
impl AESDATAIN1 {
    #[doc = "31:0\\] Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[63:32\\] of \\[127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[63:32\\] of \\[127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AESDATAIN1 {
    #[inline(always)]
    fn default() -> AESDATAIN1 {
        AESDATAIN1(0)
    }
}
impl core::fmt::Debug for AESDATAIN1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESDATAIN1")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESDATAIN1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AESDATAIN1 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "AES Data Input/Output 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESDATAIN2(pub u32);
impl AESDATAIN2 {
    #[doc = "31:0\\] Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[95:64\\] of \\[127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[95:64\\] of \\[127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AESDATAIN2 {
    #[inline(always)]
    fn default() -> AESDATAIN2 {
        AESDATAIN2(0)
    }
}
impl core::fmt::Debug for AESDATAIN2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESDATAIN2")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESDATAIN2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AESDATAIN2 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Data Input/Output."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESDATAIN3(pub u32);
impl AESDATAIN3 {
    #[doc = "31:0\\] Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[127:96\\] of \\[127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Data registers for input block data to the Crypto peripheral. These bits = AES Input Data\\[127:96\\] of \\[127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host write operation, these registers must be written with the 128-bit input block for the next AES operation. Writing at a word-aligned offset within this address range will store the word (4 bytes) of data into the corresponding position of 4-word deep (16 bytes = 128-bit AES block) data input buffer. This buffer is used for the next AES operation. If the last data block is not completely filled with valid data (see notes below), it is allowed to write only the words with valid data. Next AES operation is triggered by writing to AESCTL.INPUT_RDY. Note: AES typically operates on 128 bits block multiple input data. The CTR, GCM and CCM modes form an exception. The last block of a CTR-mode message may contain less than 128 bits (refer to \\[NIST 800-38A\\]): 0 < n <= 128 bits. For GCM/CCM, the last block of both AAD and message data may contain less than 128 bits (refer to \\[NIST 800-38D\\]). The Crypto peripheral automatically pads or masks misaligned ending data blocks with zeroes for GCM, CCM and CBC-MAC. For CTR mode, the remaining data in an unaligned data block is ignored."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AESDATAIN3 {
    #[inline(always)]
    fn default() -> AESDATAIN3 {
        AESDATAIN3(0)
    }
}
impl core::fmt::Debug for AESDATAIN3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESDATAIN3")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESDATAIN3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AESDATAIN3 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "Crypto Data Length LSW."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESDATALEN0(pub u32);
impl AESDATALEN0 {
    #[doc = "31:0\\] Used to write the Length values to the Crypto peripheral. This register contains bits \\[31:0\\] of the combined data length."]
    #[must_use]
    #[inline(always)]
    pub const fn LEN_LSW(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Used to write the Length values to the Crypto peripheral. This register contains bits \\[31:0\\] of the combined data length."]
    #[inline(always)]
    pub const fn set_LEN_LSW(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AESDATALEN0 {
    #[inline(always)]
    fn default() -> AESDATALEN0 {
        AESDATALEN0(0)
    }
}
impl core::fmt::Debug for AESDATALEN0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESDATALEN0")
            .field("LEN_LSW", &self.LEN_LSW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESDATALEN0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AESDATALEN0 {{ LEN_LSW: {=u32:?} }}", self.LEN_LSW())
    }
}
#[doc = "Crypto Data Length MSW."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESDATALEN1(pub u32);
impl AESDATALEN1 {
    #[doc = "28:0\\] Bits \\[60:32\\] of the combined data length. Bits \\[60:0\\] of the crypto length registers AESDATALEN1 and AESDATALEN0 store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to zero. Data lengths up to (2^61 - 1) bytes are allowed. For GCM, any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 - 2, resulting in a maximum number of bytes of 2^36 - 32. Writing to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN.LEN. All modes must have a length > 0. For the combined modes, it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the Crypto peripheral. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn LEN_MSW(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "28:0\\] Bits \\[60:32\\] of the combined data length. Bits \\[60:0\\] of the crypto length registers AESDATALEN1 and AESDATALEN0 store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to zero. Data lengths up to (2^61 - 1) bytes are allowed. For GCM, any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 - 2, resulting in a maximum number of bytes of 2^36 - 32. Writing to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN.LEN. All modes must have a length > 0. For the combined modes, it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the Crypto peripheral. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes."]
    #[inline(always)]
    pub const fn set_LEN_MSW(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
    #[doc = "31:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "31:29\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for AESDATALEN1 {
    #[inline(always)]
    fn default() -> AESDATALEN1 {
        AESDATALEN1(0)
    }
}
impl core::fmt::Debug for AESDATALEN1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESDATALEN1")
            .field("LEN_MSW", &self.LEN_MSW())
            .field("RESERVED", &self.RESERVED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESDATALEN1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AESDATALEN1 {{ LEN_MSW: {=u32:?}, RESERVED: {=u8:?} }}",
            self.LEN_MSW(),
            self.RESERVED()
        )
    }
}
#[doc = "Data Input/Output."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESDATAOUT0(pub u32);
impl AESDATAOUT0 {
    #[doc = "31:0\\] Data register 0 for output block data from the Crypto peripheral. These bits = AES Output Data\\[31:0\\] of {127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range will read one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, AESCTL.OUTPUT_RDY must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Data register 0 for output block data from the Crypto peripheral. These bits = AES Output Data\\[31:0\\] of {127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range will read one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, AESCTL.OUTPUT_RDY must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AESDATAOUT0 {
    #[inline(always)]
    fn default() -> AESDATAOUT0 {
        AESDATAOUT0(0)
    }
}
impl core::fmt::Debug for AESDATAOUT0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESDATAOUT0")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESDATAOUT0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AESDATAOUT0 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "AES Data Input/Output 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESDATAOUT1(pub u32);
impl AESDATAOUT1 {
    #[doc = "31:0\\] Data registers for output block data from the Crypto peripheral. These bits = AES Output Data\\[63:32\\] of \\[127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range will read one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, AESCTL.OUTPUT_RDY must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Data registers for output block data from the Crypto peripheral. These bits = AES Output Data\\[63:32\\] of \\[127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range will read one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, AESCTL.OUTPUT_RDY must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AESDATAOUT1 {
    #[inline(always)]
    fn default() -> AESDATAOUT1 {
        AESDATAOUT1(0)
    }
}
impl core::fmt::Debug for AESDATAOUT1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESDATAOUT1")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESDATAOUT1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AESDATAOUT1 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "AES Data Input/Output 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESDATAOUT2(pub u32);
impl AESDATAOUT2 {
    #[doc = "31:0\\] Data registers for output block data from the Crypto peripheral. These bits = AES Output Data\\[95:64\\] of \\[127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range will read one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, AESCTL.OUTPUT_RDY must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Data registers for output block data from the Crypto peripheral. These bits = AES Output Data\\[95:64\\] of \\[127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range will read one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, AESCTL.OUTPUT_RDY must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AESDATAOUT2 {
    #[inline(always)]
    fn default() -> AESDATAOUT2 {
        AESDATAOUT2(0)
    }
}
impl core::fmt::Debug for AESDATAOUT2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESDATAOUT2")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESDATAOUT2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AESDATAOUT2 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "AES Data Input/Output 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESDATAOUT3(pub u32);
impl AESDATAOUT3 {
    #[doc = "31:0\\] Data registers for output block data from the Crypto peripheral. These bits = AES Output Data\\[127:96\\] of \\[127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range will read one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, AESCTL.OUTPUT_RDY must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Data registers for output block data from the Crypto peripheral. These bits = AES Output Data\\[127:96\\] of \\[127:0\\] For normal operations, this register is not used, since data input and output is transferred from and to the AES engine via DMA. For a Host read operation, these registers contain the 128-bit output block from the latest AES operation. Reading from a word-aligned offset within this address range will read one word (4 bytes) of data out the 4-word deep (16 bytes = 128-bits AES block) data output buffer. The words (4 words, one full block) should be read before the core will move the next block to the data output buffer. To empty the data output buffer, AESCTL.OUTPUT_RDY must be written. For the modes with authentication (CBC-MAC, GCM and CCM), the invalid (message) bytes/words can be written with any data. Note: The AAD / authentication only data is not copied to the output buffer but only used for authentication."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AESDATAOUT3 {
    #[inline(always)]
    fn default() -> AESDATAOUT3 {
        AESDATAOUT3(0)
    }
}
impl core::fmt::Debug for AESDATAOUT3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESDATAOUT3")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESDATAOUT3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AESDATAOUT3 {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = "AES Initialization Vector."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESIV(pub u32);
impl AESIV {
    #[doc = "31:0\\] The interpretation of this field depends on the crypto operation mode."]
    #[must_use]
    #[inline(always)]
    pub const fn IV(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] The interpretation of this field depends on the crypto operation mode."]
    #[inline(always)]
    pub const fn set_IV(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AESIV {
    #[inline(always)]
    fn default() -> AESIV {
        AESIV(0)
    }
}
impl core::fmt::Debug for AESIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESIV").field("IV", &self.IV()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AESIV {{ IV: {=u32:?} }}", self.IV())
    }
}
#[doc = "Clear AES_KEY2/GHASH Key."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESKEY2(pub u32);
impl AESKEY2 {
    #[doc = "31:0\\] AESKEY2.* bits 31+x:0+x or AES_GHASH_H.* bits 31+x:0+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register array. The interpretation of this field depends on the crypto operation mode."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] AESKEY2.* bits 31+x:0+x or AES_GHASH_H.* bits 31+x:0+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register array. The interpretation of this field depends on the crypto operation mode."]
    #[inline(always)]
    pub const fn set_KEY2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AESKEY2 {
    #[inline(always)]
    fn default() -> AESKEY2 {
        AESKEY2(0)
    }
}
impl core::fmt::Debug for AESKEY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESKEY2")
            .field("KEY2", &self.KEY2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESKEY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AESKEY2 {{ KEY2: {=u32:?} }}", self.KEY2())
    }
}
#[doc = "Clear AES_KEY3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESKEY3(pub u32);
impl AESKEY3 {
    #[doc = "31:0\\] AESKEY3.* bits 31+x:0+x or AESKEY2.* bits 159+x:128+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register arrary. The interpretation of this field depends on the crypto operation mode."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] AESKEY3.* bits 31+x:0+x or AESKEY2.* bits 159+x:128+x, where x = 0, 32, 64, 96 ordered from the LSW entry of this 4-deep register arrary. The interpretation of this field depends on the crypto operation mode."]
    #[inline(always)]
    pub const fn set_KEY3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AESKEY3 {
    #[inline(always)]
    fn default() -> AESKEY3 {
        AESKEY3(0)
    }
}
impl core::fmt::Debug for AESKEY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESKEY3")
            .field("KEY3", &self.KEY3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESKEY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AESKEY3 {{ KEY3: {=u32:?} }}", self.KEY3())
    }
}
#[doc = "AES Tag Output."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AESTAGOUT(pub u32);
impl AESTAGOUT {
    #[doc = "31:0\\] This register contains the authentication TAG for the combined and authentication-only modes."]
    #[must_use]
    #[inline(always)]
    pub const fn TAG(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] This register contains the authentication TAG for the combined and authentication-only modes."]
    #[inline(always)]
    pub const fn set_TAG(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AESTAGOUT {
    #[inline(always)]
    fn default() -> AESTAGOUT {
        AESTAGOUT(0)
    }
}
impl core::fmt::Debug for AESTAGOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AESTAGOUT")
            .field("TAG", &self.TAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AESTAGOUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AESTAGOUT {{ TAG: {=u32:?} }}", self.TAG())
    }
}
#[doc = "Master Algorithm Select This register configures the internal destination of the DMA controller."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ALGSEL(pub u32);
impl ALGSEL {
    #[doc = "0:0\\] If set to 1, selects the Key Store to be loaded via DMA. The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY_STORE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] If set to 1, selects the Key Store to be loaded via DMA. The maximum transfer size to DMA engine is set to 32 bytes (however transfers of 16, 24 and 32 bytes are allowed)."]
    #[inline(always)]
    pub const fn set_KEY_STORE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] If set to 1, the AES data is loaded via DMA Both Read and Write maximum transfer size to DMA engine is set to 16 bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn AES(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] If set to 1, the AES data is loaded via DMA Both Read and Write maximum transfer size to DMA engine is set to 16 bytes."]
    #[inline(always)]
    pub const fn set_AES(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "30:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "30:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 2usize)) | (((val as u32) & 0x1fff_ffff) << 2usize);
    }
    #[doc = "31:31\\] If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest)."]
    #[must_use]
    #[inline(always)]
    pub const fn TAG(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] If this bit is cleared to 0, the DMA operation involves only data. If this bit is set, the DMA operation includes a TAG (Authentication Result / Digest)."]
    #[inline(always)]
    pub const fn set_TAG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ALGSEL {
    #[inline(always)]
    fn default() -> ALGSEL {
        ALGSEL(0)
    }
}
impl core::fmt::Debug for ALGSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALGSEL")
            .field("KEY_STORE", &self.KEY_STORE())
            .field("AES", &self.AES())
            .field("RESERVED2", &self.RESERVED2())
            .field("TAG", &self.TAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ALGSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ALGSEL {{ KEY_STORE: {=bool:?}, AES: {=bool:?}, RESERVED2: {=u32:?}, TAG: {=bool:?} }}",
            self.KEY_STORE(),
            self.AES(),
            self.RESERVED2(),
            self.TAG()
        )
    }
}
#[doc = "DMA Controller Master Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMABUSCFG(pub u32);
impl DMABUSCFG {
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
    #[doc = "8:8\\] Endianess for the AHB master."]
    #[must_use]
    #[inline(always)]
    pub const fn AHB_MST1_BIGEND(&self) -> super::vals::AHB_MST1_BIGEND {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::AHB_MST1_BIGEND::from_bits(val as u8)
    }
    #[doc = "8:8\\] Endianess for the AHB master."]
    #[inline(always)]
    pub const fn set_AHB_MST1_BIGEND(&mut self, val: super::vals::AHB_MST1_BIGEND) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Locked transform on AHB."]
    #[must_use]
    #[inline(always)]
    pub const fn AHB_MST1_LOCK_EN(&self) -> super::vals::AHB_MST1_LOCK_EN {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::AHB_MST1_LOCK_EN::from_bits(val as u8)
    }
    #[doc = "9:9\\] Locked transform on AHB."]
    #[inline(always)]
    pub const fn set_AHB_MST1_LOCK_EN(&mut self, val: super::vals::AHB_MST1_LOCK_EN) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Burst length type of AHB transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn AHB_MST1_INCR_EN(&self) -> super::vals::AHB_MST1_INCR_EN {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::AHB_MST1_INCR_EN::from_bits(val as u8)
    }
    #[doc = "10:10\\] Burst length type of AHB transfer."]
    #[inline(always)]
    pub const fn set_AHB_MST1_INCR_EN(&mut self, val: super::vals::AHB_MST1_INCR_EN) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Idle transfer insertion between consecutive burst transfers on AHB."]
    #[must_use]
    #[inline(always)]
    pub const fn AHB_MST1_IDLE_EN(&self) -> super::vals::AHB_MST1_IDLE_EN {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::AHB_MST1_IDLE_EN::from_bits(val as u8)
    }
    #[doc = "11:11\\] Idle transfer insertion between consecutive burst transfers on AHB."]
    #[inline(always)]
    pub const fn set_AHB_MST1_IDLE_EN(&mut self, val: super::vals::AHB_MST1_IDLE_EN) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "15:12\\] Maximum burst size that can be performed on the AHB bus."]
    #[must_use]
    #[inline(always)]
    pub const fn AHB_MST1_BURST_SIZE(&self) -> super::vals::AHB_MST1_BURST_SIZE {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::AHB_MST1_BURST_SIZE::from_bits(val as u8)
    }
    #[doc = "15:12\\] Maximum burst size that can be performed on the AHB bus."]
    #[inline(always)]
    pub const fn set_AHB_MST1_BURST_SIZE(&mut self, val: super::vals::AHB_MST1_BURST_SIZE) {
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
impl Default for DMABUSCFG {
    #[inline(always)]
    fn default() -> DMABUSCFG {
        DMABUSCFG(0)
    }
}
impl core::fmt::Debug for DMABUSCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMABUSCFG")
            .field("RESERVED0", &self.RESERVED0())
            .field("AHB_MST1_BIGEND", &self.AHB_MST1_BIGEND())
            .field("AHB_MST1_LOCK_EN", &self.AHB_MST1_LOCK_EN())
            .field("AHB_MST1_INCR_EN", &self.AHB_MST1_INCR_EN())
            .field("AHB_MST1_IDLE_EN", &self.AHB_MST1_IDLE_EN())
            .field("AHB_MST1_BURST_SIZE", &self.AHB_MST1_BURST_SIZE())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMABUSCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMABUSCFG {{ RESERVED0: {=u8:?}, AHB_MST1_BIGEND: {:?}, AHB_MST1_LOCK_EN: {:?}, AHB_MST1_INCR_EN: {:?}, AHB_MST1_IDLE_EN: {:?}, AHB_MST1_BURST_SIZE: {:?}, RESERVED16: {=u16:?} }}",
            self.RESERVED0(),
            self.AHB_MST1_BIGEND(),
            self.AHB_MST1_LOCK_EN(),
            self.AHB_MST1_INCR_EN(),
            self.AHB_MST1_IDLE_EN(),
            self.AHB_MST1_BURST_SIZE(),
            self.RESERVED16()
        )
    }
}
#[doc = "DMA Channel 0 Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACH0CTL(pub u32);
impl DMACH0CTL {
    #[doc = "0:0\\] DMA Channel 0 Control."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> super::vals::DMACH0CTL_EN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DMACH0CTL_EN::from_bits(val as u8)
    }
    #[doc = "0:0\\] DMA Channel 0 Control."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: super::vals::DMACH0CTL_EN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIO(&self) -> super::vals::DMACH0CTL_PRIO {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DMACH0CTL_PRIO::from_bits(val as u8)
    }
    #[doc = "1:1\\] Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
    #[inline(always)]
    pub const fn set_PRIO(&mut self, val: super::vals::DMACH0CTL_PRIO) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
impl Default for DMACH0CTL {
    #[inline(always)]
    fn default() -> DMACH0CTL {
        DMACH0CTL(0)
    }
}
impl core::fmt::Debug for DMACH0CTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACH0CTL")
            .field("EN", &self.EN())
            .field("PRIO", &self.PRIO())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACH0CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACH0CTL {{ EN: {:?}, PRIO: {:?}, RESERVED2: {=u32:?} }}",
            self.EN(),
            self.PRIO(),
            self.RESERVED2()
        )
    }
}
#[doc = "DMA Channel 0 External Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACH0EXTADDR(pub u32);
impl DMACH0EXTADDR {
    #[doc = "31:0\\] Channel external address value. Holds the last updated external address after being sent to the master interface."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Channel external address value. Holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    pub const fn set_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMACH0EXTADDR {
    #[inline(always)]
    fn default() -> DMACH0EXTADDR {
        DMACH0EXTADDR(0)
    }
}
impl core::fmt::Debug for DMACH0EXTADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACH0EXTADDR")
            .field("ADDR", &self.ADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACH0EXTADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMACH0EXTADDR {{ ADDR: {=u32:?} }}", self.ADDR())
    }
}
#[doc = "DMA Channel 0 Length."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACH0LEN(pub u32);
impl DMACH0LEN {
    #[doc = "15:0\\] DMA transfer length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Writing a non-zero value to this register field starts the transfer if the channel is enabled by setting DMACH0CTL.EN."]
    #[must_use]
    #[inline(always)]
    pub const fn LEN(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] DMA transfer length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Writing a non-zero value to this register field starts the transfer if the channel is enabled by setting DMACH0CTL.EN."]
    #[inline(always)]
    pub const fn set_LEN(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for DMACH0LEN {
    #[inline(always)]
    fn default() -> DMACH0LEN {
        DMACH0LEN(0)
    }
}
impl core::fmt::Debug for DMACH0LEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACH0LEN")
            .field("LEN", &self.LEN())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACH0LEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACH0LEN {{ LEN: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.LEN(),
            self.RESERVED16()
        )
    }
}
#[doc = "DMA Channel 1 Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACH1CTL(pub u32);
impl DMACH1CTL {
    #[doc = "0:0\\] Channel enable: Note: Disabling an active channel will interrupt the DMA operation. The ongoing block transfer will be completed, but no new transfers will be requested."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> super::vals::DMACH1CTL_EN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DMACH1CTL_EN::from_bits(val as u8)
    }
    #[doc = "0:0\\] Channel enable: Note: Disabling an active channel will interrupt the DMA operation. The ongoing block transfer will be completed, but no new transfers will be requested."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: super::vals::DMACH1CTL_EN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIO(&self) -> super::vals::DMACH1CTL_PRIO {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DMACH1CTL_PRIO::from_bits(val as u8)
    }
    #[doc = "1:1\\] Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
    #[inline(always)]
    pub const fn set_PRIO(&mut self, val: super::vals::DMACH1CTL_PRIO) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
impl Default for DMACH1CTL {
    #[inline(always)]
    fn default() -> DMACH1CTL {
        DMACH1CTL(0)
    }
}
impl core::fmt::Debug for DMACH1CTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACH1CTL")
            .field("EN", &self.EN())
            .field("PRIO", &self.PRIO())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACH1CTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACH1CTL {{ EN: {:?}, PRIO: {:?}, RESERVED2: {=u32:?} }}",
            self.EN(),
            self.PRIO(),
            self.RESERVED2()
        )
    }
}
#[doc = "DMA Channel 1 External Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACH1EXTADDR(pub u32);
impl DMACH1EXTADDR {
    #[doc = "31:0\\] Channel external address value. Holds the last updated external address after being sent to the master interface."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "31:0\\] Channel external address value. Holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    pub const fn set_ADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMACH1EXTADDR {
    #[inline(always)]
    fn default() -> DMACH1EXTADDR {
        DMACH1EXTADDR(0)
    }
}
impl core::fmt::Debug for DMACH1EXTADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACH1EXTADDR")
            .field("ADDR", &self.ADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACH1EXTADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMACH1EXTADDR {{ ADDR: {=u32:?} }}", self.ADDR())
    }
}
#[doc = "DMA Channel 1 Length."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMACH1LEN(pub u32);
impl DMACH1LEN {
    #[doc = "15:0\\] DMA transfer length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Writing a non-zero value to this register field starts the transfer if the channel is enabled by setting DMACH1CTL.EN."]
    #[must_use]
    #[inline(always)]
    pub const fn LEN(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "15:0\\] DMA transfer length in bytes. During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Writing a non-zero value to this register field starts the transfer if the channel is enabled by setting DMACH1CTL.EN."]
    #[inline(always)]
    pub const fn set_LEN(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for DMACH1LEN {
    #[inline(always)]
    fn default() -> DMACH1LEN {
        DMACH1LEN(0)
    }
}
impl core::fmt::Debug for DMACH1LEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACH1LEN")
            .field("LEN", &self.LEN())
            .field("RESERVED16", &self.RESERVED16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMACH1LEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMACH1LEN {{ LEN: {=u16:?}, RESERVED16: {=u16:?} }}",
            self.LEN(),
            self.RESERVED16()
        )
    }
}
#[doc = "DMA Controller Version."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMAHWVER(pub u32);
impl DMAHWVER {
    #[doc = "7:0\\] Version number of the DMA Controller (209)."]
    #[must_use]
    #[inline(always)]
    pub const fn VER_NUM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] Version number of the DMA Controller (209)."]
    #[inline(always)]
    pub const fn set_VER_NUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] Bit-by-bit complement of the VER_NUM field bits."]
    #[must_use]
    #[inline(always)]
    pub const fn VER_NUM_COMPL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] Bit-by-bit complement of the VER_NUM field bits."]
    #[inline(always)]
    pub const fn set_VER_NUM_COMPL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "19:16\\] Patch level."]
    #[must_use]
    #[inline(always)]
    pub const fn HW_PATCH_LVL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Patch level."]
    #[inline(always)]
    pub const fn set_HW_PATCH_LVL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "23:20\\] Minor version number."]
    #[must_use]
    #[inline(always)]
    pub const fn HW_MINOR_VER(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Minor version number."]
    #[inline(always)]
    pub const fn set_HW_MINOR_VER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "27:24\\] Major version number."]
    #[must_use]
    #[inline(always)]
    pub const fn HW_MAJOR_VER(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Major version number."]
    #[inline(always)]
    pub const fn set_HW_MAJOR_VER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED28(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED28(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for DMAHWVER {
    #[inline(always)]
    fn default() -> DMAHWVER {
        DMAHWVER(0)
    }
}
impl core::fmt::Debug for DMAHWVER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAHWVER")
            .field("VER_NUM", &self.VER_NUM())
            .field("VER_NUM_COMPL", &self.VER_NUM_COMPL())
            .field("HW_PATCH_LVL", &self.HW_PATCH_LVL())
            .field("HW_MINOR_VER", &self.HW_MINOR_VER())
            .field("HW_MAJOR_VER", &self.HW_MAJOR_VER())
            .field("RESERVED28", &self.RESERVED28())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMAHWVER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMAHWVER {{ VER_NUM: {=u8:?}, VER_NUM_COMPL: {=u8:?}, HW_PATCH_LVL: {=u8:?}, HW_MINOR_VER: {=u8:?}, HW_MAJOR_VER: {=u8:?}, RESERVED28: {=u8:?} }}",
            self.VER_NUM(),
            self.VER_NUM_COMPL(),
            self.HW_PATCH_LVL(),
            self.HW_MINOR_VER(),
            self.HW_MAJOR_VER(),
            self.RESERVED28()
        )
    }
}
#[doc = "DMA Controller Port Error."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMAPORTERR(pub u32);
impl DMAPORTERR {
    #[doc = "8:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "8:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "9:9\\] Indicates which channel was serviced last (channel 0 or channel 1) by the AHB master port."]
    #[must_use]
    #[inline(always)]
    pub const fn LAST_CH(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Indicates which channel was serviced last (channel 0 or channel 1) by the AHB master port."]
    #[inline(always)]
    pub const fn set_LAST_CH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "11:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED10(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "11:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "12:12\\] A 1 indicates that the Crypto peripheral has detected an AHB bus error."]
    #[must_use]
    #[inline(always)]
    pub const fn AHB_ERR(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] A 1 indicates that the Crypto peripheral has detected an AHB bus error."]
    #[inline(always)]
    pub const fn set_AHB_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "31:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED13(&self) -> u32 {
        let val = (self.0 >> 13usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "31:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED13(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 13usize)) | (((val as u32) & 0x0007_ffff) << 13usize);
    }
}
impl Default for DMAPORTERR {
    #[inline(always)]
    fn default() -> DMAPORTERR {
        DMAPORTERR(0)
    }
}
impl core::fmt::Debug for DMAPORTERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAPORTERR")
            .field("RESERVED0", &self.RESERVED0())
            .field("LAST_CH", &self.LAST_CH())
            .field("RESERVED10", &self.RESERVED10())
            .field("AHB_ERR", &self.AHB_ERR())
            .field("RESERVED13", &self.RESERVED13())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMAPORTERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMAPORTERR {{ RESERVED0: {=u16:?}, LAST_CH: {=bool:?}, RESERVED10: {=u8:?}, AHB_ERR: {=bool:?}, RESERVED13: {=u32:?} }}",
            self.RESERVED0(),
            self.LAST_CH(),
            self.RESERVED10(),
            self.AHB_ERR(),
            self.RESERVED13()
        )
    }
}
#[doc = "Master Protection Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMAPROTCTL(pub u32);
impl DMAPROTCTL {
    #[doc = "0:0\\] Select AHB transfer protection control for DMA transfers using the key store area as destination. 0 : transfers use 'USER' type access. 1 : transfers use 'PRIVILEGED' type access."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Select AHB transfer protection control for DMA transfers using the key store area as destination. 0 : transfers use 'USER' type access. 1 : transfers use 'PRIVILEGED' type access."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
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
impl Default for DMAPROTCTL {
    #[inline(always)]
    fn default() -> DMAPROTCTL {
        DMAPROTCTL(0)
    }
}
impl core::fmt::Debug for DMAPROTCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAPROTCTL")
            .field("EN", &self.EN())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMAPROTCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMAPROTCTL {{ EN: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.EN(),
            self.RESERVED1()
        )
    }
}
#[doc = "DMA Controller Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMASTAT(pub u32);
impl DMASTAT {
    #[doc = "0:0\\] This register field indicates if DMA channel 0 is active or not. 0: Not active 1: Active."]
    #[must_use]
    #[inline(always)]
    pub const fn CH0_ACTIVE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This register field indicates if DMA channel 0 is active or not. 0: Not active 1: Active."]
    #[inline(always)]
    pub const fn set_CH0_ACTIVE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] This register field indicates if DMA channel 1 is active or not. 0: Not active 1: Active."]
    #[must_use]
    #[inline(always)]
    pub const fn CH1_ACTIVE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] This register field indicates if DMA channel 1 is active or not. 0: Not active 1: Active."]
    #[inline(always)]
    pub const fn set_CH1_ACTIVE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "16:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x7fff;
        val as u16
    }
    #[doc = "16:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 2usize)) | (((val as u32) & 0x7fff) << 2usize);
    }
    #[doc = "17:17\\] Reflects possible transfer errors on the AHB port."]
    #[must_use]
    #[inline(always)]
    pub const fn PORT_ERR(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Reflects possible transfer errors on the AHB port."]
    #[inline(always)]
    pub const fn set_PORT_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "31:18\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED18(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x3fff;
        val as u16
    }
    #[doc = "31:18\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED18(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 18usize)) | (((val as u32) & 0x3fff) << 18usize);
    }
}
impl Default for DMASTAT {
    #[inline(always)]
    fn default() -> DMASTAT {
        DMASTAT(0)
    }
}
impl core::fmt::Debug for DMASTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMASTAT")
            .field("CH0_ACTIVE", &self.CH0_ACTIVE())
            .field("CH1_ACTIVE", &self.CH1_ACTIVE())
            .field("RESERVED2", &self.RESERVED2())
            .field("PORT_ERR", &self.PORT_ERR())
            .field("RESERVED18", &self.RESERVED18())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMASTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMASTAT {{ CH0_ACTIVE: {=bool:?}, CH1_ACTIVE: {=bool:?}, RESERVED2: {=u16:?}, PORT_ERR: {=bool:?}, RESERVED18: {=u16:?} }}",
            self.CH0_ACTIVE(),
            self.CH1_ACTIVE(),
            self.RESERVED2(),
            self.PORT_ERR(),
            self.RESERVED18()
        )
    }
}
#[doc = "DMA Controller Software Reset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMASWRESET(pub u32);
impl DMASWRESET {
    #[doc = "0:0\\] Software reset enable 0: Disable 1: Enable (self-cleared to zero). Note: Completion of the software reset must be checked in DMASTAT.CH0_ACTIVE and DMASTAT.CH1_ACTIVE."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Software reset enable 0: Disable 1: Enable (self-cleared to zero). Note: Completion of the software reset must be checked in DMASTAT.CH0_ACTIVE and DMASTAT.CH1_ACTIVE."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: bool) {
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
impl Default for DMASWRESET {
    #[inline(always)]
    fn default() -> DMASWRESET {
        DMASWRESET(0)
    }
}
impl core::fmt::Debug for DMASWRESET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMASWRESET")
            .field("RESET", &self.RESET())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMASWRESET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMASWRESET {{ RESET: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.RESET(),
            self.RESERVED1()
        )
    }
}
#[doc = "CTRL Module Version."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HWVER(pub u32);
impl HWVER {
    #[doc = "7:0\\] The version number for the Crypto peripheral, this field contains the value 120 (decimal) or 0x78."]
    #[must_use]
    #[inline(always)]
    pub const fn VER_NUM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "7:0\\] The version number for the Crypto peripheral, this field contains the value 120 (decimal) or 0x78."]
    #[inline(always)]
    pub const fn set_VER_NUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "15:8\\] These bits simply contain the complement of VER_NUM (0x87), used by a driver to ascertain that the Crypto peripheral register is indeed read."]
    #[must_use]
    #[inline(always)]
    pub const fn VER_NUM_COMPL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "15:8\\] These bits simply contain the complement of VER_NUM (0x87), used by a driver to ascertain that the Crypto peripheral register is indeed read."]
    #[inline(always)]
    pub const fn set_VER_NUM_COMPL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "19:16\\] Patch level, starts at 0 at first delivery of this version."]
    #[must_use]
    #[inline(always)]
    pub const fn HW_PATCH_LVL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "19:16\\] Patch level, starts at 0 at first delivery of this version."]
    #[inline(always)]
    pub const fn set_HW_PATCH_LVL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "23:20\\] Minor version number."]
    #[must_use]
    #[inline(always)]
    pub const fn HW_MINOR_VER(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "23:20\\] Minor version number."]
    #[inline(always)]
    pub const fn set_HW_MINOR_VER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "27:24\\] Major version number."]
    #[must_use]
    #[inline(always)]
    pub const fn HW_MAJOR_VER(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "27:24\\] Major version number."]
    #[inline(always)]
    pub const fn set_HW_MAJOR_VER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "31:28\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED28(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "31:28\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED28(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for HWVER {
    #[inline(always)]
    fn default() -> HWVER {
        HWVER(0)
    }
}
impl core::fmt::Debug for HWVER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWVER")
            .field("VER_NUM", &self.VER_NUM())
            .field("VER_NUM_COMPL", &self.VER_NUM_COMPL())
            .field("HW_PATCH_LVL", &self.HW_PATCH_LVL())
            .field("HW_MINOR_VER", &self.HW_MINOR_VER())
            .field("HW_MAJOR_VER", &self.HW_MAJOR_VER())
            .field("RESERVED28", &self.RESERVED28())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HWVER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HWVER {{ VER_NUM: {=u8:?}, VER_NUM_COMPL: {=u8:?}, HW_PATCH_LVL: {=u8:?}, HW_MINOR_VER: {=u8:?}, HW_MAJOR_VER: {=u8:?}, RESERVED28: {=u8:?} }}",
            self.VER_NUM(),
            self.VER_NUM_COMPL(),
            self.HW_PATCH_LVL(),
            self.HW_MINOR_VER(),
            self.HW_MAJOR_VER(),
            self.RESERVED28()
        )
    }
}
#[doc = "Interrupt Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQCLR(pub u32);
impl IRQCLR {
    #[doc = "0:0\\] If 1 is written to this bit, IRQSTAT.RESULT_AVAIL is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn RESULT_AVAIL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] If 1 is written to this bit, IRQSTAT.RESULT_AVAIL is cleared."]
    #[inline(always)]
    pub const fn set_RESULT_AVAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] If 1 is written to this bit, IRQSTAT.DMA_IN_DONE is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_IN_DONE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] If 1 is written to this bit, IRQSTAT.DMA_IN_DONE is cleared."]
    #[inline(always)]
    pub const fn set_DMA_IN_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "28:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "28:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 2usize)) | (((val as u32) & 0x07ff_ffff) << 2usize);
    }
    #[doc = "29:29\\] If 1 is written to this bit, IRQSTAT.KEY_ST_RD_ERR is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY_ST_RD_ERR(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] If 1 is written to this bit, IRQSTAT.KEY_ST_RD_ERR is cleared."]
    #[inline(always)]
    pub const fn set_KEY_ST_RD_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] If 1 is written to this bit, IRQSTAT.KEY_ST_WR_ERR is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY_ST_WR_ERR(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] If 1 is written to this bit, IRQSTAT.KEY_ST_WR_ERR is cleared."]
    #[inline(always)]
    pub const fn set_KEY_ST_WR_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] If 1 is written to this bit, IRQSTAT.DMA_BUS_ERR is cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_BUS_ERR(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] If 1 is written to this bit, IRQSTAT.DMA_BUS_ERR is cleared."]
    #[inline(always)]
    pub const fn set_DMA_BUS_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IRQCLR {
    #[inline(always)]
    fn default() -> IRQCLR {
        IRQCLR(0)
    }
}
impl core::fmt::Debug for IRQCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQCLR")
            .field("RESULT_AVAIL", &self.RESULT_AVAIL())
            .field("DMA_IN_DONE", &self.DMA_IN_DONE())
            .field("RESERVED2", &self.RESERVED2())
            .field("KEY_ST_RD_ERR", &self.KEY_ST_RD_ERR())
            .field("KEY_ST_WR_ERR", &self.KEY_ST_WR_ERR())
            .field("DMA_BUS_ERR", &self.DMA_BUS_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQCLR {{ RESULT_AVAIL: {=bool:?}, DMA_IN_DONE: {=bool:?}, RESERVED2: {=u32:?}, KEY_ST_RD_ERR: {=bool:?}, KEY_ST_WR_ERR: {=bool:?}, DMA_BUS_ERR: {=bool:?} }}",
            self.RESULT_AVAIL(),
            self.DMA_IN_DONE(),
            self.RESERVED2(),
            self.KEY_ST_RD_ERR(),
            self.KEY_ST_WR_ERR(),
            self.DMA_BUS_ERR()
        )
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQEN(pub u32);
impl IRQEN {
    #[doc = "0:0\\] This bit enables IRQSTAT.RESULT_AVAIL as source for IRQ."]
    #[must_use]
    #[inline(always)]
    pub const fn RESULT_AVAIL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This bit enables IRQSTAT.RESULT_AVAIL as source for IRQ."]
    #[inline(always)]
    pub const fn set_RESULT_AVAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] This bit enables IRQSTAT.DMA_IN_DONE as source for IRQ."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_IN_DONE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] This bit enables IRQSTAT.DMA_IN_DONE as source for IRQ."]
    #[inline(always)]
    pub const fn set_DMA_IN_DONE(&mut self, val: bool) {
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
impl Default for IRQEN {
    #[inline(always)]
    fn default() -> IRQEN {
        IRQEN(0)
    }
}
impl core::fmt::Debug for IRQEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQEN")
            .field("RESULT_AVAIL", &self.RESULT_AVAIL())
            .field("DMA_IN_DONE", &self.DMA_IN_DONE())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQEN {{ RESULT_AVAIL: {=bool:?}, DMA_IN_DONE: {=bool:?}, RESERVED2: {=u32:?} }}",
            self.RESULT_AVAIL(),
            self.DMA_IN_DONE(),
            self.RESERVED2()
        )
    }
}
#[doc = "Interrupt Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSET(pub u32);
impl IRQSET {
    #[doc = "0:0\\] If 1 is written to this bit, IRQSTAT.RESULT_AVAIL is set. Writing 0 has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn RESULT_AVAIL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] If 1 is written to this bit, IRQSTAT.RESULT_AVAIL is set. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn set_RESULT_AVAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] If 1 is written to this bit, IRQSTAT.DMA_IN_DONE is set. Writing 0 has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_IN_DONE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] If 1 is written to this bit, IRQSTAT.DMA_IN_DONE is set. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn set_DMA_IN_DONE(&mut self, val: bool) {
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
impl Default for IRQSET {
    #[inline(always)]
    fn default() -> IRQSET {
        IRQSET(0)
    }
}
impl core::fmt::Debug for IRQSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSET")
            .field("RESULT_AVAIL", &self.RESULT_AVAIL())
            .field("DMA_IN_DONE", &self.DMA_IN_DONE())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQSET {{ RESULT_AVAIL: {=bool:?}, DMA_IN_DONE: {=bool:?}, RESERVED2: {=u32:?} }}",
            self.RESULT_AVAIL(),
            self.DMA_IN_DONE(),
            self.RESERVED2()
        )
    }
}
#[doc = "Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSTAT(pub u32);
impl IRQSTAT {
    #[doc = "0:0\\] This bit is set high when the Crypto peripheral has a result available."]
    #[must_use]
    #[inline(always)]
    pub const fn RESULT_AVAIL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] This bit is set high when the Crypto peripheral has a result available."]
    #[inline(always)]
    pub const fn set_RESULT_AVAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] This bit returns the status of DMA data in done interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_IN_DONE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] This bit returns the status of DMA data in done interrupt."]
    #[inline(always)]
    pub const fn set_DMA_IN_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "28:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED2(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "28:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 2usize)) | (((val as u32) & 0x07ff_ffff) << 2usize);
    }
    #[doc = "29:29\\] This bit will be set when a read error is detected during the read of a key from the key store, while copying it to the AES engine. The value of this register is held until it is cleared via IRQCLR.KEY_ST_RD_ERR. Note: This error is asserted if a key location is selected in the key store that is not available. Note: This is not an interrupt source."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY_ST_RD_ERR(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] This bit will be set when a read error is detected during the read of a key from the key store, while copying it to the AES engine. The value of this register is held until it is cleared via IRQCLR.KEY_ST_RD_ERR. Note: This error is asserted if a key location is selected in the key store that is not available. Note: This is not an interrupt source."]
    #[inline(always)]
    pub const fn set_KEY_ST_RD_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] This bit is set when a write error is detected during the DMA write operation to the key store memory. The value of this register is held until it is cleared via IRQCLR.KEY_ST_WR_ERR Note: This error is asserted if a DMA operation does not cover a full key area or more areas are written than expected. Note: This is not an interrupt source."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY_ST_WR_ERR(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] This bit is set when a write error is detected during the DMA write operation to the key store memory. The value of this register is held until it is cleared via IRQCLR.KEY_ST_WR_ERR Note: This error is asserted if a DMA operation does not cover a full key area or more areas are written than expected. Note: This is not an interrupt source."]
    #[inline(always)]
    pub const fn set_KEY_ST_WR_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] This bit is set when a DMA bus error is detected during a DMA operation. The value of this register is held until it is cleared via IRQCLR.DMA_BUS_ERR Note: This error is asserted if an error is detected on the AHB master interface during a DMA operation. Note: This is not an interrupt source."]
    #[must_use]
    #[inline(always)]
    pub const fn DMA_BUS_ERR(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] This bit is set when a DMA bus error is detected during a DMA operation. The value of this register is held until it is cleared via IRQCLR.DMA_BUS_ERR Note: This error is asserted if an error is detected on the AHB master interface during a DMA operation. Note: This is not an interrupt source."]
    #[inline(always)]
    pub const fn set_DMA_BUS_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IRQSTAT {
    #[inline(always)]
    fn default() -> IRQSTAT {
        IRQSTAT(0)
    }
}
impl core::fmt::Debug for IRQSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSTAT")
            .field("RESULT_AVAIL", &self.RESULT_AVAIL())
            .field("DMA_IN_DONE", &self.DMA_IN_DONE())
            .field("RESERVED2", &self.RESERVED2())
            .field("KEY_ST_RD_ERR", &self.KEY_ST_RD_ERR())
            .field("KEY_ST_WR_ERR", &self.KEY_ST_WR_ERR())
            .field("DMA_BUS_ERR", &self.DMA_BUS_ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQSTAT {{ RESULT_AVAIL: {=bool:?}, DMA_IN_DONE: {=bool:?}, RESERVED2: {=u32:?}, KEY_ST_RD_ERR: {=bool:?}, KEY_ST_WR_ERR: {=bool:?}, DMA_BUS_ERR: {=bool:?} }}",
            self.RESULT_AVAIL(),
            self.DMA_IN_DONE(),
            self.RESERVED2(),
            self.KEY_ST_RD_ERR(),
            self.KEY_ST_WR_ERR(),
            self.DMA_BUS_ERR()
        )
    }
}
#[doc = "Control Interrupt Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQTYPE(pub u32);
impl IRQTYPE {
    #[doc = "0:0\\] If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
    #[must_use]
    #[inline(always)]
    pub const fn LEVEL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] If this bit is 0, the interrupt output is a pulse. If this bit is set to 1, the interrupt is a level interrupt that must be cleared by writing the interrupt clear register. This bit is applicable for both interrupt output signals."]
    #[inline(always)]
    pub const fn set_LEVEL(&mut self, val: bool) {
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
impl Default for IRQTYPE {
    #[inline(always)]
    fn default() -> IRQTYPE {
        IRQTYPE(0)
    }
}
impl core::fmt::Debug for IRQTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQTYPE")
            .field("LEVEL", &self.LEVEL())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQTYPE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQTYPE {{ LEVEL: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.LEVEL(),
            self.RESERVED1()
        )
    }
}
#[doc = "Key Read Area."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KEYREADAREA(pub u32);
impl KEYREADAREA {
    #[doc = "3:0\\] Selects the area of the key store RAM from where the key needs to be read that will be written to the AES engine. Only RAM areas that contain valid written keys can be selected."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA(&self) -> super::vals::RAM_AREA {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::RAM_AREA::from_bits(val as u8)
    }
    #[doc = "3:0\\] Selects the area of the key store RAM from where the key needs to be read that will be written to the AES engine. Only RAM areas that contain valid written keys can be selected."]
    #[inline(always)]
    pub const fn set_RAM_AREA(&mut self, val: super::vals::RAM_AREA) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "30:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED4(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "30:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 4usize)) | (((val as u32) & 0x07ff_ffff) << 4usize);
    }
    #[doc = "31:31\\] Key store operation busy status flag (read only) 0: operation is completed. 1: operation is not completed and the key store is busy."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Key store operation busy status flag (read only) 0: operation is completed. 1: operation is not completed and the key store is busy."]
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for KEYREADAREA {
    #[inline(always)]
    fn default() -> KEYREADAREA {
        KEYREADAREA(0)
    }
}
impl core::fmt::Debug for KEYREADAREA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYREADAREA")
            .field("RAM_AREA", &self.RAM_AREA())
            .field("RESERVED4", &self.RESERVED4())
            .field("BUSY", &self.BUSY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KEYREADAREA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "KEYREADAREA {{ RAM_AREA: {:?}, RESERVED4: {=u32:?}, BUSY: {=bool:?} }}",
            self.RAM_AREA(),
            self.RESERVED4(),
            self.BUSY()
        )
    }
}
#[doc = "Key Size This register defines the size of the keys that are written with DMA."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KEYSIZE(pub u32);
impl KEYSIZE {
    #[doc = "1:0\\] Key size When writing to this register, KEYWRITTENAREA will be reset. Note: For the Crypto peripheral this field is fixed to 128 bits. For software compatibility KEYWRITTENAREA will be reset when writing to this register."]
    #[must_use]
    #[inline(always)]
    pub const fn SIZE(&self) -> super::vals::SIZE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SIZE::from_bits(val as u8)
    }
    #[doc = "1:0\\] Key size When writing to this register, KEYWRITTENAREA will be reset. Note: For the Crypto peripheral this field is fixed to 128 bits. For software compatibility KEYWRITTENAREA will be reset when writing to this register."]
    #[inline(always)]
    pub const fn set_SIZE(&mut self, val: super::vals::SIZE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
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
impl Default for KEYSIZE {
    #[inline(always)]
    fn default() -> KEYSIZE {
        KEYSIZE(0)
    }
}
impl core::fmt::Debug for KEYSIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYSIZE")
            .field("SIZE", &self.SIZE())
            .field("RESERVED2", &self.RESERVED2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KEYSIZE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "KEYSIZE {{ SIZE: {:?}, RESERVED2: {=u32:?} }}",
            self.SIZE(),
            self.RESERVED2()
        )
    }
}
#[doc = "Key Write Area."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KEYWRITEAREA(pub u32);
impl KEYWRITEAREA {
    #[doc = "0:0\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA0(&self) -> super::vals::RAM_AREA0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RAM_AREA0::from_bits(val as u8)
    }
    #[doc = "0:0\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub const fn set_RAM_AREA0(&mut self, val: super::vals::RAM_AREA0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA1(&self) -> super::vals::RAM_AREA1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RAM_AREA1::from_bits(val as u8)
    }
    #[doc = "1:1\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub const fn set_RAM_AREA1(&mut self, val: super::vals::RAM_AREA1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA2(&self) -> super::vals::RAM_AREA2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RAM_AREA2::from_bits(val as u8)
    }
    #[doc = "2:2\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub const fn set_RAM_AREA2(&mut self, val: super::vals::RAM_AREA2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA3(&self) -> super::vals::RAM_AREA3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::RAM_AREA3::from_bits(val as u8)
    }
    #[doc = "3:3\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub const fn set_RAM_AREA3(&mut self, val: super::vals::RAM_AREA3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA4(&self) -> super::vals::RAM_AREA4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::RAM_AREA4::from_bits(val as u8)
    }
    #[doc = "4:4\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub const fn set_RAM_AREA4(&mut self, val: super::vals::RAM_AREA4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA5(&self) -> super::vals::RAM_AREA5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::RAM_AREA5::from_bits(val as u8)
    }
    #[doc = "5:5\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub const fn set_RAM_AREA5(&mut self, val: super::vals::RAM_AREA5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA6(&self) -> super::vals::RAM_AREA6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::RAM_AREA6::from_bits(val as u8)
    }
    #[doc = "6:6\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub const fn set_RAM_AREA6(&mut self, val: super::vals::RAM_AREA6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA7(&self) -> super::vals::RAM_AREA7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::RAM_AREA7::from_bits(val as u8)
    }
    #[doc = "7:7\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub const fn set_RAM_AREA7(&mut self, val: super::vals::RAM_AREA7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
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
impl Default for KEYWRITEAREA {
    #[inline(always)]
    fn default() -> KEYWRITEAREA {
        KEYWRITEAREA(0)
    }
}
impl core::fmt::Debug for KEYWRITEAREA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYWRITEAREA")
            .field("RAM_AREA0", &self.RAM_AREA0())
            .field("RAM_AREA1", &self.RAM_AREA1())
            .field("RAM_AREA2", &self.RAM_AREA2())
            .field("RAM_AREA3", &self.RAM_AREA3())
            .field("RAM_AREA4", &self.RAM_AREA4())
            .field("RAM_AREA5", &self.RAM_AREA5())
            .field("RAM_AREA6", &self.RAM_AREA6())
            .field("RAM_AREA7", &self.RAM_AREA7())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KEYWRITEAREA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "KEYWRITEAREA {{ RAM_AREA0: {:?}, RAM_AREA1: {:?}, RAM_AREA2: {:?}, RAM_AREA3: {:?}, RAM_AREA4: {:?}, RAM_AREA5: {:?}, RAM_AREA6: {:?}, RAM_AREA7: {:?}, RESERVED8: {=u32:?} }}",
            self.RAM_AREA0(),
            self.RAM_AREA1(),
            self.RAM_AREA2(),
            self.RAM_AREA3(),
            self.RAM_AREA4(),
            self.RAM_AREA5(),
            self.RAM_AREA6(),
            self.RAM_AREA7(),
            self.RESERVED8()
        )
    }
}
#[doc = "Key Written Area Status This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and will result in an error."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KEYWRITTENAREA(pub u32);
impl KEYWRITTENAREA {
    #[doc = "0:0\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA_WRITTEN0(&self) -> super::vals::RAM_AREA_WRITTEN0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RAM_AREA_WRITTEN0::from_bits(val as u8)
    }
    #[doc = "0:0\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub const fn set_RAM_AREA_WRITTEN0(&mut self, val: super::vals::RAM_AREA_WRITTEN0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA_WRITTEN1(&self) -> super::vals::RAM_AREA_WRITTEN1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RAM_AREA_WRITTEN1::from_bits(val as u8)
    }
    #[doc = "1:1\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub const fn set_RAM_AREA_WRITTEN1(&mut self, val: super::vals::RAM_AREA_WRITTEN1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA_WRITTEN2(&self) -> super::vals::RAM_AREA_WRITTEN2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RAM_AREA_WRITTEN2::from_bits(val as u8)
    }
    #[doc = "2:2\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub const fn set_RAM_AREA_WRITTEN2(&mut self, val: super::vals::RAM_AREA_WRITTEN2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA_WRITTEN3(&self) -> super::vals::RAM_AREA_WRITTEN3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::RAM_AREA_WRITTEN3::from_bits(val as u8)
    }
    #[doc = "3:3\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub const fn set_RAM_AREA_WRITTEN3(&mut self, val: super::vals::RAM_AREA_WRITTEN3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA_WRITTEN4(&self) -> super::vals::RAM_AREA_WRITTEN4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::RAM_AREA_WRITTEN4::from_bits(val as u8)
    }
    #[doc = "4:4\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub const fn set_RAM_AREA_WRITTEN4(&mut self, val: super::vals::RAM_AREA_WRITTEN4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA_WRITTEN5(&self) -> super::vals::RAM_AREA_WRITTEN5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::RAM_AREA_WRITTEN5::from_bits(val as u8)
    }
    #[doc = "5:5\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub const fn set_RAM_AREA_WRITTEN5(&mut self, val: super::vals::RAM_AREA_WRITTEN5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA_WRITTEN6(&self) -> super::vals::RAM_AREA_WRITTEN6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::RAM_AREA_WRITTEN6::from_bits(val as u8)
    }
    #[doc = "6:6\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub const fn set_RAM_AREA_WRITTEN6(&mut self, val: super::vals::RAM_AREA_WRITTEN6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM_AREA_WRITTEN7(&self) -> super::vals::RAM_AREA_WRITTEN7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::RAM_AREA_WRITTEN7::from_bits(val as u8)
    }
    #[doc = "7:7\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub const fn set_RAM_AREA_WRITTEN7(&mut self, val: super::vals::RAM_AREA_WRITTEN7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
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
impl Default for KEYWRITTENAREA {
    #[inline(always)]
    fn default() -> KEYWRITTENAREA {
        KEYWRITTENAREA(0)
    }
}
impl core::fmt::Debug for KEYWRITTENAREA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYWRITTENAREA")
            .field("RAM_AREA_WRITTEN0", &self.RAM_AREA_WRITTEN0())
            .field("RAM_AREA_WRITTEN1", &self.RAM_AREA_WRITTEN1())
            .field("RAM_AREA_WRITTEN2", &self.RAM_AREA_WRITTEN2())
            .field("RAM_AREA_WRITTEN3", &self.RAM_AREA_WRITTEN3())
            .field("RAM_AREA_WRITTEN4", &self.RAM_AREA_WRITTEN4())
            .field("RAM_AREA_WRITTEN5", &self.RAM_AREA_WRITTEN5())
            .field("RAM_AREA_WRITTEN6", &self.RAM_AREA_WRITTEN6())
            .field("RAM_AREA_WRITTEN7", &self.RAM_AREA_WRITTEN7())
            .field("RESERVED8", &self.RESERVED8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KEYWRITTENAREA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "KEYWRITTENAREA {{ RAM_AREA_WRITTEN0: {:?}, RAM_AREA_WRITTEN1: {:?}, RAM_AREA_WRITTEN2: {:?}, RAM_AREA_WRITTEN3: {:?}, RAM_AREA_WRITTEN4: {:?}, RAM_AREA_WRITTEN5: {:?}, RAM_AREA_WRITTEN6: {:?}, RAM_AREA_WRITTEN7: {:?}, RESERVED8: {=u32:?} }}",
            self.RAM_AREA_WRITTEN0(),
            self.RAM_AREA_WRITTEN1(),
            self.RAM_AREA_WRITTEN2(),
            self.RAM_AREA_WRITTEN3(),
            self.RAM_AREA_WRITTEN4(),
            self.RAM_AREA_WRITTEN5(),
            self.RAM_AREA_WRITTEN6(),
            self.RAM_AREA_WRITTEN7(),
            self.RESERVED8()
        )
    }
}
#[doc = "Software Reset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SWRESET(pub u32);
impl SWRESET {
    #[doc = "0:0\\] If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the Written Area flags; therefore the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
    #[must_use]
    #[inline(always)]
    pub const fn RESET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the Written Area flags; therefore the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
    #[inline(always)]
    pub const fn set_RESET(&mut self, val: bool) {
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
impl Default for SWRESET {
    #[inline(always)]
    fn default() -> SWRESET {
        SWRESET(0)
    }
}
impl core::fmt::Debug for SWRESET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWRESET")
            .field("RESET", &self.RESET())
            .field("RESERVED1", &self.RESERVED1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SWRESET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SWRESET {{ RESET: {=bool:?}, RESERVED1: {=u32:?} }}",
            self.RESET(),
            self.RESERVED1()
        )
    }
}

#[doc = "Data Input from DIO 0 to 31."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DIN31_0(pub u32);
impl DIN31_0 {
    #[doc = "0:0\\] Data input from DIO 0."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Data input from DIO 0."]
    #[inline(always)]
    pub const fn set_DIO0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Data input from DIO 1."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Data input from DIO 1."]
    #[inline(always)]
    pub const fn set_DIO1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Data input from DIO 2."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Data input from DIO 2."]
    #[inline(always)]
    pub const fn set_DIO2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Data input from DIO 3."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Data input from DIO 3."]
    #[inline(always)]
    pub const fn set_DIO3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Data input from DIO 4."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Data input from DIO 4."]
    #[inline(always)]
    pub const fn set_DIO4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Data input from DIO 5."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Data input from DIO 5."]
    #[inline(always)]
    pub const fn set_DIO5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Data input from DIO 6."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Data input from DIO 6."]
    #[inline(always)]
    pub const fn set_DIO6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Data input from DIO 7."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Data input from DIO 7."]
    #[inline(always)]
    pub const fn set_DIO7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Data input from DIO 8."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Data input from DIO 8."]
    #[inline(always)]
    pub const fn set_DIO8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Data input from DIO 9."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Data input from DIO 9."]
    #[inline(always)]
    pub const fn set_DIO9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Data input from DIO 10."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Data input from DIO 10."]
    #[inline(always)]
    pub const fn set_DIO10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Data input from DIO 11."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Data input from DIO 11."]
    #[inline(always)]
    pub const fn set_DIO11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Data input from DIO 12."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Data input from DIO 12."]
    #[inline(always)]
    pub const fn set_DIO12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Data input from DIO 13."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Data input from DIO 13."]
    #[inline(always)]
    pub const fn set_DIO13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Data input from DIO 14."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Data input from DIO 14."]
    #[inline(always)]
    pub const fn set_DIO14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Data input from DIO 15."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Data input from DIO 15."]
    #[inline(always)]
    pub const fn set_DIO15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Data input from DIO 16."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Data input from DIO 16."]
    #[inline(always)]
    pub const fn set_DIO16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Data input from DIO 17."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Data input from DIO 17."]
    #[inline(always)]
    pub const fn set_DIO17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Data input from DIO 18."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Data input from DIO 18."]
    #[inline(always)]
    pub const fn set_DIO18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Data input from DIO 19."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Data input from DIO 19."]
    #[inline(always)]
    pub const fn set_DIO19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Data input from DIO 20."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Data input from DIO 20."]
    #[inline(always)]
    pub const fn set_DIO20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Data input from DIO 21."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Data input from DIO 21."]
    #[inline(always)]
    pub const fn set_DIO21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Data input from DIO 22."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Data input from DIO 22."]
    #[inline(always)]
    pub const fn set_DIO22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Data input from DIO 23."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Data input from DIO 23."]
    #[inline(always)]
    pub const fn set_DIO23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Data input from DIO 24."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Data input from DIO 24."]
    #[inline(always)]
    pub const fn set_DIO24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Data input from DIO 25."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Data input from DIO 25."]
    #[inline(always)]
    pub const fn set_DIO25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Data input from DIO 26."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Data input from DIO 26."]
    #[inline(always)]
    pub const fn set_DIO26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Data input from DIO 27."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Data input from DIO 27."]
    #[inline(always)]
    pub const fn set_DIO27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Data input from DIO 28."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Data input from DIO 28."]
    #[inline(always)]
    pub const fn set_DIO28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Data input from DIO 29."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Data input from DIO 29."]
    #[inline(always)]
    pub const fn set_DIO29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Data input from DIO 30."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Data input from DIO 30."]
    #[inline(always)]
    pub const fn set_DIO30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Data input from DIO 31."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Data input from DIO 31."]
    #[inline(always)]
    pub const fn set_DIO31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DIN31_0 {
    #[inline(always)]
    fn default() -> DIN31_0 {
        DIN31_0(0)
    }
}
impl core::fmt::Debug for DIN31_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN31_0")
            .field("DIO0", &self.DIO0())
            .field("DIO1", &self.DIO1())
            .field("DIO2", &self.DIO2())
            .field("DIO3", &self.DIO3())
            .field("DIO4", &self.DIO4())
            .field("DIO5", &self.DIO5())
            .field("DIO6", &self.DIO6())
            .field("DIO7", &self.DIO7())
            .field("DIO8", &self.DIO8())
            .field("DIO9", &self.DIO9())
            .field("DIO10", &self.DIO10())
            .field("DIO11", &self.DIO11())
            .field("DIO12", &self.DIO12())
            .field("DIO13", &self.DIO13())
            .field("DIO14", &self.DIO14())
            .field("DIO15", &self.DIO15())
            .field("DIO16", &self.DIO16())
            .field("DIO17", &self.DIO17())
            .field("DIO18", &self.DIO18())
            .field("DIO19", &self.DIO19())
            .field("DIO20", &self.DIO20())
            .field("DIO21", &self.DIO21())
            .field("DIO22", &self.DIO22())
            .field("DIO23", &self.DIO23())
            .field("DIO24", &self.DIO24())
            .field("DIO25", &self.DIO25())
            .field("DIO26", &self.DIO26())
            .field("DIO27", &self.DIO27())
            .field("DIO28", &self.DIO28())
            .field("DIO29", &self.DIO29())
            .field("DIO30", &self.DIO30())
            .field("DIO31", &self.DIO31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DIN31_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DIN31_0 {{ DIO0: {=bool:?}, DIO1: {=bool:?}, DIO2: {=bool:?}, DIO3: {=bool:?}, DIO4: {=bool:?}, DIO5: {=bool:?}, DIO6: {=bool:?}, DIO7: {=bool:?}, DIO8: {=bool:?}, DIO9: {=bool:?}, DIO10: {=bool:?}, DIO11: {=bool:?}, DIO12: {=bool:?}, DIO13: {=bool:?}, DIO14: {=bool:?}, DIO15: {=bool:?}, DIO16: {=bool:?}, DIO17: {=bool:?}, DIO18: {=bool:?}, DIO19: {=bool:?}, DIO20: {=bool:?}, DIO21: {=bool:?}, DIO22: {=bool:?}, DIO23: {=bool:?}, DIO24: {=bool:?}, DIO25: {=bool:?}, DIO26: {=bool:?}, DIO27: {=bool:?}, DIO28: {=bool:?}, DIO29: {=bool:?}, DIO30: {=bool:?}, DIO31: {=bool:?} }}",
            self.DIO0(),
            self.DIO1(),
            self.DIO2(),
            self.DIO3(),
            self.DIO4(),
            self.DIO5(),
            self.DIO6(),
            self.DIO7(),
            self.DIO8(),
            self.DIO9(),
            self.DIO10(),
            self.DIO11(),
            self.DIO12(),
            self.DIO13(),
            self.DIO14(),
            self.DIO15(),
            self.DIO16(),
            self.DIO17(),
            self.DIO18(),
            self.DIO19(),
            self.DIO20(),
            self.DIO21(),
            self.DIO22(),
            self.DIO23(),
            self.DIO24(),
            self.DIO25(),
            self.DIO26(),
            self.DIO27(),
            self.DIO28(),
            self.DIO29(),
            self.DIO30(),
            self.DIO31()
        )
    }
}
#[doc = "Data Output Enable for DIO 0 to 31."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DOE31_0(pub u32);
impl DOE31_0 {
    #[doc = "0:0\\] Data output enable for DIO 0."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Data output enable for DIO 0."]
    #[inline(always)]
    pub const fn set_DIO0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Data output enable for DIO 1."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Data output enable for DIO 1."]
    #[inline(always)]
    pub const fn set_DIO1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Data output enable for DIO 2."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Data output enable for DIO 2."]
    #[inline(always)]
    pub const fn set_DIO2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Data output enable for DIO 3."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Data output enable for DIO 3."]
    #[inline(always)]
    pub const fn set_DIO3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Data output enable for DIO 4."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Data output enable for DIO 4."]
    #[inline(always)]
    pub const fn set_DIO4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Data output enable for DIO 5."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Data output enable for DIO 5."]
    #[inline(always)]
    pub const fn set_DIO5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Data output enable for DIO 6."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Data output enable for DIO 6."]
    #[inline(always)]
    pub const fn set_DIO6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Data output enable for DIO 7."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Data output enable for DIO 7."]
    #[inline(always)]
    pub const fn set_DIO7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Data output enable for DIO 8."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Data output enable for DIO 8."]
    #[inline(always)]
    pub const fn set_DIO8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Data output enable for DIO 9."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Data output enable for DIO 9."]
    #[inline(always)]
    pub const fn set_DIO9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Data output enable for DIO 10."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Data output enable for DIO 10."]
    #[inline(always)]
    pub const fn set_DIO10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Data output enable for DIO 11."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Data output enable for DIO 11."]
    #[inline(always)]
    pub const fn set_DIO11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Data output enable for DIO 12."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Data output enable for DIO 12."]
    #[inline(always)]
    pub const fn set_DIO12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Data output enable for DIO 13."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Data output enable for DIO 13."]
    #[inline(always)]
    pub const fn set_DIO13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Data output enable for DIO 14."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Data output enable for DIO 14."]
    #[inline(always)]
    pub const fn set_DIO14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Data output enable for DIO 15."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Data output enable for DIO 15."]
    #[inline(always)]
    pub const fn set_DIO15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Data output enable for DIO 16."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Data output enable for DIO 16."]
    #[inline(always)]
    pub const fn set_DIO16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Data output enable for DIO 17."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Data output enable for DIO 17."]
    #[inline(always)]
    pub const fn set_DIO17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Data output enable for DIO 18."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Data output enable for DIO 18."]
    #[inline(always)]
    pub const fn set_DIO18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Data output enable for DIO 19."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Data output enable for DIO 19."]
    #[inline(always)]
    pub const fn set_DIO19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Data output enable for DIO 20."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Data output enable for DIO 20."]
    #[inline(always)]
    pub const fn set_DIO20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Data output enable for DIO 21."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Data output enable for DIO 21."]
    #[inline(always)]
    pub const fn set_DIO21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Data output enable for DIO 22."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Data output enable for DIO 22."]
    #[inline(always)]
    pub const fn set_DIO22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Data output enable for DIO 23."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Data output enable for DIO 23."]
    #[inline(always)]
    pub const fn set_DIO23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Data output enable for DIO 24."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Data output enable for DIO 24."]
    #[inline(always)]
    pub const fn set_DIO24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Data output enable for DIO 25."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Data output enable for DIO 25."]
    #[inline(always)]
    pub const fn set_DIO25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Data output enable for DIO 26."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Data output enable for DIO 26."]
    #[inline(always)]
    pub const fn set_DIO26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Data output enable for DIO 27."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Data output enable for DIO 27."]
    #[inline(always)]
    pub const fn set_DIO27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Data output enable for DIO 28."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Data output enable for DIO 28."]
    #[inline(always)]
    pub const fn set_DIO28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Data output enable for DIO 29."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Data output enable for DIO 29."]
    #[inline(always)]
    pub const fn set_DIO29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Data output enable for DIO 30."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Data output enable for DIO 30."]
    #[inline(always)]
    pub const fn set_DIO30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Data output enable for DIO 31."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Data output enable for DIO 31."]
    #[inline(always)]
    pub const fn set_DIO31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DOE31_0 {
    #[inline(always)]
    fn default() -> DOE31_0 {
        DOE31_0(0)
    }
}
impl core::fmt::Debug for DOE31_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOE31_0")
            .field("DIO0", &self.DIO0())
            .field("DIO1", &self.DIO1())
            .field("DIO2", &self.DIO2())
            .field("DIO3", &self.DIO3())
            .field("DIO4", &self.DIO4())
            .field("DIO5", &self.DIO5())
            .field("DIO6", &self.DIO6())
            .field("DIO7", &self.DIO7())
            .field("DIO8", &self.DIO8())
            .field("DIO9", &self.DIO9())
            .field("DIO10", &self.DIO10())
            .field("DIO11", &self.DIO11())
            .field("DIO12", &self.DIO12())
            .field("DIO13", &self.DIO13())
            .field("DIO14", &self.DIO14())
            .field("DIO15", &self.DIO15())
            .field("DIO16", &self.DIO16())
            .field("DIO17", &self.DIO17())
            .field("DIO18", &self.DIO18())
            .field("DIO19", &self.DIO19())
            .field("DIO20", &self.DIO20())
            .field("DIO21", &self.DIO21())
            .field("DIO22", &self.DIO22())
            .field("DIO23", &self.DIO23())
            .field("DIO24", &self.DIO24())
            .field("DIO25", &self.DIO25())
            .field("DIO26", &self.DIO26())
            .field("DIO27", &self.DIO27())
            .field("DIO28", &self.DIO28())
            .field("DIO29", &self.DIO29())
            .field("DIO30", &self.DIO30())
            .field("DIO31", &self.DIO31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DOE31_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DOE31_0 {{ DIO0: {=bool:?}, DIO1: {=bool:?}, DIO2: {=bool:?}, DIO3: {=bool:?}, DIO4: {=bool:?}, DIO5: {=bool:?}, DIO6: {=bool:?}, DIO7: {=bool:?}, DIO8: {=bool:?}, DIO9: {=bool:?}, DIO10: {=bool:?}, DIO11: {=bool:?}, DIO12: {=bool:?}, DIO13: {=bool:?}, DIO14: {=bool:?}, DIO15: {=bool:?}, DIO16: {=bool:?}, DIO17: {=bool:?}, DIO18: {=bool:?}, DIO19: {=bool:?}, DIO20: {=bool:?}, DIO21: {=bool:?}, DIO22: {=bool:?}, DIO23: {=bool:?}, DIO24: {=bool:?}, DIO25: {=bool:?}, DIO26: {=bool:?}, DIO27: {=bool:?}, DIO28: {=bool:?}, DIO29: {=bool:?}, DIO30: {=bool:?}, DIO31: {=bool:?} }}",
            self.DIO0(),
            self.DIO1(),
            self.DIO2(),
            self.DIO3(),
            self.DIO4(),
            self.DIO5(),
            self.DIO6(),
            self.DIO7(),
            self.DIO8(),
            self.DIO9(),
            self.DIO10(),
            self.DIO11(),
            self.DIO12(),
            self.DIO13(),
            self.DIO14(),
            self.DIO15(),
            self.DIO16(),
            self.DIO17(),
            self.DIO18(),
            self.DIO19(),
            self.DIO20(),
            self.DIO21(),
            self.DIO22(),
            self.DIO23(),
            self.DIO24(),
            self.DIO25(),
            self.DIO26(),
            self.DIO27(),
            self.DIO28(),
            self.DIO29(),
            self.DIO30(),
            self.DIO31()
        )
    }
}
#[doc = "Data Out 8 to 11 Alias register for byte access to each bit in DOUT31_0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DOUT11_8(pub u32);
impl DOUT11_8 {
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#8, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO8(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#8, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#9, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO9(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#9, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#10, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO10(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#10, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#11, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO11(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#11, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO11(&mut self, val: bool) {
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
impl Default for DOUT11_8 {
    #[inline(always)]
    fn default() -> DOUT11_8 {
        DOUT11_8(0)
    }
}
impl core::fmt::Debug for DOUT11_8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT11_8")
            .field("DIO8", &self.DIO8())
            .field("RESERVED1", &self.RESERVED1())
            .field("DIO9", &self.DIO9())
            .field("RESERVED9", &self.RESERVED9())
            .field("DIO10", &self.DIO10())
            .field("RESERVED17", &self.RESERVED17())
            .field("DIO11", &self.DIO11())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DOUT11_8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DOUT11_8 {{ DIO8: {=bool:?}, RESERVED1: {=u8:?}, DIO9: {=bool:?}, RESERVED9: {=u8:?}, DIO10: {=bool:?}, RESERVED17: {=u8:?}, DIO11: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.DIO8(),
            self.RESERVED1(),
            self.DIO9(),
            self.RESERVED9(),
            self.DIO10(),
            self.RESERVED17(),
            self.DIO11(),
            self.RESERVED25()
        )
    }
}
#[doc = "Data Out 12 to 15 Alias register for byte access to each bit in DOUT31_0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DOUT15_12(pub u32);
impl DOUT15_12 {
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#12, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO12(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#12, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#13, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO13(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#13, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#14, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO14(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#14, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#15, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO15(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#15, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO15(&mut self, val: bool) {
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
impl Default for DOUT15_12 {
    #[inline(always)]
    fn default() -> DOUT15_12 {
        DOUT15_12(0)
    }
}
impl core::fmt::Debug for DOUT15_12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT15_12")
            .field("DIO12", &self.DIO12())
            .field("RESERVED1", &self.RESERVED1())
            .field("DIO13", &self.DIO13())
            .field("RESERVED9", &self.RESERVED9())
            .field("DIO14", &self.DIO14())
            .field("RESERVED17", &self.RESERVED17())
            .field("DIO15", &self.DIO15())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DOUT15_12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DOUT15_12 {{ DIO12: {=bool:?}, RESERVED1: {=u8:?}, DIO13: {=bool:?}, RESERVED9: {=u8:?}, DIO14: {=bool:?}, RESERVED17: {=u8:?}, DIO15: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.DIO12(),
            self.RESERVED1(),
            self.DIO13(),
            self.RESERVED9(),
            self.DIO14(),
            self.RESERVED17(),
            self.DIO15(),
            self.RESERVED25()
        )
    }
}
#[doc = "Data Out 16 to 19 Alias register for byte access to each bit in DOUT31_0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DOUT19_16(pub u32);
impl DOUT19_16 {
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#16, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO16(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#16, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#17, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO17(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#17, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#18, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO18(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#18, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#19, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO19(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#19, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO19(&mut self, val: bool) {
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
impl Default for DOUT19_16 {
    #[inline(always)]
    fn default() -> DOUT19_16 {
        DOUT19_16(0)
    }
}
impl core::fmt::Debug for DOUT19_16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT19_16")
            .field("DIO16", &self.DIO16())
            .field("RESERVED1", &self.RESERVED1())
            .field("DIO17", &self.DIO17())
            .field("RESERVED9", &self.RESERVED9())
            .field("DIO18", &self.DIO18())
            .field("RESERVED17", &self.RESERVED17())
            .field("DIO19", &self.DIO19())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DOUT19_16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DOUT19_16 {{ DIO16: {=bool:?}, RESERVED1: {=u8:?}, DIO17: {=bool:?}, RESERVED9: {=u8:?}, DIO18: {=bool:?}, RESERVED17: {=u8:?}, DIO19: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.DIO16(),
            self.RESERVED1(),
            self.DIO17(),
            self.RESERVED9(),
            self.DIO18(),
            self.RESERVED17(),
            self.DIO19(),
            self.RESERVED25()
        )
    }
}
#[doc = "Data Out 20 to 23 Alias register for byte access to each bit in DOUT31_0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DOUT23_20(pub u32);
impl DOUT23_20 {
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#20, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO20(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#20, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#21, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO21(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#21, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#22, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO22(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#22, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#23, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO23(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#23, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO23(&mut self, val: bool) {
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
impl Default for DOUT23_20 {
    #[inline(always)]
    fn default() -> DOUT23_20 {
        DOUT23_20(0)
    }
}
impl core::fmt::Debug for DOUT23_20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT23_20")
            .field("DIO20", &self.DIO20())
            .field("RESERVED1", &self.RESERVED1())
            .field("DIO21", &self.DIO21())
            .field("RESERVED9", &self.RESERVED9())
            .field("DIO22", &self.DIO22())
            .field("RESERVED17", &self.RESERVED17())
            .field("DIO23", &self.DIO23())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DOUT23_20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DOUT23_20 {{ DIO20: {=bool:?}, RESERVED1: {=u8:?}, DIO21: {=bool:?}, RESERVED9: {=u8:?}, DIO22: {=bool:?}, RESERVED17: {=u8:?}, DIO23: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.DIO20(),
            self.RESERVED1(),
            self.DIO21(),
            self.RESERVED9(),
            self.DIO22(),
            self.RESERVED17(),
            self.DIO23(),
            self.RESERVED25()
        )
    }
}
#[doc = "Data Out 24 to 27 Alias register for byte access to each bit in DOUT31_0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DOUT27_24(pub u32);
impl DOUT27_24 {
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#24, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO24(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#24, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#25, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO25(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#25, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#26, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO26(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#26, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#27, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO27(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#27, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO27(&mut self, val: bool) {
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
impl Default for DOUT27_24 {
    #[inline(always)]
    fn default() -> DOUT27_24 {
        DOUT27_24(0)
    }
}
impl core::fmt::Debug for DOUT27_24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT27_24")
            .field("DIO24", &self.DIO24())
            .field("RESERVED1", &self.RESERVED1())
            .field("DIO25", &self.DIO25())
            .field("RESERVED9", &self.RESERVED9())
            .field("DIO26", &self.DIO26())
            .field("RESERVED17", &self.RESERVED17())
            .field("DIO27", &self.DIO27())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DOUT27_24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DOUT27_24 {{ DIO24: {=bool:?}, RESERVED1: {=u8:?}, DIO25: {=bool:?}, RESERVED9: {=u8:?}, DIO26: {=bool:?}, RESERVED17: {=u8:?}, DIO27: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.DIO24(),
            self.RESERVED1(),
            self.DIO25(),
            self.RESERVED9(),
            self.DIO26(),
            self.RESERVED17(),
            self.DIO27(),
            self.RESERVED25()
        )
    }
}
#[doc = "Data Output for DIO 0 to 31."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DOUT31_0(pub u32);
impl DOUT31_0 {
    #[doc = "0:0\\] Data output for DIO 0."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Data output for DIO 0."]
    #[inline(always)]
    pub const fn set_DIO0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Data output for DIO 1."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Data output for DIO 1."]
    #[inline(always)]
    pub const fn set_DIO1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Data output for DIO 2."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Data output for DIO 2."]
    #[inline(always)]
    pub const fn set_DIO2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Data output for DIO 3."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Data output for DIO 3."]
    #[inline(always)]
    pub const fn set_DIO3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Data output for DIO 4."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Data output for DIO 4."]
    #[inline(always)]
    pub const fn set_DIO4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Data output for DIO 5."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Data output for DIO 5."]
    #[inline(always)]
    pub const fn set_DIO5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Data output for DIO 6."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Data output for DIO 6."]
    #[inline(always)]
    pub const fn set_DIO6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Data output for DIO 7."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Data output for DIO 7."]
    #[inline(always)]
    pub const fn set_DIO7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Data output for DIO 8."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Data output for DIO 8."]
    #[inline(always)]
    pub const fn set_DIO8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Data output for DIO 9."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Data output for DIO 9."]
    #[inline(always)]
    pub const fn set_DIO9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Data output for DIO 10."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Data output for DIO 10."]
    #[inline(always)]
    pub const fn set_DIO10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Data output for DIO 11."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Data output for DIO 11."]
    #[inline(always)]
    pub const fn set_DIO11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Data output for DIO 12."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Data output for DIO 12."]
    #[inline(always)]
    pub const fn set_DIO12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Data output for DIO 13."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Data output for DIO 13."]
    #[inline(always)]
    pub const fn set_DIO13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Data output for DIO 14."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Data output for DIO 14."]
    #[inline(always)]
    pub const fn set_DIO14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Data output for DIO 15."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Data output for DIO 15."]
    #[inline(always)]
    pub const fn set_DIO15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Data output for DIO 16."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Data output for DIO 16."]
    #[inline(always)]
    pub const fn set_DIO16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Data output for DIO 17."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Data output for DIO 17."]
    #[inline(always)]
    pub const fn set_DIO17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Data output for DIO 18."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Data output for DIO 18."]
    #[inline(always)]
    pub const fn set_DIO18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Data output for DIO 19."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Data output for DIO 19."]
    #[inline(always)]
    pub const fn set_DIO19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Data output for DIO 20."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Data output for DIO 20."]
    #[inline(always)]
    pub const fn set_DIO20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Data output for DIO 21."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Data output for DIO 21."]
    #[inline(always)]
    pub const fn set_DIO21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Data output for DIO 22."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Data output for DIO 22."]
    #[inline(always)]
    pub const fn set_DIO22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Data output for DIO 23."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Data output for DIO 23."]
    #[inline(always)]
    pub const fn set_DIO23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Data output for DIO 24."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Data output for DIO 24."]
    #[inline(always)]
    pub const fn set_DIO24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Data output for DIO 25."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Data output for DIO 25."]
    #[inline(always)]
    pub const fn set_DIO25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Data output for DIO 26."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Data output for DIO 26."]
    #[inline(always)]
    pub const fn set_DIO26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Data output for DIO 27."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Data output for DIO 27."]
    #[inline(always)]
    pub const fn set_DIO27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Data output for DIO 28."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Data output for DIO 28."]
    #[inline(always)]
    pub const fn set_DIO28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Data output for DIO 29."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Data output for DIO 29."]
    #[inline(always)]
    pub const fn set_DIO29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Data output for DIO 30."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Data output for DIO 30."]
    #[inline(always)]
    pub const fn set_DIO30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Data output for DIO 31."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Data output for DIO 31."]
    #[inline(always)]
    pub const fn set_DIO31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DOUT31_0 {
    #[inline(always)]
    fn default() -> DOUT31_0 {
        DOUT31_0(0)
    }
}
impl core::fmt::Debug for DOUT31_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT31_0")
            .field("DIO0", &self.DIO0())
            .field("DIO1", &self.DIO1())
            .field("DIO2", &self.DIO2())
            .field("DIO3", &self.DIO3())
            .field("DIO4", &self.DIO4())
            .field("DIO5", &self.DIO5())
            .field("DIO6", &self.DIO6())
            .field("DIO7", &self.DIO7())
            .field("DIO8", &self.DIO8())
            .field("DIO9", &self.DIO9())
            .field("DIO10", &self.DIO10())
            .field("DIO11", &self.DIO11())
            .field("DIO12", &self.DIO12())
            .field("DIO13", &self.DIO13())
            .field("DIO14", &self.DIO14())
            .field("DIO15", &self.DIO15())
            .field("DIO16", &self.DIO16())
            .field("DIO17", &self.DIO17())
            .field("DIO18", &self.DIO18())
            .field("DIO19", &self.DIO19())
            .field("DIO20", &self.DIO20())
            .field("DIO21", &self.DIO21())
            .field("DIO22", &self.DIO22())
            .field("DIO23", &self.DIO23())
            .field("DIO24", &self.DIO24())
            .field("DIO25", &self.DIO25())
            .field("DIO26", &self.DIO26())
            .field("DIO27", &self.DIO27())
            .field("DIO28", &self.DIO28())
            .field("DIO29", &self.DIO29())
            .field("DIO30", &self.DIO30())
            .field("DIO31", &self.DIO31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DOUT31_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DOUT31_0 {{ DIO0: {=bool:?}, DIO1: {=bool:?}, DIO2: {=bool:?}, DIO3: {=bool:?}, DIO4: {=bool:?}, DIO5: {=bool:?}, DIO6: {=bool:?}, DIO7: {=bool:?}, DIO8: {=bool:?}, DIO9: {=bool:?}, DIO10: {=bool:?}, DIO11: {=bool:?}, DIO12: {=bool:?}, DIO13: {=bool:?}, DIO14: {=bool:?}, DIO15: {=bool:?}, DIO16: {=bool:?}, DIO17: {=bool:?}, DIO18: {=bool:?}, DIO19: {=bool:?}, DIO20: {=bool:?}, DIO21: {=bool:?}, DIO22: {=bool:?}, DIO23: {=bool:?}, DIO24: {=bool:?}, DIO25: {=bool:?}, DIO26: {=bool:?}, DIO27: {=bool:?}, DIO28: {=bool:?}, DIO29: {=bool:?}, DIO30: {=bool:?}, DIO31: {=bool:?} }}",
            self.DIO0(),
            self.DIO1(),
            self.DIO2(),
            self.DIO3(),
            self.DIO4(),
            self.DIO5(),
            self.DIO6(),
            self.DIO7(),
            self.DIO8(),
            self.DIO9(),
            self.DIO10(),
            self.DIO11(),
            self.DIO12(),
            self.DIO13(),
            self.DIO14(),
            self.DIO15(),
            self.DIO16(),
            self.DIO17(),
            self.DIO18(),
            self.DIO19(),
            self.DIO20(),
            self.DIO21(),
            self.DIO22(),
            self.DIO23(),
            self.DIO24(),
            self.DIO25(),
            self.DIO26(),
            self.DIO27(),
            self.DIO28(),
            self.DIO29(),
            self.DIO30(),
            self.DIO31()
        )
    }
}
#[doc = "Data Out 28 to 31 Alias register for byte access to each bit in DOUT31_0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DOUT31_28(pub u32);
impl DOUT31_28 {
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#28, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO28(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#28, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#29, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO29(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#29, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#30, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO30(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#30, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#31, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO31(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#31, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO31(&mut self, val: bool) {
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
impl Default for DOUT31_28 {
    #[inline(always)]
    fn default() -> DOUT31_28 {
        DOUT31_28(0)
    }
}
impl core::fmt::Debug for DOUT31_28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT31_28")
            .field("DIO28", &self.DIO28())
            .field("RESERVED1", &self.RESERVED1())
            .field("DIO29", &self.DIO29())
            .field("RESERVED9", &self.RESERVED9())
            .field("DIO30", &self.DIO30())
            .field("RESERVED17", &self.RESERVED17())
            .field("DIO31", &self.DIO31())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DOUT31_28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DOUT31_28 {{ DIO28: {=bool:?}, RESERVED1: {=u8:?}, DIO29: {=bool:?}, RESERVED9: {=u8:?}, DIO30: {=bool:?}, RESERVED17: {=u8:?}, DIO31: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.DIO28(),
            self.RESERVED1(),
            self.DIO29(),
            self.RESERVED9(),
            self.DIO30(),
            self.RESERVED17(),
            self.DIO31(),
            self.RESERVED25()
        )
    }
}
#[doc = "Data Out 0 to 3 Alias register for byte access to each bit in DOUT31_0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DOUT3_0(pub u32);
impl DOUT3_0 {
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#0, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#0, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#1, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#1, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#2, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#2, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#3, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO3(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#3, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO3(&mut self, val: bool) {
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
impl Default for DOUT3_0 {
    #[inline(always)]
    fn default() -> DOUT3_0 {
        DOUT3_0(0)
    }
}
impl core::fmt::Debug for DOUT3_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT3_0")
            .field("DIO0", &self.DIO0())
            .field("RESERVED1", &self.RESERVED1())
            .field("DIO1", &self.DIO1())
            .field("RESERVED9", &self.RESERVED9())
            .field("DIO2", &self.DIO2())
            .field("RESERVED17", &self.RESERVED17())
            .field("DIO3", &self.DIO3())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DOUT3_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DOUT3_0 {{ DIO0: {=bool:?}, RESERVED1: {=u8:?}, DIO1: {=bool:?}, RESERVED9: {=u8:?}, DIO2: {=bool:?}, RESERVED17: {=u8:?}, DIO3: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.DIO0(),
            self.RESERVED1(),
            self.DIO1(),
            self.RESERVED9(),
            self.DIO2(),
            self.RESERVED17(),
            self.DIO3(),
            self.RESERVED25()
        )
    }
}
#[doc = "Data Out 4 to 7 Alias register for byte access to each bit in DOUT31_0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DOUT7_4(pub u32);
impl DOUT7_4 {
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#4, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO4(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Sets the state of the pin that is configured as DIO#4, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "7:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#5, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO5(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Sets the state of the pin that is configured as DIO#5, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED9(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "15:9\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#6, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO6(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Sets the state of the pin that is configured as DIO#6, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn RESERVED17(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "23:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn set_RESERVED17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#7, if the corresponding DOE31_0 bitfield is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO7(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Sets the state of the pin that is configured as DIO#7, if the corresponding DOE31_0 bitfield is set."]
    #[inline(always)]
    pub const fn set_DIO7(&mut self, val: bool) {
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
impl Default for DOUT7_4 {
    #[inline(always)]
    fn default() -> DOUT7_4 {
        DOUT7_4(0)
    }
}
impl core::fmt::Debug for DOUT7_4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT7_4")
            .field("DIO4", &self.DIO4())
            .field("RESERVED1", &self.RESERVED1())
            .field("DIO5", &self.DIO5())
            .field("RESERVED9", &self.RESERVED9())
            .field("DIO6", &self.DIO6())
            .field("RESERVED17", &self.RESERVED17())
            .field("DIO7", &self.DIO7())
            .field("RESERVED25", &self.RESERVED25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DOUT7_4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DOUT7_4 {{ DIO4: {=bool:?}, RESERVED1: {=u8:?}, DIO5: {=bool:?}, RESERVED9: {=u8:?}, DIO6: {=bool:?}, RESERVED17: {=u8:?}, DIO7: {=bool:?}, RESERVED25: {=u8:?} }}",
            self.DIO4(),
            self.RESERVED1(),
            self.DIO5(),
            self.RESERVED9(),
            self.DIO6(),
            self.RESERVED17(),
            self.DIO7(),
            self.RESERVED25()
        )
    }
}
#[doc = "Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT31_0 register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DOUTCLR31_0(pub u32);
impl DOUTCLR31_0 {
    #[doc = "0:0\\] Clears bit 0."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Clears bit 0."]
    #[inline(always)]
    pub const fn set_DIO0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Clears bit 1."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Clears bit 1."]
    #[inline(always)]
    pub const fn set_DIO1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Clears bit 2."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Clears bit 2."]
    #[inline(always)]
    pub const fn set_DIO2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Clears bit 3."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Clears bit 3."]
    #[inline(always)]
    pub const fn set_DIO3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Clears bit 4."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Clears bit 4."]
    #[inline(always)]
    pub const fn set_DIO4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Clears bit 5."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Clears bit 5."]
    #[inline(always)]
    pub const fn set_DIO5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Clears bit 6."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Clears bit 6."]
    #[inline(always)]
    pub const fn set_DIO6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Clears bit 7."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Clears bit 7."]
    #[inline(always)]
    pub const fn set_DIO7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Clears bit 8."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Clears bit 8."]
    #[inline(always)]
    pub const fn set_DIO8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Clears bit 9."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Clears bit 9."]
    #[inline(always)]
    pub const fn set_DIO9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Clears bit 10."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Clears bit 10."]
    #[inline(always)]
    pub const fn set_DIO10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Clears bit 11."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Clears bit 11."]
    #[inline(always)]
    pub const fn set_DIO11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Clears bit 12."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Clears bit 12."]
    #[inline(always)]
    pub const fn set_DIO12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Clears bit 13."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Clears bit 13."]
    #[inline(always)]
    pub const fn set_DIO13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Clears bit 14."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Clears bit 14."]
    #[inline(always)]
    pub const fn set_DIO14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Clears bit 15."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Clears bit 15."]
    #[inline(always)]
    pub const fn set_DIO15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Clears bit 16."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Clears bit 16."]
    #[inline(always)]
    pub const fn set_DIO16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Clears bit 17."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Clears bit 17."]
    #[inline(always)]
    pub const fn set_DIO17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Clears bit 18."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Clears bit 18."]
    #[inline(always)]
    pub const fn set_DIO18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Clears bit 19."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Clears bit 19."]
    #[inline(always)]
    pub const fn set_DIO19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Clears bit 20."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Clears bit 20."]
    #[inline(always)]
    pub const fn set_DIO20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Clears bit 21."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Clears bit 21."]
    #[inline(always)]
    pub const fn set_DIO21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Clears bit 22."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Clears bit 22."]
    #[inline(always)]
    pub const fn set_DIO22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Clears bit 23."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Clears bit 23."]
    #[inline(always)]
    pub const fn set_DIO23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Clears bit 24."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Clears bit 24."]
    #[inline(always)]
    pub const fn set_DIO24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Clears bit 25."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Clears bit 25."]
    #[inline(always)]
    pub const fn set_DIO25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Clears bit 26."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Clears bit 26."]
    #[inline(always)]
    pub const fn set_DIO26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Clears bit 27."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Clears bit 27."]
    #[inline(always)]
    pub const fn set_DIO27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Clears bit 28."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Clears bit 28."]
    #[inline(always)]
    pub const fn set_DIO28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Clears bit 29."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Clears bit 29."]
    #[inline(always)]
    pub const fn set_DIO29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Clears bit 30."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Clears bit 30."]
    #[inline(always)]
    pub const fn set_DIO30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Clears bit 31."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Clears bit 31."]
    #[inline(always)]
    pub const fn set_DIO31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DOUTCLR31_0 {
    #[inline(always)]
    fn default() -> DOUTCLR31_0 {
        DOUTCLR31_0(0)
    }
}
impl core::fmt::Debug for DOUTCLR31_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTCLR31_0")
            .field("DIO0", &self.DIO0())
            .field("DIO1", &self.DIO1())
            .field("DIO2", &self.DIO2())
            .field("DIO3", &self.DIO3())
            .field("DIO4", &self.DIO4())
            .field("DIO5", &self.DIO5())
            .field("DIO6", &self.DIO6())
            .field("DIO7", &self.DIO7())
            .field("DIO8", &self.DIO8())
            .field("DIO9", &self.DIO9())
            .field("DIO10", &self.DIO10())
            .field("DIO11", &self.DIO11())
            .field("DIO12", &self.DIO12())
            .field("DIO13", &self.DIO13())
            .field("DIO14", &self.DIO14())
            .field("DIO15", &self.DIO15())
            .field("DIO16", &self.DIO16())
            .field("DIO17", &self.DIO17())
            .field("DIO18", &self.DIO18())
            .field("DIO19", &self.DIO19())
            .field("DIO20", &self.DIO20())
            .field("DIO21", &self.DIO21())
            .field("DIO22", &self.DIO22())
            .field("DIO23", &self.DIO23())
            .field("DIO24", &self.DIO24())
            .field("DIO25", &self.DIO25())
            .field("DIO26", &self.DIO26())
            .field("DIO27", &self.DIO27())
            .field("DIO28", &self.DIO28())
            .field("DIO29", &self.DIO29())
            .field("DIO30", &self.DIO30())
            .field("DIO31", &self.DIO31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DOUTCLR31_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DOUTCLR31_0 {{ DIO0: {=bool:?}, DIO1: {=bool:?}, DIO2: {=bool:?}, DIO3: {=bool:?}, DIO4: {=bool:?}, DIO5: {=bool:?}, DIO6: {=bool:?}, DIO7: {=bool:?}, DIO8: {=bool:?}, DIO9: {=bool:?}, DIO10: {=bool:?}, DIO11: {=bool:?}, DIO12: {=bool:?}, DIO13: {=bool:?}, DIO14: {=bool:?}, DIO15: {=bool:?}, DIO16: {=bool:?}, DIO17: {=bool:?}, DIO18: {=bool:?}, DIO19: {=bool:?}, DIO20: {=bool:?}, DIO21: {=bool:?}, DIO22: {=bool:?}, DIO23: {=bool:?}, DIO24: {=bool:?}, DIO25: {=bool:?}, DIO26: {=bool:?}, DIO27: {=bool:?}, DIO28: {=bool:?}, DIO29: {=bool:?}, DIO30: {=bool:?}, DIO31: {=bool:?} }}",
            self.DIO0(),
            self.DIO1(),
            self.DIO2(),
            self.DIO3(),
            self.DIO4(),
            self.DIO5(),
            self.DIO6(),
            self.DIO7(),
            self.DIO8(),
            self.DIO9(),
            self.DIO10(),
            self.DIO11(),
            self.DIO12(),
            self.DIO13(),
            self.DIO14(),
            self.DIO15(),
            self.DIO16(),
            self.DIO17(),
            self.DIO18(),
            self.DIO19(),
            self.DIO20(),
            self.DIO21(),
            self.DIO22(),
            self.DIO23(),
            self.DIO24(),
            self.DIO25(),
            self.DIO26(),
            self.DIO27(),
            self.DIO28(),
            self.DIO29(),
            self.DIO30(),
            self.DIO31()
        )
    }
}
#[doc = "Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT31_0 register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DOUTSET31_0(pub u32);
impl DOUTSET31_0 {
    #[doc = "0:0\\] Set bit 0."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Set bit 0."]
    #[inline(always)]
    pub const fn set_DIO0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Set bit 1."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Set bit 1."]
    #[inline(always)]
    pub const fn set_DIO1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Set bit 2."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Set bit 2."]
    #[inline(always)]
    pub const fn set_DIO2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Set bit 3."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Set bit 3."]
    #[inline(always)]
    pub const fn set_DIO3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Set bit 4."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Set bit 4."]
    #[inline(always)]
    pub const fn set_DIO4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Set bit 5."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Set bit 5."]
    #[inline(always)]
    pub const fn set_DIO5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Set bit 6."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Set bit 6."]
    #[inline(always)]
    pub const fn set_DIO6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Set bit 7."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Set bit 7."]
    #[inline(always)]
    pub const fn set_DIO7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Set bit 8."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Set bit 8."]
    #[inline(always)]
    pub const fn set_DIO8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Set bit 9."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Set bit 9."]
    #[inline(always)]
    pub const fn set_DIO9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Set bit 10."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Set bit 10."]
    #[inline(always)]
    pub const fn set_DIO10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Set bit 11."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Set bit 11."]
    #[inline(always)]
    pub const fn set_DIO11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Set bit 12."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Set bit 12."]
    #[inline(always)]
    pub const fn set_DIO12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Set bit 13."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Set bit 13."]
    #[inline(always)]
    pub const fn set_DIO13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Set bit 14."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Set bit 14."]
    #[inline(always)]
    pub const fn set_DIO14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Set bit 15."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Set bit 15."]
    #[inline(always)]
    pub const fn set_DIO15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Set bit 16."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Set bit 16."]
    #[inline(always)]
    pub const fn set_DIO16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Set bit 17."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Set bit 17."]
    #[inline(always)]
    pub const fn set_DIO17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Set bit 18."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Set bit 18."]
    #[inline(always)]
    pub const fn set_DIO18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Set bit 19."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Set bit 19."]
    #[inline(always)]
    pub const fn set_DIO19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Set bit 20."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Set bit 20."]
    #[inline(always)]
    pub const fn set_DIO20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Set bit 21."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Set bit 21."]
    #[inline(always)]
    pub const fn set_DIO21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Set bit 22."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Set bit 22."]
    #[inline(always)]
    pub const fn set_DIO22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Set bit 23."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Set bit 23."]
    #[inline(always)]
    pub const fn set_DIO23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Set bit 24."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Set bit 24."]
    #[inline(always)]
    pub const fn set_DIO24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Set bit 25."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Set bit 25."]
    #[inline(always)]
    pub const fn set_DIO25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Set bit 26."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Set bit 26."]
    #[inline(always)]
    pub const fn set_DIO26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Set bit 27."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Set bit 27."]
    #[inline(always)]
    pub const fn set_DIO27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Set bit 28."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Set bit 28."]
    #[inline(always)]
    pub const fn set_DIO28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Set bit 29."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Set bit 29."]
    #[inline(always)]
    pub const fn set_DIO29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Set bit 30."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Set bit 30."]
    #[inline(always)]
    pub const fn set_DIO30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Set bit 31."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Set bit 31."]
    #[inline(always)]
    pub const fn set_DIO31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DOUTSET31_0 {
    #[inline(always)]
    fn default() -> DOUTSET31_0 {
        DOUTSET31_0(0)
    }
}
impl core::fmt::Debug for DOUTSET31_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTSET31_0")
            .field("DIO0", &self.DIO0())
            .field("DIO1", &self.DIO1())
            .field("DIO2", &self.DIO2())
            .field("DIO3", &self.DIO3())
            .field("DIO4", &self.DIO4())
            .field("DIO5", &self.DIO5())
            .field("DIO6", &self.DIO6())
            .field("DIO7", &self.DIO7())
            .field("DIO8", &self.DIO8())
            .field("DIO9", &self.DIO9())
            .field("DIO10", &self.DIO10())
            .field("DIO11", &self.DIO11())
            .field("DIO12", &self.DIO12())
            .field("DIO13", &self.DIO13())
            .field("DIO14", &self.DIO14())
            .field("DIO15", &self.DIO15())
            .field("DIO16", &self.DIO16())
            .field("DIO17", &self.DIO17())
            .field("DIO18", &self.DIO18())
            .field("DIO19", &self.DIO19())
            .field("DIO20", &self.DIO20())
            .field("DIO21", &self.DIO21())
            .field("DIO22", &self.DIO22())
            .field("DIO23", &self.DIO23())
            .field("DIO24", &self.DIO24())
            .field("DIO25", &self.DIO25())
            .field("DIO26", &self.DIO26())
            .field("DIO27", &self.DIO27())
            .field("DIO28", &self.DIO28())
            .field("DIO29", &self.DIO29())
            .field("DIO30", &self.DIO30())
            .field("DIO31", &self.DIO31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DOUTSET31_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DOUTSET31_0 {{ DIO0: {=bool:?}, DIO1: {=bool:?}, DIO2: {=bool:?}, DIO3: {=bool:?}, DIO4: {=bool:?}, DIO5: {=bool:?}, DIO6: {=bool:?}, DIO7: {=bool:?}, DIO8: {=bool:?}, DIO9: {=bool:?}, DIO10: {=bool:?}, DIO11: {=bool:?}, DIO12: {=bool:?}, DIO13: {=bool:?}, DIO14: {=bool:?}, DIO15: {=bool:?}, DIO16: {=bool:?}, DIO17: {=bool:?}, DIO18: {=bool:?}, DIO19: {=bool:?}, DIO20: {=bool:?}, DIO21: {=bool:?}, DIO22: {=bool:?}, DIO23: {=bool:?}, DIO24: {=bool:?}, DIO25: {=bool:?}, DIO26: {=bool:?}, DIO27: {=bool:?}, DIO28: {=bool:?}, DIO29: {=bool:?}, DIO30: {=bool:?}, DIO31: {=bool:?} }}",
            self.DIO0(),
            self.DIO1(),
            self.DIO2(),
            self.DIO3(),
            self.DIO4(),
            self.DIO5(),
            self.DIO6(),
            self.DIO7(),
            self.DIO8(),
            self.DIO9(),
            self.DIO10(),
            self.DIO11(),
            self.DIO12(),
            self.DIO13(),
            self.DIO14(),
            self.DIO15(),
            self.DIO16(),
            self.DIO17(),
            self.DIO18(),
            self.DIO19(),
            self.DIO20(),
            self.DIO21(),
            self.DIO22(),
            self.DIO23(),
            self.DIO24(),
            self.DIO25(),
            self.DIO26(),
            self.DIO27(),
            self.DIO28(),
            self.DIO29(),
            self.DIO30(),
            self.DIO31()
        )
    }
}
#[doc = "Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DOUTTGL31_0(pub u32);
impl DOUTTGL31_0 {
    #[doc = "0:0\\] Toggles bit 0."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Toggles bit 0."]
    #[inline(always)]
    pub const fn set_DIO0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Toggles bit 1."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Toggles bit 1."]
    #[inline(always)]
    pub const fn set_DIO1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Toggles bit 2."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Toggles bit 2."]
    #[inline(always)]
    pub const fn set_DIO2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Toggles bit 3."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Toggles bit 3."]
    #[inline(always)]
    pub const fn set_DIO3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Toggles bit 4."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Toggles bit 4."]
    #[inline(always)]
    pub const fn set_DIO4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Toggles bit 5."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Toggles bit 5."]
    #[inline(always)]
    pub const fn set_DIO5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Toggles bit 6."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Toggles bit 6."]
    #[inline(always)]
    pub const fn set_DIO6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Toggles bit 7."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Toggles bit 7."]
    #[inline(always)]
    pub const fn set_DIO7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Toggles bit 8."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Toggles bit 8."]
    #[inline(always)]
    pub const fn set_DIO8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Toggles bit 9."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Toggles bit 9."]
    #[inline(always)]
    pub const fn set_DIO9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Toggles bit 10."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Toggles bit 10."]
    #[inline(always)]
    pub const fn set_DIO10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Toggles bit 11."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Toggles bit 11."]
    #[inline(always)]
    pub const fn set_DIO11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Toggles bit 12."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Toggles bit 12."]
    #[inline(always)]
    pub const fn set_DIO12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Toggles bit 13."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Toggles bit 13."]
    #[inline(always)]
    pub const fn set_DIO13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Toggles bit 14."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Toggles bit 14."]
    #[inline(always)]
    pub const fn set_DIO14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Toggles bit 15."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Toggles bit 15."]
    #[inline(always)]
    pub const fn set_DIO15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Toggles bit 16."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Toggles bit 16."]
    #[inline(always)]
    pub const fn set_DIO16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Toggles bit 17."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Toggles bit 17."]
    #[inline(always)]
    pub const fn set_DIO17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Toggles bit 18."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Toggles bit 18."]
    #[inline(always)]
    pub const fn set_DIO18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Toggles bit 19."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Toggles bit 19."]
    #[inline(always)]
    pub const fn set_DIO19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Toggles bit 20."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Toggles bit 20."]
    #[inline(always)]
    pub const fn set_DIO20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Toggles bit 21."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Toggles bit 21."]
    #[inline(always)]
    pub const fn set_DIO21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Toggles bit 22."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Toggles bit 22."]
    #[inline(always)]
    pub const fn set_DIO22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Toggles bit 23."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Toggles bit 23."]
    #[inline(always)]
    pub const fn set_DIO23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Toggles bit 24."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Toggles bit 24."]
    #[inline(always)]
    pub const fn set_DIO24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Toggles bit 25."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Toggles bit 25."]
    #[inline(always)]
    pub const fn set_DIO25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Toggles bit 26."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Toggles bit 26."]
    #[inline(always)]
    pub const fn set_DIO26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Toggles bit 27."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Toggles bit 27."]
    #[inline(always)]
    pub const fn set_DIO27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Toggles bit 28."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Toggles bit 28."]
    #[inline(always)]
    pub const fn set_DIO28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Toggles bit 29."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Toggles bit 29."]
    #[inline(always)]
    pub const fn set_DIO29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Toggles bit 30."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Toggles bit 30."]
    #[inline(always)]
    pub const fn set_DIO30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Toggles bit 31."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Toggles bit 31."]
    #[inline(always)]
    pub const fn set_DIO31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DOUTTGL31_0 {
    #[inline(always)]
    fn default() -> DOUTTGL31_0 {
        DOUTTGL31_0(0)
    }
}
impl core::fmt::Debug for DOUTTGL31_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUTTGL31_0")
            .field("DIO0", &self.DIO0())
            .field("DIO1", &self.DIO1())
            .field("DIO2", &self.DIO2())
            .field("DIO3", &self.DIO3())
            .field("DIO4", &self.DIO4())
            .field("DIO5", &self.DIO5())
            .field("DIO6", &self.DIO6())
            .field("DIO7", &self.DIO7())
            .field("DIO8", &self.DIO8())
            .field("DIO9", &self.DIO9())
            .field("DIO10", &self.DIO10())
            .field("DIO11", &self.DIO11())
            .field("DIO12", &self.DIO12())
            .field("DIO13", &self.DIO13())
            .field("DIO14", &self.DIO14())
            .field("DIO15", &self.DIO15())
            .field("DIO16", &self.DIO16())
            .field("DIO17", &self.DIO17())
            .field("DIO18", &self.DIO18())
            .field("DIO19", &self.DIO19())
            .field("DIO20", &self.DIO20())
            .field("DIO21", &self.DIO21())
            .field("DIO22", &self.DIO22())
            .field("DIO23", &self.DIO23())
            .field("DIO24", &self.DIO24())
            .field("DIO25", &self.DIO25())
            .field("DIO26", &self.DIO26())
            .field("DIO27", &self.DIO27())
            .field("DIO28", &self.DIO28())
            .field("DIO29", &self.DIO29())
            .field("DIO30", &self.DIO30())
            .field("DIO31", &self.DIO31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DOUTTGL31_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DOUTTGL31_0 {{ DIO0: {=bool:?}, DIO1: {=bool:?}, DIO2: {=bool:?}, DIO3: {=bool:?}, DIO4: {=bool:?}, DIO5: {=bool:?}, DIO6: {=bool:?}, DIO7: {=bool:?}, DIO8: {=bool:?}, DIO9: {=bool:?}, DIO10: {=bool:?}, DIO11: {=bool:?}, DIO12: {=bool:?}, DIO13: {=bool:?}, DIO14: {=bool:?}, DIO15: {=bool:?}, DIO16: {=bool:?}, DIO17: {=bool:?}, DIO18: {=bool:?}, DIO19: {=bool:?}, DIO20: {=bool:?}, DIO21: {=bool:?}, DIO22: {=bool:?}, DIO23: {=bool:?}, DIO24: {=bool:?}, DIO25: {=bool:?}, DIO26: {=bool:?}, DIO27: {=bool:?}, DIO28: {=bool:?}, DIO29: {=bool:?}, DIO30: {=bool:?}, DIO31: {=bool:?} }}",
            self.DIO0(),
            self.DIO1(),
            self.DIO2(),
            self.DIO3(),
            self.DIO4(),
            self.DIO5(),
            self.DIO6(),
            self.DIO7(),
            self.DIO8(),
            self.DIO9(),
            self.DIO10(),
            self.DIO11(),
            self.DIO12(),
            self.DIO13(),
            self.DIO14(),
            self.DIO15(),
            self.DIO16(),
            self.DIO17(),
            self.DIO18(),
            self.DIO19(),
            self.DIO20(),
            self.DIO21(),
            self.DIO22(),
            self.DIO23(),
            self.DIO24(),
            self.DIO25(),
            self.DIO26(),
            self.DIO27(),
            self.DIO28(),
            self.DIO29(),
            self.DIO30(),
            self.DIO31()
        )
    }
}
#[doc = "Event Register for DIO 0 to 31 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVFLAGS31_0(pub u32);
impl EVFLAGS31_0 {
    #[doc = "0:0\\] Event for DIO 0."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0:0\\] Event for DIO 0."]
    #[inline(always)]
    pub const fn set_DIO0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:1\\] Event for DIO 1."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:1\\] Event for DIO 1."]
    #[inline(always)]
    pub const fn set_DIO1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "2:2\\] Event for DIO 2."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "2:2\\] Event for DIO 2."]
    #[inline(always)]
    pub const fn set_DIO2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "3:3\\] Event for DIO 3."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "3:3\\] Event for DIO 3."]
    #[inline(always)]
    pub const fn set_DIO3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4:4\\] Event for DIO 4."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "4:4\\] Event for DIO 4."]
    #[inline(always)]
    pub const fn set_DIO4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "5:5\\] Event for DIO 5."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "5:5\\] Event for DIO 5."]
    #[inline(always)]
    pub const fn set_DIO5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "6:6\\] Event for DIO 6."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "6:6\\] Event for DIO 6."]
    #[inline(always)]
    pub const fn set_DIO6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "7:7\\] Event for DIO 7."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "7:7\\] Event for DIO 7."]
    #[inline(always)]
    pub const fn set_DIO7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "8:8\\] Event for DIO 8."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "8:8\\] Event for DIO 8."]
    #[inline(always)]
    pub const fn set_DIO8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "9:9\\] Event for DIO 9."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "9:9\\] Event for DIO 9."]
    #[inline(always)]
    pub const fn set_DIO9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "10:10\\] Event for DIO 10."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "10:10\\] Event for DIO 10."]
    #[inline(always)]
    pub const fn set_DIO10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "11:11\\] Event for DIO 11."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "11:11\\] Event for DIO 11."]
    #[inline(always)]
    pub const fn set_DIO11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "12:12\\] Event for DIO 12."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "12:12\\] Event for DIO 12."]
    #[inline(always)]
    pub const fn set_DIO12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "13:13\\] Event for DIO 13."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "13:13\\] Event for DIO 13."]
    #[inline(always)]
    pub const fn set_DIO13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "14:14\\] Event for DIO 14."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "14:14\\] Event for DIO 14."]
    #[inline(always)]
    pub const fn set_DIO14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "15:15\\] Event for DIO 15."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "15:15\\] Event for DIO 15."]
    #[inline(always)]
    pub const fn set_DIO15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "16:16\\] Event for DIO 16."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "16:16\\] Event for DIO 16."]
    #[inline(always)]
    pub const fn set_DIO16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "17:17\\] Event for DIO 17."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "17:17\\] Event for DIO 17."]
    #[inline(always)]
    pub const fn set_DIO17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "18:18\\] Event for DIO 18."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "18:18\\] Event for DIO 18."]
    #[inline(always)]
    pub const fn set_DIO18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "19:19\\] Event for DIO 19."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "19:19\\] Event for DIO 19."]
    #[inline(always)]
    pub const fn set_DIO19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "20:20\\] Event for DIO 20."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "20:20\\] Event for DIO 20."]
    #[inline(always)]
    pub const fn set_DIO20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "21:21\\] Event for DIO 21."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "21:21\\] Event for DIO 21."]
    #[inline(always)]
    pub const fn set_DIO21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "22:22\\] Event for DIO 22."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "22:22\\] Event for DIO 22."]
    #[inline(always)]
    pub const fn set_DIO22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "23:23\\] Event for DIO 23."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "23:23\\] Event for DIO 23."]
    #[inline(always)]
    pub const fn set_DIO23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "24:24\\] Event for DIO 24."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "24:24\\] Event for DIO 24."]
    #[inline(always)]
    pub const fn set_DIO24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "25:25\\] Event for DIO 25."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "25:25\\] Event for DIO 25."]
    #[inline(always)]
    pub const fn set_DIO25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "26:26\\] Event for DIO 26."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "26:26\\] Event for DIO 26."]
    #[inline(always)]
    pub const fn set_DIO26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "27:27\\] Event for DIO 27."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "27:27\\] Event for DIO 27."]
    #[inline(always)]
    pub const fn set_DIO27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "28:28\\] Event for DIO 28."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "28:28\\] Event for DIO 28."]
    #[inline(always)]
    pub const fn set_DIO28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "29:29\\] Event for DIO 29."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "29:29\\] Event for DIO 29."]
    #[inline(always)]
    pub const fn set_DIO29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "30:30\\] Event for DIO 30."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "30:30\\] Event for DIO 30."]
    #[inline(always)]
    pub const fn set_DIO30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "31:31\\] Event for DIO 31."]
    #[must_use]
    #[inline(always)]
    pub const fn DIO31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "31:31\\] Event for DIO 31."]
    #[inline(always)]
    pub const fn set_DIO31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for EVFLAGS31_0 {
    #[inline(always)]
    fn default() -> EVFLAGS31_0 {
        EVFLAGS31_0(0)
    }
}
impl core::fmt::Debug for EVFLAGS31_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVFLAGS31_0")
            .field("DIO0", &self.DIO0())
            .field("DIO1", &self.DIO1())
            .field("DIO2", &self.DIO2())
            .field("DIO3", &self.DIO3())
            .field("DIO4", &self.DIO4())
            .field("DIO5", &self.DIO5())
            .field("DIO6", &self.DIO6())
            .field("DIO7", &self.DIO7())
            .field("DIO8", &self.DIO8())
            .field("DIO9", &self.DIO9())
            .field("DIO10", &self.DIO10())
            .field("DIO11", &self.DIO11())
            .field("DIO12", &self.DIO12())
            .field("DIO13", &self.DIO13())
            .field("DIO14", &self.DIO14())
            .field("DIO15", &self.DIO15())
            .field("DIO16", &self.DIO16())
            .field("DIO17", &self.DIO17())
            .field("DIO18", &self.DIO18())
            .field("DIO19", &self.DIO19())
            .field("DIO20", &self.DIO20())
            .field("DIO21", &self.DIO21())
            .field("DIO22", &self.DIO22())
            .field("DIO23", &self.DIO23())
            .field("DIO24", &self.DIO24())
            .field("DIO25", &self.DIO25())
            .field("DIO26", &self.DIO26())
            .field("DIO27", &self.DIO27())
            .field("DIO28", &self.DIO28())
            .field("DIO29", &self.DIO29())
            .field("DIO30", &self.DIO30())
            .field("DIO31", &self.DIO31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EVFLAGS31_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EVFLAGS31_0 {{ DIO0: {=bool:?}, DIO1: {=bool:?}, DIO2: {=bool:?}, DIO3: {=bool:?}, DIO4: {=bool:?}, DIO5: {=bool:?}, DIO6: {=bool:?}, DIO7: {=bool:?}, DIO8: {=bool:?}, DIO9: {=bool:?}, DIO10: {=bool:?}, DIO11: {=bool:?}, DIO12: {=bool:?}, DIO13: {=bool:?}, DIO14: {=bool:?}, DIO15: {=bool:?}, DIO16: {=bool:?}, DIO17: {=bool:?}, DIO18: {=bool:?}, DIO19: {=bool:?}, DIO20: {=bool:?}, DIO21: {=bool:?}, DIO22: {=bool:?}, DIO23: {=bool:?}, DIO24: {=bool:?}, DIO25: {=bool:?}, DIO26: {=bool:?}, DIO27: {=bool:?}, DIO28: {=bool:?}, DIO29: {=bool:?}, DIO30: {=bool:?}, DIO31: {=bool:?} }}",
            self.DIO0(),
            self.DIO1(),
            self.DIO2(),
            self.DIO3(),
            self.DIO4(),
            self.DIO5(),
            self.DIO6(),
            self.DIO7(),
            self.DIO8(),
            self.DIO9(),
            self.DIO10(),
            self.DIO11(),
            self.DIO12(),
            self.DIO13(),
            self.DIO14(),
            self.DIO15(),
            self.DIO16(),
            self.DIO17(),
            self.DIO18(),
            self.DIO19(),
            self.DIO20(),
            self.DIO21(),
            self.DIO22(),
            self.DIO23(),
            self.DIO24(),
            self.DIO25(),
            self.DIO26(),
            self.DIO27(),
            self.DIO28(),
            self.DIO29(),
            self.DIO30(),
            self.DIO31()
        )
    }
}

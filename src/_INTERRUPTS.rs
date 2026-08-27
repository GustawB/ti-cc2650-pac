#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _INTERRUPTS {
    ptr: *mut u8,
}
unsafe impl Send for _INTERRUPTS {}
unsafe impl Sync for _INTERRUPTS {}
impl _INTERRUPTS {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
}

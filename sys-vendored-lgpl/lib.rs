//! The vendored copy of `libftdi1` for use by `libftdi1-sys`.
//!
//! This crate is not intended to be used directly. If you're seeing this crate in your dependency
//! tree, this means that LGPL 2.1 licensed code will be compiled into your executable.
//!
//! For more detail and notes on maintenance see the crate readme in `README.md`.

#[cfg(test)]
mod test {
    #[repr(C)]
    pub struct ftdi_context(u8);

    extern "C" {
        fn ftdi_new() -> *mut ftdi_context;
        fn ftdi_free(ftdi: *mut ftdi_context);
    }

    #[test]
    fn smoke_test() {
        unsafe {
            let context = ftdi_new();
            assert!(!context.is_null());
            ftdi_free(context);
        }
    }
}

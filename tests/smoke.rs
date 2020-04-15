// SPDX-License-Identifier: MIT

use libftdi1_sys::*;

#[test]
fn smoke_test() {
    unsafe {
        let context = ftdi_new();
        assert!(!context.is_null());
        ftdi_free(context);
    }
}

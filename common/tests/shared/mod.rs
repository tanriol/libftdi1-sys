// SPDX-License-Identifier: MIT

// This file is intended to be included as a module in integration tests after `use crate_name::*;`

use super::*;

#[test]
fn smoke_test() {
    unsafe {
        let context = ftdi_new();
        assert!(!context.is_null());
        ftdi_free(context);
    }
}

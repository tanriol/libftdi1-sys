//! A simple test script for testing the loopback features
//! of the FTDI DLP-HS-FPGA3's default FPGA firmware.
//! Mainly used as a compile test

extern crate libftdi1_sys;

use libftdi1_sys as ftdi;
use std::os::raw::c_int;

fn write_unchecked(context: *mut ftdi::ftdi_context, data: Vec<u8>) {
    unsafe {
        ftdi::ftdi_write_data(context, data.as_ptr(), data.len() as i32);
    }
}

fn read_unchecked(context: *mut ftdi::ftdi_context, max_bytes: c_int) -> Vec<u8> {
    let mut data: Vec<u8> = vec![0; max_bytes as usize];
    unsafe {
        let bytes_read = ftdi::ftdi_read_data(context, data.as_mut_ptr(), max_bytes);
        data.truncate(bytes_read as usize);
    }
    data
}

// No error checking here, beware!
fn main() {
    println!("Starting tester...");
    let context = unsafe { ftdi::ftdi_new() };
    unsafe {
        ftdi::ftdi_set_interface(context, ftdi::ftdi_interface::INTERFACE_A);
    }

    if unsafe { ftdi::ftdi_usb_open(context, 0x0403, 0x6010) } == 0 {
        println!("Device found and opened");
        unsafe {
            ftdi::ftdi_usb_reset(context);
            ftdi::ftdi_usb_purge_buffers(context);
            ftdi::ftdi_set_latency_timer(context, 2);
        }

        // Junk test
        let buffer = read_unchecked(context, 1024);
        if buffer.len() > 0 {
            println!("Junk in line: {:?}", buffer);
        }

        // Ping test
        write_unchecked(context, vec![0x00]);
        let reply = read_unchecked(context, 100);
        if reply != vec![0x56] {
            println!("Wrong ping reply {:?} (expected {:?}", reply, vec![0x56]);
        }

        for num in 0u16..256 {
            let num = num as u8;

            // Loopback test
            write_unchecked(context, vec![0x20, num]);
            let reply = read_unchecked(context, 100);
            if reply != vec![num] {
                println!("Wrong loopback reply {:?} (expected {:?}", reply, vec![num]);
            }

            // Complement loopback test
            write_unchecked(context, vec![0x21, num]);
            let reply = read_unchecked(context, 100);
            let complement = 255 - num;
            if reply != vec![complement] {
                println!(
                    "Wrong complement reply {:?} (expected {:?}",
                    reply,
                    vec![complement]
                );
            }
        }
        println!("Testing finished");
    } else {
        println!("Cannot find/open device, runtime tests are NOP");
    }

    unsafe {
        ftdi::ftdi_free(context);
    }
}

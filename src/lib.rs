//! libftdi1 is a library for working with FTDI chips like
//! FT232BM, FT245BM, FT2232C, FT2232D, FT245R, FT232H, FT230X
//! The documentation for it is available at http://www.intra2net.com/en/developer/libftdi/documentation/
//!
//! This wrapper is based on the rust-bindgen output for libftdi 1.2

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ftdi_chip_type {
    TYPE_AM = 0,
    TYPE_BM = 1,
    TYPE_2232C = 2,
    TYPE_R = 3,
    TYPE_2232H = 4,
    TYPE_4232H = 5,
    TYPE_232H = 6,
    TYPE_230X = 7,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ftdi_parity_type {
    NONE = 0,
    ODD = 1,
    EVEN = 2,
    MARK = 3,
    SPACE = 4,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ftdi_stopbits_type {
    STOP_BIT_1 = 0,
    STOP_BIT_15 = 1,
    STOP_BIT_2 = 2,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ftdi_bits_type {
    BITS_7 = 7,
    BITS_8 = 8,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ftdi_break_type {
    BREAK_OFF = 0,
    BREAK_ON = 1,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ftdi_mpsse_mode {
    BITMODE_RESET = 0,
    BITMODE_BITBANG = 1,
    BITMODE_MPSSE = 2,
    BITMODE_SYNCBB = 4,
    BITMODE_MCU = 8,
    BITMODE_OPTO = 16,
    BITMODE_CBUS = 32,
    BITMODE_SYNCFF = 64,
    BITMODE_FT1284 = 128,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ftdi_interface {
    INTERFACE_ANY = 0,
    INTERFACE_A = 1,
    INTERFACE_B = 2,
    INTERFACE_C = 3,
    INTERFACE_D = 4,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ftdi_module_detach_mode {
    AUTO_DETACH_SIO_MODULE = 0,
    DONT_DETACH_SIO_MODULE = 1,
}

pub enum libusb_transfer {}

#[repr(C)]
#[derive(Copy)]
pub struct ftdi_transfer_control {
    pub completed: ::std::os::raw::c_int,
    pub buf: *mut ::std::os::raw::c_uchar,
    pub size: ::std::os::raw::c_int,
    pub offset: ::std::os::raw::c_int,
    pub ftdi: *mut ftdi_context,
    pub transfer: *mut libusb_transfer,
}
impl ::std::clone::Clone for ftdi_transfer_control {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for ftdi_transfer_control {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub enum libusb_context {}

pub enum libusb_device_handle {}

pub enum ftdi_eeprom {}

#[repr(C)]
#[derive(Copy)]
pub struct ftdi_context {
    pub usb_ctx: *mut libusb_context,
    pub usb_dev: *mut libusb_device_handle,
    pub usb_read_timeout: ::std::os::raw::c_int,
    pub usb_write_timeout: ::std::os::raw::c_int,
    pub _type: ftdi_chip_type,
    pub baudrate: ::std::os::raw::c_int,
    pub bitbang_enabled: ::std::os::raw::c_uchar,
    pub readbuffer: *mut ::std::os::raw::c_uchar,
    pub readbuffer_offset: ::std::os::raw::c_uint,
    pub readbuffer_remaining: ::std::os::raw::c_uint,
    pub readbuffer_chunksize: ::std::os::raw::c_uint,
    pub writebuffer_chunksize: ::std::os::raw::c_uint,
    pub max_packet_size: ::std::os::raw::c_uint,
    pub interface: ::std::os::raw::c_int,
    pub index: ::std::os::raw::c_int,
    pub in_ep: ::std::os::raw::c_int,
    pub out_ep: ::std::os::raw::c_int,
    pub bitbang_mode: ::std::os::raw::c_uchar,
    pub eeprom: *mut ftdi_eeprom,
    pub error_str: *mut ::std::os::raw::c_char,
    pub module_detach_mode: ftdi_module_detach_mode,
}
impl ::std::clone::Clone for ftdi_context {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for ftdi_context {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ftdi_eeprom_value {
    VENDOR_ID = 0,
    PRODUCT_ID = 1,
    SELF_POWERED = 2,
    REMOTE_WAKEUP = 3,
    IS_NOT_PNP = 4,
    SUSPEND_DBUS7 = 5,
    IN_IS_ISOCHRONOUS = 6,
    OUT_IS_ISOCHRONOUS = 7,
    SUSPEND_PULL_DOWNS = 8,
    USE_SERIAL = 9,
    USB_VERSION = 10,
    USE_USB_VERSION = 11,
    MAX_POWER = 12,
    CHANNEL_A_TYPE = 13,
    CHANNEL_B_TYPE = 14,
    CHANNEL_A_DRIVER = 15,
    CHANNEL_B_DRIVER = 16,
    CBUS_FUNCTION_0 = 17,
    CBUS_FUNCTION_1 = 18,
    CBUS_FUNCTION_2 = 19,
    CBUS_FUNCTION_3 = 20,
    CBUS_FUNCTION_4 = 21,
    CBUS_FUNCTION_5 = 22,
    CBUS_FUNCTION_6 = 23,
    CBUS_FUNCTION_7 = 24,
    CBUS_FUNCTION_8 = 25,
    CBUS_FUNCTION_9 = 26,
    HIGH_CURRENT = 27,
    HIGH_CURRENT_A = 28,
    HIGH_CURRENT_B = 29,
    INVERT = 30,
    GROUP0_DRIVE = 31,
    GROUP0_SCHMITT = 32,
    GROUP0_SLEW = 33,
    GROUP1_DRIVE = 34,
    GROUP1_SCHMITT = 35,
    GROUP1_SLEW = 36,
    GROUP2_DRIVE = 37,
    GROUP2_SCHMITT = 38,
    GROUP2_SLEW = 39,
    GROUP3_DRIVE = 40,
    GROUP3_SCHMITT = 41,
    GROUP3_SLEW = 42,
    CHIP_SIZE = 43,
    CHIP_TYPE = 44,
    POWER_SAVE = 45,
    CLOCK_POLARITY = 46,
    DATA_ORDER = 47,
    FLOW_CONTROL = 48,
    CHANNEL_C_DRIVER = 49,
    CHANNEL_D_DRIVER = 50,
    CHANNEL_A_RS485 = 51,
    CHANNEL_B_RS485 = 52,
    CHANNEL_C_RS485 = 53,
    CHANNEL_D_RS485 = 54,
    RELEASE_NUMBER = 55,
}

pub enum libusb_device {}

#[repr(C)]
#[derive(Copy)]
pub struct ftdi_device_list {
    pub next: *mut ftdi_device_list,
    pub dev: *mut libusb_device,
}
impl ::std::clone::Clone for ftdi_device_list {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for ftdi_device_list {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ftdi_cbus_func {
    CBUS_TXDEN = 0,
    CBUS_PWREN = 1,
    CBUS_RXLED = 2,
    CBUS_TXLED = 3,
    CBUS_TXRXLED = 4,
    CBUS_SLEEP = 5,
    CBUS_CLK48 = 6,
    CBUS_CLK24 = 7,
    CBUS_CLK12 = 8,
    CBUS_CLK6 = 9,
    CBUS_IOMODE = 10,
    CBUS_BB_WR = 11,
    CBUS_BB_RD = 12,
    CBUS_BB = 13,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ftdi_cbush_func {
    CBUSH_TRISTATE = 0,
    CBUSH_RXLED = 1,
    CBUSH_TXLED = 2,
    CBUSH_TXRXLED = 3,
    CBUSH_PWREN = 4,
    CBUSH_SLEEP = 5,
    CBUSH_DRIVE_0 = 6,
    CBUSG_DRIVE1 = 7,
    CBUSH_IOMODE = 8,
    CBUSH_TXDEN = 9,
    CBUSH_CLK30 = 10,
    CBUSH_CLK15 = 11,
    CBUSH_CLK7_5 = 12,
    CBUSH_BAT_DETECT = 13,
    CBUSH_BAT_DETECT_NEG = 14,
    CBUSH_I2C_TXE = 15,
    CBUSH_I2C_RXF = 16,
    CBUSH_VBUS_SENSE = 17,
    CBUSH_BB_WR = 18,
    CBUSH_BB_RD = 19,
    CBUSH_TIME_STAMP = 20,
    CBUSH_AWAKE = 21,
}

#[repr(C)]
#[derive(Copy)]
pub struct size_and_time {
    pub totalBytes: u64,
    pub time: libc::timeval,
}
impl ::std::clone::Clone for size_and_time {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for size_and_time {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct FTDIProgressInfo {
    pub first: size_and_time,
    pub prev: size_and_time,
    pub current: size_and_time,
    pub totalTime: ::std::os::raw::c_double,
    pub totalRate: ::std::os::raw::c_double,
    pub currentRate: ::std::os::raw::c_double,
}
impl ::std::clone::Clone for FTDIProgressInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for FTDIProgressInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub type FTDIStreamCallback = unsafe extern "C" fn(
    buffer: *mut u8,
    length: ::std::os::raw::c_int,
    progress: *mut FTDIProgressInfo,
    userdata: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int;

#[repr(C)]
#[derive(Copy)]
pub struct ftdi_version_info {
    pub major: ::std::os::raw::c_int,
    pub minor: ::std::os::raw::c_int,
    pub micro: ::std::os::raw::c_int,
    pub version_str: *const ::std::os::raw::c_char,
    pub snapshot_str: *const ::std::os::raw::c_char,
}
impl ::std::clone::Clone for ftdi_version_info {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for ftdi_version_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

// #[link(name = "ftdi1")]
extern "C" {
    pub fn ftdi_init(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
    pub fn ftdi_new() -> *mut ftdi_context;
    pub fn ftdi_set_interface(
        ftdi: *mut ftdi_context,
        interface: ftdi_interface,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_deinit(ftdi: *mut ftdi_context);
    pub fn ftdi_free(ftdi: *mut ftdi_context);
    pub fn ftdi_set_usbdev(ftdi: *mut ftdi_context, usbdev: *mut libusb_device_handle);
    pub fn ftdi_get_library_version() -> ftdi_version_info;
    pub fn ftdi_usb_find_all(
        ftdi: *mut ftdi_context,
        devlist: *mut *mut ftdi_device_list,
        vendor: ::std::os::raw::c_int,
        product: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_list_free(devlist: *mut *mut ftdi_device_list);
    pub fn ftdi_list_free2(devlist: *mut ftdi_device_list);
    pub fn ftdi_usb_get_strings(
        ftdi: *mut ftdi_context,
        dev: *mut libusb_device,
        manufacturer: *mut ::std::os::raw::c_char,
        mnf_len: ::std::os::raw::c_int,
        description: *mut ::std::os::raw::c_char,
        desc_len: ::std::os::raw::c_int,
        serial: *mut ::std::os::raw::c_char,
        serial_len: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_eeprom_set_strings(
        ftdi: *mut ftdi_context,
        manufacturer: *mut ::std::os::raw::c_char,
        product: *mut ::std::os::raw::c_char,
        serial: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_usb_open(
        ftdi: *mut ftdi_context,
        vendor: ::std::os::raw::c_int,
        product: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_usb_open_desc(
        ftdi: *mut ftdi_context,
        vendor: ::std::os::raw::c_int,
        product: ::std::os::raw::c_int,
        description: *const ::std::os::raw::c_char,
        serial: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_usb_open_desc_index(
        ftdi: *mut ftdi_context,
        vendor: ::std::os::raw::c_int,
        product: ::std::os::raw::c_int,
        description: *const ::std::os::raw::c_char,
        serial: *const ::std::os::raw::c_char,
        index: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_usb_open_dev(
        ftdi: *mut ftdi_context,
        dev: *mut libusb_device,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_usb_open_string(
        ftdi: *mut ftdi_context,
        description: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_usb_close(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
    pub fn ftdi_usb_reset(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
    pub fn ftdi_usb_purge_rx_buffer(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
    pub fn ftdi_usb_purge_tx_buffer(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
    pub fn ftdi_usb_purge_buffers(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
    pub fn ftdi_set_baudrate(
        ftdi: *mut ftdi_context,
        baudrate: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_set_line_property(
        ftdi: *mut ftdi_context,
        bits: ftdi_bits_type,
        sbit: ftdi_stopbits_type,
        parity: ftdi_parity_type,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_set_line_property2(
        ftdi: *mut ftdi_context,
        bits: ftdi_bits_type,
        sbit: ftdi_stopbits_type,
        parity: ftdi_parity_type,
        break_type: ftdi_break_type,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_read_data(
        ftdi: *mut ftdi_context,
        buf: *mut ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_read_data_set_chunksize(
        ftdi: *mut ftdi_context,
        chunksize: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_read_data_get_chunksize(
        ftdi: *mut ftdi_context,
        chunksize: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_write_data(
        ftdi: *mut ftdi_context,
        buf: *const ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_write_data_set_chunksize(
        ftdi: *mut ftdi_context,
        chunksize: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_write_data_get_chunksize(
        ftdi: *mut ftdi_context,
        chunksize: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_readstream(
        ftdi: *mut ftdi_context,
        callback: *mut FTDIStreamCallback,
        userdata: *mut ::std::os::raw::c_void,
        packetsPerTransfer: ::std::os::raw::c_int,
        numTransfers: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_write_data_submit(
        ftdi: *mut ftdi_context,
        buf: *mut ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> *mut ftdi_transfer_control;
    pub fn ftdi_read_data_submit(
        ftdi: *mut ftdi_context,
        buf: *mut ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> *mut ftdi_transfer_control;
    pub fn ftdi_transfer_data_done(tc: *mut ftdi_transfer_control) -> ::std::os::raw::c_int;
    pub fn ftdi_set_bitmode(
        ftdi: *mut ftdi_context,
        bitmask: ::std::os::raw::c_uchar,
        mode: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_disable_bitbang(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
    pub fn ftdi_read_pins(
        ftdi: *mut ftdi_context,
        pins: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_set_latency_timer(
        ftdi: *mut ftdi_context,
        latency: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_get_latency_timer(
        ftdi: *mut ftdi_context,
        latency: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_poll_modem_status(
        ftdi: *mut ftdi_context,
        status: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_setflowctrl(
        ftdi: *mut ftdi_context,
        flowctrl: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_setdtr_rts(
        ftdi: *mut ftdi_context,
        dtr: ::std::os::raw::c_int,
        rts: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_setdtr(
        ftdi: *mut ftdi_context,
        state: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_setrts(
        ftdi: *mut ftdi_context,
        state: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_set_event_char(
        ftdi: *mut ftdi_context,
        eventch: ::std::os::raw::c_uchar,
        enable: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_set_error_char(
        ftdi: *mut ftdi_context,
        errorch: ::std::os::raw::c_uchar,
        enable: ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_eeprom_initdefaults(
        ftdi: *mut ftdi_context,
        manufacturer: *mut ::std::os::raw::c_char,
        product: *mut ::std::os::raw::c_char,
        serial: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_eeprom_build(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
    pub fn ftdi_eeprom_decode(
        ftdi: *mut ftdi_context,
        verbose: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_get_eeprom_value(
        ftdi: *mut ftdi_context,
        value_name: ftdi_eeprom_value,
        value: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_set_eeprom_value(
        ftdi: *mut ftdi_context,
        value_name: ftdi_eeprom_value,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_get_eeprom_buf(
        ftdi: *mut ftdi_context,
        buf: *mut ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_set_eeprom_buf(
        ftdi: *mut ftdi_context,
        buf: *const ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_read_eeprom(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
    pub fn ftdi_read_chipid(
        ftdi: *mut ftdi_context,
        chipid: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_write_eeprom(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
    pub fn ftdi_erase_eeprom(ftdi: *mut ftdi_context) -> ::std::os::raw::c_int;
    pub fn ftdi_read_eeprom_location(
        ftdi: *mut ftdi_context,
        eeprom_addr: ::std::os::raw::c_int,
        eeprom_val: *mut ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_write_eeprom_location(
        ftdi: *mut ftdi_context,
        eeprom_addr: ::std::os::raw::c_int,
        eeprom_val: ::std::os::raw::c_ushort,
    ) -> ::std::os::raw::c_int;
    pub fn ftdi_get_error_string(ftdi: *mut ftdi_context) -> *mut ::std::os::raw::c_char;
}

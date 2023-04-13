use rusb::{devices, Device, DeviceDescriptor, DeviceHandle, Error, GlobalContext};
use std::time::Duration;

pub const NUM_REGIONS: u8 = 5;

pub const LOGITECH: u16 = 0x046d; // Vendor
pub const G213: u16 = 0xc336; // Device

const ENDPOINT: u8 = 0x82; // Read Interrupt

const REQ_TYPE: u8 = 0x21;
const REQ: u8 = 0x09;
const VALUE: u16 = 0x0211;
const INDEX: u16 = 0x0001;
const CMD_LEN: usize = 20;
const TIMEOUT_MS: u64 = 50;

const MIN_SPEED: u16 = 32;

#[repr(u8)]
#[derive(PartialEq)]
pub enum KeyboardRegions {
    WholeKeyboard = 0,
    Region1 = 1,
    Region2 = 2,
    Region3 = 3,
    Region4 = 4,
    Region5 = 5,
}

impl From<u8> for KeyboardRegions {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::WholeKeyboard,
            1 => Self::Region1,
            2 => Self::Region2,
            3 => Self::Region3,
            4 => Self::Region4,
            5 => Self::Region5,
            _ => Self::WholeKeyboard,
        }
    }
}

pub fn limit_speed(speed: u16) -> u16 {
    if speed < MIN_SPEED {
        MIN_SPEED
    } else {
        speed
    }
}

pub trait G213DeviceDescriptor {
    fn vendor_id(&self) -> u16;
    fn product_id(&self) -> u16;
}

impl G213DeviceDescriptor for DeviceDescriptor {
    fn vendor_id(&self) -> u16 {
        self.vendor_id()
    }

    fn product_id(&self) -> u16 {
        self.product_id()
    }
}

pub fn is_g213_keyboard(descriptor: &dyn G213DeviceDescriptor) -> bool {
    descriptor.vendor_id() == LOGITECH && descriptor.product_id() == G213
}

fn send_to_keyboard(
    handle: &DeviceHandle<GlobalContext>,
    bytes: &mut [u8],
) -> Result<usize, Error> {
    handle.write_control(
        REQ_TYPE,
        REQ,
        VALUE,
        INDEX,
        bytes,
        Duration::from_millis(TIMEOUT_MS),
    )?;

    handle.read_interrupt(ENDPOINT, bytes, Duration::from_millis(TIMEOUT_MS))
}

fn send_command(handle: &DeviceHandle<GlobalContext>, command: &str) -> Result<usize, Error> {
    let mut bytes = [0u8; CMD_LEN];

    hex::decode_to_slice(command, &mut bytes).unwrap();

    send_to_keyboard(handle, &mut bytes)
}

fn send_keyboard_colour(handle: &DeviceHandle<GlobalContext>, region: u8, colour: u32) {
    let command = format!(
        "11ff0c3a{:02x}01{:06x}0200000000000000000000",
        region, colour
    );

    let _bytes_sent = send_command(handle, &command).unwrap();
}

fn send_breathe(handle: &DeviceHandle<GlobalContext>, speed: u16, colour: u32) {
    let command = format!("11ff0c3a0002{:06x}{:04x}006400000000000000", colour, speed);

    let _bytes_sent = send_command(handle, &command).unwrap();
}

fn send_cycle(handle: &DeviceHandle<GlobalContext>, speed: u16) {
    let command = format!("11ff0c3a0003ffffff0000{:04x}64000000000000", speed);

    let _bytes_sent = send_command(handle, &command).unwrap();
}

fn do_show_info(descriptor: &DeviceDescriptor, handle: &DeviceHandle<GlobalContext>) {
    let timeout = std::time::Duration::from_millis(TIMEOUT_MS);
    let lang = handle.read_languages(timeout).unwrap()[0];

    println!(
        "Manufacturer: {}",
        handle
            .read_manufacturer_string(lang, descriptor, timeout)
            .unwrap()
    );

    println!(
        "Product:      {}",
        handle
            .read_product_string(lang, descriptor, timeout)
            .unwrap()
    );

    println!(
        "Serial:       {}",
        handle
            .read_serial_number_string(lang, descriptor, timeout)
            .unwrap()
    );
}

pub fn find_g213_keyboard() -> Option<Device<GlobalContext>> {
    devices().unwrap().iter().find(|device| {
        let desc = device.device_descriptor().unwrap();
        is_g213_keyboard(&desc)
    })
}

fn send_command_wrapper(
    device: &Device<GlobalContext>,
    cmd_fn: impl Fn(&DeviceHandle<GlobalContext>),
) {
    let mut handle = device.open().expect("Unable to open device!");

    let mut kernel_driver_detached = false;

    // Then we detach the kernel driver so that we can access the device
    if handle.kernel_driver_active(INDEX as u8).unwrap() {
        handle
            .detach_kernel_driver(INDEX as u8)
            .expect("Unable to detach kernel USB driver");

        kernel_driver_detached = true;
    }

    // Now we claim the interface
    handle
        .claim_interface(INDEX as u8)
        .expect("Unable to claim interface for device");

    // Do our thing
    cmd_fn(&handle);

    handle
        .release_interface(INDEX as u8)
        .expect("Unable to release interface for device");

    // Let the kernel take over again
    if kernel_driver_detached {
        handle
            .attach_kernel_driver(INDEX as u8)
            .expect("Unable to attach kernel USB driver");
    }
}

pub fn set_keyboard_colour(device: &Device<GlobalContext>, color: u32) {
    send_command_wrapper(device, |h| {
        send_keyboard_colour(h, KeyboardRegions::WholeKeyboard as u8, color);
    });
}

pub fn set_region_colour(device: &Device<GlobalContext>, region: u8, color: u32) {
    send_command_wrapper(device, |h| {
        send_keyboard_colour(h, region, color);
    });
}

pub fn set_breathe(device: &Device<GlobalContext>, speed: u16, color: u32) {
    send_command_wrapper(device, |h| {
        send_breathe(h, speed, color);
    });
}

pub fn set_cycle(device: &Device<GlobalContext>, speed: u16) {
    send_command_wrapper(device, |h| {
        send_cycle(h, speed);
    });
}

pub fn show_info(device: &Device<GlobalContext>) {
    let descriptor = device.device_descriptor().unwrap();

    send_command_wrapper(device, |h| do_show_info(&descriptor, h));
}

#[cfg(test)]
mod g213_keyboard_tests {
    // use rusb::{ffi::libusb_device_descriptor, DeviceDescriptor};

    use super::*;

    // NOTE: A lot of work to test a one line function...

    struct GoodG213DeviceDescriptor {}
    struct NonLogitechDeviceDescriptor {}
    struct NonG213DeviceDescriptor {}

    impl G213DeviceDescriptor for GoodG213DeviceDescriptor {
        fn vendor_id(&self) -> u16 {
            0x046d
        }

        fn product_id(&self) -> u16 {
            0xc336
        }
    }

    impl G213DeviceDescriptor for NonLogitechDeviceDescriptor {
        fn vendor_id(&self) -> u16 {
            0x0400
        }

        fn product_id(&self) -> u16 {
            0xc336
        }
    }

    impl G213DeviceDescriptor for NonG213DeviceDescriptor {
        fn vendor_id(&self) -> u16 {
            0x046d
        }

        fn product_id(&self) -> u16 {
            0x1234
        }
    }

    #[test]
    fn a_g213_keyboard() {
        let descriptor = GoodG213DeviceDescriptor {};

        assert_eq!(is_g213_keyboard(&descriptor), true);
    }

    #[test]
    fn not_a_logitech_device() {
        let descriptor = NonLogitechDeviceDescriptor {};

        assert_eq!(is_g213_keyboard(&descriptor), false);
    }

    #[test]
    fn not_a_g213_keyboard() {
        let descriptor = NonG213DeviceDescriptor {};

        assert_eq!(is_g213_keyboard(&descriptor), false);
    }
}

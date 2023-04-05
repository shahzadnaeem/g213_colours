use rusb::{devices, Device, DeviceDescriptor, DeviceHandle, GlobalContext};
use std::time::Duration;

pub const LOGITECH: u16 = 0x046d; // Vendor
pub const G213: u16 = 0xc336; // Device

const ENDPOINT: u8 = 0x82; // Read Interrupt

const REQ_TYPE: u8 = 0x21;
const REQ: u8 = 0x09;
const VALUE: u16 = 0x0211;
const INDEX: u16 = 0x0001;
const CMD_LEN: usize = 20;
const TIMEOUT_MS: u64 = 200;

pub fn is_g213_keyboard(descriptor: &DeviceDescriptor) -> bool {
    descriptor.vendor_id() == LOGITECH && descriptor.product_id() == G213
}

fn send_to_keyboard(handle: &DeviceHandle<GlobalContext>, bytes: &mut [u8]) -> usize {
    handle
        .write_control(
            REQ_TYPE,
            REQ,
            VALUE,
            INDEX,
            &bytes,
            Duration::from_millis(TIMEOUT_MS),
        )
        .unwrap();

    handle
        .read_interrupt(ENDPOINT, bytes, Duration::from_millis(TIMEOUT_MS))
        .unwrap()
}

fn send_set_whole_keyboard_color(handle: &DeviceHandle<GlobalContext>, color: u32) {
    let command = format!("11ff0c3a0001{:06x}0200000000000000000000", color);

    let mut bytes = [0u8; CMD_LEN];

    hex::decode_to_slice(command, &mut bytes).unwrap();

    let _bytes_sent = send_to_keyboard(handle, &mut bytes);

    // println!("{} bytes sent", _bytes_sent);
}

pub fn find_g213_keyboard() -> Option<Device<GlobalContext>> {
    devices().unwrap().iter().find(|device| {
        let desc = device.device_descriptor().unwrap();
        is_g213_keyboard(&desc)
    })
}

fn send_command_wrapper(
    device: Device<GlobalContext>,
    cmd_fn: impl Fn(&DeviceHandle<GlobalContext>),
) {
    let mut handle = device.open().expect("Unable to open device!");

    let mut kernel_driver_detached = false;

    if handle.kernel_driver_active(INDEX as u8).unwrap() {
        handle
            .detach_kernel_driver(INDEX as u8)
            .expect("Unable to detatch kernel USB driver");

        kernel_driver_detached = true;
    }

    cmd_fn(&handle);

    if kernel_driver_detached {
        handle
            .attach_kernel_driver(INDEX as u8)
            .expect("Unable to attach kernel USB driver");
    }
}

pub fn set_whole_keyboard_color(device: Device<GlobalContext>, color: u32) {
    send_command_wrapper(device, |h| {
        send_set_whole_keyboard_color(h, color);
    });
}

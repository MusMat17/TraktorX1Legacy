// Derived from Opa-/x1-mk1-usb2midi (Apache-2.0), modified for TraktorX1Legacy.
use std::sync::mpsc;

use rusb::{Device, UsbContext};

pub struct HotPlugHandler<T: UsbContext> {
    pub sender: mpsc::Sender<Device<T>>,
}

impl<T: UsbContext> rusb::Hotplug<T> for HotPlugHandler<T> {
    fn device_arrived(&mut self, device: Device<T>) {
        eprintln!(
            "[usb] X1 arrived: bus={} address={}",
            device.bus_number(),
            device.address()
        );
        if self.sender.send(device).is_err() {
            eprintln!("[usb] device queue is no longer available");
        }
    }

    fn device_left(&mut self, device: Device<T>) {
        eprintln!(
            "[usb] X1 left: bus={} address={}",
            device.bus_number(),
            device.address()
        );
    }
}

impl<T: UsbContext> Drop for HotPlugHandler<T> {
    fn drop(&mut self) {
        eprintln!("[usb] hotplug handler stopped");
    }
}

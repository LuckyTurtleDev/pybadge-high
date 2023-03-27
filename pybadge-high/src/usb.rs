use edgebadge::hal;
use hal::usb::UsbBus;
pub use usb_device::UsbError;
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_DEV: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

/// USB connection for serial communication.
pub struct Usb {}

impl Usb {
	pub(crate) fn init(usb_allocator: UsbBusAllocator<UsbBus>) -> Self {
		unsafe {
			USB_ALLOCATOR = Some(usb_allocator);
		}
		unsafe {
			USB_SERIAL = Some(SerialPort::new(USB_ALLOCATOR.as_ref().unwrap()));
		}
		unsafe {
			USB_DEV = Some(
				UsbDeviceBuilder::new(
					USB_ALLOCATOR.as_ref().unwrap(),
					UsbVidPid(0x16c0, 0x27dd)
				)
				.manufacturer("Fake company")
				.product("Serial port")
				.serial_number("TEST")
				.device_class(USB_CLASS_CDC)
				.build()
			);
		}
		Usb {}
	}

	/// Polls the UsbBus for new events.
	/// Return true if serial may have data available for reading, false otherwise.
	/// This should be called periodically as often as possible for the best data
	/// rate, or preferably from an interrupt handler. Must be called at least once every 10
	/// milliseconds while connected to the USB host to be USB compliant.
	pub fn poll(&self) -> bool {
		unsafe {
			USB_DEV
				.as_mut()
				.unwrap()
				.poll(&mut [USB_SERIAL.as_mut().unwrap()])
		}
	}

	/// Reads bytes from the port into `data` and returns the number of bytes read.
	///
	/// # Errors
	///
	/// * [`WouldBlock`](UsbError::WouldBlock) - No bytes available for reading.
	///
	/// Other errors from `usb-device` may also be propagated.
	pub fn read(&mut self, data: &mut [u8]) -> Result<usize, UsbError> {
		unsafe { USB_SERIAL.as_mut().unwrap().read(data) }
	}

	/// Writes bytes from `data` into the port and returns the number of bytes written.
	///
	/// # Errors
	///
	/// * [`WouldBlock`](UsbError::WouldBlock) - No bytes could be written because the
	///   buffers are full.
	///
	/// Other errors from `usb-device` may also be propagated.
	pub fn write(&mut self, data: &[u8]) -> Result<usize, UsbError> {
		unsafe { USB_SERIAL.as_mut().unwrap().write(data) }
	}
	//TODO: wrap more function from https://docs.rs/usbd-serial/0.1.1/usbd_serial/struct.SerialPort.html
	//TODO: wrap more function from https://docs.rs/usb-device/0.2.9/usb_device/device/struct.UsbDevice.html
}

use edgebadge::{hal, pac, pins::USB as UsbPins};
use hal::{clock::GenericClockController, usb::UsbBus};
use pac::{MCLK, USB as UsbPeripherals};
pub use usb_device::UsbError;
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_DEV: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

/// USB connection for serial communication.
#[non_exhaustive] // prevent the user from creating this struct manual, without calling init.
				  // to make sure static varibale are Some.
pub struct Usb {}

impl Usb {
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

pub struct UsbBuilder {
	pub usb_vid: u16,
	pub usb_pid: u16,
	pub manufacturer: &'static str,
	pub product: &'static str,
	pub serial_number: &'static str,
	// The Peripherals are not needed for something else,
	// after Pybadge::take() was called.
	// So simple move it to this Builder, where it is still needed.
	pub(crate) pins: UsbPins,
	pub(crate) clocks: GenericClockController,
	pub(crate) mclk: MCLK,
	pub(crate) peripherals: UsbPeripherals
}

impl UsbBuilder {
	/// Build the USB serial interface.
	///
	/// After building [`Usb::poll()`] must be called at least once every 10
	/// milliseconds while connected to the USB host to be USB compliant.
	pub fn build(mut self) -> Usb {
		let usb_allocator =
			self.pins
				.init(self.peripherals, &mut self.clocks, &mut self.mclk);
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
					UsbVidPid(self.usb_vid, self.usb_pid)
				)
				.manufacturer(self.manufacturer)
				.product(self.product)
				.serial_number(self.serial_number)
				.device_class(USB_CLASS_CDC)
				.build()
			);
		}
		Usb {}
	}

	//exist no dervie macro for this (usb_vid, get_usb_vid, set_usb_vid)?

	pub fn usb_vid(mut self, usb_vid: u16) -> Self {
		self.usb_vid = usb_vid;
		self
	}

	pub fn usb_pid(mut self, usb_pid: u16) -> Self {
		self.usb_pid = usb_pid;
		self
	}

	pub fn manufacturer(mut self, manufacturer: &'static str) -> Self {
		self.manufacturer = manufacturer;
		self
	}

	pub fn product(mut self, product: &'static str) -> Self {
		self.product = product;
		self
	}

	pub fn serial_number(mut self, serial_number: &'static str) -> Self {
		self.serial_number = serial_number;
		self
	}
}

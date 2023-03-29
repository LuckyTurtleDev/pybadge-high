//based on https://github.com/atsamd-rs/atsamd/blob/9495af975d6a35ae8bb455fae29ad0356fe20e09/boards/pygamer/examples/qspi.rs

use crate::{prelude::*, Delay};
use edgebadge::{hal, pac, pins};
use hal::qspi::{self, Command};
use pac::{MCLK, QSPI};

/// GD25Q16C 2M bytes flash storage.
///
/// Existing out of 8k pages, witch 265Bytes each.
/// Page `i` starts at adress `i << 8` and ends with `(i << 8) + 255`.
///
/// [DataSheet](https://cdn-shop.adafruit.com/product-files/4763/4763_GD25Q16CTIGR.pdf)
pub struct Flash {
	flash: edgebadge::qspi::Qspi<edgebadge::qspi::OneShot>
}
impl Flash {
	pub(crate) fn init(
		flash_pins: pins::QSPIFlash,
		mlk: &mut MCLK,
		qspi: QSPI,
		delay: &mut Delay
	) -> Self {
		let mut flash = qspi::Qspi::new(
			mlk,
			qspi,
			flash_pins.sck,
			flash_pins.cs,
			flash_pins.data0,
			flash_pins.data1,
			flash_pins.data2,
			flash_pins.data3
		);

		// Startup delay. Can't find documented but Adafruit use 5ms
		delay.delay_ms(5u8);
		// Reset. It is recommended to check the BUSY(WIP?) bit and the SUS before reset
		wait_ready(&mut flash);
		flash.run_command(Command::EnableReset).unwrap();
		flash.run_command(Command::Reset).unwrap();
		// tRST(30μs) to reset. During this period, no command will be accepted
		delay.delay_ms(1u8);

		// 120MHz / 2 = 60mhz
		// faster than 104mhz at 3.3v would require High Performance Mode
		flash.set_clk_divider(2);

		// Enable Quad SPI mode. Requires write enable. Check WIP.
		flash.run_command(Command::WriteEnable).unwrap();
		flash.write_command(Command::WriteStatus2, &[0x02]).unwrap();
		wait_ready(&mut flash);

		Self { flash }
	}

	/// Read data from flash to `read_buf`.
	/// The first byte addressed (`addr`)  can be at any location.
	/// The address is automatically incremented to the next higher address after each byte of data is shifted out until `read_buf` is full.
	pub fn read(&mut self, addr: u32, read_buf: &mut [u8]) {
		// datasheet claims 6BH needs a single dummy byte, but doesnt work then
		// adafruit uses 8, and the underlying implementation uses 8 atm as well
		self.flash.read_memory(addr, read_buf);
	}

	// The following unwraps do only unwrap an result, witch check if I had use the right command.

	/// Write bytes from `write_buf` to a page inside the flash.
	///
	/// If the 8 least significant address bits (`addr`) are not all zero, all transmitted data
	/// that goes beyond the end of the current page are programmed from the start address of the same page.
	///
	/// If more than 256 bytes are sent to the device, previously latched data
	/// are discarded and the last 256 data bytes are guaranteed to be
	/// programmed correctly within the same page. If less than 256 data
	/// bytes are sent to device, they are correctly programmed at the
	/// requested addresses without having any effects on the other bytes of
	/// the same page.
	pub fn write_page(&mut self, addr: u32, write_buf: &[u8]) {
		self.flash.run_command(Command::WriteEnable).unwrap();
		self.flash.write_memory(addr, write_buf);
		wait_ready(&mut self.flash);
	}

	///Erase the whole chip. Can take up to 140 seconds!
	pub fn erase_chip(&mut self) {
		// Chip Erase. Requires write enable. Check WIP.
		self.flash.run_command(Command::WriteEnable).unwrap();
		self.flash.erase_command(Command::EraseChip, 0x0).unwrap();
		// Worst case up to 140 seconds!
		wait_ready(&mut self.flash);
	}
}

/// Wait for the write-in-progress and suspended write/erase.
fn wait_ready(flash: &mut qspi::Qspi<qspi::OneShot>) {
	while flash_status(flash, Command::ReadStatus) & 0x01 != 0 {}
	while flash_status(flash, Command::ReadStatus2) & 0x80 != 0 {}
}

/// Returns the contents of the status register indicated by cmd.
fn flash_status(flash: &mut qspi::Qspi<qspi::OneShot>, cmd: Command) -> u8 {
	let mut out = [0u8; 1];
	flash.read_command(cmd, &mut out).ok().unwrap();
	out[0]
}

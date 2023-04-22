#![no_std]
#![no_main]

//! Show send data at the display of the pybadge and send uppercase version back

use cortex_m_rt::entry;
use embedded_graphics::{
	mono_font::{ascii::FONT_6X10, MonoTextStyle},
	prelude::*,
	text::Text
};
use heapless::String;
use pybadge::{usb::UsbError, PyBadge};
use pybadge_high as pybadge;
use pybadge_high::Color;

#[entry]
fn main() -> ! {
	let pybadge = PyBadge::take().unwrap();
	let mut usb = pybadge.usb_builder.build();
	let mut display = pybadge.display;
	let style = MonoTextStyle::new(&FONT_6X10, Color::WHITE);
	display.clear(Color::BLACK).unwrap();
	loop {
		while !usb.poll() {}
		let mut buf = [0u8; 64];
		let read_count = match usb.read(&mut buf) {
			Ok(value) => value,
			Err(error) => match error {
				UsbError::WouldBlock => continue,
				_ => panic!()
			}
		};
		let mut string: String<64> = buf
			.into_iter()
			.take(read_count)
			.map(|f| char::from(f))
			.collect();
		display.clear(Color::BLACK).unwrap();
		Text::new(&string, Point::new(20, 30), style)
			.draw(&mut display)
			.unwrap();
		string.make_ascii_uppercase();
		usb.write(string.as_bytes()).unwrap();
	}
}

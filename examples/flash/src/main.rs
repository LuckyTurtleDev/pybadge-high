#![no_std]
#![no_main]

!!! FLASH DOES NOT WORK AND IS DISABLE NOW !!!

use core::fmt::Write;
use embedded_graphics::{
	mono_font::{ascii::FONT_6X10, MonoTextStyle},
	prelude::*,
	text::Text
};
use heapless::String;
use pybadge::{prelude::*, PyBadge};
use pybadge_high as pybadge;
use pybadge_high::Color;

#[entry]
fn main() -> ! {
	let pybadge = PyBadge::take().unwrap();
	let mut flash = pybadge.flash;
	let mut display = pybadge.display;
	let mut delay = pybadge.delay;
	let style = MonoTextStyle::new(&FONT_6X10, Color::WHITE);
	let mut string = String::<32>::new();

	//test page read
	let addr = 0; // start adress of page i (i << 8)
	let write_buf = [2_u8, 34, 3, 220, 45, 84, 12, 87];
	flash.write_page(addr, &write_buf);
	delay.delay_ms(255u8);
	let mut read_buf = [1_u8; 8];
	flash.read(addr, &mut read_buf);
	display.clear(Color::BLACK).unwrap();
	write!(string, "{read_buf:?}").unwrap();
	Text::new(&string, Point::new(0, 30), style)
		.draw(&mut display)
		.unwrap();
	//assert_eq!(read_buf, write_buf);
	#[allow(clippy::empty_loop)]
	loop {}
}

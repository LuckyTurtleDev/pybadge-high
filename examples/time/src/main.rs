#![no_std]
#![no_main]
use core::fmt::Write;
use embedded_graphics::{
	mono_font::{ascii::FONT_6X10, MonoTextStyle},
	prelude::*,
	text::Text
};
use heapless::String;
use pybadge::{prelude::*, PyBadge};
use pybadge_high as pybadge;
use pybadge_high::{time::uptime, Color};

#[entry]
fn main() -> ! {
	let pybadge = PyBadge::take().unwrap();
	let mut display = pybadge.display;
	let mut delay = pybadge.delay;
	let mut string = String::<32>::new();

	loop {
		let style = MonoTextStyle::new(&FONT_6X10, Color::WHITE);
		string.clear();
		write!(string, "uptime: {} ms", uptime().0).unwrap();
		display.clear(Color::BLACK).unwrap();
		Text::new(&string, Point::new(20, 30), style)
			.draw(&mut display)
			.unwrap();
		delay.delay_ms(500_u16);
	}
}

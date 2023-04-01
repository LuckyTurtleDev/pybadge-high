#![no_std]
#![no_main]
use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_graphics::{
	mono_font::{ascii::FONT_6X10, MonoTextStyle},
	prelude::*,
	text::Text
};
use heapless::String;
use pybadge::PyBadge;
use pybadge_high as pybadge;
use pybadge_high::{time::Hertz, Color};

#[entry]
fn main() -> ! {
	let pybadge = PyBadge::take().unwrap();
	let mut speaker = pybadge.speaker;
	speaker.set_freq(Hertz(200));
	speaker.enable();

	let mut display = pybadge.display;
	display.clear(Color::BLACK).unwrap();
	let mut string = String::<32>::new();
	write!(string, "{} Herz", 200).unwrap();
	let style = MonoTextStyle::new(&FONT_6X10, Color::WHITE);
	Text::new(&string, Point::new(20, 30), style)
		.draw(&mut display)
		.unwrap();

	loop {}
}
